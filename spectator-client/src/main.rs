mod proto;
mod grpc;

use clap::Parser;
use grpc::{spawn_grpc_task, GrpcConfig, SpectatorEvent};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:50051")]
    grpc_addr: String,
    #[arg(long)]
    match_name: Option<String>,
    #[arg(long)]
    r#match: Option<String>,
}

fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let rx = spawn_grpc_task(GrpcConfig {
        addr: args.grpc_addr,
        match_name: args.match_name,
        match_id: args.r#match,
    });

    let mut shown = 0;
    for ev in rx.iter() {
        match ev {
            SpectatorEvent::Matches(m) => println!("matches: {:?}", m.iter().map(|i| &i.match_id).collect::<Vec<_>>()),
            SpectatorEvent::Snapshot(s) => {
                println!("tick={} players={} dmg={}", s.tick_number, s.players.len(), s.damage_events.len());
                shown += 1;
                if shown >= 5 { break; }
            }
            SpectatorEvent::Error(e) => { eprintln!("err: {e}"); break; }
        }
    }
}