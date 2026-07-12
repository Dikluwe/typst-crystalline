//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/scopes.md
//! @prompt-hash 573e7f07
//! @layer L1
//! @updated 2026-04-02

use std::sync::Arc;

use crate::entities::scope::Scope;
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
            base,
        }
    }

    /// Cria uma pilha para chamada de closure com o scope capturado como parent.
    ///
    /// O `parent` é partilhado via Arc — sem clone dos valores.
    /// Lookup order: top (params/auto-ref) → captured (scope da definição).
    pub fn with_parent(parent: Arc<Scope>) -> Scopes<'static> {
        Scopes {
            top: Scope::new(),
            scopes: Vec::new(),
            captured: Some(parent),
            base: None,
        }
    }

    /// Captura todos os bindings visíveis num novo Scope (snapshot eager).
    ///
    /// Ordem de inserção: captured → scopes → top (mais recente sobrescreve).
    /// Wrapping em `Arc::new(scopes.snapshot())` dá captura O(N) única,
    /// depois partilhada em O(1) por cada closure que usa o scope.
    pub fn snapshot(&self) -> Scope {
        let mut s = Scope::new();
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
        
        std::mem::replace(
            &mut self.top,
            self.scopes.pop().unwrap_or_default(),
        )
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
        cap_iter.chain(
            self.scopes.iter()
                .chain(std::iter::once(&self.top))
                .flat_map(|scope| scope.iter().map(|(name, binding)| (name, binding.value())))
        )
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
        // base (Library) — stub neste passo; sem lookup real
        let _ = self.base;
        None
    }

    /// P715 — acesso mutável a um binding existente, para atribuição (`x = v`,
    /// `x += v`, desestruturação em atribuição). Pesquisa `top` → `scopes`
    /// (mais recente primeiro) — mesma ordem de `get`. **Não** pesquisa
    /// `captured` nem `base`: mutar uma variável capturada por uma closure
    /// (do seu scope de definição) ou da stdlib não é um caso medido/alcançado
    /// (ver `rules/eval.md` §P715) — devolve `None`, tratado como "unknown
    /// variable" pelo caller, tal como um nome inexistente.
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
        assert!(scopes.get("local").is_none(), "binding local deve desaparecer após exit");
    }

    #[test]
    fn sombra_pai_pelo_filho() {
        // Binding no filho oculta binding do pai com o mesmo nome
        let mut scopes = Scopes::new(None);
        scopes.define("x", Value::None);
        scopes.enter();
        scopes.define("x", Value::None);  // sombra
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
        // Mutar uma variável capturada do scope de definição de uma closure
        // não é um caso medido/alcançado — `get_mut` devolve `None`, tratado
        // como "unknown variable" pelo caller.
        let mut base = Scope::new();
        base.define("x", Value::Int(1));
        let mut scopes = Scopes::with_parent(std::sync::Arc::new(base));
        assert!(scopes.get("x").is_some(), "get deve ver a variável capturada");
        assert!(scopes.get_mut("x").is_none(), "get_mut não deve alcançar captured");
    }
}
