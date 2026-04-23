//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/func.md
//! @prompt-hash 2e530d45
//! @layer L1
//! @updated 2026-04-13

use std::sync::Arc;

use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::scope::Scope;
use crate::entities::source_result::SourceResult;
use crate::entities::syntax_node::SyntaxNode;
use crate::entities::value::Value;

/// Função Typst — closures definidas no documento e funções nativas (stdlib).
///
/// `Arc<FuncRepr>` — clone O(1), consistente com Module e Func no original.
#[derive(Clone)]
pub struct Func(pub(crate) Arc<FuncRepr>);

pub(crate) enum FuncRepr {
    Closure(ClosureRepr),
    Native(NativeFunc),
}

/// Representação de uma closure Typst.
pub struct ClosureRepr {
    /// Nome da binding — preenchido em eval_let para permitir recursão.
    /// Injectado no call_scope em cada chamada (sem ciclo Arc).
    pub name:     Option<String>,
    /// Parâmetros com nomes e defaults opcionais.
    pub params:   Vec<ClosureParam>,
    /// Corpo da closure — SyntaxNode clone O(1) via Arc interno.
    pub body:     SyntaxNode,
    /// Scope capturado no momento da definição da closure.
    ///
    /// `Arc<Scope>` com snapshot eager (Opção B — DEBT-2):
    /// - Captura: O(N) uma única vez para construir o snapshot
    /// - Partilha: O(1) por Arc::clone em `apply_closure`
    /// - Semântica: snapshot do estado do scope no momento da definição.
    ///   Closures vêem os valores do momento da captura, não da chamada.
    ///
    /// Divergência do original (que usa comemo para lazy access):
    /// registada em DEBT-2. A integração com comemo é trabalho futuro.
    pub captured: Arc<Scope>,
}

/// Um parâmetro de closure com nome e default opcional.
pub struct ClosureParam {
    pub name:    String,
    pub default: Option<Value>,
}

/// Função nativa implementada em Rust (Passo 71 — DEBT-24).
///
/// Recebe `&mut EvalContext` para aceder ao World (ex: leitura de ficheiros).
/// Funções sem I/O usam `_ctx` (prefixo underscore suprime warning).
///
/// Passo 98 (ADR-0036 Regra 1): os campos `current_file` e `figure_numbering`
/// saíram do `EvalContext` e passaram a parâmetros explícitos do ABI.
/// `current_file: FileId` (Copy, cheap) e `figure_numbering: Option<&str>`
/// (read-only). Funções que não os usam prefixam com `_`.
pub struct NativeFunc {
    pub name: &'static str,
    pub call: fn(
        &mut crate::rules::eval::EvalContext<'_>,
        &Args,
        FileId,
        Option<&str>,
    ) -> SourceResult<Value>,
}

impl Func {
    /// Constrói uma Func a partir de uma ClosureRepr.
    pub fn closure(repr: ClosureRepr) -> Self {
        Self(Arc::new(FuncRepr::Closure(repr)))
    }

    /// Constrói uma Func nativa com um function pointer que recebe `EvalContext`.
    pub fn native(
        name: &'static str,
        call: fn(
            &mut crate::rules::eval::EvalContext<'_>,
            &Args,
            FileId,
            Option<&str>,
        ) -> SourceResult<Value>,
    ) -> Self {
        Self(Arc::new(FuncRepr::Native(NativeFunc { name, call })))
    }

    /// Acesso à representação interna (restrito a crate).
    pub(crate) fn repr(&self) -> &FuncRepr {
        &self.0
    }

    /// Retorna o nome da função — `Some(name)` para nativas e closures nomeadas.
    ///
    /// Apenas para apresentação (mensagens de erro, debug). A identidade
    /// para selectors é resolvida por `native_fn_addr()` desde o Passo 84.3
    /// (encerra DEBT-21).
    pub fn name(&self) -> Option<&str> {
        match self.0.as_ref() {
            FuncRepr::Closure(c) => c.name.as_deref(),
            FuncRepr::Native(n)  => Some(n.name),
        }
    }

    /// Endereço da função nativa subjacente como identidade opaca.
    ///
    /// Retorna `Some(fn_ptr)` apenas para `FuncRepr::Native`. Closures
    /// retornam `None` — function pointers de closures não são estáveis
    /// nem comparáveis em Rust, pelo que não servem para selectors.
    ///
    /// Usado pelo motor de show rules (Passo 84.3) para resolver o
    /// `NodeKind` correspondente a uma função nativa via `fn_addr_eq`,
    /// substituindo a comparação por nome (DEBT-21).
    ///
    /// Safe: `fn(...)` é function pointer, não `*const c_void`.
    pub fn native_fn_addr(&self)
        -> Option<fn(
            &mut crate::rules::eval::EvalContext<'_>,
            &Args,
            FileId,
            Option<&str>,
        ) -> SourceResult<Value>>
    {
        match self.0.as_ref() {
            FuncRepr::Native(n)  => Some(n.call),
            FuncRepr::Closure(_) => None,
        }
    }

    /// Define o nome da closure para recursão.
    ///
    /// Usa `Arc::get_mut` — só muta se o Arc tem exatamente uma referência forte.
    /// Se a closure já foi clonada (Arc partilhado), não muta — a referência
    /// recursiva não é necessária se a closure já foi capturada noutro sítio.
    pub fn set_name(&mut self, name: String) {
        if let Some(FuncRepr::Closure(ref mut c)) = Arc::get_mut(&mut self.0) {
            if c.name.is_none() {
                c.name = Some(name);
            }
        }
    }
}

impl std::fmt::Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function>")
    }
}

/// Igualdade por identidade de ponteiro Arc — duas Func são iguais
/// se e só se partilham o mesmo FuncRepr (mesmo objecto).
/// Consistente com Module::PartialEq (Passo 15).
impl PartialEq for Func {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::scope::Scope;
    use crate::entities::source::Source;
    use crate::entities::value::Value;

    fn make_closure() -> Func {
        let source = Source::detached("x + 1");
        let body = source.root().clone();
        Func::closure(ClosureRepr {
            name: None,
            params: vec![ClosureParam { name: "x".into(), default: None }],
            body,
            captured: Arc::new(Scope::new()),
        })
    }

    #[test]
    fn func_debug_nao_panicar() {
        let f = make_closure();
        let s = format!("{:?}", f);
        assert_eq!(s, "<function>");
    }

    #[test]
    fn func_clone_e_ptr_eq() {
        let f1 = make_closure();
        let f2 = f1.clone();
        // Clone partilha o mesmo Arc — ptr_eq é true
        assert_eq!(f1, f2);
    }

    #[test]
    fn dois_closures_distintos_nao_sao_iguais() {
        let f1 = make_closure();
        let f2 = make_closure();
        // Dois Arc distintos — ptr_eq é false
        assert_ne!(f1, f2);
    }

    #[test]
    fn set_name_funciona_em_arc_exclusivo() {
        let mut f = make_closure();
        f.set_name("fact".to_string());
        if let FuncRepr::Closure(c) = f.repr() {
            assert_eq!(c.name, Some("fact".to_string()));
        } else {
            panic!("esperava Closure");
        }
    }

    #[test]
    fn set_name_nao_muta_arc_partilhado() {
        let f1 = make_closure();
        let mut f2 = f1.clone();  // Arc com 2 refs
        f2.set_name("foo".to_string());
        // Arc::get_mut falha — nome permanece None
        if let FuncRepr::Closure(c) = f1.repr() {
            assert_eq!(c.name, None, "Arc partilhado não deve ser mutado");
        }
    }

    #[test]
    fn native_func_debug_nao_panicar() {
        let f = Func::native("type", |_ctx, _args, _cf, _fn| Ok(Value::None));
        assert_eq!(format!("{:?}", f), "<function>");
    }
}
