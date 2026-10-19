//! @prompt 00_nucleo/prompts/wiring/tests/format_png.md
//! @prompt-hash fee5015f

#[path = "../../03_infra/tests/support/wiring/format_png.rs"]
mod harness;

use harness::{Checks, TempDir};

use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const PROCESS_TIMEOUT: Duration = Duration::from_secs(90);
const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
const CONTAINER_ERROR: &str = "format set rules are not allowed inside of containers";
const PAGE_1PT: &str = "#set page(width: 1pt, height: 1pt, margin: 0pt)\n";
const OPAQUE_1PT: &str = "#rect(width: 1pt, height: 1pt, fill: black)\n";

#[derive(Debug)]
struct ProcessResult {
    success: bool,
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

struct Compilation {
    _temp: TempDir,
    process: ProcessResult,
    artifact: Option<Vec<u8>>,
}

fn typst_binary() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_typst"))
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
                    success: output.status.success(),
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
                panic!(
                    "Unknown: timeout after {:?}: {} {:?}",
                    PROCESS_TIMEOUT,
                    typst_binary().display(),
                    args
                );
            }
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("Unknown: cannot observe {}: {error}", typst_binary().display());
            }
        }
    }
}

fn eval(expression: &str, features: Option<&str>) -> ProcessResult {
    let mut args = vec![
        OsString::from("eval"),
        OsString::from(expression),
        OsString::from("--format"),
        OsString::from("json"),
    ];
    if let Some(features) = features {
        args.push(OsString::from("--features"));
        args.push(OsString::from(features));
    }
    run_typst(args)
}

fn compile(
    label: &str,
    source: &str,
    format: &str,
    ppi: Option<&str>,
    features: Option<&str>,
) -> Compilation {
    let temp = TempDir::new(label);
    let input_path = temp.path.join("main.typ");
    let output_path = temp.path.join(format!("main.{format}"));
    fs::write(&input_path, source.as_bytes())
        .unwrap_or_else(|error| panic!("cannot write {}: {error}", input_path.display()));

    let mut args = vec![
        OsString::from("compile"),
        input_path.as_os_str().to_os_string(),
        output_path.as_os_str().to_os_string(),
        OsString::from("--format"),
        OsString::from(format),
    ];
    if let Some(ppi) = ppi {
        args.push(OsString::from(format!("--ppi={ppi}")));
    }
    if let Some(features) = features {
        args.push(OsString::from("--features"));
        args.push(OsString::from(features));
    }

    let process = run_typst(args);
    let artifact = fs::read(&output_path).ok();
    Compilation { _temp: temp, process, artifact }
}

fn one_point_source(rules: &str) -> String {
    format!("{PAGE_1PT}{rules}{OPAQUE_1PT}")
}

fn png_dimensions(bytes: &[u8]) -> Result<(u32, u32), String> {
    if bytes.len() < 24 {
        return Err(format!(
            "PNG shorter than signature and IHDR: {} bytes",
            bytes.len()
        ));
    }
    if &bytes[..8] != PNG_SIGNATURE {
        return Err("invalid PNG signature".into());
    }
    let ihdr_len = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
    if ihdr_len != 13 || &bytes[12..16] != b"IHDR" {
        return Err(format!(
            "first PNG chunk is not a 13-byte IHDR: length={ihdr_len}, kind={:?}",
            &bytes[12..16]
        ));
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
    if width == 0 || height == 0 {
        return Err(format!("IHDR has zero dimension: {width}x{height}"));
    }
    Ok((width, height))
}

fn observe_png(
    checks: &mut Checks,
    label: &str,
    compiled: &Compilation,
) -> Option<(u32, u32)> {
    checks.require(
        compiled.process.success && compiled.artifact.is_some(),
        format!(
            "{label}: expected success+artifact, code={:?}, stdout={:?}, stderr={}",
            compiled.process.code, compiled.process.stdout, compiled.process.stderr
        ),
    );
    let bytes = compiled.artifact.as_deref()?;
    match png_dimensions(bytes) {
        Ok(dimensions) => Some(dimensions),
        Err(error) => {
            checks.require(false, format!("{label}: {error}"));
            None
        }
    }
}

#[test]
fn public_surface_namespace_and_manual_construction() {
    let profiles = [
        ("default", None),
        ("a11y", Some("a11y-extras")),
        ("html", Some("html")),
        ("html-a11y", Some("html,a11y-extras")),
    ];
    let expressions = [
        ("direct", "repr((type(format.png), format.png, format.png.ppi))"),
        ("std", "repr((type(std.format.png), std.format.png, std.format.png.ppi))"),
        ("alias", "{ let alias = format.png; repr((type(alias), alias, alias.ppi)) }"),
    ];
    let mut checks = Checks::default();

    for (profile, features) in profiles {
        for (access, expression) in expressions {
            let observed = eval(expression, features);
            checks.require(
                observed.success,
                format!(
                    "identity/{profile}/{access}: exit {:?}: {}",
                    observed.code, observed.stderr
                ),
            );
            checks.require(
                observed.stdout.trim() == "\"(function, png, 144.0)\"",
                format!(
                    "identity/{profile}/{access}: expected exact type/name/float default, got {:?}",
                    observed.stdout
                ),
            );
        }
    }

    let extra_namespace = eval("repr(format.png.pretty)", None);
    checks.require(
        !extra_namespace.success
            && extra_namespace.stdout.is_empty()
            && extra_namespace.stderr.contains("pretty"),
        format!(
            "namespace: only ppi is public in this step; code={:?}, stderr={}",
            extra_namespace.code, extra_namespace.stderr
        ),
    );

    let manual = eval("format.png()", None);
    checks.require(
        !manual.success
            && manual.stdout.is_empty()
            && manual.stderr.contains("cannot be constructed manually"),
        format!(
            "manual construction: expected frozen rejection, code={:?}, stderr={}",
            manual.code, manual.stderr
        ),
    );
    checks.finish();
}

#[test]
fn set_validation_uses_normal_errors_without_legacy_warning_or_artifact() {
    let fixtures = [
        ("unknown-named", "#set format.png(nope: 72)\n", "unexpected argument: nope"),
        ("string", "#set format.png(ppi: \"72\")\n", "expected float, found string"),
        ("boolean", "#set format.png(ppi: true)\n", "expected float, found boolean"),
        ("positional", "#set format.png(72)\n", "unexpected argument"),
        ("duplicate", "#set format.png(ppi: 72, ppi: 144)\n", "duplicate argument: ppi"),
    ];
    let mut checks = Checks::default();

    for (label, rule, expected_message) in fixtures {
        let compiled = compile(label, &one_point_source(rule), "png", None, None);
        checks.require(
            !compiled.process.success && compiled.artifact.is_none(),
            format!(
                "{label}: invalid set rule must fail without artifact, code={:?}, stderr={}",
                compiled.process.code, compiled.process.stderr
            ),
        );
        checks.require(
            compiled.process.stderr.contains(expected_message),
            format!("{label}: missing diagnostic {expected_message:?}"),
        );
        checks.require(
            !compiled.process.stderr.contains("warning: set: target"),
            format!("{label}: fell through to legacy ignored-target warning"),
        );
    }

    let invalid_cli =
        compile("caller-text", &one_point_source(""), "png", Some("texto"), None);
    checks.require(
        invalid_cli.process.code == Some(2)
            && !invalid_cli.process.success
            && invalid_cli.artifact.is_none(),
        format!(
            "caller-text: expected parser exit 2 without artifact, code={:?}, stderr={}",
            invalid_cli.process.code, invalid_cli.process.stderr
        ),
    );
    checks.require(
        invalid_cli.process.stderr.contains("texto"),
        format!(
            "caller-text: parser diagnostic must identify the invalid value; stderr={}",
            invalid_cli.process.stderr
        ),
    );
    checks.finish();
}

#[test]
fn one_point_ihdr_matrix_proves_defaults_cascade_precedence_and_limits() {
    let rows = [
        ("default", "", None, (2, 2)),
        ("empty", "#set format.png()\n", None, (2, 2)),
        ("document-72", "#set format.png(ppi: 72)\n", None, (1, 1)),
        ("document-2", "#set format.png(ppi: 2)\n", None, (1, 1)),
        ("document-1-5", "#set format.png(ppi: 1.5)\n", None, (1, 1)),
        (
            "document-72-empty",
            "#set format.png(ppi: 72)\n#set format.png()\n",
            None,
            (1, 1),
        ),
        (
            "order-72-288",
            "#set format.png(ppi: 72)\n#set format.png(ppi: 288)\n",
            None,
            (4, 4),
        ),
        (
            "order-288-72",
            "#set format.png(ppi: 288)\n#set format.png(ppi: 72)\n",
            None,
            (1, 1),
        ),
        ("caller-72", "", Some("72"), (1, 1)),
        ("caller-288", "", Some("288"), (4, 4)),
        ("caller-72-document-288", "#set format.png(ppi: 288)\n", Some("72"), (1, 1)),
        ("caller-288-document-72", "#set format.png(ppi: 72)\n", Some("288"), (4, 4)),
        ("document-zero", "#set format.png(ppi: 0)\n", None, (1, 1)),
        ("document-negative", "#set format.png(ppi: -72)\n", None, (1, 1)),
        ("caller-zero", "", Some("0"), (1, 1)),
        ("caller-negative", "", Some("-72"), (1, 1)),
    ];
    let mut checks = Checks::default();

    for (label, rules, ppi, expected) in rows {
        let compiled = compile(label, &one_point_source(rules), "png", ppi, None);
        if let Some(observed) = observe_png(&mut checks, label, &compiled) {
            checks.require(
                observed == expected,
                format!("{label}: expected IHDR {expected:?}, got {observed:?}"),
            );
        }
    }
    checks.finish();
}

#[test]
fn a4_default_usa_144_ppi_sem_alocacao_gigante() {
    let compiled = compile(
        "a4-default",
        "#rect(width: 1pt, height: 1pt, fill: black)\n",
        "png",
        None,
        None,
    );
    let mut checks = Checks::default();
    if let Some(observed) = observe_png(&mut checks, "a4-default", &compiled) {
        checks.require(
            observed == (1191, 1684),
            format!("a4-default: expected IHDR (1191, 1684), got {observed:?}"),
        );
    }
    checks.finish();
}

#[test]
fn nested_rules_fail_with_exact_diagnostic_and_publish_no_artifact() {
    let containers = [
        (
            "block",
            format!(
                "{PAGE_1PT}#block[#set format.png(ppi: 72)\n{OPAQUE_1PT}]\n"
            ),
        ),
        (
            "box",
            format!(
                "{PAGE_1PT}#box[#set text(fill: red)\n#set format.png(ppi: 72)\n{OPAQUE_1PT}]\n"
            ),
        ),
    ];
    let mut checks = Checks::default();

    for (label, source) in containers {
        let compiled = compile(&format!("nested-{label}"), &source, "png", None, None);
        checks.require(
            !compiled.process.success && compiled.artifact.is_none(),
            format!(
                "nested/{label}: expected failure without artifact, code={:?}, stderr={}",
                compiled.process.code, compiled.process.stderr
            ),
        );
        checks.require(
            compiled.process.stderr.contains(CONTAINER_ERROR),
            format!(
                "nested/{label}: missing exact public diagnostic; stderr={}",
                compiled.process.stderr
            ),
        );
    }
    checks.finish();
}

#[test]
fn png_document_and_caller_options_are_byte_neutral_to_svg() {
    let page = "#set page(width: 20pt, height: 20pt, margin: 0pt)\n";
    let svg_rule = "#set format.svg(pretty: true)\n";
    let shape = "#rect(width: 5pt, height: 5pt, fill: red)\n";
    let reference_source = format!("{page}{svg_rule}{shape}");
    let document_source = format!("{page}{svg_rule}#set format.png(ppi: 72)\n{shape}");
    let combined_source = format!("{page}{svg_rule}#set format.png(ppi: 288)\n{shape}");
    let variants = [
        ("caller", reference_source.as_str(), Some("72")),
        ("document", document_source.as_str(), None),
        ("combined", combined_source.as_str(), Some("72")),
    ];
    let reference =
        compile("neutral-svg-reference", &reference_source, "svg", None, None);
    let mut checks = Checks::default();
    checks.require(
        reference.process.success && reference.artifact.is_some(),
        format!(
            "neutral-svg/reference: expected success+artifact, code={:?}, stderr={}",
            reference.process.code, reference.process.stderr
        ),
    );

    for (label, source, ppi) in variants {
        let compiled = compile(&format!("neutral-svg-{label}"), source, "svg", ppi, None);
        checks.require(
            compiled.process.success && compiled.artifact.is_some(),
            format!(
                "neutral-svg/{label}: expected success+artifact, code={:?}, stderr={}",
                compiled.process.code, compiled.process.stderr
            ),
        );
        if let (Some(expected), Some(observed)) =
            (reference.artifact.as_deref(), compiled.artifact.as_deref())
        {
            checks.require(
                observed == expected,
                format!("neutral-svg/{label}: ppi changed public SVG bytes"),
            );
        }
    }
    checks.finish();
}

#[test]
fn p1465_successor_html_is_feature_gated_from_format_namespace() {
    let mut checks = Checks::default();
    let observed = eval("repr(format.html)", None);
    checks.require(
        !observed.success
            && observed.stderr.contains(
                "cannot access field `html` because the `html` feature is not enabled",
            )
            && observed.stderr.contains("try enabling the `html` feature"),
        format!(
            "sibling/html: expected feature gate, code={:?}, stderr={}",
            observed.code, observed.stderr
        ),
    );
    checks.finish();
}
