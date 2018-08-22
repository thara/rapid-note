#[macro_use]
extern crate error_chain;

mod note;
pub mod errors;

mod add_note;
mod list_notes;
mod edit_note;
mod delete_notes;

use errors::*;

pub use edit_note::UserNoteSelection;

pub struct RapidNote {
    notes: note::NoteRepository,
}

pub trait Platform {
    fn open_note(&self, path: &str) -> Result<()>;
}

#[cfg(test)]
pub mod tests {
    use errors::*;
    use note::*;

    struct NoteStoreImpl {
        data: Vec<(String, String)>
    }

    impl NoteStoreImpl {
        fn new() -> NoteStoreImpl {
            NoteStoreImpl{data: Vec::new()}
        }
    }

    impl NoteStore for NoteStoreImpl {
        fn save_content(&mut self, _title: String, _content: String) -> Result<()> {
            self.data.push((_title, _content));
            Ok(())
        }
        fn get_items(&self) -> Result<Vec<NoteSummary>> {
            let d = self.data.iter().cloned();
            let r = d.map(|(a, b)| NoteSummary{path: a, title: b});
            Ok(r.collect())
        }
        fn match_items(&self, _pattern: &str) -> Result<Vec<NoteSummary>> {
            let d = self.data.iter().cloned();
            let r = d.filter(|(a, _)| a.starts_with(_pattern)).map(|(a, b)| NoteSummary{path: a, title: b});
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
        let _ = notes.add_note(Note::new("AAA".to_string(), "".to_string()));
        let _ = notes.add_note(Note::new("AAB".to_string(), "".to_string()));
        let _ = notes.add_note(Note::new("ABB".to_string(), "".to_string()));
        let _ = notes.add_note(Note::new("BBB".to_string(), "".to_string()));
        let _ = notes.add_note(Note::new("BBA".to_string(), "".to_string()));

        let result = notes.match_notes("AA").unwrap();
        assert_eq!(result.len(), 2);

        let _ = notes.delete_notes(result);
        let result = notes.get_notes().unwrap();
        assert_eq!(result.len(), 3);
    }
}
