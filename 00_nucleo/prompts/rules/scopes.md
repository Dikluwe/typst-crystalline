# Prompt L0 — `rules/scopes`

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/scopes.rs`
**Criado em**: 2026-04-02 (Passo 31 — closures lazy capture / Scopes com captured)
**Atualizado em**: 2026-04-12 (restauro — expandido com `captured`, `with_parent`, `snapshot`, `push_scope`, `iter_all`)
**ADRs relevantes**: ADR-0017 (adiamento eval), ADR-0023 (Scope/indexmap)

---

## Contexto e Objetivo

`Scopes<'a>` é a **pilha de âmbitos léxicos** durante a avaliação (`eval.rs`).
Mantém o âmbito activo (`top`), uma pilha de âmbitos anteriores (`scopes`),
um scope capturado opcional para closures (`captured`), e uma referência
opcional à biblioteca standard (`base`).

A regra de pesquisa é: **top → scopes (reverso, mais recente primeiro) → captured → base**.

Pertence a `rules/` porque é um mecanismo de execução (avaliador), não uma
entidade de dados. Depende apenas de `Scope`, `Value` e `Library` (todos L1).

### Closures e captura lazy (Passo 31)

No Typst, closures capturam o scope no momento da definição. O cristalino
implementa captura em duas fases:

1. **Snapshot (captura eager)**: `scopes.snapshot()` cria um `Scope` com todos
   os bindings visíveis. O resultado é envolvido em `Arc::new(...)` — partilhado
   por referência com custo O(1) por closure subsequente.
2. **Lookup (lazy)**: durante a chamada de closure, `Scopes::with_parent(arc)`
   cria uma nova pilha onde `captured` aponta para o scope da definição.

Origem: `lab/typst-original/crates/typst-library/src/foundations/context.rs` e `eval.rs`

---

## Restrições Estruturais

- Camada **L1**: zero I/O. `Arc<Scope>` em `captured` é gestão de RAM (ADR-0029).
- Sem dependências externas.
- `Vec<Scope>` em `scopes` — clone é O(n); apenas `enter`/`exit` mutam a pilha.
- O campo `base: Option<&'a Library>` é somente leitura — não mutado após a criação.

---

## Instrução

### Estrutura pública

```rust
pub struct Scopes<'a> {
    /// Âmbito activo no momento.
    pub top: Scope,
    /// Âmbitos anteriores (mais antigo na posição 0, mais recente no fim).
    pub scopes: Vec<Scope>,
    /// Scope capturado pela closure — partilhado via Arc sem clone dos valores.
    /// Consultado após top/scopes e antes de base.
    pub captured: Option<Arc<Scope>>,
    /// Âmbito base — a Library (stdlib) do Typst. Somente leitura.
    pub base: Option<&'a Library>,
}
```

### Interface pública completa

```rust
impl<'a> Scopes<'a> {
    /// Cria nova pilha com top vazio e base opcional.
    pub fn new(base: Option<&'a Library>) -> Self

    /// Cria uma pilha para chamada de closure com o scope capturado como parent.
    /// Lookup order: top (params) → captured (scope da definição).
    /// O Arc é partilhado — sem clone dos valores da captura.
    pub fn with_parent(parent: Arc<Scope>) -> Scopes<'static>

    /// Captura todos os bindings visíveis num snapshot Scope (eager).
    /// Ordem de inserção: captured → scopes → top (mais recente sobrescreve).
    /// Custo: O(N) uma vez; depois partilhado em O(1) por cada closure.
    pub fn snapshot(&self) -> Scope

    // ── Gestão de pilha ─────────────────────────────────────────────────
    /// Entra num novo âmbito: empurra top para scopes, cria novo top vazio.
    pub fn enter(&mut self)

    /// Sai do âmbito activo: restaura o anterior. Retorna o âmbito saído.
    pub fn exit(&mut self) -> Scope

    /// Empurra um Scope pré-populado como novo âmbito activo.
    /// Usado por apply_closure para criar o ambiente de chamada.
    pub fn push_scope(&mut self, scope: Scope)

    // ── Bindings ────────────────────────────────────────────────────────
    /// Define um binding no âmbito activo (top).
    pub fn define(&mut self, name: impl Into<String>, value: Value)

    /// Pesquisa um nome do âmbito mais local para o mais global.
    /// Ordem: top → scopes (reverso) → captured → base.
    pub fn get(&self, name: &str) -> Option<&Value>

    /// Itera sobre todos os bindings visíveis (para snapshot e diagnóstico).
    /// Ordem: captured → scopes[0] → ... → top (mais recente sobrescreve).
    pub fn iter_all(&self) -> impl Iterator<Item = (&str, &Value)> + '_
}
```

---

## Critérios de Verificação

```
// Define e lookup no top
Scopes::new(None) + define("x", Value::None) → get("x") = Some(...)

// Lookup percorre a pilha
Scopes::new(None) + define("x", ..) + enter() → get("x") = Some(...)

// exit remove bindings do filho
Scopes::new(None) + enter() + define("local", ..) + exit()
→ get("local") = None

// enter/exit simétrico
define("global", ..) + enter() + define("local", ..) + exit()
→ get("global") = Some(..)
→ get("local")  = None

// Sombra: filho oculta pai com mesmo nome
define("x", V1) + enter() + define("x", V2)
→ top.get("x") = Some(V2)
→ scopes.last().get("x") = Some(V1)
```

---

## Resultado Esperado

- `01_core/src/rules/scopes.rs` com `Scopes<'a>` e testes co-localizados
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/rules/scopes.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-02 | Criação — Passo 31: closures lazy capture; campo `captured: Option<Arc<Scope>>` | `scopes.rs` |
| 2026-04-12 | Restauro — expandido com `with_parent`, `snapshot`, `push_scope`, `iter_all`, critérios completos | `scopes.md` |
