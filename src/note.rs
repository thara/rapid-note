use errors::*;

// a entity
#[derive(Debug)]
pub struct Note {
    title: String,
    body: String,
}

impl Note {
    pub fn new(title: String, body: String) -> Note {
        Note{title: title, body: body}
    }
}

#[derive(Debug, Clone)]
pub struct NoteSummary {
    pub path: String,
    pub title: String,
}

pub struct NoteRepository {
    imp: Box<NoteStore>
}

impl NoteRepository {
    pub fn new(imp: Box<NoteStore>) -> NoteRepository {
        NoteRepository{imp}
    }

    pub fn add_note(&mut self, new_note: Note) -> Result<()> {
        self.imp.save_content(new_note.title, new_note.body)
    }

    pub fn get_notes(&self) -> Result<Vec<NoteSummary>> {
        self.imp.get_items()
    }

    pub fn match_notes(&self, pattern: &str) -> Result<Vec<NoteSummary>> {
        self.imp.match_items(pattern)
    }

    pub fn delete_notes(&mut self, notes: Vec<NoteSummary>) -> Result<()> {
        self.imp.delete_items(notes)
    }
}

pub trait NoteStore {
    fn save_content(&mut self, title:String, content:String) -> Result<()>;
    fn get_items(&self) -> Result<Vec<NoteSummary>>;
    fn match_items(&self, pattern: &str) -> Result<Vec<NoteSummary>>;
    fn delete_items(&mut self, notes: Vec<NoteSummary>) -> Result<()>;
}
