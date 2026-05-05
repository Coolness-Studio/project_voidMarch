/// This file handles all level logic
use macroquad::prelude::*;
use tellus_level::{LayerKind, Level};

mod tiles;

mod lvl_error_handling;
use lvl_error_handling::*;

use super::assets::*;

pub fn draw_level(id: u8, assets: &Assets) -> Result<(), LevelDrawError> {
    let level = assets
        .levels
        .get(id)
        .ok_or(LevelDrawError::UnknownLevel { id })?;

    draw_loaded_level(id, level, assets)
}

fn draw_loaded_level(id: u8, level: &Level, assets: &Assets) -> Result<(), LevelDrawError> {
    for y in 0..level.height {
        for x in 0..level.width {
            let tile = level
                .tile(LayerKind::Ground, x, y)
                .map_err(|source| LevelDrawError::TileReadFailed { id, x, y, source })?;

            let x_position = f32::from(x) * f32::from(TILE_SIZE);
            let y_position = f32::from(y) * f32::from(TILE_SIZE);

            if let Some(texture) = level_tile_texture(assets, tile) {
                draw_texture(texture, x_position, y_position, WHITE);
            }
        }
    }

    Ok(())
}

fn level_tile_texture(assets: &Assets, tile: u16) -> Option<&Texture2D> {
    match tile {
        0 => Some(&assets.tiles.base_tile),
        1 => Some(&assets.tiles.grass),
        _ => None,
    }
}
