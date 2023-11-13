use std::{
    env,
    path::{Path, PathBuf},
};

pub fn cwd() -> PathBuf {
    let dir = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    Path::new(&dir).to_owned()
}
