use std::io::{self, Read};
use std::path::Path;

extern crate rapid_note;

extern crate pad;
use pad::PadStr;
extern crate clap;
use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate error_chain;

use rapid_note::cli::*;
use rapid_note::errors::*;
use rapid_note::repository::NoteRepository;
use rapid_note::{Config, NoteSummaryView};

fn run() -> Result<()> {
    let repos = Config::load()?;

    let matches = App::new("Rapid Note")
        .subcommand(
            SubCommand::with_name("add").arg(Arg::with_name("TITLE").required(false).index(1)),
        )
        .subcommand(SubCommand::with_name("list"))
        .subcommand(
            SubCommand::with_name("grep").arg(Arg::with_name("PATTERN").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("edit").arg(Arg::with_name("FILE").required(false).index(1)),
        )
        .subcommand(
            SubCommand::with_name("delete").arg(Arg::with_name("PATTERN").required(true).index(1)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let title = match matches.value_of("TITLE") {
            Some(title) => title.to_string(),
            None => {
                let mut input = String::new();
                io::stdin().read_to_string(&mut input)?;
                input
            }
        };
        let body = format!("# {}\n\n", title);
        let summary = repos.add_note(&title, &body)?;
        let _ = open_note(&summary.path);
    } else if let Some(_) = matches.subcommand_matches("list") {
        let notes = repos.get_notes()?;
        let notes = notes.into_iter().map(|x| NoteSummaryView {
            path: x.path,
            title: x.title,
        });

        for n in notes {
            let p = Path::new(&n.path);
            let file_stem = p.file_stem().unwrap();
            let mut filename = file_stem.to_str().unwrap().pad_to_width(30);
            filename.truncate(30);
            println!("{}: {}", filename, n.title);
        }
    } else if let Some(matches) = matches.subcommand_matches("grep") {
        let pattern = matches.value_of("PATTERN").unwrap();

        let notes = repos.get_notes()?;
        let paths = notes.iter().map(|x| x.path.as_str()).collect::<Vec<_>>();
        let _ = grep(&repos.grep_cmd, pattern, &paths)?;
    } else if let Some(matches) = matches.subcommand_matches("edit") {
        match matches.value_of("FILE") {
            Some(file) => {
                //FIXME should not use repos.note_dir here
                let path = Path::new(&repos.note_dir)
                    .join(file)
                    .to_str()
                    .unwrap()
                    .to_string();
                let _ = open_note(&path)?;
            }
            None => {
                let notes = repos.get_notes()?;
                let notes = notes.iter().map(|x| x.path.as_ref()).collect::<Vec<_>>();
                let selected = select_note(&repos.select_cmd, &repos.note_dir, &notes);
                open_note(&*selected)?;
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        let pattern = matches.value_of("PATTERN").unwrap();

        let notes = repos.get_notes()?;
        if notes.is_empty() {
            // FIXME Err
        } else {
            let notes = notes
                .into_iter()
                .filter(|x| x.path.contains(pattern))
                .collect::<Vec<_>>();
            {
                let titles = notes.iter().map(|x| x.title.as_ref()).collect::<Vec<_>>();
                if titles.len() == 0 {
                    println!("Nothing : {}", pattern);
                    return Ok(());
                }
                show_note_titles(&titles)?;
            }
            if confirm_delete() {
                repos.delete_notes(notes)?;
            } else {
                // FIXME Err
            }
        }
    }

    Ok(())
}

quick_main!(run);
