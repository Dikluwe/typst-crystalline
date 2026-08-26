//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/scopes.md
//! @prompt-hash 3e72b30c
//! @layer L1
//! @updated 2026-07-16

use std::sync::Arc;

use crate::entities::scope::{Capturer, Scope};
use crate::entities::value::Value;
use crate::entities::world_types::Library;

/// Pilha de âmbitos durante avaliação de Typst.
///
/// Mantém o âmbito activo (`top`), uma pilha de âmbitos anteriores
/// (`scopes`), um scope capturado opcional (para chamadas de closure),
/// e uma referência opcional à Library (âmbito base do std).
/// Pesquisa: top → scopes (do mais recente para o mais antigo) → captured → base.
pub struct Scopes<'a> {
    /// Âmbito activo no momento.
    pub top: Scope,
    /// Âmbitos anteriores (mais antigo na posição 0).
    pub scopes: Vec<Scope>,
    /// Scope capturado pela closure — partilhado via Arc sem clone dos valores.
    /// Consultado depois de `top`/`scopes` e antes de `base`.
    /// Permite lookup lazy das variáveis capturadas durante chamadas de closure.
    pub captured: Option<Arc<Scope>>,
    /// **P772q** — por que motivo `captured` foi capturado (closure normal
    /// vs bloco `context`). `None` sse `captured` também for `None`. Só
    /// afecta a mensagem de erro ao mutar um nome de `captured`
    /// (`captured_by`) — não afecta leitura nem a pesquisa normal.
    pub captured_by: Option<Capturer>,
    /// Âmbito base — a biblioteca standard do Typst.
    pub base: Option<&'a Library>,
}

impl<'a> Scopes<'a> {
    /// Cria uma nova pilha com âmbito vazio e base opcional.
    pub fn new(base: Option<&'a Library>) -> Self {
        Self {
            top: Scope::new(),
            scopes: Vec::new(),
            captured: None,
            captured_by: None,
            base,
        }
    }

    /// Cria uma pilha para chamada de closure com o scope capturado como parent.
    ///
    /// O `parent` é partilhado via Arc — sem clone dos valores.
    /// Lookup order: top (params/auto-ref) → captured (scope da definição).
    /// **P772q** — `capturer` regista por que motivo o scope foi capturado
    /// (`ClosureRepr.capturer`), usado só para a mensagem de erro de
    /// mutação (`captured_by`).
    pub fn with_parent(parent: Arc<Scope>, capturer: Capturer) -> Scopes<'static> {
        Scopes {
            top: Scope::new(),
            scopes: Vec::new(),
            captured: Some(parent),
            captured_by: Some(capturer),
            base: None,
        }
    }

    /// Captura todos os bindings visíveis num novo Scope (snapshot eager).
    ///
    /// Ordem de inserção: base → captured → scopes → top (mais recente
    /// sobrescreve) — mesma ordem de precedência de `get`.
    /// Wrapping em `Arc::new(scopes.snapshot())` dá captura O(N) única,
    /// depois partilhada em O(1) por cada closure que usa o scope.
    ///
    /// **P772n** — inclui `base` (stdlib). `Scopes::with_parent` (usado em
    /// cada chamada da closure) constrói um `Scopes<'static>` com
    /// `base: None`, porque `Library` não é `'static` — sem isto, uma
    /// closure perderia acesso à stdlib (`#let f() = upper("x")` falhava
    /// com "unknown variable: upper"). Bake-in aqui evita ter de propagar
    /// o lifetime de `Library` através de `apply_closure`. `get_mut` nunca
    /// pesquisa `captured`, por isso isto não reabre P772l §2.3: mutar um
    /// nome de stdlib capturado por uma closure continua a falhar (embora
    /// com a mensagem do caso "captured", não "constant" — P772l §2.2,
    /// deliberadamente fora do âmbito de P772n).
    pub fn snapshot(&self) -> Scope {
        let mut s = Scope::new();
        if let Some(base) = self.base {
            for (name, binding) in base.global.iter() {
                s.define(name, binding.value().clone());
            }
        }
        if let Some(cap) = &self.captured {
            for (name, binding) in cap.iter() {
                s.define(name, binding.value().clone());
            }
        }
        for scope in &self.scopes {
            for (name, binding) in scope.iter() {
                s.define(name, binding.value().clone());
            }
        }
        for (name, binding) in self.top.iter() {
            s.define(name, binding.value().clone());
        }
        s
    }

    /// Entra num novo âmbito: empurra `top` para a pilha e cria novo `top` vazio.
    pub fn enter(&mut self) {
        self.scopes.push(std::mem::take(&mut self.top));
    }

    /// Sai do âmbito actual: restaura o âmbito anterior. Retorna o âmbito saído.
    pub fn exit(&mut self) -> Scope {
        std::mem::replace(&mut self.top, self.scopes.pop().unwrap_or_default())
    }

    /// Define um binding no âmbito activo (`top`).
    pub fn define(&mut self, name: impl Into<String>, value: Value) {
        self.top.define(name, value);
    }

    /// Empurra um scope pre-populado como novo âmbito activo.
    ///
    /// Usado por `apply_closure` para criar o ambiente de chamada com
    /// variáveis capturadas e parâmetros já definidos.
    pub fn push_scope(&mut self, scope: Scope) {
        self.scopes.push(std::mem::replace(&mut self.top, scope));
    }

    /// Itera sobre todos os bindings visíveis em todos os âmbitos.
    ///
    /// Ordem: captured → scopes[0] (mais antigo) → scopes[n-1] → top (mais recente).
    /// Inserção mais recente sobrescreve anterior, garantindo o valor correcto.
    pub fn iter_all(&self) -> impl Iterator<Item = (&str, &Value)> + '_ {
        let cap_iter: Box<dyn Iterator<Item = (&str, &Value)> + '_> =
            if let Some(cap) = &self.captured {
                Box::new(cap.iter().map(|(name, binding)| (name, binding.value())))
            } else {
                Box::new(std::iter::empty())
            };
        cap_iter.chain(self.scopes.iter().chain(std::iter::once(&self.top)).flat_map(
            |scope| scope.iter().map(|(name, binding)| (name, binding.value())),
        ))
    }

    /// Pesquisa um nome do âmbito mais local para o mais global.
    ///
    /// Ordem: top → scopes (reverso) → captured → base.
    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(v) = self.top.get(name) {
            return Some(v);
        }
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v);
            }
        }
        if let Some(cap) = &self.captured {
            if let Some(v) = cap.get(name) {
                return Some(v);
            }
        }
        // P772n — base (Library) consultado a sério como último recurso.
        self.base.and_then(|base| base.global.get(name))
    }

    /// **P780** — pesquisa local/utilizador, sem consultar `base` (stdlib
    /// global). Ordem: top → scopes (reverso) → captured — igual a `get`
    /// menos o último passo. Paridade `Scopes::get_in_math` (vanilla,
    /// `foundations/scope.rs:75-92`): em modo math, o scope global do
    /// stdlib não é consultado directamente para resolver um identificador
    /// (só via `std.<nome>` explícito) — só o scope léxico local conta,
    /// depois o caller consulta o scope `math`-específico (símbolos/
    /// operadores) separadamente. Medido: `#let str = 5; $str$` mostra
    /// `5` (scope local, P780); `$str$` sem binding local dá erro
    /// (`str` só existe em `base.global`, não resolve em math) — ver
    /// `has_global` para o hint dessa distinção.
    pub fn get_local(&self, name: &str) -> Option<&Value> {
        if let Some(v) = self.top.get(name) {
            return Some(v);
        }
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v);
            }
        }
        if let Some(cap) = &self.captured {
            if let Some(v) = cap.get(name) {
                return Some(v);
            }
        }
        None
    }

    /// **P780** — `true` se `name` existe em `base.global` (stdlib), usado
    /// só para escolher o hint de `unknown_variable_math` (paridade
    /// `unknown_variable_math(var, in_global)`, vanilla) — não é usado
    /// para resolução (ver `get_local`).
    pub fn has_global(&self, name: &str) -> bool {
        self.base.is_some_and(|base| base.global.get(name).is_some())
    }

    /// P715 — acesso mutável a um binding existente, para atribuição (`x = v`,
    /// `x += v`, desestruturação em atribuição). Pesquisa `top` → `scopes`
    /// (mais recente primeiro) — mesma ordem de `get`. **Não** pesquisa
    /// `captured` nem `base`: mutar uma variável capturada por uma closure
    /// (do seu scope de definição) não é um caso medido/alcançado (ver
    /// `rules/eval.md` §P715) — devolve `None`, tratado como "unknown
    /// variable" pelo caller. Mutar um nome de `base` (stdlib) também
    /// devolve `None` aqui — mas **P772n** dá ao caller uma forma de
    /// distinguir esse caso via `is_constant`, para reportar "cannot mutate
    /// a constant" em vez de "unknown variable" (paridade vanilla,
    /// `foundations/scope.rs:63-70`).
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        if let Some(v) = self.top.get_mut(name) {
            return Some(v);
        }
        for scope in self.scopes.iter_mut().rev() {
            if let Some(v) = scope.get_mut(name) {
                return Some(v);
            }
        }
        None
    }

    /// **P772n** — true sse `name` só é alcançável via `base` (stdlib e
    /// outros bindings seedados no bootstrap do avaliador antes do âmbito
    /// do documento começar), nunca via `top`/`scopes` (mutável) nem
    /// `captured` (fecho de closure). Usado pelo caller de atribuição
    /// para escolher a mensagem de erro correcta quando `get_mut` falha.
    pub fn is_constant(&self, name: &str) -> bool {
        if self.top.get(name).is_some() {
            return false;
        }
        if self.scopes.iter().any(|scope| scope.get(name).is_some()) {
            return false;
        }
        if let Some(cap) = &self.captured {
            if cap.get(name).is_some() {
                return false;
            }
        }
        self.base.is_some_and(|base| base.global.get(name).is_some())
    }

    /// **P772q** — `Some(capturer)` sse `name` está em `captured` (não em
    /// `top`/`scopes`, que têm precedência — sombra local continua
    /// mutável). Usado pelo caller de atribuição para escolher a mensagem
    /// "variables from outside the {function|context expression}...".
    /// Verificado **antes** de `is_constant` em `access()`
    /// (`eval/bindings.rs`): paridade vanilla onde `get_mut` bem sucedido
    /// sobre um binding `Captured` falha em `.write()`, sem nunca chegar a
    /// considerar `base`.
    pub fn captured_by(&self, name: &str) -> Option<Capturer> {
        if self.top.get(name).is_some() {
            return None;
        }
        if self.scopes.iter().any(|scope| scope.get(name).is_some()) {
            return None;
        }
        if self.captured.as_ref().is_some_and(|cap| cap.get(name).is_some()) {
            return self.captured_by;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::value::Value;

    #[test]
    fn define_e_get_no_top() {
        let mut scopes = Scopes::new(None);
        scopes.define("x", Value::None);
        assert!(scopes.get("x").is_some());
        assert!(scopes.get("y").is_none());
    }

    #[test]
    fn get_percorre_pilha() {
        // Binding no âmbito pai deve ser visível no filho
        let mut scopes = Scopes::new(None);
        scopes.define("x", Value::None);
        scopes.enter();
        assert!(scopes.get("x").is_some(), "binding do pai deve ser visível no filho");
    }

    #[test]
    fn exit_remove_binding_filho() {
        let mut scopes = Scopes::new(None);
        scopes.enter();
        scopes.define("local", Value::None);
        assert!(scopes.get("local").is_some());
        scopes.exit();
        assert!(
            scopes.get("local").is_none(),
            "binding local deve desaparecer após exit"
        );
    }

    #[test]
    fn sombra_pai_pelo_filho() {
        // Binding no filho oculta binding do pai com o mesmo nome
        let mut scopes = Scopes::new(None);
        scopes.define("x", Value::None);
        scopes.enter();
        scopes.define("x", Value::None); // sombra
                                         // Ambos existem — o lookup retorna o do filho (top)
        assert!(scopes.top.get("x").is_some());
        assert!(scopes.scopes.last().unwrap().get("x").is_some());
    }

    #[test]
    fn enter_exit_simetrico() {
        let mut scopes = Scopes::new(None);
        scopes.define("global", Value::None);
        scopes.enter();
        scopes.define("local", Value::None);
        scopes.exit();
        assert!(scopes.get("global").is_some());
        assert!(scopes.get("local").is_none());
    }

    // ── P715 — get_mut ───────────────────────────────────────────────────────

    #[test]
    fn p715_get_mut_muta_binding_em_top() {
        let mut scopes = Scopes::new(None);
        scopes.define("x", Value::Int(1));
        *scopes.get_mut("x").unwrap() = Value::Int(42);
        assert_eq!(scopes.get("x"), Some(&Value::Int(42)));
    }

    #[test]
    fn p715_get_mut_atravessa_ambitos_pai() {
        // Binding no âmbito pai deve ser mutável a partir de um filho —
        // mesma travessia usada por `x = v` dentro de um bloco `{ }` aninhado.
        let mut scopes = Scopes::new(None);
        scopes.define("x", Value::Int(1));
        scopes.enter();
        *scopes.get_mut("x").unwrap() = Value::Int(99);
        assert_eq!(scopes.get("x"), Some(&Value::Int(99)));
        scopes.exit();
        // Mutação sobrevive à saída do âmbito filho (mutou o pai, não uma cópia).
        assert_eq!(scopes.get("x"), Some(&Value::Int(99)));
    }

    #[test]
    fn p715_get_mut_ausente_devolve_none() {
        let mut scopes = Scopes::new(None);
        assert!(scopes.get_mut("missing").is_none());
    }

    #[test]
    fn p715_get_mut_nao_pesquisa_captured() {
        // `get_mut` continua a nunca alcançar `captured` — devolve `None`.
        // P772q: o caller (`access()`) distingue esse `None` de um nome
        // realmente inexistente via `captured_by`, sem alterar `get_mut`.
        let mut base = Scope::new();
        base.define("x", Value::Int(1));
        let mut scopes =
            Scopes::with_parent(std::sync::Arc::new(base), Capturer::Function);
        assert!(scopes.get("x").is_some(), "get deve ver a variável capturada");
        assert!(scopes.get_mut("x").is_none(), "get_mut não deve alcançar captured");
    }

    // ── P772n — base real (Library) e is_constant ───────────────────────────

    fn library_com_calc() -> Library {
        let mut global = Scope::new();
        global.define("calc", Value::Int(1));
        Library::with_global(global)
    }

    #[test]
    fn p772n_get_consulta_base_a_serio() {
        let library = library_com_calc();
        let scopes = Scopes::new(Some(&library));
        assert_eq!(scopes.get("calc"), Some(&Value::Int(1)));
    }

    #[test]
    fn p772n_get_mut_nunca_alcanca_base() {
        let library = library_com_calc();
        let mut scopes = Scopes::new(Some(&library));
        assert!(scopes.get_mut("calc").is_none());
    }

    #[test]
    fn p772n_is_constant_true_so_para_nome_de_base() {
        let library = library_com_calc();
        let scopes = Scopes::new(Some(&library));
        assert!(scopes.is_constant("calc"));
        assert!(!scopes.is_constant("nunca-existiu"));
    }

    #[test]
    fn p772n_sombra_local_deixa_de_ser_constante() {
        // Paridade vanilla: `#let calc = 5; #{ calc = 10 }` compila sem erro
        // — a sombra em `top` é um binding normal, mutável.
        let library = library_com_calc();
        let mut scopes = Scopes::new(Some(&library));
        scopes.define("calc", Value::Int(5));
        assert!(!scopes.is_constant("calc"));
        assert_eq!(scopes.get_mut("calc"), Some(&mut Value::Int(5)));
    }

    // ── P772q — captured_by distingue Function de Context ──────────────────

    fn captured_com_x() -> Arc<Scope> {
        let mut s = Scope::new();
        s.define("x", Value::Int(1));
        Arc::new(s)
    }

    #[test]
    fn p772q_captured_by_function() {
        let scopes = Scopes::with_parent(captured_com_x(), Capturer::Function);
        assert_eq!(scopes.captured_by("x"), Some(Capturer::Function));
    }

    #[test]
    fn p772q_captured_by_context() {
        let scopes = Scopes::with_parent(captured_com_x(), Capturer::Context);
        assert_eq!(scopes.captured_by("x"), Some(Capturer::Context));
    }

    #[test]
    fn p772q_captured_by_none_para_nome_inexistente() {
        let scopes = Scopes::with_parent(captured_com_x(), Capturer::Function);
        assert_eq!(scopes.captured_by("nunca-existiu"), None);
    }

    #[test]
    fn p772q_sombra_local_de_nome_capturado_continua_mutavel() {
        // Paridade vanilla: um parâmetro ou `let` local com o mesmo nome
        // de uma variável capturada sombreia-a — mutável normalmente,
        // `captured_by` não dispara.
        let mut scopes = Scopes::with_parent(captured_com_x(), Capturer::Function);
        scopes.define("x", Value::Int(2));
        assert_eq!(scopes.captured_by("x"), None);
        assert_eq!(scopes.get_mut("x"), Some(&mut Value::Int(2)));
    }

    #[test]
    fn p772q_captured_by_precede_is_constant() {
        // Um nome de `base` capturado por uma closure (ex.: `calc` usado
        // dentro do corpo) deve dar a mensagem de "captured", não a de
        // "constant" — captured_by tem precedência (medido contra o
        // vanilla: `#let f() = { calc = 5 }; f()` dá a mensagem de
        // função, não "cannot mutate a constant").
        let library = library_com_calc();
        let base_scopes = Scopes::new(Some(&library));
        let snapshot = Arc::new(base_scopes.snapshot());
        let scopes = Scopes::with_parent(snapshot, Capturer::Function);
        assert_eq!(scopes.captured_by("calc"), Some(Capturer::Function));
        assert!(!scopes.is_constant("calc"));
    }

    // ── P780 — get_local/has_global (paridade get_in_math) ─────────────────

    #[test]
    fn p780_get_local_nao_alcanca_base() {
        let library = library_com_calc();
        let scopes = Scopes::new(Some(&library));
        assert!(scopes.get_local("calc").is_none(), "get_local não deve consultar base");
        assert_eq!(
            scopes.get("calc"),
            Some(&Value::Int(1)),
            "get normal continua a alcançar base"
        );
    }

    #[test]
    fn p780_has_global_reporta_nome_de_base() {
        let library = library_com_calc();
        let scopes = Scopes::new(Some(&library));
        assert!(scopes.has_global("calc"));
        assert!(!scopes.has_global("nunca-existiu"));
    }

    #[test]
    fn p780_get_local_encontra_binding_de_top() {
        let mut scopes = Scopes::new(None);
        scopes.define("x", Value::Int(5));
        assert_eq!(scopes.get_local("x"), Some(&Value::Int(5)));
    }

    #[test]
    fn p780_get_local_sombra_nome_de_base() {
        // Paridade vanilla: `#let calc = 5; $calc$` deve resolver ao valor
        // local (5), não ao módulo `calc` — get_local vê o top primeiro.
        let library = library_com_calc();
        let mut scopes = Scopes::new(Some(&library));
        scopes.define("calc", Value::Int(5));
        assert_eq!(scopes.get_local("calc"), Some(&Value::Int(5)));
    }

    #[test]
    fn p780_get_local_atravessa_pilha_e_captured() {
        let mut base = Scope::new();
        base.define("y", Value::Int(7));
        let mut scopes = Scopes::with_parent(Arc::new(base), Capturer::Function);
        assert_eq!(
            scopes.get_local("y"),
            Some(&Value::Int(7)),
            "captured deve ser alcançado"
        );
        scopes.enter();
        scopes.define("x", Value::Int(1));
        scopes.enter();
        assert_eq!(
            scopes.get_local("x"),
            Some(&Value::Int(1)),
            "deve atravessar a pilha de scopes"
        );
    }
}
