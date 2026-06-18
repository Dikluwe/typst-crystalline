//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/f_fronteira_e1.md
//! @prompt-hash ff6ba172
//! @layer L1
//! @updated 2026-06-12
//!
//! Lote F-1 (P334) — a **fronteira de extensão E1** (ADR-0106).
//!
//! `DynElement` é a versão **object-safe** do trait [`Element`], para a porta
//! de extensão `Content::Dynamic(Arc<dyn DynElement>)`. **Ninguém a implementa
//! à mão**: o blanket abaixo deriva-a de qualquer `Element + 'static`. O
//! utilizador escreve só `impl Element` (o **mesmo** trait dos 65 nativos — o
//! precedente vivo, o menor custo-IA da tabela P332). `DynElement` é maquinaria
//! invisível.
//!
//! **Object-safety**: os métodos genéricos `map_*<F>` do `Element` (que tornam
//! `Element` não-object-safe) viram `map_*_dyn(&mut dyn FnMut…)`;
//! `Clone`/`PartialEq`/`Hash` saem dos supertraits e voltam como:
//! - `Clone`: o `Arc<dyn DynElement>` clona o ponteiro (O(1)) — não precisa.
//! - `PartialEq`: `dyn_eq` + `as_any` (o `eq` do hub É um match).
//! - `Hash`: **nada** — `content_hash::hash_content` serializa por
//!   `format!("{:?}")` (Debug), logo o `Debug` do `dyn` basta (ajuste de Fase A
//!   P334; ver L0 §0/§3a.3).

use std::any::Any;

use super::Element;
use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Versão object-safe de [`Element`] — ver módulo. Implementada pelo blanket;
/// o utilizador nunca a escreve.
///
/// **Nomes `dyn_*` distintos dos de `Element` de propósito**: o blanket torna
/// **todo** `Element` também `DynElement`; se os nomes coincidissem, qualquer
/// `elem.is_empty()` num tipo concreto (onde os dois traits estão em scope)
/// seria ambíguo (E0034). Com nomes distintos, o caminho nativo usa
/// `Element::*` e a folha dinâmica usa `DynElement::dyn_*` sem colisão.
///
/// `Send + Sync`: o `Content` vive em contextos `Send + Sync` (introspector);
/// logo `dyn DynElement` (e o `Arc` que o embrulha) têm de o ser.
pub trait DynElement: std::fmt::Debug + Send + Sync + 'static {
    fn dyn_plain_text(&self) -> String;
    fn dyn_is_empty(&self) -> bool;
    fn dyn_map_content(
        &self,
        f: &mut dyn FnMut(&Content) -> SourceResult<Option<Content>>,
    ) -> SourceResult<Content>;
    fn dyn_map_text(&self, f: &mut dyn FnMut(&str) -> String) -> Content;
    fn dyn_get_field(&self, field: &str) -> Option<Value>;
    fn dyn_element_kind(&self) -> Option<ElementKind>;
    fn dyn_to_payload(&self) -> Option<ElementPayload>;
    /// Id estável do kind (S1) — match de seletor `#show` (F-2+). Vem de
    /// `Element::dyn_kind_name` (que o utilizador sobrepõe).
    fn dyn_kind(&self) -> &'static str;
    /// **F-item3 (P368)** — resolve campos setáveis da chain (mapa aberto). Bridge
    /// object-safe de `Element::resolve_settable`. `get(prop)` = `#set <kind>(prop:)`.
    fn dyn_resolve_settable(&self, get: &dyn Fn(&str) -> Option<Value>) -> Content;
    /// Downcast para o `eq` do hub (que é um match) — ver `dyn_eq`.
    fn as_any(&self) -> &dyn Any;
    /// Igualdade object-safe: downcast ao tipo concreto e compara
    /// estruturalmente. Tipos diferentes ⇒ `false`.
    fn dyn_eq(&self, other: &dyn DynElement) -> bool;
}

/// Blanket: **todo** `Element + Send + Sync + 'static` é um `DynElement`. Bridga
/// os métodos genéricos `map_*<F>` para `dyn_map_*(&mut dyn FnMut…)` sem custo no
/// caminho dos nativos (que nunca passam por aqui — são despachados
/// estaticamente pelo `match Content`).
impl<T: Element + Send + Sync + 'static> DynElement for T {
    fn dyn_plain_text(&self) -> String {
        Element::plain_text(self)
    }

    fn dyn_is_empty(&self) -> bool {
        Element::is_empty(self)
    }

    fn dyn_map_content(
        &self,
        f: &mut dyn FnMut(&Content) -> SourceResult<Option<Content>>,
    ) -> SourceResult<Content> {
        // `&mut dyn FnMut` não satisfaz `F: Sized` directamente; reborrow num
        // binding local torna `F = &mut dyn FnMut` (que É Sized e FnMut).
        let mut g = f;
        Element::map_content(self, &mut g)
    }

    fn dyn_map_text(&self, f: &mut dyn FnMut(&str) -> String) -> Content {
        let mut g = f;
        Element::map_text(self, &mut g)
    }

    fn dyn_get_field(&self, field: &str) -> Option<Value> {
        Element::get_field(self, field)
    }

    fn dyn_element_kind(&self) -> Option<ElementKind> {
        Element::element_kind(self)
    }

    fn dyn_to_payload(&self) -> Option<ElementPayload> {
        Element::to_payload(self)
    }

    fn dyn_kind(&self) -> &'static str {
        Element::dyn_kind_name(self)
    }

    fn dyn_resolve_settable(&self, get: &dyn Fn(&str) -> Option<Value>) -> Content {
        Element::resolve_settable(self, get)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_eq(&self, other: &dyn DynElement) -> bool {
        other.as_any().downcast_ref::<T>().is_some_and(|o| self == o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::entities::elements::divider::DividerElem;
    use crate::entities::elements::test_callout::CalloutElem;

    fn callout() -> CalloutElem {
        CalloutElem::new(Content::text("oi"), "Nota", "warn")
    }

    #[test]
    fn arc_dyn_element_construivel_e_despacha() {
        // Object-safety: `Arc<dyn DynElement>` existe e os métodos despacham.
        let e: Arc<dyn DynElement> = Arc::new(callout());
        assert_eq!(e.dyn_plain_text(), "Nota: oi");
        assert!(!e.dyn_is_empty());
        assert_eq!(e.dyn_kind(), "callout");
        assert_eq!(e.dyn_get_field("tone"), Some(Value::Str("warn".into())));
        assert_eq!(e.dyn_element_kind(), None);
        assert_eq!(e.dyn_to_payload(), None);
    }

    #[test]
    fn blanket_nativo_atravessa_dynelement_igual_ao_estatico() {
        // Um nativo qualquer (DividerElem) atravessa o `DynElement` (blanket)
        // com o **mesmo** resultado do caminho estático `Element`.
        let d = DividerElem;
        assert_eq!(DynElement::dyn_plain_text(&d), Element::plain_text(&d));
        assert_eq!(DynElement::dyn_is_empty(&d), Element::is_empty(&d));
        // Nativo: `dyn_kind` fica no default `""` (despacho estático).
        assert_eq!(DynElement::dyn_kind(&d), "");
    }

    #[test]
    fn dyn_eq_estrutural_por_downcast() {
        let a: Arc<dyn DynElement> = Arc::new(callout());
        let b: Arc<dyn DynElement> = Arc::new(callout());
        let c: Arc<dyn DynElement> =
            Arc::new(CalloutElem::new(Content::text("x"), "T", "info"));
        assert!(a.dyn_eq(b.as_ref()), "callouts iguais");
        assert!(!a.dyn_eq(c.as_ref()), "callouts diferentes");
        // Tipo diferente ⇒ sempre `false` (downcast falha).
        let n: Arc<dyn DynElement> = Arc::new(DividerElem);
        assert!(!a.dyn_eq(n.as_ref()), "tipos diferentes nunca iguais");
    }

    #[test]
    fn hub_eq_dispatch_para_dynamic() {
        // O arm `eq` do hub (Content::PartialEq) despacha `dyn_eq`.
        assert_eq!(Content::dynamic(callout()), Content::dynamic(callout()));
        assert_ne!(
            Content::dynamic(callout()),
            Content::dynamic(CalloutElem::new(Content::text("x"), "T", "info"))
        );
    }

    #[test]
    fn map_content_dyn_recursa_no_body() {
        // `map_content_dyn` aplica o transform aos filhos (body), não a si.
        let e: Arc<dyn DynElement> = Arc::new(callout());
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            if c.plain_text() == "oi" {
                Ok(Some(Content::text("trocado")))
            } else {
                Ok(None)
            }
        };
        let mapped = e.dyn_map_content(&mut f).unwrap();
        assert_eq!(mapped.plain_text(), "Nota: trocado");
    }

    #[test]
    fn hub_map_content_aplica_transform_ao_no_dinamico() {
        // Pelo hub (Content::map_content): o transform também vê o nó dinâmico.
        let c = Content::dynamic(callout());
        let mut f = |x: &Content| -> SourceResult<Option<Content>> {
            // substitui o nó dinâmico inteiro por um texto marcador.
            if matches!(x, Content::Dynamic(_)) {
                Ok(Some(Content::text("substituido")))
            } else {
                Ok(None)
            }
        };
        assert_eq!(c.map_content(&mut f).unwrap().plain_text(), "substituido");
    }
}
