use errors::*;
use note::NoteSet;

use ::RapidNote;

pub struct NoteSummaryView {
    path: String,
    title: String,
}

pub struct ListNotes<'a> {
    notes: &'a mut NoteSet,
}

impl<'a> ListNotes<'a> {
    pub fn new(notes: &'a mut NoteSet) -> Self {
        ListNotes{notes: notes}
    }

    pub fn call(&'a mut self) -> Result<Vec<NoteSummaryView>> {
        let notes = self.notes.get_notes()?;
        let notes = notes.into_iter().map(|x| NoteSummaryView{path: x.path, title: x.title});
        Ok(notes.collect::<Vec<_>>())
    }
}


impl RapidNote {
    pub fn list_notes<'a>(&'a mut self) -> ListNotes {
        ListNotes::new(&mut self.notes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::note::*;
    use ::tests::*;

    #[test]
    fn it_works() {
        let mut notes = note_set();
        let _ = notes.add_note(Note::new("WIP".to_string(), "".to_string()));
        let _ = notes.add_note(Note::new("REVIEW".to_string(), "".to_string()));

        let mut interactor = RapidNote{notes: notes};
        let result = interactor.list_notes().call();

        assert_eq!(result.unwrap().len(), 2);
    }
}
