use eframe::egui::{Pos2, Rect, Vec2};
use eframe::egui::{Context, Key, Ui, Rounding, Color32};

use crate::objects::wall::Wall;
use crate::objects::Object;

pub struct Player {
    // general information
    size: Vec2, // used for drawing and collision detection
    position: Pos2,
    velocity: Vec2,
    // horizontal specific motion
    speed: f32,
    max_speed: f32,
    friction: f32,
    // vertical specific motion
    gravity: f32,
    jump_strength: f32,
    is_on_ground: bool,
    //debug options
    keep_in_window: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            size: Vec2::splat(25.0),
            position: Pos2::default(),
            velocity: Vec2::default(),

            speed: 3072.0, // we can make this really big because we have a max-speed. If we want to be at max-speed instantly, set this to f32::MAX
            max_speed: 512.0,
            friction: 5.0, // this seems to work :shrug:

            gravity: 1024.0,
            jump_strength: 512.0,
            is_on_ground: false,

            keep_in_window: true,
        }
    }
}

impl Player { // TODO : MOVE UPDATE AND DRAW TO ENTITY TRAIT ONCE ADDED
    pub fn update(&mut self, ctx: &Context, delta_time: f32, objects: &Vec<Box<dyn Object>>) { // this function calls all player-functions which don't require ui
        if self.keep_in_window{self.keep_inside_window(ctx)}

        self.horizontal_movement(ctx, delta_time);
        self.vertical_movement(ctx, delta_time);

        self.solve_wall_collision(objects);
    }
    pub fn draw(&self, ui: &mut Ui) {
        let rect = Rect::from_min_size(self.position, Vec2::splat(25.0));
        ui.painter().rect_filled(rect, Rounding::default(), Color32::WHITE);
    }
    fn horizontal_movement(&mut self, ctx: &Context, delta_time: f32) {
        ctx.input(|input| {
            let direction = (input.key_down(Key::D) as i32 - input.key_down(Key::A) as i32) as f32;
            self.velocity.x += direction * delta_time * self.speed;
            self.velocity.x -= self.velocity.x * self.friction * delta_time;
            self.velocity.x = self.velocity.x.clamp(-self.max_speed, self.max_speed);
            self.position.x += self.velocity.x * delta_time;
        });
    }
    fn vertical_movement(&mut self, ctx: &Context, delta_time: f32) {
        self.velocity.y += self.gravity * delta_time;

        ctx.input(|input| {
            if input.key_pressed(Key::Space) && self.is_on_ground {
                self.velocity.y -= self.jump_strength;
                self.is_on_ground = false;
            }
        });

        self.position.y += self.velocity.y * delta_time;
    }
    fn collides_with_wall(&mut self, object: &Box<dyn Object>) -> bool {
        let self_rect = Rect::from_min_size(self.position, self.size);
        if self_rect.intersects(object.rect()) {true}
        else {false}
    }
    fn solve_wall_collision(&mut self, objects: &Vec<Box<dyn Object>>) {
        // this code was pretty much taken from https://stackoverflow.com/questions/5062833/detecting-the-direction-of-a-collision
        for object in objects {
            if self.collides_with_wall(&object) {
                let player_bottom = self.position.y + self.size.y;
                let object_bottom = object.rect().top() + object.rect().height();
                let player_right = self.position.x + self.size.x;
                let object_right = object.rect().left() + object.rect().width();

                let bottom_collision = object_bottom - self.position.y;
                let top_collision = player_bottom - object.rect().top();
                let left_collision = player_right - object.rect().left();
                let right_collision = object_right - self.position.x;

                if top_collision < bottom_collision && top_collision < left_collision && top_collision < right_collision {
                    self.velocity.y = self.velocity.y.min(0.0);
                    self.position.y = object.rect().top() - self.size.y;
                    self.is_on_ground = true;
                } else if bottom_collision < top_collision && bottom_collision < left_collision && bottom_collision < right_collision {
                    self.velocity.y = self.velocity.y.max(0.0);
                    self.position.y = object.rect().top() + object.rect().height();
                    self.is_on_ground = false;
                } else if left_collision < right_collision && left_collision < top_collision && left_collision < bottom_collision {
                    self.velocity.x = self.velocity.x.min(0.0);
                    self.position.x = object.rect().left() - self.size.x;
                    self.is_on_ground = false;
                } else if right_collision < left_collision && right_collision < top_collision && right_collision < bottom_collision {
                    self.velocity.x = self.velocity.x.max(0.0);
                    self.position.x = object.rect().left() + object.rect().width();
                    self.is_on_ground = false;
                } else {
                    self.is_on_ground = false;
                }
            }
        }
    }
    fn keep_inside_window(&mut self, ctx: &Context) {
        let mut vector: Vec<Box<dyn Object>> = vec![];
        let available_rect = ctx.available_rect();

        let top_wall = Wall::new(Rect::from_min_size(
            Pos2 { x: available_rect.min.x, y: available_rect.max.y },
            Vec2 { x: available_rect.width(), y: 1.0 },
        ));

        let right_wall = Wall::new(Rect::from_min_size(
            Pos2 { x: available_rect.max.x, y: available_rect.min.y },
            Vec2 { x: 1.0, y: available_rect.height() },
        ));

        let bottom_wall = Wall::new(Rect::from_min_size(
            Pos2 { x: available_rect.min.x, y: available_rect.min.y - 1.0 },
            Vec2 { x: available_rect.width(), y: 1.0 },
        ));

        let left_wall = Wall::new(Rect::from_min_size(
            Pos2 { x: available_rect.min.x - 1.0, y: available_rect.min.y },
            Vec2 { x: 1.0, y: available_rect.height() },
        ));

        vector.push(Box::new(top_wall));
        vector.push(Box::new(right_wall));
        vector.push(Box::new(bottom_wall));
        vector.push(Box::new(left_wall));

        self.solve_wall_collision(&vector);
    }
}