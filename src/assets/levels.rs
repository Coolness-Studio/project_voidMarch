/// Level storage and loading is defined here
use tellus_level::Level;

pub fn get_level(id: u8) -> Result<Level, LevelError> {
    let path = format!("assets/levels/{id}.tlvl");

    Level::load_from_file(&path).map_err(|source| LevelError::LoadFailed { id, path, source })
}
