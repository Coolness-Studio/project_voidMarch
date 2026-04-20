/// This file handles all level logic
use macroquad::prelude::*;
use tellus_level::*;

mod tiles;
pub use tiles::*;

use super::assets::*;

pub fn draw_level(id: u8, assets: &Assets) {
    match id {
        0 => {
            let mut level = get_level(id);

            for y in 0..level.height {
                for x in 0..level.width {
                    let tile = level.tile(LayerKind::Ground, x, y).unwrap();

                    // Not sure if this should be in assets now that I think about it
                    let x_position = (x * TILE_SIZE as u16) as f32;
                    let y_position = (y * TILE_SIZE as u16) as f32;

                    match tile {
                        0u16 => draw_texture(&assets.level.base_tile, x_position, y_position, WHITE),
                        1u16 => draw_texture(&assets.level.grass, x_position, y_position, WHITE),
                        _ => panic!("Tile type not known!"),
                    }
                }
            }

        }

        1 => {

        }

        _ => panic!("Level does not exist"),
    }
}

pub fn get_level(id: u8) -> Level {
    let path = format!("assets/levels/{}.tlvl", id);

    // We might want to have it return a Result instead to not make this an unrecoverable error
    Level::load_from_file(&path).expect("Level does not exist!")
}



