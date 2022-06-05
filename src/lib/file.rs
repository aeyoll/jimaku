use std::{ffi::OsStr, path::PathBuf};

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
}
