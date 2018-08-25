use errors::*;
use note::{NoteRepository};

use ::{RapidNote, Platform};

pub struct AddNote<'a, 'b> {
    notes: &'a mut NoteRepository,
    title: &'b str,
}

impl<'a, 'b> AddNote<'a, 'b> {
    pub fn new(notes: &'a mut NoteRepository, title: &'b str) -> Self {
        AddNote{notes: notes, title: title}
    }

    pub fn call<P: Platform>(&'b mut self, platform: P) -> Result<()> {
        let body = format!("# {}\n\n", self.title);
        let summary = self.notes.add_note(self.title, &body)?;
        platform.open_note(&summary.path)
    }
}

impl RapidNote {
    pub fn add_note<'a>(&'a mut self, title: &'a str) -> AddNote {
        AddNote::new(&mut self.notes, title)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::tests::*;

    #[test]
    fn it_works() {
        let mut notes = note_repos();
        let _ = notes.add_note("WIP1", "");
        let _ = notes.add_note("WIP2", "");
        let platform = PlatformImpl{};

        let mut interactor = RapidNote{notes: notes};
        let _ = interactor.add_note("WIP3").call(platform);

        let notes = interactor.list_notes().call();
        assert_eq!(notes.unwrap().len(), 3);
    }
}
