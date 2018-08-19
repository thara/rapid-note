use errors::*;
use note::NoteSet;
use ::RapidNote;

pub trait UserInteraction {
    fn confirm_delete(&self) -> bool;
    fn show_note_titles(&self, paths: &Vec<String>) -> Result<()>;
}

pub struct DeleteNotes<'a, 'b> {
    notes: &'a mut NoteSet,
    pattern: &'b str,
}

impl<'a, 'b> DeleteNotes<'a, 'b> {
    pub fn new(notes: &'a mut NoteSet, pattern: &'b str) -> Self {
        DeleteNotes{notes: notes, pattern: pattern}
    }

    pub fn call<U: UserInteraction>(&'b mut self, user: U) -> Result<()> {
        let notes = self.notes.match_notes(self.pattern)?;
        if notes.is_empty() {
            Ok(()) // FIXME Err
        } else {
            let titles = notes.clone().into_iter().map(|x| x.title).collect::<Vec<_>>();
            user.show_note_titles(&titles)?;
            if user.confirm_delete() {
                self.notes.delete_notes(notes)
            } else {
                Ok(()) // FIXME Err
            }
        }
    }
}

impl RapidNote {
    pub fn delete_notes<'a>(&'a mut self, pattern: &'a str) -> DeleteNotes {
        DeleteNotes::new(&mut self.notes, pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::tests::*;

    struct UserInteractionImpl {}
    impl UserInteraction for UserInteractionImpl {
        fn confirm_delete(&self) -> bool {
            false
        }
        fn show_note_titles(&self, _paths: &Vec<String>) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn it_works() {
        let notes = note_set();

        let mut interactor = RapidNote{notes: notes};

        let user = UserInteractionImpl{};
        let _ = interactor.delete_notes("WIP*").call(user);
    }
}
