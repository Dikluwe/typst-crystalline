# Prompt L0 — `entities/regex`
Hash do Código: dfe0ecb8

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/regex.rs`
**Criado em**: 2026-05-12 (P209D sub-passo — wrapper L1 sobre
`regex::Regex` crate para `Selector::Regex` per ADR-0076 + ADR-0077).
**ADRs relevantes**: ADR-0077 (regex em L1; PROPOSTO P209D);
ADR-0076 (M9c Introspector completion); ADR-0029 (allowlist deps
L1 política).

---

## Contexto

P209D (per ADR-0076 §Plano de materialização) introduz
`Selector::Regex(Regex)` como 5º variant do Selector enum.
Cristalino sem dep `regex` antes de P209D — necessário wrapper
L1 isolando o crate.

`regex::Regex` (crate `regex` 1.x) **não deriva** `Hash`,
`PartialEq`, `Eq`, nem `Clone` directo. O wrapper L1 implementa
estes traits manualmente via **pattern string como key**: mesma
pattern → mesmo regex (compilação determinística do crate).

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O, sem estado global.
- Read-only após construção (constructor `Regex::new(pattern)`
  é o único ponto de mutação interna).
- `Hash` + `PartialEq` + `Eq` manuais via field `pattern: String`.
- `Clone` manual via re-construção (`Regex::new(&self.pattern)
  .expect(...)`) — pattern já validada na construção original.
- `Debug` manual para output legível (`compiled` é opaco do
  crate).
- Dep `regex` 1.x autorizada via ADR-0077 + `crystalline.toml:64`
  + `01_core/Cargo.toml`.

---

## Interface pública

```rust
use std::hash::{Hash, Hasher};

#[derive(thiserror::Error, Debug)]
pub enum RegexError {
    #[error("regex inválida: {0}")]
    Invalid(String),
}

pub struct Regex {
    pattern:  String,
    compiled: regex::Regex,
}

impl Regex {
    /// Constrói uma nova `Regex`. Erro se pattern inválido.
    pub fn new(pattern: &str) -> Result<Self, RegexError>;

    /// Pattern original (string).
    pub fn pattern(&self) -> &str;

    /// Verifica se o regex matchea `text`.
    pub fn is_match(&self, text: &str) -> bool;
}

impl Hash      for Regex { /* via pattern */ }
impl PartialEq for Regex { /* via pattern */ }
impl Eq        for Regex {}
impl Clone     for Regex { /* re-construção; pattern já válida */ }
impl Debug     for Regex { /* { pattern: "..." } */ }
```

---

## Semântica

- `Regex::new(pattern)`: valida pattern via
  `regex::Regex::new`; erro contextual coerente.
- `pattern()`: retorna `&str` original (referência ao field
  interno).
- `is_match(text)`: delega a `compiled.is_match(text)`. Custo
  amortizado O(|text|) por invocação (sem cache cristalino).
- `Hash`: delega a `String::hash(&self.pattern)`. Determinismo:
  mesma pattern → mesmo hash; mesma pattern em 2 instances →
  mesmo hash.
- `PartialEq`: `self.pattern == other.pattern`. **Não** compara
  `compiled` (opaco; mesma pattern compila idêntico per crate).
- `Eq`: marker — `PartialEq` é reflexive/symmetric/transitive
  via String.
- `Clone`: `Regex::new(&self.pattern).expect("pattern previamente
  válida")`. Pattern já validada no `new` original; `expect`
  documentado.
- `Debug`: imprime `Regex { pattern: "..." }`; oculta `compiled`.

---

## Invariantes

- `self.pattern` foi aceite por `regex::Regex::new` no momento
  da construção. Mutação interna proibida (sem `&mut self`
  methods).
- `self.compiled` corresponde sempre a `self.pattern`. Não
  reconstruir externamente.
- `Hash` é estável durante a vida da instância (pattern
  imutável).

---

## Tests obrigatórios

- `regex_new_valido_ok` — `Regex::new("a+b")` retorna `Ok`.
- `regex_new_invalido_err` — `Regex::new("[")` retorna `Err`
  com mensagem coerente.
- `regex_hash_determinismo` — mesma pattern em 2 instances
  produz mesmo hash.
- `regex_is_match_basico` — `Regex::new("\\d+")?.is_match("abc123")
  == true`; `"abc"` == false.
- `regex_eq_via_pattern` — instances distintas com mesma
  pattern são `==`; patterns diferentes são `!=`.
- `regex_clone_preserva_semantica` — `r.clone()` produz
  instance funcionalmente idêntica.

---

## Consumers

- `entities::selector::Selector::Regex(Regex)` — variant
  estructural P209D.
- `entities::introspector::TagIntrospector::query` arm
  `Selector::Regex(_)` — **stub `vec![]` documentado** per
  P209A A3 (cristalino single-pass não tem Content text
  durante query phase; semântica funcional fica deferred).

---

## Sobre paridade

Vanilla `regex::Regex` é re-exportado por
`typst-utils` ou similar e usado directamente em
`Selector::Regex(Regex)` sem wrapper. Cristalino isola via
wrapper L1 por 3 razões:

1. **Allowlist L1 política** (ADR-0029): wrapper isola dep em
   1 ponto; consumers transitivos não importam `regex` directo.
2. **Hash/Eq/PartialEq manuais**: `regex::Regex` não deriva;
   wrapper materializa manualmente via pattern.
3. **Swap futuro**: pattern wrapper L1 permite trocar `regex`
   por `regex-lite` ou outro motor sem refactor cross-modular.

---

## Não-objectivos

- Não materializa query `is_match` cristalino sobre Content text
  (semântica funcional de `Selector::Regex` é stub deferred per
  P209D C5).
- Não wrappa toda a API de `regex` (apenas `is_match` actualmente;
  outros métodos como `captures`/`replace` adicionam-se sob
  demanda).
- Não permite mutação interna de `pattern`/`compiled`.
- Não cache `is_match` per `(pattern, text)`; consumers que
  precisem disto materializam cache próprio.

---

## Cross-references

- ADR-0077 PROPOSTO (P209D) — política de dep regex.
- ADR-0076 PROPOSTO (M9c) — Bloco VI Selector extensions.
- P209A C2 — Caminho A fixado (`regex` full Unicode).
- P209A C1 — estrutura Selector::Regex(Regex) (wrapper).
- ADR-0029 (allowlist L1 política).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-05-12 | P209D (M9c — Bloco VI variant 5/5): novo módulo `entities::regex` com wrapper L1 sobre `regex::Regex`. Hash/Eq/PartialEq manuais via pattern string; Clone manual via re-construção. Consumer único: `Selector::Regex(Regex)`. | `regex.rs`, `regex.md`, `crystalline.toml`, `01_core/Cargo.toml`, `Cargo.toml`, ADR-0077 |
