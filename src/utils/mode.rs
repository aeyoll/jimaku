#[derive(clap::ValueEnum, PartialEq, Eq, Debug, Clone)]
pub enum Mode {
    Movie,
    TvShow,
}

impl Default for Mode {
    fn default() -> Self {
        Self::TvShow
    }
}
