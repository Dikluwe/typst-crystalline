//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content/fields.md
//! @prompt-hash 00d48209
//! @layer L1
//! @updated 2026-09-26

use super::*;

fn styled_equation(content: &Content) -> Option<&EquationElem> {
    match content {
        Content::Equation(e) => Some(e),
        Content::Styled(styled) => styled_equation(&styled.body),
        _ => None,
    }
}

impl Content {
    pub fn get_field(&self, field: &str) -> Option<crate::entities::value::Value> {
        use crate::entities::value::Value;
        match (self, field) {
            // Modelo D (P316): Heading delega ao elemento.
            (Content::Heading(h), f) => h.get_field(f),
            (Content::OutlineEntry(e), f) => e.get_field(f),
            // P765a: Title delega ao elemento.
            (Content::Title(t), f) => t.get_field(f),
            // F-5b fatia 1 (P371): strong/emph delegam (ex.: `it.body`).
            (Content::Strong(e), f) => e.get_field(f),
            (Content::Emph(e), f) => e.get_field(f),
            (Content::Linebreak(e), f) => e.get_field(f),
            (Content::Figure(e), "body") => Some(Value::Content(e.body.clone())),
            (Content::Equation(e), f) => e.get_field(f),
            (Content::MathPrimes(e), f) => e.get_field(f),
            (Content::MathLr(e), f) => e.get_field(f),
            (Content::MathUnderbrace(e), f) => e.get_field(f),
            (Content::MathUnderbracket(e), f) => e.get_field(f),
            (Content::MathUnderparen(e), f) => e.get_field(f),
            (Content::MathUndershell(e), f) => e.get_field(f),
            (Content::MathStretch(e), f) => e.get_field(f),
            (Content::PdfArtifact(e), f) => e.get_field(f),
            (Content::Styled(styled), f) if styled_equation(&styled.body).is_some() => {
                let child = &styled.body;
                let styles = &styled.styles;
                let key = match f {
                    "numbering" => Some("equation.numbering"),
                    "number-align" => Some("equation.number-align"),
                    "supplement" => Some("equation.supplement"),
                    "alt" => Some("equation.alt"),
                    _ => None,
                };
                key.and_then(|key| {
                    styles
                        .delta()
                        .custom
                        .iter()
                        .rev()
                        .find(|(candidate, _)| candidate.as_str() == key)
                        .map(|(_, value)| value.clone())
                })
                .or_else(|| child.get_field(f))
            }
            // P408: smallcaps expõe `body` para show rules (`it.body`).
            (Content::SmallCaps { body }, "body") => {
                Some(Value::Content(body.as_ref().clone()))
            }
            // P863: parágrafo expõe `body` para show rules.
            (Content::Par { body }, "body") => {
                Some(Value::Content(body.as_ref().clone()))
            }
            // Lote F-1 (P334): leitura de campos da fronteira dinâmica (S7) —
            // o que o closure de `#show` usará (F-2+).
            (Content::Dynamic(e), f) => e.dyn_get_field(f),
            // **P844** (achado #47 de P831) — `query(<meta>).first().value`
            // (paridade vanilla `MetadataElem.value`).
            (Content::Metadata(e), "value") => Some(e.value.as_ref().clone()),
            _ => None, // neutro: N16[β] — variantes de Content sem campos expostos retornam None em get_field
        }
    }
}

#[cfg(test)]
mod fields_tests {
    use super::Content;

    #[test]
    fn unsupported_field_is_absent() {
        assert_eq!(Content::Empty.get_field("missing"), None);
    }
}
