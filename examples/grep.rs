extern crate rapid_note;

use std::os::unix::process::CommandExt;
use std::process::Command;

use rapid_note::fs::FileNoteStore;
use rapid_note::*;

fn main() {
    let cfg = Config {
        note_dir: "./examples".to_string(),
        editor: "".to_string(),
        select_cmd: "".to_string(),
        grep_cmd: "ag".to_string(),
    };
    let store = FileNoteStore { config: &cfg };
    match store.get_items() {
        Ok(files) => {
            let paths = files.into_iter().map(|x| x.path).collect::<Vec<_>>();

            let cmd = format!("{} {} {}", cfg.grep_cmd, "WIP*", paths.join(" "));
            let _error = Command::new("sh")
                .current_dir("./")
                .arg("-c")
                .arg(cmd)
                .exec();
        }
        Err(e) => {
            println!("Failed get_items: {:?}", e);
        }
    }
}
