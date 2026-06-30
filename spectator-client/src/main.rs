mod proto;
mod grpc;
mod world;
mod render;

use bevy::prelude::*;
use clap::Parser;

use grpc::{spawn_grpc_task, GrpcConfig};
use render::{drain_events, setup_scene, sync_ship_entities, EventRx, LiveWorldRes};
use world::LiveWorld;

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

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(LiveWorldRes(LiveWorld::default()))
        .insert_resource(EventRx(rx))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (drain_events, sync_ship_entities).chain())
        .run();
}