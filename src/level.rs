use eframe::egui;
use eframe::egui::Context;
use crate::objects::Object;
use crate::objects::wall::Wall;
use crate::player::Player;

pub struct Level {
    player: Player,
    objects: Vec<Box<dyn Object>>,
}

impl Default for Level {
    fn default() -> Self { // this gives a debug/testing level
        Level {
            player: Player::default(),
            objects: Wall::debug_level(),
        }
    }
}

impl Level {
    pub fn update(&mut self, ctx: &Context, delta_time: f32) {
        self.player.update(ctx, delta_time, &self.objects);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("FPS: {:.1}", 1.0 / delta_time));
            self.player.draw(ui);
            for object in &self.objects {
                object.draw(ui);
            }
            ctx.input(|input| {
               if input.key_pressed(eframe::egui::Key::Escape) {*self = Self::new_level(0);}
            });
        });
    }
    fn new_level(level_index: u8) -> Self {
        match level_index {
            0 => Self::default(),
            _ => Self::default(), // if number isn't known, we use debug level
        }
    }
}