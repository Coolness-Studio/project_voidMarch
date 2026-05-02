/// This defines the LevelError enum that holds different level related error data.
use tellus_level::{LevelFormatError, LevelIoError};

#[derive(Debug)]
pub enum LevelError {
    UnknownLevel { id: u8 },
    LoadFailed {
        id: u8,
        path: String,
        source: LevelIoError,
    },
    TileReadFailed {
        id: u8,
        x: u16,
        y: u16,
        source: LevelFormatError,
    },
}

impl fmt::Display for LevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownLevel { id } => write!(f, "level {id} is not registered"),
            Self::LoadFailed { id, path, source } => {
                write!(f, "failed to load level {id} from {path}: {source}")
            }
            Self::TileReadFailed { id, x, y, source } => {
                write!(f, "failed to read tile ({x}, {y}) in level {id}: {source}")
            }
        }
    }
}

impl Error for LevelError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::UnknownLevel { .. } => None,
            Self::LoadFailed { source, .. } => Some(source),
            Self::TileReadFailed { source, .. } => Some(source),
        }
    }
}

