//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/support/wiring/format_pdf_pages.md
//! @prompt-hash 1915277b
//! @layer L3
//! @updated 2026-10-16
//! Infraestrutura de teste; compilada apenas pelo harness L4.

#[derive(Debug)]
pub(super) struct ProcessResult {
    pub(super) code: Option<i32>,
    pub(super) stdout: String,
    pub(super) stderr: String,
}

impl ProcessResult {
    pub(super) fn success(&self) -> bool {
        self.code == Some(0)
    }
}
