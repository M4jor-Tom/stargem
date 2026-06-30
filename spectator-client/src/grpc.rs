use crossbeam_channel::{bounded, Receiver, Sender};
use tokio::runtime::Runtime;

use crate::proto::combat::GameStateSnapshot;
use crate::proto::spectator::{
    spectator_service_client::SpectatorServiceClient,
    ListMatchesRequest, MatchInfo, SubscribeRequest,
};

#[derive(Debug, Clone)]
pub struct GrpcConfig {
    pub addr: String,
    pub match_name: Option<String>,
    pub match_id: Option<String>,
}

#[derive(Debug)]
pub enum SpectatorEvent {
    Snapshot(GameStateSnapshot),
    Matches(Vec<MatchInfo>),
    Error(String),
}

pub fn spawn_grpc_task(cfg: GrpcConfig) -> Receiver<SpectatorEvent> {
    let (tx, rx) = bounded(1024);
    std::thread::spawn(move || {
        let rt = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                let _ = tx.send(SpectatorEvent::Error(format!("tokio init failed: {e}")));
                return;
            }
        };
        rt.block_on(run(cfg, tx));
    });
    rx
}

async fn run(cfg: GrpcConfig, tx: Sender<SpectatorEvent>) {
    let endpoint = format!("http://{}", cfg.addr);
    let mut client = match SpectatorServiceClient::connect(endpoint).await {
        Ok(c) => c,
        Err(e) => { let _ = tx.send(SpectatorEvent::Error(format!("connect: {e}"))); return; }
    };

    let matches = match client.list_matches(ListMatchesRequest {}).await {
        Ok(r) => r.into_inner().matches,
        Err(e) => { let _ = tx.send(SpectatorEvent::Error(format!("list: {e}"))); return; }
    };
    let _ = tx.send(SpectatorEvent::Matches(matches.clone()));

    let target_id = match (&cfg.match_id, &cfg.match_name) {
        (Some(id), _) => id.clone(),
        (None, Some(name)) => match matches.iter().find(|m| m.match_id == *name) {
            Some(m) => m.match_id.clone(),
            None => { let _ = tx.send(SpectatorEvent::Error(format!("no match id {name}"))); return; }
        },
        (None, None) => {
            if matches.len() == 1 {
                matches[0].match_id.clone()
            } else {
                let _ = tx.send(SpectatorEvent::Error(
                    "multiple matches available; pass --match-name or --match".into()));
                return;
            }
        }
    };

    let mut stream = match client.subscribe(SubscribeRequest {
        match_id: target_id,
    }).await {
        Ok(r) => r.into_inner(),
        Err(e) => { let _ = tx.send(SpectatorEvent::Error(format!("subscribe: {e}"))); return; }
    };

    loop {
        match stream.message().await {
            Ok(Some(snap)) => { if tx.send(SpectatorEvent::Snapshot(snap)).is_err() { return; } }
            Ok(None) => { let _ = tx.send(SpectatorEvent::Error("stream ended".into())); return; }
            Err(e) => { let _ = tx.send(SpectatorEvent::Error(format!("recv: {e}"))); return; }
        }
    }
}