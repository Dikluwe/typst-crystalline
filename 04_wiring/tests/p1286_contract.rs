//! Testes black-box funcionais para smart quotes e exportação PDF.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_typst");

struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let path = env::temp_dir().join(format!(
            "typst-p1286-{name}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create P1286 scratch directory");
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn diagnostic(output: &Output) -> String {
    format!(
        "exit={:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    )
}

fn compile_pdf(name: &str, source: &str) -> (Scratch, PathBuf) {
    let scratch = Scratch::new(name);
    let input = scratch.0.join("main.typ");
    let output = scratch.0.join("main.pdf");
    fs::write(&input, source).expect("write P1286 fixture");
    let result = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .output()
        .expect("run crystalline candidate");
    assert!(result.status.success(), "{}", diagnostic(&result));
    assert!(output.is_file(), "candidate did not create {}", output.display());
    (scratch, output)
}

fn pdf_text(name: &str, source: &str) -> String {
    let (scratch, pdf) = compile_pdf(name, source);
    let target = scratch.0.join("text.txt");
    let result = Command::new("pdftotext")
        .arg(&pdf)
        .arg(&target)
        .output()
        .expect("run pdftotext");
    assert!(result.status.success(), "{}", diagnostic(&result));
    fs::read_to_string(target)
        .expect("read extracted text")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn qdf(name: &str, source: &str) -> String {
    let (scratch, pdf) = compile_pdf(name, source);
    let target = scratch.0.join("main.qdf.pdf");
    let result = Command::new("qpdf")
        .args(["--qdf", "--object-streams=disable"])
        .arg(&pdf)
        .arg(&target)
        .output()
        .expect("run qpdf");
    assert!(result.status.success(), "{}", diagnostic(&result));
    String::from_utf8_lossy(&fs::read(target).expect("read QDF")).into_owned()
}

#[test]
fn p1286_smartquote_explicit_auto_erases_inherited_quotes() {
    let text = pdf_text(
        "smartquote-auto",
        "#set text(lang: \"de\")\n#set smartquote(quotes: \"()\")\n#smartquote(quotes: auto)X#smartquote(quotes: auto)",
    );
    assert_eq!(text, "„X“");
}

#[test]
fn p1286_smartquote_counts_unicode_graphemes() {
    let text = pdf_text(
        "smartquote-graphemes",
        "#smartquote(quotes: \"a\u{301}b\")X#smartquote(quotes: \"a\u{301}b\")",
    );
    assert_eq!(text, "a\u{301}Xb");
}

#[test]
fn p1286_attachment_preserves_virtual_path_and_metadata() {
    let data = qdf(
        "attachment-path-metadata",
        "#pdf.attach(\"dir/virtual.bin\", bytes((0, 65, 255)), relationship: \"supplement\", mime-type: \"application/octet-stream\", description: \"three bytes\")",
    );
    assert!(data.contains("/EmbeddedFiles"), "missing attachment name tree");
    assert!(data.contains("dir/virtual.bin"), "virtual path was not preserved");
    assert!(data.contains("/Desc (three bytes)"), "description was not preserved");
    assert!(
        data.contains("/Subtype /application#2foctet-stream")
            || data.contains("/Subtype /application#2Foctet-stream"),
        "MIME subtype was not preserved"
    );
}

#[test]
fn p1286_artifact_suppresses_descendant_formula_structure_only_locally() {
    let data = qdf("artifact-descendant-formula", "#pdf.artifact[$x$]\n$y$");
    let start = data.find("/Artifact").expect("artifact marked-content opening");
    let tail = &data[start..];
    let end = tail.find("EMC").expect("balanced artifact marked content");
    let envelope = &tail[..end];
    assert!(!envelope.contains("/MCID"), "artifact descendant leaked MCID");
    assert!(!envelope.contains("/Formula"), "artifact descendant leaked Formula tag");
    assert!(
        data.contains("/Formula") && data.contains("/S /Formula"),
        "sibling Formula control lost its marked content or StructElem"
    );
}
