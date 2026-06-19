//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash cedb58ca
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `Text` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Folha de render —
//! decodifica o `#set text`/`#set par` da chain, faz merge top-wins com o
//! estilo tipado, e dispõe cada palavra via `layout_word`. **Não re-entra
//! `layout_content`** (não orquestra). Caminho quente — content-preserving.

use crate::entities::layout_types::{Pt, TextStyle};
use crate::entities::value::Value;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de um nó `Text`: resolve o estilo efectivo (chain tipada vence o
/// render `#set` decodificado da chain `custom`) e dispõe as palavras.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    text:     &str,
) {
    // **F-5b fatia 2 (P373, §3a.13/§3a.14)**: o render do `#set text`/
    // `#set par` não vem mais assado no node — vive na chain pelo canal
    // `custom` (`"text.<campo>"` / `"par.leading"`), levado até aqui pelo
    // transporte aninhado. Decodifica-o num node-render `ns` (Value→tipo)
    // e aplica a MESMA regra de merge top-wins de antes: a chain tipada
    // (`layouter.style` — heading, e Bold/Italic de `Content::Styled`) vence;
    // o `ns` cobre o resto.
    let cs = |k: &str| layouter.chain.custom(k);
    let ns_bold   = matches!(cs("text.bold"),   Some(Value::Bool(true)));
    let ns_italic = matches!(cs("text.italic"), Some(Value::Bool(true)));
    let ns_size   = match cs("text.size") {
        Some(Value::Length(l)) => Some(Pt(l.abs.to_pt())),
        _ => None,
    };
    let ns_fill   = match cs("text.fill") {
        Some(Value::Color(c)) => Some(c.clone()),
        _ => None,
    };
    let ns_weight = match cs("text.weight") {
        Some(Value::Int(n)) => u16::try_from(*n).ok(),
        _ => None,
    };
    let ns_tracking = match cs("text.tracking") {
        Some(Value::Length(l)) => Some(l.clone()),
        _ => None,
    };
    let ns_leading = match cs("par.leading") {
        Some(Value::Length(l)) => Some(l.clone()),
        _ => None,
    };
    let ns_lang = match cs("text.lang") {
        Some(Value::Str(s)) => {
            use std::str::FromStr;
            crate::entities::lang::Lang::from_str(s).ok()
        }
        _ => None,
    };
    let ns_font = match cs("text.font") {
        Some(Value::Array(arr)) => {
            let fams: Vec<_> = arr.iter().filter_map(|v| {
                if let Value::Str(s) = v {
                    Some(crate::entities::font_list::FontFamily::new(s.clone()))
                } else {
                    None
                }
            }).collect();
            crate::entities::font_list::FontList::new(fams)
        }
        _ => None,
    };
    let effective = TextStyle {
        bold:   ns_bold   || layouter.style.bold,
        italic: ns_italic || layouter.style.italic,
        size:   if layouter.style.size > layouter.font_size_pt {
            layouter.style.size   // heading ou Content::Styled aumentou
        } else {
            // F-5b fatia 2 (P373): equivalente exato do antigo
            // `node_style.size` = `TextStyle::from(chain).size` =
            // `chain.size()` (default 11.0); `ns_size` = `#set text(size)`
            // (custom). NÃO `layouter.font_size_pt` (12.0 — default do layouter,
            // distinto do default da chain; ver P373).
            ns_size.unwrap_or(Pt(layouter.chain.size()))
        },
        fill:          layouter.style.fill.or(ns_fill),
        heading_level: layouter.style.heading_level,
        // Top-wins: chain tipada (heading) vence; senão o ns (#set).
        weight:        layouter.style.weight.or(ns_weight),
        tracking:      layouter.style.tracking.clone().or(ns_tracking),
        leading:       layouter.style.leading.clone().or(ns_leading),
        lang:          layouter.style.lang.clone().or(ns_lang),
        font:          layouter.style.font.clone().or(ns_font),
    };
    let prev_style = layouter.style.clone();
    layouter.style = effective;
    for word in text.split_whitespace() {
        layouter.layout_word(word);
    }
    layouter.style = prev_style;
}
