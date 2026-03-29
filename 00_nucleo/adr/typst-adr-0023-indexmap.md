# ⚖️ ADR-0023: `indexmap` → `[l1_allowed_external]`

**Status**: `IMPLEMENTADO`
**Data**: 2026-03-27

---

## Contexto

`Scope` em `typst-library/foundations/scope.rs` usa
`IndexMap<EcoString, Binding>` como estrutura de dados central.
`IndexMap` é um mapa com ordem de inserção preservada — importante
para `Scope` porque a ordem de declaração de bindings em Typst
é semanticamente significativa (ex: ordem de importação, ordem
de definição de funções).

`rustc_hash` já está autorizado em L1 (ADR-0018). O `IndexMap`
em `Scope` usa `FxBuildHasher` como hasher — compatível com a
autorização existente.

---

## Diagnóstico obrigatório antes de decidir

```bash
# Dependências externas de scope.rs
grep "^use\|^extern" \
  lab/typst-original/crates/typst-library/src/foundations/scope.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Confirmar que IndexMap usa FxBuildHasher
grep -n "IndexMap\|FxBuildHasher\|rustc_hash" \
  lab/typst-original/crates/typst-library/src/foundations/scope.rs \
  | head -10

# Versão de indexmap usada no original
grep "indexmap" lab/typst-original/Cargo.toml
grep "indexmap" lab/typst-original/crates/typst-library/Cargo.toml
```

**Reportar output antes de continuar.**

---

## Análise de pureza

| Propriedade | Estado |
|-------------|--------|
| Zero I/O | ✓ — estrutura de dados em memória |
| Zero estado global mutável | ✓ |
| Determinismo total | ✓ — ordem de inserção preservada deterministicamente |
| Dependências transitivas | ✓ — `hashbrown` (já usado pelo `std`) |

`IndexMap` é conceptualmente equivalente a `Vec<(K, V)>` com
lookup O(1) — uma estrutura de dados, não infraestrutura.
A ordem de inserção é uma propriedade de domínio para `Scope`.
`HashMap` do `std` não substitui porque não preserva ordem.

---

## Decisão

`indexmap` é adicionado a `[l1_allowed_external]`:

```toml
[l1_allowed_external]
rust = [
    "thiserror",
    "comemo",
    "unicode_ident",
    "unicode_math_class",
    "unicode_script",
    "unicode_segmentation",
    "rustc_hash",
    "time",
    "indexmap",  # ADR-0023 — ordem de inserção em Scope; sem substituto no std
]
```

`indexmap` usa `FxBuildHasher` de `rustc_hash` (já autorizado) —
sem nova crate de hashing.

---

## O que esta ADR não decide

- A representação interna completa de `Scope` — decidida no Passo 11
  após diagnóstico completo de `scope.rs`
- Se outros tipos de `foundations/` também usam `indexmap` — avaliar
  caso a caso com ADR separada se necessário

---

## Consequências

**Positivas**: `Scope` pode migrar para L1 com a sua representação
interna real; ordem de declaração de bindings preservada.

**Negativas**: Nova crate em `[l1_allowed_external]` — justificada
porque `std` não tem equivalente com ordem de inserção.

**Neutras**: `indexmap` já é dependência transitiva de vários
pacotes comuns no ecossistema Rust.

---

## Referências

- ADR-0018 — `rustc_hash` em L1 (hasher usado por IndexMap em Scope)
- ADR-0016 — adiamento de eval() e estratégia typst-library
- `indexmap`: https://github.com/indexmap-rs/indexmap
