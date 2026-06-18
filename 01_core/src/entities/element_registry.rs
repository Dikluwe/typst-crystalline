//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/f_fronteira_e1.md
//! @prompt-hash ff6ba172
//! @layer L1
//! @updated 2026-06-12
//!
//! Lote F-1 (P334) — **registro de elementos dinâmicos** (ADR-0106; L0
//! `entities/f_fronteira_e1.md` §3a.5).
//!
//! Mapeia **nome → construtor** para o **caminho typst** (o autor escreve
//! `#callout(...)`; o nome resolve para o construtor registrado, que devolve um
//! `Content::Dynamic`). O **caminho Rust** (o programador implementa `Element` e
//! constrói via `Content::dynamic(...)`) não precisa do registro.
//!
//! **Pureza L1**: o registro é **injetado** — sem `static`/`OnceLock` global
//! (V13). O pipeline (L4) constrói-o e passa-o por parâmetro. `Arc<dyn Fn>` em
//! campo de struct é permitido (ADR-0029).
//!
//! **Escopo F-1**: o tipo + a construção por nome + a trava
//! (`teste-varre-registro`). `#set`/`#show` (que mexem na chain) e a integração
//! no eval real do CLI são **F-2+** (a fronteira fica registada).

use std::collections::HashMap;
use std::sync::Arc;

use ecow::EcoString;

use crate::entities::content::Content;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// Construtor de um elemento dinâmico por nome. Recebe args posicionais e
/// devolve um `Content::Dynamic`. **`Send + Sync`** (Lote F-3 inc-2): o
/// construtor entra num `Value::Func` (via `FuncRepr::Element`) e o `Value`/
/// `Content` vivem em contextos `Send + Sync` (eco da fronteira E1).
pub type ElementCtor = Arc<dyn Fn(&[Value]) -> SourceResult<Content> + Send + Sync>;

/// Registro injetado de elementos dinâmicos (nome → construtor). Sem estado
/// global — instanciado e passado pelo pipeline (pureza L1).
#[derive(Clone, Default)]
pub struct ElementRegistry {
    ctors: HashMap<EcoString, ElementCtor>,
}

impl ElementRegistry {
    /// Registro vazio.
    pub fn new() -> Self {
        Self {
            ctors: HashMap::new(),
        }
    }

    /// Regista um construtor sob `name`. Re-registar o mesmo nome substitui.
    pub fn register(&mut self, name: impl Into<EcoString>, ctor: ElementCtor) {
        self.ctors.insert(name.into(), ctor);
    }

    /// `true` se `name` está registado.
    pub fn is_registered(&self, name: &str) -> bool {
        self.ctors.contains_key(name)
    }

    /// Itera os nomes registados (para a trava `teste-varre-registro`).
    pub fn names(&self) -> impl Iterator<Item = &EcoString> {
        self.ctors.keys()
    }

    /// O construtor de `name` (clone O(1) do `Arc`), para o definir como função
    /// no escopo do eval (Lote F-3 inc-2 — `#name(args)` resolve aqui).
    pub fn ctor(&self, name: &str) -> Option<ElementCtor> {
        self.ctors.get(name).cloned()
    }

    /// Constrói o elemento `name` com `args`. **Elemento desconhecido = erro
    /// declarado** (`SourceResult` Err), **não** panic.
    pub fn construct(&self, name: &str, args: &[Value]) -> SourceResult<Content> {
        match self.ctors.get(name) {
            Some(ctor) => ctor(args),
            None => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("elemento dinâmico desconhecido: `{name}`"),
            )]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::elements::test_callout::CalloutElem;

    /// Constrói um registro de teste com o `callout` da fixture (caminho Rust
    /// → registo → caminho typst). `args = [body, title, tone?]` (Str).
    fn registry_with_callout() -> ElementRegistry {
        let mut reg = ElementRegistry::new();
        reg.register(
            "callout",
            Arc::new(|args: &[Value]| {
                let s = |i: usize| match args.get(i) {
                    Some(Value::Str(s)) => s.clone(),
                    _ => Default::default(),
                };
                Ok(Content::dynamic(CalloutElem::new(
                    Content::text(s(0)),
                    s(1),
                    s(2),
                )))
            }),
        );
        reg
    }

    #[test]
    fn elemento_desconhecido_e_erro_nao_panic() {
        let reg = ElementRegistry::new();
        let r = reg.construct("inexistente", &[]);
        assert!(r.is_err(), "elemento desconhecido deve ser Err, não panic");
    }

    /// A **trava ADR-0105 cláusula 3** (teste-varre-registro): para cada nome
    /// registado, o construtor produz um `Content::Dynamic` que **despacha
    /// pelos 6 métodos do hub** sem panic e faz **round-trip de `get_field`**.
    /// Repõe a verificação que o compilador deixa de dar no caminho dinâmico.
    #[test]
    fn varre_registro_todo_nome_constroi_e_despacha() {
        let reg = registry_with_callout();
        assert!(reg.names().count() >= 1, "registro deve ter ao menos 1 nome");

        for name in reg.names() {
            // body="oi", title="Nota", tone="warn"
            let args = [
                Value::Str("oi".into()),
                Value::Str("Nota".into()),
                Value::Str("warn".into()),
            ];
            let c = reg
                .construct(name, &args)
                .unwrap_or_else(|_| panic!("construct({name}) falhou"));

            assert!(
                matches!(c, Content::Dynamic(_)),
                "construtor de `{name}` deve devolver Content::Dynamic"
            );

            // Os 6 métodos do hub despacham sem panic:
            let _ = c.plain_text();
            let _ = c.is_empty();
            let _ = c.map_text(&mut |s| s.to_string());
            let _ = c.map_content(&mut |_| Ok(None)).expect("map_content");
            assert_eq!(c, c.clone(), "eq/clone do nó dinâmico");

            // Round-trip de get_field (S7): o campo declarado lê de volta.
            assert_eq!(
                c.get_field("title"),
                Some(Value::Str("Nota".into())),
                "get_field('title') round-trip"
            );
        }
    }
}
