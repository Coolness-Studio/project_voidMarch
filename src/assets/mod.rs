/// This file loads and exposes all assets to the rest of the codebase
pub mod definitions;
use definitions::*;

pub mod levels;
use levels::*;

mod constants;
pub use constants::*;

pub struct Assets {
    pub tiles: Tiles,
    pub enemies: Enemies,
    pub player_races: Races,
    pub levels: Levels,
}

impl Assets {
    pub async fn load() -> Self {
        let tiles = Tiles::load().await;
        let enemies = Enemies::load().await;
        let player_races = Races::load().await;
        let levels = Levels::load().await;

        Self {
            tiles,
            enemies,
            player_races,
            levels,
        }
    }
}


