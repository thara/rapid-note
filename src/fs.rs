use chrono::prelude::*;

use std::path::Path;
use std::fs::{self, File, DirEntry};
use std::io::{BufReader, Write};
use std::io::prelude::*;

use regex::Regex;
use shellexpand;

use errors::*;
use note::{NoteStore, NoteSummary};
use config::Config;

pub struct FileNoteStore<'a> {
    pub config: &'a Config
}

impl<'a> NoteStore for FileNoteStore<'a> {
    fn save_item(&mut self, title: &str, content: &str) -> Result<NoteSummary> {
        let date = Local::now();
        let filename = format!("{}-{}.md", date.format("%Y-%m-%d"), escape(title));
        let pathname = shellexpand::env(&self.config.note_dir).unwrap().into_owned();
        let dir = Path::new(&pathname);
        let path = dir.with_file_name(filename).as_path().to_str().unwrap().to_string();
        let mut file = File::create(&path)?;
        file.write_all(content.as_bytes())?;
        Ok(NoteSummary{path: path, title: title.to_string()})
    }
    fn get_items(&self) -> Result<Vec<NoteSummary>> {
        let mut v = Vec::new();
        for entry in fs::read_dir(&self.config.note_dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                let title = first_line(&path.as_path().to_str().unwrap())?;
                v.push(NoteSummary{path: path.to_str().unwrap().to_string(), title: title});
            }
        }
        Ok(v)
    }
    fn match_items(&self, pattern: &str) -> Result<Vec<NoteSummary>> {
        Ok(Vec::new())
    }
    fn delete_items(&mut self, notes: Vec<NoteSummary>) -> Result<()> {
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
    let num_bytes = r.read_line(&mut s)?;
    if s.starts_with("Title :") {
        Ok(s[6..].to_string())
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
