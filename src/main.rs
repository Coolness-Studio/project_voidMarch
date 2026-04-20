use macroquad::prelude::*;

mod ui;
use ui::{SettingsMenu, draw_menu, draw_settings};

mod levels;
use levels::draw_level;

mod assets;
use assets::*;

#[derive(Debug)]
enum State {
    Menu,
    Settings(SettingsMenu),
    Level(u8),
}

#[macroquad::main("Project: VoidMarch")]
pub async fn main() {
    let mut state: State = State::Menu;

    let assets = Assets::load().await;

    loop {
        clear_background(BLACK);

        match &mut state {
            State::Menu => draw_menu(&mut state),
            State::Settings(menu) => draw_settings(&menu),
            State::Level(id) => draw_level(*id, &assets),
        }

        next_frame().await
    }
}
