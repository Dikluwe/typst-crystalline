//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/support/wiring/format_svg.md
//! @prompt-hash 173d3760
//! @layer L3
//! @updated 2026-10-16
//! Infraestrutura de teste; compilada apenas pelo harness L4.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

pub(super) struct TempDir {
    pub(super) path: PathBuf,
}

impl TempDir {
    pub(super) fn new(label: &str) -> Self {
        loop {
            let suffix = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("typst-p1462-{label}-{}-{suffix}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Self { path },
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    continue
                }
                Err(error) => panic!("cannot create {}: {error}", path.display()),
            }
        }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[derive(Default)]
pub(super) struct Checks {
    failures: Vec<String>,
}

impl Checks {
    pub(super) fn require(&mut self, condition: bool, message: impl Into<String>) {
        if !condition {
            self.failures.push(message.into());
        }
    }

    pub(super) fn finish(self) {
        assert!(
            self.failures.is_empty(),
            "P1462 contract failures:\n- {}",
            self.failures.join("\n- ")
        );
    }
}

pub(super) fn read_svg_pages(directory: &Path) -> Result<Vec<Vec<u8>>, String> {
    let mut pages = BTreeMap::new();
    for entry in fs::read_dir(directory).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("svg") {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("invalid SVG filename: {}", path.display()))?;
        let page = name
            .strip_prefix("main-")
            .and_then(|name| name.strip_suffix(".svg"))
            .and_then(|number| number.parse::<usize>().ok())
            .filter(|page| *page > 0)
            .ok_or_else(|| format!("invalid numbered SVG filename: {name}"))?;
        if name != format!("main-{page}.svg") {
            return Err(format!("noncanonical SVG page filename: {name}"));
        }
        let bytes = fs::read(&path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        if pages.insert(page, bytes).is_some() {
            return Err(format!("duplicate SVG page {page}"));
        }
    }
    if pages.is_empty() {
        return Err("no numbered SVG pages emitted".into());
    }
    for (index, page) in pages.keys().enumerate() {
        if *page != index + 1 {
            return Err(format!("missing SVG page {} before page {page}", index + 1));
        }
    }
    Ok(pages.into_values().collect())
}
