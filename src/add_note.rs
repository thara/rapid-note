use errors::*;
use note::{NoteRepository};

use ::{RapidNote, Platform};

trait UserInput {
    fn input_title(&self) -> String;
}

pub struct AddNote<'a, 'b, P: Platform> {
    notes: &'a mut NoteRepository,
    title: &'b str,
    platform: P,
}

impl<'a, 'b, P: Platform> AddNote<'a, 'b, P> {
    pub fn new(notes: &'a mut NoteRepository, title: &'b str, platform: P) -> Self {
        AddNote{notes: notes, title: title, platform: platform}
    }

    pub fn call(&'b mut self) -> Result<()> {
        let body = format!("# {}\n\n", self.title);
        let summary = self.notes.add_note(self.title, &body)?;
        self.platform.open_note(&summary.path)
    }
}

impl RapidNote {
    pub fn add_note<'a, P: Platform>(&'a mut self, title: &'a str, platform: P) -> AddNote<P> {
        AddNote::new(&mut self.notes, title, platform)
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
        let _ = interactor.add_note("WIP3", platform).call();

        let notes = interactor.list_notes().call();
        assert_eq!(notes.unwrap().len(), 3);
    }
}
