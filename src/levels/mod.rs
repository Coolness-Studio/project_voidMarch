/// This file handles all level logic
use std::{error::Error, fmt};

use macroquad::prelude::*;
use tellus_level::{LayerKind, Level};

mod tiles;

mod lvl_error_handling;
use lvl_error_handling::*;

use super::assets::*;

pub fn draw_level(id: u8, level: &Level, assets: &Assets) -> Result<(), LevelError> {
    for y in 0..level.height {
        for x in 0..level.width {
            let tile = level
                .tile(LayerKind::Ground, x, y)
                .map_err(|source| LevelError::TileReadFailed { id, x, y, source })?;

            let x_position = f32::from(x) * f32::from(TILE_SIZE);
            let y_position = f32::from(y) * f32::from(TILE_SIZE);

            if let Some(texture) = level_tile_texture(assets, tile) {
                draw_texture(texture, x_position, y_position, WHITE);
            } else {
                draw_missing_tile(x_position, y_position);
            }
        }
    }

    Ok(())
}

fn level_tile_texture(assets: &Assets, tile: u16) -> Option<&Texture2D> {
    match tile {
        0 => Some(&assets.level.base_tile),
        1 => Some(&assets.level.grass),
        _ => None,
    }
}
