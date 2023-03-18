use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::fetcher::State;

pub struct Loader {
    path: PathBuf,
    pub state: State,
}

impl Loader {
    pub fn new(cwd: &Path, repo: &str) -> Self {
        let (path, base_url) = normalize_args(cwd, repo);
        let state = State::new(base_url);
        Self { path, state }
    }

    pub fn load_or_create(cwd: &Path, repo: &str) -> anyhow::Result<Self> {
        let (path, base_url) = normalize_args(cwd, repo);
        match fs::File::open(&path) {
            Ok(file) => {
                let state: State = serde_json::from_reader(file)?;
                anyhow::ensure!(state.base_url == base_url);
                tracing::debug!(
                    ?path,
                    dependents = state.dependents.len(),
                    chunks = ?state.chunks,
                    "loaded persisted state from file"
                );
                Ok(Self { path, state })
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                let state = State::new(base_url);
                Ok(Self { path, state })
            }
            Err(err) => Err(err.into()),
        }
    }

    pub fn store(&self) -> anyhow::Result<()> {
        let file = fs::File::create(&self.path)?;
        serde_json::to_writer_pretty(file, &self.state)?;
        tracing::debug!(
            path = ?self.path,
            dependents = self.state.dependents.len(),
            chunks = ?self.state.chunks,
            "persisted state to file"
        );
        Ok(())
    }
}

fn normalize_args(cwd: &Path, repo: &str) -> (PathBuf, String) {
    let base_url = format!("https://github.com/{repo}/network/dependents");
    let mut safe_filename = repo.replace(|c: char| !c.is_ascii_alphanumeric(), "_");
    safe_filename.push_str(".json");

    let path = cwd.join(safe_filename);

    (path, base_url)
}
