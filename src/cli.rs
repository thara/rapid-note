use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;

use strfmt::strfmt;

use errors::*;

pub fn open_note(path: &str) -> Result<()> {
    let cmd = format!("vim {}", path);
    let _e = Command::new("sh")
        .current_dir("./")
        .arg("-c")
        .arg(cmd)
        .exec();
    Ok(())
}

pub fn grep(grep_cmd: &str, pattern: &str, paths: &Vec<&str>) -> Result<()> {
    let list = paths.join(" ");

    let mut vars = HashMap::new();
    vars.insert("PATTERN".to_string(), pattern);
    vars.insert("LIST".to_string(), &list);
    let cmd = strfmt(grep_cmd, &vars).unwrap();
    let _error = Command::new("sh")
        .current_dir("./")
        .arg("-c")
        .arg(cmd)
        .exec();
    Ok(())
}

pub fn select_note(select_cmd: &str, note_dir: &str, paths: &Vec<&str>) -> String {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(select_cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let filenames = paths
        .iter()
        .map(|x| {
            Path::new(x)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(filenames.join("\n").as_bytes())
        .expect("failed to write into stdin");
    //FIXME
    let output = child.wait_with_output().expect("failed to wait on child");
    assert!(output.status.success());

    let s = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let len = s.trim_right().len();
    let mut s = s.to_string();
    s.truncate(len);
    Path::new(note_dir).join(s).to_str().unwrap().to_string()
}

pub fn show_note_titles(paths: &Vec<&str>) -> Result<()> {
    if paths.len() == 0 {
        bail!("Nothing!");
    }
    for p in paths {
        println!("{}", p);
    }
    Ok(())
}

pub fn confirm_delete() -> bool {
    print!("Will delete those entry. Are you sure? (y/N) : ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    let mut input = String::new();
    let stdin = io::stdin();
    stdin
        .lock()
        .read_line(&mut input)
        .expect("Could not read line");
    let answer = input.trim_right();
    answer == "y" || answer == "Y"
}
