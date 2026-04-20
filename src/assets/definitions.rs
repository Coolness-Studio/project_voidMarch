/// This file has the definitions for the structs containing the textures
use macroquad::prelude::*;

pub struct Level {
    pub base_tile: Texture2D, // We'll add in more easily as we go
    pub grass: Texture2D,
}
impl Level {
    pub async fn load() -> Self {
        // EXAMPLE
        let base_tile = load_texture("assets/textures/tiles/base_tile.png")
            .await
            .expect("Failed to load assets/textures/tiles/base_tile.png");
        base_tile.set_filter(FilterMode::Nearest);

        let grass = load_texture("assets/textures/tiles/grass.png")
            .await
            .expect("Failed to load assets/textures/tiles/grass.png");
        grass.set_filter(FilterMode::Nearest);

        Self { 
            base_tile, 
            grass,
        }
    }
}

pub struct Enemies {
    pub turtle_master: Texture2D,
}
impl Enemies {
    pub async fn load() -> Self {
        let turtle_master = Texture2D::empty();
        Self { turtle_master }
    }
}

pub struct Races {
    pub void_crawler: Texture2D,
    pub human: Texture2D,
}
impl Races {
    pub async fn load() -> Self {
        let (void_crawler, human) = (Texture2D::empty(), Texture2D::empty());
        Self {
            void_crawler,
            human,
        }
    }
}
