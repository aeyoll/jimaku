use crate::lib::mode::Mode;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Filepath
    #[clap(value_parser)]
    pub filepath: std::path::PathBuf,

    /// Language
    #[clap(short, long, default_value_t = String::from("en"))]
    pub language: String,

    /// Mode
    #[clap(short, long, value_enum, default_value_t)]
    pub mode: Mode,
}
