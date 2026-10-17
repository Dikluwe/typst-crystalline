//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/test-support/wiring/cli.md
//! @prompt-hash 88d1b864
//! @layer L3
//! @updated 2026-10-16
//! Infraestrutura de teste; compilada apenas pelo harness L4.

use std::process::Child;

pub(super) struct WatchedChild(pub(super) Child);

impl WatchedChild {
    pub(super) fn assert_alive(&mut self, phase: &str) {
        match self.0.try_wait() {
            Ok(None) => {}
            Ok(Some(status)) => {
                panic!("watch terminou durante {phase} com status {status}")
            }
            Err(error) => {
                panic!("não foi possível consultar watch durante {phase}: {error}")
            }
        }
    }
}

impl Drop for WatchedChild {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
