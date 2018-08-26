use std::process::Command;
use std::os::unix::process::CommandExt;
use std::io::{self, Read};

extern crate rapid_note;

extern crate pad;
use pad::PadStr;
extern crate clap;
use clap::{Arg, App, SubCommand};

#[macro_use]
extern crate error_chain;

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

fn run() -> Result<()> {
    let cfg = Config::load()?;
    let note_store = FileNoteStore::new(cfg);
    let repos = NoteRepository::new(Box::new(note_store));
    let mut rapid_note = RapidNote::new(repos);

    let editor = EditorImpl{};

    let matches = App::new("Rapid Note")
                          .subcommand(SubCommand::with_name("add")
                                      .arg(Arg::with_name("TITLE")
                                           .required(false)
                                           .index(1)))
                          .subcommand(SubCommand::with_name("list"))
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
    }

    Ok(())
}

quick_main!(run);
