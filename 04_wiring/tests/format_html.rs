//! @prompt 00_nucleo/prompts/wiring/tests/format_html.md
//! @prompt-hash 625c8461

#[path = "../../03_infra/tests/support/wiring/format_html.rs"]
mod harness;

use harness::{Checks, ProcessResult};

use std::ffi::{OsStr, OsString};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const PROCESS_TIMEOUT: Duration = Duration::from_secs(60);
const HTML_WARNING: &str =
    "warning: html export is under active development and incomplete";
const FEATURE_HINT: &str = "try enabling the `html` feature";
const FEATURE_HELP: &str =
    "see https://typst.app/help/compiler-features for more details";
static FIXTURE_ID: AtomicU64 = AtomicU64::new(0);

const TYPED_TAGS: &[&str] = &[
    "div",
    "span",
    "p",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "strong",
    "em",
    "ul",
    "ol",
    "li",
    "a",
    "br",
    "abbr",
    "address",
    "article",
    "aside",
    "b",
    "bdi",
    "bdo",
    "cite",
    "code",
    "dfn",
    "i",
    "kbd",
    "dd",
    "dl",
    "dt",
    "figcaption",
    "figure",
    "footer",
    "header",
    "hgroup",
    "legend",
    "main",
    "mark",
    "menu",
    "nav",
    "picture",
    "pre",
    "s",
    "samp",
    "search",
    "section",
    "small",
    "sub",
    "sup",
    "u",
    "var",
    "datalist",
    "noscript",
    "summary",
    "ruby",
    "rp",
    "rt",
    "html",
    "head",
    "body",
    "title",
    "button",
    "col",
    "iframe",
    "select",
    "template",
    "video",
    "wbr",
    "area",
    "audio",
    "base",
    "blockquote",
    "canvas",
    "caption",
    "colgroup",
    "data",
    "del",
    "details",
    "dialog",
    "embed",
    "fieldset",
    "form",
    "hr",
    "img",
    "input",
    "ins",
    "label",
    "link",
    "map",
    "meta",
    "meter",
    "object",
    "optgroup",
    "option",
    "output",
    "progress",
    "q",
    "script",
    "slot",
    "source",
    "style",
    "table",
    "tbody",
    "td",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "tr",
    "track",
];

#[derive(Clone, Copy)]
struct Profile {
    name: &'static str,
    features: Option<&'static str>,
    html: bool,
}

const PROFILES: [Profile; 4] = [
    Profile { name: "default", features: None, html: false },
    Profile {
        name: "a11y",
        features: Some("a11y-extras"),
        html: false,
    },
    Profile { name: "html", features: Some("html"), html: true },
    Profile {
        name: "html+a11y",
        features: Some("html,a11y-extras"),
        html: true,
    },
];

fn typst_binary() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_typst"))
}

fn run_typst<I, S>(args: I, input: Option<&str>) -> ProcessResult
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args: Vec<OsString> =
        args.into_iter().map(|arg| arg.as_ref().to_os_string()).collect();
    let mut command = Command::new(typst_binary());
    command
        .args(&args)
        .env("NO_COLOR", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if input.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().unwrap_or_else(|error| {
        panic!("cannot spawn {} {:?}: {error}", typst_binary().display(), args)
    });
    if let Some(input) = input {
        child
            .stdin
            .take()
            .expect("piped stdin")
            .write_all(input.as_bytes())
            .expect("write Typst source");
    }

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
    run_typst(args, None)
}

fn compile_html(source: &str, pretty: Option<bool>) -> ProcessResult {
    let tmp = PathBuf::from(std::env::var_os("TMPDIR").expect("TMPDIR must be explicit"));
    assert!(
        tmp.starts_with("/dev/shm"),
        "P1465 fixtures must stay in /dev/shm, got {}",
        tmp.display()
    );
    let fixture = tmp.join(format!(
        "p1465-{}-{}.typ",
        std::process::id(),
        FIXTURE_ID.fetch_add(1, Ordering::Relaxed)
    ));
    let output = fixture.with_extension("html");
    std::fs::write(&fixture, source).expect("write P1465 fixture in tmpfs");
    let mut args = vec![
        OsString::from("compile"),
        fixture.as_os_str().to_os_string(),
        output.as_os_str().to_os_string(),
        OsString::from("--format"),
        OsString::from("html"),
        OsString::from("--features"),
        OsString::from("html"),
        OsString::from("--html-serialization"),
        OsString::from("vanilla"),
    ];
    if let Some(pretty) = pretty {
        args.push(OsString::from(format!("--pretty={pretty}")));
    }
    let mut observed = run_typst(args, None);
    if observed.success() {
        observed.stdout =
            std::fs::read_to_string(&output).expect("read P1465 HTML output from tmpfs");
    }
    std::fs::remove_file(&fixture).expect("remove P1465 fixture from tmpfs");
    if output.exists() {
        std::fs::remove_file(&output).expect("remove P1465 HTML output from tmpfs");
    }
    observed
}

fn require_html_gate(checks: &mut Checks, label: &str, observed: &ProcessResult) {
    checks.require(
        observed.code == Some(1),
        format!("{label}: expected exit 1, got {observed:?}"),
    );
    checks.require(
        observed.stdout.is_empty(),
        format!("{label}: expected empty stdout, got {:?}", observed.stdout),
    );
    checks.require(
        observed.stderr.starts_with(
            "error: cannot access field `html` because the `html` feature is not enabled\n",
        ) && observed.stderr.contains(FEATURE_HINT)
            && observed.stderr.contains(FEATURE_HELP),
        format!("{label}: wrong html gate: {:?}", observed.stderr),
    );
}

fn equality_expression(target: &str) -> String {
    let mut members = vec!["elem", "frame"];
    members.extend_from_slice(TYPED_TAGS);
    let comparisons = members
        .iter()
        .map(|name| format!("{target}.{name} == html.{name}"))
        .collect::<Vec<_>>()
        .join(",");
    format!("({comparisons})")
}

fn require_all_true(checks: &mut Checks, label: &str, observed: &ProcessResult) {
    checks.require(
        observed.success() && observed.stderr.is_empty(),
        format!("{label}: expected clean success, got {observed:?}"),
    );
    checks.require(
        observed.stdout.matches("true").count() == 114
            && !observed.stdout.contains("false"),
        format!("{label}: expected 114 true identities, got {:?}", observed.stdout),
    );
}

fn structurally_pretty(html: &str) -> bool {
    html.contains("\n  <head>") && html.ends_with("\n")
}

fn strip_ascii_whitespace(value: &str) -> String {
    value.chars().filter(|ch| !ch.is_ascii_whitespace()).collect()
}

fn inline_svg_is_pretty(html: &str) -> Option<bool> {
    let start = html.find("<svg")?;
    let end = start + html[start..].find("</svg>")? + "</svg>".len();
    Some(html[start..end].contains('\n'))
}

#[test]
fn p1465_gate_surface_and_manual_construction() {
    let mut checks = Checks::default();
    for profile in PROFILES {
        if profile.html {
            for (label, expression) in [
                (
                    "direct",
                    "(repr(type(format.html)),repr(format.html),repr(format.html.pretty),format.html == std.format.html)",
                ),
                (
                    "std",
                    "(repr(type(std.format.html)),repr(std.format.html),repr(std.format.html.pretty),std.format.html == format.html)",
                ),
                (
                    "alias",
                    "{ let alias = format.html; (repr(type(alias)),repr(alias),repr(alias.pretty),alias == format.html) }",
                ),
            ] {
                let observed = eval(expression, profile.features);
                checks.expect_json(
                    &format!("surface/{}/{label}", profile.name),
                    &observed,
                    r#"["function","html","false",true]"#,
                );
            }
            let constructed = eval("format.html()", profile.features);
            checks.require(
                constructed.code == Some(1)
                    && constructed.stdout.is_empty()
                    && constructed
                        .stderr
                        .starts_with("error: cannot be constructed manually\n"),
                format!("manual/{}: wrong result {constructed:?}", profile.name),
            );
            let missing = eval("format.html.nope", profile.features);
            checks.require(
                missing.code == Some(1)
                    && missing.stdout.is_empty()
                    && missing.stderr.starts_with(
                        "error: function `html` does not contain field `nope`\n",
                    ),
                format!("missing/{}: wrong result {missing:?}", profile.name),
            );
        } else {
            for (label, expression) in [
                ("direct", "format.html"),
                ("std", "std.format.html"),
                ("alias-format", "{ let root = format; root.html }"),
                (
                    "set-before-panic",
                    "{ set format.html(nope: panic(\"must-not-run\")); none }",
                ),
                (
                    "set-pos-before-panic",
                    "{ set format.html(panic(\"must-not-run\")); none }",
                ),
            ] {
                let observed = eval(expression, profile.features);
                require_html_gate(
                    &mut checks,
                    &format!("gate/{}/{label}", profile.name),
                    &observed,
                );
                checks.require(
                    !observed.stderr.starts_with("error: panicked with: must-not-run\n"),
                    format!("gate/{}/{label}: argument ran before gate", profile.name),
                );
            }
        }
    }
    checks.finish();
}

#[test]
fn p1465_all_114_members_reuse_global_html_and_survive_with() {
    assert_eq!(TYPED_TAGS.len(), 112, "typed HTML public catalog changed");
    let mut checks = Checks::default();
    for profile in PROFILES.into_iter().filter(|profile| profile.html) {
        let direct = eval(&equality_expression("format.html"), profile.features);
        require_all_true(
            &mut checks,
            &format!("members/{}/direct", profile.name),
            &direct,
        );

        let with = eval(
            &equality_expression("format.html.with(pretty: true)"),
            profile.features,
        );
        require_all_true(&mut checks, &format!("members/{}/with", profile.name), &with);

        let sentinels = eval(
            "(repr(format.html.elem),repr(format.html.frame),repr(format.html.div),repr(format.html.track),repr(type(format.html.elem)),repr(type(format.html.frame)),repr(type(format.html.div)),repr(type(format.html.track)))",
            profile.features,
        );
        checks.expect_json(
            &format!("members/{}/sentinels", profile.name),
            &sentinels,
            r#"["elem","frame","div","track","function","function","function","function"]"#,
        );
    }
    checks.finish();
}

#[test]
fn p1465_set_rule_validation_order_and_container_boundary() {
    let mut checks = Checks::default();
    for (label, expression, expected) in [
        ("unknown", "{ set format.html(nope: true); none }", "unexpected argument: nope"),
        (
            "type",
            "{ set format.html(pretty: 1); none }",
            "expected boolean, found integer",
        ),
        ("positional", "{ set format.html(true); none }", "unexpected argument"),
        (
            "duplicate",
            "{ set format.html(pretty: true, pretty: false); none }",
            "duplicate argument: pretty",
        ),
        (
            "argument-before-schema",
            "{ set format.html(nope: panic(\"argument-first\")); none }",
            "panicked with: argument-first",
        ),
    ] {
        let observed = eval(expression, Some("html"));
        checks.require(
            observed.code == Some(1)
                && observed.stdout.is_empty()
                && observed.stderr.starts_with(&format!("error: {expected}\n")),
            format!("set/{label}: expected {expected:?}, got {observed:?}"),
        );
    }

    for (label, source) in [
        ("block", "#block[\n#set format.html(pretty: true)\nNested\n]"),
        ("box", "#box[\n#set format.html(pretty: true)\nNested\n]"),
    ] {
        let observed = compile_html(source, None);
        checks.require(
            observed.code == Some(1)
                && observed.stdout.is_empty()
                && observed
                    .stderr
                    .contains("format set rules are not allowed inside of containers"),
            format!("nested/{label}: wrong result {observed:?}"),
        );
    }
    checks.finish();
}

#[test]
fn p1465_document_and_cli_precedence_preserve_dom() {
    let cases = [
        ("default", "#html.div[Hello]", None, false),
        ("document-true", "#set format.html(pretty: true)\n#html.div[Hello]", None, true),
        (
            "empty-preserves-true",
            "#set format.html(pretty: true)\n#set format.html()\n#html.div[Hello]",
            None,
            true,
        ),
        (
            "true-to-false",
            "#set format.html(pretty: true)\n#set format.html(pretty: false)\n#html.div[Hello]",
            None,
            false,
        ),
        (
            "false-to-true",
            "#set format.html(pretty: false)\n#set format.html(pretty: true)\n#html.div[Hello]",
            None,
            true,
        ),
        (
            "cli-false-wins",
            "#set format.html(pretty: true)\n#html.div[Hello]",
            Some(false),
            false,
        ),
        (
            "cli-true-wins",
            "#set format.html(pretty: false)\n#html.div[Hello]",
            Some(true),
            true,
        ),
    ];
    let mut checks = Checks::default();
    for (label, source, override_, expected_pretty) in cases {
        let observed = compile_html(source, override_);
        checks.require(
            observed.success() && observed.stderr.contains(HTML_WARNING),
            format!("precedence/{label}: expected HTML success, got {observed:?}"),
        );
        checks.require(
            structurally_pretty(&observed.stdout) == expected_pretty,
            format!(
                "precedence/{label}: expected pretty={expected_pretty}, got {:?}",
                observed.stdout
            ),
        );
        let counterpart = compile_html(source, Some(!expected_pretty));
        checks.require(
            counterpart.success() && counterpart.stderr.contains(HTML_WARNING),
            format!("precedence/{label}/counterpart: expected HTML success, got {counterpart:?}"),
        );
        checks.require(
            structurally_pretty(&counterpart.stdout) != expected_pretty,
            format!(
                "precedence/{label}/counterpart: expected pretty={}, got {:?}",
                !expected_pretty, counterpart.stdout
            ),
        );
        let normalized = strip_ascii_whitespace(&observed.stdout);
        let counterpart_normalized = strip_ascii_whitespace(&counterpart.stdout);
        checks.require(
            normalized == counterpart_normalized,
            format!(
                "precedence/{label}: pretty changed DOM/escaping: compact/pretty pair {normalized:?} vs {counterpart_normalized:?}"
            ),
        );
    }

    for (label, source, override_, expected_pretty) in [
        (
            "frame-document-true-cli-false",
            "#set format.html(pretty: true)\n#html.frame[Frame]",
            Some(false),
            false,
        ),
        (
            "frame-document-false-cli-true",
            "#set format.html(pretty: false)\n#html.frame[Frame]",
            Some(true),
            true,
        ),
    ] {
        let observed = compile_html(source, override_);
        checks.require(
            observed.success() && observed.stderr.contains(HTML_WARNING),
            format!("precedence/{label}: expected HTML success, got {observed:?}"),
        );
        checks.require(
            structurally_pretty(&observed.stdout) == expected_pretty,
            format!(
                "precedence/{label}: outer HTML did not receive pretty={expected_pretty}"
            ),
        );
        checks.require(
            inline_svg_is_pretty(&observed.stdout) == Some(expected_pretty),
            format!(
                "precedence/{label}: inline SVG did not receive pretty={expected_pretty}"
            ),
        );
    }
    checks.finish();
}
