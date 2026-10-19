# Relatório — typst-passo-852: `rect()` aceita `radius`

**Data:** 2026-07-23  
**Executor:** Kimi Code (agente principal).  
**Proveniência das medições:** working tree não commitado (sobre HEAD `dfe3c2282`).  
**Estado:** **implementado e validado**.

---

## 1. Problema

`#rect(radius: 10pt)` e `#rect(radius: (top-left: 15pt, ...))` divergiam do vanilla:

- **Vanilla:** aceita `radius` como `Length` uniforme ou dicionário por canto; renderiza cantos arredondados.
- **Cristalino (antes):** `error: argumento nomeado inesperado em rect(): 'radius'`.

Causa: `native_rect` não incluía `radius` na whitelist de argumentos.

---

## 2. Sonda

Código identificado:

- `01_core/src/engine/stdlib/shapes.rs:75` — `native_rect` só aceitava `width`, `height`, `fill`, `stroke`.
- `01_core/src/engine/stdlib/layout.rs:740` — `extract_corners_length_value` já extraía `Corners<Length>` de `Length` ou `Dict` (usado por `block()` e `box()`).
- `01_core/src/entities/geometry.rs:66` — `ShapeKind::RoundedRect { radii: Corners<Length> }` já existia.
- `01_core/src/entities/content.rs:1800` — `Content::shape` criava `ShapeElem` sem radius.

A infraestrutura de cantos arredondados já existia para `box()`; faltava ligá-la a `rect()` (e `square()`).

---

## 3. Implementação

Alterações:

- `01_core/src/entities/content.rs:1800` — adicionado `Content::shape_with_radius`, que converte `ShapeKind::Rect` em `ShapeKind::RoundedRect` quando o radius é não-zero; `Content::shape` mantém-se como wrapper para compatibilidade.
- `01_core/src/engine/stdlib/layout.rs:740` — `extract_corners_length_value` tornada `pub(crate)` para reutilização em `shapes.rs`.
- `01_core/src/engine/stdlib/shapes.rs:75` — `native_rect` passou a aceitar `radius` e usar `Content::shape_with_radius`.
- `01_core/src/engine/stdlib/shapes.rs:121` — `native_square` passou a aceitar `radius` (paridade vanilla; `square` é helper morfológico de `rect`).
- Testes adicionados em `01_core/src/engine/stdlib/mod.rs`:
  - `rect_radius_uniforme_produz_rounded_rect`
  - `rect_radius_zero_mantem_rect`

---

## 4. Validação

- `cargo test --workspace`: **4649 passed; 0 failed** (estado após P851–P853).
- `crystalline-lint .`: exit 0 (zero violations relevantes).

Comportamento observável:

- `#rect(radius: 10pt)` → aceite; produz `ShapeKind::RoundedRect`.
- `#rect(radius: (top-left: 5pt, top-right: 10pt, bottom-right: 5pt, bottom-left: 10pt))` → aceite.
- `#rect(radius: 0pt)` → mantém `ShapeKind::Rect` (sem custo de path arredondado).
- `#square(radius: 5pt)` → aceite.

Nota: a verificação geométrica exacta dos cantos (curva Bezier no PDF) depende do exportador, que já suporta `RoundedRect` (P242). Este passo validou a aceitação do argumento e a construção do `ShapeElem` correcto.

---

## 5. Próximo passo

O dono deve rever e fazer commit. Não há mais ações pendentes para o P852.
