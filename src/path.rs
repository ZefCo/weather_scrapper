use std::{env, path::{Path, PathBuf}};

pub fn getcwd() -> std::io::Result<PathBuf>{
    let cwd = env::current_dir()?;
    Ok(cwd)
}

pub fn json_fils(root: PathBuf) {
    let json_dir = "jsons_files";
    let json_locations = "location.json";
    let json_locations_file = [root.display(), json_dir, json_locations];
}