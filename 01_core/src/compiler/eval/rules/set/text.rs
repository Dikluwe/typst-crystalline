//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules/set/text.md
//! @prompt-hash 58b0842a
//! @layer L1
//! @updated 2026-09-30
//!
//! Branch `#set text`.

use super::*;

pub(super) fn eval_set_text(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // **F-5b fatia 2 (P373, §3a.13/§3a.14)**: o render do `#set text` deixa de assar
    // num `StyleDelta` tipado capturado no node — passa a viajar na chain pelo canal
    // `custom` `"text.<campo>"` (Value canónico), **transparente à morfologia**
    // (`is_semantically_empty` ignora o custom → `#set text X == X`) e levado ao
    // layout pelo transporte aninhado (`eval_markup`). A validação/erro-hard de
    // `lang`/`font` permanece (paridade ADR-0033/0052/0053); só a **saída** muda
    // (typed → custom). O layout (merge arm) decodifica o Value de volta.
    let args = call_dispatch::eval_args(set.args(), scopes, ctx, engine)?;
    let mut remaining = args.clone();
    for property in ["size", "fill"] {
        if args.named.contains_key(property) {
            continue;
        }
        let index = remaining.items.iter().position(|value| match property {
            "size" => matches!(value, Value::Length(_)),
            "fill" => matches!(value, Value::Color(_)),
            _ => unreachable!(),
        });
        if let Some(index) = index {
            let value = remaining.remove_positional(index).unwrap();
            *engine.styles = engine.styles.push_custom(format!("text.{property}"), value);
        }
    }
    let occurrences = args.occurrence_sequence();
    let last_size = occurrences
        .iter()
        .rposition(|arg| arg.name.as_deref() == Some("size"));
    for (index, occurrence) in occurrences.into_iter().enumerate() {
        if let Some(name) = &occurrence.name {
            let key = name.as_str().to_owned();
            let val = occurrence.value;
            let value_span = if occurrence.value_span.is_detached() {
                args.span
            } else {
                occurrence.value_span
            };
            let name_span =
                if occurrence.span.is_detached() { args.span } else { occurrence.span };
            let source_expr = set.args().items().find_map(|arg| match arg {
                Arg::Named(named) => {
                    if named.name().as_str() == key && named.expr().span() == value_span {
                        Some(named.expr())
                    } else {
                        None
                    }
                }
                Arg::Pos(_) | Arg::Spread(_) => None,
            });
            match key.as_str() {
                // P665: `bold` e `italic` como argumentos nomeados de `#set text`
                // não existem no vanilla; o vanilla usa `weight: "bold"` e
                // `style: "italic"`. Rejeitar para manter a linguagem alinhada.
                "bold" | "italic" => {
                    return Err(vec![SourceDiagnostic::error(
                        name_span,
                        format!("unexpected argument: {}", key),
                    )]);
                }
                "size" => {
                    // P816 (achado #3c de P810): tipo errado deixa de ser
                    // ignorado em silêncio — erro hard `expected length,
                    // found {type}` (+ hint para Int), paridade vanilla
                    // (`foundations/cast.rs:325-343`).
                    let span = value_span;
                    match val {
                        Value::Length(l) => {
                            if last_size == Some(index) {
                                *engine.styles = engine
                                    .styles
                                    .push_custom("text.size", Value::Length(l));
                            }
                        }
                        other => {
                            return Err(vec![expected_length_error(&other, span)]);
                        }
                    }
                }
                "fill" => match val {
                    Value::Color(c) => {
                        *engine.styles =
                            engine.styles.push_custom("text.fill", Value::Color(c))
                    }
                    Value::Gradient(_) | Value::Tiling(_) => {
                        // Existing partial transport; Paint needs its own contract.
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            value_span,
                            format!(
                                "expected color, gradient, or tiling, found {}",
                                vanilla_type_name(&other)
                            ),
                        )])
                    }
                },
                "weight" => {
                    // Int direto ou nome simbólico (FontWeight::from_name) → u16
                    // canónico em Value::Int. P636: rejeitar tipos inválidos.
                    let span = value_span;
                    match &val {
                        Value::Int(n) => {
                            if let Ok(w) = u16::try_from(*n) {
                                *engine.styles = engine
                                    .styles
                                    .push_custom("text.weight", Value::Int(w as i64));
                            } else {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    "font weight must be between 100 and 900".to_string(),
                                )]);
                            }
                        }
                        Value::Str(s) => {
                            if let Some(fw) = FontWeight::from_name(s.as_str()) {
                                *engine.styles = engine.styles.push_custom(
                                    "text.weight",
                                    Value::Int(fw.to_number() as i64),
                                );
                            } else {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!("unknown font weight name: {s}"),
                                )]);
                            }
                        }
                        Value::None => {}
                        other => {
                            return Err(vec![type_mismatch(
                                "int or string",
                                other,
                                span,
                            )]);
                        }
                    }
                }
                "style" => {
                    // P665: `style` é a forma canónica do vanilla para itálico.
                    // Aceita os três valores standard; guarda em `text.style`.
                    let span = value_span;
                    if let Value::Str(s) = &val {
                        match s.as_str() {
                            "normal" | "italic" | "oblique" => {
                                *engine.styles = engine
                                    .styles
                                    .push_custom("text.style", Value::Str(s.clone()));
                            }
                            _ => {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!("unknown font style name: {s}"),
                                )]);
                            }
                        }
                    } else {
                        return Err(vec![type_mismatch("string", &val, span)]);
                    }
                }
                "variations" => {
                    // P836 (achado #21 de P831): campo `#[fold] #[ghost]`
                    // settable no vanilla (`text/mod.rs:846-850`). Validação
                    // partilhada com o constructor — mensagens/hints verbatim
                    // do vanilla (`variations.rs:217-236`, `tag.rs:85-117`).
                    let span = value_span;
                    crate::entities::font_variations::FontVariations::from_value(
                        &val, span,
                    )?;
                    *engine.styles = engine.styles.push_custom("text.variations", val);
                }
                "tracking" => {
                    // P816: mesma validação de tipo de `size` (Length no
                    // vanilla, `text/mod.rs:333`).
                    let span = value_span;
                    match val {
                        Value::Length(l) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("text.tracking", Value::Length(l));
                        }
                        other => {
                            return Err(vec![expected_length_error(&other, span)]);
                        }
                    }
                }
                "lang" => {
                    // ADR-0052: valida via `Lang::from_str` (erro hard em
                    // inválido); guarda o código canónico (`Value::Str`).
                    if let Value::Str(s) = val {
                        match Lang::from_str(&s) {
                            Ok(lang) => {
                                *engine.styles = engine.styles.push_custom(
                                    "text.lang",
                                    Value::Str(lang.as_str().into()),
                                );
                            }
                            Err(msg) => {
                                return Err(vec![SourceDiagnostic::error(
                                    value_span,
                                    msg.to_string(),
                                )]);
                            }
                        }
                    }
                }
                "font" => {
                    // P407 (DEBT-52): string/array/dict aceites. Inspeccionamos o
                    // AST do argumento porque `Value::Dict` tem keys `EcoString`, não
                    // suportando regex keys (que o vanilla exige para `text.font`).
                    // Persiste na chain custom como `Value::Array` de items:
                    //   - `Value::Str(name)` → literal, variants vazio (P292/P373).
                    //   - `Value::Dict` com "name" (Str|Regex) + "variants" (Array[Str])
                    //     → dict form com regex keys.
                    // Zero tipo novo em `Value`: reusa Array/Dict/Str/Regex.
                    let span = value_span;
                    let arr: Vec<Value> = if let Some(font_expr) = source_expr {
                        match font_expr {
                            Expr::Str(node) => {
                                vec![Value::Str(EcoString::from(node.get()?))]
                            }
                            Expr::Array(arr_node) => {
                                let mut items = Vec::new();
                                for item in arr_node.items() {
                                    if let ArrayItem::Pos(expr) = item {
                                        if let Expr::Str(node) = expr {
                                            items.push(Value::Str(EcoString::from(
                                                node.get()?,
                                            )));
                                        } else {
                                            return Err(vec![SourceDiagnostic::error(
                                                span,
                                                "font array must contain only strings"
                                                    .to_string(),
                                            )]);
                                        }
                                    }
                                }
                                if items.is_empty() {
                                    return Err(vec![SourceDiagnostic::error(
                                        span,
                                        "font array must not be empty".to_string(),
                                    )]);
                                }
                                items
                            }
                            Expr::Dict(dict_node) => {
                                // P414: detectar dict named fields (tem chave `family`)
                                // vs formato legado P407 (chaves são nomes de fonte).
                                const NAMED_FIELDS: &[&str] =
                                    &["family", "variant", "weight", "style", "fallback"];
                                let is_named_fields = dict_node.items().any(|item| {
                                matches!(
                                    item,
                                    DictItem::Named(named_item)
                                        if NAMED_FIELDS.contains(&named_item.name().as_str())
                                )
                            });

                                if is_named_fields {
                                    font_dict::parse_font_dict_named_fields(
                                        dict_node, span, scopes, ctx, engine,
                                    )?
                                } else {
                                    font_dict::parse_font_dict_legacy(
                                        dict_node, span, scopes, ctx, engine,
                                    )?
                                }
                            }
                            _ => {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    "font expects a string, array of strings, or dict"
                                        .to_string(),
                                )]);
                            }
                        }
                    } else {
                        match &val {
                            Value::Str(name) => vec![Value::Str(name.clone())],
                            Value::Array(items) => {
                                if items.is_empty()
                                    || !items
                                        .iter()
                                        .all(|item| matches!(item, Value::Str(_)))
                                {
                                    return Err(vec![SourceDiagnostic::error(
                                        span,
                                        "font expects a string, array of strings, or dict",
                                    )]);
                                }
                                items.clone()
                            }
                            _ => {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    "font expects a string, array of strings, or dict",
                                )])
                            }
                        }
                    };
                    // P816 (achado #3b de P810): warning `unknown font
                    // family` para nomes literais ausentes do FontBook —
                    // paridade vanilla (`check_font_list`,
                    // `text/mod.rs:1577-1588`, chamado no parse do arg
                    // `font` em `text/mod.rs:170-176`). Nomes em regex
                    // (dict `Value::Regex`) não são verificáveis — tal
                    // como no vanilla, que só avisa sobre `FontFamily`
                    // literais. Nome impresso lowercased (normalização
                    // de `FontFamily::new` do vanilla).
                    let missing_families: Vec<String> = {
                        let book = engine.world.book();
                        arr.iter()
                            .filter_map(|item| match item {
                                Value::Str(s) => Some(s.as_str()),
                                Value::Dict(d) => match d.get("name") {
                                    Some(name) => match name {
                                        Value::Str(s) => Some(s.as_str()),
                                        Value::None
                                        | Value::Bool(..)
                                        | Value::Int(..)
                                        | Value::Float(..)
                                        | Value::Array(..)
                                        | Value::Dict(..)
                                        | Value::Module(..)
                                        | Value::Datetime(..)
                                        | Value::Func(..)
                                        | Value::Content(..)
                                        | Value::LocatedContent(..)
                                        | Value::Auto
                                        | Value::Length(..)
                                        | Value::Relative(..)
                                        | Value::Ratio(..)
                                        | Value::Angle(..)
                                        | Value::Color(..)
                                        | Value::Stroke(..)
                                        | Value::Fraction(..)
                                        | Value::Align(..)
                                        | Value::Location(..)
                                        | Value::Gradient(..)
                                        | Value::Regex(..)
                                        | Value::Tiling(..)
                                        | Value::Bytes(..)
                                        | Value::Decimal(..)
                                        | Value::Duration(..)
                                        | Value::Version(..)
                                        | Value::Selector(..)
                                        | Value::Symbol(..)
                                        | Value::Args(..)
                                        | Value::State(..)
                                        | Value::Counter(..)
                                        | Value::Label(..)
                                        | Value::Dir(..)
                                        | Value::Path(..)
                                        | Value::Type(..)
                                        | Value::SpotColorant(..) => None,
                                    },
                                    None => None,
                                },
                                Value::None
                                | Value::Bool(..)
                                | Value::Int(..)
                                | Value::Float(..)
                                | Value::Array(..)
                                | Value::Module(..)
                                | Value::Datetime(..)
                                | Value::Func(..)
                                | Value::Content(..)
                                | Value::LocatedContent(..)
                                | Value::Auto
                                | Value::Length(..)
                                | Value::Relative(..)
                                | Value::Ratio(..)
                                | Value::Angle(..)
                                | Value::Color(..)
                                | Value::Stroke(..)
                                | Value::Fraction(..)
                                | Value::Align(..)
                                | Value::Location(..)
                                | Value::Gradient(..)
                                | Value::Regex(..)
                                | Value::Tiling(..)
                                | Value::Bytes(..)
                                | Value::Decimal(..)
                                | Value::Duration(..)
                                | Value::Version(..)
                                | Value::Selector(..)
                                | Value::Symbol(..)
                                | Value::Args(..)
                                | Value::State(..)
                                | Value::Counter(..)
                                | Value::Label(..)
                                | Value::Dir(..)
                                | Value::Path(..)
                                | Value::Type(..)
                                | Value::SpotColorant(..) => None,
                            })
                            .filter(|name| book.select_family(name).next().is_none())
                            .map(|name| name.to_lowercase())
                            .collect()
                    };
                    for family in missing_families {
                        engine.sink.warn_note(
                            span,
                            &format!("unknown font family: {family}"),
                            "",
                        );
                    }
                    *engine.styles =
                        engine.styles.push_custom("text.font", Value::Array(arr));
                }
                "top-edge" => {
                    // P837 (achados #22/#23 de P831) — paridade do cast
                    // `TopEdge` do vanilla (`text/mod.rs:1169-1177`):
                    // métrica do domínio enumerado ou `Length`; o resto é
                    // erro hard verbatim.
                    match val {
                        Value::Str(s) if TOP_EDGE_METRICS.contains(&s.as_str()) => {
                            *engine.styles =
                                engine.styles.push_custom("text.top-edge", Value::Str(s));
                        }
                        Value::Length(l) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("text.top-edge", Value::Length(l));
                        }
                        other => {
                            return Err(vec![edge_cast_error(true, &other, value_span)]);
                        }
                    }
                }
                "bottom-edge" => {
                    // P837 — idem, cast `BottomEdge` (`text/mod.rs:1217-1225`).
                    match val {
                        Value::Str(s) if BOTTOM_EDGE_METRICS.contains(&s.as_str()) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("text.bottom-edge", Value::Str(s));
                        }
                        Value::Length(l) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("text.bottom-edge", Value::Length(l));
                        }
                        other => {
                            return Err(vec![edge_cast_error(false, &other, value_span)]);
                        }
                    }
                }
                "costs" => {
                    let entries = crate::compiler::text_costs::parse(&val, value_span)?;
                    let pagination = entries.iter().any(|(key, _)| {
                        matches!(*key, "text.costs.widow" | "text.costs.orphan")
                    });
                    for (key, value) in entries {
                        *engine.styles = engine.styles.push_custom(key, value);
                    }
                    if pagination {
                        engine.sink.warn_note(value_span,
                        "text.costs: proteção de widow/orphan limitada a parágrafos preparados com métricas verticais uniformes",
                        "colunas, notas e conteúdo misto ainda não têm proteção regional certificada");
                    }
                }
                "overhang" => match val {
                    Value::Bool(_) => {
                        *engine.styles = engine.styles.push_custom("text.overhang", val);
                    }
                    other => {
                        return Err(vec![break_policy_error(
                            "expected boolean",
                            &other,
                            value_span,
                        )])
                    }
                },
                "hyphenate" => match val {
                    Value::Bool(_) | Value::Auto => {
                        *engine.styles = engine.styles.push_custom("text.hyphenate", val);
                    }
                    other => {
                        return Err(vec![break_policy_error(
                            "expected boolean or auto",
                            &other,
                            value_span,
                        )])
                    }
                },
                "dir" => {
                    if let Value::Dir(dir) = val {
                        if dir.is_vertical() {
                            return Err(vec![SourceDiagnostic::error(
                                value_span,
                                "text direction must be horizontal".to_string(),
                            )]);
                        }
                        *engine.styles =
                            engine.styles.push_custom("text.dir", Value::Dir(dir));
                    }
                }
                _ => {
                    // P816 (achado #3a de P810): nome fora da lista de
                    // propriedades settable do `TextElem` vanilla é erro
                    // hard `unexpected argument: {name}` (paridade
                    // `foundations/args.rs:262`, exit 1). Nomes válidos no
                    // vanilla mas ainda não capturados (ex.: `stroke`,
                    // `stroke`, `baseline`) mantêm o warning de scope-out
                    // do Passo 107 (encerra DEBT-49, hint ADR-0040).
                    if !VANILLA_TEXT_SET_PROPS.contains(&key.as_str()) {
                        return Err(vec![SourceDiagnostic::error(
                            name_span,
                            format!("unexpected argument: {}", key),
                        )]);
                    }
                    let (msg, hint) =
                        unsupported_property_warn("text", &key, Some("0040"));
                    engine.sink.warn_note(name_span, &msg, &hint);
                }
            }
        }
    }

    if let Some(arg) =
        remaining.occurrence_sequence().iter().find(|arg| arg.name.is_none())
    {
        return Err(vec![SourceDiagnostic::error(arg.span, "unexpected argument")]);
    }

    Ok(Value::None)
}

#[cfg(test)]
mod v16_c_structural_tests {

    use crate::contracts::world::World;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use crate::entities::{file_id::FileId, font_book::FontBook, source::Source};
    struct TestWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(
                    FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                    text.into(),
                )
                .expect("valid Source test fixture"),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
        fn include_source(&self, _: FileId, _: &str) -> Result<Source, String> {
            panic!("refused include accessed World")
        }
        fn include_path(
            &self,
            _: &crate::entities::path::RootedPath,
        ) -> Result<Source, String> {
            panic!("refused include accessed World")
        }
    }

    #[test]
    fn v16_c_font_warning_literal_families_and_invalid_inputs() {
        for text in [
            "#set text(font: \"MISSING FAMILY\")",
            "#set text(font: (\"MISSING FAMILY\",))",
            "#set text(font: (family: \"MISSING FAMILY\"))",
        ] {
            let world = TestWorld::new(text);
            let (result, sink) = crate::compiler::eval::tests::eval_for_test_keep_sink(
                &world,
                &world.source,
            );
            result.unwrap();
            let warnings = sink.into_diagnostics();
            assert_eq!(warnings.len(), 1, "{text}: {warnings:?}");
            assert_eq!(warnings[0].message, "unknown font family: missing family");
        }
        for text in ["#set text(font: true)", "#set text(font: (true,))"] {
            let world = TestWorld::new(text);
            let (result, sink) = crate::compiler::eval::tests::eval_for_test_keep_sink(
                &world,
                &world.source,
            );
            assert!(result.is_err());
            assert!(sink.into_diagnostics().is_empty());
        }
    }
    #[test]
    fn v16_c_font_regex_is_not_a_literal_missing_family() {
        let world = TestWorld::new("#set text(font: (family: regex(\"MISSING.*\")))");
        let (result, sink) =
            crate::compiler::eval::tests::eval_for_test_keep_sink(&world, &world.source);
        result.unwrap();
        assert!(sink.into_diagnostics().is_empty());
    }
}
