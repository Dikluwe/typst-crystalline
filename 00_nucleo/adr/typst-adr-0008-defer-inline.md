# ⚖️ ADR-0008: Inlining de `typst_utils::defer!` em L1

**Status**: `PROPOSTO`
**Data**: 2026-03-23

---

## Contexto

O diagnóstico do Passo 4 revelou que `parser.rs` usa a macro
`defer!` da crate `typst_utils` para garantia de cleanup no estilo
RAII — o equivalente Rust de um bloco `defer` de Go ou um
destrutor de guarda.

A crate `typst_utils` contém também `default_math_class` (tratada
em ADR-0009) e outras utilidades do ecossistema Typst que não
pertencem a L1. Autorizar `typst_utils` em `[l1_allowed_external]`
para aceder apenas a `defer!` traria toda a crate como dependência.

A macro `defer!` é uma implementação simples de ~10 linhas sem
dependências externas — candidata directa a inlining.

---

## Decisão

A macro `defer!` é copiada para `01_core/src/utils.rs` como
utilitário interno de L1:

```rust
// 01_core/src/utils.rs
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/parse.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-23

/// RAII guard que executa uma closure ao sair do scope.
///
/// Inlinado de `typst_utils::defer!` — ADR-0008.
/// Origem: https://github.com/typst/typst/blob/main/crates/typst-utils/src/lib.rs
struct DeferGuard<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Drop for DeferGuard<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

/// Executa `$expr` quando o scope actual termina.
///
/// ```rust
/// let _guard = defer!(println!("cleanup"));
/// ```
macro_rules! defer {
    ($expr:expr) => {
        let _guard = $crate::utils::DeferGuard(Some(|| $expr));
    };
}
pub(crate) use defer;
```

`typst_utils` é removido de `01_core/Cargo.toml` após ADR-0009
(que trata o outro uso da crate).

---

## Nota de atribuição

A implementação é derivada de `typst_utils` (licença Apache-2.0).
O comentário de origem é mantido no ficheiro para rastreabilidade.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/rules/parse.md` | Documentar `defer!` como utilitário interno; referenciar ADR-0008 |

---

## Consequências

**Positivas**: `typst_utils` removida como dependência de L1 (após
ADR-0009); L1 sem dependência de crate do ecossistema Typst.

**Negativas**: Se `typst_utils::defer!` for actualizado upstream,
a versão inlinada não recebe a actualização automaticamente. Risco
negligenciável — a implementação é trivial e estável.

**Neutras**: `utils.rs` torna-se o lugar canónico para utilitários
internos de L1 que não pertencem a nenhuma entidade específica.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Autorizar `typst_utils` em `[l1_allowed_external]` | Sem cópia | Toda a crate entra como dependência |
| Usar `scopeguard` crate | Mais completo | Dependência externa desnecessária para um macro de 10 linhas |
| Remover `defer!` sem substituição | Simples | Pode alterar semântica de cleanup em edge cases |

---

## Referências

- Diagnóstico Passo 4 — ocorrências de `defer!` em parser.rs
- ADR-0009 — segundo uso de `typst_utils` (`default_math_class`)
- `typst_utils`: https://github.com/typst/typst/tree/main/crates/typst-utils
