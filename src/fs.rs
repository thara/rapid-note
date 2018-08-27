use chrono::prelude::*;

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufReader, Write};
use std::path::Path;

use regex::Regex;
use shellexpand;

use config::Config;
use errors::*;
use note::{NoteStore, NoteSummary};

pub struct FileNoteStore {
    pub config: Config,
}

impl<'a> FileNoteStore {
    pub fn new(cfg: Config) -> Self {
        FileNoteStore { config: cfg }
    }
}

impl NoteStore for FileNoteStore {
    fn save_item(&mut self, title: &str, content: &str) -> Result<NoteSummary> {
        let date = Local::now();
        let filename = format!("{}-{}.md", date.format("%Y-%m-%d"), escape(title));
        let pathname = shellexpand::env(&self.config.note_dir)
            .unwrap()
            .into_owned();
        let dir = Path::new(&pathname);
        let path = dir.join(filename).as_path().to_str().unwrap().to_string();
        let mut file = File::create(&path)?;
        file.write_all(content.as_bytes())?;
        Ok(NoteSummary {
            path: path,
            title: title.to_string(),
        })
    }

    fn get_items(&self) -> Result<Vec<NoteSummary>> {
        let pathname = shellexpand::env(&self.config.note_dir)
            .unwrap()
            .into_owned();
        let dir = Path::new(&pathname);

        let mut v = Vec::new();
        for entry in fs::read_dir(&self.config.note_dir)? {
            let entry = entry?;
            let path = entry.path();
            let filename = path.as_path().to_str().unwrap();
            if !path.is_dir() && filename.ends_with(".md") {
                let path = dir
                    .with_file_name(filename)
                    .as_path()
                    .to_str()
                    .unwrap()
                    .to_string();
                let title = first_line(&path)?;
                v.push(NoteSummary {
                    path: path,
                    title: title,
                });
            }
        }
        Ok(v)
    }

    fn delete_items(&mut self, notes: Vec<NoteSummary>) -> Result<()> {
        for entry in notes {
            fs::remove_file(entry.path)?;
        }
        Ok(())
    }
}

fn escape(s: &str) -> String {
    let escape_chars_1 = Regex::new(r#"[ <>:"/\\|?*%#]"#).unwrap();
    let escape_chars_2 = Regex::new(r"--+").unwrap();

    let s = escape_chars_1.replace_all(s, "-");
    escape_chars_2.replace_all(&s, "-").into_owned()
}

fn first_line(path: &str) -> Result<String> {
    let f = File::open(path)?;
    let mut r = BufReader::new(f);

    let mut s = String::new();
    let _ = r.read_line(&mut s)?;
    if s.starts_with("# ") {
        let l = s.trim_right().len();
        s.truncate(l);
        Ok(s[2..].to_string())
    } else {
        Ok("".to_string())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_escape() {
        assert_eq!(escape(r#" <>:"/\|?*%#"#), "-");
    }
}
