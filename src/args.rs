use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Filepath
    #[clap(parse(from_os_str))]
    pub filepath: Option<std::path::PathBuf>,

    /// Language
    #[clap(short, long, default_value_t = String::from("VO"))]
    pub language: String,
}
