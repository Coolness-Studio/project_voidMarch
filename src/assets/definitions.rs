/// This file has the definitions for the structs containing the textures
use macroquad::prelude::*;

async fn load(path: &str) -> Texture2D {
    let texture = load_texture(path)
        .await
        .unwrap_or_else(|_| panic!("Failed to load {path}"));
    texture.set_filter(FilterMode::Nearest);
    texture
}

pub struct Tiles {
    pub base_tile: Texture2D,
    pub grass: Texture2D,
    pub space_ship: SpaceShipTiles,
}
impl Tiles {
    pub async fn load() -> Self {
        Self {
            base_tile: load("assets/textures/tiles/base_tile.png").await,
            grass: load("assets/textures/tiles/grass.png").await,
            space_ship: SpaceShipTiles::load().await,
        }
    }
}

pub struct SpaceShipTiles {
    pub border_wall_or_floor: Texture2D,
    pub bottom_left_wall: Texture2D,
    pub bottom_right_wall: Texture2D,
    pub bottom_wall: Texture2D,
    pub full_blue_wall: Texture2D,
    pub gray_floor: Texture2D,
    pub hole: Texture2D,
    pub left_wall: Texture2D,
    pub metal_white_floor_wall: Texture2D,
    pub right_wall: Texture2D,
    pub tiled_white_floor_wall: Texture2D,
    pub top_left_wall: Texture2D,
    pub top_right_wall: Texture2D,
    pub top_wall: Texture2D,
    pub white_floor_wall: Texture2D,
}
impl SpaceShipTiles {
    pub async fn load() -> Self {
        Self {
            border_wall_or_floor: load("assets/textures/tiles/space_ship/border_wall_or_floor.png").await,
            bottom_left_wall: load("assets/textures/tiles/space_ship/bottom_left_wall.png").await,
            bottom_right_wall: load("assets/textures/tiles/space_ship/bottom_right_wall.png").await,
            bottom_wall: load("assets/textures/tiles/space_ship/bottom_wall.png").await,
            full_blue_wall: load("assets/textures/tiles/space_ship/full_blue_wall.png").await,
            gray_floor: load("assets/textures/tiles/space_ship/gray_floor.png").await,
            hole: load("assets/textures/tiles/space_ship/hole.png").await,
            left_wall: load("assets/textures/tiles/space_ship/left_wall.png").await,
            metal_white_floor_wall: load("assets/textures/tiles/space_ship/metal_white_floor_wall.png").await,
            right_wall: load("assets/textures/tiles/space_ship/right_wall.png").await,
            tiled_white_floor_wall: load("assets/textures/tiles/space_ship/tiled_white_floor_wall.png").await,
            top_left_wall: load("assets/textures/tiles/space_ship/top_left_wall.png").await,
            top_right_wall: load("assets/textures/tiles/space_ship/top_right_wall.png").await,
            top_wall: load("assets/textures/tiles/space_ship/top_wall.png").await,
            white_floor_wall: load("assets/textures/tiles/space_ship/white_floor_wall.png").await,
        }
    }
}

pub struct Enemies {
    pub turtle: TurtleEnemy,
    pub big_turtle: BigTurtleEnemy,
}
impl Enemies {
    pub async fn load() -> Self {
        Self {
            turtle: TurtleEnemy::load().await,
            big_turtle: BigTurtleEnemy::load().await,
        }
    }
}

pub struct TurtleEnemy {
    pub front_sprite_sheet: Texture2D,
}
impl TurtleEnemy {
    pub async fn load() -> Self {
        Self {
            front_sprite_sheet: load("assets/textures/enemies/turtle/front_sprite_sheet.png").await,
        }
    }
}

pub struct BigTurtleEnemy {
    pub front_sprite_sheet: Texture2D,
}
impl BigTurtleEnemy {
    pub async fn load() -> Self {
        Self {
            front_sprite_sheet: load("assets/textures/enemies/big_turtle/front_sprite_sheet.png").await,
        }
    }
}

pub struct Races {
    pub human: HumanRace,
    pub space_lizard: SpaceLizardRace,
    pub void_crawler: VoidCrawlerRace,
}
impl Races {
    pub async fn load() -> Self {
        Self {
            human: HumanRace::load().await,
            space_lizard: SpaceLizardRace::load().await,
            void_crawler: VoidCrawlerRace::load().await,
        }
    }
}

pub struct HumanRace {
    pub left: Texture2D,
    pub left_roll: Texture2D,
    pub right_roll: Texture2D,
    pub back_and_left: Texture2D,
    pub back_and_right: Texture2D,
    pub front_and_right: Texture2D,
    pub roll_from_behind: Texture2D,
    pub roll_from_behind_and_left: Texture2D,
}
impl HumanRace {
    pub async fn load() -> Self {
        Self {
            left: load("assets/textures/players/human/left.png").await,
            left_roll: load("assets/textures/players/human/left_roll.png").await,
            right_roll: load("assets/textures/players/human/right_roll.png").await,
            back_and_left: load("assets/textures/players/human/back_and_left.png").await,
            back_and_right: load("assets/textures/players/human/back_and_right.png").await,
            front_and_right: load("assets/textures/players/human/front_and_right.png").await,
            roll_from_behind: load("assets/textures/players/human/roll_from_behind.png").await,
            roll_from_behind_and_left: load("assets/textures/players/human/roll_from_behind_and_left.png").await,
        }
    }
}

pub struct SpaceLizardRace {
    pub left: Texture2D,
    pub left_roll: Texture2D,
    pub right_roll: Texture2D,
    pub back_and_left: Texture2D,
    pub back_and_right: Texture2D,
    pub front_and_right: Texture2D,
}
impl SpaceLizardRace {
    pub async fn load() -> Self {
        Self {
            left: load("assets/textures/players/space_lizard/left.png").await,
            left_roll: load("assets/textures/players/space_lizard/left_roll.png").await,
            right_roll: load("assets/textures/players/space_lizard/right_roll.png").await,
            back_and_left: load("assets/textures/players/space_lizard/back_and_left.png").await,
            back_and_right: load("assets/textures/players/space_lizard/back_and_right.png").await,
            front_and_right: load("assets/textures/players/space_lizard/front_and_right.png").await,
        }
    }
}

pub struct VoidCrawlerRace {
    pub walking: Texture2D,
    pub walking_left: Texture2D,
    pub back_and_left: Texture2D,
    pub back_and_right: Texture2D,
    pub back_roll: Texture2D,
    pub back_roll_left: Texture2D,
    pub roll_left_front: Texture2D,
    pub roll_right_front: Texture2D,
    pub sword_attack_front_right: Texture2D,
    pub sword_attack_left: Texture2D,
}
impl VoidCrawlerRace {
    pub async fn load() -> Self {
        Self {
            walking: load("assets/textures/players/void_crawler/walking.png").await,
            walking_left: load("assets/textures/players/void_crawler/walking_left.png").await,
            back_and_left: load("assets/textures/players/void_crawler/back_and_left.png").await,
            back_and_right: load("assets/textures/players/void_crawler/back_and_right.png").await,
            back_roll: load("assets/textures/players/void_crawler/back_roll.png").await,
            back_roll_left: load("assets/textures/players/void_crawler/back_roll_left.png").await,
            roll_left_front: load("assets/textures/players/void_crawler/roll_left_front.png").await,
            roll_right_front: load("assets/textures/players/void_crawler/roll_right_front.png").await,
            sword_attack_front_right: load("assets/textures/players/void_crawler/sword_attack_front_right.png").await,
            sword_attack_left: load("assets/textures/players/void_crawler/sword_attack_left.png").await,
        }
    }
}

pub struct Ui {
    pub button_sprite_sheet: Texture2D,
    pub cursor: Texture2D,
    pub cursor_pointer_or_attack: Texture2D,
    pub cursor_select: Texture2D,
}
impl Ui {
    pub async fn load() -> Self {
        Self {
            button_sprite_sheet: load("assets/textures/ui/button_sprite_sheet.png").await,
            cursor: load("assets/textures/ui/cursor.png").await,
            cursor_pointer_or_attack: load("assets/textures/ui/cursor_pointer_or_attack.png").await,
            cursor_select: load("assets/textures/ui/cursor_select.png").await,
        }
    }
}

pub struct Furniture {
    pub computer_left: Texture2D,
    pub computer_right: Texture2D,
    pub golden_chest: Texture2D,
    pub golden_chest_opened: Texture2D,
    pub gray_chest: Texture2D,
    pub gray_chest_opened: Texture2D,
}
impl Furniture {
    pub async fn load() -> Self {
        Self {
            computer_left: load("assets/textures/furniture/computer_left.png").await,
            computer_right: load("assets/textures/furniture/computer_right.png").await,
            golden_chest: load("assets/textures/furniture/golden_chest.png").await,
            golden_chest_opened: load("assets/textures/furniture/golden_chest_opened.png").await,
            gray_chest: load("assets/textures/furniture/gray_chest.png").await,
            gray_chest_opened: load("assets/textures/furniture/gray_chest_opened.png").await,
        }
    }
}

pub struct Weapons {
    pub blue_dagger: Texture2D,
    pub green_sword: Texture2D,
}
impl Weapons {
    pub async fn load() -> Self {
        Self {
            blue_dagger: load("assets/textures/weapons/blue_dagger.png").await,
            green_sword: load("assets/textures/weapons/green_sword.png").await,
        }
    }
}

pub struct Cutscenes {
    pub all_stars_sprite_sheet: Texture2D,
    pub planet_cracking_sprite_sheet: Texture2D,
}
impl Cutscenes {
    pub async fn load() -> Self {
        Self {
            all_stars_sprite_sheet: load("assets/textures/cutscenes/all_stars_sprite_sheet.png").await,
            planet_cracking_sprite_sheet: load("assets/textures/cutscenes/planet_cracking_sprite_sheet.png").await,
        }
    }
}
