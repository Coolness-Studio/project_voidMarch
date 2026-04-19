use core::panic;

use macroquad::prelude::*;

mod ui;
use ui::{draw_menu};
#[derive(Debug)]
enum STATE {
    MENU,
    SETTINGS,
    GAMEPLAY(i32),
}

#[macroquad::main("Project: VoidMarch")]
pub async fn main() {
    let state: STATE = STATE::MENU;

    loop {
        clear_background(BLACK);

        match state {
            STATE::MENU => draw_menu(),
            _ => {
                panic!("not implemented {:?}", state)
            }
        }

        next_frame().await
    }
}

