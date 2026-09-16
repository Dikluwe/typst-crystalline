//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/p1428-tests.md
//! @prompt-hash a8451345
//! @layer L3
//!
//! Recalibração independente dos observadores públicos `math.frac` P1428.

use std::path::{Path, PathBuf};

use typst_core::compiler::layout::FontMetrics;
use typst_core::contracts::world::World;
use typst_core::entities::compiler_features::Features;
use typst_core::entities::content::Content;
use typst_core::entities::layout_types::TextStyle;
use typst_infra::font_metrics::FontBookMetrics;
use typst_infra::pipeline::{
    compile_to_html_string_with_features, compile_to_svg_string, eval_to_module_with_sink,
};
use typst_infra::world::SystemWorld;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unknown {
    HtmlEntity,
    HtmlMarkup,
    MathFracCardinality,
    ContentTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Classification {
    Preserved,
    Violated,
    Unknown,
}

fn mutant(id: &str) -> bool {
    std::env::var("P1428_MUTANT").as_deref() == Ok(id)
}

struct TempDir(PathBuf);

impl TempDir {
    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn fixture_world(prefix: &str, source_text: &str) -> (TempDir, SystemWorld) {
    let path = std::env::temp_dir().join(format!(
        "typst-p1428-{prefix}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
    ));
    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(path.join("main.typ"), source_text).unwrap();
    let dir = TempDir(path);
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    (dir, world)
}

fn fixture_world_with_case(source_text: &str, case_text: &str) -> (TempDir, SystemWorld) {
    let (dir, world) = fixture_world("module-indirect", source_text);
    std::fs::write(dir.path().join("case.typ"), case_text).unwrap();
    (dir, world)
}

fn compile(source_text: &str) -> Result<String, String> {
    let (_dir, world) = fixture_world("html", source_text);
    let source = world.source(world.main()).unwrap();
    compile_to_html_string_with_features(&world, &source, Features::html())
        .0
        .map_err(|errors| {
            errors
                .into_iter()
                .map(|error| error.message)
                .collect::<Vec<_>>()
                .join("\n")
        })
}

fn html(source: &str) -> String {
    compile(source).unwrap_or_else(|error| panic!("fixture failed: {source:?}: {error}"))
}

fn svg(source_text: &str) -> String {
    let (_dir, world) = fixture_world("svg", source_text);
    let source = world.source(world.main()).unwrap();
    compile_to_svg_string(&world, &source)
        .0
        .unwrap_or_else(|errors| panic!("SVG fixture {source_text:?}: {errors:?}"))
}

fn decode_text_node(input: &str) -> Result<String, Unknown> {
    let mut output = String::new();
    let mut cursor = 0;
    while let Some(relative) = input[cursor..].find('&') {
        let amp = cursor + relative;
        output.push_str(&input[cursor..amp]);
        let tail = &input[amp + 1..];
        let Some(end) = tail.find(';') else {
            return Err(Unknown::HtmlEntity);
        };
        let reference = &tail[..end];
        let decoded = match reference {
            "amp" => '&',
            "lt" => '<',
            "gt" => '>',
            "quot" => '"',
            "apos" => '\'',
            numeric if numeric.starts_with("#x") || numeric.starts_with("#X") => {
                u32::from_str_radix(&numeric[2..], 16)
                    .ok()
                    .and_then(char::from_u32)
                    .ok_or(Unknown::HtmlEntity)?
            }
            numeric if numeric.starts_with('#') => numeric[1..]
                .parse::<u32>()
                .ok()
                .and_then(char::from_u32)
                .ok_or(Unknown::HtmlEntity)?,
            _ if mutant("M04") => '?',
            _ => return Err(Unknown::HtmlEntity),
        };
        output.push(decoded);
        cursor = amp + end + 2;
    }
    output.push_str(&input[cursor..]);
    Ok(output)
}

fn tag_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut quote = None;
    for (offset, byte) in bytes[start..].iter().copied().enumerate() {
        match (quote, byte) {
            (None, b'\'' | b'"') => quote = Some(byte),
            (Some(open), close) if open == close => quote = None,
            (None, b'>') => return Some(start + offset),
            _ => {}
        }
    }
    None
}

fn tag_name(token: &str) -> Option<&str> {
    let token = token.trim_start();
    let end = token
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
        .unwrap_or(token.len());
    (end > 0).then_some(&token[..end])
}

fn extract_html_text_once(serialized: &str) -> Result<String, Unknown> {
    if mutant("M03") && !serialized.contains('<') {
        return decode_text_node(serialized);
    }
    let bytes = serialized.as_bytes();
    let mut output = String::new();
    let mut stack: Vec<String> = Vec::new();
    let mut cursor = 0;
    let mut starts = 0usize;
    let mut matched = 0usize;

    while cursor < bytes.len() {
        if bytes[cursor] != b'<' {
            let end = serialized[cursor..]
                .find('<')
                .map_or(bytes.len(), |offset| cursor + offset);
            output.push_str(&decode_text_node(&serialized[cursor..end])?);
            cursor = end;
            continue;
        }

        let end = tag_end(bytes, cursor + 1).ok_or(Unknown::HtmlMarkup)?;
        let token = &serialized[cursor + 1..end];
        let trimmed = token.trim();
        if trimmed.starts_with("!--") || trimmed.starts_with('?') {
            return Err(Unknown::HtmlMarkup);
        }
        if trimmed.starts_with('!') {
            if !trimmed.to_ascii_lowercase().starts_with("!doctype ") {
                return Err(Unknown::HtmlMarkup);
            }
        } else if let Some(rest) = trimmed.strip_prefix('/') {
            let name = tag_name(rest).ok_or(Unknown::HtmlMarkup)?.to_ascii_lowercase();
            if rest[name.len()..].trim().is_empty()
                && stack.last().is_some_and(|open| open == &name)
            {
                stack.pop();
                matched += 1;
            } else {
                return Err(Unknown::HtmlMarkup);
            }
        } else {
            let name = tag_name(trimmed).ok_or(Unknown::HtmlMarkup)?.to_ascii_lowercase();
            if matches!(name.as_str(), "script" | "style" | "textarea" | "title") {
                return Err(Unknown::HtmlMarkup);
            }
            starts += 1;
            let is_void = matches!(
                name.as_str(),
                "area"
                    | "base"
                    | "br"
                    | "col"
                    | "embed"
                    | "hr"
                    | "img"
                    | "input"
                    | "link"
                    | "meta"
                    | "param"
                    | "source"
                    | "track"
                    | "wbr"
            );
            let self_closing = trimmed.ends_with('/');
            if !is_void && !self_closing {
                stack.push(name);
            }
        }
        cursor = end + 1;
    }

    if starts == 0 || matched == 0 || !stack.is_empty() {
        return Err(Unknown::HtmlMarkup);
    }
    if mutant("M02") {
        output = output
            .replace("&quot;", "\"")
            .replace("&apos;", "'")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&");
    }
    Ok(output)
}

fn math_constant_offset(bytes: &[u8], constant_offset: usize) -> usize {
    let num_tables = u16::from_be_bytes([bytes[4], bytes[5]]) as usize;
    let record = (0..num_tables)
        .map(|index| 12 + index * 16)
        .find(|&offset| &bytes[offset..offset + 4] == b"MATH")
        .expect("MATH table record");
    let table = u32::from_be_bytes([
        bytes[record + 8],
        bytes[record + 9],
        bytes[record + 10],
        bytes[record + 11],
    ]) as usize;
    let constants = u16::from_be_bytes([bytes[table + 4], bytes[table + 5]]) as usize;
    table + constants + constant_offset
}

fn productive_constants(
    bytes: &[u8],
) -> typst_core::entities::math_constants::MathConstants {
    FontBookMetrics::from_bytes(bytes)
        .expect("valid font fixture")
        .math_constants(&TextStyle::default())
}

fn evaluate_module_carriers(
    source_text: &str,
) -> Result<Vec<(bool, bool, bool, bool)>, Unknown> {
    let (_dir, world) = fixture_world("module", source_text);
    let source = world.source(world.main()).unwrap();
    let module = eval_to_module_with_sink(&world, &source)
        .0
        .unwrap_or_else(|errors| panic!("module fixture {source_text:?}: {errors:?}"));
    module.content().map_or_else(|| Ok(Vec::new()), collect_math_fracs)
}

fn evaluate_indirect_module_carriers(
    case_text: &str,
) -> Result<Vec<(bool, bool, bool, bool)>, Unknown> {
    let main = "#include \"case.typ\"";
    let (_dir, world) = fixture_world_with_case(main, case_text);
    let source = world.source(world.main()).unwrap();
    let module = eval_to_module_with_sink(&world, &source)
        .0
        .unwrap_or_else(|errors| panic!("indirect module fixture: {errors:?}"));
    module.content().map_or_else(|| Ok(Vec::new()), collect_math_fracs)
}

fn evaluate_unique_frac(source_text: &str) -> Result<(bool, bool), Unknown> {
    if mutant("M05") {
        let observed = svg(source_text);
        assert!(!observed.is_empty(), "SVG observer must return readable bytes");
        return Ok((false, false));
    }
    let carrier_bits =
        unique_frac_bits_from_carriers(&evaluate_module_carriers(source_text)?)?;
    if mutant("M09") {
        Ok((source_text.contains("$(a) /"), source_text.contains("/ (b)$")))
    } else {
        Ok(carrier_bits)
    }
}

fn evaluate_indirect_unique_frac(case_text: &str) -> Result<(bool, bool), Unknown> {
    if mutant("M09") {
        let observed_main = "#include \"case.typ\"";
        return Ok((observed_main.contains("$(a) /"), observed_main.contains("/ (b)$")));
    }
    unique_frac_bits_from_carriers(&evaluate_indirect_module_carriers(case_text)?)
}

fn unique_frac_bits_from_carriers(
    carriers: &[(bool, bool, bool, bool)],
) -> Result<(bool, bool), Unknown> {
    let [(line, style_is_none, numerator, denominator)] = carriers else {
        if mutant("M06") && carriers.is_empty() {
            return Ok((false, false));
        }
        if mutant("M07") && !carriers.is_empty() {
            let (_, _, numerator, denominator) = carriers[0];
            return Ok((numerator, denominator));
        }
        return Err(Unknown::MathFracCardinality);
    };
    assert!(*line, "unique MathFrac must retain line=true");
    assert!(*style_is_none, "slash carrier must retain style=None");
    if mutant("M08") {
        Ok((*denominator, *numerator))
    } else {
        Ok((*numerator, *denominator))
    }
}

fn conjunction(bits: Classification, layout: Classification) -> Classification {
    if mutant("M10") {
        return bits;
    }
    match (bits, layout) {
        (Classification::Violated, _) | (_, Classification::Violated) => {
            Classification::Violated
        }
        (Classification::Unknown, _) | (_, Classification::Unknown) => {
            Classification::Unknown
        }
        (Classification::Preserved, Classification::Preserved) => {
            Classification::Preserved
        }
    }
}

fn identity_classification(matches: bool) -> Classification {
    matches
        .then_some(Classification::Preserved)
        .unwrap_or(Classification::Unknown)
}

fn execution_classification(readable: bool) -> Classification {
    readable
        .then_some(Classification::Preserved)
        .unwrap_or(Classification::Unknown)
}

fn mutation_credit(classification: Classification) -> usize {
    if mutant("M12") && classification == Classification::Unknown {
        1
    } else {
        usize::from(classification == Classification::Violated)
    }
}

const ROUTE_CONTROL: u8 = 0b0001;
const DIAGNOSTIC_CONTROL: u8 = 0b0010;
const OPENTYPE_CONTROL: u8 = 0b0100;
const BINOM_CONTROL: u8 = 0b1000;
const ALL_CONTROLS: u8 =
    ROUTE_CONTROL | DIAGNOSTIC_CONTROL | OPENTYPE_CONTROL | BINOM_CONTROL;

fn declared_control_mask() -> u8 {
    if mutant("M11") {
        ALL_CONTROLS & !DIAGNOSTIC_CONTROL
    } else {
        ALL_CONTROLS
    }
}

const CONTRACT_CASES: [&str; 17] = [
    "P01", "P02", "P03", "P04", "P05", "P06", "P07", "P08", "C01", "C02", "C03", "C04",
    "C05", "U02", "U03", "U04", "U05",
];

fn assert_repr_case(style: &str) {
    let serialized = html(&format!("#repr(math.frac([a], [b], style: \"{style}\"))"));
    let text = extract_html_text_once(&serialized).unwrap();
    assert!(text.contains(&format!("style: \"{style}\"")), "{style}: {text}");
}

fn assert_route_control() {
    for (source, expected) in [
        ("#repr(math.frac([a], [b]))", "frac(num: [a], denom: [b])"),
        ("#repr($frac(a,b)$.body)", "frac(num: a, denom: b)"),
        ("#repr($a/b$.body)", "frac(num: a, denom: b)"),
    ] {
        let text = extract_html_text_once(&html(source)).unwrap();
        assert!(text.contains(expected), "{source}: {text}");
    }
}

fn assert_diagnostic_control() {
    for (source, expected) in [
        ("#math.frac()", "missing argument: num"),
        ("#math.frac([a])", "missing argument: denom"),
        ("#math.frac([a], [b], [c])", "unexpected argument"),
        ("#math.frac(num: [a], [b])", "the argument `num` is positional"),
        ("#math.frac([a], denom: [b])", "the argument `denom` is positional"),
        ("#math.frac([a], [b], foo: 1)", "unexpected argument: foo"),
        ("#math.frac([a], [b], style: \"vertical\", style: \"skewed\")", "duplicate"),
        ("#math.frac([a], [b], style: \"diagonal\")", "vertical"),
        ("#math.frac([a], [b], style: 1)", "integer"),
        ("#math.frac(1, [b])", "expected content, found integer"),
        ("#math.frac([a], 1)", "expected content, found integer"),
        ("$frac(a)$", "missing argument: denom"),
        ("$frac(a,b,c)$", "unexpected argument"),
        ("$frac(a,b,foo: 1)$", "unexpected argument"),
        ("#set math.frac(style: \"diagonal\"); $a/b$", "vertical"),
        ("#set math.frac(style: 1); $a/b$", "integer"),
    ] {
        let error = compile(source).unwrap_err();
        assert!(
            error.contains(expected),
            "{source}: expected {expected:?}, got {error:?}"
        );
    }
}

fn assert_opentype_control() {
    let bytes = include_bytes!("../fixtures/fonts/NewCMMath-Book.otf");
    let productive = productive_constants(bytes);
    assert_eq!(productive.skewed_fraction_vertical_gap, 96.0);
    assert_eq!(productive.skewed_fraction_horizontal_gap, 350.0);
    for constant_offset in [156usize, 160usize] {
        let mut zeroed = bytes.to_vec();
        let offset = math_constant_offset(&zeroed, constant_offset);
        zeroed[offset..offset + 2].copy_from_slice(&0i16.to_be_bytes());
        let extracted = productive_constants(&zeroed);
        if constant_offset == 156 {
            assert_eq!(extracted.skewed_fraction_horizontal_gap, 0.0);
            assert_eq!(extracted.skewed_fraction_vertical_gap, 96.0);
        } else {
            assert_eq!(extracted.skewed_fraction_vertical_gap, 0.0);
            assert_eq!(extracted.skewed_fraction_horizontal_gap, 350.0);
        }
    }
}

fn assert_binom_control() {
    let before = extract_html_text_once(&html("#repr($binom(n,k)$.body)")).unwrap();
    let after = extract_html_text_once(&html(
        "#set math.frac(style: \"skewed\"); #repr($binom(n,k)$.body)",
    ))
    .unwrap();
    let expected = "binom(upper: [n], lower: ([k],))";
    assert!(before.contains(expected), "{before}");
    assert!(after.contains(expected), "{after}");
    assert!(!after.contains("style:") && !after.contains("frac("), "{after}");
}

fn assert_html_boundary_control() {
    assert_eq!(
        extract_html_text_once("<p>style: &quot;vertical&quot;</p>"),
        Ok("style: \"vertical\"".into())
    );
    assert_eq!(extract_html_text_once("<p>&amp;quot;</p>"), Ok("&quot;".into()));
    assert_eq!(extract_html_text_once("<p>&#34;&#x22;</p>"), Ok("\"\"".into()));
    assert_eq!(extract_html_text_once("<p>&bogus;</p>"), Err(Unknown::HtmlEntity));
    assert_eq!(
        extract_html_text_once("style: &quot;vertical&quot;"),
        Err(Unknown::HtmlMarkup)
    );
}

fn run_contract_case(id: &str) -> Classification {
    match id {
        "P01" => assert_repr_case("vertical"),
        "P02" => assert_repr_case("skewed"),
        "P03" => assert_repr_case("horizontal"),
        "P04" => assert_eq!(
            evaluate_unique_frac("#set math.frac(style: \"horizontal\")\n$a / b$"),
            Ok((false, false))
        ),
        "P05" => assert_eq!(
            evaluate_unique_frac("#set math.frac(style: \"horizontal\")\n$(a) / b$"),
            Ok((true, false))
        ),
        "P06" => assert_eq!(
            evaluate_unique_frac("#set math.frac(style: \"horizontal\")\n$a / (b)$"),
            Ok((false, true))
        ),
        "P07" => assert_eq!(
            evaluate_unique_frac("#set math.frac(style: \"horizontal\")\n$(a) / (b)$"),
            Ok((true, true))
        ),
        "P08" => assert_eq!(
            conjunction(Classification::Preserved, Classification::Preserved),
            Classification::Preserved
        ),
        "C01" => assert_route_control(),
        "C02" => assert_diagnostic_control(),
        "C03" => assert_opentype_control(),
        "C04" => assert_binom_control(),
        "C05" => assert_html_boundary_control(),
        "U02" => {
            assert_eq!(
                extract_html_text_once("<p>&bogus;</p>"),
                Err(Unknown::HtmlEntity)
            );
            return Classification::Unknown;
        }
        "U03" => {
            assert_eq!(extract_html_text_once("plain text"), Err(Unknown::HtmlMarkup));
            return Classification::Unknown;
        }
        "U04" => {
            assert_eq!(
                evaluate_unique_frac("plain text without a fraction"),
                Err(Unknown::MathFracCardinality)
            );
            return Classification::Unknown;
        }
        "U05" => {
            assert_eq!(
                evaluate_unique_frac("$a/b$ and $c/d$"),
                Err(Unknown::MathFracCardinality)
            );
            return Classification::Unknown;
        }
        _ => panic!("unknown P1428 contract case: {id}"),
    }
    Classification::Preserved
}

#[test]
fn p1428_runner_ordem_contrato() {
    let order = std::env::var("P1428_ORDER").unwrap_or_else(|_| "forward".into());
    let ids: Vec<&str> = match order.as_str() {
        "forward" => CONTRACT_CASES.to_vec(),
        "reverse" => CONTRACT_CASES.iter().rev().copied().collect(),
        _ => panic!("P1428_ORDER must be forward or reverse, got {order:?}"),
    };
    let mut vector = Vec::with_capacity(ids.len());
    for id in ids {
        let classification = run_contract_case(id);
        eprintln!("P1428_CASE id={id} classification={classification:?}");
        vector.push(format!("{id}={classification:?}"));
    }
    eprintln!("P1428_VECTOR order={order} {}", vector.join(","));
}

fn collect_math_fracs(
    content: &Content,
) -> Result<Vec<(bool, bool, bool, bool)>, Unknown> {
    let mut carriers = Vec::new();
    content
        .map_content(&mut |node| {
            if let Content::MathFrac(frac) = node {
                carriers.push((
                    frac.line,
                    frac.style.is_none(),
                    frac.num_deparenthesized,
                    frac.denom_deparenthesized,
                ));
            }
            Ok(None)
        })
        .map_err(|_| Unknown::ContentTraversal)?;
    Ok(carriers)
}

#[test]
fn p1428_repr_compara_texto_html_decodificado() {
    for style in ["vertical", "skewed", "horizontal"] {
        let serialized = html(&format!("#repr(math.frac([a], [b], style: \"{style}\"))"));
        let text = if mutant("M01") {
            serialized.clone()
        } else {
            extract_html_text_once(&serialized).unwrap_or_else(|unknown| {
                panic!("{style}: unexpected {unknown:?}: {serialized}")
            })
        };
        let expected = format!("style: \"{style}\"");
        assert!(text.contains(&expected), "{style}: {text}");
    }
}

#[test]
fn p1428_decoder_html_e_fail_closed() {
    assert_eq!(
        extract_html_text_once("<p>style: &quot;vertical&quot;</p>"),
        Ok("style: \"vertical\"".into())
    );
    assert_eq!(extract_html_text_once("<p>&amp;quot;</p>"), Ok("&quot;".into()));
    assert_eq!(extract_html_text_once("<p>&#34;&#x22;</p>"), Ok("\"\"".into()));
    assert_eq!(extract_html_text_once("<p>&bogus;</p>"), Err(Unknown::HtmlEntity));
    assert_eq!(
        extract_html_text_once("style: &quot;vertical&quot;"),
        Err(Unknown::HtmlMarkup)
    );
    assert_eq!(extract_html_text_once("<p>unbalanced"), Err(Unknown::HtmlMarkup));
}

#[test]
fn p1428_modulo_expoe_frac_unica_e_quatro_bits() {
    for (source, expected) in [
        ("#set math.frac(style: \"horizontal\")\n$a / b$", (false, false)),
        ("#set math.frac(style: \"horizontal\")\n$(a) / b$", (true, false)),
        ("#set math.frac(style: \"horizontal\")\n$a / (b)$", (false, true)),
        ("#set math.frac(style: \"horizontal\")\n$(a) / (b)$", (true, true)),
    ] {
        assert_eq!(evaluate_unique_frac(source), Ok(expected), "{source}");
    }
    assert_eq!(
        evaluate_unique_frac("plain text without a fraction"),
        Err(Unknown::MathFracCardinality)
    );
    assert_eq!(
        evaluate_unique_frac("$a/b$ and $c/d$"),
        Err(Unknown::MathFracCardinality)
    );
    assert_eq!(
        evaluate_indirect_unique_frac(
            "#set math.frac(style: \"horizontal\")\n$(a) / (b)$"
        ),
        Ok((true, true)),
        "carrier bits must come from the evaluated dependency, not main source spelling"
    );
}

#[test]
fn p1428_controles_rotas_diagnosticos_constantes_e_binom() {
    assert_eq!(
        declared_control_mask(),
        ALL_CONTROLS,
        "C01-C04 meta-gate: route, diagnostic, OpenType and binom controls are mandatory"
    );
    for (source, expected) in [
        ("#repr(math.frac([a], [b]))", "frac(num: [a], denom: [b])"),
        ("#repr($frac(a,b)$.body)", "frac(num: a, denom: b)"),
        ("#repr($a/b$.body)", "frac(num: a, denom: b)"),
    ] {
        let text = extract_html_text_once(&html(source)).unwrap();
        assert!(text.contains(expected), "{source}: {text}");
    }

    for (source, expected) in [
        ("#math.frac()", "missing argument: num"),
        ("#math.frac([a])", "missing argument: denom"),
        ("#math.frac([a], [b], [c])", "unexpected argument"),
        ("#math.frac(num: [a], [b])", "the argument `num` is positional"),
        ("#math.frac([a], denom: [b])", "the argument `denom` is positional"),
        ("#math.frac([a], [b], foo: 1)", "unexpected argument: foo"),
        ("#math.frac([a], [b], style: \"vertical\", style: \"skewed\")", "duplicate"),
        ("#math.frac([a], [b], style: \"diagonal\")", "vertical"),
        ("#math.frac([a], [b], style: 1)", "integer"),
        ("#math.frac(1, [b])", "expected content, found integer"),
        ("#math.frac([a], 1)", "expected content, found integer"),
        ("$frac(a)$", "missing argument: denom"),
        ("$frac(a,b,c)$", "unexpected argument"),
        ("$frac(a,b,foo: 1)$", "unexpected argument"),
        ("#set math.frac(style: \"diagonal\"); $a/b$", "vertical"),
        ("#set math.frac(style: 1); $a/b$", "integer"),
    ] {
        let error = compile(source).unwrap_err();
        assert!(
            error.contains(expected),
            "{source}: expected {expected:?}, got {error:?}"
        );
    }

    let bytes = include_bytes!("../fixtures/fonts/NewCMMath-Book.otf");
    let productive = productive_constants(bytes);
    assert_eq!(productive.skewed_fraction_vertical_gap, 96.0);
    assert_eq!(productive.skewed_fraction_horizontal_gap, 350.0);
    for constant_offset in [156usize, 160usize] {
        let mut zeroed = bytes.to_vec();
        let offset = math_constant_offset(&zeroed, constant_offset);
        zeroed[offset..offset + 2].copy_from_slice(&0i16.to_be_bytes());
        let extracted = productive_constants(&zeroed);
        if constant_offset == 156 {
            assert_eq!(extracted.skewed_fraction_horizontal_gap, 0.0);
            assert_eq!(extracted.skewed_fraction_vertical_gap, 96.0);
        } else {
            assert_eq!(extracted.skewed_fraction_vertical_gap, 0.0);
            assert_eq!(extracted.skewed_fraction_horizontal_gap, 350.0);
        }
    }

    let before = extract_html_text_once(&html("#repr($binom(n,k)$.body)")).unwrap();
    let after = extract_html_text_once(&html(
        "#set math.frac(style: \"skewed\"); #repr($binom(n,k)$.body)",
    ))
    .unwrap();
    let expected = "binom(upper: [n], lower: ([k],))";
    assert!(before.contains(expected), "{before}");
    assert!(after.contains(expected), "{after}");
    assert!(!after.contains("style:") && !after.contains("frac("), "{after}");
}

#[test]
fn p1428_classificadores_conjuntivo_score_e_meta_gate() {
    assert_eq!(
        conjunction(Classification::Preserved, Classification::Preserved),
        Classification::Preserved
    );
    assert_eq!(
        conjunction(Classification::Preserved, Classification::Unknown),
        Classification::Unknown,
        "P08 cannot be granted from bits when layout is opaque"
    );
    assert_eq!(
        conjunction(Classification::Preserved, Classification::Violated),
        Classification::Violated
    );

    let u01 = identity_classification(false);
    let u06 = execution_classification(false);
    assert_eq!(u01, Classification::Unknown);
    assert_eq!(u06, Classification::Unknown);
    assert_eq!(
        mutation_credit(u01) + mutation_credit(u06),
        0,
        "U01 and U06 Unknown receive no mutation credit"
    );
    assert_eq!(declared_control_mask(), ALL_CONTROLS, "all four controls are declared");
}
