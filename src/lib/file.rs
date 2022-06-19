use crate::lib::hash::compute;
use anyhow::{anyhow, Error};
use std::fs;
use std::{ffi::OsStr, path::PathBuf};

#[derive(Clone)]
pub struct File {
    pub filepath: PathBuf,
}

impl File {
    pub fn new(filepath: PathBuf) -> Self {
        File { filepath }
    }

    pub fn get_filename(&self) -> &OsStr {
        self.filepath.file_name().unwrap()
    }

    pub fn get_subtitle_filename(&self) -> PathBuf {
        let mut subtitle_filename = self.filepath.clone();
        subtitle_filename.set_extension("srt");
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
                info!("Subtitle successfully saved");
                Ok(())
            }
            Err(_) => return Err(anyhow!("Unable to write subtitle file")),
        }
    }
}
