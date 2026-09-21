//! @prompt 00_nucleo/prompts/wiring/tests/p1468-format-pdf-pages.md
//! @prompt-hash fb3f2fc1
//!
//! P1468 (RED — Agente B) — integração de binário para `pdf.pages`
//! (`#set format.pdf(pages: ...)`) e a flag `compile --pages`. Modelado em
//! `p1467_format_pdf_tagged_pretty.rs` (harness de subprocesso real,
//! fixtures em `/dev/shm`, inspeção de bytes crus); ao contrário desse
//! ficheiro, este exercita seleção de páginas: contagem de páginas
//! exportadas, a interação `pages`+`tagged` (erro documental, warning
//! `auto`, silêncio `false`) e a árvore de bookmarks nativa — na versão
//! RETIFICADA (headings de páginas incluídas sobrevivem; NÃO a variante
//! "outline sempre vazia" que é um bug identificado do vanilla e que o
//! dono decidiu não reproduzir — ver `infra/export/builder.md` §P1468).
//!
//! Não duplica a cobertura `tagged`/`pretty` isolada já fechada em P1467;
//! aqui o eixo é exclusivamente `pages` e sua interação com esses dois
//! campos.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const PROCESS_TIMEOUT: Duration = Duration::from_secs(60);
static FIXTURE_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct ProcessResult {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

impl ProcessResult {
    fn success(&self) -> bool {
        self.code == Some(0)
    }
}

fn typst_binary() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_typst"))
}

/// Raiz de fixtures descartáveis — exclusivamente `/dev/shm` (mesma
/// disciplina de `p1467_format_pdf_tagged_pretty.rs`), sem fallback ao disco.
fn shm_root() -> PathBuf {
    let tmp = PathBuf::from(std::env::var_os("TMPDIR").expect("TMPDIR must be explicit"));
    assert!(
        tmp.starts_with("/dev/shm"),
        "P1468 fixtures must stay in /dev/shm, got {}",
        tmp.display()
    );
    tmp
}

fn fixture_paths(name: &str) -> (PathBuf, PathBuf) {
    let tmp = shm_root();
    let id = FIXTURE_ID.fetch_add(1, Ordering::Relaxed);
    let input = tmp.join(format!("p1468-{name}-{}-{}.typ", std::process::id(), id));
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
    std::fs::write(&input, source).expect("write P1468 fixture in tmpfs");

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

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn count_occurrences(haystack: &[u8], needle: &[u8]) -> usize {
    let mut count = 0;
    let mut start = 0;
    while start <= haystack.len() {
        match find_subslice(&haystack[start..], needle) {
            Some(pos) => {
                count += 1;
                start += pos + needle.len();
            }
            None => break,
        }
    }
    count
}

/// Conta objetos de página individuais pelo padrão literal emitido pelo
/// builder cristalino (`03_infra/src/export/builder.rs`:
/// `"<< /Type /Page /Parent ..."`). O espaço final distingue de
/// `/Type /Pages` (nó da árvore de páginas), que nunca teria esse sufixo.
fn count_page_objects(pdf: &[u8]) -> usize {
    count_occurrences(pdf, b"/Type /Page ")
}

/// Extrai o primeiro `/Count N` (nó `/Type /Pages`) do PDF.
fn parsed_count_field(pdf: &[u8]) -> Option<usize> {
    let raw = String::from_utf8_lossy(pdf);
    let marker = "/Count ";
    let idx = raw.find(marker)?;
    let start = idx + marker.len();
    let mut digits = String::new();
    for byte in raw.as_bytes().iter().skip(start).copied() {
        if byte.is_ascii_digit() {
            digits.push(byte as char);
        } else {
            break;
        }
    }
    digits.parse().ok()
}

fn annots_count(pdf: &[u8]) -> usize {
    count_occurrences(pdf, b"/Annots")
}

/// Mirror byte-a-byte de `utf16be_hex_string` em
/// `03_infra/src/export/builder.rs` — usado para localizar o título de um
/// heading na árvore de bookmarks nativa (`/Title <hex>`).
fn utf16be_hex_string(s: &str) -> String {
    let mut bytes: Vec<u8> = Vec::with_capacity(2 + s.encode_utf16().count() * 2);
    bytes.extend_from_slice(&[0xFE, 0xFF]); // BOM
    for unit in s.encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    let hex: String = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    format!("<{hex}>")
}

fn has_outline_title(pdf: &[u8], title: &str) -> bool {
    let needle = format!("/Title {}", utf16be_hex_string(title));
    let raw = String::from_utf8_lossy(pdf);
    raw.contains(&needle)
}

fn has_outlines_catalog_entry(pdf: &[u8]) -> bool {
    String::from_utf8_lossy(pdf).contains("/Outlines")
}

// ── Contagem de páginas: campo documental `pdf.pages` ──────────────────

#[test]
fn p1468_document_pages_int_selects_single_page() {
    let (result, pdf) = compile_pdf(
        "doc-int",
        "#set format.pdf(pages: 2)\nFirst\n#pagebreak()\nSecond\n#pagebreak()\nThird\n#pagebreak()\nFourth",
        &[],
    );
    assert!(result.success(), "stderr:\n{}", result.stderr);
    assert_eq!(count_page_objects(&pdf), 1, "pages:2 deve exportar exatamente 1 página");
    assert_eq!(parsed_count_field(&pdf), Some(1));
}

#[test]
fn p1468_document_pages_dict_range_selects_two_pages() {
    let (result, pdf) = compile_pdf(
        "doc-dict-range",
        "#set format.pdf(pages: (from: 2, to: 3))\nFirst\n#pagebreak()\nSecond\n#pagebreak()\nThird\n#pagebreak()\nFourth",
        &[],
    );
    assert!(result.success(), "stderr:\n{}", result.stderr);
    assert_eq!(count_page_objects(&pdf), 2);
    assert_eq!(parsed_count_field(&pdf), Some(2));
}

#[test]
fn p1468_cli_pages_flag_selects_pages_without_document_decision() {
    let (result, pdf) = compile_pdf(
        "cli-flag",
        "First\n#pagebreak()\nSecond\n#pagebreak()\nThird\n#pagebreak()\nFourth",
        &["--pages=2,4"],
    );
    assert!(result.success(), "stderr:\n{}", result.stderr);
    assert_eq!(count_page_objects(&pdf), 2, "--pages=2,4 deve exportar 2 páginas");
}

#[test]
fn p1468_cli_pages_overrides_document_pages() {
    // Documento pede 1 página; CLI pede 3 — CLI deve vencer (precedência
    // `infra/pipeline.md`/`wiring.md` P1468).
    let (result, pdf) = compile_pdf(
        "cli-wins",
        "#set format.pdf(pages: 1)\nFirst\n#pagebreak()\nSecond\n#pagebreak()\nThird\n#pagebreak()\nFourth",
        &["--pages=2-4"],
    );
    assert!(result.success(), "stderr:\n{}", result.stderr);
    assert_eq!(
        count_page_objects(&pdf),
        3,
        "--pages deve vencer a decisão documental (1 página)"
    );
}

#[test]
fn p1468_page_beyond_document_end_does_not_crash() {
    // O L0 não especifica um fallback para uma seleção além do fim do
    // documento; não fixamos uma contagem de páginas como contrato — só
    // que o processo não morre por sinal (crash real, distinto de um
    // diagnóstico de erro controlado com exit code normal).
    let (result, _pdf) = compile_pdf(
        "beyond-end",
        "First\n#pagebreak()\nSecond\n#pagebreak()\nThird",
        &["--pages=10"],
    );
    assert!(
        result.code.is_some(),
        "processo não deve morrer por sinal (crash); stderr:\n{}",
        result.stderr
    );
}

// ── Interação `pages` + `tagged` ────────────────────────────────────────

#[test]
fn p1468_document_pages_and_tagged_true_is_compile_error() {
    let (result, _pdf) = compile_pdf(
        "doc-pages-tagged-true",
        "#set format.pdf(pages: 1, tagged: true)\nHello",
        &[],
    );
    assert!(!result.success(), "deveria falhar a compilação");
    assert!(
        result
            .stderr
            .contains("cannot enable tagged PDF and export a page range"),
        "mensagem documental ausente; stderr:\n{}",
        result.stderr
    );
}

#[test]
fn p1468_cli_pages_with_pdf_tagged_true_is_cli_only_error() {
    let (result, _pdf) = compile_pdf(
        "cli-pages-tagged-true",
        "Hello",
        &["--pages=1", "--pdf-tagged=true"],
    );
    assert!(!result.success(), "deveria falhar antes de compilar");
    assert_eq!(result.code, Some(1));
    assert!(
        result
            .stderr
            .contains("cannot enable PDF tags when exporting a page range"),
        "mensagem CLI-only ausente (diferente da documental); stderr:\n{}",
        result.stderr
    );
}

#[test]
fn p1468_document_pages_with_tagged_auto_warns_and_disables_tags() {
    let (result, pdf) =
        compile_pdf("doc-pages-tagged-auto", "#set format.pdf(pages: 1)\nHello", &[]);
    assert!(result.success(), "stderr:\n{}", result.stderr);
    assert!(!has_tagged_pdf_morphology(&pdf), "auto + pages deve desabilitar tagging");
    assert!(
        result.stderr.contains("exporting a page range disables PDF tagging"),
        "warning ausente; stderr:\n{}",
        result.stderr
    );
    assert!(
        result.stderr.contains("the resulting PDF will be inaccessible"),
        "hint 1 ausente; stderr:\n{}",
        result.stderr
    );
    assert!(
        result
            .stderr
            .contains("set `pdf(tagged: false)` to silence this warning"),
        "hint 2 ausente; stderr:\n{}",
        result.stderr
    );
}

#[test]
fn p1468_document_pages_with_tagged_false_is_silent_and_untagged() {
    let (result, pdf) = compile_pdf(
        "doc-pages-tagged-false",
        "#set format.pdf(pages: 1, tagged: false)\nHello",
        &[],
    );
    assert!(result.success(), "stderr:\n{}", result.stderr);
    assert!(!has_tagged_pdf_morphology(&pdf));
    assert!(
        !result.stderr.contains("disables PDF tagging"),
        "tagged:false explícito não deve emitir o warning de pages+auto; stderr:\n{}",
        result.stderr
    );
}

// ── Bookmarks nativos: versão RETIFICADA (não o bug do vanilla) ─────────

#[test]
fn p1468_native_bookmarks_survive_for_included_page_heading() {
    let (result, pdf) = compile_pdf(
        "outline-survive",
        "#set format.pdf(pages: 2)\n= First\n#pagebreak()\n= Second\n#pagebreak()\n= Third",
        &[],
    );
    assert!(result.success(), "stderr:\n{}", result.stderr);
    assert!(
        has_outlines_catalog_entry(&pdf),
        "divergência deliberada (builder.md P1468): headings de páginas incluídas \
         devem sobreviver — a árvore não deve ficar sempre vazia como no vanilla"
    );
    assert!(
        has_outline_title(&pdf, "Second"),
        "heading da página incluída deve aparecer no outline nativo"
    );
    assert!(
        !has_outline_title(&pdf, "First"),
        "heading de página excluída não deve aparecer"
    );
    assert!(
        !has_outline_title(&pdf, "Third"),
        "heading de página excluída não deve aparecer"
    );
}

#[test]
fn p1468_outline_child_is_reparented_when_parent_page_is_excluded() {
    let (result, pdf) = compile_pdf(
        "outline-reparent",
        "#set format.pdf(pages: 2)\n= Parent\n#pagebreak()\n== Child",
        &[],
    );
    assert!(result.success(), "stderr:\n{}", result.stderr);
    assert!(
        has_outline_title(&pdf, "Child"),
        "filho de nível mais profundo cuja página está incluída deve sobreviver, \
         reparentado ao ancestral incluído mais próximo (ou top-level)"
    );
    assert!(
        !has_outline_title(&pdf, "Parent"),
        "pai cuja página foi excluída não deve aparecer na árvore"
    );
}

// ── Links para páginas removidas ────────────────────────────────────────

#[test]
fn p1468_link_to_removed_page_omits_annotation() {
    // `link()` nesta base só aceita URL string (não label — gap pré-existente
    // e ortogonal a P1468); o mecanismo que produz `LinkTarget::Destination`
    // a partir de um label é a referência cruzada `@label`
    // (`compiler/layout/references.rs`), não `#outline()` (cujo próprio
    // mecanismo de sumário foi descartado como fonte de teste — ver
    // `00_nucleo/materialization/typst-passo-1468.md` §8.1 item 3). Heading
    // numerado para que `@target` seja referenciável.
    let source = "#set heading(numbering: \"1.\")\nSee @target.\n#pagebreak()\n= Target <target>\n#pagebreak()\nTrailer";

    let (baseline_result, baseline_pdf) = compile_pdf("link-baseline", source, &[]);
    assert!(baseline_result.success(), "stderr:\n{}", baseline_result.stderr);
    assert!(
        annots_count(&baseline_pdf) >= 1,
        "controle: sem seleção de páginas o link deve produzir pelo menos uma \
         annotation (prova que o mecanismo normal funciona)"
    );

    let (result, pdf) = compile_pdf("link-removed", source, &["--pages=1"]);
    assert!(result.success(), "stderr:\n{}", result.stderr);
    assert_eq!(
        annots_count(&pdf),
        0,
        "link cujo alvo caiu numa página excluída deve ter a annotation omitida, \
         não redirecionada"
    );
}
