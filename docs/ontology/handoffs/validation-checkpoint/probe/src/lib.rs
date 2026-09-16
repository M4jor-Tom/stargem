// Disposable observation tests: some PASS by proving a current validation hole.
// These are not proposed post-fix regression expectations.
#[cfg(test)]
mod tests {
    use ontology::*;
    use serde_json::{json, Value};
    use std::{collections::HashSet, path::Path};

    const DATA: &str = "/home/theta/repos/stargem.nix/ontology/instances";
    fn catalog() -> Catalog { Catalog::load(Path::new(DATA)).unwrap() }
    fn stock(c: &Catalog) -> PlayerShip { c.buy("anaconda", "owner", "ship").unwrap() }
    fn stats_ok(s: Stats) -> bool {
        let a = [s.shield_hp, s.armor_hp, s.energy, s.energy_regen, s.speed, s.agility];
        a.iter().all(|x| x.is_finite()) && s.shield_hp >= 0. && s.energy_regen >= 0.
            && s.armor_hp > 0. && s.energy > 0. && s.speed > 0. && s.agility > 0.
    }

    #[test]
    fn seed_all_stock_and_enum_coverage() {
        let c = catalog();
        assert!(c.validate().is_empty());
        assert_eq!(c.damage_types.len()+c.special_modules.len()+c.ship_models.len()+c.passive_modules.len()+c.active_modules.len()+c.weapons.len()+c.missiles.len()+c.game_modes.len(), 46);
        for m in &c.ship_models {
            let s = c.buy(&m.id, "owner", "ship").unwrap();
            assert!(c.validate_loadout(&s).is_empty());
            assert!(stats_ok(c.effective_stats(&s).unwrap()));
            assert!(s.loadout.active_module_ids.is_empty() && s.loadout.passive_module_ids.is_empty());
            assert!(c.damage(c.weapon(&s.loadout.weapon_id).unwrap().damage_type).is_some());
            assert!(c.damage(c.missile(&s.loadout.missile_id).unwrap().damage_type).is_some());
        }
        assert_eq!(c.ship_models.iter().map(|m| m.role).collect::<HashSet<_>>().len(), 9);
        assert_eq!(c.ship_models.iter().map(|m| m.size()).collect::<HashSet<_>>().len(), 3);
        assert_eq!(c.special_modules.iter().map(|m| m.id).collect::<HashSet<_>>().len(), 9);
        assert_eq!(c.passive_modules.iter().map(|m| m.module_type).collect::<HashSet<_>>().len(), 5);
        assert!(c.active_modules.iter().any(|a| matches!(a.flow, ActivationFlow::OneShot {..})));
        assert!(c.active_modules.iter().any(|a| matches!(a.flow, ActivationFlow::Ongoing {..})));
        for a in [Activation::Press, Activation::Toggle] { assert!(c.special_modules.iter().any(|m| m.activation == a)); }
        for g in [GameModeCategory::Pvp, GameModeCategory::Pve, GameModeCategory::OpenWorld] { assert!(c.game_modes.iter().any(|m| m.category == g)); }
    }

    #[test]
    fn seed_every_capacity_legal_passive_multiset_has_valid_stats() {
        fn enumerate(c: &Catalog, s: &mut PlayerShip, i: usize, total: &mut usize) {
            if i == c.passive_modules.len() {
                assert!(c.validate_loadout(s).is_empty());
                assert!(stats_ok(c.effective_stats(s).unwrap()));
                *total += 1;
                return;
            }
            let p = &c.passive_modules[i];
            let cap = c.ship_model(&s.ship_model_id).unwrap().slot_count(p.module_type) as usize;
            let used = s.loadout.passive_module_ids.iter().filter(|id| c.passive(id).unwrap().module_type == p.module_type).count();
            let before = s.loadout.passive_module_ids.len();
            for n in 0..=cap-used {
                s.loadout.passive_module_ids.resize(before+n, p.id.clone());
                enumerate(c, s, i+1, total);
            }
            s.loadout.passive_module_ids.truncate(before);
        }
        let c = catalog();
        let mut total = 0;
        for m in &c.ship_models { enumerate(&c, &mut c.buy(&m.id, "owner", "ship").unwrap(), 0, &mut total); }
        println!("capacity-legal passive multisets checked = {total}");
        assert!(total > 0);
    }

    #[test]
    fn passive_count_256_panics_in_debug_or_is_accepted_in_release() {
        let c = catalog();
        let mut s = stock(&c);
        s.loadout.passive_module_ids = vec!["shield-extender".into(); 256];
        let r = std::panic::catch_unwind(|| c.validate_loadout(&s));
        if cfg!(debug_assertions) { assert!(r.is_err()); println!("256 repeats: caught integer-overflow panic"); }
        else { assert!(r.unwrap().is_empty()); println!("256 repeats: validator accepted; shield={}", c.effective_stats(&s).unwrap().shield_hp); }
    }

    #[test]
    fn finite_json_1e39_becomes_infinity_and_passes_into_flight() {
        let text = std::fs::read_to_string(format!("{DATA}/ship_models.json")).unwrap();
        let text = text.replacen("\"speed\": 180", "\"speed\": 1e39", 1);
        let mut c = catalog();
        c.ship_models = serde_json::from_str(&text).unwrap();
        assert_eq!(c.ship_models[0].base.speed, f32::INFINITY);
        assert!(c.validate().is_empty());
        let s = stock(&c);
        assert!(c.validate_loadout(&s).is_empty());
        let p = sim::FlightParams::from(&c.effective_stats(&s).unwrap());
        let mut b = sim::Body::at(sim::glam::Vec3::ZERO, 1.);
        sim::step(&mut b, &p, &sim::Input { throttle: 1., ..Default::default() }, 1./60.);
        assert!(!b.position.is_finite());
        println!("JSON 1e39 -> speed={} -> position={:?}", p.max_speed, b.position);
    }

    #[test]
    fn runtime_nan_facets_and_passive_modifiers_are_accepted() {
        let mut c = catalog();
        c.ship_models[0].base.armor_hp = f32::NAN;
        c.weapons[0].fire_rate = f32::NAN;
        c.missiles[0].reload_s = f32::NAN;
        c.active_modules[0].flow = ActivationFlow::OneShot { energy_cost: f32::NAN, cooldown_s: f32::NAN };
        c.passive_modules[0].modifiers.shield_hp = f32::NAN;
        assert!(c.validate().is_empty());
        let mut s = stock(&c);
        s.loadout.passive_module_ids.push("shield-extender".into());
        assert!(c.validate_loadout(&s).is_empty());
        assert!(!stats_ok(c.effective_stats(&s).unwrap()));
        // Heat's conjunction and damage profile comparisons do reject NaN.
        c.weapons[0].heat_per_shot = f32::NAN;
        assert!(!c.validate().is_empty());
    }

    #[test]
    fn finite_legal_deltas_can_overflow_the_effective_sum() {
        let mut c = catalog();
        c.passive_modules[0].modifiers.shield_hp = f32::MAX;
        assert!(c.validate().is_empty());
        let mut s = stock(&c);
        s.loadout.passive_module_ids = vec!["shield-extender".into(); 2];
        assert!(c.validate_loadout(&s).is_empty());
        assert_eq!(c.effective_stats(&s).unwrap().shield_hp, f32::INFINITY);
    }

    #[test]
    fn thermal_100_100_passes_despite_not_near_one() {
        let mut c = catalog();
        let d = c.damage_types.iter_mut().find(|d| d.id == DamageKind::Thermic).unwrap();
        d.vs_shield = 100.; d.vs_armor = 100.;
        assert!(c.validate().is_empty());
    }

    #[test]
    fn duplicate_damage_and_special_are_not_rejected() {
        let mut c = catalog();
        let mut d = c.damage_types[0].clone();
        d.vs_shield = -1.;
        c.damage_types.push(d);
        let mut s = c.special_modules[0].clone();
        s.effect = "Conflicting effect".into();
        c.special_modules.push(s);
        assert!(c.validate().is_empty());
        assert_eq!(c.damage(DamageKind::Electromagnetic).unwrap().vs_shield, 1.5);
        c.damage_types.swap(0, 3);
        assert!(!c.validate().is_empty());
    }

    #[test]
    fn effective_stats_returns_invalid_result_but_loadout_check_rejects_it() {
        let mut c = catalog();
        c.passive_modules.iter_mut().find(|p| p.id == "light-armor").unwrap().modifiers.armor_hp = -7000.;
        assert!(c.validate().is_empty());
        let mut s = stock(&c);
        s.loadout.passive_module_ids.push("light-armor".into());
        assert_eq!(c.effective_stats(&s).unwrap().armor_hp, -1000.);
        assert_eq!(c.validate_loadout(&s), ["effective stats out of range"]);
    }

    #[test]
    fn catalog_load_and_buy_do_not_validate_stock() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../mutated-catalog");
        std::fs::create_dir_all(&dir).unwrap();
        for file in std::fs::read_dir(DATA).unwrap() {
            let file = file.unwrap();
            std::fs::copy(file.path(), dir.join(file.file_name())).unwrap();
        }
        let path = dir.join("ship_models.json");
        let mut models: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        models[0]["price"] = json!(0);
        models[0]["base"]["speed"] = json!(-1);
        models[0]["stock_weapon_id"] = json!("missing");
        std::fs::write(path, serde_json::to_vec(&models).unwrap()).unwrap();
        let c = Catalog::load(&dir).unwrap();
        assert_eq!(c.validate().len(), 3);
        let s = stock(&c);
        assert_eq!(c.effective_stats(&s).unwrap().speed, -1.);
        assert_eq!(c.validate_loadout(&s).len(), 2);
    }

    #[test]
    fn required_unknown_fields_id_grammar_and_map_conventions() {
        let c = catalog();
        let mut v = serde_json::to_value(&c.ship_models[0]).unwrap();
        v["base"].as_object_mut().unwrap().remove("speed");
        assert!(serde_json::from_value::<ShipModel>(v).is_err());
        let m: StatModifiers = serde_json::from_str(r#"{"sheild_hp":1500}"#).unwrap();
        assert_eq!(m, StatModifiers::default());
        let mut c = c;
        c.game_modes[0].id = "BAD ID".into();
        assert!(c.validate().is_empty());
        c.game_modes[0].id.clear();
        assert!(c.validate().is_empty());
        let mut v = serde_json::to_value(&c.ship_models[0]).unwrap();
        v["slots"] = json!({});
        let m: ShipModel = serde_json::from_value(v.clone()).unwrap();
        assert_eq!(m.slot_count(PassiveSlotType::Shield), 0);
        v["slots"] = json!({"sheild": 1});
        assert!(serde_json::from_value::<ShipModel>(v.clone()).is_err());
        v["slots"] = json!({"shield": 256});
        assert!(serde_json::from_value::<ShipModel>(v).is_err());
        let dup: ShipModel = serde_json::from_str(&serde_json::to_string(&c.ship_models[0]).unwrap().replace("\"shield\":3", "\"shield\":3,\"shield\":1")).unwrap();
        assert_eq!(dup.slot_count(PassiveSlotType::Shield), 1);
        assert!(serde_json::from_str::<ShipRole>("\"unknown\"").is_err());
        assert!(serde_json::from_str::<Stats>(r#"{"shield_hp":0,"armor_hp":1,"energy":1,"energy_regen":0,"speed":NaN,"agility":1}"#).is_err());
    }

    #[test]
    fn rejection_controls_for_refs_multiplicity_ranges_and_hangar() {
        let c = catalog();
        let mut bad = c.clone(); bad.ship_models.push(bad.ship_models[0].clone()); assert!(!bad.validate().is_empty());
        let mut bad = c.clone(); bad.ship_models[0].slots.insert(PassiveSlotType::Shield, 4); assert!(!bad.validate().is_empty());
        let mut bad = c.clone(); bad.ship_models[0].stock_weapon_id = "pulse-laser".into(); assert!(!bad.validate().is_empty());
        let mut bad = c.clone(); bad.ship_models[0].stock_missile_id = "missing".into(); assert!(!bad.validate().is_empty());
        let mut bad = c.clone(); bad.damage_types.pop(); assert!(!bad.validate().is_empty());
        let mut bad = c.clone(); bad.special_modules.pop(); assert!(!bad.validate().is_empty());
        let mut s = stock(&c);
        s.loadout.active_module_ids = vec!["shield-boost".into(); 4]; assert!(c.validate_loadout(&s).is_empty());
        s.loadout.active_module_ids.push("shield-boost".into()); assert!(!c.validate_loadout(&s).is_empty());
        for field in ["model", "passive", "active", "weapon", "missile"] {
            let mut s = stock(&c);
            match field {
                "model" => s.ship_model_id = "missing".into(),
                "passive" => s.loadout.passive_module_ids.push("missing".into()),
                "active" => s.loadout.active_module_ids.push("missing".into()),
                "weapon" => s.loadout.weapon_id = "missing".into(),
                _ => s.loadout.missile_id = "missing".into(),
            }
            assert!(!c.validate_loadout(&s).is_empty(), "{field}");
        }
        let h = Hangar { owner_id: "owner".into(), slots: [Some("ship".into()), None, None, None] };
        assert!(h.validate(&[stock(&c)]).is_empty());
        assert_eq!(h.validate(&[]).len(), 1);
        let mut foreign = stock(&c); foreign.owner_id = "other".into();
        assert_eq!(h.validate(&[foreign.clone()]).len(), 1);
        // Requires a unique-ID supplied store: ambiguous rows are first-match wins.
        assert!(h.validate(&[stock(&c), foreign.clone()]).is_empty());
        assert_eq!(h.validate(&[foreign, stock(&c)]).len(), 1);
        assert!(serde_json::from_str::<Hangar>(r#"{"owner_id":"owner","slots":[null,null,null]}"#).is_err());
        assert!(serde_json::from_str::<Hangar>(r#"{"owner_id":"owner","slots":[null,null,null,null,null]}"#).is_err());
        let mut v = serde_json::to_value(stock(&c).loadout).unwrap();
        v.as_object_mut().unwrap().remove("missile_id"); assert!(serde_json::from_value::<Loadout>(v).is_err());
    }
}
