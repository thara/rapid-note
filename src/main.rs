use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::os::unix::process::CommandExt;
use std::io::{self, Read};
use std::collections::HashMap;
use std::str;

extern crate rapid_note;

extern crate pad;
use pad::PadStr;
extern crate clap;
use clap::{Arg, App, SubCommand};

#[macro_use]
extern crate error_chain;
extern crate strfmt;
use strfmt::strfmt;

use rapid_note::*;
use rapid_note::errors::*;
use rapid_note::fs::FileNoteStore;
use std::path::Path;

struct EditorImpl;

impl Editor for EditorImpl {
    fn open_note(&self, path: &str) -> Result<()> {
        let cmd = format!("vim {}", path);
        let _e = Command::new("sh")
            .current_dir("./")
            .arg("-c")
            .arg(cmd)
            .exec();
        Ok(())
    }
}

struct FullTextSearcherImpl<'a> {
    grep_cmd : &'a str
}

impl<'a> FullTextSearcher for FullTextSearcherImpl<'a> {
    fn grep(&self, pattern : &str, paths: &Vec<&str>) -> Result<()> {
        let list = paths.join(" ");

        let mut vars = HashMap::new();
        vars.insert("PATTERN".to_string(), pattern);
        vars.insert("LIST".to_string(), &list);
        let cmd = strfmt(self.grep_cmd, &vars).unwrap();
        let _error = Command::new("sh")
            .current_dir("./")
            .arg("-c")
            .arg(cmd)
            .exec();
        Ok(())
    }
}

struct UserNoteSelectionImpl<'a> {
    note_dir : &'a str,
    select_cmd : &'a str,
}

impl<'a> UserNoteSelection for UserNoteSelectionImpl<'a> {
    fn select_note(&self, paths: &Vec<&str>) -> String {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(self.select_cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().expect("failed to execute child");

        let filenames = paths.iter().map(|x| Path::new(x).file_name().unwrap().to_str().unwrap().to_string()).collect::<Vec<_>>();
        child.stdin.as_mut().unwrap().write_all(filenames.join("\n").as_bytes()).expect("failed to write into stdin");
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
        Path::new(self.note_dir).join(s).to_str().unwrap().to_string()
    }
}

fn run() -> Result<()> {
    let cfg = Config::load()?;
    let note_store = FileNoteStore::new(cfg.clone());
    let repos = NoteRepository::new(Box::new(note_store));
    let mut rapid_note = RapidNote::new(repos);

    let editor = EditorImpl{};

    let matches = App::new("Rapid Note")
                          .subcommand(SubCommand::with_name("add")
                                      .arg(Arg::with_name("TITLE")
                                           .required(false)
                                           .index(1)))
                          .subcommand(SubCommand::with_name("list"))
                          .subcommand(SubCommand::with_name("grep")
                                      .arg(Arg::with_name("PATTERN")
                                           .required(true)
                                           .index(1)))
                          .subcommand(SubCommand::with_name("edit")
                                      .arg(Arg::with_name("FILE")
                                           .required(false)
                                           .index(1)))
                          .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let title = match matches.value_of("TITLE") {
            Some(title) => { title.to_string() },
            None => {
                let mut input = String::new();
                io::stdin().read_to_string(&mut input)?;
                input
            }
        };
        let _ = rapid_note.add_note(&title).call(editor);
    } else if let Some(matches) = matches.subcommand_matches("list") {
        let notes = rapid_note.list_notes().call()?;
        for n in notes {
            let p = Path::new(&n.path);
            let file_stem = p.file_stem().unwrap();
            let mut filename = file_stem.to_str().unwrap().pad_to_width(30);
            filename.truncate(30);
            println!("{}: {}", filename, n.title);
        }
    } else if let Some(matches) = matches.subcommand_matches("grep") {
        let searcher = FullTextSearcherImpl{grep_cmd: &cfg.grep_cmd};
        let pattern = matches.value_of("PATTERN").unwrap();
        let _ = rapid_note.search_notes(pattern).call(searcher)?;
    } else if let Some(matches) = matches.subcommand_matches("edit") {
        match matches.value_of("FILE") {
            Some(file) => {
                //FIXME should not use cfg.note_dir here
                let path = Path::new(&cfg.note_dir).join(file).to_str().unwrap().to_string();
                rapid_note.edit_note(&path).call(editor)?;
            },
            None => {
                let user = UserNoteSelectionImpl{note_dir: &cfg.note_dir, select_cmd: &cfg.select_cmd};
                rapid_note.select_and_edit_note().call(editor, user)?;
            }
        }
    }

    Ok(())
}

quick_main!(run);
