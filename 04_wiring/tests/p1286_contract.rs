//! Testes black-box independentes do Passo 1286.
//!
//! Este ficheiro e deliberadamente separado da implementacao candidata. A
//! fonte de verdade observacional e o baseline vanilla congelado pelo autor
//! de oraculos; os casos suplementares cobrem witnesses do plano adversarial
//! que nao cabem na matriz principal.

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_typst");
const RUNNER_SHA256: &str =
    "cc0d7986804a7abf006d03e44ffb6eb12a736b794db557b9d60d52e5cc02bf61";
const BASELINE_SHA256: &str =
    "7452eea8ac3bc055a6f67586820a0754925ca3a1a80f1244e693cae2989e19fb";

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

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("04_wiring has repository parent")
        .to_path_buf()
}

fn sha256(path: &Path) -> String {
    let output = Command::new("sha256sum").arg(path).output().expect("run sha256sum");
    assert!(output.status.success(), "sha256sum failed: {output:?}");
    String::from_utf8(output.stdout)
        .expect("sha256sum UTF-8")
        .split_whitespace()
        .next()
        .expect("sha256sum digest")
        .to_owned()
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
fn p1286_candidate_matches_frozen_vanilla_oracle() {
    let root = repository_root();
    let runner = root.join("lab/surface-inventory/run_p1286_oracles.py");
    let baseline = root.join("lab/surface-inventory/p1286-oracle-baseline.json");
    assert_eq!(sha256(&runner), RUNNER_SHA256, "oracle runner drifted");
    assert_eq!(sha256(&baseline), BASELINE_SHA256, "oracle baseline drifted");

    let output = Command::new("python3")
        .arg(&runner)
        .args(["--binary", BIN, "--baseline"])
        .arg(&baseline)
        .current_dir(&root)
        .output()
        .expect("run frozen P1286 oracle");
    assert!(
        output.status.success(),
        "candidate diverges from the frozen P1286 oracle\n{}",
        diagnostic(&output)
    );
    let report = String::from_utf8_lossy(&output.stdout);
    assert!(report.contains("\"verdict\": \"Preserved\""), "{report}");
    assert!(report.contains("\"Unknown\": 0"), "Unknown is never success: {report}");
    assert!(report.contains("\"Violated\": 0"), "{report}");
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

#[test]
fn p1286_mutation_manifest_has_25_distinct_kill_witnesses() {
    // Cada ID aponta para um caso congelado ou para um teste suplementar
    // acima. A campanha injeta um ID por vez; Unknown nao recebe credito.
    let witnesses = [
        ("SQ-PRECEDENCE-01", "smartquote_explicit_beats_alternative"),
        ("SQ-AUTO-02", "p1286_smartquote_explicit_auto_erases_inherited_quotes"),
        ("SQ-GRAPHEME-03", "p1286_smartquote_counts_unicode_graphemes"),
        ("SQ-LANG-04", "smartquote_default_de"),
        ("SQ-ALTERNATIVE-05", "smartquote_alternative_de"),
        ("LN-MAX-01", "line_positive_origin"),
        ("LN-ABS-02", "line_inverted_vector"),
        ("LN-NORMALIZE-03", "line_negative_origin"),
        ("LN-START-04", "line_positive_origin"),
        ("LN-END-PRECEDENCE-05", "line_end_precedence"),
        ("CM-SEQUENTIAL-01", "color_float_weights"),
        ("CM-RENORM-02", "color_ratio_weights"),
        ("CM-NEGATIVE-03", "color_negative_positive_sum"),
        ("CM-HUE-NGT2-04", "color_error_hsl_n3+color_error_hsv_n3+color_error_oklch_n3"),
        ("PA-DROP-01", "attach_path"),
        ("PA-BYTES-02", "attach_bytes_metadata"),
        ("PA-PATH-03", "p1286_attachment_preserves_virtual_path_and_metadata"),
        ("PA-METADATA-04", "p1286_attachment_preserves_virtual_path_and_metadata"),
        ("PA-DUPLICATE-05", "attach_duplicate"),
        ("AR-PASSTHROUGH-01", "artifact_default_other"),
        ("AR-FORMULA-02", "artifact_header"),
        ("AR-MCID-03", "artifact_default_other"),
        (
            "AR-DESCENDANT-MCID-04",
            "p1286_artifact_suppresses_descendant_formula_structure_only_locally",
        ),
        ("AR-TAGS-OFF-VISUAL-05", "artifact_tags_disabled"),
        ("AR-TAGS-OFF-MARK-06", "artifact_tags_disabled"),
    ];
    let ids = witnesses.iter().map(|(id, _)| *id).collect::<BTreeSet<_>>();
    assert_eq!(witnesses.len(), 25);
    assert_eq!(ids.len(), 25, "mutation IDs must be unique");

    let baseline = fs::read_to_string(
        repository_root().join("lab/surface-inventory/p1286-oracle-baseline.json"),
    )
    .expect("read frozen oracle baseline");
    for (_, witness) in witnesses {
        if witness.starts_with("p1286_") {
            continue;
        }
        for case in witness.split('+') {
            assert!(
                baseline.contains(&format!("\"{case}\"")),
                "frozen witness {case} is missing"
            );
        }
    }
}
