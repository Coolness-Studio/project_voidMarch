/// Level storage and loading is defined here
use std::{error::Error, fmt, path::PathBuf};
use tellus_level::Level as TellusLevel;
use tellus_level::LevelIoError;

// We add in more levels here as we go
const LEVEL_IDS: [u8; 1] = [0];

pub struct Levels {
    levels: Vec<LoadedLevel>,
}

struct LoadedLevel {
    id: u8,
    level: TellusLevel,
}

// I think we should maybe possibly move the error type to its own file in the future.
#[derive(Debug)]
pub struct LevelLoadError {
    id: u8,
    path: PathBuf,
    source: LevelIoError,
}

impl LevelLoadError {
    fn new(id: u8, path: PathBuf, source: LevelIoError) -> Self {
        Self { id, path, source }
    }
}

impl fmt::Display for LevelLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to load level {} from {}: {}",
            self.id,
            self.path.display(),
            self.source
        )
    }
}

impl Error for LevelLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl Levels {
    pub async fn load() -> Result<Levels, LevelLoadError> {
        let mut levels = Vec::with_capacity(LEVEL_IDS.len());

        for id in LEVEL_IDS {
            levels.push(LoadedLevel {
                id,
                level: Self::load_level(id)?,
            });
        }

        Ok(Levels { levels })
    }

    pub fn get(&self, id: u8) -> Option<&TellusLevel> {
        self.levels.iter()
            .find(|loaded_level| loaded_level.id == id)
            .map(|loaded_level| &loaded_level.level)
    }

    fn load_level(id: u8) -> Result<TellusLevel, LevelLoadError> {
        let path = PathBuf::from(format!("assets/levels/{id}.tlvl"));

        TellusLevel::load_from_file(&path).map_err(|source| LevelLoadError::new(id, path, source))
    }
}
