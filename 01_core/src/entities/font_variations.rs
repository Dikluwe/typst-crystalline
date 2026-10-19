//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/font_variations.md
//! @prompt-hash b2f3f115
//! @layer L1
//! @updated 2026-07-22
//!
//! P836 (achado #21 de P831) — `FontVariations`: coordenadas de eixo
//! OpenType explícitas de `#text(variations:)` / `#set text(variations:)`.
//!
//! Contraponto de `text/font/variations.rs` do vanilla 0.15.0. A tag é
//! `[u8; 4]` com padding de espaço (paridade `Tag::from_bytes_lossy`,
//! `tag.rs:22-27`) — L1 não depende de `ttf_parser`. A validação
//! (`from_value`) replica verbatim as mensagens e hints do vanilla
//! (`variations.rs:217-236`, `tag.rs:85-117`), medidos em `temp/p836/`.

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::source_result::SourceDiagnostic;
use crate::entities::span::Span;
use crate::entities::value::Value;

/// Lista de coordenadas de eixo de variação `(tag, valor)`, normalizada
/// (ordenada por tag, dedup "later wins" — paridade `normalized()`,
/// `variations.rs:196-208`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FontVariations(pub Vec<([u8; 4], f32)>);

impl FontVariations {
    /// `true` se não houver eixos.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Valida um `Value` vindo do eval (espera `Value::Dict`) e constrói
    /// a lista normalizada. Erros replicam o vanilla verbatim, com o hint
    /// final `occurred in tag at index {i} (`"{key}"`)` por entrada
    /// (`tag_hint_helper`, `variations.rs:233-235`).
    pub fn from_value(value: &Value, span: Span) -> Result<Self, Vec<SourceDiagnostic>> {
        let dict = match value {
            Value::Dict(d) => d,
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected dictionary, found {}",
                        vanilla_type_name(other.type_name())
                    ),
                )]);
            }
        };

        let mut entries: Vec<([u8; 4], f32)> = Vec::with_capacity(dict.len());
        for (i, (key, val)) in dict.iter().enumerate() {
            let tag_hint = format!("occurred in tag at index {} (`\"{}\"`)", i, key);
            let tag = match validate_axis_tag(key) {
                Ok(t) => t,
                Err((msg, mut hints)) => {
                    hints.push(tag_hint);
                    let mut diag = SourceDiagnostic::error(span, msg);
                    for h in hints {
                        diag = diag.with_hint(h);
                    }
                    return Err(vec![diag]);
                }
            };
            let value = match val {
                Value::Int(n) => *n as f32,
                Value::Float(f) => *f as f32,
                other => {
                    let diag = SourceDiagnostic::error(
                        span,
                        format!(
                            "expected float, found {}",
                            vanilla_type_name(other.type_name())
                        ),
                    )
                    .with_hint(tag_hint);
                    return Err(vec![diag]);
                }
            };
            entries.push((tag, value));
        }

        Ok(Self(normalize(entries)))
    }

    /// Constrói a partir de um dict já validado no eval (caminho do
    /// resolver da style chain — nunca falha; entradas inesperadas são
    /// ignoradas).
    pub fn from_validated_dict(dict: &IndexMap<EcoString, Value, FxBuildHasher>) -> Self {
        let mut entries: Vec<([u8; 4], f32)> = Vec::with_capacity(dict.len());
        for (key, val) in dict.iter() {
            let value = match val {
                Value::Int(n) => *n as f32,
                Value::Float(f) => *f as f32,
                Value::None
                | Value::Bool(_)
                | Value::Str(_)
                | Value::Array(_)
                | Value::Dict(_)
                | Value::Module(_)
                | Value::Datetime(_)
                | Value::Func(_)
                | Value::Content(_)
                | Value::LocatedContent(_, _)
                | Value::Auto
                | Value::Length(_)
                | Value::Relative(_)
                | Value::Ratio(_)
                | Value::Angle(_)
                | Value::Color(_)
                | Value::Stroke(_)
                | Value::Fraction(_)
                | Value::Align(_)
                | Value::Location(_)
                | Value::Gradient(_)
                | Value::Regex(_)
                | Value::Tiling(_)
                | Value::Bytes(_)
                | Value::Decimal(_)
                | Value::Duration(_)
                | Value::Version(_)
                | Value::Selector(_)
                | Value::Symbol(_)
                | Value::Args(_)
                | Value::State(_)
                | Value::Counter(_)
                | Value::Label(_)
                | Value::Dir(_)
                | Value::Path(_)
                | Value::Type(_)
                | Value::SpotColorant(_) => continue,
            };
            if let Ok(tag) = validate_axis_tag(key) {
                entries.push((tag, value));
            }
        }
        Self(normalize(entries))
    }

    /// Fold da style chain (paridade `Fold for FontVariations`,
    /// `variations.rs:210-214`): `self` (interno) vence por tag; tags de
    /// `outer` ausentes em `self` sobrevivem.
    pub fn fold(&self, outer: &FontVariations) -> FontVariations {
        let mut entries = outer.0.clone();
        entries.extend(self.0.iter().copied());
        FontVariations(normalize(entries))
    }
}

/// Ordena por tag e remove duplicados "later wins" (a última ocorrência
/// vence — paridade `rdedup_by_key` após sort estável).
fn normalize(mut entries: Vec<([u8; 4], f32)>) -> Vec<([u8; 4], f32)> {
    entries.sort_by_key(|(tag, _)| *tag);
    let mut out: Vec<([u8; 4], f32)> = Vec::with_capacity(entries.len());
    for (tag, value) in entries {
        if let Some(last) = out.last_mut() {
            if last.0 == tag {
                last.1 = value;
                continue;
            }
        }
        out.push((tag, value));
    }
    out
}

/// Valida uma tag de eixo — paridade do cast de `Tag`
/// (`tag.rs:85-117`). Devolve a tag `[u8; 4]` com padding de espaço,
/// ou `(mensagem, hints)` com os textos verbatim do vanilla.
///
/// Ordem das verificações (como no vanilla): ASCII imprimível →
/// comprimento → espaços só como padding final.
///
/// Divergência registada: o vanilla itera grapheme clusters
/// (unicode-segmentation); o cristalino itera `chars()` (a crate não
/// está na whitelist L1). Difere apenas no hint `found invalid
/// cluster` para sequências combinantes (ex. `e` + U+0301).
fn validate_axis_tag(s: &str) -> Result<[u8; 4], (String, Vec<String>)> {
    // 1. Apenas ASCII imprimível (0x20..=0x7E).
    if let Some(cluster) = s.chars().find(|c| !(*c as u32 >= 0x20 && *c as u32 <= 0x7E)) {
        return Err((
            "tag may contain only printable ASCII characters".to_string(),
            vec![format!("found invalid cluster `\"{}\"`", cluster)],
        ));
    }

    // 2. Comprimento 1..=4 (bytes == chars aqui: ASCII garantido acima).
    if !(1..=4).contains(&s.len()) {
        return Err((
            "tag must be one to four characters in length".to_string(),
            vec![format!("found {} characters", s.len())],
        ));
    }

    // 3. Espaços apenas como padding final.
    let mut within_padding = false;
    for (i, &b) in s.as_bytes().iter().enumerate() {
        if (within_padding && b != b' ') || (i == 0 && b == b' ') {
            return Err((
                "spaces may only appear as padding following a tag".to_string(),
                Vec::new(),
            ));
        }
        within_padding |= b == b' ';
    }

    // Paridade `Tag::from_bytes_lossy` — padding com espaço.
    let mut tag = [b' '; 4];
    tag[..s.len()].copy_from_slice(s.as_bytes());
    Ok(tag)
}

/// Nomes de tipo do vanilla nos erros de cast — o `type_name()`
/// cristalino usa formas curtas (`int`, `str`, `bool`).
fn vanilla_type_name(name: &str) -> &str {
    match name {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dict(entries: &[(&str, Value)]) -> Value {
        let mut d = IndexMap::with_hasher(FxBuildHasher);
        for (k, v) in entries {
            d.insert(EcoString::from(*k), v.clone());
        }
        Value::Dict(d)
    }

    #[test]
    fn p836_from_value_valido_normaliza_e_ordena() {
        let v = dict(&[("wght", Value::Int(250)), ("ital", Value::Float(1.0))]);
        let fv = FontVariations::from_value(&v, Span::detached()).unwrap();
        // Ordenado por tag: "ital" < "wght".
        assert_eq!(fv.0, vec![(*b"ital", 1.0), (*b"wght", 250.0)]);
    }

    #[test]
    fn p836_from_value_tag_curta_padding_espaco() {
        let v = dict(&[("w", Value::Int(2))]);
        let fv = FontVariations::from_value(&v, Span::detached()).unwrap();
        assert_eq!(fv.0, vec![([b'w', b' ', b' ', b' '], 2.0)]);
    }

    #[test]
    fn p836_tag5_erro_verbatim() {
        let v = dict(&[("wgght", Value::Int(1))]);
        let err = FontVariations::from_value(&v, Span::detached()).unwrap_err();
        assert_eq!(err[0].message, "tag must be one to four characters in length");
        assert_eq!(
            err[0].hints,
            vec![
                "found 5 characters".to_string(),
                "occurred in tag at index 0 (`\"wgght\"`)".to_string(),
            ]
        );
    }

    #[test]
    fn p836_tag_vazia_erro_verbatim() {
        let v = dict(&[("", Value::Int(1))]);
        let err = FontVariations::from_value(&v, Span::detached()).unwrap_err();
        assert_eq!(err[0].message, "tag must be one to four characters in length");
        assert_eq!(
            err[0].hints,
            vec![
                "found 0 characters".to_string(),
                "occurred in tag at index 0 (`\"\"`)".to_string(),
            ]
        );
    }

    #[test]
    fn p836_tag_nao_ascii_erro_verbatim() {
        let v = dict(&[("wg€t", Value::Int(1))]);
        let err = FontVariations::from_value(&v, Span::detached()).unwrap_err();
        assert_eq!(err[0].message, "tag may contain only printable ASCII characters");
        assert_eq!(
            err[0].hints,
            vec![
                "found invalid cluster `\"€\"`".to_string(),
                "occurred in tag at index 0 (`\"wg€t\"`)".to_string(),
            ]
        );
    }

    #[test]
    fn p836_tag_espaco_interior_erro_verbatim() {
        let v = dict(&[("w g", Value::Int(1))]);
        let err = FontVariations::from_value(&v, Span::detached()).unwrap_err();
        assert_eq!(err[0].message, "spaces may only appear as padding following a tag");
        assert_eq!(
            err[0].hints,
            vec!["occurred in tag at index 0 (`\"w g\"`)".to_string()]
        );
    }

    #[test]
    fn p836_tag_espaco_padding_final_aceite() {
        let v = dict(&[("w  ", Value::Int(3))]);
        let fv = FontVariations::from_value(&v, Span::detached()).unwrap();
        assert_eq!(fv.0, vec![([b'w', b' ', b' ', b' '], 3.0)]);
    }

    #[test]
    fn p836_valor_string_erro_verbatim() {
        let v = dict(&[("wght", Value::Str(EcoString::from("bold")))]);
        let err = FontVariations::from_value(&v, Span::detached()).unwrap_err();
        assert_eq!(err[0].message, "expected float, found string");
        assert_eq!(
            err[0].hints,
            vec!["occurred in tag at index 0 (`\"wght\"`)".to_string()]
        );
    }

    #[test]
    fn p836_nao_dict_erro_verbatim() {
        let err =
            FontVariations::from_value(&Value::Int(5), Span::detached()).unwrap_err();
        assert_eq!(err[0].message, "expected dictionary, found integer");
        assert!(err[0].hints.is_empty());
    }

    #[test]
    fn p836_indice_no_hint_segue_posicao_do_dict() {
        let v = dict(&[("wght", Value::Int(1)), ("xxxxx", Value::Int(2))]);
        let err = FontVariations::from_value(&v, Span::detached()).unwrap_err();
        assert_eq!(
            err[0].hints,
            vec![
                "found 5 characters".to_string(),
                "occurred in tag at index 1 (`\"xxxxx\"`)".to_string(),
            ]
        );
    }

    #[test]
    fn p836_fold_interno_vence_por_tag_externo_sobrevive() {
        let inner = FontVariations(vec![(*b"wght", 250.0)]);
        let outer = FontVariations(vec![(*b"ital", 1.0), (*b"wght", 700.0)]);
        let folded = inner.fold(&outer);
        assert_eq!(folded.0, vec![(*b"ital", 1.0), (*b"wght", 250.0)]);
    }

    #[test]
    fn p836_from_validated_dict_leniente() {
        let Value::Dict(d) = dict(&[("wght", Value::Int(300))]) else {
            panic!("dict esperado")
        };
        let fv = FontVariations::from_validated_dict(&d);
        assert_eq!(fv.0, vec![(*b"wght", 300.0)]);
    }

    #[test]
    fn validated_axes_omit_every_current_nonnumeric_category() {
        use crate::entities::args::Args;
        use crate::entities::bytes::Bytes;
        use crate::entities::color::{Color, SpotColorant};
        use crate::entities::content::Content;
        use crate::entities::counter::Counter;
        use crate::entities::decimal::Decimal;
        use crate::entities::dir::Dir;
        use crate::entities::duration::Duration;
        use crate::entities::func::{ClosureRepr, Func};
        use crate::entities::geometry::Stroke;
        use crate::entities::gradient::Gradient;
        use crate::entities::label::Label;
        use crate::entities::layout_types::{Align2D, Angle, Length, Ratio};
        use crate::entities::locator::Locator;
        use crate::entities::module::Module;
        use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
        use crate::entities::regex::Regex;
        use crate::entities::rel::Rel;
        use crate::entities::scope::Scope;
        use crate::entities::selector::Selector;
        use crate::entities::state::State;
        use crate::entities::symbol::Symbol;
        use crate::entities::syntax_kind::SyntaxKind;
        use crate::entities::syntax_node::SyntaxNode;
        use crate::entities::tiling::{Tiling, TilingBody};
        use crate::entities::value::{IntrospectedContent, Type};
        use crate::entities::version::Version;
        use crate::entities::world_types::Datetime;
        use std::sync::Arc;

        let location = Locator::new().next();
        let black = Color::rgb(0, 0, 0);
        let closure = Func::closure(ClosureRepr {
            name: None,
            params: vec![],
            sink_name: None,
            body: SyntaxNode::leaf(SyntaxKind::Int, "1"),
            captured: Arc::new(Scope::new()),
            capturer: crate::entities::scope::Capturer::Function,
        });
        let cases = [
            ("None", Value::None),
            ("Bool", Value::Bool(true)),
            ("Str", Value::Str("300".into())),
            ("Array", Value::Array(vec![Value::Int(300)])),
            ("Dict", dict(&[("wght", Value::Int(300))])),
            ("Module", Value::Module(Module::new("axes", Scope::new()))),
            ("Datetime", Value::Datetime(Datetime::new_date(2026, 10, 19).unwrap())),
            ("Func", Value::Func(closure)),
            ("Content", Value::Content(Content::Empty)),
            (
                "LocatedContent",
                Value::LocatedContent(
                    IntrospectedContent::new(Content::Empty, None),
                    location,
                ),
            ),
            ("Auto", Value::Auto),
            ("Length", Value::Length(Length::pt(300.0))),
            ("Relative", Value::Relative(Rel::from_percent(30.0))),
            ("Ratio", Value::Ratio(Ratio(0.3))),
            ("Angle", Value::Angle(Angle::rad(0.3))),
            ("Color", Value::Color(black.clone())),
            ("Stroke", Value::Stroke(Arc::new(Stroke::default()))),
            ("Fraction", Value::Fraction(300.0)),
            ("Align", Value::Align(Align2D::default())),
            ("Location", Value::Location(location)),
            ("Gradient", Value::Gradient(Gradient::linear(vec![], Angle::rad(0.0)))),
            ("Regex", Value::Regex(Regex::new("300").unwrap())),
            (
                "Tiling",
                Value::Tiling(Arc::new(Tiling::new(TilingBody::Color(black.clone())))),
            ),
            ("Bytes", Value::Bytes(Bytes::new(vec![3, 0, 0]))),
            ("Decimal", Value::Decimal(Decimal::from_i64(300))),
            ("Duration", Value::Duration(Duration::from_seconds(300))),
            ("Version", Value::Version(Arc::new(Version::new(3, 0, 0)))),
            ("Selector", Value::Selector(Selector::Location(location))),
            ("Symbol", Value::Symbol(Symbol::new("3", "three"))),
            ("Args", Value::Args(Args::positional(vec![Value::Int(300)]))),
            (
                "State",
                Value::State(State {
                    key: "axes".into(),
                    init: Box::new(Value::Int(300)),
                }),
            ),
            ("Counter", Value::Counter(Counter { key: "axes".into() })),
            ("Label", Value::Label(Label("axes".into()))),
            ("Dir", Value::Dir(Dir::LTR)),
            (
                "Path",
                Value::Path(RootedPath::new(
                    VirtualRoot::Project,
                    VirtualPath::new("/300").unwrap(),
                )),
            ),
            ("Type", Value::Type(Type::Int)),
            (
                "SpotColorant",
                Value::SpotColorant(SpotColorant {
                    name: Some("axes".into()),
                    fallback: black,
                }),
            ),
        ];
        assert_eq!(cases.len(), 37);
        for (category, value) in cases {
            let Value::Dict(input) = dict(&[
                ("wght", Value::Int(250)),
                ("wdth", value),
                ("ital", Value::Float(0.5)),
            ]) else {
                panic!("expected dictionary")
            };
            assert_eq!(
                FontVariations::from_validated_dict(&input).0,
                vec![(*b"ital", 0.5), (*b"wght", 250.0)],
                "omitted category: {category}",
            );
        }
    }

    #[test]
    fn validated_axes_skip_invalid_tags_and_preserve_normalized_precedence() {
        let Value::Dict(empty) = dict(&[]) else { panic!("expected dictionary") };
        assert!(FontVariations::from_validated_dict(&empty).is_empty());
        let Value::Dict(input) = dict(&[
            ("w", Value::Int(1)),
            ("w ", Value::Int(2)),
            ("w  ", Value::Str("ignored".into())),
            ("w   ", Value::Float(3.5)),
            ("", Value::Int(9)),
            ("abcde", Value::Int(9)),
            ("é", Value::Int(9)),
            ("a b", Value::Int(9)),
            (" abc", Value::Int(9)),
            ("ital", Value::Float(-2.0)),
        ]) else {
            panic!("expected dictionary")
        };
        let axes = FontVariations::from_validated_dict(&input);
        assert_eq!(axes.0, vec![(*b"ital", -2.0), (*b"w   ", 3.5)]);
        assert_eq!(
            axes.fold(&FontVariations(vec![(*b"wdth", 80.0), (*b"w   ", 99.0)])).0,
            vec![(*b"ital", -2.0), (*b"w   ", 3.5), (*b"wdth", 80.0)],
        );
        let Value::Dict(ignored_collision) =
            dict(&[("w", Value::Int(1)), ("w ", Value::Bool(false))])
        else {
            panic!("expected dictionary")
        };
        assert_eq!(
            FontVariations::from_validated_dict(&ignored_collision).0,
            vec![(*b"w   ", 1.0)]
        );
    }

    #[test]
    fn strict_axes_report_span_and_hint_while_validated_axes_omit() {
        let id = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap(),
        );
        let span = Span::from_number(id, 7).unwrap();
        let input = dict(&[("wght", Value::Int(250)), ("ital", Value::Bool(true))]);
        let errors = FontVariations::from_value(&input, span).unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].span, span);
        assert_eq!(errors[0].message, "expected float, found boolean");
        assert_eq!(
            errors[0].hints,
            vec!["occurred in tag at index 1 (`\"ital\"`)".to_string()]
        );
        let Value::Dict(input) = input else { panic!("expected dictionary") };
        assert_eq!(
            FontVariations::from_validated_dict(&input).0,
            vec![(*b"wght", 250.0)]
        );
    }
}
