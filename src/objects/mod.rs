use eframe::egui::{Rect, Ui};

pub mod wall;

pub trait Object {
    fn draw(&self, ui: &Ui);
    fn rect(&self) -> Rect; // bounding rect, or collision rect
}