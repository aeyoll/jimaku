mod args;
mod lib;

use crate::lib::lang::Lang;
use crate::lib::mode::Mode;
use crate::lib::providers::betaseries::BetaSeriesProvider;
use crate::lib::providers::opensubtitles::OpenSubtitleProvider;
use crate::lib::providers::Providers;
use anyhow::Error;
use args::Args;
use clap::Parser;
use lib::file::File;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::path::PathBuf;

#[macro_use]
extern crate log;
extern crate simplelog;

fn run_app() -> Result<(), Error> {
    // Define the logger
    let level = LevelFilter::Info;
    TermLogger::init(
        level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    let args = Args::parse();
    let filepath: PathBuf = args.filepath;
    let mode: Mode = args.mode;
    let language = Lang {
        code: args.language,
    };

    let file = File::new(filepath, language.clone(), mode.clone());

    let mut providers = Providers::new();
    if mode == Mode::TvShow {
        let bs = BetaSeriesProvider::new(file.clone()).unwrap();
        providers.push(bs);
    }

    let osp = OpenSubtitleProvider::new(file).unwrap();
    providers.push(osp);

    providers.run(language).unwrap();

    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            error!("{}", err.to_string());
            1
        }
    });
}
