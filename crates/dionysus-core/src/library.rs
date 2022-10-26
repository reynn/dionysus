use std::path::PathBuf;

pub struct Library {
    c: Config,
}
pub struct Config {
    paths: Vec<PathBuf>,
}

impl Library {
    pub fn initialize(cfg: Config) -> Library {
        Self { c: cfg }
    }
}
