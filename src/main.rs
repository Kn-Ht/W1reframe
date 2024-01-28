use macroquad::prelude::*;

mod game;
use game::{Game, GameState};

fn window_conf() -> Conf {
    Conf {
        window_title: "W1reFrame".to_owned(),
        window_width: 800, window_height: 600,
        high_dpi: true,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    game.start_loop(|game| {
        clear_background(BLACK);

        if let GameState::TitleScreen = game.gamestate {
            game.render_title_screen();
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
                .show(egui_ctx, |ui| {
                    ui.label(format!("FPS: {}", get_fps()));
                    ui.label(format!("Screen size: {}x{}", game.state.screen_w, game.state.screen_h));
                    ui.label(format!("Key W: {:?}", is_key_down(KeyCode::W)));
                });
        });

        egui_macroquad::draw();
    }).await;
}
