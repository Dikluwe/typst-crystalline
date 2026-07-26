//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/eval.md
//! @prompt-hash 6e69a751
//! @layer L1
//! @updated 2026-07-22
//!
//! Show rules e set rules — aplicação e intercepção. Extraído de `eval.rs`
//! no Passo 96.1 conforme ADR-0037 (coesão por domínio).

use std::sync::Arc;

use std::str::FromStr;

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::engine::scopes::Scopes;
use crate::entities::args::Args;
use crate::entities::ast::code::{SetRule, ShowRule as ShowRuleNode};
use crate::entities::ast::expr::{Arg, ArrayItem, Dict, DictItem, Expr};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::engine::Engine;
use crate::entities::font_book::FontWeight;
use crate::entities::lang::Lang;
use crate::entities::selector::Selector as QuerySelector;
use crate::entities::show::{NodeKind, Selector, ShowRule, Transformation};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::{Style, Styles};
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use crate::entities::world_types::check_show_depth as route_check_show_depth;

use super::{closures, eval_expr, EvalContext};

/// P636 — mensagem de mismatch de tipo no formato do vanilla
/// (`foundations/cast.rs:325-335`): "expected {expected}, found {actual}".
pub(crate) fn type_mismatch(expected: &str, found: &Value, span: Span) -> SourceDiagnostic {
    SourceDiagnostic::error(
        span,
        format!("expected {}, found {}", expected, found.type_name()),
    )
}

/// **P816** (achado #3c de P810) — erro de cast para `Length` no formato
/// do vanilla (`foundations/cast.rs:325-343`): `expected length, found
/// {type}` com hint `a length needs a unit - did you mean {i}pt?` quando o
/// valor recebido é `Int` (hint medido no vanilla, `text(mod.rs)` via
/// `cast.rs:341-343`).
pub(crate) fn expected_length_error(found: &Value, span: Span) -> SourceDiagnostic {
    // Nome do tipo no vocabulário do vanilla (`foundations/cast.rs`,
    // `Value::ty`): o `type_name()` cristalino diverge em `int`/`str`/`bool`
    // (P636); o vanilla diz `integer`/`string`/`boolean` — medido em
    // `#set text(size: 12)` → "expected length, found integer".
    let found_name = match found.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    };
    let mut diag = SourceDiagnostic::error(span, format!("expected length, found {found_name}"));
    if let Value::Int(i) = found {
        diag = diag.with_hint(format!("a length needs a unit - did you mean {i}pt?"));
    }
    diag
}

/// **P837** (achado #22 de P831) — erro de cast para `top-edge`/
/// `bottom-edge` no formato verbatim do vanilla 0.15.0 (medido:
/// `error: expected "ascender", "cap-height", "x-height", "baseline",
/// "bounds", or length` para top; `expected "baseline", "descender",
/// "bounds", or length` para bottom; exit 1). String fora do domínio
/// enumerado falha o cast de `TopEdgeMetric`/`BottomEdgeMetric` sem
/// sufixo `found`; outros tipos levam `, found {type}` e, para `Int`,
/// o hint `a length needs a unit - did you mean {i}pt?` (mesmo hint de
/// `expected_length_error`, P816, `foundations/cast.rs:341-343`).
pub(crate) fn edge_cast_error(is_top: bool, found: &Value, span: Span) -> SourceDiagnostic {
    let expected = if is_top {
        "\"ascender\", \"cap-height\", \"x-height\", \"baseline\", \"bounds\", or length"
    } else {
        "\"baseline\", \"descender\", \"bounds\", or length"
    };
    if let Value::Str(_) = found {
        return SourceDiagnostic::error(span, format!("expected {expected}"));
    }
    // Nome do tipo no vocabulário do vanilla (mesmo mapeamento de
    // `expected_length_error`, P816).
    let found_name = match found.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    };
    let mut diag =
        SourceDiagnostic::error(span, format!("expected {expected}, found {found_name}"));
    if let Value::Int(i) = found {
        diag = diag.with_hint(format!("a length needs a unit - did you mean {i}pt?"));
    }
    diag
}

/// **P837** — domínios enumerados válidos de `top-edge`/`bottom-edge`
/// (cast de `TopEdgeMetric`/`BottomEdgeMetric`, vanilla
/// `text/mod.rs:1180-1248`). `"bounds"` é válido em ambos (resolvido no
/// vanilla via bbox do glyph; no cristalino cai no fallback defensivo de
/// `edge_offset_pt` — limitação conhecida, ver relatório P837).
const TOP_EDGE_METRICS: &[&str] =
    &["ascender", "cap-height", "x-height", "baseline", "bounds"];
const BOTTOM_EDGE_METRICS: &[&str] = &["baseline", "descender", "bounds"];

/// **P816** (achado #3a de P810) — propriedades nomeadas que o `TextElem`
/// do vanilla aceita em `#set text(...)` (campos settable de
/// `text/mod.rs:182-788`, vanilla 0.15.0, em kebab-case). Nomes fora
/// desta lista são **erro hard** `unexpected argument: {name}` (paridade
/// `foundations/args.rs:262`, exit 1); nomes dentro da lista mas ainda
/// não capturados pelo cristalino mantêm o warning de scope-out do
/// Passo 107 (ADR-0040).
pub(crate) const VANILLA_TEXT_SET_PROPS: &[&str] = &[
    "font",
    "fallback",
    "style",
    "weight",
    "stretch",
    "size",
    "fill",
    "stroke",
    "tracking",
    "spacing",
    "cjk-latin-spacing",
    "baseline",
    "overhang",
    "top-edge",
    "bottom-edge",
    "lang",
    "region",
    "script",
    "dir",
    "hyphenate",
    "costs",
    "kerning",
    "alternates",
    "stylistic-set",
    "ligatures",
    "discretionary-ligatures",
    "historical-ligatures",
    "number-type",
    "number-width",
    "slashed-zero",
    "fractions",
    "features",
    // P836 — `#[fold] #[ghost]` settable no vanilla (`text/mod.rs:846-850`).
    "variations",
];

/// **P417 (M)** — Converte um `entities::selector::Selector` (query)
/// para um `entities::show::Selector` (show rule). Apenas `Kind` e
/// `Where` sobre `Kind` de elementos nativos suportados são convertidos;
/// outros selectors são scope-out com erro claro.
fn query_selector_to_show_selector(
    sel: QuerySelector,
    span: Span,
) -> SourceResult<Selector> {
    fn kind_to_node(kind: ElementKind) -> Option<NodeKind> {
        match kind {
            ElementKind::Heading => Some(NodeKind::Heading),
            ElementKind::Figure => Some(NodeKind::Figure),
            ElementKind::Link => Some(NodeKind::Link),
            ElementKind::Raw => Some(NodeKind::Raw),
            ElementKind::Quote => Some(NodeKind::Quote),
            ElementKind::Footnote => Some(NodeKind::Footnote),
            ElementKind::List => Some(NodeKind::List),
            ElementKind::Enum => Some(NodeKind::Enum),
            _ => None,
        }
    }

    match sel {
        QuerySelector::Kind(kind) => match kind_to_node(kind) {
            Some(node) => Ok(Selector::NodeKind(node)),
            None => Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "selector de kind '{}' não é suportado em show rule",
                    kind.as_str()
                ),
            )]),
        },
        // P493c — Where encadeado: converte recursivamente a base.
        QuerySelector::Where { base, field, value } => {
            let base_sel = query_selector_to_show_selector(*base, span)?;
            if is_node_rule(&base_sel) {
                Ok(Selector::Where { base: Box::new(base_sel), field, value })
            } else {
                Err(vec![SourceDiagnostic::error(
                    span,
                    "selector where com base não-node não suportado em show rule"
                        .to_string(),
                )])
            }
        }
        // **P423 (S-M)** — combinadores And/Or convertidos recursivamente.
        QuerySelector::And(sels) => {
            let mut converted = Vec::with_capacity(sels.len());
            for s in sels.iter() {
                converted.push(query_selector_to_show_selector(s.clone(), span)?);
            }
            Ok(Selector::And(converted))
        }
        QuerySelector::Or(sels) => {
            let mut converted = Vec::with_capacity(sels.len());
            for s in sels.iter() {
                converted.push(query_selector_to_show_selector(s.clone(), span)?);
            }
            Ok(Selector::Or(converted))
        }
        _ => Err(vec![SourceDiagnostic::error(
            span,
            "selector não suportado em show rule".to_string(),
        )]),
    }
}

/// Helper partilhado para construir warning de propriedade não suportada
/// em `#set` (Passo 107, encerra DEBT-49). Formato consistente para o
/// utilizador final.
///
/// Passo 134: parâmetro `adr_ref` opcional — `Some("0040")` referencia
/// catálogo vivo (set text); `None` produz hint genérico (set par, até
/// ADR dedicada existir).
fn unsupported_property_warn(
    target: &str,
    field: &str,
    adr_ref: Option<&str>,
) -> (String, String) {
    let msg = format!("{target}: propriedade '{field}' ainda não suportada");
    let hint = match adr_ref {
        Some(adr) => format!("ver ADR-{adr} para propriedades cobertas por set {target}"),
        None => format!("propriedades de set {target} ainda não são capturadas"),
    };
    (msg, hint)
}

/// Helper para warning de `#set` com target desconhecido (Passo 107).
/// Lista actualizada em Passo 133 para incluir `par` (activado).
fn unsupported_target_warn(target: &str) -> (String, String) {
    (
        format!("set: target '{target}' ainda não suportado"),
        "targets suportados: heading, page, figure, text, par, table, smartquote"
            .to_string(),
    )
}

/// Casa um nó contra um selector de element rule (`NodeKind`/`DynKind`).
/// **Partilhado** (P352) pelo loop α (transformação func/content) e pela passagem
/// de show-set: ambos usam exatamente o mesmo critério de match. `Selector::Text`
/// nunca casa aqui (tratado por `map_text`).
fn is_styled_origin(
    work: &Content,
    bold_from_strong: bool,
    italic_from_emph: bool,
    subscript: bool,
    superscript: bool,
    highlight: bool,
) -> bool {
    match work {
        Content::Styled(_, styles) => {
            let d = styles.delta();
            let mut ok = false;
            if bold_from_strong {
                ok |= d.bold == Some(true) && d.bold_from_strong == Some(true);
            }
            if italic_from_emph {
                ok |= d.italic == Some(true) && d.italic_from_emph == Some(true);
            }
            if subscript {
                ok |= d.subscript == Some(true);
            }
            if superscript {
                ok |= d.superscript == Some(true);
            }
            if highlight {
                ok |= d.highlight.is_some();
            }
            ok
        }
        _ => false,
    }
}

fn selector_matches(work: &Content, selector: &Selector) -> bool {
    // F-5b fatia 1 (P371): `show strong/emph` casam as **variantes próprias**
    // `Content::Strong`/`Emph` (S1, por tipo). P431 (DEBT-50) acrescenta a
    // distinção de origem no `Style` para o cenário pós-bake-in (wrapping):
    // um `Content::Styled` com `Bold { from_strong: true }` casa `show strong`,
    // mas `Bold { from_strong: false }` (de `#set text(bold)`) não casa.
    match selector {
        Selector::NodeKind(kind) => {
            matches!(
                (work, kind),
                (Content::Heading(_), NodeKind::Heading)
                | (Content::Figure(_), NodeKind::Figure)
                | (Content::Raw { .. }, NodeKind::Raw)
                | (Content::Equation { .. }, NodeKind::Equation)
                | (Content::Par { .. }, NodeKind::Par)
                | (Content::ListItem(_), NodeKind::ListItem)
                | (Content::Strong(_), NodeKind::Strong)
                | (Content::Emph(_), NodeKind::Emph)
                | (Content::Underline(_), NodeKind::Underline)
                | (Content::Strike(_), NodeKind::Strike)
                | (Content::Overline(_), NodeKind::Overline)
                | (Content::SmallCaps { .. }, NodeKind::Smallcaps)
                // P494
                | (Content::Link(_), NodeKind::Link)
                | (Content::Quote(_), NodeKind::Quote)
                | (Content::Footnote(_), NodeKind::Footnote)
            ) || matches!(
                (work, kind),
                (_, NodeKind::List)
                    if matches!(work, Content::Sequence(seq) if seq.iter().all(|c| matches!(c, Content::ListItem(_))))
                    || matches!(work, Content::ListItem(_))
            ) || matches!(
                (work, kind),
                (_, NodeKind::Enum)
                    if matches!(work, Content::Sequence(seq) if seq.iter().all(|c| matches!(c, Content::EnumItem(_))))
                    || matches!(work, Content::EnumItem(_))
            ) || matches!(
                (work, kind),
                (_, NodeKind::Strong) if is_styled_origin(work, true, false, false, false, false)
            ) || matches!(
                (work, kind),
                (_, NodeKind::Emph) if is_styled_origin(work, false, true, false, false, false)
            ) || matches!(
                (work, kind),
                (_, NodeKind::Subscript) if is_styled_origin(work, false, false, true, false, false)
            ) || matches!(
                (work, kind),
                (_, NodeKind::Superscript) if is_styled_origin(work, false, false, false, true, false)
            ) || matches!(
                (work, kind),
                (_, NodeKind::Highlight) if is_styled_origin(work, false, false, false, false, true)
            )
        }
        Selector::DynKind(name) => {
            matches!(work, Content::Dynamic(e) if e.dyn_kind() == name)
        }
        Selector::Text(_) => false,
        Selector::Regex(_) => false,
        // P791 — regras de label NÃO casam na travessia principal: são
        // aplicadas por `intercept_labelled` no ponto de associação
        // retroactiva (evita dupla aplicação das outras regras e garante
        // `it` = corpo, não o wrapper).
        Selector::Label(_) => false,
        Selector::Where { base, field, value } => {
            selector_matches(work, base)
                && work
                    .get_field(field.as_str())
                    .map(|actual| values_eq_semantic(&actual, value.as_ref()))
                    .unwrap_or(false)
        }
        // **P423 (S-M)** — combinadores And/Or com curto-circuito.
        // And/Or vazios retornam `false` (Opção A fixada em P209C/P423).
        Selector::And(sels) => {
            !sels.is_empty() && sels.iter().all(|s| selector_matches(work, s))
        }
        Selector::Or(sels) => {
            !sels.is_empty() && sels.iter().any(|s| selector_matches(work, s))
        }
    }
}

/// **P417 (M)** — Igualdade semântica de `Value` para matching de `Where`.
/// Replica ADR-0025 (coerção Int↔Float em comparações) e ADR-0107
/// (paridade comportamental, não mecânica).
fn values_eq_semantic(actual: &Value, expected: &Value) -> bool {
    match (actual, expected) {
        (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
        (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
        (a, b) => a == b,
    }
}

/// **P417/P423** — Verifica se um selector de show rule deve viajar pela
/// travessia de nós (`map_content`). Recursivo para `Where` com base
/// `NodeKind`/`DynKind`; P423 estende a `And`/`Or` (todos os sub-selectors
/// devem ser node-like).
fn is_node_rule(selector: &Selector) -> bool {
    match selector {
        Selector::NodeKind(_) | Selector::DynKind(_) => true,
        Selector::Where { base, .. } => is_node_rule(base.as_ref()),
        // **P423 (S-M)** — combinadores viajam pela travessia de nós sse
        // todos os sub-selectors forem node-like.
        Selector::And(sels) | Selector::Or(sels) => sels.iter().all(|s| is_node_rule(s)),
        Selector::Text(_) | Selector::Regex(_) => false,
        // P791 — regras de label não viajam pela travessia de nós
        // (aplicação dedicada em `intercept_labelled`).
        Selector::Label(_) => false,
    }
}

/// **P790** — Fatia `text` nas ocorrências de `pattern` e emenda o
/// replacement produzido para cada match (paridade vanilla
/// `visit_regex_match`, `typst-realize/src/lib.rs:1391`): o texto antes e
/// depois do match é preservado como `Content::Text` e o output da regra
/// entra no lugar do match. `Ok(None)` quando não há match (nó inalterado).
/// Fatias vazias são omitidas; `Content::sequence` colapsa o Vec se só
/// restar um nó (match de texto integral). Guarda defensiva: padrão vazio
/// nunca casa (o eval já rejeita com "text selector is empty").
fn splice_text_rule_matches(
    text: &str,
    pattern: &str,
    mut replacement: impl FnMut(&str) -> SourceResult<Content>,
) -> SourceResult<Option<Content>> {
    if pattern.is_empty() || !text.contains(pattern) {
        return Ok(None);
    }
    let mut parts: Vec<Content> = Vec::new();
    let mut rest = text;
    while let Some(idx) = rest.find(pattern) {
        let (before, with_match) = rest.split_at(idx);
        if !before.is_empty() {
            parts.push(Content::text(before));
        }
        let (matched, after) = with_match.split_at(pattern.len());
        parts.push(replacement(matched)?);
        rest = after;
    }
    if !rest.is_empty() {
        parts.push(Content::text(rest));
    }
    Ok(Some(Content::sequence(parts)))
}

/// **P863** — Realiza parágrafos antes de aplicar show rules.
///
/// Agrupa, em cada contexto de fluxo (`Sequence`), o conteúdo entre
/// `Content::Parbreak`s em nós `Content::Par`. Sub-árvores matemáticas
/// (`Equation`, `MathSequence` e família `Math*`) são preservadas sem wrapping.
pub(crate) fn realize_paragraphs(content: Content) -> Content {
    realize_node(&content)
}

/// Retorna os itens de um fluxo já realizados. `Sequence` é achatada e os
/// trechos entre `Parbreak` são embrulhados em `Content::Par`.
fn realize_flow(content: &Content) -> Vec<Content> {
    match content {
        Content::Sequence(seq) => {
            let mut out = Vec::new();
            let mut current = Vec::new();
            for child in seq.iter() {
                for piece in realize_flow(child) {
                    if matches!(piece, Content::Parbreak) {
                        if !current.is_empty() {
                            out.push(Content::par(Content::sequence(current)));
                            current = Vec::new();
                        }
                        out.push(piece);
                    } else {
                        current.push(piece);
                    }
                }
            }
            if !current.is_empty() {
                out.push(Content::par(Content::sequence(current)));
            }
            out
        }
        other => vec![realize_node(other)],
    }
}

macro_rules! realize_body {
    ($e:expr, $variant:path) => {{
        let mut elem = Arc::unwrap_or_clone(Arc::clone($e));
        elem.body = realize_node(&elem.body);
        $variant(Arc::new(elem))
    }};
}

fn realize_node(content: &Content) -> Content {
    match content {
        Content::Sequence(_) => Content::sequence(realize_flow(content)),
        Content::Par { body } => Content::par(realize_node(body)),
        Content::Styled(body, styles) => {
            Content::Styled(Box::new(realize_node(body)), styles.clone())
        }
        Content::Document {
            title,
            author,
            date,
            keywords,
        } => Content::Document {
            title: title.as_ref().map(|t| Box::new(realize_node(t))),
            author: author.clone(),
            date: *date,
            keywords: keywords.clone(),
        },

        // Containers com um único `body`.
        Content::Block(e) => realize_body!(e, Content::Block),
        Content::Boxed(e) => realize_body!(e, Content::Boxed),
        Content::Pad(e) => realize_body!(e, Content::Pad),
        Content::Align(e) => realize_body!(e, Content::Align),
        Content::Hide(e) => realize_body!(e, Content::Hide),
        Content::Transform(e) => realize_body!(e, Content::Transform),
        Content::Columns(e) => realize_body!(e, Content::Columns),
        Content::Repeat(e) => realize_body!(e, Content::Repeat),
        Content::ListItem(e) => realize_body!(e, Content::ListItem),
        Content::EnumItem(e) => realize_body!(e, Content::EnumItem),
        Content::Footnote(e) => realize_body!(e, Content::Footnote),
        Content::Label(e) => realize_body!(e, Content::Label),
        Content::Strong(e) => realize_body!(e, Content::Strong),
        Content::Emph(e) => realize_body!(e, Content::Emph),
        Content::Underline(e) => realize_body!(e, Content::Underline),
        Content::Strike(e) => realize_body!(e, Content::Strike),
        Content::Overline(e) => realize_body!(e, Content::Overline),
        Content::GridCell(e) => realize_body!(e, Content::GridCell),
        Content::GridHeader(e) => realize_body!(e, Content::GridHeader),
        Content::GridFooter(e) => realize_body!(e, Content::GridFooter),
        Content::TableCell(e) => realize_body!(e, Content::TableCell),
        Content::TableHeader(e) => realize_body!(e, Content::TableHeader),
        Content::TableFooter(e) => realize_body!(e, Content::TableFooter),
        Content::Title(e) => realize_body!(e, Content::Title),
        // `OutlineElem` não tem campo `body`; é folha para realização.
        Content::Outline(_) => content.clone(),
        Content::MathStyled(e) => realize_body!(e, Content::MathStyled),

        // Containers com body explícito inline.
        Content::SmallCaps { body } => Content::smallcaps(realize_node(body)),

        // Containers com múltiplos conteúdos.
        Content::Figure(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.body = realize_node(&e.body);
            e.caption = e.caption.as_ref().map(realize_node);
            Content::Figure(Arc::new(e))
        }
        Content::Quote(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.body = realize_node(&e.body);
            e.attribution = e.attribution.as_ref().map(realize_node);
            Content::Quote(Arc::new(e))
        }
        Content::TermItem(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.term = realize_node(&e.term);
            e.description = realize_node(&e.description);
            Content::TermItem(Arc::new(e))
        }
        Content::Grid(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.cells = e.cells.iter().map(realize_node).collect();
            e.header = e.header.as_ref().map(realize_node);
            e.footer = e.footer.as_ref().map(realize_node);
            Content::Grid(Arc::new(e))
        }
        Content::Table(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.children = e.children.iter().map(realize_node).collect();
            e.caption = e.caption.as_ref().map(realize_node);
            Content::Table(Arc::new(e))
        }
        Content::Stack(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.children = Arc::from(e.children.iter().map(realize_node).collect::<Vec<_>>());
            Content::Stack(Arc::new(e))
        }
        Content::Terms(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.items = e.items.iter().map(realize_node).collect();
            Content::Terms(Arc::new(e))
        }

        // Matemática e folhas: preservar sem wrapping.
        other => other.clone(),
    }
}

/// Aplica as show rules activas ao Content (Passo 70 — DEBT-23 encerrado).
///
/// NodeKind rules: única travessia `map_content` para todas as regras (O(N)).
/// Dentro da closure, itera o snapshot de regras e salta as que estão em
/// `active_guards` (anti-recursão por rule ID — DEBT-20 encerrado).
///
/// Text rules: aplicadas separadamente via `map_text` após a travessia principal.
pub(crate) fn apply_show_rules(
    mut content: Content,
    rules: &[ShowRule],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if rules.is_empty() {
        return Ok(content);
    }

    // P394: scope vazio para apply_func; closures de show-rule usam captured
    // scope como parent, e nativas comuns não usam scopes.
    let mut scopes = Scopes::new(None);

    // Limite de aninhamento vanilla (MAX_SHOW_RULE_DEPTH = 64) —
    // paridade com `typst-realize/src/lib.rs:402` (ADR-0033).
    // Pago parcial do DEBT-45 no Passo 93.
    route_check_show_depth(engine.route)?;

    // P863: se há regras sobre parágrafos, realizar os parágrafos antes de
    // aplicar as show rules. Realização é transparente para o layout.
    let has_par_rule = rules
        .iter()
        .any(|r| matches!(r.selector, Selector::NodeKind(NodeKind::Par)));
    if has_par_rule {
        content = realize_paragraphs(content);
    }

    // Separar regras por tipo para travessias distintas. Lote F-3 inc-2: as
    // regras de **kind dinâmico** (`#show callout:`) viajam pela MESMA travessia
    // que as NodeKind — mesmo `apply_all`, mesma ordem, mesmo guard por `RuleId`.
    let has_node_rules = rules.iter().any(|r| is_node_rule(&r.selector));

    if has_node_rules {
        // Única travessia para todas as NodeKind + DynKind rules, incluindo
        // Where com base node-like (P417).
        let node_rules: Vec<ShowRule> =
            rules.iter().filter(|r| is_node_rule(&r.selector)).cloned().collect();

        let mut apply_all = |node: &Content| -> SourceResult<Option<Content>> {
            // P348 (modelo α, ADR-0107): a element rule cujo output **re-casa** é
            // **revisitada** até **ponto-fixo morfológico** — o loop re-alimenta o
            // output no mesmo conjunto de regras e para quando a regra é um **no-op
            // morfológico** (`morph_canon(out) == morph_canon(work)`, o `==` do P345).
            // Recursão **não-convergente** (ciclo / divergente) é cortada pelo **teto**
            // backstop (`MAX_SHOW_RULE_DEPTH`, mecânica) e vira erro com a mensagem base
            // do vanilla. O `active_guards` continua a impedir a recursão **durante** a
            // chamada do recipe (criação aninhada); a revisitação é este loop, após o
            // recipe devolver. Divergência consciente vs vanilla (P347d/ADR-0107):
            // `#show heading: it => [= Z]` **converge para "Z"** (ponto-fixo) onde o
            // vanilla erra — o vanilla termina por identidade de instância (mecânica,
            // GEROU em P347b/c), o cristalino por morfologia.
            let mut work = node.clone();
            let mut applied = 0usize;
            // P350c — flag de erro completo: sob a flag, manter o histórico de
            // morfologias do caminho para classificar o erro (cíclico vs
            // não-convergente). Com a flag DESLIGADA (default), `history` fica vazio
            // e nada é alocado/computado — caminho quente intacto. `full_error` é Copy
            // (lido uma vez; `ctx` continua livre para `apply_func`).
            let full_error = ctx.full_error;
            let mut history: Vec<Content> =
                if full_error { vec![work.morph_canon()] } else { Vec::new() };
            let mut cycle = false;
            loop {
                // Aplicar a primeira regra func que casa `work`, uma vez, em ordem
                // **innermost-first** (última-declarada primeiro — P358): no
                // subconjunto onde uma func é efetiva, a última-declarada vence,
                // casando o vanilla (`styles.rs:835` `next_back`). É reordenação, não
                // acumulação (o vanilla não acumula func same-kind — P357). O fold de
                // show-set (abaixo) NÃO é invertido (mantém last-declared-overrides).
                let mut produced: Option<Content> = None;
                for rule in node_rules.iter().rev() {
                    // Saltar se esta regra está em execução (anti-recursão na criação
                    // aninhada — Lote F-3 inc-2; guard por `RuleId`, vale p/ DynKind).
                    if engine.active_guards.contains(&rule.id) {
                        continue;
                    }

                    // Show-set (P352): NÃO consome o passe de func — é tratado após o
                    // loop α (embrulha o nó em `Content::Styled`). Saltado aqui.
                    if matches!(rule.transform, Transformation::Style(_)) {
                        continue;
                    }

                    if !selector_matches(&work, &rule.selector) {
                        continue;
                    }

                    match &rule.transform {
                        Transformation::Func(func) => {
                            let args =
                                Args::positional(vec![Value::Content(work.clone())]);
                            engine.active_guards.push(rule.id);
                            let call_result = closures::apply_func(
                                func.clone(),
                                args,
                                &mut scopes,
                                ctx,
                                engine,
                            );
                            engine.active_guards.pop();
                            produced = Some(match call_result? {
                                Value::Content(c) => c,
                                Value::Str(s) => Content::text(s.as_str()),
                                other => {
                                    return Err(vec![SourceDiagnostic::error(
                                        Span::detached(),
                                        format!(
                                            "show rule deve retornar Content ou String, \
                                         recebeu {}",
                                            other.type_name()
                                        ),
                                    )])
                                }
                            });
                            break;
                        }
                        Transformation::Content(c) => {
                            produced = Some(c.clone());
                            break;
                        }
                        // `Str` só é válida sobre `Selector::Text` (tratada no loop
                        // de texto). Sobre NodeKind/DynKind é erro — paridade com o
                        // comportamento anterior ("recebeu str").
                        Transformation::Str(_) => {
                            return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            "show rule com selector de tipo requer função ou Content, \
                             recebeu str".to_string(),
                        )])
                        }
                        // Saltado acima; inalcançável.
                        Transformation::Style(_) => continue,
                    }
                }

                let Some(out) = produced else { break };
                applied += 1;

                // P350c (só sob a flag): registrar a morfologia do output e detectar
                // se uma forma do caminho **repetiu** (ciclo — fato medido pelo `==`
                // do P345 sobre `morph_canon`, sem alterá-los). Não corta cedo: a
                // terminação continua no teto (timing idêntico ao flag-off); só o hint
                // muda. Com a flag off, este bloco não corre.
                if full_error {
                    let out_canon = out.morph_canon();
                    if history.iter().any(|h| *h == out_canon) {
                        cycle = true;
                    }
                    history.push(out_canon);
                }

                // A partir da 2ª aplicação, revisitamos um output: detectar o
                // ponto-fixo (no-op morfológico) e cortar runaway. O caminho comum —
                // uma aplicação cujo output **não** re-casa — NÃO paga `morph_canon`:
                // a 2ª iteração apenas falha o match e sai (M-trigger, P348).
                if applied >= 2 {
                    if out.morph_canon() == work.morph_canon() {
                        work = out;
                        break; // ponto-fixo morfológico
                    }
                    if applied >= crate::entities::world_types::Route::MAX_SHOW_RULE_DEPTH
                    {
                        // Teto backstop (mecânica). Mensagem base + 2 hints
                        // BYTE-IDÊNTICOS ao vanilla (`engine.rs:350`, ADR-0033: a
                        // mensagem é comportamento observável).
                        let mut diag = SourceDiagnostic::error(
                            Span::detached(),
                            "maximum show rule depth exceeded",
                        )
                        .with_hint("maybe a show rule matches its own output")
                        .with_hint("maybe there are too deeply nested elements");
                        // P350c: sob a flag, 3º hint com a classificação — DOIS rótulos
                        // sólidos: **cíclico** (uma morfologia do caminho repetiu — fato)
                        // ou **não-convergente** (teto sem repetição). NÃO há terceiro
                        // rótulo ("converge-fundo") — distinguir divergente de
                        // converge-fundo adivinharia o futuro pós-corte (ADR-0108:
                        // afirmar só o medido). Sem a flag, a mensagem é byte-idêntica.
                        if full_error {
                            diag = diag.with_hint(if cycle {
                                "erro completo: recursão CÍCLICA — uma forma de conteúdo \
                                 repetiu-se no caminho de revisitação (a regra de #show \
                                 reescreve para algo que reaparece)"
                            } else {
                                "erro completo: recursão NÃO-CONVERGENTE — passou do limite \
                                 sem repetir nem estabilizar (verifique se a regra termina, \
                                 ou se é recursão legítima profunda)"
                            });
                        }
                        return Err(vec![diag]);
                    }
                }
                work = out;
            }

            // Show-set (P352/P356, S5): embrulha o output no `Styles` das regras
            // **show-set** que casam o **ELEMENTO** — o nó **original** que entrou na
            // realização (`node`), NÃO o `work` pós-func. **NÃO consome o passe** —
            // espelha `map.apply(transform); continue` do vanilla
            // (`typst-realize/src/lib.rs:458-464` / `styles.rs:504`).
            //
            // **Ordem show-set-vs-func (P356, paridade `lib.rs:341,357`).** O vanilla
            // dobra a show-set na chain (`map`) e aplica a func **sob** `chained =
            // styles.chain(&map)`. Casar contra `node` (o elemento) e embrulhar o
            // `work` (o output da func) reproduz isso: a show-set fica ativa quando a
            // func realiza. Casar contra `work` (pós-func) — o bug que isto conserta —
            // perdia a show-set (o output da func não casa o seletor do elemento).
            // Sem func, `work == node` → idêntico ao P352 (show-set-só e múltiplos
            // show-set por `collapse`). O confinamento vem do `Content::Styled`
            // (fatia 1, `f_fronteira_e1.md §3a.8`). `collapse` resolve top-wins.
            let mut set_chain = StyleChain::empty();
            let mut any_set = false;
            for rule in &node_rules {
                if let Transformation::Style(styles) = &rule.transform {
                    if engine.active_guards.contains(&rule.id) {
                        continue;
                    }
                    if selector_matches(node, &rule.selector) {
                        set_chain = set_chain.push(styles.delta().clone());
                        any_set = true;
                    }
                }
            }
            let mut wrapped = false;
            if any_set {
                let delta = set_chain.collapse();
                if !delta.is_empty() {
                    work = Content::Styled(Box::new(work), Styles::from_delta(delta));
                    wrapped = true;
                }
            }

            if applied > 0 || wrapped {
                Ok(Some(work))
            } else {
                Ok(None)
            }
        };

        content = content.map_content(&mut apply_all)?;
    }

    // Regex rules (P393) — aplicadas a nós de texto que casam.
    // Última-declarada vence (consistente com NodeKind). Transformações
    // Func/Content/Str são suportadas; Style foi rejeitado em parse.
    let regex_rules: Vec<ShowRule> = rules
        .iter()
        .filter(|r| matches!(r.selector, Selector::Regex(_)))
        .cloned()
        .collect();

    if !regex_rules.is_empty() {
        let mut apply_regex = |node: &Content| -> SourceResult<Option<Content>> {
            let Content::Text(text) = node else {
                return Ok(None);
            };
            for rule in regex_rules.iter().rev() {
                if engine.active_guards.contains(&rule.id) {
                    continue;
                }
                let Selector::Regex(re) = &rule.selector else { continue };
                if !re.is_match(text.as_str()) {
                    continue;
                }
                let produced = match &rule.transform {
                    Transformation::Func(func) => {
                        let args = Args::positional(vec![Value::Content(node.clone())]);
                        engine.active_guards.push(rule.id);
                        let call_result = closures::apply_func(
                            func.clone(),
                            args,
                            &mut scopes,
                            ctx,
                            engine,
                        );
                        engine.active_guards.pop();
                        match call_result? {
                            Value::Content(c) => c,
                            Value::Str(s) => Content::text(s.as_str()),
                            other => {
                                return Err(vec![SourceDiagnostic::error(
                                    Span::detached(),
                                    format!(
                                    "show rule regex deve retornar Content ou String, \
                                     recebeu {}",
                                    other.type_name()
                                ),
                                )])
                            }
                        }
                    }
                    Transformation::Content(c) => c.clone(),
                    Transformation::Str(s) => Content::text(s.as_str()),
                    Transformation::Style(_) => continue,
                };
                return Ok(Some(produced));
            }
            Ok(None)
        };
        content = content.map_content(&mut apply_regex)?;
    }

    // Text rules — P790: `Str` via `map_text` (comportamento pré-existente);
    // `Content`/`Func` por splice em `map_content` — o `Content::Text` é
    // fatiado nas ocorrências do padrão e o replacement é emendado entre as
    // fatias (paridade vanilla `visit_regex_match`; `map_content` não
    // reentra no nó substituído, logo o output não é re-varrido pela mesma
    // regra — equivalente à `Revocation` do vanilla). Match por nó de texto
    // individual (cross-node: scope-out registado no L0, eval.md §P790).
    for rule in rules {
        let Selector::Text(pattern) = &rule.selector else { continue };
        match &rule.transform {
            Transformation::Str(s) => {
                let replacement = s.to_string();
                let mut do_replace =
                    |text: &str| text.replace(pattern.as_str(), &replacement);
                content = content.map_text(&mut do_replace);
            }
            Transformation::Content(replacement) => {
                let mut apply_text = |node: &Content| -> SourceResult<Option<Content>> {
                    let Content::Text(text) = node else { return Ok(None) };
                    splice_text_rule_matches(text.as_str(), pattern, |_| {
                        Ok(replacement.clone())
                    })
                };
                content = content.map_content(&mut apply_text)?;
            }
            Transformation::Func(func) => {
                let mut apply_text = |node: &Content| -> SourceResult<Option<Content>> {
                    let Content::Text(text) = node else { return Ok(None) };
                    splice_text_rule_matches(text.as_str(), pattern, |matched| {
                        let args = Args::positional(vec![Value::Content(Content::text(
                            matched,
                        ))]);
                        engine.active_guards.push(rule.id);
                        let call_result = closures::apply_func(
                            func.clone(),
                            args,
                            &mut scopes,
                            ctx,
                            engine,
                        );
                        engine.active_guards.pop();
                        match call_result? {
                            Value::Content(c) => Ok(c),
                            Value::Str(s) => Ok(Content::text(s.as_str())),
                            other => Err(vec![SourceDiagnostic::error(
                                Span::detached(),
                                format!(
                                    "show rule deve retornar Content ou String, \
                                     recebeu {}",
                                    other.type_name()
                                ),
                            )]),
                        }
                    })
                };
                content = content.map_content(&mut apply_text)?;
            }
            Transformation::Style(_) => {
                // Rejeitado em `eval_show_rule` para `Selector::Text` —
                // inalcançável (show-set é sobre elementos).
            }
        }
    }

    Ok(content)
}

/// Aplica show rules ao Content produzido por eval (Passo 70 — DEBT-20 encerrado).
///
/// Anti-recursão via `active_guards` (stack de RuleId) em vez de booleano global.
/// Permite composição entre regras distintas; snapshot explícito evita borrow
/// conflict durante a travessia (DEBT-22).
pub(crate) fn intercept_content(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    // P498 — passagem de captura do conteúdo original: não aplicar show-rules,
    // apenas registá-las (feito em `eval_show_rule`). Isto preserva os elementos
    // locatable para a introspecção, espelhando o modelo vanilla.
    if !ctx.apply_show_rules {
        return Ok(content);
    }

    if engine.show_rules.is_empty() {
        return Ok(content);
    }

    // Passo 84.4 (encerra DEBT-22): snapshot Arc::clone — O(1) refcount.
    // O slice partilhado permite iterar sobre uma cópia estável enquanto
    // `engine.show_rules` pode ser reatribuído por nested `#show` rules
    // sem interferência durante a travessia.
    let rules = Arc::clone(&*engine.show_rules);
    apply_show_rules(content, &rules, ctx, engine)
}

/// **P863** — Intercepção específica de parágrafos no fluxo montado pelo markup.
///
/// Parágrafos são sintetizados a partir de vários nós de texto/`Space`/`Parbreak`,
/// pelo que não têm um único ponto de construção onde `intercept_content` possa
/// atuar. Esta função realiza os parágrafos e aplica **apenas** as regras
/// `NodeKind::Par`, deixando as regras de texto (já aplicadas eager em cada nó
/// de texto) e as restantes regras de elemento (já aplicadas nos seus pontos de
/// construção) intactas.
pub(crate) fn intercept_paragraphs(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !ctx.apply_show_rules {
        return Ok(content);
    }

    let par_rules: Vec<ShowRule> = engine
        .show_rules
        .iter()
        .filter(|r| matches!(r.selector, Selector::NodeKind(NodeKind::Par)))
        .cloned()
        .collect();

    if par_rules.is_empty() {
        return Ok(content);
    }

    apply_show_rules(content, &par_rules, ctx, engine)
}

/// **P791** — Intercepção de wrappers `Content::Label` para regras
/// `Selector::Label` (`#show <sp>: …`). Chamada no ponto de associação
/// retroactiva de `<label>` em markup (Passo 56, `eval/mod.rs`): o wrapper é
/// criado **depois** do corpo já ter viajado pela maquinaria geral de show
/// rules, logo aqui só regras de label casam — as outras já aplicaram nos
/// seus pontos próprios (sem dupla aplicação).
///
/// Semântica (paridade vanilla, `target.label()` — o label é metadado do
/// elemento, não um nó): `it` = o **corpo** rotulado; a saída substitui o
/// wrapper inteiro (label consumido). Aplicação **única**,
/// última-declarada primeiro (innermost-first, P358) — sem revisitação
/// (equivalente à `Revocation` do vanilla). `Content` substitui; `Str` é
/// erro (consistente com `NodeKind`); show-set (`Style`) **não consome o
/// passe** — as que casam são dobradas e embrulham o wrapper em
/// `Content::Styled` (paridade P352).
pub(crate) fn intercept_labelled(
    labelled: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !ctx.apply_show_rules || engine.show_rules.is_empty() {
        return Ok(labelled);
    }
    let Content::Label(e) = &labelled else {
        return Ok(labelled);
    };
    // Snapshot (Passo 84.4): slice estável enquanto closures podem mutar
    // `engine.show_rules`.
    let rules = Arc::clone(&*engine.show_rules);
    let mut scopes = Scopes::new(None);

    // Show-set: dobrar todas as regras de label que casam (ordem de
    // declaração, como no loop α) e embrulhar uma vez (collapse = top-wins).
    let mut set_chain = StyleChain::empty();
    let mut any_set = false;
    for rule in rules.iter() {
        if engine.active_guards.contains(&rule.id) {
            continue;
        }
        if let (Selector::Label(l), Transformation::Style(styles)) =
            (&rule.selector, &rule.transform)
        {
            if e.name.as_str() == l.0.as_str() {
                set_chain = set_chain.push(styles.delta().clone());
                any_set = true;
            }
        }
    }

    // Func/Content: primeira regra que casa (última-declarada primeiro),
    // uma única aplicação.
    for rule in rules.iter().rev() {
        if engine.active_guards.contains(&rule.id) {
            continue;
        }
        let Selector::Label(l) = &rule.selector else { continue };
        if e.name.as_str() != l.0.as_str() {
            continue;
        }
        let produced: Option<Content> = match &rule.transform {
            Transformation::Func(func) => {
                let args = Args::positional(vec![Value::Content(e.body.clone())]);
                engine.active_guards.push(rule.id);
                let call_result =
                    closures::apply_func(func.clone(), args, &mut scopes, ctx, engine);
                engine.active_guards.pop();
                Some(match call_result? {
                    Value::Content(c) => c,
                    Value::Str(s) => Content::text(s.as_str()),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "show rule deve retornar Content ou String, \
                                 recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                })
            }
            Transformation::Content(c) => Some(c.clone()),
            // `Str` sobre label é erro — paridade com o comportamento sobre
            // `NodeKind` ("requer função ou Content, recebeu str").
            Transformation::Str(_) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "show rule com selector de label requer função ou Content, \
                     recebeu str"
                        .to_string(),
                )])
            }
            // Show-set: tratada acima (não consome o passe).
            Transformation::Style(_) => None,
        };
        if let Some(out) = produced {
            return Ok(out);
        }
    }

    if any_set {
        let delta = set_chain.collapse();
        if !delta.is_empty() {
            return Ok(Content::Styled(Box::new(labelled), Styles::from_delta(delta)));
        }
    }
    Ok(labelled)
}

// ── Dispatcher arms: SetRule / ShowRule (Passo 96.2, ADR-0037 Regra 4) ────

pub(super) fn eval_set_rule(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // Extrair target — deve ser um Ident (ex: "text").
    // Targets suportados: heading, page, figure, text. Outros emitem
    // warning via Sink (Passo 107, encerra DEBT-49).
    let target = set.target().to_untyped().text_str().to_owned();
    let target_span = set.target().to_untyped().span();

    if target == "heading" {
        // #set heading(numbering: "1.1") — activa numeração automática.
        // Outros argumentos de heading ignorados por agora (DEBT-10).
        let mut active = false;
        let mut pattern: Option<ecow::EcoString> = None;
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                if named.name().as_str() == "numbering" {
                    // Defensivo: só String activa a numeração.
                    // Closures, none, ou outros tipos → ignorar.
                    let val = eval_expr(named.expr(), scopes, ctx, engine)
                        .unwrap_or(Value::None);
                    if let Value::Str(s) = val {
                        active = true;
                        pattern = Some(s.clone());
                    }
                }
            }
        }
        // Lote F-2 S1 (P335): em vez de um marcador global `SetHeadingNumbering`,
        // empurra para a chain léxica (`engine.styles` é escopado por
        // `local_styles`). O heading assa este valor na criação
        // (`eval/markup.rs`). Fecha o canal global → escopo de container (DEBT 99.E).
        // P451: transporta também o pattern string para formatação configurável.
        *engine.styles =
            engine.styles.push_custom("heading.numbering", Value::Bool(active));
        if let Some(pattern) = pattern {
            *engine.styles = engine
                .styles
                .push_custom("heading.numbering.pattern", Value::Str(pattern));
        }
        return Ok(Value::None);
    }

    // Lote F-2 S2 / **B1** (P335): `#set math.equation(numbering:)` — target
    // **pontuado** (`FieldAccess` `math.equation`), que `text_str()` não capta
    // (nó interno → ""). Era a lacuna do P331 (sem produtor eval). Empurra para
    // a chain léxica; a equação assa o valor (`eval/mod.rs`). Paridade vanilla:
    // `lab/.../math/equation.rs` — numeração de equação de bloco.
    // P456: guarda o pattern string (não Bool) para formatar via format_counter,
    // análogo a figure.numbering (P454) e heading.numbering.pattern (P451).
    if let Expr::FieldAccess(fa) = set.target() {
        if fa.target().to_untyped().text_str() == "math"
            && fa.field().to_untyped().text_str() == "equation"
        {
            let numbering = set.args().items().find_map(|arg| {
                if let Arg::Named(named) = arg {
                    if named.name().as_str() == "numbering" {
                        // P636: propagar erro de avaliação (variável indefinida,
                        // por exemplo) em vez de descartar com `.ok()`.
                        return Some((
                            named.expr().span(),
                            eval_expr(named.expr(), scopes, ctx, engine),
                        ));
                    }
                }
                None
            });
            match numbering {
                Some((_, Ok(Value::Str(s)))) => {
                    *engine.styles =
                        engine.styles.push_custom("equation.numbering", Value::Str(s));
                }
                Some((_, Ok(Value::None))) => {
                    // Limpa a numeração no escopo (None = ausente).
                    *engine.styles =
                        engine.styles.push_custom("equation.numbering", Value::None);
                }
                Some((span, Ok(other))) => {
                    return Err(vec![type_mismatch("string or none", &other, span)]);
                }
                Some((_, Err(err))) => return Err(err),
                None => {}
            }
            return Ok(Value::None);
        }
    }

    if target == "document" {
        // **P536** — `#set document(title: ..., author: ..., keywords: ...)`.
        // Extrai os valores, converte arrays para strings separadas por vírgula,
        // e acumula no `EvalContext`. O eval copia isto para o `Module` no final;
        // o pipeline transporta para o exportador PDF (`/Info`).
        fn value_to_eco_string(
            val: &crate::entities::value::Value,
            span: Span,
            allow_array: bool,
        ) -> SourceResult<Option<ecow::EcoString>> {
            match val {
                crate::entities::value::Value::Str(s) => Ok(Some(s.clone())),
                crate::entities::value::Value::Array(arr) if allow_array => {
                    let mut parts = Vec::new();
                    for v in arr.iter() {
                        match v {
                            crate::entities::value::Value::Str(s) => {
                                parts.push(s.as_str())
                            }
                            other => {
                                return Err(vec![type_mismatch("string", other, span)]);
                            }
                        }
                    }
                    if parts.is_empty() {
                        Ok(None)
                    } else {
                        Ok(Some(ecow::EcoString::from(parts.join(", "))))
                    }
                }
                crate::entities::value::Value::None => Ok(None),
                other => {
                    let expected =
                        if allow_array { "string or array of strings" } else { "string" };
                    Err(vec![type_mismatch(expected, other, span)])
                }
            }
        }

        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = named.name().as_str();
                let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                let span = named.expr().span();
                match key {
                    // P637: no vanilla, title é Option<Content> (não array);
                    // author/keywords são OneOrMultiple<EcoString>.
                    "title" => {
                        ctx.document_info.title = value_to_eco_string(&val, span, false)?
                    }
                    "author" => {
                        ctx.document_info.author = value_to_eco_string(&val, span, true)?
                    }
                    "keywords" => {
                        ctx.document_info.keywords =
                            value_to_eco_string(&val, span, true)?
                    }
                    _ => {}
                }
            }
        }
        return Ok(Value::None);
    }

    if target == "page" {
        // #set page(width: .., height: .., margin: ..) — Passo 81.
        // Valores ausentes ficam None e preservam o valor actual em layout.
        // P636: rejeitar tipos inválidos em vez de ignorar silenciosamente.
        // **P757** — `em` em dimensões de página deve resolver contra o
        // tamanho de fonte activo na chain (default 11 pt), não contra zero.
        // Anteriormente usava-se `l.abs.to_pt()`, que descartava a componente
        // `em` e produzia `0pt` silenciosamente para valores como `7em`.
        // **P867** — `width`/`height` aceitam `auto`; `margin` mantém-se
        // length/float/int/none neste passo.
        let size_pt = engine.styles.size();
        fn extract_page_dimension(
            val: &Value,
            span: Span,
            size_pt: f64,
        ) -> SourceResult<Option<crate::entities::layout_types::PageDimension>> {
            match val {
                Value::Length(l) => Ok(Some(
                    crate::entities::layout_types::PageDimension::Length(l.resolve_pt(size_pt)),
                )),
                Value::Float(f) => Ok(Some(crate::entities::layout_types::PageDimension::Length(*f))),
                Value::Int(i) => Ok(Some(crate::entities::layout_types::PageDimension::Length(*i as f64))),
                Value::Auto => Ok(Some(crate::entities::layout_types::PageDimension::Auto)),
                Value::None => Ok(None),
                other => Err(vec![type_mismatch("length, float, int, or auto", other, span)]),
            }
        }
        fn extract_pt(
            val: &Value,
            span: Span,
            size_pt: f64,
        ) -> SourceResult<Option<f64>> {
            match val {
                Value::Length(l) => Ok(Some(l.resolve_pt(size_pt))),
                Value::Float(f) => Ok(Some(*f)),
                Value::Int(i) => Ok(Some(*i as f64)),
                Value::None => Ok(None),
                other => Err(vec![type_mismatch("length, float, or int", other, span)]),
            }
        }
        let mut width: Option<crate::entities::layout_types::PageDimension> = None;
        let mut height: Option<crate::entities::layout_types::PageDimension> = None;
        let mut margin = None;
        let mut margin_left = None;
        let mut margin_right = None;
        let mut margin_top = None;
        let mut margin_bottom = None;
        let mut numbering = None;
        let mut columns: Option<usize> = None;
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = named.name().as_str();
                let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                let span = named.expr().span();
                match key {
                    "width" => width = extract_page_dimension(&val, span, size_pt)?,
                    "height" => height = extract_page_dimension(&val, span, size_pt)?,
                    "margin" => match &val {
                        Value::Dict(d) => {
                            let extract_field = |k: &str| -> SourceResult<Option<f64>> {
                                if let Some(v) = d.get(k) {
                                    extract_pt(v, span, size_pt)
                                } else {
                                    Ok(None)
                                }
                            };
                            let mx = extract_field("x")?;
                            let my = extract_field("y")?;
                            margin_left = extract_field("left")?.or(mx);
                            margin_right = extract_field("right")?.or(mx);
                            margin_top = extract_field("top")?.or(my);
                            margin_bottom = extract_field("bottom")?.or(my);
                            margin = margin_left.or(margin_top);
                        }
                        other => {
                            if let Some(m) = extract_pt(other, span, size_pt)? {
                                margin_left = Some(m);
                                margin_right = Some(m);
                                margin_top = Some(m);
                                margin_bottom = Some(m);
                                margin = Some(m);
                            }
                        }
                    },
                    "numbering" => {
                        numbering = match val {
                            Value::Str(s) => Some(s),
                            Value::None => Some(ecow::EcoString::new()),
                            other => {
                                return Err(vec![type_mismatch(
                                    "string or none",
                                    &other,
                                    span,
                                )]);
                            }
                        };
                    }
                    "columns" => {
                        columns = match val {
                            Value::Int(n) if n >= 1 => Some(n as usize),
                            Value::Int(n) if n < 1 => {
                                return Err(vec![SourceDiagnostic::error(
                                    named.span(),
                                    "columns must be at least 1".to_string(),
                                )]);
                            }
                            Value::None => None,
                            other => {
                                return Err(vec![type_mismatch("int", &other, span)]);
                            }
                        };
                    }
                    _ => {}
                }
            }
        }

        // Empurrar as dimensões da página para a StyleChain para que o layout() possa ler
        if let Some(w) = width {
            let v = match w {
                crate::entities::layout_types::PageDimension::Auto => Value::Auto,
                crate::entities::layout_types::PageDimension::Length(x) => Value::Float(x),
            };
            *engine.styles = engine.styles.push_custom("page.width", v);
        }
        if let Some(h) = height {
            let v = match h {
                crate::entities::layout_types::PageDimension::Auto => Value::Auto,
                crate::entities::layout_types::PageDimension::Length(x) => Value::Float(x),
            };
            *engine.styles = engine.styles.push_custom("page.height", v);
        }
        if let Some(ml) = margin_left {
            *engine.styles =
                engine.styles.push_custom("page.margin-left", Value::Float(ml));
        }
        if let Some(mr) = margin_right {
            *engine.styles =
                engine.styles.push_custom("page.margin-right", Value::Float(mr));
        }
        if let Some(mt) = margin_top {
            *engine.styles =
                engine.styles.push_custom("page.margin-top", Value::Float(mt));
        }
        if let Some(mb) = margin_bottom {
            *engine.styles =
                engine.styles.push_custom("page.margin-bottom", Value::Float(mb));
        }

        return Ok(Value::Content(Content::SetPage {
            width,
            height,
            margin,
            numbering,
            columns,
        }));
    }

    if target == "figure" {
        // #set figure(numbering: "1") — activa numeração automática de figuras.
        // Lote F-2 S3 (P335): empurra para a chain léxica (`custom`), como
        // heading/equation, em vez de mutar o campo global `engine.figure_numbering`.
        // `native_figure` assa o valor lendo a chain (`closures.rs`). Escopo
        // léxico de graça (fecha o DEBT 99.E para figure).
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                if named.name().as_str() == "numbering" {
                    let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                    let span = named.expr().span();
                    match val {
                        Value::Str(s) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("figure.numbering", Value::Str(s));
                        }
                        Value::None => {
                            // Limpa a numeração no escopo (Value::None = ausente).
                            *engine.styles = engine
                                .styles
                                .push_custom("figure.numbering", Value::None);
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "string or none",
                                &other,
                                span,
                            )]);
                        }
                    }
                }
            }
        }
        return Ok(Value::None);
    }

    if target == "table" {
        // P459 — `#set table(numbering: "1.")` activa numeração automática de
        // tables. O padrão vive na chain léxica (`custom("table.numbering")`),
        // análogo a figure/equation (F-2 S3). O layout lê o gate e prefixa o
        // caption quando ambos (caption + numbering) estão presentes.
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                if named.name().as_str() == "numbering" {
                    let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                    let span = named.expr().span();
                    match val {
                        Value::Str(s) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("table.numbering", Value::Str(s));
                        }
                        Value::None => {
                            *engine.styles =
                                engine.styles.push_custom("table.numbering", Value::None);
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "string or none",
                                &other,
                                span,
                            )]);
                        }
                    }
                }
            }
        }
        return Ok(Value::None);
    }

    if target == "par" {
        // Passo 133: target `par` activado sem arms concretos.
        // Passo 134: arm `leading` migrado de `text` para `par`
        // (paridade ADR-0033; resolve divergência temporal do 128).
        // Outras propriedades (justify, first-line-indent, etc.)
        // continuam no fallback até serem activadas on-demand.
        // F-5b fatia 2 (P373): `leading` migra de `StyleDelta` tipado para o canal
        // `custom` `"par.leading"` (morph-transparente, transportado). O layout
        // (merge arm) decodifica de volta.
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = named.name().as_str().to_owned();
                let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                match key.as_str() {
                    "leading" => {
                        if let Value::Length(l) = val {
                            *engine.styles = engine
                                .styles
                                .push_custom("par.leading", Value::Length(l));
                        }
                    }
                    _ => {
                        let (msg, hint) = unsupported_property_warn("par", &key, None);
                        engine.sink.warn_note(
                            named.name().to_untyped().span(),
                            &msg,
                            &hint,
                        );
                    }
                }
            }
        }
        return Ok(Value::None);
    }

    if target == "smartquote" {
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = named.name().as_str();
                let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                let span = named.expr().span();
                match key {
                    "enabled" => match val {
                        Value::Bool(b) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("smartquote.enabled", Value::Bool(b));
                        }
                        other => {
                            return Err(vec![type_mismatch("bool", &other, span)]);
                        }
                    },
                    "quotes" => match val {
                        Value::Str(s) => {
                            let char_count = s.chars().count();
                            if char_count != 2 {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!(
                                        "expected 2 characters, found {} characters",
                                        char_count
                                    ),
                                )]);
                            }
                            *engine.styles = engine
                                .styles
                                .push_custom("smartquote.quotes", Value::Str(s));
                        }
                        Value::None | Value::Auto => {
                            *engine.styles = engine
                                .styles
                                .push_custom("smartquote.quotes", Value::None);
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "string, auto or none",
                                &other,
                                span,
                            )]);
                        }
                    },
                    _ => {
                        let (msg, hint) =
                            unsupported_property_warn("smartquote", key, None);
                        engine.sink.warn_note(
                            named.name().to_untyped().span(),
                            &msg,
                            &hint,
                        );
                    }
                }
            }
        }
        return Ok(Value::None);
    }

    // F-item3 (P368, `f_fronteira_e1.md` §3a.11): `#set <elemento-de-usuário>(prop:)`
    // pelo mapa aberto. Se o `target` resolve em `scopes` a um `Func::element`
    // (elemento de usuário registrado), em vez do warn, empurra `("<kind>.<prop>",
    // Value)` no canal `custom` da chain — transparente à morfologia
    // (`is_semantically_empty` ignora o custom, P366). O elemento lê pela chain no
    // layout (precedência: construído explícito > chain > default). Aditivo: os
    // `#set` nativos (acima) não mudam.
    let is_user_elem = matches!(
        scopes.get(&target),
        Some(Value::Func(f)) if f.element_name().is_some()
    );
    if is_user_elem {
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = format!("{}.{}", target, named.name().as_str());
                let val =
                    eval_expr(named.expr(), scopes, ctx, engine).unwrap_or(Value::None);
                *engine.styles = engine.styles.push_custom(key, val);
            }
        }
        return Ok(Value::None);
    }

    if target != "text" {
        let (msg, hint) = unsupported_target_warn(&target);
        engine.sink.warn_note(target_span, &msg, &hint);
        return Ok(Value::None);
    }

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
                    *engine.styles =
                        engine.styles.push_custom("text.variations", val);
                }
                "tracking" => {
                    // P816: mesma validação de tipo de `size` (Length no
                    // vanilla, `text/mod.rs:333`).
                    let span = named.expr().span();
                    match val {                        Value::Length(l) => {
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
                                parse_font_dict_named_fields(
                                    dict_node, span, scopes, ctx, engine,
                                )?
                            } else {
                                parse_font_dict_legacy(
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
                                _ => None,
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
                            *engine.styles =
                                engine.styles.push_custom("text.top-edge", Value::Length(l));
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
                            *engine.styles =
                                engine.styles.push_custom("text.bottom-edge", Value::Str(s));
                        }
                        Value::Length(l) => {
                            *engine.styles =
                                engine.styles.push_custom("text.bottom-edge", Value::Length(l));
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
                    // vanilla mas ainda não capturados (ex.: `hyphenate`,
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

/// **Show-set (P352)** — captura o efeito de um `#set` como `Styles`, **sem**
/// mutar o estilo global. Reusa `eval_set_rule` (fiel a todos os targets, aos
/// warns e aos erros hard) aplicando-o a uma `StyleChain::empty()` temporária e
/// extrai o delta resultante (`collapse`). Sobre `empty()`, o resultado é só o
/// que o `set` definiu — sem os defaults. `engine.styles` é restaurado mesmo
/// em caso de erro (o swap de volta corre antes do `?`).
fn capture_set_styles(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Styles> {
    let mut scratch = StyleChain::empty();
    std::mem::swap(&mut *engine.styles, &mut scratch);
    let result = eval_set_rule(set, scopes, ctx, engine);
    std::mem::swap(&mut *engine.styles, &mut scratch);
    result?;
    Ok(Styles::from_delta(scratch.collapse()))
}

pub(super) fn eval_show_rule(
    show_rule: ShowRuleNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // P790 — `page`/`par` como alvo de show: identificadores especiais
    // reconhecidos ANTES de avaliar o selector (no vanilla são element
    // functions em scope; na stdlib cristalina não existem como variáveis —
    // caíam em `unknown variable` fatal, achado P786). Paridade medida por
    // execução (`typst-eval/src/rules.rs:67-95`):
    // - `#show page: <qualquer transformação>` → warning específico, nenhuma
    //   regra registada, compilação prossegue (a regra não tem efeito no
    //   vanilla).
    // - `#show par: set block(...)` com `spacing`/`above`/`below` → warning
    //   específico, nenhuma regra registada (vanilla condiciona a
    //   `BlockElem::above`/`below` na chain).
    // - `#show par: <outra transformação>` → erro explícito (scope-out
    //   documentado no L0, eval.md §P790: no vanilla `show par` é regra viva
    //   sobre `ParElem`; element rule de par é candidata a passo futuro).
    // P863: selector especial `par` pode ser resolvido antes de avaliar a
    // expressão (não existe como variável na stdlib cristalina).
    let mut preselected: Option<Selector> = None;
    if let Some(Expr::Ident(ident)) = show_rule.selector() {
        let rule_span = show_rule.to_untyped().span();
        match ident.as_str() {
            "page" => {
                engine.sink.warn_note(
                    rule_span,
                    "`show page` is not supported and has no effect",
                    "customize pages with `set page(..)` instead",
                );
                return Ok(Value::None);
            }
            "par" => {
                let is_set_block_spacing = matches!(
                    show_rule.transform(),
                    Expr::SetRule(set)
                        if set.target().to_untyped().text_str() == "block"
                            && set.args().items().any(|arg| matches!(
                                arg,
                                Arg::Named(named)
                                    if matches!(
                                        named.name().as_str(),
                                        "spacing" | "above" | "below"
                                    )
                            ))
                );
                if is_set_block_spacing {
                    engine.sink.warn_note2(
                        rule_span,
                        "`show par: set block(spacing: ..)` has no effect anymore",
                        "write `set par(spacing: ..)` instead",
                        "this is specific to paragraphs as they are not considered \
                         blocks anymore",
                    );
                    return Ok(Value::None);
                }
                preselected = Some(Selector::NodeKind(NodeKind::Par));
            }
            _ => {}
        }
    }

    // Avaliar o selector — pode ser uma string ou uma função da stdlib.
    // `selector()` retorna `Option<Expr>` — None significa selector omitido (não suportado).
    let selector = if let Some(sel) = preselected {
        sel
    } else {
        match show_rule.selector() {
        None => {
            return Err(vec![SourceDiagnostic::error(
                show_rule.to_untyped().span(),
                "show rule requer um selector".to_string(),
            )])
        }
        Some(sel_expr) => {
            let selector_val = eval_expr(sel_expr, scopes, ctx, engine)?;
            match selector_val {
                Value::Str(s) => {
                    // P790 — paridade vanilla `selector.rs:110` (medido por
                    // execução): selector de texto vazio é erro, não no-op.
                    if s.is_empty() {
                        return Err(vec![SourceDiagnostic::error(
                            sel_expr.span(),
                            "text selector is empty".to_string(),
                        )]);
                    }
                    Selector::Text(s.to_string())
                }
                // P393: regex(pattern) → selector regex sobre texto.
                Value::Regex(re) => Selector::Regex(re),
                // P791 — `<lbl>` → selector por label (paridade vanilla
                // `Selector::Label`). Aplicação dedicada em
                // `intercept_labelled` (não viaja pela travessia principal).
                Value::Label(l) => Selector::Label(l),
                // **P417 (M)** — Selector como valor de primeira classe
                // (`heading.where(level: 1)`). Converte do selector de query
                // para o selector de show rule.
                Value::Selector(sel) => {
                    query_selector_to_show_selector(sel, sel_expr.span())?
                }
                // Lote F-3 inc-2: elemento de utilizador (fronteira E1) — o
                // selector `callout` resolve para uma `FuncRepr::Element`, que
                // não tem fn-ptr nativo. Casa por **kind dinâmico** (nome).
                Value::Func(ref f) if f.element_name().is_some() => {
                    Selector::DynKind(f.element_name().unwrap().to_string())
                }
                Value::Func(ref f) => {
                    // Passo 84.3 (encerra DEBT-21): resolver NodeKind
                    // por identidade do function pointer da nativa
                    // subjacente, não pelo nome textual. Aliasing via
                    // `#let alias = heading` (clone do mesmo Arc<Func>)
                    // ou re-registo da mesma fn com nome diferente
                    // continuam a apontar para o mesmo `fn` — match.
                    //
                    // Closures retornam `None` em `native_fn_addr()` —
                    // function pointers de closures não são estáveis.
                    use crate::engine::stdlib::{
                        native_emph, native_enum, native_figure, native_footnote,
                        native_heading, native_link, native_list, native_overline,
                        native_par, native_quote, native_raw, native_smallcaps,
                        native_strike, native_strong, native_subscript, native_superscript,
                        native_underline,
                    };
                    use std::ptr::fn_addr_eq;
                    match f.native_fn_addr() {
                        Some(addr) if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Heading),
                        Some(addr) if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Figure),
                        Some(addr) if fn_addr_eq(addr, native_strong as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Strong),
                        Some(addr) if fn_addr_eq(addr, native_emph as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Emph),
                        Some(addr) if fn_addr_eq(addr, native_raw as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Raw),
                        Some(addr) if fn_addr_eq(addr, native_underline as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Underline),
                        Some(addr) if fn_addr_eq(addr, native_strike as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Strike),
                        Some(addr) if fn_addr_eq(addr, native_overline as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Overline),
                        Some(addr) if fn_addr_eq(addr, native_smallcaps as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Smallcaps),
                        Some(addr) if fn_addr_eq(addr, native_subscript as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Subscript),
                        Some(addr) if fn_addr_eq(addr, native_superscript as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Superscript),
                        // P494 — selectores de elementos de documento.
                        Some(addr) if fn_addr_eq(addr, native_link as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Link),
                        Some(addr) if fn_addr_eq(addr, native_quote as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Quote),
                        Some(addr) if fn_addr_eq(addr, native_footnote as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Footnote),
                        Some(addr) if fn_addr_eq(addr, native_list as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::List),
                        Some(addr) if fn_addr_eq(addr, native_enum as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Enum),
                        // P863 — `par` como função nativa mapeia para nó de parágrafo.
                        Some(addr) if fn_addr_eq(addr, native_par as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Par),
                        Some(_) => return Err(vec![SourceDiagnostic::error(
                            sel_expr.span(),
                            format!(
                                "função '{}' não é um tipo de nó suportado como selector. \
                                 Tipos suportados: heading, figure, strong, emph, raw, \
                                 underline, strike, overline, smallcaps, sub, super, \
                                 link, quote, list, enum, par.",
                                f.name().unwrap_or("<anónima>")
                            ),
                        )]),
                        None => return Err(vec![SourceDiagnostic::error(
                            sel_expr.span(),
                            "o selector de show rule deve ser uma função nativa \
                             ou uma string literal. Closures não são suportadas."
                                .to_string(),
                        )]),
                    }
                }
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        sel_expr.span(),
                        format!(
                            "selector inválido para show rule: {}",
                            other.type_name()
                        ),
                    )])
                }
            }
        }
    }
};

    // Classificar a transformação. Show-set (`#show k: set …`) é detetado pelo
    // tipo do nó (`Expr::SetRule`) e **capturado sem mutar `engine.styles`**
    // globalmente (P352); as restantes formas avaliam para um `Value` e
    // classificam-se em `Transformation`.
    let transform_expr = show_rule.transform();
    let transform = match transform_expr {
        Expr::SetRule(set) => {
            // Show-set é sobre um elemento (NodeKind/DynKind), não sobre texto
            // nem regex (P393).
            if matches!(selector, Selector::Text(_) | Selector::Regex(_)) {
                return Err(vec![SourceDiagnostic::error(
                    set.to_untyped().span(),
                    "show-set (`#show …: set …`) não é válido para um selector de \
                     texto — use uma função ou Content"
                        .to_string(),
                )]);
            }
            Transformation::Style(capture_set_styles(set, scopes, ctx, engine)?)
        }
        expr => {
            let value = eval_expr(expr, scopes, ctx, engine)?;
            match value {
                Value::Func(f) => Transformation::Func(f),
                Value::Content(c) => Transformation::Content(c),
                Value::Str(s) => Transformation::Str(s),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        expr.span(),
                        format!(
                            "transformação de show rule inválida: esperado função, \
                         Content, string ou set rule, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            }
        }
    };
    let id = ctx.next_rule_id;
    ctx.next_rule_id += 1;
    // Reconstruir o slice com a nova regra (Passo 95 + 109: show_rules
    // é campo do Engine, mutação local via &mut).
    let mut rules = engine.show_rules.to_vec();
    rules.push(ShowRule { id, selector, transform });
    *engine.show_rules = Arc::from(rules);
    Ok(Value::None)
}

/// P414: parsing do dict `text.font` no formato named fields vanilla.
///
/// Campos suportados: `family` (Str|Regex), `variant` (Str),
/// `weight` (Int|Str), `style` (Str), `fallback` (Bool, default true).
/// `stretch` e outros campos são rejeitados.
fn parse_font_dict_named_fields<'a>(
    dict_node: Dict<'a>,
    span: Span,
    scopes: &mut Scopes,
    ctx: &mut EvalContext,
    engine: &mut Engine,
) -> SourceResult<Vec<Value>> {
    use crate::entities::value::Value;

    let mut family: Option<Value> = None;
    let mut variant: Option<EcoString> = None;
    let mut weight: Option<EcoString> = None;
    let mut style: Option<EcoString> = None;
    let mut fallback = true;

    for item in dict_node.items() {
        let named = match item {
            DictItem::Named(n) => n,
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "font dict named fields must use identifiers as keys".to_string(),
                )]);
            }
        };

        let field = named.name().as_str();
        let value = eval_expr(named.expr(), scopes, ctx, engine)?;

        match field {
            "family" => {
                if !matches!(value, Value::Str(_) | Value::Regex(_)) {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "font dict field 'family' expects string or regex, recebeu {}",
                            value.type_name()
                        ),
                    )]);
                }
                family = Some(value);
            }
            "variant" => match value {
                Value::Str(s) => variant = Some(s),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "font dict field 'variant' expects string, recebeu {}",
                            other.type_name()
                        ),
                    )]);
                }
            },
            "weight" => {
                let s = match value {
                    Value::Int(n) => EcoString::from(n.to_string()),
                    Value::Str(s) => s,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "font dict field 'weight' expects integer or string, recebeu {}",
                                other.type_name()
                            ),
                        )]);
                    }
                };
                weight = Some(s);
            }
            "style" => match value {
                Value::Str(s) => style = Some(s),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "font dict field 'style' expects string, recebeu {}",
                            other.type_name()
                        ),
                    )]);
                }
            },
            "fallback" => match value {
                Value::Bool(b) => fallback = b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "font dict field 'fallback' expects boolean, recebeu {}",
                            other.type_name()
                        ),
                    )]);
                }
            },
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("unknown font dict field: {}", other),
                )]);
            }
        }
    }

    let family = family.ok_or_else(|| {
        vec![SourceDiagnostic::error(
            span,
            "font dict missing field 'family'".to_string(),
        )]
    })?;

    if !fallback {
        // `fallback: false` semanticamente força fonte única. Como a forma
        // named fields gera um único item, a verificação é documental; o
        // layout respeita a lista resultante.
    }

    let mut entry: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    entry.insert(EcoString::from("name"), family);
    entry.insert(EcoString::from("variants"), Value::Array(vec![]));
    if let Some(v) = variant {
        entry.insert(EcoString::from("variant"), Value::Str(v));
    }
    if let Some(w) = weight {
        entry.insert(EcoString::from("weight"), Value::Str(w));
    }
    if let Some(s) = style {
        entry.insert(EcoString::from("style"), Value::Str(s));
    }

    Ok(vec![Value::Dict(entry)])
}

/// P407: parsing do dict `text.font` no formato legado cristalino.
///
/// Chaves são nomes de família (string literal, identificador ou regex);
/// valores são variant names (string ou array de strings).
fn parse_font_dict_legacy<'a>(
    dict_node: Dict<'a>,
    span: Span,
    scopes: &mut Scopes,
    ctx: &mut EvalContext,
    engine: &mut Engine,
) -> SourceResult<Vec<Value>> {
    let mut items = Vec::new();
    for dict_item in dict_node.items() {
        let (name_val, variants_val) = match dict_item {
            DictItem::Keyed(keyed) => {
                let key_expr = keyed.key();
                let name_val = match key_expr {
                    Expr::Str(node) => Value::Str(EcoString::from(node.get()?)),
                    Expr::FuncCall(call) => {
                        // regex("...") avalia para Value::Regex.
                        match eval_expr(Expr::FuncCall(call), scopes, ctx, engine)? {
                            Value::Regex(re) => Value::Regex(re),
                            other => {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!("font dict key must be string or regex, recebeu {}", other.type_name()),
                                )]);
                            }
                        }
                    }
                    _other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            "font dict key must be string or regex".to_string(),
                        )]);
                    }
                };
                let value = eval_expr(keyed.expr(), scopes, ctx, engine)?;
                let variants = variants_from_value(value, span)?;
                (name_val, variants)
            }
            DictItem::Named(named_item) => {
                // Identificador como key: "Name": value é syntax sugar
                // para string literal lowercased.
                let name = named_item.name().as_str();
                let name_val = Value::Str(EcoString::from(name));
                let value = eval_expr(named_item.expr(), scopes, ctx, engine)?;
                let variants = variants_from_value(value, span)?;
                (name_val, variants)
            }
            DictItem::Spread(_) => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "font dict spread not supported".to_string(),
                )]);
            }
        };
        let mut entry: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        entry.insert(EcoString::from("name"), name_val);
        entry.insert(EcoString::from("variants"), Value::Array(variants_val));
        items.push(Value::Dict(entry));
    }
    if items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            span,
            "font dict must not be empty".to_string(),
        )]);
    }
    Ok(items)
}

/// Helper partilhado entre os dois parsers de dict de fonte.
fn variants_from_value(value: Value, span: Span) -> SourceResult<Vec<Value>> {
    match value {
        Value::Str(s) => Ok(vec![Value::Str(s)]),
        Value::Array(arr) => {
            let mut vs = Vec::with_capacity(arr.len());
            for v in arr.iter() {
                if let Value::Str(s) = v {
                    vs.push(Value::Str(s.clone()));
                } else {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        "font variants must be strings".to_string(),
                    )]);
                }
            }
            Ok(vs)
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "font dict value must be string or array of strings, recebeu {}",
                other.type_name()
            ),
        )]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::show::Selector;
    use crate::entities::value::Value;
    use ecow::EcoString;

    fn where_selector(field: &str, value: Value) -> Selector {
        Selector::Where {
            base: Box::new(Selector::NodeKind(NodeKind::Heading)),
            field: EcoString::from(field),
            value: Box::new(value),
        }
    }

    #[test]
    fn p417_matches_where_positivo() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("level", Value::Int(1));
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_valor_errado() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("level", Value::Int(2));
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_campo_inexistente() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("inexistente", Value::Int(1));
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_base_nao_casa() {
        let content =
            Content::figure(Content::text("Fig"), None, Some("image".to_string()), None);
        let sel = where_selector("level", Value::Int(1));
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_int_float_coerce() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("level", Value::Float(1.0));
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_body_content() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("body", Value::Content(Content::text("Intro")));
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p417_extract_field_heading_level() {
        let content = Content::heading(2, Content::text("X"));
        assert_eq!(content.get_field("level"), Some(Value::Int(2)));
    }

    #[test]
    fn p417_extract_field_heading_body() {
        let content = Content::heading(1, Content::text("Intro"));
        assert_eq!(
            content.get_field("body"),
            Some(Value::Content(Content::text("Intro")))
        );
    }

    #[test]
    fn p417_extract_field_heading_inexistente() {
        let content = Content::heading(1, Content::text("X"));
        assert_eq!(content.get_field("inexistente"), None);
    }

    // ── P423 (S-M) — combinadores And/Or em show rules ──────────────────────

    fn or_selector(a: Selector, b: Selector) -> Selector {
        Selector::Or(vec![a, b])
    }

    fn and_selector(a: Selector, b: Selector) -> Selector {
        Selector::And(vec![a, b])
    }

    #[test]
    fn p423_matches_or_positivo_heading() {
        let content = Content::heading(1, Content::text("T"));
        let sel = or_selector(
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        );
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_or_positivo_figure() {
        let content =
            Content::figure(Content::text("F"), None, Some("image".to_string()), None);
        let sel = or_selector(
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        );
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_or_negativo_paragraph() {
        let content = Content::text("par");
        let sel = or_selector(
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        );
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_and_positivo_where_plus_kind() {
        let content = Content::heading(1, Content::text("T"));
        let sel = and_selector(
            Selector::Where {
                base: Box::new(Selector::NodeKind(NodeKind::Heading)),
                field: "level".into(),
                value: Box::new(Value::Int(1)),
            },
            Selector::NodeKind(NodeKind::Heading),
        );
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_and_negativo_contraditorio() {
        let content = Content::heading(1, Content::text("T"));
        let sel = and_selector(
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        );
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_and_vazio_false() {
        let content = Content::heading(1, Content::text("T"));
        let sel = Selector::And(vec![]);
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_or_vazio_false() {
        let content = Content::heading(1, Content::text("T"));
        let sel = Selector::Or(vec![]);
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p423_is_node_rule_and_or_com_node_kinds() {
        let sel = Selector::And(vec![
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        ]);
        assert!(is_node_rule(&sel));
        let sel = Selector::Or(vec![
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        ]);
        assert!(is_node_rule(&sel));
    }

    #[test]
    fn p423_is_node_rule_and_nao_node_rejeita() {
        let sel = Selector::And(vec![
            Selector::NodeKind(NodeKind::Heading),
            Selector::Text("x".to_string()),
        ]);
        assert!(!is_node_rule(&sel));
    }

    // ── Passo 444 (P284) — selectors para text decoration ───────────────────

    #[test]
    fn p444_selector_underline_casa_content_underline() {
        let content = Content::underline(Content::text("x"), None, None, None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Underline)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Strike)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Overline)));
    }

    #[test]
    fn p444_selector_strike_casa_content_strike() {
        let content = Content::strike(Content::text("x"), None, None, None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Strike)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Underline)));
    }

    #[test]
    fn p444_selector_overline_casa_content_overline() {
        let content = Content::overline(Content::text("x"), None, None, None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Overline)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Underline)));
    }

    #[test]
    fn p444_selector_decoration_nao_casa_texto_plano() {
        let content = Content::text("x");
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Underline)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Strike)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Overline)));
    }

    // ── Passo 446 (P408) — selector para smallcaps ──────────────────────────

    #[test]
    fn p446_selector_smallcaps_casa_content_smallcaps() {
        let content = Content::smallcaps(Content::text("x"));
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Smallcaps)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Strong)));
    }

    #[test]
    fn p446_selector_smallcaps_nao_casa_texto_plano() {
        let content = Content::text("x");
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Smallcaps)));
    }

    // ── Passo 449 (P449) — selector para highlight ────────────────────────

    #[test]
    fn p449_selector_highlight_casa_styled_highlight() {
        let content = Content::highlight(Content::text("x"), None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Highlight)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Strong)));
    }

    #[test]
    fn p449_selector_highlight_nao_casa_texto_plano() {
        let content = Content::text("x");
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Highlight)));
    }

    #[test]
    fn p449_selector_highlight_fill_none_ainda_casa() {
        // `fill: none` é um highlight "desactivado"; continua a casar o selector
        // de tipo, exactamente como um `set` vazio não remove a origem sintática.
        let content = Content::highlight(Content::text("x"), None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Highlight)));
    }
}
