use errors::*;
use note::NoteRepository;

use RapidNote;

pub struct NoteSummaryView {
    pub path: String,
    pub title: String,
}

pub struct ListNotes<'a> {
    notes: &'a mut NoteRepository,
}

impl<'a> ListNotes<'a> {
    pub fn new(notes: &'a mut NoteRepository) -> Self {
        ListNotes { notes: notes }
    }

    pub fn call(&mut self) -> Result<Vec<NoteSummaryView>> {
        let notes = self.notes.get_notes()?;
        let notes = notes.into_iter().map(|x| NoteSummaryView {
            path: x.path,
            title: x.title,
        });
        Ok(notes.collect::<Vec<_>>())
    }
}

impl RapidNote {
    pub fn list_notes(&mut self) -> ListNotes {
        ListNotes::new(&mut self.notes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tests::*;

    #[test]
    fn it_works() {
        let mut notes = note_repos();
        let _ = notes.add_note("WIP", "");
        let _ = notes.add_note("REVIEW", "");

        let mut interactor = RapidNote { notes: notes };
        let result = interactor.list_notes().call();

        assert_eq!(result.unwrap().len(), 2);
    }
}
