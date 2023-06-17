use bsdiff::{diff, patch};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::{Cursor, Read, Write};

use super::error::formatted_error::FmtError;

pub struct DiffHandler;

impl DiffHandler {
    pub fn get_delta(new_version: &String, old_version: Vec<u8>) -> Vec<u8> {
        let mut cursor = Vec::new();

        diff::diff(new_version.as_bytes(), &old_version, &mut cursor)
            .expect(FmtError::FailedToProcess("delta").fmt().as_str());

        let compressed_delta = Self::compress_bytes(cursor)
            .expect(FmtError::FailedToProcess("compressed_delta").fmt().as_str());

        compressed_delta
    }

    pub fn get_patch(delta: &Vec<u8>, length: i32, full_version: String) -> String {
        let delta = Self::decompress_bytes(delta.to_vec()).expect(
            FmtError::FailedToProcess("decompressed_delta")
                .fmt()
                .as_str(),
        );

        let mut cursor = Cursor::new(delta);

        let mut patched = vec![0; length as usize]; // TODO ? type
        patch::patch(full_version.as_bytes(), &mut cursor, &mut patched)
            .expect(FmtError::FailedToProcess("patch").fmt().as_str());

        let patched_string = Self::get_string_from_bytes(&patched);

        patched_string
    }

    pub fn get_string_from_bytes(input: &Vec<u8>) -> String {
        String::from_utf8(input.to_vec())
            .expect(FmtError::FailedToProcess("parse from utf8").fmt().as_str())
    }

    fn compress_bytes(input: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

        encoder.write_all(input.as_slice())?;
        encoder.finish()
    }

    fn decompress_bytes(compressed: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
        let mut decoder = ZlibDecoder::new(compressed.as_slice());
        let mut decompressed_data = Vec::new();

        decoder.read_to_end(&mut decompressed_data)?;
        Ok(decompressed_data)
    }
}
