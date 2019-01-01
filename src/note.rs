use errors::*;

#[derive(Debug, Clone)]
pub struct NoteSummary {
    pub path: String,
    pub title: String,
}

pub trait NoteRepository {
    fn add_note(&self, title: &str, content: &str) -> Result<NoteSummary>;
    fn get_notes(&self) -> Result<Vec<NoteSummary>>;
    fn delete_notes(&self, notes: Vec<NoteSummary>) -> Result<()>;
}
