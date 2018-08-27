extern crate rapid_note;

use std::process::Command;
use std::str;

use std::os::unix::process::CommandExt;

use rapid_note::errors::*;
use rapid_note::Editor;

struct EditorImpl {}

impl rapid_note::Editor for EditorImpl {
    fn open_note(&self, path: &str) -> Result<()> {
        let cmd = format!("vim {}", path);
        let _error = Command::new("sh")
            .current_dir("./")
            .arg("-c")
            .arg(cmd)
            .exec();
        Ok(())
    }
}

fn main() {
    let imp = EditorImpl {};
    let _ = imp.open_note("sample.text");
}
