//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/test_callout.md
//! @prompt-hash f3abeada
//! @layer L1
//! @updated 2026-06-12
//!
//! Lote F-1 (P334) — **fixture** do elemento de utilizador `callout`
//! (ADR-0106; L0 `entities/f_fronteira_e1.md` §"Dois públicos"). **Não é um
//! elemento nativo** (fica fora dos 65; só existe em builds de teste,
//! `#[cfg(test)]`). Demonstra o **caminho Rust**: o programador implementa o
//! **mesmo** `trait Element` dos 65 e ganha `Content::Dynamic` via o blanket
//! `DynElement` — `#set`/`#show`/`query`/render reais chegam em F-2+.
//!
//! `callout { body, title, tone }` — eco do spike `lab/spikes/f-extensao/e1/`
//! (referência de leitura; nada importado de `lab/`).

use ecow::EcoString;

use super::Element;
use crate::entities::content::Content;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Elemento de utilizador de teste: um aviso com corpo, título e tom.
#[derive(Clone, PartialEq, Debug)]
pub struct CalloutElem {
    pub body: Content,
    pub title: EcoString,
    pub tone: EcoString,
}

impl CalloutElem {
    pub fn new(
        body: Content,
        title: impl Into<EcoString>,
        tone: impl Into<EcoString>,
    ) -> Self {
        Self { body, title: title.into(), tone: tone.into() }
    }
}

// `Content` não implementa `Hash` (o hash é por `content_hash::hash_content`
// via Debug); logo `CalloutElem` faz `Hash` **manual via Debug** — a convenção
// do modelo D para campos sem `Hash` (`_comum.md` §A.1.1.b; eco dos Lotes 4+).
impl std::hash::Hash for CalloutElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for CalloutElem {
    fn plain_text(&self) -> String {
        format!("{}: {}", self.title, self.body.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        // Recursa no body; devolve o nó com filhos transformados (o hub aplica
        // `transform` ao nó — mesmo contrato dos 65).
        Ok(Content::dynamic(Self {
            body: self.body.map_content(transform)?,
            ..self.clone()
        }))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::dynamic(Self {
            body: self.body.map_text(transform),
            ..self.clone()
        })
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "title" => Some(Value::Str(self.title.clone())),
            "tone" => Some(Value::Str(self.tone.clone())),
            "body" => Some(Value::Content(self.body.clone())),
            _ => None,
        }
    }

    /// Nome do kind para o match de `#show` (S1; F-2+). O caminho Rust
    /// controla-o (sobrepõe o default `""` dos nativos).
    fn dyn_kind_name(&self) -> &'static str {
        "callout"
    }
}

/// Segundo elemento de utilizador de teste (Lote F-3 inc-2) — kind **distinto**
/// (`"badge"`), para provar que `#show callout:` **não** pega outro kind.
#[derive(Clone, PartialEq, Hash, Debug)]
pub struct BadgeElem {
    pub label: EcoString,
    /// **F-item3 (P368)** — campo **opcional setável** pela chain. `None` =
    /// não-construído → resolve-se de `#set badge(note:)` (mapa aberto) no layout.
    /// Afeta o `plain_text` (logo o output observável), provando a capacidade.
    pub note: Option<EcoString>,
}

impl BadgeElem {
    pub fn new(label: impl Into<EcoString>) -> Self {
        Self { label: label.into(), note: None }
    }
}

impl Element for BadgeElem {
    fn plain_text(&self) -> String {
        match &self.note {
            Some(n) => format!("{} ({})", self.label, n),
            None => self.label.to_string(),
        }
    }
    fn map_content<F>(&self, _t: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::dynamic(self.clone()))
    }
    fn map_text<F>(&self, _t: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::dynamic(self.clone())
    }
    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "label" => Some(Value::Str(self.label.clone())),
            "note" => self.note.clone().map(Value::Str),
            _ => None,
        }
    }
    fn dyn_kind_name(&self) -> &'static str {
        "badge"
    }

    /// F-item3 (P368): `note` opcional resolve-se da chain (`#set badge(note:)`).
    /// Precedência: construído explícito (`Some`) vence; senão lê `get("note")`.
    fn resolve_settable(&self, get: &dyn Fn(&str) -> Option<Value>) -> Content {
        if self.note.is_some() {
            return Content::dynamic(self.clone()); // explícito vence
        }
        let note = match get("note") {
            Some(Value::Str(s)) => Some(s),
            _ => None,
        };
        Content::dynamic(BadgeElem { label: self.label.clone(), note })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn callout() -> CalloutElem {
        CalloutElem::new(Content::text("corpo"), "Aviso", "warn")
    }

    #[test]
    fn plain_text_combina_titulo_e_corpo() {
        assert_eq!(callout().plain_text(), "Aviso: corpo");
    }

    #[test]
    fn get_field_le_campos_declarados() {
        let c = callout();
        assert_eq!(c.get_field("title"), Some(Value::Str("Aviso".into())));
        assert_eq!(c.get_field("tone"), Some(Value::Str("warn".into())));
        assert_eq!(c.get_field("inexistente"), None);
    }

    #[test]
    fn dyn_kind_name_sobrepoe_default() {
        // O caminho Rust controla o id de kind (vs `""` default dos nativos).
        assert_eq!(callout().dyn_kind_name(), "callout");
    }

    #[test]
    fn map_text_recursa_no_body_preservando_titulo() {
        let mapped = callout().map_text(&mut |s| s.to_uppercase());
        assert_eq!(mapped.plain_text(), "Aviso: CORPO");
    }
}
