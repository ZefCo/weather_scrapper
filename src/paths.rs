use std::{env, path::{PathBuf}};

pub fn getcwd() -> std::io::Result<PathBuf>{
    let cwd = env::current_dir()?;
    Ok(cwd)
}

// pub fn sub_file(root: PathBuf, subdir: &str, file: &str) -> PathBuf {
//     let rel_path: PathBuf = [subdir, file].iter().collect(); 
//     let abs_path = root.join(rel_path);
//     return abs_path
// }

pub fn sub_file(root: PathBuf, subdir: &str, file: &str) -> PathBuf {
    let rel_path: PathBuf = [subdir, file].iter().collect(); 
    let abs_path = root.join(rel_path);
    // println!("{}", abs_path.exists())
    // let test = abs_path.exists()?;
    if abs_path.exists() {
        return abs_path
    } else {
        panic!("{} does not exist, did the file get moved?", abs_path.display())
    }
}
