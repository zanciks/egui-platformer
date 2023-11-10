use std::time::{Instant, Duration};
use eframe::{App, Frame};
use eframe::egui::{self, Context};

use crate::player::Player;
use crate::objects::Object;
use crate::objects::wall::Wall;

static mut PREVIOUS_TIME: Option<Instant> = None;

pub struct Game {
    delta_time: f32,
    player: Player,
    objects: Vec<Box<dyn Object>>
}

impl Default for Game {
    fn default() -> Self {
        Game {
            delta_time: 0.0,
            player: Player::default(),
            objects: Wall::debug_level(),
        }
    }
}

impl App for Game {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Updates which don't require ui
        self.calculate_delta_time();
        self.player.update(ctx, self.delta_time, &self.objects);

        // Updates which require ui (drawing/displaying)
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.label(format!("{}", self.player.velocity.x));
            ui.label(format!("FPS: {:.1}", 1.0 / self.delta_time));
            self.player.draw(ui);
            for object in &self.objects {
                object.draw(ui);
            }
        });

        ctx.request_repaint(); // simply starting the next frame without any input
    }
}

impl Game {
    fn calculate_delta_time(&mut self) {
        unsafe {
            if PREVIOUS_TIME.is_none() {PREVIOUS_TIME = Some(Instant::now());}
            let current_time = Instant::now();
            self.delta_time = match PREVIOUS_TIME {
                Some(previous_time) => current_time.duration_since(previous_time).as_secs_f32(),
                None => Duration::from_secs(0).as_secs_f32(),
            };

            PREVIOUS_TIME = Some(current_time);
        }
    }
}