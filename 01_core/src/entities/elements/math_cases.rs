//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/elements/math_cases.md
//! @prompt-hash 7f505e68
//! @layer L1
//! @updated 2026-10-10
//!
//! `MathCasesElem` — Lote 2 P317 (família math). Função por ramos `cases(...)`.
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source_result::SourceResult;

/// Presença morfológica dos argumentos nomeados de `math.cases`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MathCasesExplicit {
    pub delim: bool,
    pub reverse: bool,
    pub gap: bool,
}

/// Função definida por ramos (`cases(...)`). Cada filho é um ramo e conserva
/// os seus pontos de alinhamento até ao consumer de layout.
#[derive(Debug, Clone, PartialEq)]
pub struct MathCasesElem {
    pub children: Vec<Content>,
    pub delim: (Option<char>, Option<char>),
    pub reverse: bool,
    pub gap: Rel<Length>,
    pub explicit: MathCasesExplicit,
}

impl Hash for MathCasesElem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        fn bits(value: f64) -> u64 {
            if value == 0.0 {
                0
            } else {
                value.to_bits()
            }
        }
        self.children.hash(state);
        self.delim.hash(state);
        self.reverse.hash(state);
        bits(self.gap.rel).hash(state);
        bits(self.gap.abs.abs.0).hash(state);
        bits(self.gap.abs.em).hash(state);
        self.explicit.hash(state);
    }
}

impl Element for MathCasesElem {
    fn get_field(&self, field: &str) -> Option<crate::entities::value::Value> {
        use crate::entities::value::Value;
        match field {
            "children" => Some(Value::Array(
                self.children.iter().cloned().map(Value::Content).collect(),
            )),
            "delim" if self.explicit.delim => Some(Value::Array(
                [self.delim.0, self.delim.1]
                    .into_iter()
                    .map(|c| {
                        c.map(|c| Value::Str(c.to_string().into())).unwrap_or(Value::None)
                    })
                    .collect(),
            )),
            "reverse" if self.explicit.reverse => Some(Value::Bool(self.reverse)),
            "gap" if self.explicit.gap => Some(Value::Relative(self.gap)),
            _ => None,
        }
    }

    fn plain_text(&self) -> String {
        self.children
            .iter()
            .map(Content::plain_text)
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathCases(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathCases(Arc::new(self.clone()))
    }
}

impl MathCasesElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::map_children_if_changed(&self.children, transform)?.map(|children| {
            Content::MathCases(Arc::new(Self {
                children,
                delim: self.delim,
                reverse: self.reverse,
                gap: self.gap,
                explicit: self.explicit,
            }))
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathCasesElem {
        MathCasesElem {
            children: vec![Content::text("ab"), Content::text("c")],
            delim: (Some('{'), Some('}')),
            reverse: false,
            gap: Rel { rel: 0.0, abs: Length::em(0.2) },
            explicit: MathCasesExplicit::default(),
        }
    }

    #[test]
    fn plain_text_celulas_e_ramos() {
        assert_eq!(ex().plain_text(), "ab, c");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.children.pop();
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_grelha() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let r = ex().map_content(&mut f).unwrap();
        match r {
            Content::MathCases(e) => assert_eq!(e.plain_text(), "ab, c"),
            _ => panic!("esperado MathCases"),
        }
    }
}
