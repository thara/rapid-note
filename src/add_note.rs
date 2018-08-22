use errors::*;
use note::{Note, NoteSet};

use ::RapidNote;

#[derive(Debug)]
pub struct NewNote<'a> {
    title: &'a str,
    body: &'a str,
}

pub struct AddNote<'a, 'b> {
    notes: &'a mut NoteSet,
    input: &'b NewNote<'b>,
}

impl<'a, 'b> AddNote<'a, 'b> {
    pub fn new(notes: &'a mut NoteSet, input: &'b mut NewNote) -> Self {
        AddNote{notes: notes, input: input}
    }

    pub fn call(&'b mut self) -> Result<()> {
        let note = Note::new(self.input.title.to_string(), self.input.body.to_string());
        self.notes.add_note(note)
    }
}

impl RapidNote {
    pub fn add_note<'a>(&'a mut self, input: &'a mut NewNote) -> AddNote {
        AddNote::new(&mut self.notes, input)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::tests::*;

    #[test]
    fn it_works() {
        let mut notes = note_set();
        let _ = notes.add_note(Note::new("WIP1".to_string(), "".to_string()));
        let _ = notes.add_note(Note::new("WIP2".to_string(), "".to_string()));

        let mut interactor = RapidNote{notes: notes};
        let mut input = NewNote{title: "WIP3", body: ""};
        let _ = interactor.add_note(&mut input).call();

        let notes = interactor.list_notes().call();
        assert_eq!(notes.unwrap().len(), 3);
    }
}
