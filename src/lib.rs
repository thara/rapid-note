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

mod config;

pub mod cli;
pub mod repository;

pub use config::Config;

#[derive(Debug, Clone)]
pub struct NoteSummary {
    pub path: String,
    pub title: String,
}

pub struct NoteSummaryView {
    pub path: String,
    pub title: String,
}
