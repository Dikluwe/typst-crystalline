//! @prompt 00_nucleo/prompts/wiring/tests/p1469-format-pdf-standard.md
//! @prompt-hash e526464d
//!
//! P1469 (RED — Agente B) — integração de binário para `pdf.standard`
//! (`#set format.pdf(standard: ...)`) e a flag `compile --pdf-standard`.
//! Modelado em `p1468_format_pdf_pages.rs` (harness de subprocesso real,
//! fixtures em `/dev/shm`, inspeção de bytes crus); ao contrário desse
//! ficheiro, este exercita o CONTRATO DE LINGUAGEM de `standard` — ESCOPO
//! REDUZIDO por decisão do dono (2026-09-21, ver
//! `entities/document_format_options.md` P1469 e
//! `lab/history/construction/records/00_nucleo/materialization/typst-passo-1469.md` §7.1): só parsing dos 17
//! nomes, validação de conflitos, versão efetiva do header `%PDF-{version}` e
//! interação com `tagged`/`pages` já materializada em P1467/P1468. NÃO
//! verifica conformidade PDF/A ou PDF/UA estrutural real (sem identificação
//! XMP com nível de conformidade, sem restrições de cor/fontes, sem
//! contraste de acessibilidade) — isso é scope-out explícito, fora deste
//! ficheiro e deste passo.

#[path = "../../03_infra/tests/support/wiring/p1469_format_pdf_standard.rs"]
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
/// disciplina de `p1468_format_pdf_pages.rs`), sem fallback ao disco.
fn shm_root() -> PathBuf {
    let tmp = PathBuf::from(std::env::var_os("TMPDIR").expect("TMPDIR must be explicit"));
    assert!(
        tmp.starts_with("/dev/shm"),
        "P1469 fixtures must stay in /dev/shm, got {}",
        tmp.display()
    );
    tmp
}

fn fixture_paths(name: &str) -> (PathBuf, PathBuf) {
    let tmp = shm_root();
    let id = FIXTURE_ID.fetch_add(1, Ordering::Relaxed);
    let input = tmp.join(format!("p1469-{name}-{}-{}.typ", std::process::id(), id));
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
    std::fs::write(&input, source).expect("write P1469 fixture in tmpfs");

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

/// Extrai a string de versão do header `%PDF-{version}` (até o próximo
/// `\r`/`\n`), espelhando o literal `format!("%PDF-{}", ...)` de
/// `infra/export/builder.md` P1469. `None` se o ficheiro não começar com o
/// marcador `%PDF-`.
fn pdf_header_version(pdf: &[u8]) -> Option<String> {
    let prefix = b"%PDF-";
    if !pdf.starts_with(prefix) {
        return None;
    }
    let mut version = String::new();
    for &byte in pdf[prefix.len()..].iter() {
        if byte == b'\n' || byte == b'\r' {
            break;
        }
        version.push(byte as char);
    }
    Some(version)
}

// ── Contrato de linguagem: header `%PDF-{version}` dinâmico ────────────────

#[test]
fn p1469_document_standard_explicit_version_sets_header() {
    let (result, pdf) =
        compile_pdf("doc-version-1-4", "#set format.pdf(standard: \"1.4\")\nHello", &[]);
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert_eq!(pdf_header_version(&pdf).as_deref(), Some("1.4"));
}

#[test]
fn p1469_document_standard_archival_without_explicit_version_uses_range_maximum() {
    // `a-2b` (range 1.4..=1.7, sem versão explícita) -> efetivo 1.7 (máximo
    // do range), medido em `krilla/src/configure/mod.rs:68-96`.
    let (result, pdf) =
        compile_pdf("doc-a2b", "#set format.pdf(standard: \"a-2b\")\nHello", &[]);
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert_eq!(pdf_header_version(&pdf).as_deref(), Some("1.7"));
}

#[test]
fn p1469_document_standard_a1b_without_explicit_version_uses_1_4() {
    // `a-1b` (range 1.4..=1.4, min == max) -> efetivo 1.4.
    let (result, pdf) =
        compile_pdf("doc-a1b", "#set format.pdf(standard: \"a-1b\")\nHello", &[]);
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert_eq!(pdf_header_version(&pdf).as_deref(), Some("1.4"));
}

#[test]
fn p1469_cli_pdf_standard_flag_sets_header_without_document_decision() {
    let (result, pdf) = compile_pdf("cli-2-0", "Hello", &["--pdf-standard=2.0"]);
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert_eq!(pdf_header_version(&pdf).as_deref(), Some("2.0"));
}

#[test]
fn p1469_cli_pdf_standard_overrides_document_standard() {
    // Documento pede `a-1b` (-> efetivo 1.4); CLI pede `a-4` (-> efetivo
    // 2.0). CLI deve vencer (precedência `infra/pipeline.md`/`wiring.md`
    // P1469, mesmo Núcleo `format-option-precedence` de P1468).
    let (result, pdf) = compile_pdf(
        "cli-wins-standard",
        "#set format.pdf(standard: \"a-1b\")\nHello",
        &["--pdf-standard=a-4"],
    );
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert_eq!(
        pdf_header_version(&pdf).as_deref(),
        Some("2.0"),
        "--pdf-standard deve vencer a decisão documental"
    );
}

#[test]
fn p1469_no_standard_decision_anywhere_keeps_default_header_1_7() {
    // Regressão zero: sem `pdf.standard` (nem documento, nem CLI), o header
    // continua exatamente `%PDF-1.7` (comportamento fixado por P1462-P1468
    // e pelos ~8 testes pré-existentes de `export/tests.rs`).
    let (result, pdf) = compile_pdf("no-standard", "Hello", &[]);
    assert!(result.success(), "stdout:\n{}\nstderr:\n{}", result.stdout, result.stderr);
    assert_eq!(pdf_header_version(&pdf).as_deref(), Some("1.7"));
}

// ── Interação `standard` + `tagged` (documental) ────────────────────────────

#[test]
fn p1469_document_accessible_profile_with_tagged_false_is_compile_error() {
    let (result, _pdf) = compile_pdf(
        "doc-a1a-tagged-false",
        "#set format.pdf(standard: \"a-1a\", tagged: false)\nHello",
        &[],
    );
    assert!(!result.success(), "deveria falhar a compilação");
    assert!(
        result
            .stderr
            .contains("cannot set `pdf(tagged: false)` when exporting a"),
        "mensagem documental ausente; stderr:\n{}",
        result.stderr
    );
}

// ── Interação `standard` + `tagged`/`pages` (gate CLI-only) ─────────────────

#[test]
fn p1469_cli_accessible_profile_with_pdf_tagged_false_is_cli_only_error() {
    // Medido exatamente em `shell/cli.md` P1469 via `typst compile`
    // 2026-09-21: mensagem "cannot disable..." (distinta da documental
    // "cannot set..."), sem hint (sem `--pages`), exit 1.
    let (result, _pdf) = compile_pdf(
        "cli-a1a-tagged-false",
        "Hello",
        &["--pdf-standard=a-1a", "--pdf-tagged=false"],
    );
    assert!(!result.success(), "deveria falhar antes de compilar");
    assert_eq!(result.code, Some(1));
    assert!(
        result
            .stderr
            .contains("cannot disable PDF tags when exporting a PDF/A-1a document"),
        "mensagem CLI-only exata ausente; stderr:\n{}",
        result.stderr
    );
    assert!(
        !result.stderr.contains("implies --pdf-tagged=false"),
        "sem --pages, não deve haver hint; stderr:\n{}",
        result.stderr
    );
}

#[test]
fn p1469_cli_accessible_profile_with_pages_and_no_explicit_tagged_has_pages_hint() {
    // Medido exatamente em `shell/cli.md` P1469: `ua-1` + `--pages` sem
    // `--pdf-tagged` explícito produz a mesma mensagem "cannot disable..."
    // COM o hint de `--pages`, exit 1.
    let (result, _pdf) =
        compile_pdf("cli-ua1-pages", "Hello", &["--pdf-standard=ua-1", "--pages=1"]);
    assert!(!result.success(), "deveria falhar antes de compilar");
    assert_eq!(result.code, Some(1));
    assert!(
        result
            .stderr
            .contains("cannot disable PDF tags when exporting a PDF/UA-1 document"),
        "mensagem CLI-only exata ausente; stderr:\n{}",
        result.stderr
    );
    assert!(
        result.stderr.contains("using --pages implies --pdf-tagged=false"),
        "hint de --pages ausente (causa raiz foi pages); stderr:\n{}",
        result.stderr
    );
}

// ── Conflito entre nomes na CLI ─────────────────────────────────────────────

#[test]
fn p1469_cli_pdf_standard_conflicting_versions_is_rejected_before_compiling() {
    // `wiring.md` P1469: conflito entre nomes da própria lista `--pdf-standard`
    // é validado por `PdfStandards::new` e aborta com a mesma mensagem
    // documental de `compiler/eval/rules.rs` P1469 (função reaproveitada).
    let (result, _pdf) =
        compile_pdf("cli-conflict", "Hello", &["--pdf-standard=1.4,1.7"]);
    assert!(!result.success(), "deveria falhar antes de compilar");
    assert_eq!(result.code, Some(1));
    assert!(
        result
            .stderr
            .contains("PDF cannot conform to PDF 1.4 and PDF 1.7 at the same time"),
        "mensagem de conflito reaproveitada ausente; stderr:\n{}",
        result.stderr
    );
}
