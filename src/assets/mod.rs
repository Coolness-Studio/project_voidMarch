/// This file loads and exposes all assets to the rest of the codebase
use tellus_level as tlvl;

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

    pub lvl1: Level,
    pub lvl2: Level,
    pub lvl3: Level,
    pub lvl4: Level,
}

impl Assets {
    pub async fn load() -> Self {
        let level = Level::load().await;
        let enemies = Enemies::load().await;
        let player_races = Races::load().await;

        Self {
            level,
            enemies,
            player_races,
        }
    }
}


