//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/set/text.md
//! @prompt-hash 12fb180f
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
    for arg in set.args().items() {
        if let Arg::Named(named) = arg {
            let key = named.name().as_str().to_owned();
            let val = eval_expr(named.expr(), scopes, ctx, engine)?;
            match key.as_str() {
                // P665: `bold` e `italic` como argumentos nomeados de `#set text`
                // não existem no vanilla; o vanilla usa `weight: "bold"` e
                // `style: "italic"`. Rejeitar para manter a linguagem alinhada.
                "bold" | "italic" => {
                    return Err(vec![SourceDiagnostic::error(
                        named.name().to_untyped().span(),
                        format!("unexpected argument: {}", key),
                    )]);
                }
                "size" => {
                    // P816 (achado #3c de P810): tipo errado deixa de ser
                    // ignorado em silêncio — erro hard `expected length,
                    // found {type}` (+ hint para Int), paridade vanilla
                    // (`foundations/cast.rs:325-343`).
                    let span = named.expr().span();
                    match val {
                        Value::Length(l) => {
                            *engine.styles =
                                engine.styles.push_custom("text.size", Value::Length(l));
                        }
                        other => {
                            return Err(vec![expected_length_error(&other, span)]);
                        }
                    }
                }
                "fill" => {
                    if let Value::Color(c) = val {
                        *engine.styles =
                            engine.styles.push_custom("text.fill", Value::Color(c));
                    }
                }
                "weight" => {
                    // Int direto ou nome simbólico (FontWeight::from_name) → u16
                    // canónico em Value::Int. P636: rejeitar tipos inválidos.
                    let span = named.expr().span();
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
                    let span = named.expr().span();
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
                    let span = named.expr().span();
                    crate::entities::font_variations::FontVariations::from_value(
                        &val, span,
                    )?;
                    *engine.styles = engine.styles.push_custom("text.variations", val);
                }
                "tracking" => {
                    // P816: mesma validação de tipo de `size` (Length no
                    // vanilla, `text/mod.rs:333`).
                    let span = named.expr().span();
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
                                    named.expr().span(),
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
                    let span = named.expr().span();
                    let font_expr = named.expr();
                    let arr: Vec<Value> = match font_expr {
                        Expr::Str(node) => vec![Value::Str(EcoString::from(node.get()?))],
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
                                    Some(Value::Str(s)) => Some(s.as_str()),
                                    _ => None,
                                },
                                _ => None, // neutro: N16[β] — valores não-nominais filtrados em extracção de famílias tipográficas
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
                            return Err(vec![edge_cast_error(
                                true,
                                &other,
                                named.expr().span(),
                            )]);
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
                            return Err(vec![edge_cast_error(
                                false,
                                &other,
                                named.expr().span(),
                            )]);
                        }
                    }
                }
                "costs" => {
                    let entries =
                        crate::compiler::text_costs::parse(&val, named.expr().span())?;
                    let pagination = entries.iter().any(|(key, _)| {
                        matches!(*key, "text.costs.widow" | "text.costs.orphan")
                    });
                    for (key, value) in entries {
                        *engine.styles = engine.styles.push_custom(key, value);
                    }
                    if pagination {
                        engine.sink.warn_note(named.expr().span(),
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
                            named.expr().span(),
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
                            named.expr().span(),
                        )])
                    }
                },
                "dir" => {
                    if let Value::Dir(dir) = val {
                        if dir.is_vertical() {
                            return Err(vec![SourceDiagnostic::error(
                                named.expr().span(),
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
                            named.name().to_untyped().span(),
                            format!("unexpected argument: {}", key),
                        )]);
                    }
                    let (msg, hint) =
                        unsupported_property_warn("text", &key, Some("0040"));
                    engine.sink.warn_note(named.name().to_untyped().span(), &msg, &hint);
                }
            }
        }
    }

    Ok(Value::None)
}
