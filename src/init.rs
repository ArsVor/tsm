use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub struct AppPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub db: PathBuf,
}

pub fn init_paths() -> AppPaths {
    let proj =
        ProjectDirs::from("com", "ars", "tsm").expect("Cannot determine project directories");

    let config_dir = proj.config_dir().to_path_buf();
    let data_dir = proj.data_dir().to_path_buf();
    let cache_dir = proj.cache_dir().to_path_buf();

    fs::create_dir_all(&config_dir).unwrap();
    fs::create_dir_all(&data_dir).unwrap();
    fs::create_dir_all(&cache_dir).unwrap();

    let db = data_dir.join("tsm.db");

    AppPaths {
        config_dir,
        data_dir,
        cache_dir,
        db,
    }
}
