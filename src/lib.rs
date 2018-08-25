#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate regex;
extern crate shellexpand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod note;
pub mod errors;

mod add_note;
mod list_notes;
mod edit_note;
mod delete_notes;
mod search_notes;
mod config;

pub mod fs;

use errors::*;

pub use note::{NoteRepository, NoteStore};
pub use add_note::AddNote;
pub use edit_note::{EditNote, UserNoteSelection};
pub use list_notes::{ListNotes, NoteSummaryView};
pub use delete_notes::{DeleteNotes, UserInteraction};
pub use search_notes::{SearchNotes, FullTextSearcher};
pub use config::Config;

pub struct RapidNote {
    notes: note::NoteRepository,
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
        data: Vec<(String, String)>
    }

    impl NoteStoreImpl {
        fn new() -> NoteStoreImpl {
            NoteStoreImpl{data: Vec::new()}
        }
    }

    impl NoteStore for NoteStoreImpl {
        fn save_item(&mut self, title: &str, content: &str) -> Result<NoteSummary> {
            self.data.push((title.to_string(), content.to_string()));
            Ok(NoteSummary{path: title.to_string(), title: title.to_string()})
        }
        fn get_items(&self) -> Result<Vec<NoteSummary>> {
            let d = self.data.iter().cloned();
            let r = d.map(|(a, b)| NoteSummary{path: a, title: b});
            Ok(r.collect())
        }

        fn delete_items(&mut self, _notes: Vec<NoteSummary>) -> Result<()> {
            _notes.iter().for_each(move |x| {
                let key = &x.path;
                self.data.retain(|(x,_)| x != key);
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
