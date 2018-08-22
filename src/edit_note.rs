use errors::*;
use note::NoteSet;

use std::iter::FromIterator;
use std::ops::Deref;
use std::convert::AsRef;
use ::{RapidNote, Platform};

pub trait UserNoteSelection {
    fn select_note(&self, note_ids: &Vec<&str>) -> String;
}

pub struct EditNote<'a, P: Platform> {
    input: &'a str,
    platform: P,
}

impl<'a, P: Platform> EditNote<'a, P> {

    pub fn new(input: &'a str, platform: P) -> Self {
        EditNote{input: input, platform: platform}
    }

    pub fn call(&'a mut self) -> Result<()> {
        self.platform.open_note(self.input)
    }
}

pub struct SelectAndEditNote<'a, P: Platform> {
    notes: &'a mut NoteSet,
    platform: P,
}

impl<'a, P: Platform> SelectAndEditNote<'a, P> {

    pub fn new(notes: &'a mut NoteSet, platform: P) -> Self {
        SelectAndEditNote{notes: notes, platform: platform}
    }

    pub fn call<U: UserNoteSelection>(&'a mut self, user: U) -> Result<()> {
        let notes = self.notes.get_notes()?;
        let notes = notes.iter().map(|x| x.title.as_ref()).collect::<Vec<_>>();
        let selected = user.select_note(&notes);
        self.platform.open_note(&*selected)
    }
}

impl RapidNote {
    pub fn edit_note<'a, P: Platform>(&'a mut self, input: &'a str, platform: P) -> EditNote<P> {
        EditNote::new(input, platform)
    }
    pub fn select_and_edit_note<'a, P: Platform>(&'a mut self, platform: P) -> SelectAndEditNote<P> {
        SelectAndEditNote::new(&mut self.notes, platform)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::note::*;
    use ::tests::*;

    struct UserNoteSelectionImpl {}
    impl UserNoteSelection for UserNoteSelectionImpl {
        fn select_note(&self, _note_ids: &Vec<&str>) -> String {
            _note_ids[0].to_string()
        }
    }

    struct PlatformImpl {}
    impl Platform for PlatformImpl {
        fn open_note(&self, _path: &str) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn it_works() {
        let mut notes = note_set();
        let _ = notes.add_note(Note::new("WIP1".to_string(), "".to_string()));
        let _ = notes.add_note(Note::new("WIP2".to_string(), "".to_string()));

        let platform = PlatformImpl{};

        let mut interactor = RapidNote{notes: notes};
        let ret = interactor.edit_note("WIP1", platform).call();
        assert_eq!(ret.is_ok(), true);

        let platform = PlatformImpl{};
        let user = UserNoteSelectionImpl{};
        let ret = interactor.select_and_edit_note(platform).call(user);
        assert_eq!(ret.is_ok(), true);
    }
}
