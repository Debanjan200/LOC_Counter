use clap::ValueEnum;
use serde::Serialize;

#[derive(Default, Debug, Serialize)]
pub struct LineStats {
    pub total: usize,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Plain,
    Json,
}