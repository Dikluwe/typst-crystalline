//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/main.md
//! @prompt-hash 1c7d61a0
//! @layer L4
//! @updated 2026-09-26

mod compile;
mod completions;
mod diagnostics;
mod eval;
mod eviction;
mod fonts;
mod info;
mod init;
mod query;
mod watch;

use std::process::ExitCode;
use typst_infra::watch::Published;
use typst_shell::cli::{self, RunIntent};

fn main() -> ExitCode {
    match cli::parse() {
        RunIntent::Compile(intent) => compile::run_compile(intent),
        RunIntent::Watch(intent) => watch::run_watch(intent),
        RunIntent::Eval(intent) => eval::run_eval(intent),
        RunIntent::Query(intent) => query::run_query(intent),
        RunIntent::Fonts(intent) => fonts::run_fonts(intent),
        RunIntent::Completions(intent) => completions::run_completions(intent),
        RunIntent::Info(intent) => info::run_info(intent),
        RunIntent::Init(intent) => init::run_init(intent),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_is_only_a_closed_command_dispatcher() {
        let source = include_str!("main.rs");
        for name in [
            "compile",
            "watch",
            "eval",
            "query",
            "fonts",
            "completions",
            "info",
            "init",
            "diagnostics",
        ] {
            assert!(
                source.lines().any(|line| line == format!("mod {name};")),
                "missing module {name}"
            );
        }
        assert!(!source.lines().any(|line| line.starts_with("fn run_")));
        assert!(source.contains("match cli::parse()"));
    }
}
