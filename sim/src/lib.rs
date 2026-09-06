//! Headless flight + collision, derived from `ontology/domain.md`:
//! arcade physics (speed cap, drag, instant acceleration) and parry3d collision.
//! Shared by the client (prediction/rendering) and the authoritative server.

pub use glam;
use glam::{Quat, Vec3};
use ontology::{ShipSize, Stats};
use parry3d::math::Pose;
use parry3d::query;
use parry3d::shape::Ball;

/// Normalised pilot intent for one tick.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Input {
    /// -1..=1, forward is positive
    pub throttle: f32,
    /// -1..=1 each, positive = nose left / nose up / roll right
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Body {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    /// bounding sphere, metres
    pub radius: f32,
}

impl Body {
    pub fn at(position: Vec3, radius: f32) -> Self {
        Self { position, rotation: Quat::IDENTITY, velocity: Vec3::ZERO, radius }
    }
    /// glTF convention: nose along -Z, up +Y.
    pub fn forward(&self) -> Vec3 { self.rotation * Vec3::NEG_Z }
    pub fn up(&self) -> Vec3 { self.rotation * Vec3::Y }
    pub fn right(&self) -> Vec3 { self.rotation * Vec3::X }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlightParams {
    /// m/s cap (`stats.speed`)
    pub max_speed: f32,
    /// rad/s (`stats.agility` is deg/s)
    pub turn_rate: f32,
    /// 1/s: how fast velocity chases the throttle target. Big = "instant".
    pub accel: f32,
    /// 1/s: velocity decay with throttle at 0
    pub drag: f32,
}

impl From<&Stats> for FlightParams {
    fn from(s: &Stats) -> Self {
        // ponytail: accel/drag are tuning knobs, not ontology stats yet; promote them if balance needs per-ship values
        Self { max_speed: s.speed, turn_rate: s.agility.to_radians(), accel: 6.0, drag: 1.5 }
    }
}

/// Bounding-sphere radius per ship size, metres.
pub fn hull_radius(size: ShipSize) -> f32 {
    match size {
        ShipSize::Frigate => 12.0,
        ShipSize::Fighter => 7.0,
        ShipSize::Interceptor => 4.0,
    }
}

/// One fixed-step integration of arcade flight.
pub fn step(body: &mut Body, p: &FlightParams, input: &Input, dt: f32) {
    let dyaw = input.yaw.clamp(-1.0, 1.0) * p.turn_rate * dt;
    let dpitch = input.pitch.clamp(-1.0, 1.0) * p.turn_rate * dt;
    let droll = -input.roll.clamp(-1.0, 1.0) * p.turn_rate * dt;
    // local-axis rotations: yaw about up, pitch about right, roll about forward
    body.rotation = (body.rotation * Quat::from_euler(glam::EulerRot::YXZ, dyaw, dpitch, droll)).normalize();

    let throttle = input.throttle.clamp(-1.0, 1.0);
    let target = body.forward() * (throttle * p.max_speed);
    let rate = if throttle != 0.0 { p.accel } else { p.drag };
    let t = 1.0 - (-rate * dt).exp(); // frame-rate independent
    body.velocity += (target - body.velocity) * t;
    let speed = body.velocity.length();
    if speed > p.max_speed { body.velocity *= p.max_speed / speed; }
    body.position += body.velocity * dt;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obstacle {
    pub position: Vec3,
    pub radius: f32,
}

/// Pushes `body` out of every obstacle it penetrates and removes the velocity
/// component into it. Returns true if any contact happened.
// ponytail: balls only; swap Ball for a ConvexPolyhedron built from the glTF hull when hitboxes matter
pub fn resolve_collisions(body: &mut Body, obstacles: &[Obstacle]) -> bool {
    let hull = Ball::new(body.radius);
    let mut hit = false;
    for o in obstacles {
        let p1 = Pose::from_translation(body.position);
        let p2 = Pose::from_translation(o.position);
        let Ok(Some(c)) = query::contact(&p1, &hull, &p2, &Ball::new(o.radius), 0.0) else { continue };
        if c.dist < 0.0 {
            hit = true;
            body.position += c.normal1 * c.dist; // dist < 0: move against the normal, out of the obstacle
            let into = body.velocity.dot(c.normal1);
            if into > 0.0 { body.velocity -= c.normal1 * into; }
        }
    }
    hit
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params() -> FlightParams {
        FlightParams { max_speed: 100.0, turn_rate: 1.0, accel: 6.0, drag: 1.5 }
    }

    #[test]
    fn throttle_reaches_cap_and_drag_stops() {
        let p = params();
        let mut b = Body::at(Vec3::ZERO, 1.0);
        let full = Input { throttle: 1.0, ..Default::default() };
        for _ in 0..600 { step(&mut b, &p, &full, 1.0 / 60.0); }
        assert!((b.velocity.length() - 100.0).abs() < 0.5, "{}", b.velocity.length());
        assert!(b.position.z < 0.0, "flies along -Z");
        for _ in 0..600 { step(&mut b, &p, &Input::default(), 1.0 / 60.0); }
        assert!(b.velocity.length() < 0.1);
    }

    #[test]
    fn yaw_turns_the_nose() {
        let p = params();
        let mut b = Body::at(Vec3::ZERO, 1.0);
        let turn = Input { yaw: 1.0, ..Default::default() };
        for _ in 0..90 { step(&mut b, &p, &turn, 1.0 / 60.0); } // 1.5 rad ≈ 86°
        assert!(b.forward().x < -0.9, "{:?}", b.forward());
    }

    #[test]
    fn ship_is_pushed_out_of_an_obstacle() {
        let mut b = Body::at(Vec3::new(0.0, 0.0, -5.0), 2.0);
        b.velocity = Vec3::new(0.0, 0.0, -50.0);
        let rock = Obstacle { position: Vec3::new(0.0, 0.0, -10.0), radius: 5.0 };
        assert!(resolve_collisions(&mut b, &[rock]));
        assert!(b.position.distance(rock.position) >= 7.0 - 1e-3);
        assert!(b.velocity.z >= 0.0, "no velocity into the rock: {:?}", b.velocity);
        assert!(!resolve_collisions(&mut b, &[rock]));
    }
}
