# Prompt L0 — `infra/export/gradients/function_dict` — PDF Function dict
Hash do Código: fc627472

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/gradients/function_dict.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0087 (gradient linear PDF), ADR-0091 (CMYK strategy)

---

## Contexto

Emit do PDF Function dict para gradients:
- 2 stops → Type 2 (exponential interpolation linear).
- N > 2 stops → Type 3 stitching (sub-Functions Type 2 entre stops adjacentes).

Duas variants:
- `emit_function_dict` — RGB (3 componentes `/C0 [r g b] /C1 [r g b]`).
- `emit_function_dict_cmyk` — CMYK (4 componentes).

## Restrições estruturais

- L3 puro. Apenas formatação `format!` sobre stops já amostrados.
- `pub(crate)` — chamado por `super::super::builder::emit_gradient_objects`.
- Self-contained (não depende de outros submódulos de gradients/).
- `sub_first_id: &mut usize` — caller pré-aloca IDs para sub-Functions Type 2.

## Interface

```rust
pub(crate) fn emit_function_dict(
    stops: &[(f32, f32, f32)],
    function_id: usize,
    sub_first_id: &mut usize,
) -> (String, Vec<(usize, String)>);

pub(crate) fn emit_function_dict_cmyk(
    stops: &[(f32, f32, f32, f32)],
    function_id: usize,
    sub_first_id: &mut usize,
) -> (String, Vec<(usize, String)>);
```

Returns `(main_dict_string, vec_of_sub_function_objects)`.

## Invariantes

- 2 stops → retorna `(dict, vec![])` (sem sub-Functions).
- N > 2 → retorna `(stitching_dict, sub_objs)` com `sub_objs.len() == n - 1`.
- Cada sub-Function tem ID único alocado por `*sub_first_id += 1`.

## Critérios de verificação

Tests `p263_emit_function_dict_*` em `super::super::tests`.
