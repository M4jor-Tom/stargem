use std::collections::VecDeque;
use indexmap::IndexMap;

use crate::proto::combat::GameStateSnapshot;

pub const JOURNAL_CAP: usize = 200;

pub struct LiveWorld {
    pub players: IndexMap<String, ShipSnap>,
    pub journal: VecDeque<JournalEntry>,
    pub tick: u64,
    pub follow_idx: usize,
}

impl Default for LiveWorld {
    fn default() -> Self {
        Self {
            players: IndexMap::new(),
            journal: VecDeque::new(),
            tick: 0,
            follow_idx: 0,
        }
    }
}

#[derive(Clone)]
pub struct ShipSnap {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub shield_hp: f32,
    pub armor_hp: f32,
}

pub struct JournalEntry {
    pub tick: u64,
    pub text: String,
}

pub fn apply_snapshot(world: &mut LiveWorld, snap: &GameStateSnapshot) {
    world.tick = snap.tick_number as u64;
    for ship in snap.players.iter() {
        let key = ship.player_id.clone();
        let pos = ship.position.as_ref();
        let rot = ship.rotation.as_ref();
        let snap = ShipSnap {
            position: [
                pos.map(|v| v.x).unwrap_or(0.0),
                pos.map(|v| v.y).unwrap_or(0.0),
                pos.map(|v| v.z).unwrap_or(0.0),
            ],
            rotation: [
                rot.map(|v| v.x).unwrap_or(0.0),
                rot.map(|v| v.y).unwrap_or(0.0),
                rot.map(|v| v.z).unwrap_or(0.0),
                rot.map(|v| v.w).unwrap_or(1.0),
            ],
            shield_hp: ship.shield_hp,
            armor_hp: ship.armor_hp,
        };
        world.players.insert(key, snap);
    }
    for d in &snap.damage_events {
        let src = d.source.as_ref().map(|p| p.id.clone()).unwrap_or_default();
        let tgt = d.target.as_ref().map(|p| p.id.clone()).unwrap_or_default();
        let text = format!("{src} → {tgt}  {} dmg {} ({:.0} mit)", d.damage_type, d.raw_amount, d.mitigated_amount);
        world.journal.push_front(JournalEntry { tick: world.tick, text });
        while world.journal.len() > JOURNAL_CAP { world.journal.pop_back(); }
    }
    if world.follow_idx >= world.players.len() && !world.players.is_empty() {
        world.follow_idx = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::combat::{DamageEvent, ShipState};
    use crate::proto::common::{PlayerId, Quaternion, Vector3};

    fn ship(id: &str, x: f32) -> ShipState {
        ShipState {
            version: 1,
            position: Some(Vector3 { x, y: 0.0, z: 0.0 }),
            velocity: None,
            rotation: Some(Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }),
            shield_hp: 50.0, armor_hp: 25.0,
            energy: 0.0, heat_level: 0.0,
            player_id: id.into(),
        }
    }

    #[test]
    fn apply_snapshot_updates_world_and_journal() {
        let mut w = LiveWorld::default();
        let snap = GameStateSnapshot {
            version: 1, tick_number: 3,
            players: vec![ship("atk", 0.0), ship("def", 10.0)],
            damage_events: vec![DamageEvent {
                version: 1,
                source: Some(PlayerId { id: "atk".into() }),
                target: Some(PlayerId { id: "def".into() }),
                damage_type: "kinetic".into(),
                raw_amount: 25.0, mitigated_amount: 12.5,
            }],
            missile_states: vec![],
        };
        apply_snapshot(&mut w, &snap);
        assert_eq!(w.tick, 3);
        assert_eq!(w.players.len(), 2);
        assert_eq!(w.players.get("atk").unwrap().position[0], 0.0);
        assert_eq!(w.players.get("def").unwrap().position[0], 10.0);
        assert_eq!(w.journal.len(), 1);
        assert!(w.journal[0].text.contains("atk"));
    }

    #[test]
    fn journal_is_capped() {
        let mut w = LiveWorld::default();
        for i in 0..(JOURNAL_CAP + 50) {
            let snap = GameStateSnapshot {
                version: 1, tick_number: i as u32,
                players: vec![ship("a", 0.0)],
                damage_events: vec![DamageEvent {
                    version: 1,
                    source: Some(PlayerId { id: "a".into() }),
                    target: Some(PlayerId { id: "b".into() }),
                    damage_type: "kinetic".into(),
                    raw_amount: 1.0, mitigated_amount: 0.5,
                }],
                missile_states: vec![],
            };
            apply_snapshot(&mut w, &snap);
        }
        assert_eq!(w.journal.len(), JOURNAL_CAP);
    }
}