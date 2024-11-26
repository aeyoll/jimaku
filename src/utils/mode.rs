#[derive(clap::ValueEnum, Default, PartialEq, Eq, Debug, Clone)]
pub enum Mode {
    #[default]
    Movie,
    TvShow,
}
