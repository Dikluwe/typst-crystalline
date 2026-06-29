//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/enum_item.md
//! @prompt-hash 30f3050a
//! @layer L1
//! @updated 2026-06-29
//!
//! `EnumItemElem` — Lote 3 P318 (família lista/termos). Item de lista ordenada.
//! Campo `numbering` adicionado em P470. Contentor de prosa: recurse em
//! map_content E map_text (precedente Heading).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::enum_numbering::EnumNumbering;
use crate::entities::layout_types::Length;
use crate::entities::source_result::SourceResult;

/// Item de lista ordenada (`+ ...` ou `1. ...`).
#[derive(Debug, Clone, PartialEq)]
pub struct EnumItemElem {
    pub number:      Option<u32>,
    pub body:        Content,
    pub numbering:   Option<EnumNumbering>,  // None = Decimal default
    /// **P505** — indentação do rótulo numérico em relação à margem esquerda.
    pub indent:      Option<Length>,
    /// **P505** — indentação do corpo do item em relação ao rótulo.
    pub body_indent: Option<Length>,
    /// **P505** — `false` adiciona espaçamento de parágrafo entre itens.
    pub tight:       Option<bool>,
}

// `Hash` manual via `Debug` (paridade `content_hash::hash_content`).
// CAUSA: `Length` carrega `f64` e não implementa `Hash`; o trait `Element`
// exige `Hash`, logo `#[derive(Hash)]` não compila.
// RESSALVA: hash-via-Debug pode violar Hash/Eq para `-0.0` vs `0.0`
// (PartialEq-iguais, Debug distinto). Aceitável: regime do `content_hash`
// pré-existente e os `…Elem` NÃO são chaves de mapa. Revisitar se vier a sê-lo.
impl std::hash::Hash for EnumItemElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for EnumItemElem {
    fn plain_text(&self) -> String {
        let label = match self.number {
            Some(n) => {
                let scheme = self.numbering.as_ref().unwrap_or(&EnumNumbering::Decimal);
                format!("{} ", scheme.format(n))
            }
            None => String::new(),
        };
        format!("{}{}", label, self.body.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::EnumItem(Arc::new(EnumItemElem {
            number:      self.number,
            body:        self.body.map_content(transform)?,
            numbering:   self.numbering.clone(),
            indent:      self.indent.clone(),
            body_indent: self.body_indent.clone(),
            tight:       self.tight,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::EnumItem(Arc::new(EnumItemElem {
            number:      self.number,
            body:        self.body.map_text(transform),
            numbering:   self.numbering.clone(),
            indent:      self.indent.clone(),
            body_indent: self.body_indent.clone(),
            tight:       self.tight,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_com_e_sem_numero_decimal() {
        let com = EnumItemElem {
            number: Some(3),
            body: Content::text("x"),
            numbering: None,
            indent: None,
            body_indent: None,
            tight: None,
        };
        assert_eq!(com.plain_text(), "3. x");
        let sem = EnumItemElem {
            number: None,
            body: Content::text("x"),
            numbering: None,
            indent: None,
            body_indent: None,
            tight: None,
        };
        assert_eq!(sem.plain_text(), "x");
    }

    #[test]
    fn plain_text_com_lower_alpha() {
        let e = EnumItemElem {
            number:      Some(1),
            body:        Content::text("a"),
            numbering:   Some(EnumNumbering::LowerAlpha),
            indent:      None,
            body_indent: None,
            tight:       None,
        };
        assert_eq!(e.plain_text(), "a) a");
    }

    #[test]
    fn map_content_preserva_number_numbering_e_indentacao() {
        let e = EnumItemElem {
            number:      Some(7),
            body:        Content::text("a"),
            numbering:   Some(EnumNumbering::UpperAlpha),
            indent:      Some(Length::em(1.5)),
            body_indent: Some(Length::em(0.5)),
            tight:       Some(false),
        };
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        match e.map_content(&mut f).unwrap() {
            Content::EnumItem(el) => {
                assert_eq!(el.number, Some(7));
                assert_eq!(el.numbering, Some(EnumNumbering::UpperAlpha));
                assert_eq!(el.indent, Some(Length::em(1.5)));
                assert_eq!(el.body_indent, Some(Length::em(0.5)));
                assert_eq!(el.tight, Some(false));
            }
            _ => panic!("esperado EnumItem"),
        }
    }

    #[test]
    fn igualdade_estrutural() {
        let a = EnumItemElem {
            number: Some(1),
            body: Content::text("x"),
            numbering: None,
            indent: None,
            body_indent: None,
            tight: None,
        };
        assert_eq!(a.clone(), a.clone());
        assert_ne!(
            a,
            EnumItemElem {
                number: Some(2),
                body: Content::text("x"),
                numbering: None,
                indent: None,
                body_indent: None,
                tight: None,
            }
        );
        assert_ne!(
            a,
            EnumItemElem {
                number:      Some(1),
                body:        Content::text("x"),
                numbering:   Some(EnumNumbering::LowerAlpha),
                indent:      None,
                body_indent: None,
                tight:       None,
            }
        );
    }
}
