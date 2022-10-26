use std::path::Path;

mod ffmpeg;

pub trait Transcoder {
    fn transcode(file_path: &'_ Path);
}

pub fn needs_to_transcode() -> bool {
    false
}
