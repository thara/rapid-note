use chrono::prelude::*;

use std::path::Path;
use std::fs::File;
use std::io::Write;

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
        Ok(Vec::new())
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_escape() {
        assert_eq!(escape(r#" <>:"/\|?*%#"#), "-");
    }
}
