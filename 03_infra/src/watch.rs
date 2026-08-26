//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/watch.md
//! @prompt-hash 0f218b5b
//! @layer L3
//! @updated 2026-08-23

//! Filesystem observation and atomic artifact replacement for `typst watch`.

use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, SystemTime};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Fingerprint {
    modified: Option<SystemTime>,
    len: u64,
    content_hash: u64,
}

fn fingerprint(path: &Path) -> Option<Fingerprint> {
    let metadata = fs::metadata(path).ok()?;
    let bytes = fs::read(path).ok()?;
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    Some(Fingerprint {
        modified: metadata.modified().ok(),
        len: metadata.len(),
        content_hash: hasher.finish(),
    })
}

/// Blocks until one of the observed paths changes, appears, or disappears.
pub fn wait_for_change(paths: &[PathBuf], interval: Duration) {
    let baseline = paths
        .iter()
        .map(|path| (path.clone(), fingerprint(path)))
        .collect::<Vec<_>>();
    loop {
        thread::sleep(interval);
        if baseline.iter().any(|(path, previous)| fingerprint(path) != *previous) {
            return;
        }
    }
}

/// Moves a completed staging artifact into place in one filesystem operation.
pub fn commit_output(staging: &Path, destination: &Path) -> io::Result<()> {
    fs::rename(staging, destination)
}

/// Removes an abandoned staging artifact, if it exists.
pub fn discard_output(staging: &Path) {
    let _ = fs::remove_file(staging);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_content_change_even_when_length_is_stable() {
        let path = std::env::temp_dir()
            .join(format!("crystalline-watch-fingerprint-{}", std::process::id()));
        fs::write(&path, "one").unwrap();
        let before = fingerprint(&path);
        fs::write(&path, "two").unwrap();
        assert_ne!(before, fingerprint(&path));
        let _ = fs::remove_file(path);
    }
}
