mod game;
mod objects;
mod player;

fn main() -> Result<(), eframe::Error> {
    run_app()
}

fn run_app() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::Vec2::splat(512.0)),
        vsync: false,
        ..Default::default()
    };
    eframe::run_native(
        "Game!",
        options,
        Box::new(|_cc| {
            Box::<game::Game>::default()
        }),
    )
}