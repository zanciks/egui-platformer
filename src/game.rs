use std::time::{Instant, Duration};
use eframe::{App, Frame};
use eframe::egui::Context;

use crate::level::Level;

static mut PREVIOUS_TIME: Option<Instant> = None;

pub struct Game {
    delta_time: f32,
    level: Level
}

impl Default for Game {
    fn default() -> Self {
        Game {
            delta_time: 0.0,
            level: Level::default(),
        }
    }
}

impl App for Game {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Updates which don't require ui
        self.calculate_delta_time();
        self.level.update(ctx, self.delta_time);

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