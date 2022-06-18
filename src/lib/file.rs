use anyhow::{anyhow, Error};
use std::fs;
use std::fs::File as FsFile;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::mem;
use std::{ffi::OsStr, path::PathBuf};

const HASH_BLK_SIZE: u64 = 65536;

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

    pub unsafe fn create_hash(&self) -> Result<String, std::io::Error> {
        let fsize = fs::metadata(&self.filepath).unwrap().len();
        let file = FsFile::open(&self.filepath)?;

        let mut buf = [0u8; 8];
        let mut word: u64;

        let mut hash_val: u64 = fsize; // seed hash with file size

        let iterations = HASH_BLK_SIZE / 8;

        let mut reader = BufReader::with_capacity(HASH_BLK_SIZE as usize, file);

        for _ in 0..iterations {
            reader.read(&mut buf)?;

            word = mem::transmute(buf);
            hash_val = hash_val.wrapping_add(word);
        }

        reader.seek(SeekFrom::Start(fsize - HASH_BLK_SIZE))?;

        for _ in 0..iterations {
            reader.read(&mut buf)?;

            word = mem::transmute(buf);
            hash_val = hash_val.wrapping_add(word);
        }

        let hash_string = format!("{:01$x}", hash_val, 16);

        Ok(hash_string)
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
