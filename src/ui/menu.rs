/// This file defines the menu screen
use macroquad::prelude::*;
use crate::State;

pub fn draw_menu(state: &mut State) {
    let button_width = 240.0;
    let button_height = 64.0;
    let button_x = (screen_width() - button_width) / 2.0;
    let button_y = (screen_height() / 2.0) + 150.0;

    draw_text(
        "Project: Voidmarch",
        screen_width() / 2.0 - get_text_center("Project: VoidMarch", None, 64, 1.0, 0.0).x,
        (screen_height() / 2.0) - 120.0,
        64.0,
        WHITE,
    );

    draw_circle(screen_width() / 2.0, screen_height() / 2.0, 100.0, RED);
    draw_text(
        ":D",
        screen_width() / 2.0 - get_text_center(":D", None, 64, 1.0, 0.0).x,
        (screen_height() / 2.0) - get_text_center(":D", None, 64, 1.0, 0.0).y,
        64.0,
        WHITE,
    );

    let (mouse_x, mouse_y) = mouse_position();
    let hovered = mouse_x >= button_x
        && mouse_x <= button_x + button_width
        && mouse_y >= button_y
        && mouse_y <= button_y + button_height;

    let button_color = if hovered { DARKGREEN } else { GREEN };
    draw_rectangle(button_x, button_y, button_width, button_height, button_color);
    draw_text(
        "Play Level 0",
        button_x + 26.0,
        button_y + 42.0,
        36.0,
        WHITE,
    );

    if hovered && is_mouse_button_pressed(MouseButton::Left) {
        *state = State::Level(0);
    }
}
