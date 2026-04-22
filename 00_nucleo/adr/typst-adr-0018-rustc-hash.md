# ⚖️ ADR-0018: `rustc_hash` → `[l1_allowed_external]` — revoga ADR-0007

**Status**: `IMPLEMENTADO`
**Revoga**: ADR-0007
**Data**: 2026-03-23

---

## Contexto

A ADR-0007 substituiu `FxHashMap`/`FxHashSet` de `rustc_hash` por
`std::collections::HashMap`/`HashSet` no parser, com a justificação
de que `rustc_hash` era uma dependência de performance não justificada
em L1.

Essa decisão estava errada. A análise de `typst-library/foundations/`
revelou que `rustc_hash` é necessário para `Scope` (que é necessário
para `Module` real), o que obrigou a uma reavaliação.

---

## Por que ADR-0007 foi um erro

`rustc_hash` fornece `FxHasher` — um hasher sem resistência a
ataques DoS (HashDoS), optimizado para chaves pequenas como
identificadores de compiladores. É usado pelo próprio compilador
Rust internamente.

Propriedades relevantes para L1:
- Zero I/O
- Zero estado global mutável
- Zero efeitos colaterais
- Determinismo total — mesma entrada, mesmo hash, em qualquer ambiente

A ADR-0007 aplicou o critério "remover externo" sem verificar se o
externo violava algum princípio de L1. Não viola. `SipHash-1-3` do
`std` é resistente a HashDoS — propriedade relevante para servidores
web que recebem input adversarial, não para compiladores que
processam código de utilizador. A substituição por `std` custou
performance sem ganho arquitectural real.

**Lição registada**: O critério para `[l1_allowed_external]` não é
"é externo?" mas "viola pureza funcional de L1?". Crates sem I/O,
sem estado global mutável e sem efeitos colaterais são elegíveis
independentemente de serem "do std" ou não.

---

## Decisão

`rustc_hash` é adicionado a `[l1_allowed_external]`:

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
]
```

`FxHashMap` e `FxHashSet` são reintroduzidos em
`01_core/rules/parse.rs` substituindo os `std::collections::HashMap`
e `HashSet` introduzidos pela ADR-0007.

`rustc_hash` torna-se o padrão de hashing para todo L1 onde
performance de lookup é relevante. `std::collections::HashMap`
permanece válido para casos onde N é trivialmente pequeno ou
onde a legibilidade justifica.

---

## Impacto no parser (reversão de ADR-0007)

ADR-0007 fez 2 substituições em `parse.rs` (declarações de tipo)
e 16 substituições em usos. Reverter:

```rust
// Remover (introduzido por ADR-0007):
use std::collections::{HashMap, HashSet};

// Restaurar (original):
use rustc_hash::{FxHashMap, FxHashSet};
```

---

## Impacto em Scope (Passo 8+)

`Scope` em `typst-library/foundations/scope.rs` usa `indexmap` com
`rustc_hash` como hasher (`IndexMap<_, _, FxBuildHasher>`). Com
`rustc_hash` autorizado em L1, `Scope` pode migrar para L1 quando
`indexmap` também for autorizado (ADR-0019).

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/rules/parse.md` | Restaurar `rustc_hash`; referenciar ADR-0018 e revogação de ADR-0007 |

---

## Consequências

**Positivas**: Performance de hashing no parser restaurada; `rustc_hash`
disponível para `Scope` e outros tipos de L1 que precisem de hashing
eficiente.

**Negativas**: ADR-0007 foi trabalho que precisa de ser revertido.
O custo é baixo (search-replace), mas o processo produziu um ciclo
desnecessário.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Manter ADR-0007 (std no parser), ADR-0018 só para Scope | Consistência aparente | Inconsistência real — mesma crate com decisões opostas em L1 |
| Autorizar `rustc_hash` apenas para Scope | Scope funciona | Parser continua com performance degradada sem razão |

---

## Referências

- ADR-0007 — decisão revogada
- Análise foundations/ — `Scope` requer `rustc_hash`
- `rustc-hash`: https://github.com/rust-lang/rustc-hash
