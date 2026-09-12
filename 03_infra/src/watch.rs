//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/watch.md
//! @prompt-hash e2688d1f
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

/// Opaque baseline for a set of observed filesystem paths.
pub struct WatchSnapshot {
    entries: Vec<(PathBuf, Option<Fingerprint>)>,
}

/// Capability for finalizing a watch cycle after its baseline was captured.
pub struct ArmedWatch {
    snapshot: WatchSnapshot,
}

/// Captures the current state of each observed path exactly once.
pub fn snapshot(paths: &[PathBuf]) -> WatchSnapshot {
    WatchSnapshot {
        entries: paths.iter().map(|path| (path.clone(), fingerprint(path))).collect(),
    }
}

/// Captures the observed paths immediately and arms cycle finalization.
pub fn arm(paths: &[PathBuf]) -> ArmedWatch {
    ArmedWatch { snapshot: snapshot(paths) }
}

impl ArmedWatch {
    /// Atomically publishes staging and returns the already captured baseline.
    pub fn publish(
        self,
        staging: &Path,
        destination: &Path,
    ) -> io::Result<WatchSnapshot> {
        match fs::rename(staging, destination) {
            Ok(()) => Ok(self.snapshot),
            Err(error) => {
                let _ = fs::remove_file(staging);
                Err(error)
            }
        }
    }

    /// Abandons staging best-effort and returns the already captured baseline.
    pub fn abandon(self, staging: &Path) -> WatchSnapshot {
        let _ = fs::remove_file(staging);
        self.snapshot
    }
}

/// Blocks until one of the snapshotted paths changes, appears, or disappears.
pub fn wait_for_change_since(snapshot: WatchSnapshot, interval: Duration) {
    loop {
        thread::sleep(interval);
        if snapshot
            .entries
            .iter()
            .any(|(path, previous)| fingerprint(path) != *previous)
        {
            return;
        }
    }
}

/// Blocks until one of the observed paths changes, appears, or disappears.
pub fn wait_for_change(paths: &[PathBuf], interval: Duration) {
    wait_for_change_since(snapshot(paths), interval);
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
