//! @prompt 00_nucleo/prompts/wiring/tests/p1467-format-pdf-tagged-pretty.md
//! @prompt-hash 9477993d
//!
//! P1467 (RED — Agente B) — integração de binário para
//! `#set format.pdf(tagged: ..., pretty: ...)` e as flags `--pdf-tagged`,
//! `--no-pdf-tags`, `--pretty`. Modelado em `p1464_format_pdf.rs` (superfície
//! `format.pdf`, sem export real) e `p1465_format_html.rs` (fixtures em
//! `/dev/shm`, resolução documento/CLI); ao contrário de ambos, este ficheiro
//! COMPILA PDFs de verdade e inspeciona os bytes produzidos — é o consumer
//! exclusivo do L0 `wiring/tests/p1467-format-pdf-tagged-pretty.md`.
//!
//! Não duplica a cobertura de superfície/gate de `a11y-extras` já fechada em
//! P1464, nem a matriz de perfis de feature — aqui o eixo é exclusivamente
//! `tagged`/`pretty` e a precedência CLI > documento > default.

#[path = "../../03_infra/tests/support/wiring/p1467_format_pdf_tagged_pretty.rs"]
mod harness;

use harness::ProcessResult;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const PROCESS_TIMEOUT: Duration = Duration::from_secs(60);
static FIXTURE_ID: AtomicU64 = AtomicU64::new(0);

fn typst_binary() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_typst"))
}

/// Raiz de fixtures descartáveis — exclusivamente `/dev/shm` (mesma
/// disciplina de `p1465_format_html.rs`), sem fallback ao disco.
fn shm_root() -> PathBuf {
    let tmp = PathBuf::from(std::env::var_os("TMPDIR").expect("TMPDIR must be explicit"));
    assert!(
        tmp.starts_with("/dev/shm"),
        "P1467 fixtures must stay in /dev/shm, got {}",
        tmp.display()
    );
    tmp
}

fn fixture_paths(name: &str) -> (PathBuf, PathBuf) {
    let tmp = shm_root();
    let id = FIXTURE_ID.fetch_add(1, Ordering::Relaxed);
    let input = tmp.join(format!("p1467-{name}-{}-{}.typ", std::process::id(), id));
    let output = input.with_extension("pdf");
    (input, output)
}

fn run_typst<I, S>(args: I) -> ProcessResult
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args: Vec<OsString> =
        args.into_iter().map(|arg| arg.as_ref().to_os_string()).collect();
    let mut child = Command::new(typst_binary())
        .args(&args)
        .env("NO_COLOR", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| {
            panic!("cannot spawn {} {:?}: {error}", typst_binary().display(), args)
        });

    let began = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                let output = child.wait_with_output().unwrap_or_else(|error| {
                    panic!("cannot collect {} output: {error}", typst_binary().display())
                });
                return ProcessResult {
                    code: output.status.code(),
                    stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                    stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
                };
            }
            Ok(None) if began.elapsed() < PROCESS_TIMEOUT => {
                thread::sleep(Duration::from_millis(20));
            }
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("Unknown: timeout after {:?}: {:?}", PROCESS_TIMEOUT, args);
            }
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("Unknown: cannot observe child: {error}");
            }
        }
    }
}

/// Compila `source` para PDF via subprocesso real (`typst compile`),
/// devolvendo o resultado do processo e os bytes produzidos (vazio se a
/// compilação falhou ou não escreveu o output).
fn compile_pdf(name: &str, source: &str, flags: &[&str]) -> (ProcessResult, Vec<u8>) {
    let (input, output) = fixture_paths(name);
    std::fs::write(&input, source).expect("write P1467 fixture in tmpfs");

    let mut args = vec![
        OsString::from("compile"),
        input.as_os_str().to_os_string(),
        output.as_os_str().to_os_string(),
    ];
    args.extend(flags.iter().map(OsString::from));

    let result = run_typst(args);
    let pdf = std::fs::read(&output).unwrap_or_default();

    let _ = std::fs::remove_file(&input);
    if output.exists() {
        let _ = std::fs::remove_file(&output);
    }
    (result, pdf)
}

fn has_tagged_pdf_morphology(pdf: &[u8]) -> bool {
    let raw = String::from_utf8_lossy(pdf);
    raw.contains("/StructTreeRoot") && raw.contains("/MarkInfo")
}

fn newline_count(pdf: &[u8]) -> usize {
    pdf.iter().filter(|&&byte| byte == b'\n').count()
}

#[test]
fn p1467_absence_of_set_rule_resolves_auto_to_tagged() {
    let (result, pdf) = compile_pdf("absence", "Hello", &[]);
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert!(
        has_tagged_pdf_morphology(&pdf),
        "auto sem seleção de páginas deve resolver tagueado por omissão"
    );
}

#[test]
fn p1467_document_tagged_false_untags_pdf() {
    let (result, pdf) =
        compile_pdf("doc-tagged-false", "#set format.pdf(tagged: false)\nHello", &[]);
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert!(
        !has_tagged_pdf_morphology(&pdf),
        "#set format.pdf(tagged: false) deve remover a struct tree"
    );
}

#[test]
fn p1467_document_tagged_true_tags_pdf() {
    let (result, pdf) =
        compile_pdf("doc-tagged-true", "#set format.pdf(tagged: true)\nHello", &[]);
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert!(has_tagged_pdf_morphology(&pdf));
}

#[test]
fn p1467_cli_pdf_tagged_false_wins_over_document_tagged_true() {
    let (result, pdf) = compile_pdf(
        "cli-false-wins",
        "#set format.pdf(tagged: true)\nHello",
        &["--pdf-tagged=false"],
    );
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert!(
        !has_tagged_pdf_morphology(&pdf),
        "--pdf-tagged=false deve vencer o documento"
    );
}

#[test]
fn p1467_cli_pdf_tagged_true_wins_over_document_tagged_false() {
    let (result, pdf) = compile_pdf(
        "cli-true-wins",
        "#set format.pdf(tagged: false)\nHello",
        &["--pdf-tagged=true"],
    );
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert!(has_tagged_pdf_morphology(&pdf), "--pdf-tagged=true deve vencer o documento");
}

#[test]
fn p1467_cli_no_pdf_tags_wins_over_document_tagged_true() {
    let (result, pdf) = compile_pdf(
        "legacy-no-pdf-tags-wins",
        "#set format.pdf(tagged: true)\nHello",
        &["--no-pdf-tags"],
    );
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert!(
        !has_tagged_pdf_morphology(&pdf),
        "--no-pdf-tags (legado) deve vencer o documento"
    );
}

#[test]
fn p1467_document_pretty_true_without_cli_override() {
    let (baseline_result, baseline_pdf) = compile_pdf("pretty-baseline", "Hello", &[]);
    assert!(
        baseline_result.success(),
        "stdout:\n{}\nstderr:\n{}",
        baseline_result.stdout,
        baseline_result.stderr
    );

    let (doc_result, doc_pdf) =
        compile_pdf("doc-pretty-true", "#set format.pdf(pretty: true)\nHello", &[]);
    assert!(
        doc_result.success(),
        "stdout:\n{}\nstderr:\n{}",
        doc_result.stdout,
        doc_result.stderr
    );

    assert!(
        newline_count(&doc_pdf) > newline_count(&baseline_pdf),
        "documento com pretty:true deve produzir mais newlines estruturais que o \
         baseline compacto"
    );
}

#[test]
fn p1467_cli_pretty_wins_over_document_pretty_false() {
    let (compact_result, compact_pdf) = compile_pdf(
        "pretty-doc-false-compact",
        "#set format.pdf(pretty: false)\nHello",
        &[],
    );
    assert!(
        compact_result.success(),
        "stdout:\n{}\nstderr:\n{}",
        compact_result.stdout,
        compact_result.stderr
    );

    let (pretty_result, pretty_pdf) = compile_pdf(
        "pretty-cli-wins",
        "#set format.pdf(pretty: false)\nHello",
        &["--pretty"],
    );
    assert!(
        pretty_result.success(),
        "stdout:\n{}\nstderr:\n{}",
        pretty_result.stdout,
        pretty_result.stderr
    );

    assert!(
        newline_count(&pretty_pdf) > newline_count(&compact_pdf),
        "--pretty deve vencer #set format.pdf(pretty: false) e produzir mais newlines"
    );
}

#[test]
fn p1467_tagged_and_pretty_combine_independently() {
    let (result, pdf) = compile_pdf(
        "combine-independent",
        "#set format.pdf(tagged: false)\n#set format.pdf(pretty: true)\nHello",
        &[],
    );
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert!(
        !has_tagged_pdf_morphology(&pdf),
        "tagged:false deve permanecer em vigor mesmo com pretty:true"
    );

    let (compact_result, compact_pdf) = compile_pdf(
        "combine-independent-compact-control",
        "#set format.pdf(tagged: false)\nHello",
        &[],
    );
    assert!(
        compact_result.success(),
        "stdout:\n{}\nstderr:\n{}",
        compact_result.stdout,
        compact_result.stderr
    );
    assert!(
        newline_count(&pdf) > newline_count(&compact_pdf),
        "pretty:true deve continuar a produzir mais newlines mesmo sem tagging"
    );
}

#[test]
fn p1467_nested_format_pdf_rule_inside_block_fails_with_canonical_message() {
    let (result, _pdf) = compile_pdf(
        "nested-block",
        "#block[\n#set format.pdf(tagged: false)\nNested\n]",
        &[],
    );
    assert!(
        !result.success(),
        "regra PDF aninhada num container deve falhar a compilação"
    );
    assert!(
        result
            .stderr
            .contains("format set rules are not allowed inside of containers"),
        "mensagem canônica ausente; stderr:\n{}",
        result.stderr
    );
}
