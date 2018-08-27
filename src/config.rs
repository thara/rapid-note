use std::env;
use shellexpand;
use std::path::Path;
use std::io::prelude::*;
use std::fs;
use std::fs::File;

use toml;

use errors::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub note_dir: String,
    pub editor: String,
    pub select_cmd: String,
    pub grep_cmd: String,
}

impl Config {
    pub fn load() -> Result<Config> {
        let home = env::home_dir().unwrap();
        let home_path = home.to_str().unwrap();

        let dir_path = Path::new(home_path).join(".rapid_note");
        let dir = dir_path.to_str().unwrap();
        let _ = fs::create_dir_all(dir)?;

        let config_path = Path::new(dir).join("config.toml");

        if config_path.exists() {
            let mut input = String::new();
            File::open(&config_path).and_then(|mut f| {
                f.read_to_string(&mut input)
            }).unwrap();
            let mut cfg: Config = toml::from_str(&input).unwrap();
            cfg.note_dir = shellexpand::env(&cfg.note_dir).unwrap().into_owned();
            Ok(cfg)
        } else {
            let path = Path::new(dir).join(".posts");
            let note_dir = path.to_str().unwrap();
            let _ = fs::create_dir_all(note_dir)?;

            let editor = env::var("EDITOR").unwrap_or_default();

            let note_dir = env::var("RAPID_NOTE_DIR").unwrap_or(note_dir.to_string());

            let cfg = Config{
                note_dir: note_dir,
                editor: editor,
                select_cmd: "peco".to_string(),
                grep_cmd: "grep -nH {PATTERN} {LIST}".to_string(),
            };
            let toml = toml::to_string(&cfg).unwrap();

            let mut file = File::create(&config_path)?;
            file.write_all(toml.as_bytes())?;
            Ok(cfg)
        }
    }
}
