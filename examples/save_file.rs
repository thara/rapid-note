extern crate rapid_note;

use rapid_note::*;
use rapid_note::fs::FileNoteStore;

fn main() {
    let cfg = Config{note_dir: "./".to_string(), editor: "".to_string(), select_cmd: "".to_string(), grep_cmd: "".to_string()};
    let mut store = FileNoteStore{config: &cfg};
    store.save_item("sample.csv", "Title: sample.csv\n");
}

