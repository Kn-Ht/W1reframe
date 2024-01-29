use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    TitleScreen,
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalState {
    pub screen_w: f32,
    pub screen_h: f32,
    pub fullscreen: bool,
}

impl GlobalState {
    fn update(&mut self) {
        self.screen_w = screen_width();
        self.screen_h = screen_height();
    }
    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
        set_fullscreen(self.fullscreen);
    }
}

pub struct Game {
    pub gamestate: GameState,
    pub state: GlobalState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            gamestate: GameState::TitleScreen,
            state: GlobalState::default(),
        }
    }
    /// Start the main loop by calling `callback` each frame
    pub async fn start_loop<F: Fn(&mut Self)>(&mut self, callback: F) -> ! {
        loop {
            self.state.update(); // 1. Update shared state
            callback(self); // 2. Run user-specified code
            next_frame().await; // 3. Render the frame
        }
    }
    /// Render the title screen and check if a button was pressed
    pub fn render_title_screen(&mut self) {
        const PADDING: f32 = 5.0;

        draw_rectangle_lines(
            PADDING,
            PADDING,
            self.state.screen_w - 2. * PADDING,
            self.state.screen_h - 2. * PADDING,
            2.0,
            LIGHTGRAY,
        );
    }
}
