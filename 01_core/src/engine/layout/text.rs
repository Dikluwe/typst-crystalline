//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/atomizacao_elementos.md
//! @prompt-hash a54abe5a
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `Text` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Folha de render —
//! decodifica o `#set text`/`#set par` da chain, faz merge top-wins com o
//! estilo tipado, e dispõe cada palavra via `layout_word`. **Não re-entra
//! `layout_content`** (não orquestra). Caminho quente — content-preserving.

use crate::entities::layout_types::{Length, Pt, TextStyle};
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
    let ns_style_italic = matches!(cs("text.style"), Some(Value::Str(s)) if s.as_str() == "italic" || s.as_str() == "oblique");
    let ns_tracking = match cs("text.tracking") {
        Some(Value::Length(l)) => Some(l.clone()),
        _ => None,
    };
    let ns_leading = match cs("par.leading") {
        Some(Value::Length(l)) => Some(l.clone()),
        _ => None,
    };
    let ns_top_edge = match cs("text.top-edge") {
        Some(Value::Str(s)) => Some(s.clone()),
        _ => None,
    };
    let ns_bottom_edge = match cs("text.bottom-edge") {
        Some(Value::Str(s)) => Some(s.clone()),
        _ => None,
    };
    let ns_lang = match cs("text.lang") {
        Some(Value::Str(s)) => {
            use std::str::FromStr;
            crate::entities::lang::Lang::from_str(s).ok()
        }
        _ => None,
    };
    let ns_dir = match cs("text.dir") {
        Some(Value::Dir(dir)) => Some(*dir),
        _ => None,
    };
    let ns_font = match cs("text.font") {
        Some(Value::Array(arr)) => {
            use crate::entities::font_list::{FontFamily, FontNamePattern};
            use ecow::EcoString;

            let fams: Vec<_> = arr.iter().filter_map(|v| {
                match v {
                    // Forma P292/P373: string literal, variants vazio.
                    Value::Str(s) => Some(FontFamily::new(s.clone())),
                    // Forma P407/P414: dict com name (Str|Regex) + variants
                    // (Array[Str]) e campos named optionais variant/weight/style.
                    Value::Dict(dict) => {
                        let name = dict.get("name")?;
                        let pattern = match name {
                            Value::Str(s) => FontNamePattern::Literal(s.clone()),
                            Value::Regex(re) => FontNamePattern::Regex(re.clone()),
                            _ => return None,
                        };
                        let variants: Vec<EcoString> = match dict.get("variants") {
                            Some(Value::Array(arr)) => arr.iter()
                                .filter_map(|v| match v {
                                    Value::Str(s) => Some(s.clone()),
                                    _ => None,
                                })
                                .collect(),
                            _ => return None,
                        };
                        let variant = dict.get("variant")
                            .and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None });
                        let weight = dict.get("weight")
                            .and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None });
                        let style = dict.get("style")
                            .and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None });
                        Some(FontFamily {
                            name: pattern,
                            variants,
                            variant,
                            weight,
                            style,
                            covers: None,
                        })
                    }
                    _ => None,
                }
            }).collect();
            crate::entities::font_list::FontList::new(fams)
        }
        _ => None,
    };
    let mut effective = TextStyle {
        bold:   ns_bold   || layouter.style.bold,
        italic: ns_italic || ns_style_italic || layouter.style.italic,
        size:   ns_size.unwrap_or(layouter.style.size),
        fill:          layouter.style.fill.or(ns_fill),
        heading_level: layouter.style.heading_level,
        // Top-wins: chain tipada (heading) vence; senão o ns (#set).
        weight:        layouter.style.weight.or(ns_weight),
        tracking:      layouter.style.tracking.clone().or(ns_tracking),
        leading:       layouter.style.leading.clone().or(ns_leading),
        top_edge:      layouter.style.top_edge.clone().or(ns_top_edge),
        bottom_edge:   layouter.style.bottom_edge.clone().or(ns_bottom_edge),
        lang:          layouter.style.lang.clone().or(ns_lang),
        // P660 fix: `#set text(font: ...)` deve substituir a fonte default da
        // chain (Libertinus Serif). A chain tipada (heading) ainda vence porque
        // o eval show-rule já assa a fonte no `StyleDelta::font` da chain, e
        // `ns_font` (do canal custom `text.font`) é None nesse caminho.
        font:          ns_font.or(layouter.style.font.clone()),
        dir:           layouter.style.dir.or(ns_dir),
        subscript:        layouter.style.subscript,
        superscript:      layouter.style.superscript,
        highlight:        layouter.style.highlight,
        highlight_radius: layouter.style.highlight_radius,
        highlight_extent: layouter.style.highlight_extent,
        subscript_size:   layouter.style.subscript_size,
        superscript_size: layouter.style.superscript_size,
        baseline_offset:  layouter.style.baseline_offset,
        // P784 — herda do style corrente (regular ou math); este merge não
        // é math-específico, só reflecte o valor já activo no layouter.
        math:             layouter.style.math,
    };

    // **P448/P471**: subscrito/sobrescrito reduzem o corpo e deslocam a baseline.
    // P471: `size` explícito substitui a escala padrão de 65%.
    const SCRIPT_SCALE: f64 = 0.65;
    if effective.subscript {
        effective.size = effective.subscript_size
            .map(|l| Pt(l.resolve_pt(effective.size.val())))
            .unwrap_or_else(|| Pt(effective.size.0 * SCRIPT_SCALE));
        effective.baseline_offset = Length::em(-0.2);
    } else if effective.superscript {
        effective.size = effective.superscript_size
            .map(|l| Pt(l.resolve_pt(effective.size.val())))
            .unwrap_or_else(|| Pt(effective.size.0 * SCRIPT_SCALE));
        effective.baseline_offset = Length::em(0.3);
    }

    let prev_style = layouter.style.clone();
    layouter.style = effective;

    // **P544/P547** — preservar espaços entre palavras dentro de um nó Text,
    // incluindo espaços iniciais/finais e nós Text compostos só por espaços.
    // `split_whitespace()` perdia esses separadores, juntando palavras e
    // removendo o espaço antes de pontuação. `split(' ')` mantém cada
    // posição de espaço como um segmento vazio; adicionamos `space_width()`
    // entre segmentos e para cada segmento vazio.
    let mut parts = text.split(' ').peekable();
    while let Some(part) = parts.next() {
        if !part.is_empty() {
            if layouter.smallcaps {
                const SCALE: f64 = 0.8;
                let chars: Vec<char> = part.chars().collect();
                let mut i = 0;
                while i < chars.len() {
                    let is_lower = chars[i].is_lowercase();
                    let mut j = i;
                    while j < chars.len() && chars[j].is_lowercase() == is_lower {
                        j += 1;
                    }
                    let run: String = chars[i..j].iter().collect();
                    if is_lower {
                        let upper = run.to_uppercase();
                        let base_size = layouter.style.size;
                        layouter.style.size = Pt(base_size.0 * SCALE);
                        layouter.layout_chunk(&upper);
                        layouter.style.size = base_size;
                    } else {
                        layouter.layout_chunk(&run);
                    }
                    i = j;
                }
            } else {
                layouter.layout_word(part);
            }
        }
        if parts.peek().is_some() {
            layouter.regions.current.cursor_x += layouter.space_width();
        }
    }
    layouter.style = prev_style;
}
