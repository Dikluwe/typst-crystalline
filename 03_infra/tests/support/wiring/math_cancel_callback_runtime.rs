//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/support/wiring/math_cancel_callback_runtime.md
//! @prompt-hash c0611858
//! @layer L3
//! @updated 2026-10-16
//! Infraestrutura de teste; compilada apenas pelo harness L4.

use std::path::PathBuf;
use std::{env, fs};

pub(super) struct Scratch(pub(super) PathBuf);

impl Scratch {
    pub(super) fn new(name: &str) -> Self {
        let path = env::temp_dir().join(format!(
            "typst-p1291-callback-{name}-{}-{:?}",
            std::process::id(),
            std::thread::current().id(),
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create P1291 scratch directory");
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
