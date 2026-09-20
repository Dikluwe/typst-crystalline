//! @prompt 00_nucleo/prompts/wiring/tests/p1462-format-svg.md
//! @prompt-hash d0ca19bd

use std::collections::BTreeMap;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const PROCESS_TIMEOUT: Duration = Duration::from_secs(90);
const OBSERVER_TIMEOUT: Duration = Duration::from_secs(30);
const PAGE: &str = "#set page(width: 20pt, height: 20pt, margin: 0pt)\n";
const CONTAINER_ERROR: &str = "format set rules are not allowed inside of containers";

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(label: &str) -> Self {
        loop {
            let suffix = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("typst-p1462-{label}-{}-{suffix}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Self { path },
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    continue
                }
                Err(error) => panic!("cannot create {}: {error}", path.display()),
            }
        }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[derive(Debug)]
struct ProcessResult {
    success: bool,
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

struct Compilation {
    _temp: TempDir,
    output_path: PathBuf,
    process: ProcessResult,
    artifact: Option<Vec<u8>>,
}

#[derive(Default)]
struct Checks {
    failures: Vec<String>,
}

impl Checks {
    fn require(&mut self, condition: bool, message: impl Into<String>) {
        if !condition {
            self.failures.push(message.into());
        }
    }

    fn finish(self) {
        assert!(
            self.failures.is_empty(),
            "P1462 contract failures:\n- {}",
            self.failures.join("\n- ")
        );
    }
}

#[derive(Debug)]
struct XmlObservation {
    canonical: String,
    pretty: bool,
}

fn typst_binary() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_typst"))
}

fn run_command<I, S>(program: &Path, args: I, timeout: Duration) -> ProcessResult
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args: Vec<OsString> =
        args.into_iter().map(|arg| arg.as_ref().to_os_string()).collect();
    let mut child = Command::new(program)
        .args(&args)
        .env("NO_COLOR", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| {
            panic!("cannot spawn {} {:?}: {error}", program.display(), args)
        });
    let began = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                let output = child.wait_with_output().unwrap_or_else(|error| {
                    panic!("cannot collect {} output: {error}", program.display())
                });
                return ProcessResult {
                    success: output.status.success(),
                    code: output.status.code(),
                    stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                    stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
                };
            }
            Ok(None) if began.elapsed() < timeout => {
                thread::sleep(Duration::from_millis(20));
            }
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                panic!(
                    "Unknown: timeout after {:?}: {} {:?}",
                    timeout,
                    program.display(),
                    args
                );
            }
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("Unknown: cannot observe {}: {error}", program.display());
            }
        }
    }
}

fn run_typst<I, S>(args: I) -> ProcessResult
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_command(typst_binary(), args, PROCESS_TIMEOUT)
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
    pretty: Option<bool>,
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
    if let Some(pretty) = pretty {
        args.push(OsString::from(format!("--pretty={pretty}")));
    }
    if let Some(features) = features {
        args.push(OsString::from("--features"));
        args.push(OsString::from(features));
    }

    let process = run_typst(args);
    let artifact = fs::read(&output_path).ok();
    Compilation { _temp: temp, output_path, process, artifact }
}

fn normalize_space_outside_quotes(input: &str) -> Result<String, String> {
    let mut output = String::new();
    let mut quote = None;
    let mut pending_space = false;
    for ch in input.chars() {
        if let Some(active) = quote {
            output.push(ch);
            if ch == active {
                quote = None;
            }
        } else if ch == '\'' || ch == '"' {
            if pending_space && !output.ends_with(['<', '/', ' ']) {
                output.push(' ');
            }
            pending_space = false;
            quote = Some(ch);
            output.push(ch);
        } else if ch.is_whitespace() {
            pending_space = true;
        } else {
            if pending_space
                && !output.ends_with(['<', '/', ' '])
                && ch != '>'
                && ch != '/'
            {
                output.push(' ');
            }
            pending_space = false;
            output.push(ch);
        }
    }
    if quote.is_some() {
        return Err("unterminated quoted attribute".into());
    }
    Ok(output)
}

fn tag_end(bytes: &[u8], start: usize) -> Result<usize, String> {
    let mut quote = None;
    let mut cursor = start;
    while cursor < bytes.len() {
        let byte = bytes[cursor];
        if let Some(active) = quote {
            if byte == active {
                quote = None;
            }
        } else if byte == b'\'' || byte == b'"' {
            quote = Some(byte);
        } else if byte == b'>' {
            return Ok(cursor);
        }
        cursor += 1;
    }
    Err("unterminated tag".into())
}

fn tag_name(tag: &str) -> Result<&str, String> {
    let body = tag.trim_start_matches('<').trim_start_matches('/').trim_start();
    let end = body
        .find(|ch: char| ch.is_whitespace() || ch == '/' || ch == '>')
        .unwrap_or(body.len());
    if end == 0 {
        Err(format!("tag without name: {tag}"))
    } else {
        Ok(&body[..end])
    }
}

fn normalize_xml(bytes: &[u8]) -> Result<XmlObservation, String> {
    let source = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let raw = source.as_bytes();
    let mut cursor = 0;
    let mut stack: Vec<String> = Vec::new();
    let mut root = None;
    let mut root_count = 0;
    let mut canonical = String::new();

    while cursor < raw.len() {
        if raw[cursor] != b'<' {
            let next = raw[cursor..]
                .iter()
                .position(|byte| *byte == b'<')
                .map(|offset| cursor + offset)
                .unwrap_or(raw.len());
            let text = &source[cursor..next];
            if !text.trim().is_empty() {
                canonical.push_str("T{");
                canonical.push_str(text);
                canonical.push('}');
            }
            cursor = next;
            continue;
        }

        if raw[cursor..].starts_with(b"<!--") {
            let end = source[cursor + 4..]
                .find("-->")
                .ok_or_else(|| "unterminated XML comment".to_string())?;
            cursor += 4 + end + 3;
            continue;
        }
        if raw[cursor..].starts_with(b"<![CDATA[") {
            let end = source[cursor + 9..]
                .find("]]>")
                .ok_or_else(|| "unterminated CDATA".to_string())?;
            canonical.push_str("C{");
            canonical.push_str(&source[cursor + 9..cursor + 9 + end]);
            canonical.push('}');
            cursor += 9 + end + 3;
            continue;
        }
        if raw[cursor..].starts_with(b"<?") {
            let end = source[cursor + 2..]
                .find("?>")
                .ok_or_else(|| "unterminated processing instruction".to_string())?;
            cursor += 2 + end + 2;
            continue;
        }
        if raw[cursor..].starts_with(b"<!") {
            let end = tag_end(raw, cursor + 2)?;
            cursor = end + 1;
            continue;
        }

        let end = tag_end(raw, cursor + 1)?;
        let tag = &source[cursor..=end];
        let normalized = normalize_space_outside_quotes(tag)?;
        let name = tag_name(tag)?.to_string();
        let closing = tag.starts_with("</");
        let self_closing = tag[..tag.len() - 1].trim_end().ends_with('/');

        if closing {
            let open = stack
                .pop()
                .ok_or_else(|| format!("closing tag without opener: {name}"))?;
            if open != name {
                return Err(format!("mismatched tags: opened {open}, closed {name}"));
            }
            canonical.push_str(&normalized);
        } else {
            if stack.is_empty() {
                root_count += 1;
                root.get_or_insert_with(|| name.clone());
            }
            canonical.push_str(&normalized);
            if !self_closing {
                stack.push(name);
            }
        }
        cursor = end + 1;
    }

    if !stack.is_empty() {
        return Err(format!("unclosed tags: {stack:?}"));
    }
    if root_count != 1 {
        return Err(format!("expected one XML root, found {root_count}"));
    }
    let root = root.ok_or_else(|| "missing XML root".to_string())?;
    if root != "svg" && !root.ends_with(":svg") {
        return Err(format!("expected SVG root, found {root}"));
    }

    let pretty = source.lines().any(|line| line.starts_with("  <"));
    Ok(XmlObservation { canonical, pretty })
}

fn normalize_html(bytes: &[u8]) -> Result<String, String> {
    let source = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let raw = source.as_bytes();
    let mut cursor = 0;
    let mut stack: Vec<String> = Vec::new();
    let mut saw_html = false;
    let mut canonical = String::new();
    let void = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
        "param", "source", "track", "wbr",
    ];

    while cursor < raw.len() {
        if raw[cursor] != b'<' {
            let next = raw[cursor..]
                .iter()
                .position(|byte| *byte == b'<')
                .map(|offset| cursor + offset)
                .unwrap_or(raw.len());
            let text =
                source[cursor..next].split_whitespace().collect::<Vec<_>>().join(" ");
            if !text.is_empty() {
                canonical.push_str("T{");
                canonical.push_str(&text);
                canonical.push('}');
            }
            cursor = next;
            continue;
        }
        if raw[cursor..].starts_with(b"<!--") {
            let end = source[cursor + 4..]
                .find("-->")
                .ok_or_else(|| "unterminated HTML comment".to_string())?;
            cursor += 4 + end + 3;
            continue;
        }
        if raw[cursor..].starts_with(b"<!") {
            let end = tag_end(raw, cursor + 2)?;
            cursor = end + 1;
            continue;
        }

        let end = tag_end(raw, cursor + 1)?;
        let tag = &source[cursor..=end];
        let mut name = tag_name(tag)?.to_ascii_lowercase();
        if let Some((_, local)) = name.rsplit_once(':') {
            name = local.to_string();
        }
        let closing = tag.starts_with("</");
        let self_closing = tag[..tag.len() - 1].trim_end().ends_with('/');
        let normalized = normalize_space_outside_quotes(tag)?;
        saw_html |= name == "html";

        if closing {
            let open = stack
                .pop()
                .ok_or_else(|| format!("closing HTML tag without opener: {name}"))?;
            if open != name {
                return Err(format!(
                    "mismatched HTML tags: opened {open}, closed {name}"
                ));
            }
        } else if !self_closing && !void.contains(&name.as_str()) {
            stack.push(name);
        }
        canonical.push_str(&normalized);
        cursor = end + 1;
    }

    if !stack.is_empty() {
        return Err(format!("unclosed HTML tags: {stack:?}"));
    }
    if !saw_html {
        return Err("missing html root".into());
    }
    Ok(canonical)
}

fn inline_svg(html: &[u8]) -> Result<XmlObservation, String> {
    let source = std::str::from_utf8(html).map_err(|error| error.to_string())?;
    let start = source
        .find("<svg")
        .ok_or_else(|| "html.frame output has no inline SVG".to_string())?;
    let relative_end = source[start..]
        .find("</svg>")
        .ok_or_else(|| "inline SVG has no closing tag".to_string())?;
    let end = start + relative_end + "</svg>".len();
    normalize_xml(source[start..end].as_bytes())
}

fn pdf_summary(path: &Path) -> Result<(String, BTreeMap<String, String>), String> {
    let text = run_command(
        Path::new("pdftotext"),
        [path.as_os_str(), OsStr::new("-")],
        OBSERVER_TIMEOUT,
    );
    if !text.success {
        return Err(format!("pdftotext failed ({:?}): {}", text.code, text.stderr));
    }
    let info = run_command(Path::new("pdfinfo"), [path.as_os_str()], OBSERVER_TIMEOUT);
    if !info.success {
        return Err(format!("pdfinfo failed ({:?}): {}", info.code, info.stderr));
    }
    let selected =
        ["Pages", "Page size", "Page rot", "Encrypted", "Tagged", "PDF version"];
    let mut metadata = BTreeMap::new();
    for line in info.stdout.lines() {
        if let Some((key, value)) = line.split_once(':') {
            if selected.contains(&key) {
                metadata.insert(key.to_string(), value.trim().to_string());
            }
        }
    }
    if metadata.len() != selected.len() {
        return Err(format!("pdfinfo omitted selected metadata: {metadata:?}"));
    }
    Ok((text.stdout, metadata))
}

fn png_signature(path: &Path) -> Result<String, String> {
    let observed = run_command(
        Path::new("identify"),
        [OsStr::new("-format"), OsStr::new("%wx%h|%#"), path.as_os_str()],
        OBSERVER_TIMEOUT,
    );
    if observed.success {
        Ok(observed.stdout)
    } else {
        Err(format!("identify failed ({:?}): {}", observed.code, observed.stderr))
    }
}

fn require_success(checks: &mut Checks, label: &str, compiled: &Compilation) {
    checks.require(
        compiled.process.success && compiled.artifact.is_some(),
        format!(
            "{label}: expected success+artifact, code={:?}, stderr={}",
            compiled.process.code, compiled.process.stderr
        ),
    );
}

fn observe_svg(
    checks: &mut Checks,
    label: &str,
    compiled: &Compilation,
) -> Option<XmlObservation> {
    require_success(checks, label, compiled);
    let Some(bytes) = compiled.artifact.as_deref() else {
        return None;
    };
    match normalize_xml(bytes) {
        Ok(observed) => Some(observed),
        Err(error) => {
            checks.require(false, format!("{label}: invalid XML: {error}"));
            None
        }
    }
}

#[test]
fn public_identity_default_and_manual_construction() {
    let profiles = [
        ("default", None),
        ("a11y", Some("a11y-extras")),
        ("html", Some("html")),
        ("html-a11y", Some("html,a11y-extras")),
    ];
    let expressions = [
        ("direct", "repr((type(format.svg), format.svg, format.svg.pretty))"),
        ("std", "repr((type(std.format.svg), std.format.svg, std.format.svg.pretty))"),
        ("alias", "{ let alias = format.svg; repr((type(alias), alias, alias.pretty)) }"),
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
                observed.stdout.trim() == "\"(function, svg, false)\"",
                format!(
                    "identity/{profile}/{access}: expected exact repr/default, got {:?}",
                    observed.stdout
                ),
            );
        }
    }

    let manual = eval("format.svg()", None);
    checks.require(
        !manual.success && manual.stderr.contains("cannot be constructed manually"),
        format!(
            "manual construction: expected frozen rejection, code={:?}, stderr={}",
            manual.code, manual.stderr
        ),
    );
    checks.finish();
}

#[test]
fn set_validation_uses_normal_errors_and_no_ignored_target_warning() {
    let fixtures = [
        (
            "unknown-named",
            "#set format.svg(other: true)\n",
            Some("unexpected argument: other"),
        ),
        (
            "integer",
            "#set format.svg(pretty: 1)\n",
            Some("expected boolean, found integer"),
        ),
        ("positional", "#set format.svg(true)\n", None),
        ("duplicate", "#set format.svg(pretty: true, pretty: false)\n", None),
    ];
    let mut checks = Checks::default();
    for (label, rule, expected_message) in fixtures {
        let compiled = compile(label, &format!("{PAGE}{rule}"), "svg", None, None);
        checks.require(
            !compiled.process.success && compiled.artifact.is_none(),
            format!(
                "{label}: invalid set rule must fail without artifact, code={:?}, stderr={}",
                compiled.process.code, compiled.process.stderr
            ),
        );
        checks.require(
            !compiled.process.stderr.contains("warning: set: target"),
            format!("{label}: fell through to ignored-target warning"),
        );
        if let Some(expected_message) = expected_message {
            checks.require(
                compiled.process.stderr.contains(expected_message),
                format!("{label}: missing diagnostic {expected_message:?}"),
            );
        }
    }
    checks.finish();
}

#[test]
fn root_is_accepted_and_two_distinct_containers_are_rejected() {
    let root = compile(
        "container-root-control",
        &format!("{PAGE}#set format.svg(pretty: true)\n"),
        "svg",
        None,
        None,
    );
    let mut checks = Checks::default();
    let root_xml = observe_svg(&mut checks, "container/root", &root);
    if let Some(xml) = root_xml {
        checks.require(xml.pretty, "container/root: explicit true must be pretty");
    }

    let containers = [
        ("block", format!("{PAGE}#block[#set format.svg(pretty: true)\nx]\n")),
        (
            "box",
            format!(
                "{PAGE}#box[#set text(fill: red)\n#set format.svg(pretty: true)\nx]\n"
            ),
        ),
    ];
    for (label, source) in containers {
        let compiled = compile(&format!("container-{label}"), &source, "svg", None, None);
        checks.require(
            !compiled.process.success && compiled.artifact.is_none(),
            format!(
                "container/{label}: expected failure without artifact, code={:?}",
                compiled.process.code
            ),
        );
        checks.require(
            compiled.process.stderr.contains(CONTAINER_ERROR),
            format!(
                "container/{label}: missing exact public diagnostic; stderr={}",
                compiled.process.stderr
            ),
        );
    }
    checks.finish();
}

#[test]
fn document_cascade_and_caller_override_select_svg_serialization() {
    let rows = [
        ("none-none", "", None, false),
        ("empty-none", "#set format.svg()\n", None, false),
        ("document-true", "#set format.svg(pretty: true)\n", None, true),
        ("document-false", "#set format.svg(pretty: false)\n", None, false),
        (
            "caller-false-document-true",
            "#set format.svg(pretty: true)\n",
            Some(false),
            false,
        ),
        (
            "caller-true-document-false",
            "#set format.svg(pretty: false)\n",
            Some(true),
            true,
        ),
        ("caller-false-none", "", Some(false), false),
        ("caller-true-none", "", Some(true), true),
        (
            "order-true-false",
            "#set format.svg(pretty: true)\n#set format.svg(pretty: false)\n",
            None,
            false,
        ),
        (
            "order-false-true",
            "#set format.svg(pretty: false)\n#set format.svg(pretty: true)\n",
            None,
            true,
        ),
        (
            "order-true-empty",
            "#set format.svg(pretty: true)\n#set format.svg()\n",
            None,
            true,
        ),
        (
            "order-false-empty",
            "#set format.svg(pretty: false)\n#set format.svg()\n",
            None,
            false,
        ),
    ];
    let mut checks = Checks::default();
    for (label, rules, caller, expected_pretty) in rows {
        let compiled = compile(label, &format!("{PAGE}{rules}"), "svg", caller, None);
        if let Some(xml) = observe_svg(&mut checks, label, &compiled) {
            checks.require(
                xml.pretty == expected_pretty,
                format!(
                    "{label}: expected {} serialization",
                    if expected_pretty { "pretty" } else { "compact" }
                ),
            );
        }
    }
    checks.finish();
}

#[test]
fn pretty_and_compact_are_valid_xml_with_same_source_morphology() {
    let source = format!(
        "{PAGE}#set text(fill: red)\n#set format.svg(pretty: true)\n[Styled root]\n"
    );
    let document_pretty = compile("same-source-pretty", &source, "svg", None, None);
    let caller_compact =
        compile("same-source-compact", &source, "svg", Some(false), None);
    let mut checks = Checks::default();
    let pretty = observe_svg(&mut checks, "same-source/pretty", &document_pretty);
    let compact = observe_svg(&mut checks, "same-source/compact", &caller_compact);
    if let (Some(pretty), Some(compact)) = (pretty, compact) {
        checks.require(pretty.pretty, "same-source/pretty: indentation missing");
        checks.require(!compact.pretty, "same-source/compact: indentation leaked");
        checks.require(
            pretty.canonical == compact.canonical,
            "same-source: compact/pretty changed normalized XML morphology",
        );
    }
    checks.finish();
}

#[test]
fn svg_document_option_is_neutral_to_pdf_png_html_and_html_frame() {
    let false_page = format!(
        "{PAGE}#set format.svg(pretty: false)\n#rect(width: 5pt, height: 5pt, fill: red)\n[Neutral]\n"
    );
    let true_page = false_page.replace("pretty: false", "pretty: true");
    let mut checks = Checks::default();

    let pdf_false = compile("neutral-pdf-false", &false_page, "pdf", None, None);
    let pdf_true = compile("neutral-pdf-true", &true_page, "pdf", None, None);
    require_success(&mut checks, "neutral/PDF/false", &pdf_false);
    require_success(&mut checks, "neutral/PDF/true", &pdf_true);
    if pdf_false.artifact.is_some() && pdf_true.artifact.is_some() {
        match (pdf_summary(&pdf_false.output_path), pdf_summary(&pdf_true.output_path)) {
            (Ok(left), Ok(right)) => checks.require(
                left == right,
                format!(
                    "neutral/PDF: public text/metadata differ: {left:?} != {right:?}"
                ),
            ),
            (left, right) => checks.require(
                false,
                format!("neutral/PDF: observer failure: {left:?}; {right:?}"),
            ),
        }
    }

    let png_false = compile("neutral-png-false", &false_page, "png", None, None);
    let png_true = compile("neutral-png-true", &true_page, "png", None, None);
    require_success(&mut checks, "neutral/PNG/false", &png_false);
    require_success(&mut checks, "neutral/PNG/true", &png_true);
    if png_false.artifact.is_some() && png_true.artifact.is_some() {
        match (
            png_signature(&png_false.output_path),
            png_signature(&png_true.output_path),
        ) {
            (Ok(left), Ok(right)) => checks.require(
                left == right,
                format!("neutral/PNG: dimensions/pixels differ: {left:?} != {right:?}"),
            ),
            (left, right) => checks.require(
                false,
                format!("neutral/PNG: observer failure: {left:?}; {right:?}"),
            ),
        }
    }

    let html_false_source = "#set format.svg(pretty: false)\n[Neutral HTML]\n";
    let html_true_source = html_false_source.replace("pretty: false", "pretty: true");
    let html_false =
        compile("neutral-html-false", html_false_source, "html", None, Some("html"));
    let html_true =
        compile("neutral-html-true", &html_true_source, "html", None, Some("html"));
    require_success(&mut checks, "neutral/HTML/false", &html_false);
    require_success(&mut checks, "neutral/HTML/true", &html_true);
    if let (Some(left), Some(right)) =
        (html_false.artifact.as_deref(), html_true.artifact.as_deref())
    {
        match (normalize_html(left), normalize_html(right)) {
            (Ok(left), Ok(right)) => {
                checks.require(left == right, "neutral/HTML: normalized DOM changed")
            }
            (left, right) => checks.require(
                false,
                format!("neutral/HTML: DOM observer failure: {left:?}; {right:?}"),
            ),
        }
    }

    let frame_false_source = "#set format.svg(pretty: false)\n#html.frame[Frame]\n";
    let frame_true_source = frame_false_source.replace("pretty: false", "pretty: true");
    let frame_false =
        compile("neutral-frame-false", frame_false_source, "html", None, Some("html"));
    let frame_true =
        compile("neutral-frame-true", &frame_true_source, "html", None, Some("html"));
    require_success(&mut checks, "neutral/HTML-FRAME/false", &frame_false);
    require_success(&mut checks, "neutral/HTML-FRAME/true", &frame_true);
    if let (Some(left), Some(right)) =
        (frame_false.artifact.as_deref(), frame_true.artifact.as_deref())
    {
        match (normalize_html(left), normalize_html(right)) {
            (Ok(left_dom), Ok(right_dom)) => checks.require(
                left_dom == right_dom,
                "neutral/HTML-FRAME: normalized DOM changed",
            ),
            (left, right) => checks.require(
                false,
                format!("neutral/HTML-FRAME: DOM observer failure: {left:?}; {right:?}"),
            ),
        }
        match (inline_svg(left), inline_svg(right)) {
            (Ok(left_svg), Ok(right_svg)) => checks.require(
                left_svg.canonical == right_svg.canonical,
                "neutral/HTML-FRAME: normalized inline SVG changed",
            ),
            (left, right) => checks.require(
                false,
                format!("neutral/HTML-FRAME: SVG observer failure: {left:?}; {right:?}"),
            ),
        }
    }

    checks.finish();
}

#[test]
fn html_remains_absent_from_format_namespace() {
    let mut checks = Checks::default();
    let observed = eval("repr(format.html)", None);
    checks.require(
        !observed.success
            && observed.stderr.contains("module `format` does not contain `html`"),
        format!(
            "sibling/html: expected strict absence, code={:?}, stderr={}",
            observed.code, observed.stderr
        ),
    );

    checks.finish();
}
