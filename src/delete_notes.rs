use std::convert::AsRef;

use errors::*;
use note::NoteRepository;
use ::RapidNote;

pub trait UserInteraction {
    fn confirm_delete(&self) -> bool;
    fn show_note_titles(&self, paths: &Vec<&str>) -> Result<()>;
}

pub struct DeleteNotes<'a, 'b> {
    notes: &'a mut NoteRepository,
    pattern: &'b str,
}

impl<'a, 'b> DeleteNotes<'a, 'b> {
    pub fn new(notes: &'a mut NoteRepository, pattern: &'b str) -> Self {
        DeleteNotes{notes: notes, pattern: pattern}
    }

    pub fn call<U: UserInteraction>(&'b mut self, user: U) -> Result<()> {
        let notes = self.notes.get_notes()?;
        if notes.is_empty() {
            Ok(()) // FIXME Err
        } else {
            let notes = notes.into_iter().filter(|x| x.path.contains(self.pattern)).collect::<Vec<_>>();
            {
                let titles = notes.iter().map(|x| x.title.as_ref()).collect::<Vec<_>>();
                if titles.len() == 0 {
                    println!("Nothing : {}", self.pattern);
                    return Ok(())
                }
                user.show_note_titles(&titles)?;
            }
            if user.confirm_delete() {
                self.notes.delete_notes(notes)
            } else {
                Ok(()) // FIXME Err
            }
        }
    }
}

impl RapidNote {
    pub fn delete_notes<'a, 'b>(&'a mut self, pattern: &'b str) -> DeleteNotes<'a, 'b> {
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
            true
        }
        fn show_note_titles(&self, _paths: &Vec<&str>) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn it_works() {
        let mut notes = note_repos();
        let _ = notes.add_note("WIP-XXX", "WIP-XXX");
        let _ = notes.add_note("WIP-YYY", "WIP-YYY");
        let _ = notes.add_note("REVIEW", "REVIEW");

        let mut interactor = RapidNote{notes: notes};

        let user = UserInteractionImpl{};
        let _ = interactor.delete_notes("WIP").call(user);

        let notes = interactor.list_notes().call();
        assert_eq!(notes.unwrap().len(), 1);
    }
}
