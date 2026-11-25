//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/tests/language_and_export_contract.md
//! @prompt-hash b4a0a3fd
//! @layer L4
//! @updated 2026-09-01
//!
//! Testes black-box de linguagem, diagnósticos, geometria semântica e DOM.

#[path = "../../03_infra/tests/support/wiring/language_and_export_contract.rs"]
mod harness;

use harness::{parse_dom, visit_elements, ParsedDom, Scratch, Transcript};

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use typst_core::entities::content::Content;
use typst_infra::export::{
    export_html, export_html_with_serialization, HtmlSerializationMode,
};

const BIN: &str = env!("CARGO_BIN_EXE_typst");
fn vanilla() -> String {
    env::var("TYPST_VANILLA").expect(
        "set TYPST_VANILLA to the independently verified oracle for pin bd5ebec285505889e8ff1d4f68804ab147a699f3",
    )
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
    let vanilla = collect_eval(&vanilla(), cases, html);
    let candidate = collect_eval(BIN, cases, html);

    let polarity_violations = cases
        .iter()
        .filter_map(|case| {
            let observed = &vanilla[case.id];
            (observed.success() != case.succeeds).then(|| {
                format!(
                    "{}: vanilla polarity differs; expected success={} got {observed:#?}",
                    case.id, case.succeeds
                )
            })
        })
        .collect::<Vec<_>>();
    assert!(polarity_violations.is_empty(), "{}", polarity_violations.join("\n\n"));

    let mut violations = Vec::new();
    for case in cases {
        let expected = &vanilla[case.id];
        let actual = &candidate[case.id];
        if actual != expected {
            violations.push(format!(
                "{}: public behavior differs\nvanilla={expected:#?}\ncandidate={actual:#?}",
                case.id
            ));
        }
    }
    assert!(violations.is_empty(), "{}", violations.join("\n\n"));
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

    let mut violations = Vec::new();
    for (name, source, expected) in &vectors {
        for (side, program) in [("vanilla", vanilla().as_str()), ("candidate", BIN)] {
            let (_scratch, path, transcript) =
                compile(program, &format!("b-{side}-{name}"), source, "svg", false);
            if !transcript.success() {
                violations.push(format!("B-P07 {side}/{name} failed: {transcript:#?}"));
                continue;
            }
            let svg = fs::read_to_string(path).expect("read P1293 SVG");
            let summary = svg_semantic_summary(&svg);
            for (index, expected_value) in expected.iter().enumerate() {
                if (summary.0[index] - *expected_value).abs() > 0.01 {
                    violations.push(format!(
                        "B-P07 {side}/{name} viewBox[{index}] expected {} +/- 0.01, got {}",
                        expected_value, summary.0[index]
                    ));
                }
            }
            if name.starts_with("binom") && summary.1 != 0 {
                violations.push(format!("B-P04 {side}/{name} drew a fraction rule"));
            }
        }
    }

    let none_sources = [
        ("omitted", "#set page(width:auto,height:auto,margin:0pt)\n#math.equation(math.attach([x]),block:false)"),
        ("explicit-none", "#set page(width:auto,height:auto,margin:0pt)\n#math.equation(math.attach([x],t:none),block:false)"),
    ];
    for (side, program) in [("vanilla", vanilla().as_str()), ("candidate", BIN)] {
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
    let mut observed: BTreeMap<String, (ParsedDom, String)> = BTreeMap::new();
    let mut violations = Vec::new();
    for (name, source) in &fixtures {
        for (side, program) in [("vanilla", vanilla().as_str()), ("candidate", BIN)] {
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
                violations.push(format!("C-P06 {side}/{name} failed: {transcript:#?}"));
                continue;
            }
            let html_text = fs::read_to_string(path).expect("read P1293 HTML");
            let parsed = parse_dom(&html_text);
            observed.insert(format!("{side}/{name}"), (parsed, html_text));
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
    let off_paged = eval(BIN, "repr(html.button)", false);
    assert!(!off_paged.success(), "C-P07: paged target enabled HTML feature");
    assert!(
        off_paged.stderr.contains("the `html` feature is not enabled"),
        "C-P07 wrong feature-off diagnostic: {off_paged:#?}"
    );

    let on_paged = eval(BIN, "(type(html.button),repr(html.button))", true);
    assert_eq!(
        on_paged,
        eval(&vanilla(), "(type(html.button),repr(html.button))", true),
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
    assert_eq!(eval(BIN, expression, false), expected);
}

#[test]
fn p1293_d_ten_short_names_with_calls_and_flat_aliases_preserved() {
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
            stdout: "\"(\\n  (..) => ..,\\n  (..) => ..,\\n  (..) => ..,\\n  (..) => ..,\\n  (..) => ..,\\n  (..) => ..,\\n  (..) => ..,\\n  (..) => ..,\\n  (..) => ..,\\n  (..) => ..,\\n)\"".to_owned(),
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
fn p1293_d_preexisting_arity_and_serializer_transcript_is_preserved() {
    for (expression, message) in [
        ("repr(grid.cell())", "grid_cell() exige body como argumento posicional\n\n  while calling `cell` at <input-expression>:1:5\n    grid.cell()"),
        (
            "repr(grid.header())",
            "grid_header() exige pelo menos uma célula como argumento posicional\n\n  while calling `header` at <input-expression>:1:5\n    grid.header()",
        ),
        (
            "repr(grid.footer())",
            "grid_footer() exige pelo menos uma célula como argumento posicional\n\n  while calling `footer` at <input-expression>:1:5\n    grid.footer()",
        ),
        ("repr(table.cell())", "table_cell() exige body como argumento posicional\n\n  while calling `cell` at <input-expression>:1:5\n    table.cell()"),
        (
            "repr(table.header())",
            "table_header() exige pelo menos uma célula como argumento posicional\n\n  while calling `header` at <input-expression>:1:5\n    table.header()",
        ),
        (
            "repr(table.footer())",
            "table_footer() exige pelo menos uma célula como argumento posicional\n\n  while calling `footer` at <input-expression>:1:5\n    table.footer()",
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

const TEXT_GRADIENT: &str = "gradient.linear(red, blue)";

fn compile_text(body: &str, format: &str) -> (Transcript, Vec<u8>) {
    let scratch = Scratch::new("text-gradient");
    let input = scratch.0.join("text-paint.typ");
    let output = scratch.0.join(format!("text-paint.{format}"));
    fs::write(
        &input,
        format!("#set page(width: 120pt, height: 60pt, margin: 5pt)\n{body}"),
    )
    .unwrap();
    let transcript =
        run(BIN, &["compile", input.to_str().unwrap(), output.to_str().unwrap()]);
    (transcript, fs::read(&output).unwrap_or_default())
}

fn text_svg(body: &str) -> String {
    let (transcript, bytes) = compile_text(body, "svg");
    assert!(transcript.success(), "{body}: {transcript:#?}");
    String::from_utf8(bytes).unwrap()
}

#[test]
fn text_gradient_paints_every_input_route() {
    for body in [
        format!("#text(fill: {TEXT_GRADIENT})[ok]"),
        format!("#text({TEXT_GRADIENT})[ok]"),
        format!("#set text(fill: {TEXT_GRADIENT})\nok"),
        format!("#set text({TEXT_GRADIENT})\nok"),
        "#set text(fill: gradient.radial(red, blue))\nok".into(),
        "#set text(fill: gradient.conic(red, blue))\nok".into(),
        "#text(fill: gradient.linear(red, blue, relative: \"parent\"))[ok]".into(),
        format!("#show strong: set text(fill: {TEXT_GRADIENT})\n*ok*"),
        format!("#set text(fill: {TEXT_GRADIENT})\n#rotate(20deg)[ok] #box[ok]"),
        format!("#set text(fill: {TEXT_GRADIENT})\n$x^2$"),
    ] {
        let svg = text_svg(&body);
        assert!(svg.contains("fill=\"url(#"), "{body}: {svg}");
        assert!(!svg.contains("fill=\"#000000\""), "{body}: black glyph remains");
    }
}

#[test]
fn text_gradient_respects_scope_and_solid_overrides() {
    let svg = text_svg(&format!(
        "a #[#set text(fill: {TEXT_GRADIENT}); b #text(fill: green)[c]] d"
    ));
    assert_eq!(svg.matches("fill=\"url(#").count(), 1, "{svg}");
    assert_eq!(svg.matches("fill=\"#2ecc40\"").count(), 1, "{svg}");
    assert_eq!(svg.matches("fill=\"#000000\"").count(), 2, "{svg}");
}

#[test]
fn text_gradient_uses_nearest_fill_across_constructor_and_set() {
    for (outer, inner, expected) in [
        ("red", TEXT_GRADIENT, ["#ff4136", "gradient", "#ff4136"]),
        (TEXT_GRADIENT, "green", ["gradient", "#2ecc40", "gradient"]),
    ] {
        let body = format!("#text(fill: {outer})[A #[#set text(fill: {inner}); B] C]");
        let svg = text_svg(&body);
        let paints = svg
            .split("<use ")
            .skip(1)
            .map(|glyph| {
                let (_, fill) = glyph.split_once(" fill=\"").unwrap();
                let value = fill.split_once('"').unwrap().0;
                if value.starts_with("url(#") {
                    "gradient"
                } else {
                    value
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(paints, expected, "{body}: {svg}");
    }
}

#[test]
fn text_gradient_decorations_follow_local_paint_and_stroke_precedence() {
    for name in ["strike", "overline"] {
        for (body, expected) in [
            (format!("#{name}[#text(fill: {TEXT_GRADIENT})[ABC]]"), vec!["gradient"]),
            (
                format!(
                    "#set text(fill: {TEXT_GRADIENT})\n#{name}[#text(fill: green)[ABC]]"
                ),
                vec!["#2ecc40"],
            ),
            (
                format!(
                    "#{name}[#text(fill: {TEXT_GRADIENT})[AB]#text(fill: green)[CD]]"
                ),
                vec!["gradient", "#2ecc40"],
            ),
            (
                format!("#{name}(stroke: red)[#text(fill: {TEXT_GRADIENT})[ABC]]"),
                vec!["#ff4136"],
            ),
        ] {
            let svg = text_svg(&body);
            let strokes = svg
                .split(" stroke=\"")
                .skip(1)
                .map(|stroke| {
                    let value = stroke.split_once('"').unwrap().0;
                    if value.starts_with("url(#") {
                        "gradient"
                    } else {
                        value
                    }
                })
                .collect::<Vec<_>>();
            assert_eq!(strokes, expected, "{body}: {svg}");
        }
    }
}

#[test]
fn text_gradient_reaches_decorations_and_math_bars() {
    for body in ["#underline[ok]", "$a/b$", "$sqrt(a)$"] {
        let svg = text_svg(&format!("#set text(fill: {TEXT_GRADIENT})\n{body}"));
        assert!(svg.contains("stroke=\"url(#"), "{body}: {svg}");
    }
    for body in ["$a/b$", "$sqrt(a)$"] {
        let svg = text_svg(&format!("#set text(fill: red)\n{body}"));
        assert!(svg.contains("stroke=\"#ff4136\""), "{body}: {svg}");
    }
}

#[test]
fn text_gradient_emits_pdf_patterns_per_root() {
    let pattern = |body: &str, flag: &[&str]| {
        let scratch = Scratch::new("text-gradient");
        let input = scratch.0.join("p.typ");
        let output = scratch.0.join("p.pdf");
        fs::write(&input, body).unwrap();
        let mut args = vec!["compile", input.to_str().unwrap(), output.to_str().unwrap()];
        args.extend(flag);
        assert!(run(BIN, &args).success());
        let bytes = fs::read(&output).unwrap();
        String::from_utf8_lossy(&bytes).matches("/PatternType 2").count()
    };
    let src = format!(
        "#let g = {TEXT_GRADIENT}\n#set text(fill: g)\nok #box(width: 30pt, height: 20pt)[in] $a/b$"
    );
    for flag in [&[][..], &["--compact"][..]] {
        assert_eq!(pattern(&src, flag), 2, "page and box roots stay distinct");
        assert_eq!(pattern("#text(fill: red)[ok]", flag), 0);
    }
}

#[test]
fn text_gradient_diagnostics_and_limits() {
    for body in [
        "#text(fill: gradient.linear(red, blue, relative: \"self\"))[ok]",
        "#set text(fill: gradient.linear(red, blue, relative: \"self\"))\nok",
        "#set text(gradient.linear(red, blue, relative: \"self\"))\nok",
    ] {
        let (transcript, _) = compile_text(body, "svg");
        assert!(!transcript.success(), "{body}");
        assert!(
            transcript
                .stderr
                .contains("gradients and tilings on text must be relative to the parent"),
            "{body}: {transcript:#?}"
        );
        assert!(
            transcript
                .stderr
                .contains("make sure to set `relative: auto` on your text fill"),
            "{body}: {transcript:#?}"
        );
    }
    let tiling = "tiling(size: (4pt, 4pt), rect(width: 2pt, height: 2pt, fill: red))";
    assert!(!compile_text(&format!("#text(fill: {tiling})[ok]"), "svg").0.success());
    let silent = text_svg(&format!("#set text(fill: {tiling})\nok"));
    assert!(!silent.contains("url(#"), "{silent}");
    for body in ["#set text(fill: 3)\nok", "#text(fill: none)[ok]"] {
        let (transcript, _) = compile_text(body, "svg");
        assert!(transcript.stderr.contains("expected color, gradient, or tiling"));
    }
}

#[test]
fn text_gradient_keeps_geometry_and_paints_png() {
    let body = |paint: &str| {
        format!("#set text(fill: {paint})\noffice #box(width: 30pt)[in] #rotate(9deg)[x]")
    };
    let placements = |svg: &str| {
        svg.split("<use ")
            .skip(1)
            .map(|tag| tag.split(" fill=").next().unwrap().to_owned())
            .collect::<Vec<_>>()
    };
    let gradient = text_svg(&body(TEXT_GRADIENT));
    let solid = text_svg(&body("red"));
    assert!(!placements(&solid).is_empty());
    assert_eq!(placements(&gradient), placements(&solid));

    let png = |paint: &str| {
        let (transcript, bytes) = compile_text(&body(paint), "png");
        assert!(transcript.success(), "{transcript:#?}");
        bytes
    };
    let painted = png(TEXT_GRADIENT);
    for solid in ["black", "red", "blue"] {
        assert_ne!(painted, png(solid), "PNG text must be painted by the gradient");
    }
    assert_ne!(painted, png("gradient.linear(blue, red)"));
}

#[test]
fn text_gradient_identity_separates_resources() {
    let svg = text_svg(
        "#text(fill: gradient.linear(red, blue))[same] #text(fill: gradient.linear(blue, red))[same]",
    );
    assert_eq!(svg.matches("gradientUnits=\"userSpaceOnUse\"").count(), 2, "{svg}");
}

// Expectations: approved calc package (values measured on the pinned reference,
// except the divergences declared by the function owners).
const CALC_MIN: &str = "(-9223372036854775807 - 1)";

fn calc_repr(expression: &str, expected: &str) {
    let output = run(BIN, &["eval", &format!("repr({expression})")]);
    assert!(output.success(), "{expression}: {output:#?}");
    let quoted = format!("\"{}\"", expected.replace('"', "\\\""));
    assert_eq!(output.stdout.trim(), quoted, "{expression}");
}

/// The marker is the byte column of the diagnostic inside `expression`, 0-based.
fn calc_error(expression: &str, message: &str, column: Option<usize>) {
    let output = run(BIN, &["eval", expression]);
    assert!(!output.success(), "{expression}: {output:#?}");
    assert!(
        output.stderr.contains(&format!("error: {message}")),
        "{expression}: {output:#?}"
    );
    if let Some(column) = column {
        assert!(
            output.stderr.contains(&format!(":1:{column}")),
            "{expression}: {output:#?}"
        );
    }
}

#[test]
fn calc_conversions_never_saturate() {
    for function in ["floor", "ceil", "trunc"] {
        for value in [
            "1e300",
            "-1e300",
            "float.inf",
            "-float.inf",
            "9223372036854775808.0",
            "decimal(\"79228162514264337593543950335\")",
        ] {
            calc_error(
                &format!("calc.{function}({value})"),
                "the result is too large",
                None,
            );
        }
        calc_error(
            &format!("calc.{function}(float.nan)"),
            "the result is not a real number",
            None,
        );
        calc_repr(
            &format!("calc.{function}(-9223372036854775808.0) == {CALC_MIN}"),
            "true",
        );
        calc_repr(&format!("calc.{function}(9007199254740993)"), "9007199254740993");
    }
    calc_repr("(calc.floor(-2.5), calc.ceil(-2.5), calc.trunc(-2.5))", "(-3, -2, -2)");
    calc_error("calc.quo(1e300, 0.5)", "the result is too large", None);
    calc_error("calc.quo(float.nan, 2)", "the result is not a real number", None);
}

#[test]
fn calc_extrema_use_language_comparison_and_keep_the_argument() {
    for (expression, expected) in [
        ("calc.max(9007199254740993, 1.0)", "9007199254740993"),
        ("calc.max(1, 0.5)", "1"),
        ("calc.min(2.5, 3)", "2.5"),
        ("calc.max(2.5, 3)", "3"),
        ("calc.max(1, 1.0)", "1"),
        ("calc.max(1.0, 1)", "1.0"),
        ("calc.max(float.nan)", "float.nan"),
        ("calc.max(\"a\", \"b\")", "\"b\""),
        ("calc.min(true, false)", "false"),
        ("calc.min(1deg, 2deg)", "1deg"),
        ("calc.max(10%, 20%)", "20%"),
        ("calc.min(1fr, 2fr)", "1fr"),
        ("calc.max((1, 2), (1, 3))", "(1, 3)"),
        ("calc.max(version(1, 2), version(1, 3))", "version(1, 3)"),
        ("calc.max(decimal(\"1\"), 2, decimal(\"1.5\"))", "2"),
        ("calc.min(decimal(\"1.5\"), 2)", "decimal(\"1.5\")"),
        ("calc.max(1em, 2em)", "2em"),
    ] {
        calc_repr(expression, expected);
    }
    calc_error("calc.max(float.nan, 1)", "cannot compare 1.0 with float.nan", Some(20));
    calc_error("calc.max(1, (2,))", "cannot compare array and integer", Some(12));
    calc_error("calc.min(1em, 2pt)", "cannot compare 2pt with 1em", Some(14));
}

#[test]
fn calc_divisions_and_clamp_accept_decimal() {
    for (expression, expected) in [
        ("calc.quo(decimal(\"7\"), 2)", "3"),
        ("calc.quo(decimal(\"-7\"), 2)", "-4"),
        ("calc.quo(7, decimal(\"2\"))", "3"),
        ("calc.rem(decimal(\"2.5\"), decimal(\"2\"))", "decimal(\"0.5\")"),
        ("calc.rem(-3, decimal(\"2\"))", "decimal(\"-1\")"),
        ("calc.rem-euclid(-3, decimal(\"2\"))", "decimal(\"1\")"),
        ("calc.div-euclid(-3, decimal(\"2\"))", "decimal(\"-2\")"),
        ("calc.div-euclid(decimal(\"-6\"), 2)", "decimal(\"-3\")"),
        ("calc.div-euclid(decimal(\"-7\"), -2)", "decimal(\"4\")"),
        ("calc.clamp(5, 0, decimal(\"3\"))", "decimal(\"3\")"),
        ("calc.clamp(decimal(\"2.5\"), 0, 3)", "decimal(\"2.5\")"),
        ("calc.clamp(decimal(\"2.5\"), 3, 4)", "decimal(\"3\")"),
        ("calc.rem((-9223372036854775807 - 1), -1)", "0"),
        ("calc.quo(-7, 2)", "-4"),
        ("calc.rem(-7.0, 2)", "-1.0"),
    ] {
        calc_repr(expression, expected);
    }
    for function in ["quo", "rem", "rem-euclid", "div-euclid"] {
        calc_error(
            &format!("calc.{function}(decimal(\"1\"), 2.0)"),
            "cannot apply this operation to a decimal and a float",
            None,
        );
        for zero in ["0", "0.0", "decimal(\"0\")"] {
            calc_error(
                &format!("calc.{function}(7, {zero})"),
                "divisor must not be zero",
                Some(function.len() + 9),
            );
        }
        calc_error(
            &format!("calc.{function}(\"a\", 1)"),
            "expected integer, float, or decimal, found string",
            Some(function.len() + 6),
        );
    }
    calc_error(
        "calc.clamp(decimal(\"1\"), 0.0, 2)",
        "cannot apply this operation to a decimal and a float",
        None,
    );
    calc_error(
        "calc.clamp(1, decimal(\"3\"), 2)",
        "max must be greater than or equal to min",
        Some(28),
    );
}

#[test]
fn calc_real_functions_propagate_non_finite_results() {
    for (expression, expected) in [
        ("calc.exp(1000)", "float.inf"),
        ("calc.exp(5000000000)", "float.inf"),
        ("calc.exp(-float.inf)", "0.0"),
        ("calc.sin(float.inf)", "float.nan"),
        ("calc.cos(float.nan)", "float.nan"),
        ("calc.tan(-float.inf)", "float.nan"),
        ("calc.sinh(-1e300)", "-float.inf"),
        ("calc.cosh(1e300)", "float.inf"),
        ("calc.tanh(float.nan)", "float.nan"),
        ("calc.asinh(-float.inf)", "-float.inf"),
        ("calc.acosh(float.inf)", "float.inf"),
        ("calc.atanh(float.nan)", "float.nan"),
        ("calc.sqrt(float.inf)", "float.inf"),
        ("calc.sqrt(float.nan)", "float.nan"),
        ("calc.root(-float.inf, 3)", "-float.inf"),
        ("calc.root(0, -2)", "float.inf"),
        ("calc.pow(0, -2)", "float.inf"),
        ("calc.pow(-1e300, 3)", "-float.inf"),
        ("calc.ln(float.nan)", "float.nan"),
        ("calc.deg(1e308)", "float.inf"),
        ("calc.round(float.inf, digits: 2)", "float.inf"),
        ("calc.round(float.nan, digits: 2)", "float.nan"),
        ("calc.round(1.6e308, digits: -308)", "float.inf"),
        ("calc.round(-float.inf, digits: -309)", "-float.inf"),
    ] {
        calc_repr(expression, expected);
    }
    for expression in [
        "calc.pow(-8, 0.5)",
        "calc.pow(float.nan, 2)",
        "calc.log(float.nan)",
        "calc.log(float.inf)",
        "calc.log10(float.inf)",
    ] {
        calc_error(expression, "the result is not a real number", None);
    }
    calc_error("calc.ln(float.inf)", "the result is not a real number", None);
    for value in ["float.inf", "float.nan"] {
        calc_error(
            &format!("calc.exp({value})"),
            "exponent may not be infinite, subnormal, or NaN",
            Some(9),
        );
    }
    for expression in ["calc.asin(float.nan)", "calc.acos(float.nan)"] {
        assert!(!run(BIN, &["eval", expression]).success(), "{expression}");
    }
}

#[test]
fn calc_norm_and_erf_values() {
    for (expression, expected) in [
        ("calc.norm(p: float.inf, 1, -2)", "2.0"),
        ("calc.norm(1e300, 0) == 1e300", "true"),
        ("calc.norm(float.inf, 1)", "float.inf"),
        ("calc.norm(p: 0.5, 1, -float.inf)", "float.inf"),
        ("calc.norm(p: float.inf, float.nan, float.inf)", "float.nan"),
        ("calc.norm(float.nan, float.inf)", "float.nan"),
        ("calc.norm(p: float.nan, 1, 2)", "float.nan"),
        ("calc.norm(p: float.nan, 1)", "1.0"),
        ("calc.norm(p: float.nan, float.inf)", "float.nan"),
        ("calc.norm()", "0.0"),
        ("calc.norm(0, -0.0)", "0.0"),
        ("calc.norm(3, 4)", "5.0"),
        ("calc.erf(2.5)", "0.999593047982555"),
        ("calc.erf(-3)", "-0.9999779095030014"),
        ("calc.erf(5)", "0.9999999999984626"),
        ("calc.erf(float.nan)", "float.nan"),
        ("calc.erf(-float.inf)", "-1.0"),
        ("calc.erf(-0.0)", "-0.0"),
    ] {
        calc_repr(expression, expected);
    }
    for p in ["0", "-1", "-float.inf"] {
        calc_error(
            &format!("calc.norm(p: {p}, 1)"),
            "p must be greater than zero",
            Some(13),
        );
    }
}

#[test]
fn calc_non_finite_results_reach_layout_without_panic() {
    let scratch = Scratch::new("calc-non-finite");
    for (index, body) in [
        "#rect(width: calc.exp(1000) * 1pt)",
        "#h(calc.sin(float.inf) * 1pt)x",
        "#text(size: calc.exp(1000) * 1pt)[a]",
        "#rotate(calc.sqrt(float.inf) * 1deg)[a]",
        "#set page(width: calc.cosh(1e300) * 1pt); a",
    ]
    .iter()
    .enumerate()
    {
        // The control reaches layout with the same value through operators.
        let control = body
            .replace("calc.exp(1000)", "float.inf")
            .replace("calc.sin(float.inf)", "(float.inf - float.inf)")
            .replace("calc.sqrt(float.inf)", "float.inf")
            .replace("calc.cosh(1e300)", "float.inf");
        let outcome = |name: &str, source: &str| {
            let input = scratch.0.join(format!("{name}{index}.typ"));
            let output = scratch.0.join(format!("{name}{index}.pdf"));
            fs::write(&input, source).unwrap();
            run(BIN, &["compile", input.to_str().unwrap(), output.to_str().unwrap()])
                .success()
        };
        assert_eq!(outcome("calc", body), outcome("control", &control), "{body}");
    }
}

// Expectations: approved value comparison package.
#[test]
fn comparison_rejects_unordered_values() {
    for operator in ["<", "<=", ">", ">="] {
        for (left, right, message) in [
            ("1", "float.nan", "cannot compare 1.0 with float.nan"),
            ("float.nan", "1", "cannot compare float.nan with 1.0"),
            ("float.nan", "float.nan", "cannot compare float.nan with float.nan"),
            ("float.inf", "float.nan", "cannot compare float.inf with float.nan"),
            (
                "9007199254740993",
                "float.nan",
                "cannot compare 9007199254740993 with float.nan",
            ),
            ("(1,)", "(float.nan,)", "cannot compare 1.0 with float.nan"),
            ("1em", "(0% + 1pt)", "cannot compare 1em with 1pt"),
        ] {
            calc_error(&format!("{left} {operator} {right}"), message, Some(0));
        }
    }
    for expression in [
        "if 1 < float.nan { 1 } else { 2 }",
        "{ let i = 0; while i < float.nan { i += 1 }; i }",
        "range(3).filter(x => x < float.nan)",
    ] {
        assert!(!run(BIN, &["eval", expression]).success(), "{expression}");
    }
    calc_repr("false and 1 < float.nan", "false");
    calc_repr(
        "(1 < 1.5, 2.0 >= 2, 1 > 1, 2 <= decimal(\"2\"))",
        "(true, true, false, true)",
    );
    calc_error("calc.max(float.nan, 1)", "cannot compare 1.0 with float.nan", Some(20));
}

#[test]
fn sorted_uses_language_ordering_without_panic() {
    for (expression, expected) in [
        ("(true, false).sorted()", "(false, true)"),
        ("(2pt, 1pt).sorted()", "(1pt, 2pt)"),
        ("(2deg, 1deg).sorted()", "(1deg, 2deg)"),
        ("(2%, 1%).sorted()", "(1%, 2%)"),
        ("(2fr, 1fr).sorted()", "(1fr, 2fr)"),
        ("((2, 1), (1, 2)).sorted()", "((1, 2), (2, 1))"),
        (
            "(duration(days: 2), duration(days: 1)).sorted().map(x => x.days())",
            "(1.0, 2.0)",
        ),
        (
            "(datetime(year: 2021, month: 1, day: 1), datetime(year: 2020, month: 1, day: 1)).sorted().first() == datetime(year: 2020, month: 1, day: 1)",
            "true",
        ),
        ("(decimal(\"2\"), 1).sorted()", "(1, decimal(\"2\"))"),
        ("(\"b\", \"a\").sorted()", "(\"a\", \"b\")"),
        ("((k: 2), (k: 1)).sorted(key: d => (d.k,))", "((k: 1), (k: 2))"),
        ("().sorted()", "()"),
        ("(float.nan,).sorted()", "(float.nan,)"),
        ("range(100).rev().sorted() == range(100)", "true"),
        ("range(100).sorted(by: (a, b) => a > b) == range(100).rev()", "true"),
        (
            "range(200).map(x => calc.rem(x * 37, 101)).sorted(by: (a, b) => calc.rem(a * b, 3) == 0).len()",
            "200",
        ),
        ("range(50).sorted(by: (a, b) => true) == range(50)", "true"),
        ("range(50).sorted(by: (a, b) => false) == range(50)", "true"),
    ] {
        calc_repr(expression, expected);
    }
    // Stable with strict and inclusive callbacks, with and without key.
    for (operator, expected) in
        [("<", "bdac"), ("<=", "bdac"), (">", "acbd"), (">=", "acbd")]
    {
        let items = "((2, \"a\"), (1, \"b\"), (2, \"c\"), (1, \"d\"))";
        for sorted in [
            format!("{items}.sorted(by: (l, r) => l.at(0) {operator} r.at(0))"),
            format!("{items}.sorted(key: x => x.at(0), by: (l, r) => l {operator} r)"),
        ] {
            calc_repr(
                &format!("{sorted}.map(x => x.at(1)).join()"),
                &format!("\"{expected}\""),
            );
        }
    }
    for (expression, message) in [
        ("(2, float.nan, 1).sorted()", "cannot compare float.nan with 2.0"),
        ("(1, \"a\").sorted()", "cannot compare string and integer"),
        ("(1pt, 1em).sorted()", "cannot compare 1em with 1pt"),
        ("(3, 1.5, decimal(\"2\")).sorted()", "cannot compare decimal and float"),
        ("(1, 2).sorted(key: x => float.nan)", "cannot compare float.nan with float.nan"),
        (
            "(1, 2).sorted(by: (l, r) => l < float.nan)",
            "cannot compare 1.0 with float.nan",
        ),
        ("(1, 2).sorted(by: (l, r) => 1)", "expected boolean, found integer"),
    ] {
        calc_error(expression, message, None);
    }
    for element in ["\"a\"", "float.nan", "none"] {
        let output = run(
            BIN,
            &[
                "eval",
                &format!(
                    "range(60).map(x => if calc.rem(x * 7, 5) == 0 {{ {element} }} else {{ calc.rem(x * 37, 61) }}).sorted()"
                ),
            ],
        );
        assert!(!output.success() && !output.stderr.contains("panicked"), "{output:#?}");
        assert!(output.stderr.contains("cannot compare"), "{output:#?}");
    }
}

#[test]
fn equality_joins_integer_and_decimal() {
    for (expression, expected) in [
        ("1 == decimal(\"1.00\")", "true"),
        ("decimal(\"1\") != 1", "false"),
        ("1 == decimal(\"1.5\")", "false"),
        ("9007199254740993 == decimal(\"9007199254740992\")", "false"),
        ("1.0 == decimal(\"1\")", "false"),
        ("decimal(\"1\") in (1,)", "true"),
        ("decimal(\"1\") not in (1,)", "false"),
        ("(1,).contains(decimal(\"1\"))", "true"),
        ("(1, 2, 3).split(2.0)", "((1,), (3,))"),
        ("(1, 2, 3).split(decimal(\"2\"))", "((1,), (3,))"),
        ("(1,).contains(1.0)", "true"),
        ("(a: (1,)) == (a: (decimal(\"1\"),))", "true"),
        ("{ assert.eq(1, decimal(\"1\")); true }", "true"),
        ("(1, 1.0, decimal(\"1\")).dedup()", "(1,)"),
        ("(1.0, decimal(\"1\"), 1).dedup()", "(1.0, decimal(\"1\"))"),
        ("(10%, 10% + 0pt).dedup().len()", "1"),
        ("(1pt, 1pt + 0%).dedup().len()", "1"),
        ("(1, 1.0, 2).dedup(key: x => x)", "(1, 2)"),
        ("(float.nan, float.nan).dedup().len()", "2"),
        ("(3, 1, 3, \"a\", \"a\").dedup()", "(3, 1, \"a\")"),
        ("float.nan * 1% != 50% + 0pt", "true"),
        ("50% + 0pt != float.nan * 1%", "true"),
        ("float.nan * 1% == 50% + 0pt", "false"),
        ("50% != 50% + 0pt", "false"),
    ] {
        calc_repr(expression, expected);
    }
}

// Expectations: approved set contract package.
fn set_svg(label: &str, body: &str) -> (Transcript, String) {
    let scratch = Scratch::new(&format!("set-{label}"));
    let input = scratch.0.join("main.typ");
    let output = scratch.0.join("main.svg");
    fs::write(
        &input,
        format!("#set page(width: 200pt, height: 260pt, margin: 10pt)\n{body}"),
    )
    .unwrap();
    let run = run(
        BIN,
        &[
            "compile",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
            "--format",
            "svg",
        ],
    );
    let svg = fs::read_to_string(&output).unwrap_or_default();
    (run, svg)
}

fn set_equals(label: &str, declared: &str, explicit: &str) {
    let (first, declared_svg) = set_svg(label, declared);
    let (second, explicit_svg) = set_svg(label, explicit);
    assert!(first.success() && second.success(), "{label}: {first:#?} {second:#?}");
    assert!(!first.stderr.contains("ainda não"), "{label}: {}", first.stderr);
    assert!(declared_svg == explicit_svg, "{label}: {declared}");
}

fn set_differs(label: &str, first: &str, second: &str) {
    assert_ne!(set_svg(label, first).1, set_svg(label, second).1, "{label}");
}

#[test]
fn set_admission_rejects_what_the_language_rejects() {
    for (source, message) in [
        ("#set table(nao: 1)", "unexpected argument: nao"),
        ("#set table([a])", "unexpected argument"),
        ("#set block(body: [a])", "unexpected argument: body"),
        ("#set figure(nao: 1)", "unexpected argument: nao"),
        ("#set heading(nao: 1)", "unexpected argument: nao"),
        ("#set par(nao: 1)", "unexpected argument: nao"),
        ("#set document(nao: 1)", "unexpected argument: nao"),
        ("#set smartquote(nao: 1)", "unexpected argument: nao"),
        ("#set list(nao: 1)", "unexpected argument: nao"),
        ("#set math.vec(nao: 1)", "unexpected argument: nao"),
        (
            "#set rect(fill: \"lixo\")",
            "expected color, gradient, tiling, or none, found string",
        ),
        ("#set block(breakable: 1)", "expected boolean, found integer"),
        ("#set table(stroke: auto)", "expected length, color, gradient, tiling, dictionary, stroke, none, array, or function, found auto"),
        ("#set image(fit: \"x\")", "expected \"cover\", \"contain\", or \"stretch\""),
        (
            "#set image(fit: 1)",
            "expected \"cover\", \"contain\", or \"stretch\", found integer",
        ),
        ("#set math.vec(gap: \"a\")", "expected relative length, found string"),
        ("#set calc.abs(1)", "only element functions can be used in set rules"),
        ("#let f() = 1\n#set f()", "only element functions can be used in set rules"),
        (
            "#let r = rect.with(fill: red)\n#set r()",
            "only element functions can be used in set rules",
        ),
        ("#let x = 1\n#set x(a: 1)", "expected function, found integer"),
        ("#set table(fill: red, fill: blue)", "duplicate argument: fill"),
        ("#let t = table\n#set t(nao: 1)", "unexpected argument: nao"),
        ("#let d = (nao: 1)\n#set table(..d)", "unexpected argument: nao"),
    ] {
        let (output, _) = set_svg("admission", source);
        assert!(!output.success(), "{source}: {output:#?}");
        assert!(output.stderr.contains(message), "{source}: {}", output.stderr);
    }
    let (output, _) = set_svg("false", "#set table(nao: 1) if false\nok");
    assert!(output.success(), "{output:#?}");
}

#[test]
fn set_admission_keeps_every_element_settable() {
    for element in [
        "align",
        "bibliography",
        "block",
        "box",
        "circle",
        "cite",
        "colbreak",
        "columns",
        "curve",
        "document",
        "ellipse",
        "emph",
        "enum",
        "figure",
        "figure.caption",
        "footnote",
        "footnote.entry",
        "grid",
        "grid.cell",
        "grid.footer",
        "grid.header",
        "grid.hline",
        "grid.vline",
        "h",
        "heading",
        "hide",
        "highlight",
        "image",
        "line",
        "linebreak",
        "link",
        "list",
        "math.accent",
        "math.cancel",
        "math.cases",
        "math.equation",
        "math.frac",
        "math.lr",
        "math.mat",
        "math.op",
        "math.vec",
        "move",
        "outline",
        "outline.entry",
        "overline",
        "pad",
        "page",
        "pagebreak",
        "par",
        "par.line",
        "place",
        "polygon",
        "quote",
        "raw",
        "rect",
        "ref",
        "repeat",
        "rotate",
        "scale",
        "skew",
        "smallcaps",
        "smartquote",
        "square",
        "stack",
        "strike",
        "strong",
        "sub",
        "super",
        "table",
        "table.cell",
        "table.footer",
        "table.header",
        "table.hline",
        "table.vline",
        "terms",
        "text",
        "title",
        "underline",
        "v",
    ] {
        let (output, _) = set_svg("element", &format!("#set {element}()\nok"));
        assert!(output.success(), "{element}: {output:#?}");
    }
    // Valid declarations without effect warn and never fail.
    for (source, name) in [
        ("#set rect(fill: red)", "rect.fill"),
        ("#set list(marker: [x])", "list.marker"),
        ("#set table(fill: gradient.linear(red, blue))", "table.fill"),
        ("#set table(gutter: 3pt)", "table.gutter"),
        ("#set block(fill: gradient.linear(red, blue))", "block.fill"),
        ("#set outline(depth: 2)", "outline.depth"),
        ("#set heading(outlined: false)", "heading.outlined"),
        ("#set figure(scope: \"parent\")", "figure.scope"),
        ("#set document(date: auto)", "document.date"),
        ("#set underline(stroke: 1pt)", "underline.stroke"),
        ("#set grid.cell(fill: red)", "grid.cell.fill"),
    ] {
        let (output, _) = set_svg("warn", &format!("{source}\nok"));
        assert!(output.success(), "{source}: {output:#?}");
        assert!(output.stderr.contains(name), "{source}: {}", output.stderr);
        assert!(output.stderr.contains("ainda não tem efeito"), "{source}");
    }
}

#[test]
fn set_table_properties_equal_the_constructor() {
    let cells = "[a][b][c][d]";
    for (property, value) in [
        ("columns", "2"),
        ("columns", "(1fr, 30pt)"),
        ("rows", "40pt"),
        ("fill", "red"),
        ("fill", "none"),
        ("stroke", "3pt"),
        ("stroke", "red"),
        ("stroke", "2pt + blue"),
        ("stroke", "none"),
        ("inset", "12pt"),
    ] {
        set_equals(
            "table",
            &format!("#set table({property}: {value})\n#table{cells}"),
            &format!("#table({property}: {value}){cells}"),
        );
        // Explicit value wins.
        set_equals(
            "table-explicit",
            &format!("#set table(columns: 3, fill: blue, stroke: 9pt + green, inset: 1pt, rows: 9pt)\n#table(columns: 2, rows: auto, fill: none, stroke: none, inset: 5pt){cells}"),
            &format!("#table(columns: 2, rows: auto, fill: none, stroke: none, inset: 5pt){cells}"),
        );
    }
    let target = "#table(fill: red)[a]";
    for (label, source) in [
        ("late", "#let t = table[a]\n#set table(fill: red)\n#t"),
        ("alias", "#let t = table\n#set t(fill: red)\n#table[a]"),
        ("spread", "#let d = (fill: red)\n#set table(..d)\n#table[a]"),
        ("show-set", "#show table: set table(fill: red)\n#table[a]"),
        ("template", "#let w(b) = { set table(fill: red); b }\n#show: w\n#table[a]"),
        ("function", "#let w(b) = { set table(fill: red); b }\n#w(table[a])"),
        ("nearest", "#set table(fill: blue)\n#set table(fill: red)\n#table[a]"),
    ] {
        set_equals(label, source, target);
    }
    set_equals(
        "scope",
        "#[#set table(fill: red)\n#table[a]]\n\n#table[b]",
        "#[#table(fill: red)[a]]\n\n#table[b]",
    );
    for source in [
        "#set table(stroke: red)\n#set table(stroke: 3pt)\n#table[a]",
        "#set table(stroke: red)\n#table(stroke: 3pt)[a]",
        "#set table(stroke: 3pt)\n#table(stroke: red)[a]",
        "#show table: set table(stroke: red)\n#table(stroke: 3pt)[a]",
    ] {
        set_equals("fold", source, "#table(stroke: 3pt + red)[a]");
    }
    set_equals(
        "inset-fold",
        "#set table(inset: (x: 20pt))\n#set table(inset: (y: 2pt))\n#table[a]",
        "#set table.cell(inset: (x: 20pt, y: 2pt))\n#table[a]",
    );
    set_equals(
        "hline",
        "#set table(columns: 2)\n#table([a], [b], table.hline(stroke: 4pt), [c], [d])",
        "#table(columns: 2, [a], [b], table.hline(stroke: 4pt), [c], [d])",
    );
}

#[test]
fn set_block_properties_equal_the_constructor() {
    for (property, value) in [
        ("width", "60pt"),
        ("height", "40pt"),
        ("inset", "9pt"),
        ("inset", "(x: 20pt)"),
        ("outset", "6pt"),
        ("radius", "4pt"),
        ("clip", "true"),
        ("fill", "red"),
        ("stroke", "3pt + blue"),
        ("spacing", "50pt"),
        ("above", "50pt"),
        ("below", "50pt"),
        ("breakable", "false"),
        ("sticky", "true"),
    ] {
        let extra =
            if matches!(property, "radius" | "outset") { ", fill: red" } else { "" };
        let body = |args: &str| {
            format!("x\n\n#block({args})[#lorem(5)]\n\n#block({args})[b]\n\ny")
        };
        set_equals(
            property,
            &format!("#set block({property}: {value}{extra})\n{}", body("")),
            &body(&format!("{property}: {value}{extra}")),
        );
    }
    set_equals(
        "block-explicit",
        "#set block(fill: red, stroke: 2pt, inset: 9pt, radius: 5pt)\n#block(fill: none, stroke: none, inset: 0pt, radius: 0pt)[a]",
        "#block[a]",
    );
    set_equals(
        "block-sides",
        "#set block(inset: (x: 20pt, top: 3pt), fill: red)\n#block(inset: (left: 1pt))[a]",
        "#block(inset: (left: 1pt, right: 20pt, top: 3pt), fill: red)[a]",
    );
    set_equals(
        "block-stroke",
        "#set block(stroke: red)\n#block(stroke: 3pt)[a]",
        "#set block(stroke: 3pt)\n#block(stroke: red)[a]",
    );
    set_equals(
        "block-show-set",
        "#show block: set block(fill: red)\n#block[a]",
        "#block(fill: red)[a]",
    );
    set_equals(
        "block-spacing-last",
        "#set block(above: 50pt)\n#set block(spacing: 5pt)\nx #block[a] y",
        "x #block(spacing: 5pt)[a] y",
    );
    // The same value under two scopes.
    set_equals(
        "block-reuse",
        "#let b = block[a]\n#b #[#set block(fill: red)\n#b] #b",
        "#block[a] #[#block(fill: red)[a]] #block[a]",
    );
}

#[test]
fn set_heading_figure_and_footnote_properties_take_effect() {
    set_equals("offset", "#set heading(offset: 1)\n= A\n== B", "== A\n=== B");
    set_equals(
        "offset-native",
        "#set heading(offset: 1, numbering: \"1.\")\n= Z\n#heading[A]\n#heading(level: 1)[B]\n#heading(depth: 2)[C]",
        "#set heading(numbering: \"1.\")\n== Z\n#heading(level: 2)[A]\n#heading(level: 1)[B]\n#heading(level: 3)[C]",
    );
    set_equals("offset-arg", "#heading(offset: 2)[A]", "=== A");
    set_equals(
        "offset-select",
        "#set heading(offset: 1)\n#show heading.where(level: 2): set text(red)\n= A",
        "#show heading.where(level: 2): set text(red)\n== A",
    );
    set_equals(
        "gap-default",
        "#set figure(gap: 0.65em)\n#figure(caption: [c])[b]",
        "#figure(caption: [c])[b]",
    );
    set_differs(
        "gap",
        "#set figure(gap: 40pt)\n#figure(caption: [c])[b]",
        "#figure(caption: [c])[b]",
    );
    set_equals(
        "footnote",
        "#set footnote(numbering: \"a\")\nx#footnote[n]",
        "x#footnote(numbering: \"a\")[n]",
    );
    set_differs(
        "heading-above",
        "#show heading: set block(above: 80pt)\nx\n= A",
        "x\n= A",
    );
    set_differs(
        "heading-below",
        "#show heading: set block(below: 80pt)\n= A\nx",
        "= A\nx",
    );
    set_equals("heading-ambient", "#set block(above: 80pt)\nx\n= A", "x\n= A");
}
