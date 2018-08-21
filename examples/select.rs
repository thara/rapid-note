extern crate rapid_note;

use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::str;

use rapid_note::UserNoteSelection;

struct UserNoteSelectionImpl {}

impl rapid_note::UserNoteSelection for UserNoteSelectionImpl {

    fn select_note(&self, note_ids: &Vec<&str>) -> String {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("peco")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().expect("failed to execute child");

        child.stdin.as_mut().unwrap().write_all(note_ids.join("\n").as_bytes()).expect("failed to write into stdin");
        //FIXME
        let output = child.wait_with_output().expect("failed to wait on child");
        assert!(output.status.success());

        let s = match str::from_utf8(&output.stdout){
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let len = s.trim_right().len();
        let mut s = s.to_string();
        s.truncate(len);
        s
    }
}

fn main() {
    let imp = UserNoteSelectionImpl{};

    let s = imp.select_note(&vec!["aaaaaa", "bbbbb", "ccccc"]);
    println!("Hello, world! ::: {}", s);
}
