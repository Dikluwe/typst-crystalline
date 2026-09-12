//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/func.md
//! @prompt-hash e6921e0f
//! @layer L1
//! @updated 2026-04-13

use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::scope::{Capturer, Scope};
use crate::entities::source_result::SourceResult;
use crate::entities::span::Span;
use crate::entities::syntax_node::SyntaxNode;
use crate::entities::value::Value;

/// Função Typst — closures definidas no documento e funções nativas (stdlib).
///
/// `Arc<FuncRepr>` — clone O(1), consistente com Module e Func no original.
#[derive(Clone)]
pub struct Func(pub(crate) Arc<FuncRepr>, Span);

pub(crate) enum FuncRepr {
    Closure(ClosureRepr),
    Native(NativeFunc),
    /// **P394** — variante de native function com acesso ao `Scopes` e `Engine`
    /// actuais. Usada por `eval(source)` para re-avaliar código Typst no contexto
    /// corrente. O ABI geral das nativas permanece inalterado.
    NativeWithEngine(NativeFuncWithEngine),
    /// **Lote F-3 inc-2** — construtor de elemento de utilizador (fronteira E1).
    /// `#callout(args)` resolve para isto (definido no escopo a partir do
    /// `ElementRegistry`); `apply_func` invoca o `ctor` e devolve
    /// `Value::Content(Content::Dynamic(...))`. Caminho único com os nativos.
    Element(ElementFunc),
    /// **P699** — função de plugin WASM. O nome do export é dinâmico (vem do
    /// módulo), logo não cabe num fn-ptr nativo; a chamada é delegada ao
    /// `PluginHost` capturado em `PluginFunc`. Ver `entities/plugin_func.rs`.
    Plugin(crate::entities::plugin_func::PluginFunc),
    /// **P702** — aplicação parcial de argumentos (`f.with(...)`). Guarda a
    /// função original e os `Args` pré-ligados; a chamada final funde-os com
    /// os novos args e delega (`apply_func` em `rules/eval/closures.rs`).
    With(Arc<(Func, Args)>),
}

/// Construtor de elemento de utilizador no escopo do eval (Lote F-3 inc-2).
pub struct ElementFunc {
    pub name: String,
    pub ctor: crate::entities::element_registry::ElementCtor,
}

/// Representação de uma closure Typst.
pub struct ClosureRepr {
    /// Nome da binding — preenchido em eval_let para permitir recursão.
    /// Injectado no call_scope em cada chamada (sem ciclo Arc).
    pub name: Option<String>,
    /// Parâmetros com nomes e defaults opcionais.
    pub params: Vec<ClosureParam>,
    /// Nome do sink de argumentos (`..args`). Se `Some`, todos os args não
    /// consumidos por `params` são empacotados num `Value::Args` e ligados a
    /// este nome no scope da chamada (P504).
    pub sink_name: Option<String>,
    /// Corpo da closure — SyntaxNode clone O(1) via Arc interno.
    pub body: SyntaxNode,
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
    /// **P772q** — por que motivo este scope foi capturado: closure normal
    /// (`Expr::Closure`) ou bloco `context { }` (`Expr::Contextual`).
    /// Decidido na definição, não na chamada — paridade vanilla
    /// `CapturesVisitor::new(scopes, capturer)`. Propagado por
    /// `apply_closure` para `Scopes::with_parent`, usado só para a
    /// mensagem de erro ao mutar um nome capturado.
    pub capturer: Capturer,
}

/// Um parâmetro de closure com nome e default opcional.
pub struct ClosureParam {
    pub name: String,
    pub default: Option<Value>,
    /// **P724** — pattern completo de `Param::Pos` não-`Ident`
    /// (destructuring, parenthesized, placeholder): `SyntaxNode` owned
    /// (clone O(1) via Arc interno), reparseado na chamada via
    /// `Pattern::from_untyped` e ligado por `destructure_let` — o mesmo
    /// mecanismo do `body` (`Expr::from_untyped`). `None` para `Ident`
    /// posicional e para `Param::Named`. Quando `Some`, `name` é `""`
    /// (nunca consultado: named lookup com chave vazia não casa) e
    /// `default` é `None` (pattern posicional nunca tem default).
    pub pattern: Option<SyntaxNode>,
}

/// Função nativa implementada em Rust (Passo 71 — DEBT-24).
///
/// Recebe `&dyn World` para aceder a I/O (ex: leitura de ficheiros).
/// Funções sem I/O usam `_world` (prefixo underscore suprime warning).
///
/// Passo 98 (ADR-0036 Regra 1): `current_file` passou a parâmetro explícito do ABI.
/// Passo 109 (ADR-0044): `world` deixou de estar em `EvalContext` (agora em
/// `Engine`); para manter native functions desacopladas do `Engine`, o
/// `world` entra directamente no ABI como parâmetro extra.
/// **F-5a de-bake (P365):** o parâmetro `figure_numbering: Option<&str>` foi
/// **removido** do ABI — era a fonte assada do `#set figure(numbering:)`, agora
/// vive só na chain (`custom("figure.numbering")`). Morto em todas as natives após
/// o de-bake → colapsado (disciplina anti-morto).
/// **P493** — namespace anexado para sub-funções via field access (`table.header`).
pub struct NativeFunc {
    pub name: &'static str,
    pub call: fn(
        &mut crate::compiler::eval::EvalContext,
        &Args,
        &dyn crate::contracts::world::World,
        FileId,
    ) -> SourceResult<Value>,
    pub namespace: Option<Arc<Scope>>,
}

/// **P394** — native function com acesso ao `Scopes` e `Engine` actuais.
/// Usada por `eval(source)` para re-avaliar código Typst no contexto de chamada.
/// **P493** — namespace anexado para sub-funções via field access.
pub struct NativeFuncWithEngine {
    pub name: &'static str,
    pub call: fn(
        &mut crate::compiler::eval::EvalContext,
        &Args,
        &dyn crate::contracts::world::World,
        FileId,
        &mut crate::compiler::scopes::Scopes<'_>,
        &mut crate::entities::engine::Engine<'_>,
    ) -> SourceResult<Value>,
    pub namespace: Option<Arc<Scope>>,
}

impl Func {
    /// Constrói uma Func a partir de uma ClosureRepr.
    pub fn closure(repr: ClosureRepr) -> Self {
        Self(Arc::new(FuncRepr::Closure(repr)), Span::detached())
    }

    /// Constrói uma Func nativa com um function pointer que recebe
    /// `EvalContext`, `World`, `FileId` (F-5a P365: `figure_numbering` removido).
    pub fn native(
        name: &'static str,
        call: fn(
            &mut crate::compiler::eval::EvalContext,
            &Args,
            &dyn crate::contracts::world::World,
            FileId,
        ) -> SourceResult<Value>,
    ) -> Self {
        Self(
            Arc::new(FuncRepr::Native(NativeFunc { name, call, namespace: None })),
            Span::detached(),
        )
    }

    /// **P493** — constrói uma Func nativa com namespace anexado.
    pub fn native_with_namespace(
        name: &'static str,
        call: fn(
            &mut crate::compiler::eval::EvalContext,
            &Args,
            &dyn crate::contracts::world::World,
            FileId,
        ) -> SourceResult<Value>,
        namespace: Arc<Scope>,
    ) -> Self {
        Self(
            Arc::new(FuncRepr::Native(NativeFunc {
                name,
                call,
                namespace: Some(namespace),
            })),
            Span::detached(),
        )
    }

    /// **P394** — constrói uma Func nativa com acesso ao `Scopes` e `Engine`
    /// actuais. Usada por `eval(source)`.
    pub fn native_with_engine(
        name: &'static str,
        call: fn(
            &mut crate::compiler::eval::EvalContext,
            &Args,
            &dyn crate::contracts::world::World,
            FileId,
            &mut crate::compiler::scopes::Scopes<'_>,
            &mut crate::entities::engine::Engine<'_>,
        ) -> SourceResult<Value>,
    ) -> Self {
        Self(
            Arc::new(FuncRepr::NativeWithEngine(NativeFuncWithEngine {
                name,
                call,
                namespace: None,
            })),
            Span::detached(),
        )
    }

    /// **P493** — constrói uma Func nativa com acesso ao `Scopes`/`Engine` e
    /// namespace anexado.
    pub fn native_with_engine_and_namespace(
        name: &'static str,
        call: fn(
            &mut crate::compiler::eval::EvalContext,
            &Args,
            &dyn crate::contracts::world::World,
            FileId,
            &mut crate::compiler::scopes::Scopes<'_>,
            &mut crate::entities::engine::Engine<'_>,
        ) -> SourceResult<Value>,
        namespace: Arc<Scope>,
    ) -> Self {
        Self(
            Arc::new(FuncRepr::NativeWithEngine(NativeFuncWithEngine {
                name,
                call,
                namespace: Some(namespace),
            })),
            Span::detached(),
        )
    }

    /// Constrói uma Func de elemento de utilizador (Lote F-3 inc-2) — `#name(args)`
    /// invoca `ctor` e devolve `Content::Dynamic`.
    pub fn element(
        name: impl Into<String>,
        ctor: crate::entities::element_registry::ElementCtor,
    ) -> Self {
        Self(
            Arc::new(FuncRepr::Element(ElementFunc { name: name.into(), ctor })),
            Span::detached(),
        )
    }

    /// **P699** — Constrói uma Func de plugin WASM a partir de um `PluginFunc`
    /// (export de módulo). Chamada delegada ao `PluginHost` em `apply_func`.
    pub fn plugin(p: crate::entities::plugin_func::PluginFunc) -> Self {
        Self(Arc::new(FuncRepr::Plugin(p)), Span::detached())
    }

    /// **P702** — `f.with(args)`: devolve nova `Func` com `args` pré-ligados.
    /// A fusão com os args da chamada final acontece em `apply_func`
    /// (`rules/eval/closures.rs`), não aqui.
    pub fn with(self, args: Args) -> Self {
        let span = self.1;
        Self(Arc::new(FuncRepr::With(Arc::new((self, args)))), span)
    }

    /// Origem diagnóstica transportada sem participar da identidade da função.
    pub(crate) fn diagnostic_span(&self) -> Span {
        self.1
    }

    /// Conserva a primeira origem conhecida, inclusive através de aliases.
    pub(crate) fn with_diagnostic_span(mut self, span: Span) -> Self {
        if self.1.is_detached() {
            self.1 = span;
        }
        self
    }

    /// Acesso à representação interna (restrito a crate).
    pub(crate) fn repr(&self) -> &FuncRepr {
        &self.0
    }

    /// Lote F-3 inc-2 — `Some(kind)` se esta Func é um construtor de elemento de
    /// utilizador (`FuncRepr::Element`); usado pelo `#show` para construir
    /// `Selector::DynKind`. `None` para closures/nativas.
    pub fn element_name(&self) -> Option<&str> {
        match self.0.as_ref() {
            FuncRepr::Element(e) => Some(&e.name),
            _ => None,
        }
    }

    /// Retorna o nome da função — `Some(name)` para nativas e closures nomeadas.
    ///
    /// Apenas para apresentação (mensagens de erro, debug). A identidade
    /// para selectors é resolvida por `native_fn_addr()` desde o Passo 84.3
    /// (encerra DEBT-21).
    pub fn name(&self) -> Option<&str> {
        match self.0.as_ref() {
            FuncRepr::Closure(c) => c.name.as_deref(),
            FuncRepr::Native(n) => Some(n.name),
            FuncRepr::NativeWithEngine(n) => Some(n.name),
            FuncRepr::Element(e) => Some(&e.name),
            FuncRepr::Plugin(p) => Some(p.name.as_str()),
            // P702 — delega ao nome da função interna (paridade vanilla,
            // `FuncInner::With(with) => with.0.name()`).
            FuncRepr::With(w) => w.0.name(),
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
    pub fn native_fn_addr(
        &self,
    ) -> Option<
        fn(
            &mut crate::compiler::eval::EvalContext,
            &Args,
            &dyn crate::contracts::world::World,
            FileId,
        ) -> SourceResult<Value>,
    > {
        match self.0.as_ref() {
            FuncRepr::Native(n) => Some(n.call),
            FuncRepr::Closure(_) => None,
            // P394: assinatura diferente; não comparável com nativas normais.
            FuncRepr::NativeWithEngine(_) => None,
            // Elemento de utilizador não tem fn-ptr nativo — o selector de
            // `#show` casa-o por **kind dinâmico** (S2), não por endereço.
            FuncRepr::Element(_) => None,
            // P699 — plugin WASM não tem fn-ptr nativo (chamada via host).
            FuncRepr::Plugin(_) => None,
            // P702 — aplicação parcial não é directamente um fn-ptr nativo
            // (o `apply_func` já resolve `With` antes de chegar aqui).
            FuncRepr::With(_) => None,
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

    /// **P493** — retorna o namespace anexado à função nativa, se existir.
    pub fn namespace(&self) -> Option<&Scope> {
        match self.0.as_ref() {
            FuncRepr::Native(n) => n.namespace.as_deref(),
            FuncRepr::NativeWithEngine(n) => n.namespace.as_deref(),
            // P702 — delega à função interna. Medido contra o vanilla (não
            // assumido): `table.with(columns: 2).cell` compila e devolve
            // `function` (`foundations/func.rs:275`,
            // `FuncInner::With(with) => with.0.scope()`).
            FuncRepr::With(w) => w.0.namespace(),
            _ => None,
        }
    }
}

impl std::fmt::Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function>")
    }
}

/// Igualdade: identidade de ponteiro Arc **ou**, para nativas, igualdade
/// de nome dentro do mesmo kind.
///
/// **P742** — medição vanilla 0.15.0: `color.rgb == rgb` → `true`,
/// `red.space() == rgb` → `true`. O vanilla materializa nativas como
/// singletons estáticos (identidade); o cristalino cria Arcs frescos por
/// lookup, logo a identidade equivalente é o nome. Closures, `Element`,
/// `With` e `Plugin` mantêm identidade de ponteiro. Consistente com
/// `Module::PartialEq` (Passo 15) para os casos não-nativos.
impl PartialEq for Func {
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(&self.0, &other.0) {
            return true;
        }
        match (self.0.as_ref(), other.0.as_ref()) {
            (FuncRepr::Native(a), FuncRepr::Native(b)) => a.name == b.name,
            (FuncRepr::NativeWithEngine(a), FuncRepr::NativeWithEngine(b)) => {
                a.name == b.name
            }
            _other => false, // neutro: N16[β] — funções com representações internas diferentes são desiguais,
        }
    }
}

impl Eq for Func {}

impl Hash for Func {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.0.as_ref() {
            FuncRepr::Native(native) => {
                0u8.hash(state);
                native.name.hash(state);
            }
            FuncRepr::NativeWithEngine(native) => {
                1u8.hash(state);
                native.name.hash(state);
            }
            _ => {
                2u8.hash(state);
                Arc::as_ptr(&self.0).hash(state);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::scope::{Capturer, Scope};
    use crate::entities::source::Source;
    use crate::entities::value::Value;

    fn make_closure() -> Func {
        let source = Source::detached("x + 1");
        let body = source.root().clone();
        Func::closure(ClosureRepr {
            name: None,
            params: vec![ClosureParam { name: "x".into(), default: None, pattern: None }],
            sink_name: None,
            body,
            captured: Arc::new(Scope::new()),
            capturer: Capturer::Function,
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
        let mut f2 = f1.clone(); // Arc com 2 refs
        f2.set_name("foo".to_string());
        // Arc::get_mut falha — nome permanece None
        if let FuncRepr::Closure(c) = f1.repr() {
            assert_eq!(c.name, None, "Arc partilhado não deve ser mutado");
        }
    }

    #[test]
    fn native_func_debug_nao_panicar() {
        let f = Func::native("type", |_ctx, _args, _world, _cf| Ok(Value::None));
        assert_eq!(format!("{:?}", f), "<function>");
    }

    #[test]
    fn element_func_e_veiculo_de_construtor_nao_chamavel_generico() {
        // **Contrato NEGATIVO da `FuncRepr::Element`** (carona C1, fecho P337 do
        // Lote F-3 inc-2). A variante é **veículo do construtor de elemento** —
        // identificável e distinta de closure/nativa — **não** um `Func` chamável
        // genérico nem um nativo despachável por fn-ptr.
        use crate::entities::content::Content;
        let ctor: crate::entities::element_registry::ElementCtor =
            Arc::new(|_args| Ok(Content::Empty));
        let elem = Func::element("callout", ctor);
        let closure = make_closure();
        let native = Func::native("type", |_ctx, _args, _world, _cf| Ok(Value::None));

        // (a) `element_name()` SÓ é `Some` para `Element` — o `#show` constrói
        //     `Selector::DynKind` apenas para elementos; uma closure/nativa de
        //     utilizador **nunca** vira `DynKind` (não há confusão de superfície).
        assert_eq!(elem.element_name(), Some("callout"));
        assert_eq!(closure.element_name(), None, "closure não é elemento");
        assert_eq!(native.element_name(), None, "nativa não é elemento");

        // (b) `native_fn_addr()` é `None` para `Element` — **não** é selecionável
        //     pelo caminho de fn-ptr nativo (S2); só pelo `DynKind` (kind). Logo
        //     não colide com a identidade dos 65 nativos.
        assert!(elem.native_fn_addr().is_none(), "Element não tem fn-ptr nativo");

        // (c) `name()` apresenta o nome do elemento (erros/debug), como as nativas.
        assert_eq!(elem.name(), Some("callout"));

        // Nota arquitetural (não testável por ausência): a ÚNICA via que produz
        // `FuncRepr::Element` é `Func::element`, invocada só no threading
        // registry→escopo (`eval/mod.rs`). A superfície de linguagem comum não
        // tem sintaxe que a construa — só o registry injetado a alcança.
    }

    // ── P702 — FuncRepr::With ────────────────────────────────────────────────

    #[test]
    fn with_delega_nome_a_funcao_interna() {
        let native = Func::native("round", |_ctx, _args, _world, _cf| Ok(Value::None));
        let wrapped = native.with(Args::positional(vec![Value::Int(2)]));
        assert_eq!(
            wrapped.name(),
            Some("round"),
            "with() delega o nome à função interna"
        );
    }

    #[test]
    fn with_native_fn_addr_e_none() {
        let native = Func::native("round", |_ctx, _args, _world, _cf| Ok(Value::None));
        let wrapped = native.with(Args::positional(vec![Value::Int(2)]));
        assert!(
            wrapped.native_fn_addr().is_none(),
            "with() não é directamente um fn-ptr nativo"
        );
    }

    #[test]
    fn with_namespace_delega_a_funcao_interna() {
        // Medido contra o vanilla (não assumido): `table.with(columns: 2).cell`
        // compila e devolve `function` — func.rs:275 do vanilla delega
        // `scope()` através de `With`. Reproduzido aqui ao nível de `Func`.
        let mut ns = Scope::new();
        ns.define(
            "cell",
            Value::Func(Func::native("table_cell", |_ctx, _args, _world, _cf| {
                Ok(Value::None)
            })),
        );
        let table = Func::native_with_namespace(
            "table",
            |_ctx, _args, _world, _cf| Ok(Value::None),
            Arc::new(ns),
        );
        let wrapped = table.with(Args::positional(vec![]));
        assert!(
            wrapped.namespace().is_some(),
            "with() delega o namespace à função interna"
        );
        assert!(wrapped.namespace().unwrap().get("cell").is_some());
    }

    #[test]
    fn with_encadeado_preserva_a_funcao_original() {
        let native = Func::native("round", |_ctx, _args, _world, _cf| Ok(Value::None));
        let w1 = native.with(Args::positional(vec![Value::Int(1)]));
        let w2 = w1.with(Args::positional(vec![Value::Int(2)]));
        assert_eq!(w2.name(), Some("round"), "encadeamento continua a delegar o nome");
    }
}
