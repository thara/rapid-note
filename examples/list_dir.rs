extern crate rapid_note;

use rapid_note::*;
use rapid_note::fs::FileNoteStore;

fn main() {
    let cfg = Config{note_dir: "./".to_string(), editor: "".to_string(), select_cmd: "".to_string(), grep_cmd: "".to_string()};
    let store = FileNoteStore{config: &cfg};
    match store.get_items() {
        Ok(v) => {
            for e in v {
                println!("{} : {}", e.path, e.title);
            }
        },
        Err(e) => {
            println!("Failed get_items: {:?}", e);
        }
    }
}


