//! Typed mirror of `domain.md`. Loads `instances/` and checks the constraints (C1–C11).
//! Runtime-only constraints (C12–C17) belong to the server simulation.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

pub type Id = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShipSize { Frigate, Fighter, Interceptor }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShipRole { Engineer, LongRange, Guard, Tackler, Gunship, Command, CovertOps, Recon, Ecm }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpecialModuleKind {
    Drones, SniperWeapon, PhasicShield, Cloak, Overclock, CommandShield, PlasmaWeb, HyperPropulsion, EmSurge,
}

impl ShipRole {
    pub const ALL: [ShipRole; 9] = [
        Self::Engineer, Self::LongRange, Self::Guard, Self::Tackler, Self::Gunship, Self::Command,
        Self::CovertOps, Self::Recon, Self::Ecm,
    ];
    /// relation `role-has-size`
    pub fn size(self) -> ShipSize {
        match self {
            Self::Engineer | Self::LongRange | Self::Guard => ShipSize::Frigate,
            Self::Tackler | Self::Gunship | Self::Command => ShipSize::Fighter,
            Self::CovertOps | Self::Recon | Self::Ecm => ShipSize::Interceptor,
        }
    }
    /// relation `role-has-special`
    pub fn special(self) -> SpecialModuleKind {
        match self {
            Self::Engineer => SpecialModuleKind::Drones,
            Self::LongRange => SpecialModuleKind::SniperWeapon,
            Self::Guard => SpecialModuleKind::PhasicShield,
            Self::Tackler => SpecialModuleKind::Cloak,
            Self::Gunship => SpecialModuleKind::Overclock,
            Self::Command => SpecialModuleKind::CommandShield,
            Self::CovertOps => SpecialModuleKind::PlasmaWeb,
            Self::Recon => SpecialModuleKind::HyperPropulsion,
            Self::Ecm => SpecialModuleKind::EmSurge,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DamageKind { Electromagnetic, Kinetic, Thermic }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PassiveSlotType { Shield, Armor, Capacitor, Motor, Computer }

impl PassiveSlotType {
    pub const ALL: [PassiveSlotType; 5] = [Self::Shield, Self::Armor, Self::Capacitor, Self::Motor, Self::Computer];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Activation { Press, Toggle }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GameModeCategory { Pvp, Pve, OpenWorld }

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub shield_hp: f32,
    pub armor_hp: f32,
    pub energy: f32,
    pub energy_regen: f32,
    pub speed: f32,
    pub agility: f32,
}

/// Additive deltas; every field defaults to 0.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StatModifiers {
    pub shield_hp: f32,
    pub armor_hp: f32,
    pub energy: f32,
    pub energy_regen: f32,
    pub speed: f32,
    pub agility: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageType {
    pub id: DamageKind,
    pub vs_shield: f32,
    pub vs_armor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialModule {
    pub id: SpecialModuleKind,
    pub name: String,
    pub activation: Activation,
    pub effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipModel {
    pub id: Id,
    pub name: String,
    pub role: ShipRole,
    pub price: u32,
    pub base: Stats,
    pub slots: BTreeMap<PassiveSlotType, u8>,
    pub stock_weapon_id: Id,
    pub stock_missile_id: Id,
}

impl ShipModel {
    pub fn size(&self) -> ShipSize { self.role.size() }
    pub fn slot_count(&self, t: PassiveSlotType) -> u8 { self.slots.get(&t).copied().unwrap_or(0) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassiveModule {
    pub id: Id,
    pub name: String,
    pub module_type: PassiveSlotType,
    pub model_code: String,
    pub modifiers: StatModifiers,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ActivationFlow {
    OneShot { energy_cost: f32, cooldown_s: f32 },
    Ongoing { energy_per_s: f32, cooldown_s: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveModule {
    pub id: Id,
    pub name: String,
    /// empty = any role
    pub allowed_roles: Vec<ShipRole>,
    pub flow: ActivationFlow,
    pub effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub id: Id,
    pub name: String,
    pub size: ShipSize,
    pub damage_type: DamageKind,
    pub damage_per_shot: f32,
    pub fire_rate: f32,
    pub heat_per_shot: f32,
    pub cooling_per_s: f32,
    pub overheat_penalty_s: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissileModel {
    pub id: Id,
    pub name: String,
    pub damage_type: DamageKind,
    pub damage: f32,
    pub speed: f32,
    pub ammo: u8,
    pub reload_s: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMode {
    pub id: Id,
    pub name: String,
    pub category: GameModeCategory,
    pub respawn_allowed: bool,
}

// ---- player-owned, mutable ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loadout {
    pub passive_module_ids: Vec<Id>,
    pub active_module_ids: Vec<Id>,
    pub weapon_id: Id,
    pub missile_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerShip {
    pub id: Id,
    pub owner_id: Id,
    pub ship_model_id: Id,
    pub loadout: Loadout,
}

pub const HANGAR_SLOTS: usize = 4;
pub const MAX_ACTIVE_MODULES: usize = 4;
pub const MAX_SLOTS_PER_TYPE: u8 = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hangar {
    pub owner_id: Id,
    pub slots: [Option<Id>; HANGAR_SLOTS],
}

impl Hangar {
    /// C11
    pub fn validate(&self, owned: &[PlayerShip]) -> Vec<String> {
        let mut errs = Vec::new();
        let mut seen = HashSet::new();
        for id in self.slots.iter().flatten() {
            if !seen.insert(id) { errs.push(format!("hangar: {id} in two slots")); }
            match owned.iter().find(|s| &s.id == id) {
                None => errs.push(format!("hangar: unknown ship {id}")),
                Some(s) if s.owner_id != self.owner_id => errs.push(format!("hangar: {id} not owned by {}", self.owner_id)),
                _ => {}
            }
        }
        errs
    }
}

// ---- catalog ----

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub damage_types: Vec<DamageType>,
    pub special_modules: Vec<SpecialModule>,
    pub ship_models: Vec<ShipModel>,
    pub passive_modules: Vec<PassiveModule>,
    pub active_modules: Vec<ActiveModule>,
    pub weapons: Vec<Weapon>,
    pub missiles: Vec<MissileModel>,
    pub game_modes: Vec<GameMode>,
}

impl Catalog {
    pub fn load(dir: &Path) -> Result<Self, String> {
        fn read<T: for<'de> Deserialize<'de>>(dir: &Path, name: &str) -> Result<T, String> {
            let p = dir.join(name);
            let s = std::fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))?;
            serde_json::from_str(&s).map_err(|e| format!("{}: {e}", p.display()))
        }
        Ok(Self {
            damage_types: read(dir, "damage_types.json")?,
            special_modules: read(dir, "special_modules.json")?,
            ship_models: read(dir, "ship_models.json")?,
            passive_modules: read(dir, "passive_modules.json")?,
            active_modules: read(dir, "active_modules.json")?,
            weapons: read(dir, "weapons.json")?,
            missiles: read(dir, "missiles.json")?,
            game_modes: read(dir, "game_modes.json")?,
        })
    }

    pub fn ship_model(&self, id: &str) -> Option<&ShipModel> { self.ship_models.iter().find(|x| x.id == id) }
    pub fn passive(&self, id: &str) -> Option<&PassiveModule> { self.passive_modules.iter().find(|x| x.id == id) }
    pub fn active(&self, id: &str) -> Option<&ActiveModule> { self.active_modules.iter().find(|x| x.id == id) }
    pub fn weapon(&self, id: &str) -> Option<&Weapon> { self.weapons.iter().find(|x| x.id == id) }
    pub fn missile(&self, id: &str) -> Option<&MissileModel> { self.missiles.iter().find(|x| x.id == id) }
    pub fn damage(&self, k: DamageKind) -> Option<&DamageType> { self.damage_types.iter().find(|x| x.id == k) }

    /// C1, C2, C9 and positivity facets on catalog data.
    pub fn validate(&self) -> Vec<String> {
        let mut e = Vec::new();
        fn unique<'a>(e: &mut Vec<String>, fam: &str, ids: impl Iterator<Item = &'a str>) {
            let mut seen = HashSet::new();
            for id in ids { if !seen.insert(id) { e.push(format!("{fam}: duplicate id {id}")); } }
        }
        unique(&mut e, "ship-model", self.ship_models.iter().map(|x| x.id.as_str()));
        unique(&mut e, "passive-module", self.passive_modules.iter().map(|x| x.id.as_str()));
        unique(&mut e, "active-module", self.active_modules.iter().map(|x| x.id.as_str()));
        unique(&mut e, "weapon", self.weapons.iter().map(|x| x.id.as_str()));
        unique(&mut e, "missile-model", self.missiles.iter().map(|x| x.id.as_str()));
        unique(&mut e, "game-mode", self.game_modes.iter().map(|x| x.id.as_str()));

        // C9: three damage types with the documented shape
        for k in [DamageKind::Electromagnetic, DamageKind::Kinetic, DamageKind::Thermic] {
            match self.damage(k) {
                None => e.push(format!("damage-type: missing {k:?}")),
                Some(d) => {
                    if d.vs_shield <= 0.0 || d.vs_armor <= 0.0 { e.push(format!("damage-type {k:?}: multipliers must be > 0")); }
                    let ok = match k {
                        DamageKind::Electromagnetic => d.vs_shield > 1.0 && d.vs_armor < 1.0,
                        DamageKind::Kinetic => d.vs_armor > 1.0 && d.vs_shield < 1.0,
                        DamageKind::Thermic => (d.vs_shield - d.vs_armor).abs() < 0.2,
                    };
                    if !ok { e.push(format!("damage-type {k:?}: violates C9 profile")); }
                }
            }
        }
        // C17: one special per role, all present
        for r in ShipRole::ALL {
            if !self.special_modules.iter().any(|s| s.id == r.special()) {
                e.push(format!("special-module: missing {:?} for {r:?}", r.special()));
            }
        }
        for m in &self.ship_models {
            if m.price == 0 { e.push(format!("ship-model {}: price must be > 0", m.id)); }
            let b = &m.base;
            if b.armor_hp <= 0.0 || b.energy <= 0.0 || b.speed <= 0.0 || b.agility <= 0.0 || b.shield_hp < 0.0 || b.energy_regen < 0.0 {
                e.push(format!("ship-model {}: base stats out of range", m.id));
            }
            for (t, n) in &m.slots {
                if *n > MAX_SLOTS_PER_TYPE { e.push(format!("ship-model {}: {t:?} slots {n} > {MAX_SLOTS_PER_TYPE}", m.id)); }
            }
            match self.weapon(&m.stock_weapon_id) {
                None => e.push(format!("ship-model {}: unknown stock weapon {}", m.id, m.stock_weapon_id)),
                Some(w) if w.size != m.size() => e.push(format!("ship-model {}: stock weapon {} is {:?}, ship is {:?}", m.id, w.id, w.size, m.size())),
                _ => {}
            }
            if self.missile(&m.stock_missile_id).is_none() {
                e.push(format!("ship-model {}: unknown stock missile {}", m.id, m.stock_missile_id));
            }
        }
        for w in &self.weapons {
            if w.damage_per_shot <= 0.0 || w.fire_rate <= 0.0 || w.cooling_per_s <= 0.0
                || !(w.heat_per_shot > 0.0 && w.heat_per_shot <= 1.0) || w.overheat_penalty_s < 0.0 {
                e.push(format!("weapon {}: facet violation", w.id));
            }
        }
        for m in &self.missiles {
            if m.damage <= 0.0 || m.speed <= 0.0 || m.ammo == 0 || m.reload_s <= 0.0 { e.push(format!("missile-model {}: facet violation", m.id)); }
        }
        for a in &self.active_modules {
            let bad = match a.flow {
                ActivationFlow::OneShot { energy_cost, cooldown_s } => energy_cost <= 0.0 || cooldown_s < 0.0,
                ActivationFlow::Ongoing { energy_per_s, cooldown_s } => energy_per_s <= 0.0 || cooldown_s < 0.0,
            };
            if bad { e.push(format!("active-module {}: facet violation", a.id)); }
        }
        e
    }

    /// C4, C6, C7, C8 (+C5 via effective_stats) for one player ship.
    pub fn validate_loadout(&self, ship: &PlayerShip) -> Vec<String> {
        let mut e = Vec::new();
        let Some(model) = self.ship_model(&ship.ship_model_id) else {
            return vec![format!("player-ship {}: unknown model {}", ship.id, ship.ship_model_id)];
        };
        let lo = &ship.loadout;

        // C4
        let mut used: BTreeMap<PassiveSlotType, u8> = BTreeMap::new();
        for id in &lo.passive_module_ids {
            match self.passive(id) {
                None => e.push(format!("unknown passive {id}")),
                Some(p) => *used.entry(p.module_type).or_default() += 1,
            }
        }
        for t in PassiveSlotType::ALL {
            let (u, n) = (used.get(&t).copied().unwrap_or(0), model.slot_count(t));
            if u > n { e.push(format!("{t:?}: {u} equipped, {n} slots")); }
        }
        // C6
        if lo.active_module_ids.len() > MAX_ACTIVE_MODULES { e.push(format!("{} active modules > {MAX_ACTIVE_MODULES}", lo.active_module_ids.len())); }
        for id in &lo.active_module_ids {
            match self.active(id) {
                None => e.push(format!("unknown active {id}")),
                Some(a) if !a.allowed_roles.is_empty() && !a.allowed_roles.contains(&model.role) => {
                    e.push(format!("active {id} not allowed for {:?}", model.role))
                }
                _ => {}
            }
        }
        // C7
        match self.weapon(&lo.weapon_id) {
            None => e.push(format!("unknown weapon {}", lo.weapon_id)),
            Some(w) if w.size != model.size() => e.push(format!("weapon {} is {:?}, ship is {:?}", w.id, w.size, model.size())),
            _ => {}
        }
        // C8
        if self.missile(&lo.missile_id).is_none() { e.push(format!("unknown missile {}", lo.missile_id)); }
        // C5
        if let Some(s) = self.effective_stats(ship) {
            if s.armor_hp <= 0.0 || s.energy <= 0.0 || s.speed <= 0.0 || s.agility <= 0.0 || s.shield_hp < 0.0 || s.energy_regen < 0.0 {
                e.push("effective stats out of range".into());
            }
        }
        e
    }

    /// Q7 / C5: base + Σ passive modifiers. None if the model or a passive is unknown.
    pub fn effective_stats(&self, ship: &PlayerShip) -> Option<Stats> {
        let mut s = self.ship_model(&ship.ship_model_id)?.base;
        for id in &ship.loadout.passive_module_ids {
            let m = self.passive(id)?.modifiers;
            s.shield_hp += m.shield_hp;
            s.armor_hp += m.armor_hp;
            s.energy += m.energy;
            s.energy_regen += m.energy_regen;
            s.speed += m.speed;
            s.agility += m.agility;
        }
        Some(s)
    }

    /// C10: a freshly bought ship.
    pub fn buy(&self, model_id: &str, owner_id: &str, ship_id: &str) -> Option<PlayerShip> {
        let m = self.ship_model(model_id)?;
        Some(PlayerShip {
            id: ship_id.into(),
            owner_id: owner_id.into(),
            ship_model_id: m.id.clone(),
            loadout: Loadout {
                passive_module_ids: vec![],
                active_module_ids: vec![],
                weapon_id: m.stock_weapon_id.clone(),
                missile_id: m.stock_missile_id.clone(),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog() -> Catalog {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("instances");
        Catalog::load(&dir).expect("instances load")
    }

    #[test]
    fn instances_satisfy_catalog_constraints() {
        let c = catalog();
        assert_eq!(c.validate(), Vec::<String>::new());
        assert_eq!(c.ship_models.len(), 9);
        assert_eq!(c.game_modes.len(), 4);
    }

    #[test]
    fn bought_ship_is_valid_and_loadout_rules_hold() {
        let c = catalog();
        let mut ship = c.buy("anaconda", "user-1", "ps-1").unwrap();
        assert_eq!(c.validate_loadout(&ship), Vec::<String>::new());

        // C4: 3 shield slots on anaconda, 4 shield modules must fail
        ship.loadout.passive_module_ids = vec!["shield-extender".into(); 4];
        assert_eq!(c.validate_loadout(&ship).len(), 1);
        ship.loadout.passive_module_ids = vec!["shield-extender".into(); 3];
        assert!(c.validate_loadout(&ship).is_empty());
        assert_eq!(c.effective_stats(&ship).unwrap().shield_hp, 9000.0 + 3.0 * 1500.0);

        // C6: tackler-only module on an engineer
        ship.loadout.active_module_ids = vec!["engine-suppressor".into()];
        assert_eq!(c.validate_loadout(&ship).len(), 1);
        ship.loadout.active_module_ids = vec!["remote-repair".into(), "shield-boost".into()];
        assert!(c.validate_loadout(&ship).is_empty());

        // C7: fighter weapon on a frigate
        ship.loadout.weapon_id = "pulse-laser".into();
        assert_eq!(c.validate_loadout(&ship).len(), 1);
    }

    #[test]
    fn hangar_rules() {
        let c = catalog();
        let a = c.buy("hydra", "user-1", "ps-a").unwrap();
        let b = c.buy("kite", "user-2", "ps-b").unwrap();
        let h = Hangar { owner_id: "user-1".into(), slots: [Some("ps-a".into()), Some("ps-a".into()), Some("ps-b".into()), None] };
        let errs = h.validate(&[a, b]);
        assert_eq!(errs.len(), 2, "{errs:?}"); // duplicate + foreign ship
    }

    #[test]
    fn role_tables_cover_every_role() {
        let mut specials = HashSet::new();
        for r in ShipRole::ALL { assert!(specials.insert(r.special())); }
        assert_eq!(specials.len(), 9);
    }
}
