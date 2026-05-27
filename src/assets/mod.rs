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
    pub ui: Ui,
    pub furniture: Furniture,
    pub weapons: Weapons,
    pub cutscenes: Cutscenes,
    pub levels: Levels,
}

impl Assets {
    pub async fn load() -> Result<Self, LevelLoadError> {
        let tiles = Tiles::load().await;
        let enemies = Enemies::load().await;
        let player_races = Races::load().await;
        let ui = Ui::load().await;
        let furniture = Furniture::load().await;
        let weapons = Weapons::load().await;
        let cutscenes = Cutscenes::load().await;
        let levels = Levels::load().await?;

        Ok(Self {
            tiles,
            enemies,
            player_races,
            ui,
            furniture,
            weapons,
            cutscenes,
            levels,
        })
    }
}
