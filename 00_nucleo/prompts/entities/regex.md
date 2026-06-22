# Prompt L0 — `Regex` — wrapper L1 sobre `regex::Regex`
Hash do Código: 719a0a76

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/regex.rs`, `01_core/src/entities/value.rs`
**Criado em**: 2026-05-12 (P209D sub-passo — wrapper L1 sobre
`regex::Regex` crate para `Selector::Regex` per ADR-0077).
**Refinado em**: 2026-06-22 (P402 — refino para tipo de primeiro-cidadão:
`EcoString` + `Arc<regex::Regex>`, integração `Value::Regex`, cast).
**ADRs relevantes**: ADR-0077 (regex em L1); ADR-0017 (portão aberto);
ADR-0107 (paridade linguagem); ADR-0029 (pureza L1); ADR-0054 (operações
regex scope-out); DEBT-52 (`text.font` dict desbloqueado para futuro).

---

## Contexto

P209D introduziu `Selector::Regex(Regex)` com wrapper L1 sobre `regex::Regex`.
P393 activou `Value::Regex(Regex)` para `regex(pattern)` em `#show`. P402
refina o tipo para primeiro-cidadão pleno: `pattern` passa a `EcoString`,
`compiled` passa a `Arc<regex::Regex>` (clone barato), e adiciona-se cast
`Str → Regex`.

## Restrições estruturais

- Camada **L1**: struct puro, sem I/O.
- `Hash` + `PartialEq` + `Eq` manuais via `pattern`.
- `Clone` via derive sobre `Arc<regex::Regex>` — clone O(1).
- `Debug` manual (`compiled` é opaco do crate).
- Dep `regex` autorizada via ADR-0077 + `crystalline.toml` + `01_core/Cargo.toml`.

## Interface pública

```rust
use ecow::EcoString;
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum RegexError {
    #[error("regex inválida: {0}")]
    Invalid(String),
}

#[derive(Clone)]
pub struct Regex {
    pattern: EcoString,
    compiled: Arc<regex::Regex>,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Self, RegexError>;
    pub fn pattern(&self) -> &str;
    pub fn is_match(&self, text: &str) -> bool;
}

impl Hash for Regex      { /* via pattern */ }
impl PartialEq for Regex { /* via pattern */ }
impl Eq for Regex        {}
impl Debug for Regex     { /* Regex { pattern: "..." } */ }
impl Default for Regex   { /* pattern vazia */ }
```

## Semântica

- `Regex::new(pattern)`: valida pattern; erro contextual.
- `pattern()`: pattern original.
- `is_match(text)`: delega ao `regex::Regex` compilado.
- `Hash`/`PartialEq`/`Eq`: por `pattern` (valor linguagem).
- `Clone`: partilha `Arc<regex::Regex>`; não recompila.

## Consumers

- `entities::selector::Selector::Regex(Regex)` — P209D.
- `entities::value::Value::Regex(Regex)` — P393 + P402 refino.
- `cast_regex()` em `Value`: `Regex` identidade; `Str` → compile fallible.

## Tests obrigatórios

- `regex_new_valido_ok`, `regex_new_invalido_err`.
- `regex_is_match_basico`.
- `regex_eq_via_pattern`, `regex_clone_preserva_semantica`, `regex_hash_determinismo`.
- `value_regex_type_name`, `value_regex_cast_identity`, `value_regex_cast_from_str`,
  `value_regex_cast_from_str_invalid`, `value_regex_partial_eq`.

## Não-objectivos

- Operações regex (`match`, `replace`, etc.) — scope-out ADR-0054.
- `text.font` dict — DEBT-52, depende de refactor futuro.
- Flags regex — scope-out ADR-0054.
- Query `Selector::Regex` sobre Content text — continua stub.
