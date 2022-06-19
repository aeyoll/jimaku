use std::fs;
use std::io;
use std::io::prelude::*;

fn checksum_64k(file: &mut fs::File, position: io::SeekFrom, buffer: &mut [u8; 64 * 1024]) -> u64 {
    // Compute a checksum of the given file.
    // This checksum is defined as sum of 2 unsigned integers, each one being
    // 64 kB converted to a u64. One sum is done on the first 64 kB of the file
    // while the other is done on the last 64 kB.
    let mut hash: u64 = 0;

    // Load 64 kB from the file
    file.seek(position).expect("could not seek into the file");
    file.read_exact(buffer).expect("could not read from file");

    // Compute the checksum
    for chunk in buffer.chunks(8) {
        hash = hash.wrapping_add(to_u64(&chunk));
    }

    hash
}

fn to_u64(bytes: &[u8]) -> u64 {
    // Convert a slice of bytes to a 64bit unsigned integer, necessary for
    // computing the OpenSubtitles hash.
    (bytes[0] as u64) << (8 * 0)
        | (bytes[1] as u64) << (8 * 1)
        | (bytes[2] as u64) << (8 * 2)
        | (bytes[3] as u64) << (8 * 3)
        | (bytes[4] as u64) << (8 * 4)
        | (bytes[5] as u64) << (8 * 5)
        | (bytes[6] as u64) << (8 * 6)
        | (bytes[7] as u64) << (8 * 7)
}

pub fn compute(file_path: &String) -> (String, u64) {
    // Compute the OpenSubtitles hash of a file.
    // This implies having the file size and a checksum, more info at:
    // http://trac.opensubtitles.org/projects/opensubtitles/wiki/HashSourceCodes

    // File size
    let metadata = fs::metadata(file_path).expect("could not access file metadata");
    let file_size: u64 = metadata.len();
    let mut hash: u64 = file_size;

    // Open the file for reading
    let mut file = fs::File::open(file_path).expect("failed to open file");

    // Create a buffer of 64 kB
    let mut buffer = [0u8; 64 * 1024];

    // Read the first 64 kB
    let start = io::SeekFrom::Start(0);
    hash = hash.wrapping_add(checksum_64k(&mut file, start, &mut buffer));

    // Read the last 64 kB
    let end = io::SeekFrom::End(-64 * 1024);
    hash = hash.wrapping_add(checksum_64k(&mut file, end, &mut buffer));

    // Convert hash to hexadecimal
    (format!("{:016x}", hash), file_size)
}
