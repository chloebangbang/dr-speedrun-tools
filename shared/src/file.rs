use std::path::PathBuf;

/// Returns a [PathBuf] if the directory exists or can be created, and None otherwise
fn get_cfg_dir() -> Option<PathBuf> {
    dirs::data_dir().and_then(|mut path| {
        path.push("chloebangbang");
        if !path.is_dir() {
            std::fs::create_dir(&path).ok()?;
        }
        Some(path)
    })
}

fn get_tmp_dir() -> Option<PathBuf> {
    let mut path = std::env::temp_dir();
    path.push("chloebangbang");
    if !path.is_dir() {
        std::fs::create_dir(&path).ok()?;
    }
    Some(path)
}

/// Reads a relative file to string if possible.
/// Returns None if there is no file and on any errors. Should be failsafe
pub fn read_file<P: AsRef<std::path::Path>>(filename: P) -> Option<String> {
    let mut path = get_cfg_dir()?;
    path.push(filename);

    std::fs::read_to_string(path).ok()
}

/// Saves bytes to a given filename. 
pub fn save_file<P: AsRef<std::path::Path>, C: AsRef<[u8]>>(filename: P, contents: C) {
    if let Some(mut path) = get_cfg_dir() {
        path.push(filename);
        
        if let Err(e) = std::fs::write(path, contents) {
            log::error!("Failed to save to file! {}", e);
        }
    } else {
        log::error!("Unable to get save directory!");
    }
}

pub fn write_temp_file<P: AsRef<std::path::Path>, C: AsRef<[u8]>>(filename: P, contents: C) {
    if let Some(mut path) = get_tmp_dir() {
        path.push(filename);

        if let Err(e) = std::fs::write(path, contents) {
            log::error!("Failed to save temp file! {}", e);
        }
    } else {
        log::error!("Unable to get tmp directory!");
    }
}