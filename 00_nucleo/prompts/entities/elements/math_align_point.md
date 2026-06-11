# Prompt L0 — `entities/elements/math_align_point` — `MathAlignPointElem`
Hash do Código: 80501352

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_align_point.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait e regras
partilhadas: ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

> **Marcador unit** (largura de uso 10; `&` ponto de alinhamento). Incluído no
> Lote 2 (11 variantes element-shaped) — `…Elem` é struct vazio; o construtor
> ergonómico `Content::math_align_point()` substitui o variant unit anterior.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathAlignPointElem;
```

`Content::MathAlignPoint` (unit) → `Content::MathAlignPoint(Arc<MathAlignPointElem>)`.
Construtor ergonómico: `Content::math_align_point()`.

## `impl Element for MathAlignPointElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1632`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **terminal**: `Ok(Content::MathAlignPoint(Arc::new(self.clone())))` |
| `map_text` | **terminal** (math structural; `content.rs:2614`): `Content::MathAlignPoint(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` em struct unit → sempre igual (paridade `content.rs:1824`:
`(MathAlignPoint, MathAlignPoint) => true`).

## Critério

`plain_text` vazio; map_* terminais; igualdade trivial.
