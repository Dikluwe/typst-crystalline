//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/lang/quotes.md
//! @prompt-hash 363884f5
//! @layer L1
//! @updated 2026-10-06
//!
//! Smart-quotes lang-aware (Passo 155, ADR-0060 Fase 1, sub-passo 2).
//!
//! Tabela estática mapeando códigos ISO 639-1/2/3 a pares de aspas
//! `(open, close)`. Lookup linear por exact match no `Lang::as_str()`;
//! fallback `DEFAULT_QUOTES` (ASCII) para línguas não listadas.
//!
//! Cristalino's `Lang` (ADR-0052) é 2-3 letras ASCII puro — não tem
//! region/country (e.g. `pt-BR`); por isso o lookup BCP47 com prefixo
//! sugerido na spec P155 simplifica para exact match.

use crate::entities::elements::smartquote::{
    SmartQuoteOverrides, SmartQuotePair, SmartQuoteQuotes,
};
use crate::entities::lang::Lang;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use unicode_segmentation::UnicodeSegmentation;

/// Tabela inicial de aspas primárias por idioma (Passo 155).
///
/// Cobertura inicial: 6 idiomas + default ASCII.
/// Outras línguas (zh, ja, ar, ...) caem em `DEFAULT_QUOTES`; expansível
/// em passo futuro sem breaking change.
const LANG_QUOTES: &[(&str, (&str, &str))] = &[
    // (lang_code, (open, close))
    ("pt", ("\u{00AB}", "\u{00BB}")), // « »
    ("en", ("\u{201C}", "\u{201D}")), // " "
    ("de", ("\u{201E}", "\u{201C}")), // „ "
    ("fr", ("\u{00AB}\u{00A0}", "\u{00A0}\u{00BB}")), // « »  com NBSP
    ("es", ("\u{00AB}", "\u{00BB}")), // « »
    ("it", ("\u{00AB}", "\u{00BB}")), // « »
];

/// Aspas default (ASCII) para línguas não cobertas pela tabela.
pub const DEFAULT_QUOTES: (&str, &str) = ("\u{201C}", "\u{201D}");

/// Aspas simples default (curly inglesas) para línguas não cobertas.
pub const DEFAULT_SINGLE_QUOTES: (&str, &str) = ("\u{2018}", "\u{2019}");

/// Tabela inicial de aspas simples por idioma (Passo 445).
///
/// Cobertura inicial: inglês + default curly. Outras línguas caem em
/// `DEFAULT_SINGLE_QUOTES`.
const LANG_SINGLE_QUOTES: &[(&str, (&str, &str))] = &[
    // (lang_code, (open, close))
    ("en", ("\u{2018}", "\u{2019}")), // ‘ ’
    ("de", ("\u{201A}", "\u{2018}")), // ‚ ‘
];

fn pair(open: &str, close: &str) -> SmartQuotePair {
    SmartQuotePair { open: open.into(), close: close.into() }
}

fn pair_from_string(value: &str, span: Span) -> SourceResult<SmartQuotePair> {
    let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(value, true).collect();
    if graphemes.len() != 2 {
        let suffix = if graphemes.len() == 1 { "" } else { "s" };
        return Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected 2 characters, found {} character{}",
                graphemes.len(),
                suffix
            ),
        )]);
    }
    Ok(pair(graphemes[0], graphemes[1]))
}

fn pair_from_array(values: &[Value], span: Span) -> SourceResult<SmartQuotePair> {
    if values.len() != 2 {
        let suffix = if values.len() == 1 { "" } else { "s" };
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("expected 2 quotes, found {} quote{}", values.len(), suffix),
        )]);
    }
    let Value::Str(open) = &values[0] else {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("expected string, found {}", values[0].type_name()),
        )]);
    };
    let Value::Str(close) = &values[1] else {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("expected string, found {}", values[1].type_name()),
        )]);
    };
    Ok(SmartQuotePair { open: open.clone(), close: close.clone() })
}

fn optional_pair(value: &Value, span: Span) -> SourceResult<Option<SmartQuotePair>> {
    match value {
        Value::Auto | Value::None => Ok(None),
        Value::Str(value) => pair_from_string(value, span).map(Some),
        Value::Array(values) => pair_from_array(values, span).map(Some),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected auto, string, or array, found {}", other.type_name()),
        )]),
    }
}

/// Valida e canoniza a superfície `quotes:` do elemento smartquote.
pub fn parse_smartquote_quotes(
    value: &Value,
    span: Span,
) -> SourceResult<SmartQuoteQuotes> {
    match value {
        Value::Auto | Value::None => Ok(SmartQuoteQuotes::Auto),
        Value::Str(value) => Ok(SmartQuoteQuotes::Custom(SmartQuoteOverrides {
            single: None,
            double: Some(pair_from_string(value, span)?),
        })),
        Value::Array(values) => Ok(SmartQuoteQuotes::Custom(SmartQuoteOverrides {
            single: None,
            double: Some(pair_from_array(values, span)?),
        })),
        Value::Dict(values) => {
            for key in values.keys() {
                if !matches!(key.as_str(), "single" | "double") {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "unexpected key \"{}\", valid keys are \"double\" and \"single\"",
                            key
                        ),
                    )]);
                }
            }
            Ok(SmartQuoteQuotes::Custom(SmartQuoteOverrides {
                single: values
                    .get("single")
                    .map(|value| optional_pair(value, span))
                    .transpose()?
                    .flatten(),
                double: values
                    .get("double")
                    .map(|value| optional_pair(value, span))
                    .transpose()?
                    .flatten(),
            }))
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected auto, string, array, or dictionary, found {}",
                other.type_name()
            ),
        )]),
    }
}

/// Resolve o par efectivo depois da precedência leaf > style > localizado.
pub fn resolve_smartquote_pair(
    lang: Option<&Lang>,
    alternative: bool,
    quotes: Option<&SmartQuoteQuotes>,
    double: bool,
) -> SmartQuotePair {
    let localized = if alternative && lang.is_some_and(|lang| lang.as_str() == "de") {
        if double {
            pair("»", "«")
        } else {
            pair("›", "‹")
        }
    } else if double {
        let (open, close) = lang.map(localize_quotes).unwrap_or(DEFAULT_QUOTES);
        pair(open, close)
    } else {
        let (open, close) =
            lang.map(localize_single_quotes).unwrap_or(DEFAULT_SINGLE_QUOTES);
        pair(open, close)
    };

    match quotes {
        Some(SmartQuoteQuotes::Custom(overrides)) => {
            let configured = if double { &overrides.double } else { &overrides.single };
            configured.clone().unwrap_or(localized)
        }
        Some(SmartQuoteQuotes::Auto) | None => localized,
    }
}

/// Context of one logical flow after show realization, independent of frames.
#[derive(Debug, Clone, Default)]
pub struct SmartQuoteContext {
    stack: Vec<bool>,
    previous: Option<char>,
}

impl SmartQuoteContext {
    pub fn observe_text(&mut self, text: &str) {
        if let Some(last) = text.chars().last() {
            self.previous = Some(last);
        }
    }
    pub fn observe_object(&mut self) {
        self.previous = Some('\u{FFFC}');
    }
    pub fn reset_paragraph(&mut self) {
        self.stack.clear();
        self.previous = None;
    }
}

/// Resolve explicit fields over scoped styles, and retain causal nesting.
pub fn resolve_smart_quote(
    context: &mut SmartQuoteContext,
    element: &crate::entities::elements::smartquote::SmartQuoteElem,
    chain: &crate::entities::style_chain::StyleChain,
    effective_lang: Option<&Lang>,
) -> SourceResult<ecow::EcoString> {
    let boolean = |key: &str, fallback: bool| match chain.custom(key) {
        Some(Value::Bool(value)) => *value,
        _ => fallback,
    };
    let double = element.double.unwrap_or_else(|| boolean("smartquote.double", true));
    let enabled = element.enabled.unwrap_or_else(|| boolean("smartquote.enabled", true));
    if !enabled {
        let glyph = if double { "\"" } else { "'" };
        context.observe_text(glyph);
        return Ok(glyph.into());
    }
    let alternative = element
        .alternative
        .unwrap_or_else(|| boolean("smartquote.alternative", false));
    let inherited = if element.quotes.is_none() {
        chain
            .custom("smartquote.quotes")
            .map(|v| parse_smartquote_quotes(v, Span::detached()))
            .transpose()?
    } else {
        None
    };
    let lang = chain.lang();
    let pair = resolve_smartquote_pair(
        effective_lang.or(lang.as_ref()),
        alternative,
        element.quotes.as_ref().or(inherited.as_ref()),
        double,
    );
    let before = context.previous.unwrap_or(' ');
    let top = context.stack.last().copied();
    let glyph = if before.is_numeric() && top != Some(double) {
        if double {
            "″".into()
        } else {
            "′".into()
        }
    } else if !double
        && top != Some(false)
        && (before.is_alphabetic() || before == '\u{FFFC}')
    {
        "’".into()
    } else if top == Some(double)
        && !before.is_whitespace()
        && !matches!(before, '(' | '[' | '{' | '<' | '⟨')
    {
        context.stack.pop();
        pair.close
    } else {
        context.stack.push(double);
        pair.open
    };
    context.observe_text(&glyph);
    Ok(glyph)
}

/// Devolve par `(open, close)` de aspas primárias para o `Lang` dado.
///
/// Lookup por exact match no código ISO. Línguas não cobertas devolvem
/// `DEFAULT_QUOTES` (ASCII).
pub fn localize_quotes(lang: &Lang) -> (&'static str, &'static str) {
    let code = lang.as_str();
    for (key, pair) in LANG_QUOTES.iter() {
        if *key == code {
            return *pair;
        }
    }
    DEFAULT_QUOTES
}

/// Devolve par `(open, close)` de aspas simples para o `Lang` dado.
///
/// Lookup por exact match no código ISO. Línguas não cobertas devolvem
/// `DEFAULT_SINGLE_QUOTES` (curly inglesas).
pub fn localize_single_quotes(lang: &Lang) -> (&'static str, &'static str) {
    let code = lang.as_str();
    for (key, pair) in LANG_SINGLE_QUOTES.iter() {
        if *key == code {
            return *pair;
        }
    }
    DEFAULT_SINGLE_QUOTES
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn localize_quotes_pt_devolve_aspas_baixas() {
        let lang = Lang::from_str("pt").unwrap();
        assert_eq!(localize_quotes(&lang), ("\u{00AB}", "\u{00BB}"));
    }

    #[test]
    fn localize_quotes_en_devolve_curly() {
        let lang = Lang::from_str("en").unwrap();
        assert_eq!(localize_quotes(&lang), ("\u{201C}", "\u{201D}"));
    }

    #[test]
    fn localize_quotes_de_devolve_par_germanico() {
        let lang = Lang::from_str("de").unwrap();
        // Alemão: open low (U+201E „), close high left (U+201C ").
        assert_eq!(localize_quotes(&lang), ("\u{201E}", "\u{201C}"));
    }

    #[test]
    fn localize_quotes_fr_inclui_nbsp() {
        let lang = Lang::from_str("fr").unwrap();
        let (open, close) = localize_quotes(&lang);
        assert!(open.contains('\u{00A0}'), "FR open deve conter NBSP: {:?}", open);
        assert!(close.contains('\u{00A0}'), "FR close deve conter NBSP: {:?}", close);
    }

    #[test]
    fn localize_quotes_lang_desconhecido_devolve_default_curved() {
        // 'jp' (3-letter for Japanese) ou outro lang fora da tabela.
        let lang = Lang::from_str("jp").unwrap();
        assert_eq!(localize_quotes(&lang), DEFAULT_QUOTES);
        assert_eq!(localize_quotes(&lang), ("“", "”"));
    }

    #[test]
    fn localize_quotes_es_e_it_partilham_chevrons() {
        let es = Lang::from_str("es").unwrap();
        let it = Lang::from_str("it").unwrap();
        assert_eq!(localize_quotes(&es), ("\u{00AB}", "\u{00BB}"));
        assert_eq!(localize_quotes(&it), ("\u{00AB}", "\u{00BB}"));
    }

    #[test]
    fn localize_quotes_3_letter_iso_caia_default() {
        // ISO 639-2/3 codes (3 letras) que não estão na tabela.
        // `por` (Portuguese) é o 3-letter code; vão a default.
        let lang = Lang::from_str("por").unwrap();
        assert_eq!(localize_quotes(&lang), DEFAULT_QUOTES);
    }

    // ── Passo 445 — aspas simples (single quotes) ─────────────────────────

    #[test]
    fn localize_single_quotes_en_devolve_curly() {
        let lang = Lang::from_str("en").unwrap();
        assert_eq!(localize_single_quotes(&lang), ("\u{2018}", "\u{2019}"));
    }

    #[test]
    fn localize_single_quotes_default_curly_inglesas() {
        let lang = Lang::from_str("pt").unwrap();
        assert_eq!(localize_single_quotes(&lang), DEFAULT_SINGLE_QUOTES);
        assert_eq!(DEFAULT_SINGLE_QUOTES, ("\u{2018}", "\u{2019}"));
    }
}

#[cfg(test)]
mod contextual_tests {
    use super::*;
    use crate::entities::{content::Content, style_chain::StyleChain};
    fn emit(ctx: &mut SmartQuoteContext, double: bool, chain: &StyleChain) -> String {
        let Content::SmartQuote(e) = Content::smartquote(double) else { unreachable!() };
        resolve_smart_quote(ctx, &e, chain, None).unwrap().to_string()
    }
    #[test]
    fn semantic_quote_context_observes_objects_numbers_nesting_and_boundaries() {
        let chain = StyleChain::default_chain();
        let mut ctx = SmartQuoteContext::default();
        assert_eq!(emit(&mut ctx, true, &chain), "“");
        ctx.observe_text("outer ");
        assert_eq!(emit(&mut ctx, false, &chain), "‘");
        ctx.observe_text("inner");
        assert_eq!(emit(&mut ctx, false, &chain), "’");
        ctx.observe_text(" tail");
        assert_eq!(emit(&mut ctx, true, &chain), "”");
        ctx.reset_paragraph();
        ctx.observe_object();
        assert_eq!(emit(&mut ctx, false, &chain), "’");
        ctx.reset_paragraph();
        ctx.observe_text("6");
        assert_eq!(emit(&mut ctx, false, &chain), "′");
        ctx.observe_text("2");
        assert_eq!(emit(&mut ctx, true, &chain), "″");
        ctx.reset_paragraph();
        assert_eq!(emit(&mut ctx, true, &chain), "“");
        ctx.observe_text("before\n");
        assert_eq!(emit(&mut ctx, false, &chain), "‘");
        ctx.observe_text("after");
        assert_eq!(emit(&mut ctx, false, &chain), "’");
        assert_eq!(emit(&mut ctx, true, &chain), "”");
    }
    #[test]
    fn semantic_quote_explicit_fields_beat_styles_without_fabricating_presence() {
        use crate::entities::elements::smartquote::SmartQuoteElem;
        let chain = StyleChain::default_chain()
            .push_custom("smartquote.enabled", Value::Bool(false));
        let omitted = SmartQuoteElem {
            double: None,
            enabled: None,
            alternative: None,
            quotes: None,
        };
        let explicit = SmartQuoteElem { enabled: Some(true), ..omitted.clone() };
        assert_ne!(omitted, explicit);
        let mut ctx = SmartQuoteContext::default();
        assert_eq!(resolve_smart_quote(&mut ctx, &omitted, &chain, None).unwrap(), "\"");
        ctx.reset_paragraph();
        assert_eq!(resolve_smart_quote(&mut ctx, &explicit, &chain, None).unwrap(), "“");
        assert!(omitted.double.is_none() && omitted.enabled.is_none());
    }
}
