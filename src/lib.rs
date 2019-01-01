#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate regex;
extern crate serde;
extern crate shellexpand;
#[macro_use]
extern crate serde_derive;
extern crate strfmt;
extern crate toml;

pub mod errors;
mod note;

mod config;

pub mod cli;
pub mod fs;

pub use config::Config;
pub use fs::FileNoteStore;
pub use note::NoteSummary;

pub struct NoteSummaryView {
    pub path: String,
    pub title: String,
}
