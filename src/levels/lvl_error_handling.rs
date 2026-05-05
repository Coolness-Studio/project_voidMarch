use std::error::Error;
use std::fmt;
use tellus_level::LevelFormatError;

#[derive(Debug)]
pub enum LevelDrawError {
    UnknownLevel { id: u8 },
    TileReadFailed {
        id: u8,
        x: u16,
        y: u16,
        source: LevelFormatError,
    },
}

impl fmt::Display for LevelDrawError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownLevel { id } => write!(f, "level {id} is not registered"),
            Self::TileReadFailed { id, x, y, source } => {
                write!(f, "failed to read tile ({x}, {y}) in level {id}: {source}")
            }
        }
    }
}

impl Error for LevelDrawError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::UnknownLevel { .. } => None,
            Self::TileReadFailed { source, .. } => Some(source),
        }
    }
}
