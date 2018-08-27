#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate regex;
extern crate serde;
extern crate shellexpand;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod errors;
mod note;

mod add_note;
mod config;
mod delete_notes;
mod edit_note;
mod list_notes;
mod search_notes;

pub mod fs;

use errors::*;

pub use add_note::AddNote;
pub use config::Config;
pub use delete_notes::{DeleteNotes, UserInteraction};
pub use edit_note::{EditNote, SelectAndEditNote, UserNoteSelection};
pub use list_notes::{ListNotes, NoteSummaryView};
pub use note::{NoteRepository, NoteStore};
pub use search_notes::{FullTextSearcher, SearchNotes};

pub struct RapidNote {
    notes: note::NoteRepository,
}

impl<'a> RapidNote {
    pub fn new(repos: NoteRepository) -> Self {
        RapidNote { notes: repos }
    }
}

pub trait Editor {
    fn open_note(&self, path: &str) -> Result<()>;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use note::*;

    pub struct EditorImpl {}
    impl Editor for EditorImpl {
        fn open_note(&self, _path: &str) -> Result<()> {
            Ok(())
        }
    }

    struct NoteStoreImpl {
        data: Vec<(String, String)>,
    }

    impl NoteStoreImpl {
        fn new() -> NoteStoreImpl {
            NoteStoreImpl { data: Vec::new() }
        }
    }

    impl NoteStore for NoteStoreImpl {
        fn save_item(&mut self, title: &str, content: &str) -> Result<NoteSummary> {
            self.data.push((title.to_string(), content.to_string()));
            Ok(NoteSummary {
                path: title.to_string(),
                title: title.to_string(),
            })
        }
        fn get_items(&self) -> Result<Vec<NoteSummary>> {
            let d = self.data.iter().cloned();
            let r = d.map(|(a, b)| NoteSummary { path: a, title: b });
            Ok(r.collect())
        }

        fn delete_items(&mut self, _notes: Vec<NoteSummary>) -> Result<()> {
            _notes.iter().for_each(move |x| {
                let key = &x.path;
                self.data.retain(|(x, _)| x != key);
            });
            Ok(())
        }
    }

    pub fn note_repos() -> NoteRepository {
        NoteRepository::new(Box::new(NoteStoreImpl::new()))
    }

    #[test]
    fn it_works() {
        let mut notes = note_repos();
        let _ = notes.add_note("AAA", "");
        let _ = notes.add_note("AAB", "");
        let _ = notes.add_note("ABB", "");
        let _ = notes.add_note("BBB", "");
        let _ = notes.add_note("BBA", "");

        let result = notes.get_notes().unwrap();
        assert_eq!(result.len(), 5);

        let _ = notes.delete_notes(result);
        let result = notes.get_notes().unwrap();
        assert_eq!(result.len(), 0);
    }
}
