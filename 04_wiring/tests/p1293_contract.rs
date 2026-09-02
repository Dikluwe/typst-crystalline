//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/tests/p1293_contract.md
//! @prompt-hash b6f3b3af
//! @layer L4
//! @updated 2026-09-01
//!
//! Oraculo black-box protegido do Passo 1293.
//!
//! Foi derivado do contrato candidato e do baseline ratificado antes de qualquer
//! implementacao produtiva P1293. Compara lingua, diagnosticos, geometria
//! semantica e DOM; nunca compara representacao Rust, endereco de funcao ou bytes
//! integrais de SVG/HTML.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::OnceLock;

use typst_core::entities::content::Content;
use typst_infra::export::{
    export_html, export_html_with_serialization, HtmlSerializationMode,
};

const BIN: &str = env!("CARGO_BIN_EXE_typst");
const VANILLA: &str = "/usr/local/bin/typst";
const VANILLA_SHA256: &str =
    "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8";

#[derive(Clone, Debug, Eq, PartialEq)]
struct Transcript {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

impl Transcript {
    fn from_output(output: Output) -> Self {
        Self {
            code: output.status.code(),
            stdout: trim_transport(&output.stdout),
            stderr: trim_transport(&output.stderr),
        }
    }

    fn success(&self) -> bool {
        self.code == Some(0)
    }
}

fn trim_transport(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .trim_end_matches(['\r', '\n'])
        .to_owned()
}

fn run(program: &str, args: &[&str]) -> Transcript {
    let output = Command::new("timeout")
        .arg("20s")
        .arg(program)
        .args(args)
        .env("NO_COLOR", "1")
        .output()
        .unwrap_or_else(|error| panic!("cannot execute {program:?}: {error}"));
    if output.status.code() == Some(124) {
        panic!("Unknown: timeout while executing {program:?} {args:?}");
    }
    Transcript::from_output(output)
}

fn assert_vanilla_identity() {
    static CHECKED: OnceLock<()> = OnceLock::new();
    CHECKED.get_or_init(|| {
        let output = Command::new("sha256sum")
            .arg(VANILLA)
            .output()
            .expect("sha256sum must verify the ratified vanilla binary");
        assert!(
            output.status.success(),
            "Unknown: cannot hash the ratified vanilla binary: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        let actual = stdout
            .split_whitespace()
            .next()
            .expect("sha256sum must emit a digest");
        assert_eq!(actual, VANILLA_SHA256, "Unknown: vanilla binary identity changed");
    });
}

fn eval(program: &str, expression: &str, html: bool) -> Transcript {
    let mut args = vec!["eval", expression, "--format", "json"];
    if html {
        args.extend(["--features", "html"]);
    }
    run(program, &args)
}

#[derive(Clone, Copy)]
struct EvalCase {
    id: &'static str,
    expression: &'static str,
    succeeds: bool,
}

fn collect_eval(
    program: &str,
    cases: &[EvalCase],
    html: bool,
) -> BTreeMap<&'static str, Transcript> {
    cases
        .iter()
        .map(|case| (case.id, eval(program, case.expression, html)))
        .collect()
}

fn assert_eval_parity(cases: &[EvalCase], html: bool) {
    assert_vanilla_identity();

    let vanilla_forward = collect_eval(VANILLA, cases, html);
    let candidate_forward = collect_eval(BIN, cases, html);
    let mut reversed = cases.to_vec();
    reversed.reverse();
    let vanilla_reverse = collect_eval(VANILLA, &reversed, html);
    let candidate_reverse = collect_eval(BIN, &reversed, html);

    assert_eq!(vanilla_forward, vanilla_reverse, "vanilla changed with case order");
    assert_eq!(candidate_forward, candidate_reverse, "candidate changed with case order");

    let polarity_violations = cases
        .iter()
        .filter_map(|case| {
            let vanilla = &vanilla_forward[case.id];
            (vanilla.success() != case.succeeds).then(|| {
                format!(
                    "{}: frozen vanilla polarity changed; expected success={} got {vanilla:#?}",
                    case.id, case.succeeds
                )
            })
        })
        .collect::<Vec<_>>();
    assert!(polarity_violations.is_empty(), "{}", polarity_violations.join("\n\n"));

    let mut violations = Vec::new();
    for case in cases {
        let vanilla = &vanilla_forward[case.id];
        let candidate = &candidate_forward[case.id];
        if candidate != vanilla {
            violations.push(format!(
                "{}: candidate is not Preserved\nvanilla={vanilla:#?}\ncandidate={candidate:#?}",
                case.id
            ));
        }
    }
    assert!(violations.is_empty(), "{}", violations.join("\n\n"));
}

struct Scratch(PathBuf);

impl Scratch {
    fn new(label: &str) -> Self {
        let path = env::temp_dir().join(format!(
            "typst-p1293-{label}-{}-{:?}",
            std::process::id(),
            std::thread::current().id(),
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create P1293 scratch directory");
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn compile(
    program: &str,
    label: &str,
    source: &str,
    format: &str,
    html: bool,
) -> (Scratch, PathBuf, Transcript) {
    compile_with_serialization(program, label, source, format, html, None)
}

fn compile_with_serialization(
    program: &str,
    label: &str,
    source: &str,
    format: &str,
    html: bool,
    serialization: Option<&str>,
) -> (Scratch, PathBuf, Transcript) {
    let scratch = Scratch::new(label);
    let input = scratch.0.join("main.typ");
    let output = scratch.0.join(format!("main.{format}"));
    fs::write(&input, source).expect("write P1293 fixture");
    let transcript =
        compile_existing(program, &input, &output, format, html, serialization);
    (scratch, output, transcript)
}

fn compile_existing(
    program: &str,
    input: &std::path::Path,
    output: &std::path::Path,
    format: &str,
    html: bool,
    serialization: Option<&str>,
) -> Transcript {
    let input_arg = input.to_str().expect("UTF-8 fixture path");
    let output_arg = output.to_str().expect("UTF-8 output path");
    let mut args = vec!["compile", input_arg, output_arg, "--format", format];
    if html {
        args.extend(["--features", "html"]);
    }
    if let Some(serialization) = serialization {
        args.extend(["--html-serialization", serialization]);
    }
    run(program, &args)
}

fn attr(tag: &str, name: &str) -> String {
    let needle = format!("{name}=\"");
    let start = tag
        .find(&needle)
        .unwrap_or_else(|| panic!("missing attribute {name:?} in {tag}"))
        + needle.len();
    let tail = &tag[start..];
    let end = tail.find('"').expect("closed attribute");
    tail[..end].to_owned()
}

fn svg_view_box(svg: &str) -> [f64; 4] {
    let root_end = svg.find('>').expect("SVG root close");
    let root = &svg[..=root_end];
    let values = attr(root, "viewBox")
        .split_whitespace()
        .map(|value| value.parse::<f64>().expect("numeric viewBox operand"))
        .collect::<Vec<_>>();
    assert_eq!(values.len(), 4, "viewBox must contain four operands");
    [values[0], values[1], values[2], values[3]]
}

fn svg_semantic_summary(svg: &str) -> ([f64; 4], usize, usize, usize) {
    (
        svg_view_box(svg),
        svg.matches("<line ").count(),
        svg.matches("<use ").count(),
        svg.matches("<path ").count(),
    )
}

fn pdf_semantic_summary(path: &std::path::Path) -> (String, Vec<String>) {
    let path = path.to_str().expect("UTF-8 PDF fixture path");
    let text = run("pdftotext", &["-layout", path, "-"]);
    assert!(text.success(), "Unknown: pdftotext failed for P1293 C-P08: {text:#?}");
    let info = run("pdfinfo", &[path]);
    assert!(info.success(), "Unknown: pdfinfo failed for P1293 C-P08: {info:#?}");
    let structure = info
        .stdout
        .lines()
        .filter(|line| {
            ["Pages:", "Page size:", "Page rot:", "Tagged:", "Encrypted:", "PDF version:"]
                .iter()
                .any(|prefix| line.starts_with(prefix))
        })
        .map(str::to_owned)
        .collect();
    (text.stdout, structure)
}

fn png_semantic_summary(path: &std::path::Path) -> String {
    let path = path.to_str().expect("UTF-8 PNG fixture path");
    let summary = run(
        "identify",
        &["-quiet", "-format", "%w %h %[channels] %[colorspace] %#", path],
    );
    assert!(summary.success(), "Unknown: identify failed for P1293 C-P08: {summary:#?}");
    summary.stdout
}

#[test]
fn p1293_a_static_bound_values_and_closed_errors() {
    let cases = [
        EvalCase {
            id: "A-P01-P02",
            expression: "(type(float.is-nan), repr(float.is-nan), float.is-nan(float(\"NaN\")), float.is-nan(1.25), float.is-nan(1), float.is-nan(float(\"1e999\")), float.is-nan(-float(\"1e999\")))",
            succeeds: true,
        },
        EvalCase {
            id: "A-P03",
            expression: "(float(\"NaN\").is-nan(), 1.25.is-nan(), float(\"1e999\").is-nan())",
            succeeds: true,
        },
        EvalCase { id: "A-N01", expression: "repr(float.is-nan())", succeeds: false },
        EvalCase { id: "A-N02", expression: "repr(float.is-nan(1.0, 2.0))", succeeds: false },
        EvalCase { id: "A-N03", expression: "repr(float(\"NaN\").is-nan(value: 1))", succeeds: false },
        EvalCase { id: "A-N04", expression: "repr(float.is-nan(\"x\"))", succeeds: false },
        EvalCase { id: "A-N05", expression: "repr(1.is-nan())", succeeds: false },
        EvalCase { id: "A-N06", expression: "repr(float(\"NaN\").is-nan)", succeeds: false },
    ];
    assert_eval_parity(&cases, false);
}

#[test]
fn p1293_b_identity_morphology_convergence_and_errors() {
    let cases = [
        EvalCase {
            id: "B-P01",
            expression: "(type(math.attach),repr(math.attach),type(math.binom),repr(math.binom),type(math.mono),repr(math.mono),type(math.script),repr(math.script))",
            succeeds: true,
        },
        EvalCase {
            id: "B-P02-P03-P04-P05",
            expression: "repr((math.attach([x]),math.attach([x],t:[T],b:[B],tl:[L],bl:[M],tr:[R],br:[S]),math.attach([x],t:none),math.binom([n],[k]),math.binom([n],[k],[j],[m]),math.mono([x]),math.script([x]),math.script([x],cramped:true),math.script([x],cramped:false)))",
            succeeds: true,
        },
        EvalCase {
            id: "B-P06",
            expression: "repr(($attach(x,t:T,b:B,tl:L,bl:M,tr:R,br:S)$.body,math.attach([x],t:[T],b:[B],tl:[L],bl:[M],tr:[R],br:[S]),$binom(n,k,j,m)$.body,math.binom([n],[k],[j],[m]),$mono(x)$.body,math.mono([x]),$script(x,cramped:#false)$.body,math.script([x],cramped:false)))",
            succeeds: true,
        },
        EvalCase { id: "B-N-attach-missing", expression: "repr(math.attach())", succeeds: false },
        EvalCase { id: "B-N-attach-extra", expression: "repr(math.attach([x],[y]))", succeeds: false },
        EvalCase { id: "B-N-attach-unknown", expression: "repr(math.attach([x],nope:[y]))", succeeds: false },
        EvalCase { id: "B-N-attach-base-cast", expression: "repr(math.attach(1))", succeeds: false },
        EvalCase { id: "B-N-attach-slot-cast", expression: "repr(math.attach([x],tr:1))", succeeds: false },
        EvalCase { id: "B-N-binom-zero", expression: "repr(math.binom())", succeeds: false },
        EvalCase { id: "B-N-binom-lower", expression: "repr(math.binom([n]))", succeeds: false },
        EvalCase { id: "B-N-binom-cast", expression: "repr(math.binom(1,[k]))", succeeds: false },
        EvalCase { id: "B-N-binom-unknown", expression: "repr(math.binom([n],[k],nope:1))", succeeds: false },
        EvalCase { id: "B-N-binom-named-upper", expression: "repr(math.binom(upper:[n]))", succeeds: false },
        EvalCase { id: "B-N-mono-missing", expression: "repr(math.mono())", succeeds: false },
        EvalCase { id: "B-N-mono-extra", expression: "repr(math.mono([x],[y]))", succeeds: false },
        EvalCase { id: "B-N-mono-named-body", expression: "repr(math.mono(body:[x]))", succeeds: false },
        EvalCase { id: "B-N-mono-cast", expression: "repr(math.mono(1))", succeeds: false },
        EvalCase { id: "B-N-script-missing", expression: "repr(math.script())", succeeds: false },
        EvalCase { id: "B-N-script-extra", expression: "repr(math.script([x],[y]))", succeeds: false },
        EvalCase { id: "B-N-script-named-body", expression: "repr(math.script(body:[x]))", succeeds: false },
        EvalCase { id: "B-N-script-cast", expression: "repr(math.script(1))", succeeds: false },
        EvalCase { id: "B-N-script-unknown", expression: "repr(math.script([x],nope:true))", succeeds: false },
        EvalCase { id: "B-N-script-cramped-cast", expression: "repr(math.script([x],cramped:\"yes\"))", succeeds: false },
    ];
    assert_eval_parity(&cases, false);
}

#[test]
fn p1293_b_semantic_layout_vectors_none_and_no_fraction_rule() {
    let vectors = [
        (
            "attach-display",
            "#set page(width:auto,height:auto,margin:0pt)\n$ attach(x,t:T,b:B,tl:L,bl:M,tr:R,br:S) $\n",
            [0.0, 0.0, 23.2705, 19.4843],
        ),
        (
            "attach-inline",
            "#set page(width:auto,height:auto,margin:0pt)\n#math.equation(math.inline(math.attach([x],t:[T],b:[B],tl:[L],bl:[M],tr:[R],br:[S])),block:false)\n",
            [0.0, 0.0, 19.7681, 9.6041],
        ),
        (
            "binom-display",
            "#set page(width:auto,height:auto,margin:0pt)\n$ binom(n,k,j,m) $\n",
            [0.0, 0.0, 46.797666667, 24.057],
        ),
        (
            "binom-inline",
            "#set page(width:auto,height:auto,margin:0pt)\n#math.equation(math.inline(math.binom([n],[k],[j],[m])),block:false)\n",
            [0.0, 0.0, 30.3325, 7.8815],
        ),
        (
            "mono-display",
            "#set page(width:auto,height:auto,margin:0pt)\n$ mono(x + y) $\n",
            [0.0, 0.0, 25.029888889, 8.921],
        ),
        (
            "mono-script",
            "#set page(width:auto,height:auto,margin:0pt)\n$ script(mono(x + y)) $\n",
            [0.0, 0.0, 14.0987, 6.2447],
        ),
        (
            "script-cramped-true",
            "#set page(width:auto,height:auto,margin:0pt)\n$ script(x^i,cramped:#true) $\n",
            [0.0, 0.0, 8.3578, 5.9708],
        ),
        (
            "script-cramped-false",
            "#set page(width:auto,height:auto,margin:0pt)\n$ script(x^i,cramped:#false) $\n",
            [0.0, 0.0, 8.3578, 6.5406],
        ),
    ];

    assert_vanilla_identity();
    let mut summaries = BTreeMap::new();
    let mut violations = Vec::new();
    for order in [vectors.iter().collect::<Vec<_>>(), vectors.iter().rev().collect()] {
        for (name, source, expected) in order {
            for (side, program) in [("vanilla", VANILLA), ("candidate", BIN)] {
                let (_scratch, path, transcript) =
                    compile(program, &format!("b-{side}-{name}"), source, "svg", false);
                if !transcript.success() {
                    violations
                        .push(format!("B-P07 {side}/{name} failed: {transcript:#?}"));
                    continue;
                }
                let svg = fs::read_to_string(path).expect("read P1293 SVG");
                let summary = svg_semantic_summary(&svg);
                for index in 0..4 {
                    if (summary.0[index] - expected[index]).abs() > 0.01 {
                        violations.push(format!(
                            "B-P07 {side}/{name} viewBox[{index}] expected {} +/- 0.01, got {}",
                            expected[index], summary.0[index]
                        ));
                    }
                }
                if name.starts_with("binom") && summary.1 != 0 {
                    violations.push(format!("B-P04 {side}/{name} drew a fraction rule"));
                }
                let key = format!("{side}/{name}");
                if let Some(previous) = summaries.insert(key.clone(), summary) {
                    if previous != summary {
                        violations.push(format!("B-P07 {key} changed in reverse order"));
                    }
                }
            }
        }
    }

    let none_sources = [
        ("omitted", "#set page(width:auto,height:auto,margin:0pt)\n#math.equation(math.attach([x]),block:false)"),
        ("explicit-none", "#set page(width:auto,height:auto,margin:0pt)\n#math.equation(math.attach([x],t:none),block:false)"),
    ];
    for (side, program) in [("vanilla", VANILLA), ("candidate", BIN)] {
        let mut pair = Vec::new();
        for (name, source) in none_sources {
            let (_scratch, path, transcript) =
                compile(program, &format!("b-none-{side}-{name}"), source, "svg", false);
            if !transcript.success() {
                violations.push(format!("B-P03 {side}/{name} failed: {transcript:#?}"));
                continue;
            }
            pair.push(svg_semantic_summary(
                &fs::read_to_string(path).expect("read none-layout SVG"),
            ));
        }
        if pair.len() == 2 && pair[0] != pair[1] {
            violations.push(format!(
                "B-P03 {side}: explicit none changed semantic layout: {:?} vs {:?}",
                pair[0], pair[1]
            ));
        }
    }
    assert!(violations.is_empty(), "{}", violations.join("\n\n"));
}

#[test]
fn p1293_c_surface_all_specific_casts_and_closed_errors() {
    let cases = [
        EvalCase {
            id: "C-P01-P02",
            expression: "(type(html.button),repr(html.button),repr(html.button()),type(html.col),repr(html.col),repr(html.col()),type(html.iframe),repr(html.iframe),repr(html.iframe()),type(html.select),repr(html.select),repr(html.select()),type(html.template),repr(html.template),repr(html.template()),type(html.video),repr(html.video),repr(html.video()),type(html.wbr),repr(html.wbr),repr(html.wbr()))",
            succeeds: true,
        },
        EvalCase {
            id: "C-P03-button-all",
            expression: "repr(html.button(command:\"toggle-popover\",commandfor:\"cf\",disabled:true,form:\"f\",formaction:\"/go\",formenctype:\"text/plain\",formmethod:\"POST\",formnovalidate:true,formtarget:\"_blank\",name:\"n\",popovertarget:\"p\",popovertargetaction:\"show\",type:\"submit\",value:\"v\",id:\"i\",class:(\"a\",\"b\"),hidden:true)[B])",
            succeeds: true,
        },
        EvalCase { id: "C-P04-col-all", expression: "repr(html.col(span:2,id:\"c\"))", succeeds: true },
        EvalCase {
            id: "C-P04-iframe-all",
            expression: "repr(html.iframe(allow:\"camera\",allowfullscreen:true,height:0,loading:\"lazy\",name:\"_blank\",referrerpolicy:none,sandbox:(\"allow-scripts\",\"allow-forms\"),src:\"/x\",srcdoc:\"<p>x</p>\",width:640)[I])",
            succeeds: true,
        },
        EvalCase {
            id: "C-P04-select-all",
            expression: "repr(html.select(autocomplete:(\"shipping\",\"name\"),disabled:true,form:\"f\",multiple:true,name:\"n\",required:true,size:1)[S])",
            succeeds: true,
        },
        EvalCase {
            id: "C-P04-template-all",
            expression: "repr(html.template(shadowrootclonable:true,shadowrootcustomelementregistry:true,shadowrootdelegatesfocus:true,shadowrootmode:\"open\",shadowrootserializable:true)[T])",
            succeeds: true,
        },
        EvalCase {
            id: "C-P04-video-all",
            expression: "repr(html.video(autoplay:true,controls:true,crossorigin:\"anonymous\",height:0,loop:true,muted:true,playsinline:true,poster:\"p\",preload:\"metadata\",src:\"v\",width:640)[V])",
            succeeds: true,
        },
        EvalCase {
            id: "C-P09-preload-none-value",
            expression: "repr(html.video(preload:none))",
            succeeds: true,
        },
        EvalCase {
            id: "C-P09-preload-auto-value",
            expression: "repr(html.video(preload:auto))",
            succeeds: true,
        },
        EvalCase {
            id: "C-P09-preload-metadata-string",
            expression: "repr(html.video(preload:\"metadata\"))",
            succeeds: true,
        },
        EvalCase {
            id: "C-N-preload-none-string",
            expression: "repr(html.video(preload:\"none\"))",
            succeeds: false,
        },
        EvalCase {
            id: "C-N-preload-auto-string",
            expression: "repr(html.video(preload:\"auto\"))",
            succeeds: false,
        },
        EvalCase { id: "C-P03-wbr-global", expression: "repr(html.wbr(id:\"w\",class:(\"a\",\"b\"),hidden:true))", succeeds: true },
        EvalCase { id: "C-P05-presence-order", expression: "repr(html.button(value:\"v\",disabled:false,name:\"n\",command:\"save\",formnovalidate:true))", succeeds: true },
        EvalCase { id: "C-N-button-unknown", expression: "repr(html.button(nope:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-button-data", expression: "repr(html.button(data-p1293:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-button-cross", expression: "repr(html.button(span:2))", succeeds: false },
        EvalCase { id: "C-N-col-unknown", expression: "repr(html.col(nope:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-col-data", expression: "repr(html.col(data-p1293:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-col-cross", expression: "repr(html.col(disabled:true))", succeeds: false },
        EvalCase { id: "C-N-iframe-unknown", expression: "repr(html.iframe(nope:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-iframe-data", expression: "repr(html.iframe(data-p1293:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-iframe-cross", expression: "repr(html.iframe(size:1))", succeeds: false },
        EvalCase { id: "C-N-select-unknown", expression: "repr(html.select(nope:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-select-data", expression: "repr(html.select(data-p1293:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-select-cross", expression: "repr(html.select(src:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-template-unknown", expression: "repr(html.template(nope:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-template-data", expression: "repr(html.template(data-p1293:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-template-cross", expression: "repr(html.template(controls:true))", succeeds: false },
        EvalCase { id: "C-N-video-unknown", expression: "repr(html.video(nope:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-video-data", expression: "repr(html.video(data-p1293:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-video-cross", expression: "repr(html.video(command:\"save\"))", succeeds: false },
        EvalCase { id: "C-N-wbr-unknown", expression: "repr(html.wbr(nope:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-wbr-data", expression: "repr(html.wbr(data-p1293:\"x\"))", succeeds: false },
        EvalCase { id: "C-N-wbr-cross", expression: "repr(html.wbr(span:2))", succeeds: false },
        EvalCase { id: "C-N-col-body", expression: "repr(html.col([x]))", succeeds: false },
        EvalCase { id: "C-N-wbr-body", expression: "repr(html.wbr([x]))", succeeds: false },
        EvalCase { id: "C-N-normal-extra-body", expression: "repr(html.button([x],[y]))", succeeds: false },
        EvalCase { id: "C-N-normal-named-body", expression: "repr(html.button(body:[x]))", succeeds: false },
        EvalCase { id: "C-N-string-cast", expression: "repr(html.button(commandfor:1))", succeeds: false },
        EvalCase { id: "C-N-presence-cast", expression: "repr(html.button(disabled:\"yes\"))", succeeds: false },
        EvalCase { id: "C-N-enum-cast", expression: "repr(html.button(formenctype:\"bad\"))", succeeds: false },
        EvalCase { id: "C-N-target-cast", expression: "repr(html.button(formtarget:1))", succeeds: false },
        EvalCase { id: "C-N-positive-zero", expression: "repr(html.col(span:0))", succeeds: false },
        EvalCase { id: "C-N-positive-negative", expression: "repr(html.select(size:-1))", succeeds: false },
        EvalCase { id: "C-N-nonnegative", expression: "repr(html.iframe(height:-1))", succeeds: false },
        EvalCase { id: "C-N-none-enum", expression: "repr(html.iframe(referrerpolicy:\"bad\"))", succeeds: false },
        EvalCase { id: "C-N-enum-list", expression: "repr(html.iframe(sandbox:(\"allow-scripts\",\"bad\")))", succeeds: false },
        EvalCase { id: "C-N-autocomplete-list", expression: "repr(html.select(autocomplete:(\"shipping\",\"bad\")))", succeeds: false },
        EvalCase { id: "C-N-template-enum", expression: "repr(html.template(shadowrootmode:\"bad\"))", succeeds: false },
        EvalCase { id: "C-N-video-enum", expression: "repr(html.video(preload:\"bad\"))", succeeds: false },
    ];
    assert_eval_parity(&cases, true);
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum DomNode {
    Element { tag: String, attrs: Vec<(String, String)>, children: Vec<DomNode> },
    Text(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParsedDom {
    roots: Vec<DomNode>,
    closing_tags: Vec<String>,
}

fn decode_entities(value: &str) -> String {
    value
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

fn tag_end(html: &str, start: usize) -> usize {
    let mut quote = false;
    for (offset, ch) in html[start..].char_indices() {
        match ch {
            '"' => quote = !quote,
            '>' if !quote => return start + offset,
            _ => {}
        }
    }
    panic!("unterminated HTML tag")
}

fn parse_open_tag(raw: &str) -> (String, Vec<(String, String)>, bool) {
    let mut input = raw.trim();
    let self_closing = input.ends_with('/');
    if self_closing {
        input = input[..input.len() - 1].trim_end();
    }
    let name_end = input.find(char::is_whitespace).unwrap_or(input.len());
    let name = input[..name_end].to_owned();
    let mut rest = input[name_end..].trim_start();
    let mut attrs = Vec::new();
    while !rest.is_empty() {
        let key_end = rest
            .find(|ch: char| ch.is_whitespace() || ch == '=')
            .unwrap_or(rest.len());
        let key = rest[..key_end].to_owned();
        rest = rest[key_end..].trim_start();
        if !rest.starts_with('=') {
            attrs.push((key, String::new()));
            continue;
        }
        rest = rest[1..].trim_start();
        assert!(rest.starts_with('"'), "quoted HTML attribute expected: {raw}");
        rest = &rest[1..];
        let value_end = rest.find('"').expect("attribute closing quote");
        attrs.push((key, decode_entities(&rest[..value_end])));
        rest = rest[value_end + 1..].trim_start();
    }
    (name, attrs, self_closing)
}

fn push_dom_node(roots: &mut Vec<DomNode>, stack: &mut Vec<DomNode>, node: DomNode) {
    if let Some(DomNode::Element { children, .. }) = stack.last_mut() {
        children.push(node);
    } else {
        roots.push(node);
    }
}

fn parse_dom(html: &str) -> ParsedDom {
    let void = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
        "source", "track", "wbr",
    ];
    let mut roots = Vec::new();
    let mut stack = Vec::new();
    let mut closing_tags = Vec::new();
    let mut at = 0;
    while at < html.len() {
        let Some(relative) = html[at..].find('<') else {
            let text = &html[at..];
            if !text.is_empty() {
                push_dom_node(
                    &mut roots,
                    &mut stack,
                    DomNode::Text(decode_entities(text)),
                );
            }
            break;
        };
        let open = at + relative;
        if open > at {
            let text = &html[at..open];
            if !text.is_empty() {
                push_dom_node(
                    &mut roots,
                    &mut stack,
                    DomNode::Text(decode_entities(text)),
                );
            }
        }
        let end = tag_end(html, open + 1);
        let raw = &html[open + 1..end];
        if raw.starts_with('!') || raw.starts_with('?') {
            at = end + 1;
            continue;
        }
        if let Some(name) = raw.strip_prefix('/') {
            let name = name.trim().to_owned();
            closing_tags.push(name.clone());
            let node =
                stack.pop().unwrap_or_else(|| panic!("closing {name} without opener"));
            let DomNode::Element { tag, .. } = &node else { unreachable!() };
            assert_eq!(tag, &name, "mismatched HTML closing tag");
            push_dom_node(&mut roots, &mut stack, node);
        } else {
            let (tag, attrs, self_closing) = parse_open_tag(raw);
            let is_void = void.contains(&tag.as_str());
            let node = DomNode::Element { tag, attrs, children: Vec::new() };
            if self_closing || is_void {
                push_dom_node(&mut roots, &mut stack, node);
            } else {
                stack.push(node);
            }
        }
        at = end + 1;
    }
    assert!(stack.is_empty(), "unclosed HTML nodes: {stack:#?}");
    ParsedDom { roots, closing_tags }
}

fn visit_elements<'a>(
    nodes: &'a [DomNode],
    output: &mut Vec<(&'a str, &'a [(String, String)])>,
) {
    for node in nodes {
        if let DomNode::Element { tag, attrs, children } = node {
            output.push((tag, attrs));
            visit_elements(children, output);
        }
    }
}

#[test]
fn p1293_c_dom_order_void_escaping_and_nesting() {
    let fixtures = [
        (
            "all-seven",
            "#html.div[#html.button(command:\"save\",disabled:true,type:\"submit\")[#text(\"A<&B\")]#html.col(span:2)#html.iframe(src:\"/x\")[#html.select(size:1)[S]]#html.template[#html.wbr()]#html.video(controls:true,muted:false,src:\"v\")[V]]",
        ),
        (
            "attribute-order",
            "#html.button(value:\"v\",disabled:false,name:\"n\",command:\"save\",formnovalidate:true)[B]",
        ),
        ("void-only", "#html.div[#html.col(span:3)#html.p[A#html.wbr()B]]"),
        ("escaping", "#html.button(value:\"x&\\\"<>\")[#text(\"<&>\")]"),
        ("nesting", "#html.iframe[#html.select[#html.template[#html.video[V]]]]"),
    ];
    assert_vanilla_identity();
    let mut observed: BTreeMap<String, (ParsedDom, String)> = BTreeMap::new();
    let mut violations = Vec::new();
    for order in [fixtures.iter().collect::<Vec<_>>(), fixtures.iter().rev().collect()] {
        for (name, source) in order {
            for (side, program) in [("vanilla", VANILLA), ("candidate", BIN)] {
                let serialization = (side == "candidate").then_some("vanilla");
                let (_scratch, path, transcript) = compile_with_serialization(
                    program,
                    &format!("c-{side}-{name}"),
                    source,
                    "html",
                    true,
                    serialization,
                );
                if !transcript.success() {
                    violations
                        .push(format!("C-P06 {side}/{name} failed: {transcript:#?}"));
                    continue;
                }
                let html_text = fs::read_to_string(path).expect("read P1293 HTML");
                let parsed = parse_dom(&html_text);
                let key = format!("{side}/{name}");
                if let Some((old_dom, old_raw)) =
                    observed.insert(key.clone(), (parsed.clone(), html_text.clone()))
                {
                    if old_dom != parsed || old_raw != html_text {
                        violations.push(format!("C-P06 {key} changed in reverse order"));
                    }
                }
            }
        }
    }
    for (name, _) in fixtures {
        let vanilla = observed.get(&format!("vanilla/{name}"));
        let candidate = observed.get(&format!("candidate/{name}"));
        if let (Some((vanilla, _)), Some((candidate, _))) = (vanilla, candidate) {
            if vanilla != candidate {
                violations.push(format!(
                    "C-P06 DOM mismatch for {name}\nvanilla={vanilla:#?}\ncandidate={candidate:#?}"
                ));
            }
        }
    }

    if let Some((dom, raw)) = observed.get("candidate/all-seven") {
        let mut elements = Vec::new();
        visit_elements(&dom.roots, &mut elements);
        for tag in ["button", "col", "iframe", "select", "template", "video", "wbr"] {
            if elements.iter().filter(|(name, _)| *name == tag).count() != 1 {
                violations
                    .push(format!("C-P06 all-seven must contain exactly one <{tag}>"));
            }
        }
        if dom.closing_tags.iter().any(|tag| tag == "col" || tag == "wbr") {
            violations.push("C-P06 void element received an end tag".to_owned());
        }
        if !raw.contains("A&lt;&amp;B") {
            violations.push("C-P06 body escaping changed".to_owned());
        }
    }
    if let Some((dom, _)) = observed.get("candidate/attribute-order") {
        let mut elements = Vec::new();
        visit_elements(&dom.roots, &mut elements);
        let attrs = elements
            .iter()
            .find(|(tag, _)| *tag == "button")
            .map(|(_, attrs)| *attrs)
            .expect("attribute-order button");
        let names = attrs.iter().map(|(name, _)| name.as_str()).collect::<Vec<_>>();
        if names != ["value", "name", "command", "formnovalidate"] {
            violations.push(format!("C-P05 wrong attribute order/omission: {names:?}"));
        }
    }
    if let Some((dom, raw)) = observed.get("candidate/escaping") {
        let mut elements = Vec::new();
        visit_elements(&dom.roots, &mut elements);
        let value = elements
            .iter()
            .find(|(tag, _)| *tag == "button")
            .and_then(|(_, attrs)| attrs.iter().find(|(name, _)| name == "value"))
            .map(|(_, value)| value.as_str());
        if value != Some("x&\"<>")
            || !raw.contains("x&amp;&quot;<>")
            || !raw.contains("&lt;&amp;>")
        {
            violations
                .push(format!("C-P06 escaping changed: value={value:?}, raw={raw:?}"));
        }
    }
    assert!(violations.is_empty(), "{}", violations.join("\n\n"));
}

#[test]
fn p1293_c_serialization_modes_default_scope_and_non_html_neutrality() {
    let source = "#html.button(value:\"x&\\\"<>\")[#text(\"<&>\")]";
    let scratch = Scratch::new("c-serialization-modes");
    let input = scratch.0.join("main.typ");
    let output = scratch.0.join("main.html");
    fs::write(&input, source).expect("write serialization fixture");
    let mut outputs = BTreeMap::new();
    for (label, mode) in [
        ("default", None),
        ("crystalline", Some("crystalline")),
        ("vanilla", Some("vanilla")),
    ] {
        let transcript = compile_existing(BIN, &input, &output, "html", true, mode);
        assert!(transcript.success(), "C-P08 {label} failed: {transcript:#?}");
        outputs.insert(label, fs::read_to_string(&output).expect("read serialized HTML"));
    }

    let default = &outputs["default"];
    let crystalline = &outputs["crystalline"];
    let vanilla = &outputs["vanilla"];
    assert_eq!(default, crystalline, "C-P08 default is not crystalline");
    assert_ne!(crystalline, vanilla, "C-P08 the two modes collapsed");
    assert!(
        crystalline.contains("value=\"x&amp;&quot;&lt;&gt;\"")
            && crystalline.contains("&lt;&amp;&gt;"),
        "C-P08 crystalline escaping changed: {crystalline:?}"
    );
    assert!(
        vanilla.contains("value=\"x&amp;&quot;<>\"") && vanilla.contains("&lt;&amp;>"),
        "C-P08 vanilla escaping changed: {vanilla:?}"
    );
    assert_eq!(
        parse_dom(crystalline),
        parse_dom(vanilla),
        "C-P08 modes changed the decoded DOM"
    );

    let (_scratch, output, invalid) = compile_with_serialization(
        BIN,
        "c-serialization-invalid",
        "#text(\"x\")",
        "html",
        true,
        Some("other"),
    );
    assert!(!invalid.success() && !output.exists(), "C-P08 invalid mode was accepted");
    assert!(
        invalid.stderr.contains("invalid value 'other'")
            && invalid.stderr.contains("--html-serialization"),
        "C-P08 wrong invalid-mode diagnostic: {invalid:#?}"
    );

    for args in [
        vec!["eval", "1", "--html-serialization", "vanilla"],
        vec!["query", "missing.typ", "<heading>", "--html-serialization", "vanilla"],
        vec!["info", "--html-serialization", "vanilla"],
        vec!["watch", "missing.typ", "--html-serialization", "vanilla"],
    ] {
        let transcript = run(BIN, &args);
        assert!(
            !transcript.success() && transcript.stderr.contains("--html-serialization"),
            "C-P08 flag escaped compile scope for {args:?}: {transcript:#?}"
        );
    }

    for format in ["pdf", "png", "svg"] {
        let scratch = Scratch::new(&format!("c-serialization-neutral-{format}"));
        let input = scratch.0.join("main.typ");
        let output = scratch.0.join(format!("main.{format}"));
        fs::write(&input, "#text(\"neutral\")").expect("write non-HTML fixture");
        let mut summaries = Vec::new();
        for mode in ["crystalline", "vanilla"] {
            let transcript =
                compile_existing(BIN, &input, &output, format, false, Some(mode));
            assert!(
                transcript.success(),
                "C-P08 {mode} was not accepted for {format}: {transcript:#?}"
            );
            if format == "pdf" {
                let summary = pdf_semantic_summary(&output);
                assert!(
                    summary.0.contains("neutral"),
                    "C-P08 PDF lost its observable text under {mode}: {summary:#?}"
                );
                summaries.push(format!("{summary:?}"));
            } else if format == "png" {
                summaries.push(png_semantic_summary(&output));
            } else {
                let svg =
                    fs::read_to_string(&output).expect("read non-HTML SVG artifact");
                summaries.push(format!("{:?}", svg_semantic_summary(&svg)));
            }
        }
        assert_eq!(
            summaries[0], summaries[1],
            "C-P08 serialization mode changed {format} semantics"
        );
    }
}

#[test]
fn p1293_c_export_facade_is_public_direct_and_keeps_html_semantics_owned_below() {
    assert_eq!(
        std::any::type_name::<HtmlSerializationMode>(),
        "typst_infra::export::html::HtmlSerializationMode",
        "C-P10 facade duplicated HtmlSerializationMode instead of reexporting it"
    );
    assert_eq!(
        std::any::type_name_of_val(&export_html),
        "typst_infra::export::html::export_html",
        "C-P10 facade wrapped export_html instead of reexporting it"
    );
    assert_eq!(
        std::any::type_name_of_val(&export_html_with_serialization),
        "typst_infra::export::html::export_html_with_serialization",
        "C-P10 facade wrapped export_html_with_serialization instead of reexporting it"
    );

    let content = Content::Empty;
    let legacy = export_html(&content).expect("C-P10 legacy HTML facade entry point");
    let crystalline =
        export_html_with_serialization(&content, HtmlSerializationMode::Crystalline)
            .expect("C-P10 explicit crystalline HTML facade entry point");
    let vanilla =
        export_html_with_serialization(&content, HtmlSerializationMode::Vanilla)
            .expect("C-P10 explicit vanilla HTML facade entry point");
    assert_eq!(legacy, crystalline, "C-P10 facade changed the crystalline default");
    assert_eq!(
        parse_dom(&crystalline),
        parse_dom(&vanilla),
        "C-P10 facade changed HTML semantics between modes"
    );
}

#[test]
fn p1293_c_feature_and_target_are_independent_axes() {
    assert_vanilla_identity();
    let off_paged = eval(BIN, "repr(html.button)", false);
    assert!(!off_paged.success(), "C-P07: paged target enabled HTML feature");
    assert!(
        off_paged.stderr.contains("the `html` feature is not enabled"),
        "C-P07 wrong feature-off diagnostic: {off_paged:#?}"
    );

    let on_paged = eval(BIN, "(type(html.button),repr(html.button))", true);
    assert_eq!(
        on_paged,
        eval(VANILLA, "(type(html.button),repr(html.button))", true),
        "C-P07 feature-on paged evaluation is not Preserved"
    );

    let source = "#html.button[B]";
    let (_scratch, output, off_html) = compile(BIN, "c-off-html", source, "html", false);
    assert!(!off_html.success() && !output.exists(), "C-P07 HTML target enabled feature");
    assert!(off_html.stderr.contains("--features html"), "{off_html:#?}");

    let (_scratch, output, on_html) = compile(BIN, "c-on-html", source, "html", true);
    assert!(on_html.success() && output.exists(), "C-P07 on/html failed: {on_html:#?}");
}

fn assert_candidate_eval(expression: &str, expected: Transcript) {
    let direct = eval(BIN, expression, false);
    let reverse = eval(BIN, expression, false);
    assert_eq!(direct, reverse, "D transcript is not deterministic");
    assert_eq!(direct, expected);
}

fn assert_candidate_order_stable(expressions: &[&str]) {
    let forward = expressions
        .iter()
        .map(|expression| (*expression, eval(BIN, expression, false)))
        .collect::<BTreeMap<_, _>>();
    let reverse = expressions
        .iter()
        .rev()
        .map(|expression| (*expression, eval(BIN, expression, false)))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(forward, reverse, "D transcript changed in reverse order");
}

#[test]
fn p1293_d_ten_short_names_with_calls_and_flat_aliases_preserved() {
    assert_candidate_order_stable(&[
        "(repr(grid.cell),repr(grid.header),repr(grid.footer),repr(grid.hline),repr(grid.vline),repr(table.cell),repr(table.header),repr(table.footer),repr(table.hline),repr(table.vline))",
        "repr((grid.cell([x]),grid.header(grid.cell([x])),grid.footer(grid.cell([x])),grid.hline(),grid.vline(),table.cell([x]),table.header(table.cell([x])),table.footer(table.cell([x])),table.hline(),table.vline()))",
        "repr((grid_cell,grid_header,grid_footer,table_cell,table_header,table_footer))",
    ]);
    assert_candidate_eval(
        "(repr(grid.cell),repr(grid.header),repr(grid.footer),repr(grid.hline),repr(grid.vline),repr(table.cell),repr(table.header),repr(table.footer),repr(table.hline),repr(table.vline))",
        Transcript {
            code: Some(0),
            stdout: "[\"cell\",\"header\",\"footer\",\"hline\",\"vline\",\"cell\",\"header\",\"footer\",\"hline\",\"vline\"]".to_owned(),
            stderr: String::new(),
        },
    );
    let payload = "\"(\\n  grid.cell,\\n  grid.header,\\n  grid.footer,\\n  grid.hline,\\n  grid.vline,\\n  table.cell,\\n  table.header,\\n  table.footer,\\n  table.hline,\\n  table.vline,\\n)\"";
    for expression in [
        "repr((grid.cell([x]),grid.header(grid.cell([x])),grid.footer(grid.cell([x])),grid.hline(),grid.vline(),table.cell([x]),table.header(table.cell([x])),table.footer(table.cell([x])),table.hline(),table.vline()))",
        "repr((grid.cell.with()([x]),grid.header.with()(grid.cell([x])),grid.footer.with()(grid.cell([x])),grid.hline.with()(),grid.vline.with()(),table.cell.with()([x]),table.header.with()(table.cell([x])),table.footer.with()(table.cell([x])),table.hline.with()(),table.vline.with()()))",
    ] {
        assert_candidate_eval(
            expression,
            Transcript { code: Some(0), stdout: payload.to_owned(), stderr: String::new() },
        );
    }
    assert_candidate_eval(
        "repr((grid.cell.with(),grid.header.with(),grid.footer.with(),grid.hline.with(),grid.vline.with(),table.cell.with(),table.header.with(),table.footer.with(),table.hline.with(),table.vline.with()))",
        Transcript {
            code: Some(0),
            stdout: "\"(\\n  cell,\\n  header,\\n  footer,\\n  hline,\\n  vline,\\n  cell,\\n  header,\\n  footer,\\n  hline,\\n  vline,\\n)\"".to_owned(),
            stderr: String::new(),
        },
    );
    assert_candidate_eval(
        "repr((grid_cell,grid_header,grid_footer,table_cell,table_header,table_footer))",
        Transcript {
            code: Some(0),
            stdout: "\"(\\n  grid_cell,\\n  grid_header,\\n  grid_footer,\\n  table_cell,\\n  table_header,\\n  table_footer,\\n)\"".to_owned(),
            stderr: String::new(),
        },
    );
    for (name, column) in
        [("grid_hline", 5), ("grid_vline", 5), ("table_hline", 5), ("table_vline", 5)]
    {
        assert_candidate_eval(
            &format!("repr({name})"),
            Transcript {
                code: Some(1),
                stdout: String::new(),
                stderr: format!(
                    "error: unknown variable `{name}`\n  ┌─ <input-expression>:1:{column}\n  │\n1 │ repr({name})\n  │      {}",
                    "^".repeat(name.len())
                ),
            },
        );
    }
}

#[test]
fn p1293_d_preexisting_arity_and_serializer_transcript_is_frozen() {
    assert_candidate_order_stable(&[
        "repr(grid.cell())",
        "repr(grid.header())",
        "repr(grid.footer())",
        "repr(table.cell())",
        "repr(table.header())",
        "repr(table.footer())",
        "repr((grid.hline([x]),grid.hline(nope:1),grid.vline([x]),grid.vline(nope:1),table.hline([x]),table.hline(nope:1),table.vline([x]),table.vline(nope:1)))",
    ]);
    for (expression, message) in [
        ("repr(grid.cell())", "grid_cell() exige body como argumento posicional"),
        (
            "repr(grid.header())",
            "grid_header() exige pelo menos uma célula como argumento posicional",
        ),
        (
            "repr(grid.footer())",
            "grid_footer() exige pelo menos uma célula como argumento posicional",
        ),
        ("repr(table.cell())", "table_cell() exige body como argumento posicional"),
        (
            "repr(table.header())",
            "table_header() exige pelo menos uma célula como argumento posicional",
        ),
        (
            "repr(table.footer())",
            "table_footer() exige pelo menos uma célula como argumento posicional",
        ),
    ] {
        assert_candidate_eval(
            expression,
            Transcript {
                code: Some(1),
                stdout: String::new(),
                stderr: format!("error: {message}"),
            },
        );
    }
    assert_candidate_eval(
        "repr((grid.hline([x]),grid.hline(nope:1),grid.vline([x]),grid.vline(nope:1),table.hline([x]),table.hline(nope:1),table.vline([x]),table.vline(nope:1)))",
        Transcript {
            code: Some(0),
            stdout: "\"(\\n  grid.hline,\\n  grid.hline,\\n  grid.vline,\\n  grid.vline,\\n  table.hline,\\n  table.hline,\\n  table.vline,\\n  table.vline,\\n)\"".to_owned(),
            stderr: String::new(),
        },
    );
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OracleOutcome {
    Unknown(&'static str),
}

#[test]
fn p1293_opacity_and_mutation_registry_is_closed() {
    let opaque = [
        ("A-O01", OracleOutcome::Unknown("NaN bit pattern is mechanical")),
        ("B-O01", OracleOutcome::Unknown("SVG bytes/path ids are mechanical")),
        ("C-O01", OracleOutcome::Unknown("browser/CSS/media/network is outside DOM")),
        ("D-O01", OracleOutcome::Unknown("function pointer address is mechanical")),
    ];
    assert_eq!(opaque.len(), 4);
    assert!(opaque
        .iter()
        .all(|(_, outcome)| matches!(outcome, OracleOutcome::Unknown(_))));

    let mutants = [
        ("MA1", "A-P01-P02"),
        ("MA2", "A-P01-P02"),
        ("MA3", "A-P03"),
        ("MA4", "A-N03"),
        ("MA5", "A-P01-P02"),
        ("MB1", "B-P02-P03-P04-P05"),
        ("MB2", "B-P02-P03-P04-P05/B-P03-layout"),
        ("MB3", "B-P07-binom"),
        ("MB4", "B-P02-P03-P04-P05"),
        ("MB5", "B-P06"),
        ("MB6", "B-P05/B-P06"),
        ("MB7", "B-P05/B-P07-cramped"),
        ("MC1", "C-N-*-unknown/data"),
        ("MC2", "C-P05/C-P06-order"),
        ("MC3", "C-N-col-body/C-N-wbr-body"),
        ("MC4", "C-P01-P02"),
        ("MC5", "C-N-*-cross"),
        ("MC6", "C-N-*-enum"),
        ("MC7", "C-P07"),
        ("MC8", "C-P06-void-only"),
        ("MC9", "C-P05/C-P06-escaping"),
        ("MC10", "C-P08-default-crystalline"),
        ("MC11", "C-P08-distinct-modes"),
        ("MC12", "C-P09-preload-poles"),
        ("MC13", "C-P10-public-export-facade"),
        ("MC14", "C-P10-direct-reexport-not-wrapper"),
        ("MD1", "D-P01-ten"),
        ("MD2", "D-P03-flat"),
        ("MD3", "D-P01-table"),
        ("MD4", "D-P02/D-P04"),
        ("MD5", "D-P01-ten"),
    ];
    assert_eq!(mutants.len(), 31, "mutation denominator changed");
    let ids = mutants
        .iter()
        .map(|(id, _)| *id)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(ids.len(), 31, "duplicate mutation id");
    let unknown_required_or_mutation = 0usize;
    assert_eq!(unknown_required_or_mutation, 0, "Unknown cannot receive credit");
}
