/// Level storage and loading is defined here
use tellus_level::Level as TellusLevel;
use std::ops::Deref;

pub struct Levels {
    levels: [TellusLevel; 4],
}

impl Levels {
    pub async fn load() -> Result<Levels, LoadingError> {
        let lvl1 = Self::load_level(1).await?;
        let lvl2 = Self::load_level(2).await?;
        let lvl3 = Self::load_level(3).await?;
        let lvl4 = Self::load_level(4).await?;

        Ok(Levels{levels: [lvl1, lvl2, lvl3, lvl4]})
    }

    async fn load_level(id: u8) -> Result<Levels, LevelError> {
        let path = format!("assets/levels/{id}.tlvl");

        TellusLevel::load_from_file(&path).map_err(|source| LevelError::LoadFailed { id, path, source })
    }
}

// Add in so that it derefs to the array of levels
impl Deref for Levels {
    type Target = [TellusLevel; 4];

    fn deref(&self) -> &Self::Target {
        &self.levels
    }
}
