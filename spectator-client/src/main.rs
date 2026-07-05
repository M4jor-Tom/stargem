mod proto;
mod grpc;
mod world;
mod render;
mod camera;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use clap::Parser;

use grpc::{spawn_grpc_task, GrpcConfig};
use render::{draw_shots_system, drain_events, setup_scene, sync_ship_entities, EventRx, LiveWorldRes};
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
        .add_plugins(EguiPlugin)
        .insert_resource(LiveWorldRes(LiveWorld::default()))
        .insert_resource(EventRx(rx))
        .init_resource::<camera::CameraOrbit>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (
            drain_events,
            sync_ship_entities,
            camera::switch_target_system,
            camera::orbit_input_system,
            camera::follow_camera_system,
            draw_shots_system,
            ui::ui_system,
        ).chain())
        .run();
}