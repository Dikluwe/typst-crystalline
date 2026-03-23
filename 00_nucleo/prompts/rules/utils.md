# Prompt L0 — `utils` (utilitários internos de L1)

**Camada**: L1
**Ficheiro**: `01_core/src/utils.rs`
**ADRs**: `00_nucleo/adr/typst-adr-0006-typst-timing.md`,
          `00_nucleo/adr/typst-adr-0008-defer-inline.md`

---

## Contexto

`utils.rs` é o módulo canónico para utilitários internos de L1 que não
pertencem a nenhuma entidade específica. Contém dois utilitários inlinados
de `typst_utils` (crate upstream do Typst):

1. **`timing_scope!`** — macro no-op que substitui `typst_timing::TimingScope`
   (ADR-0006). Os pontos de instrumentação estão registados em `01_core/DEBT.md`.
   Religação prevista no Passo 10.

2. **`defer`** — função RAII que executa uma closure ao sair do scope
   (ADR-0008). Inlinado de `typst_utils::defer`. Licença: Apache-2.0.

---

## Interface pública (crate-interna)

```rust
/// No-op macro que marca pontos de instrumentação.
/// ADR-0006: substituição de typst_timing::TimingScope.
macro_rules! timing_scope { ($name:expr) => { () }; }

/// RAII handle — executa `deferred` sobre `thing` quando dropped.
pub(crate) struct DeferHandle<'a, T, F: FnOnce(&mut T)> { ... }

/// Retorna um DeferHandle que corre `deferred(thing)` ao ser dropped.
pub(crate) fn defer<T, F: FnOnce(&mut T)>(
    thing: &mut T,
    deferred: F,
) -> DeferHandle<'_, T, F>;
```

---

## Critérios de verificação

**defer executa ao sair do scope**
- `defer(&mut value, |v| *v = 42)` → value == 42 após o scope

**defer com DerefMut**
- Modificações ao guard via `*guard += 100` + Drop `+1` → acumulação correcta

**timing_scope não panic**
- `timing_scope!("parse")` expande para `()` sem efeitos

---

## Notas

- `timing_scope!` é `pub(crate)` via `#[macro_export]` + `#[doc(hidden)]`
  apenas quando parser.rs existir. Até lá, é local ao módulo.
- `DeferHandle` e `defer` são `pub(crate)` — não fazem parte da API pública de L1.
- Quando ADR-0006 for religado no Passo 10, substituir `timing_scope!` pelo
  mecanismo de telemetria escolhido em cada local marcado com
  `// ADR-0006: timing removed — ver 01_core/DEBT.md`.
