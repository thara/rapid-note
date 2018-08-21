use std::process::{Command};
use std::str;

struct UserNoteSelectionImpl {}

impl UserNoteSelection for UserNoteSelectionImpl {

    fn select_note(&self, note_ids: &Vec<String>) -> String {
        let child = Command::new("sh")
            .arg("-c")
            .arg("peco")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().expect("failed to execute child");
        let stdout = child.stdout.as_mut().unwrap();
        let stdin = child.stdin.as_mut().unwrap();

        stdin.write_all(note_ids.join(os.linesep));
        //FIXME
        let output = child.wait_with_output().expect("failed to wait on child");
        assert!(output.status.success());

        let mut s = match str::from_utf8(&output.stdout){
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
        let len = s.trim_right().len();
        s.truncate(len);
        s
    }
}
