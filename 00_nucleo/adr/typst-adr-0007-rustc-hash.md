# ⚖️ ADR-0007: Substituição de `rustc_hash` por `std::collections`

**Status**: `REVOGADO`
**Revogado por**: ADR-0018
**Data**: 2026-03-23

---

## Contexto

O diagnóstico do Passo 4 revelou 8 ocorrências de `FxHashMap` e
`FxHashSet` (da crate `rustc_hash`) no parser, usadas para detecção
de duplicados em listas de parâmetros e padrões de destructuring.

`rustc_hash` fornece um hasher (`FxHasher`) sem resistência a
ataques DoS (HashDoS), mais rápido que o `SipHash-1-3` padrão do
Rust para chaves pequenas.

O parser usa estas estruturas exclusivamente para conjuntos de
identificadores em contextos com N pequeno (parâmetros de função,
variáveis de destructuring). A diferença de performance entre
`FxHasher` e `SipHash` neste contexto é sub-microsecond e
imperceptível no tempo total de compilação de qualquer documento.

`rustc_hash` é uma dependência de performance, não de correctitude.
A sua presença em L1 não é justificada pelo uso actual.

---

## Decisão

`FxHashMap` e `FxHashSet` são substituídos por
`std::collections::HashMap` e `std::collections::HashSet` em
`01_core/rules/parse.rs`.

`rustc_hash` é removido de `01_core/Cargo.toml`.

Nenhuma alteração de lógica — apenas substituição de tipo.

---

## Critério de revisão

Se profiling futuro em L3/L4 (com documentos reais) demonstrar que
o hasher do `std` é um gargalo mensurável no pipeline de parse,
a solução correcta é passar um hasher customizado via parâmetro de
tipo ou substituir o hasher do `std` via `BuildHasher` — sem
necessidade de `rustc_hash` em L1.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/rules/parse.md` | Sem alteração — detalhe de implementação não visível na interface |

---

## Consequências

**Positivas**: L1 sem dependência de crate de performance; `rustc_hash`
removido do `Cargo.toml` de `typst-core`.

**Negativas**: Regressão teórica de performance em operações de
hash para N pequeno — imperceptível no contexto de uso actual.

**Neutras**: `std::collections::HashMap` e `HashSet` são a escolha
padrão de Rust — sem surpresas para qualquer contribuidor futuro.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Autorizar `rustc_hash` em `[l1_allowed_external]` | Zero regressão de performance | Dependência de performance em L1; precendente para outras crates de optimização |
| `HashMap` com `BuildHasherDefault<FxHasher>` em L1 | Hasher customizável | Ainda exige `rustc_hash` como dependência |

---

## Referências

- Diagnóstico Passo 4 — 8 ocorrências em parser.rs
- `rustc_hash`: https://github.com/rust-lang/rustc-hash
