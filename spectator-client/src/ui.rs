use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::render::LiveWorldRes;

pub fn ui_system(mut ctx: EguiContexts, mut world_res: ResMut<LiveWorldRes>) {
    let ctx = ctx.ctx_mut();

    egui::SidePanel::left("players").resizable(false).show(ctx, |ui| {
        ui.heading("Players");
        let snapshot: Vec<(String, f32, f32)> = world_res.0.players.iter()
            .map(|(k, s)| (k.clone(), s.shield_hp, s.armor_hp))
            .collect();
        for (i, (k, sh, ar)) in snapshot.iter().enumerate() {
            let selected = i == world_res.0.follow_idx;
            let label = format!("{}  sh:{:.0}  ar:{:.0}", k, sh, ar);
            if ui.selectable_label(selected, label).clicked() {
                world_res.0.follow_idx = i;
            }
        }
    });

    egui::SidePanel::right("journal").resizable(false).show(ctx, |ui| {
        ui.heading(format!("Journal (tick {})", world_res.0.tick));
        egui::ScrollArea::vertical().show(ui, |ui| {
            for e in &world_res.0.journal {
                ui.label(format!("[{}] {}", e.tick, e.text));
            }
        });
    });
}