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
    notes: note::NoteSet,
}

pub trait Platform {
    fn open_note(&self, path: &str) -> Result<()>;
}

#[cfg(test)]
pub mod tests {
    use errors::*;
    use note::*;

    struct NoteStoreImpl{}

    impl NoteStore for NoteStoreImpl {
        fn save_content(&self, _title: String, _content: String) -> Result<()> {
            Ok(())
        }
        fn get_items(&self) -> Result<Vec<NoteSummary>> {
            Ok(Vec::new())
        }
        fn match_items(&self, _pattern: &str) -> Result<Vec<NoteSummary>> {
            Ok(Vec::new())
        }

        fn delete_items(&self, _notes: Vec<NoteSummary>) -> Result<()> {
            Ok(())
        }
    }

    pub fn note_set() -> NoteSet {
        NoteSet::new(Box::new(NoteStoreImpl{}))
    }
}
