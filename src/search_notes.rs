use std::convert::AsRef;

use errors::*;
use note::{NoteRepository};

use ::RapidNote;

pub trait FullTextSearcher {
    fn grep(&self, pattern : &str, paths: &Vec<&str>) -> Result<()>;
}

pub struct SearchNotes<'a, 'b> {
    notes: &'a mut NoteRepository,
    pattern: &'b str,
}

impl<'a, 'b> SearchNotes<'a, 'b> {
    pub fn new(notes: &'a mut NoteRepository, pattern: &'b str) -> Self {
        SearchNotes{notes: notes, pattern: pattern}
    }

    pub fn call<S: FullTextSearcher>(&'b mut self, searcher: FullTextSearcher) -> Result<()> {
        let notes = self.notes.get_notes()?;
        let paths = notes.iter().map(|x| x.path.as_str()).collect::<Vec<_>>();
        searcher.grep(self.pattern, paths)?
    }
}

impl RapidNote {
    pub fn search_notes<'a>(&'a mut self, pattern: &'a str) -> SearchNotes {
        SearchNotes::new(&mut self.notes, pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::tests::*;

    struct FullTextSearcherImpl {}
    impl FullTextSearcher for FullTextSearcherImpl {
        fn grep(&self, pattern : &str, paths: &Vec<&str>) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn it_works() {
        let mut notes = note_repos();
        let _ = notes.add_note("WIP-XXX", "");
        let _ = notes.add_note("WIP-YYY", "");
        let _ = notes.add_note("REVIEW", "");

        let mut interactor = RapidNote{notes: notes};

        let searcher = FullTextSearcherImpl{};
        let _ = interactor.search_notes("WIP*").call(searcher);
    }
}
