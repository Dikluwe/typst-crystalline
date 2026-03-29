# Prompt L0 — rules/scopes

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/scopes.rs`
**ADRs relevantes**: ADR-0017 (adiamento eval), ADR-0023 (Scope/indexmap)

## Contexto

`Scopes<'a>` é a pilha de âmbitos durante avaliação de Typst.
Mantém o `top` (âmbito activo), uma pilha de âmbitos anteriores,
e uma referência opcional à Library (âmbito base do std).

É lógica de domínio pura — dependências apenas de `Scope` e `Library`
(ambos em L1). Pertence a `rules/` porque é mecanismo de execução,
não entidade de dados.

No original: `top: Scope`, `scopes: Vec<Scope>`, `base: Option<&'a Library>`.
Pesquisa: top → scopes (reverso) → base.

## Interface pública

```rust
pub struct Scopes<'a> {
    pub top: Scope,
    pub scopes: Vec<Scope>,
    pub base: Option<&'a Library>,
}

impl<'a> Scopes<'a> {
    pub fn new(base: Option<&'a Library>) -> Self
    pub fn enter(&mut self)               // empurra top para scopes, cria novo top
    pub fn exit(&mut self) -> Scope       // pop de scopes para top, retorna o antigo top
    pub fn define(&mut self, name: impl Into<String>, value: Value)
    pub fn get(&self, name: &str) -> Option<&Value>
}
```

## Critérios de Verificação

```
Dado Scopes::new(None) e define("x", Value(()))
Quando get("x") for chamado
Então Some(&Value(()))

Dado Scopes com binding "x" no âmbito pai
Quando enter() + get("x") no âmbito filho
Então Some — lookup percorre a pilha

Dado Scopes com binding "x" no âmbito filho
Quando exit() for chamado
Então binding "x" desaparece (âmbito filho removido)

Dado Scopes com binding "x" no âmbito filho e pai
Quando get("x") no filho
Então retorna o do filho (sombra do pai)
```
