use std::{env, path::Path};

pub mod config;

pub fn is_program_in_path(program: &str) -> bool {
    let path = env::var("PATH").unwrap();
    let paths = path.split(":");
    for path in paths {
        let path = format!("{}/{}", path, program);
        if Path::new(&path).exists() {
            return true;
        }
    }
    return false;
}

pub fn relative_to_home(path: &str) -> String {
    let home = get_home_dir();
    let path = format!("{}/{}", home, path);
    return path;
}

pub fn get_home_dir() -> String {
    return env::var("HOME").unwrap();
}
