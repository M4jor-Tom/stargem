//! raylib client: flies one ship from the ontology catalog through a field of asteroids.
//! `cargo run -p client [ship-model-id]`

use glam::Vec3;
use ontology::Catalog;
use raylib::prelude::*;
use sim::{Body, FlightParams, Input, Obstacle};
use std::path::Path;

fn v3(v: Vec3) -> Vector3 { Vector3::new(v.x, v.y, v.z) }

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let catalog = Catalog::load(&root.join("ontology/instances")).expect("catalog");
    let model_id = std::env::args().nth(1).unwrap_or_else(|| "hydra".into());
    let ship = catalog.buy(&model_id, "local", "local-ship").unwrap_or_else(|| panic!("unknown ship model {model_id}"));
    let model = catalog.ship_model(&model_id).unwrap();
    let stats = catalog.effective_stats(&ship).unwrap();
    let params = FlightParams::from(&stats);
    let radius = sim::hull_radius(model.size());

    let (mut rl, thread) = raylib::init().size(1280, 720).title("stargem").build();
    rl.set_target_fps(120);

    // ponytail: one placeholder hull for every model; map ship-model id → glTF path when the designer delivers per-ship art
    let gltf = root.join("docs/3D/star_conflict_ships.gltf/gltf/mjolnir/scene.gltf");
    let art = rl.load_model(&thread, gltf.to_str().unwrap()).ok();
    let art_scale = art.as_ref().map(|m| {
        let bb = m.get_model_bounding_box();
        let extent = (bb.max.x - bb.min.x).max(bb.max.y - bb.min.y).max(bb.max.z - bb.min.z);
        2.0 * radius / extent
    });

    let mut body = Body::at(Vec3::ZERO, radius);
    let rocks: Vec<Obstacle> = (0..24)
        .map(|i| {
            let a = i as f32 * 0.7;
            Obstacle { position: Vec3::new(a.cos() * (150.0 + i as f32 * 12.0), (i % 5) as f32 * 25.0 - 50.0, a.sin() * (150.0 + i as f32 * 12.0) - 200.0), radius: 10.0 + (i % 4) as f32 * 8.0 }
        })
        .collect();

    let mut cam = Camera3D::perspective(Vector3::new(0.0, 10.0, 30.0), Vector3::zero(), Vector3::new(0.0, 1.0, 0.0), 60.0);
    let mut hit_timer = 0.0f32;
    // STARGEM_SHOT=<file.png>: save frame 120 and quit (headless visual check)
    let shot = std::env::var("STARGEM_SHOT").ok();
    let mut frame = 0u32;

    while !rl.window_should_close() {
        frame += 1;
        if let Some(p) = shot.as_deref().filter(|_| frame == 120) {
            rl.take_screenshot(&thread, p);
            break;
        }
        let dt = rl.get_frame_time();
        let axis = |neg: KeyboardKey, pos: KeyboardKey| (rl.is_key_down(pos) as i32 - rl.is_key_down(neg) as i32) as f32;
        let input = Input {
            throttle: axis(KeyboardKey::KEY_S, KeyboardKey::KEY_W),
            yaw: axis(KeyboardKey::KEY_RIGHT, KeyboardKey::KEY_LEFT) + axis(KeyboardKey::KEY_D, KeyboardKey::KEY_A),
            pitch: axis(KeyboardKey::KEY_UP, KeyboardKey::KEY_DOWN),
            roll: axis(KeyboardKey::KEY_Q, KeyboardKey::KEY_E),
        };
        sim::step(&mut body, &params, &input, dt);
        if sim::resolve_collisions(&mut body, &rocks) { hit_timer = 0.3; }
        hit_timer = (hit_timer - dt).max(0.0);

        // third-person follow: exponential smoothing, frame-rate independent
        let want = body.position - body.forward() * (radius * 4.0) + body.up() * (radius * 1.5);
        let t = 1.0 - (-8.0 * dt).exp();
        let cp = Vec3::new(cam.position.x, cam.position.y, cam.position.z).lerp(want, t);
        cam.position = v3(cp);
        cam.target = v3(body.position + body.forward() * 30.0);
        cam.up = v3(body.up());

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(8, 10, 20, 255));
        {
            let mut d3 = d.begin_mode3D(cam);
            d3.draw_grid(60, 20.0);
            for r in &rocks {
                d3.draw_sphere(v3(r.position), r.radius, Color::DARKGRAY);
                d3.draw_sphere_wires(v3(r.position), r.radius, 8, 8, Color::GRAY);
            }
            let tint = if hit_timer > 0.0 { Color::RED } else { Color::WHITE };
            let (ax, angle) = body.rotation.to_axis_angle();
            match (&art, art_scale) {
                (Some(m), Some(s)) => d3.draw_model_ex(m, v3(body.position), v3(ax), angle.to_degrees(), Vector3::new(s, s, s), tint),
                _ => d3.draw_cube_wires(v3(body.position), radius, radius * 0.4, radius * 1.6, tint),
            }
        }
        d.draw_text(&format!("{} ({:?} {:?})", model.name, model.size(), model.role), 12, 12, 20, Color::RAYWHITE);
        d.draw_text(&format!("speed {:5.1} / {:.0} m/s   shield {:.0}  armor {:.0}", body.velocity.length(), params.max_speed, stats.shield_hp, stats.armor_hp), 12, 36, 20, Color::RAYWHITE);
        d.draw_text("W/S throttle  arrows/AD pitch+yaw  Q/E roll", 12, 60, 18, Color::GRAY);
        if hit_timer > 0.0 { d.draw_text("COLLISION", 12, 90, 30, Color::RED); }
    }
}
