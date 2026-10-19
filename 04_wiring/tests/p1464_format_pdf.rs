//! @prompt 00_nucleo/prompts/wiring/tests/p1464-format-pdf.md
//! @prompt-hash a8b9c925

#[path = "../../03_infra/tests/support/wiring/p1464_format_pdf.rs"]
mod harness;

use harness::{Checks, ProcessResult};

use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const PROCESS_TIMEOUT: Duration = Duration::from_secs(60);
const DISABLED_HINT: &str = "try enabling the `a11y-extras` feature";
const FEATURE_HELP_HINT: &str =
    "see https://typst.app/help/compiler-features for more details";

#[derive(Clone, Copy)]
struct Profile {
    name: &'static str,
    features: Option<&'static str>,
    a11y: bool,
}

const PROFILES: [Profile; 4] = [
    Profile { name: "default", features: None, a11y: false },
    Profile {
        name: "a11y-extras",
        features: Some("a11y-extras"),
        a11y: true,
    },
    Profile { name: "html", features: Some("html"), a11y: false },
    Profile {
        name: "html+a11y",
        features: Some("html,a11y-extras"),
        a11y: true,
    },
];

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

fn exact_gated_stderr(expression: &str, field: &str) -> String {
    let field_start = expression.rfind(field).unwrap_or_else(|| {
        panic!("field {field:?} absent from expression {expression:?}")
    });
    format!(
        "error: cannot access field `{field}` because the `a11y-extras` feature is not enabled\n\
         \x20 ┌─ <input-expression>:1:{field_start}\n\
         \x20 │\n\
         1 │ {expression}\n\
         \x20 │ {}{}\n\
         \x20 │\n\
         \x20 = hint: {DISABLED_HINT}\n\
         \x20 = hint: {FEATURE_HELP_HINT}\n\n",
        " ".repeat(field_start),
        "^".repeat(field.len()),
    )
}

fn require_field_span(
    checks: &mut Checks,
    label: &str,
    expression: &str,
    field: &str,
    stderr: &str,
) {
    let field_start = expression.rfind(field).unwrap_or_else(|| {
        panic!("field {field:?} absent from expression {expression:?}")
    });
    let location = format!("<input-expression>:1:{field_start}");
    let underline = format!("  │ {}{}", " ".repeat(field_start), "^".repeat(field.len()));
    checks.require(
        stderr.contains(&location),
        format!("{label}: missing exact field location {location:?}; stderr={stderr:?}"),
    );
    checks.require(
        stderr.lines().any(|line| line == underline),
        format!("{label}: missing field-only underline {underline:?}; stderr={stderr:?}"),
    );
}

#[test]
fn p1464_surface_defaults_aliases_and_manual_construction() {
    const EXPECTED: &str =
        r#"["function","pdf","none","(\"1.7\",)","auto","false",true]"#;
    let accesses = [
        (
            "direct",
            "(repr(type(format.pdf)),repr(format.pdf),repr(format.pdf.pages),repr(format.pdf.standard),repr(format.pdf.tagged),repr(format.pdf.pretty),format.pdf == format.pdf)",
        ),
        (
            "std",
            "(repr(type(std.format.pdf)),repr(std.format.pdf),repr(std.format.pdf.pages),repr(std.format.pdf.standard),repr(std.format.pdf.tagged),repr(std.format.pdf.pretty),std.format.pdf == format.pdf)",
        ),
        (
            "alias",
            "{ let alias = format.pdf; (repr(type(alias)),repr(alias),repr(alias.pages),repr(alias.standard),repr(alias.tagged),repr(alias.pretty),alias == format.pdf) }",
        ),
    ];
    let mut checks = Checks::default();

    for profile in PROFILES {
        for (access, expression) in accesses {
            let observed = eval(expression, profile.features);
            checks.expect_json(
                &format!("surface/{}/{access}", profile.name),
                &observed,
                EXPECTED,
            );
        }

        for (access, expression) in
            [("direct", "format.pdf()"), ("alias", "{ let alias = format.pdf; alias() }")]
        {
            let observed = eval(expression, profile.features);
            checks.require(
                observed.code == Some(1)
                    && observed.stdout.is_empty()
                    && observed.stderr.starts_with("error: cannot be constructed manually\n"),
                format!(
                    "manual/{}/{access}: expected exact construction rejection, got {observed:?}",
                    profile.name
                ),
            );
        }
    }

    checks.finish();
}

#[test]
fn p1464_permanent_members_are_reused_and_survive_with() {
    const EXPECTED: &str = r#"["function","attach",true,"function","artifact",true]"#;
    let targets = [
        ("direct", "format.pdf"),
        ("std", "std.format.pdf"),
        ("with", "format.pdf.with(pretty: true)"),
    ];
    let mut checks = Checks::default();

    for profile in PROFILES {
        for (access, target) in targets {
            let expression = format!(
                "(repr(type({target}.attach)),repr({target}.attach),{target}.attach == pdf.attach,repr(type({target}.artifact)),repr({target}.artifact),{target}.artifact == pdf.artifact)"
            );
            let observed = eval(&expression, profile.features);
            checks.expect_json(
                &format!("permanent/{}/{access}", profile.name),
                &observed,
                EXPECTED,
            );
        }

        let alias_expression = "{ let alias = format.pdf; (repr(type(alias.attach)),repr(alias.attach),alias.attach == pdf.attach,repr(type(alias.artifact)),repr(alias.artifact),alias.artifact == pdf.artifact) }";
        let observed = eval(alias_expression, profile.features);
        checks.expect_json(
            &format!("permanent/{}/alias", profile.name),
            &observed,
            EXPECTED,
        );
    }

    checks.finish();
}

#[test]
fn p1464_a11y_trio_is_all_or_nothing_with_exact_diagnostics() {
    let fields = ["table-summary", "header-cell", "data-cell"];
    let mut checks = Checks::default();

    for profile in PROFILES {
        for field in fields {
            if profile.a11y {
                let expected =
                    format!(r#"["function","{field}",true,"function","{field}"]"#);
                for (access, expression) in [
                    (
                        "direct",
                        format!(
                            "(repr(type(format.pdf.{field})),repr(format.pdf.{field}),format.pdf.{field} == pdf.{field},repr(type(pdf.{field})),repr(pdf.{field}))"
                        ),
                    ),
                    (
                        "std",
                        format!(
                            "(repr(type(std.format.pdf.{field})),repr(std.format.pdf.{field}),std.format.pdf.{field} == pdf.{field},repr(type(pdf.{field})),repr(pdf.{field}))"
                        ),
                    ),
                    (
                        "alias",
                        format!(
                            "{{ let alias = format.pdf; (repr(type(alias.{field})),repr(alias.{field}),alias.{field} == pdf.{field},repr(type(pdf.{field})),repr(pdf.{field})) }}"
                        ),
                    ),
                    (
                        "with",
                        format!(
                            "{{ let bound = format.pdf.with(pretty: true); (repr(type(bound.{field})),repr(bound.{field}),bound.{field} == pdf.{field},repr(type(pdf.{field})),repr(pdf.{field})) }}"
                        ),
                    ),
                ] {
                    let observed = eval(&expression, profile.features);
                    checks.expect_json(
                        &format!("a11y-on/{}/{access}/{field}", profile.name),
                        &observed,
                        &expected,
                    );
                }
            } else {
                for (access, expression) in [
                    ("direct", format!("format.pdf.{field}")),
                    ("std", format!("std.format.pdf.{field}")),
                    ("alias", format!("{{ let alias = format.pdf; alias.{field} }}")),
                    ("with", format!("format.pdf.with(pretty: true).{field}")),
                    ("global", format!("pdf.{field}")),
                ] {
                    let observed = eval(&expression, profile.features);
                    let label = format!("a11y-off/{}/{access}/{field}", profile.name);
                    checks.require(
                        observed.code == Some(1),
                        format!("{label}: expected exit 1, got {:?}", observed.code),
                    );
                    checks.require(
                        observed.stdout.is_empty(),
                        format!(
                            "{label}: expected empty stdout, got {:?}",
                            observed.stdout
                        ),
                    );
                    let expected_stderr = exact_gated_stderr(&expression, field);
                    checks.require(
                        observed.stderr == expected_stderr,
                        format!(
                            "{label}: diagnostic differs\nexpected={expected_stderr:?}\nobserved={:?}",
                            observed.stderr
                        ),
                    );
                }
            }
        }
    }

    checks.finish();
}

#[test]
fn p1464_svg_png_global_pdf_html_and_homonymous_closure_controls() {
    const FORMAT_SIBLINGS: &str =
        r#"["function","svg","false","function","png","144.0"]"#;
    const GLOBAL_PDF: &str =
        r#"["function","pdf","function","attach","function","artifact"]"#;
    let mut checks = Checks::default();

    for profile in PROFILES {
        let siblings = eval(
            "(repr(type(format.svg)),repr(format.svg),repr(format.svg.pretty),repr(type(format.png)),repr(format.png),repr(format.png.ppi))",
            profile.features,
        );
        checks.expect_json(
            &format!("siblings/{}", profile.name),
            &siblings,
            FORMAT_SIBLINGS,
        );

        let global_pdf = eval(
            "(repr(type(pdf)),repr(pdf),repr(type(pdf.attach)),repr(pdf.attach),repr(type(pdf.artifact)),repr(pdf.artifact))",
            profile.features,
        );
        checks.expect_json(
            &format!("global-pdf/{}", profile.name),
            &global_pdf,
            GLOBAL_PDF,
        );

        let html_expression = "format.html";
        let observed_html = eval(html_expression, profile.features);
        let html_label = format!("format-html-successor/{}", profile.name);
        if profile.features.is_some_and(|features| features.contains("html")) {
            checks.expect_json(&html_label, &observed_html, "\"html\"");
        } else {
            checks.require(
                observed_html.code == Some(1)
                    && observed_html.stdout.is_empty()
                    && observed_html.stderr.starts_with(
                        "error: cannot access field `html` because the `html` feature is not enabled\n",
                    )
                    && observed_html.stderr.contains("try enabling the `html` feature"),
                format!("{html_label}: expected P1465 feature gate, got {observed_html:?}"),
            );
            require_field_span(
                &mut checks,
                &html_label,
                html_expression,
                "html",
                &observed_html.stderr,
            );
        }
    }

    for (label, expression) in [
        ("bare", "{ let pdf = () => none; pdf.table-summary }"),
        (
            "homonymous-parameters-with",
            "{ let pdf = (pages: none, standard: (\"1.7\",), tagged: auto, pretty: false, attach: none, artifact: none) => none; pdf.with(pretty: true).table-summary }",
        ),
    ] {
        let observed = eval(expression, None);
        checks.require(
            observed.code == Some(1)
                && observed.stdout.is_empty()
                && observed
                    .stderr
                    .starts_with("error: cannot access fields on user-defined functions\n"),
            format!("closure/{label}: expected ordinary closure diagnostic, got {observed:?}"),
        );
        checks.require(
            !observed.stderr.contains("the `a11y-extras` feature is not enabled")
                && !observed.stderr.contains("= hint:"),
            format!("closure/{label}: special feature diagnostic leaked: {:?}", observed.stderr),
        );
        require_field_span(
            &mut checks,
            &format!("closure/{label}"),
            expression,
            "table-summary",
            &observed.stderr,
        );
    }

    checks.finish();
}
