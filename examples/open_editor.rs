extern crate rapid_note;

use std::process::Command;
use std::str;

use std::os::unix::process::CommandExt;

use rapid_note::Platform;
use rapid_note::errors::*;

struct PlatformImpl{}

impl rapid_note::Platform for PlatformImpl {
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
    let imp = PlatformImpl{};
    let _ = imp.open_note("sample.text");
}
