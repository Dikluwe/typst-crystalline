//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/support/wiring/math_decoration_and_float_flush.md
//! @prompt-hash 7cb2be8f
//! @layer L3
//! @updated 2026-10-16
//! Infraestrutura de teste; compilada apenas pelo harness L4.

use std::path::PathBuf;
use std::{env, fs};

pub(super) struct Scratch(pub(super) PathBuf);

impl Scratch {
    pub(super) fn new(name: &str) -> Self {
        let path = env::temp_dir().join(format!(
            "typst-p1292-{name}-{}-{:?}",
            std::process::id(),
            std::thread::current().id(),
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create P1292 scratch directory");
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Clone, Copy)]
pub(super) struct SvgTransform {
    pub(super) a: f64,
    pub(super) b: f64,
    pub(super) c: f64,
    pub(super) d: f64,
    pub(super) e: f64,
    pub(super) f: f64,
}

impl SvgTransform {
    pub(super) const IDENTITY: Self =
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 };

    pub(super) fn compose(self, inner: Self) -> Self {
        Self {
            a: self.a * inner.a + self.c * inner.b,
            b: self.b * inner.a + self.d * inner.b,
            c: self.a * inner.c + self.c * inner.d,
            d: self.b * inner.c + self.d * inner.d,
            e: self.a * inner.e + self.c * inner.f + self.e,
            f: self.b * inner.e + self.d * inner.f + self.f,
        }
    }
}
