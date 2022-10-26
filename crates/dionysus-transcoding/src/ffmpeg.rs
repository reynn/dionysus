use {
    crate::Transcoder,
    std::path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Ffmpeg {
    bin_path: PathBuf,
}

impl Ffmpeg {
    pub fn new(bin_path: PathBuf) -> Self {
        Self { bin_path }
    }
}

impl Transcoder for Ffmpeg {
    fn transcode(file_path: &'_ Path) {}
}
