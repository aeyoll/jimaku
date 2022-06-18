mod args;
mod lib;

use crate::lib::lang::Lang;
use crate::lib::providers::betaseries::{BetaSeriesLang, BetaSeriesProvider};
use crate::lib::providers::HttpProvider;
use anyhow::{anyhow, Error};
use args::Args;
use clap::Parser;
use lib::file::File;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::{env, fs};

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
    let language = BetaSeriesLang::from_code(&args.language).unwrap();

    let filepath = args.filepath.unwrap();
    let file = File::new(filepath);

    let filename = file.get_filename().to_string_lossy().into_owned();
    let subtitle_filename = file.get_subtitle_filename().to_string_lossy().into_owned();

    info!("Searching subtitle for \"{}\"", filename);

    let bs = BetaSeriesProvider::new().unwrap();
    let query = filename;

    let subtitle = match bs.search_subtitle(query, &language) {
        Ok((episode, subtitle)) => {
            info!(
                "Found subtitle for {}: {} ({})",
                episode.show.title, episode.title, episode.code
            );
            subtitle
        }
        Err(_) => return Err(anyhow!("No subtitle found for this episode")),
    };

    let contents = match bs.download_subtitle(subtitle) {
        Ok(contents) => contents,
        Err(_) => return Err(anyhow!("Failed to download the subtitle")),
    };

    match fs::write(subtitle_filename, contents) {
        Ok(_) => info!("Subtitle successfully saved"),
        Err(_) => return Err(anyhow!("Unable to write subtitle file")),
    }

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
