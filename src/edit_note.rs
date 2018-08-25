use errors::*;
use note::NoteRepository;

use std::convert::AsRef;
use ::{RapidNote, Editor};

pub trait UserNoteSelection {
    fn select_note(&self, note_ids: &Vec<&str>) -> String;
}

pub struct EditNote<'a> {
    input: &'a str,
}

impl<'a> EditNote<'a> {

    pub fn new(input: &'a str) -> Self {
        EditNote{input: input}
    }

    pub fn call<E: Editor>(&'a mut self, editor: E) -> Result<()> {
        editor.open_note(self.input)
    }
}

pub struct SelectAndEditNote<'a> {
    notes: &'a mut NoteRepository,
}

impl<'a> SelectAndEditNote<'a> {

    pub fn new(notes: &'a mut NoteRepository) -> Self {
        SelectAndEditNote{notes: notes}
    }

    pub fn call< E: Editor, U: UserNoteSelection>(&'a mut self, editor: E, user: U) -> Result<()> {
        let notes = self.notes.get_notes()?;
        let notes = notes.iter().map(|x| x.title.as_ref()).collect::<Vec<_>>();
        let selected = user.select_note(&notes);
        editor.open_note(&*selected)
    }
}

impl RapidNote {
    pub fn edit_note<'a>(&'a mut self, input: &'a str) -> EditNote {
        EditNote::new(input)
    }
    pub fn select_and_edit_note<'a>(&'a mut self) -> SelectAndEditNote {
        SelectAndEditNote::new(&mut self.notes)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::tests::*;

    struct UserNoteSelectionImpl {}
    impl UserNoteSelection for UserNoteSelectionImpl {
        fn select_note(&self, _note_ids: &Vec<&str>) -> String {
            _note_ids[0].to_string()
        }
    }

    #[test]
    fn it_works() {
        let mut notes = note_repos();
        let _ = notes.add_note("WIP1", "");
        let _ = notes.add_note("WIP2", "");

        let editor = EditorImpl{};
        let mut interactor = RapidNote{notes: notes};
        let ret = interactor.edit_note("WIP1").call(editor);
        assert_eq!(ret.is_ok(), true);

        let editor = EditorImpl{};
        let user = UserNoteSelectionImpl{};
        let ret = interactor.select_and_edit_note().call(editor, user);
        assert_eq!(ret.is_ok(), true);
    }
}
