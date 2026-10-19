//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/support/wiring/format_html.md
//! @prompt-hash 129b9e94
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

    pub(super) fn expect_json(
        &mut self,
        label: &str,
        observed: &ProcessResult,
        expected: &str,
    ) {
        self.require(
            observed.success(),
            format!("{label}: expected exit 0, got {observed:?}"),
        );
        self.require(
            observed.stderr.is_empty(),
            format!("{label}: expected empty stderr, got {:?}", observed.stderr),
        );
        self.require(
            observed.stdout.trim_end() == expected,
            format!("{label}: expected {expected:?}, got {:?}", observed.stdout),
        );
    }

    pub(super) fn finish(self) {
        assert!(
            self.failures.is_empty(),
            "P1465 contract failures:\n- {}",
            self.failures.join("\n- ")
        );
    }
}
