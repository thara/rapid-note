extern crate rapid_note;

use rapid_note::*;
use rapid_note::fs::FileNoteStore;

fn main() {
    let cfg = Config{note_dir: "./examples".to_string(), editor: "".to_string(), select_cmd: "".to_string(), grep_cmd: "ag".to_string()};
    let store = FileNoteStore{config: &cfg};
    store.match_items("WIP*");
}

