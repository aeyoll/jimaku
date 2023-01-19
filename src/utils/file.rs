use crate::utils::hash::compute;
use crate::utils::lang::Lang;
use crate::utils::mode::Mode;
use anyhow::{anyhow, Error};
use std::fs;
use std::{ffi::OsStr, path::PathBuf};

#[derive(Clone)]
pub struct File {
    pub filepath: PathBuf,
    pub lang: Lang,
    pub mode: Mode,
}

impl File {
    pub fn new(filepath: PathBuf, lang: Lang, mode: Mode) -> Self {
        File {
            filepath,
            lang,
            mode,
        }
    }

    pub fn get_filename(&self) -> &OsStr {
        self.filepath.file_name().unwrap()
    }

    pub fn get_subtitle_filename(&self) -> PathBuf {
        let mut subtitle_filename = self.filepath.clone();
        subtitle_filename.set_extension(format!("{}.srt", self.lang.code));
        subtitle_filename.to_path_buf()
    }

    pub fn get_hash(&self) -> (String, u64) {
        let file_path = self.filepath.clone().to_string_lossy().into_owned();
        compute(&file_path)
    }

    pub fn download(&self, contents: String) -> Result<(), Error> {
        let subtitle_filename = self.get_subtitle_filename().to_string_lossy().into_owned();

        match fs::write(subtitle_filename, contents) {
            Ok(_) => {
                log::info!("Subtitle successfully saved");
                Ok(())
            }
            Err(_) => Err(anyhow!("Unable to write subtitle file")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compute_subtitle_filename() {
        let file = File {
            filepath: PathBuf::from("~/path/file.mp4"),
            lang: Lang {
                code: "fr".to_owned(),
            },
            mode: Mode::TvShow,
        };
        assert_eq!(
            file.get_subtitle_filename().to_string_lossy().into_owned(),
            "~/path/file.fr.srt"
        );
    }
}
