use eframe::egui::{Color32, Pos2, Rect, Rounding, Ui, Vec2};

use crate::objects::Object;

pub struct Wall {
    rect: Rect,
}

impl Default for Wall {
    fn default() -> Self {
        Wall {
            rect: Rect::ZERO,
        }
    }
}

impl Object for Wall {
    fn draw(&self, ui: &Ui) {
        ui.painter().rect_filled(self.rect, Rounding::default(), Color32::RED);
    }
    fn rect(&self) -> Rect {
        self.rect
    }
}

impl Wall {
    pub fn new(rect: Rect) -> Self {
        Wall {rect}
    }
    pub fn from_pos_size(pos: Pos2, size: Vec2) -> Self {
        let rect = Rect::from_min_size(pos, size);
        Wall::new(rect)
    }
    pub fn debug_level() -> Vec<Box<dyn Object>> {
        let mut vector: Vec<Box<dyn Object>> = vec![];

        let platform_pos = Pos2::new(0.0, 412.0);
        let platform_vec = Vec2::new(512.0 * 1000.0, 25.0);
        let platform = Wall::from_pos_size(platform_pos, platform_vec);
        vector.push(Box::new(platform));

        return vector;
    }
}