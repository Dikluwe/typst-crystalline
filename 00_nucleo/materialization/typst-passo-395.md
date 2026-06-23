# Passo 395 — Modelagem de Tipos: `Value::Tiling` (M)

**Tipo**: Modelagem de tipos primitivos (L1 — pureza; zero I/O; expande enum `Value` fechado per ADR-0017).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389 + DEBT-62 + ADR-0017); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0017 (trava arquitetural — enum fechado; variant novo exige tipo migrado primeiro), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (graded scope-out).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `Value::Tiling` ausente, bloqueia `tiling()` + gradient combinado; DEBT-62 (read binário) depende de `Value::Bytes` (S futuro).

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

A sonda 389 identificou `Value::Tiling` como a **trava real recorrente** do portão ADR-0017. O enum `Value` é fechado (ADR-0017 + ADR-0026); cada variant novo exige:
1. Tipo subjacente projetado e migrado em L1 (pureza, alloc, semântica).
2. Injeção no enum com todos os `match` afetados (eval, layout, repr, cast, PartialEq).
3. Testes end-to-end do pipeline de valor.

O vanilla expõe `tiling(...)` como construtor de padrão de preenchimento (pattern fill) para gradientes e imagens. A morfologia linguagem é: `tiling(image, size: auto, relative: "self")` retorna um `Tiling` que pode ser usado como `fill` em shapes, boxes, e strokes.

Este passo é **abertura do portão ADR-0017** — não implementa `tiling()` ainda, apenas modela o tipo. O portão aberto permite:
- P396+ = `native_tiling` + `tiling()` consumer (M).
- P3xx = `Value::Bytes` (S), `Value::Decimal` (S), `Value::Duration` (S), `Value::Version` (S) — todos S, fluem sem parar.

---

## 2. Decisão de engenharia

### 2.1 — Estrutura do tipo `Tiling` (L1 entity)

Paridade vanilla `visualize/tiling.rs` + `visualize/gradient.rs` (tiling é usado como fill):

```rust
// entities/tiling.rs — tipo L1 puro
pub struct Tiling {
    pub body: TilingBody,           // Image | Gradient | Color
    pub size: Option<Size2D>,        // None ↔ auto (bounds do body)
    pub relative: TilingRelative,    // "self" | "parent"
    pub spacing: Option<Size2D>,     // gap entre repetições (None ↔ zero)
}

pub enum TilingBody {
    Image(ImageSource),   // reusa Image existente (P72-74)
    Gradient(Gradient),   // placeholder — Gradient é scope-out ADR-0054, mas TilingBody o antecipa
    Color(Color),         // fallback simples
}

pub enum TilingRelative {
    Self,    // relativo ao objeto preenchido
    Parent,  // relativo ao pai / página
}
```

**Decisão ADR-0017**: `TilingBody::Gradient` é **placeholder** — o tipo `Gradient` ainda não existe (scope-out ADR-0054). O variant existe no enum para paridade futura, mas seu único consumer inicial é `TilingBody::Color` e `TilingBody::Image`. Isso é **adiamento consciente**, não dívida — documentado no L0.

### 2.2 — Enum `Value::Tiling`

```rust
// entities/value.rs — novo variant
Tiling(Arc<Tiling>),  // Arc para cheap clone (paridade Pattern: Arc<[T]> em ADR-0026-R1)
```

**Por que Arc**: `Tiling` contém `ImageSource` (já Arc internamente) e potencialmente dados de gradient. Clone O(1) é invariante do Tekt para valores pesados.

### 2.3 — Impacto cross-module (match exhaustivo)

| Módulo | O que muda | Como |
|--------|-----------|------|
| `entities/value.rs` | +1 variant | `Tiling(Arc<Tiling>)` |
| `eval/repr.rs` | +1 arm | `Repr::Tiling(...)` ou string "tiling(...)" |
| `eval/cast.rs` | +1 arm | `Tiling → Tiling` (identity); `Color → TilingBody::Color` |
| `eval/ops.rs` | +1 arm | `==` por struct equality (paridade PartialEq) |
| `entities/paint.rs` | +1 variant | `Paint::Tiling(Tiling)` — usado em `ShapeElem::fill`; fallback `Color` em outros fills |
| `stdlib/visualize.rs` | +1 func futura | `native_tiling` (P396, não este passo) |
| `export.rs` | +1 arm | `Paint::Tiling` → `Color` fallback (scope-out ADR-0054 graded) |

> **Nota retroativa (medição ADR-0108):** a sonda do substrato confirmou que **não existe
> `enum Fill` isolado** no cristalino. `Fill` é `Style::Fill(Color)`
> (`01_core/src/entities/style.rs:43`) ou `Option<Paint>` nos consumers (`ShapeElem::fill`).
> Por isso a integração de `Tiling` foi feita via `Paint::Tiling`, não via `Fill::Tiling`.

**Decisão**: todos os match afetados são **adição de braço**, não refactor. O padrão é paralelo a `Value::Color` (P102) — tipo visual primitivo com pipeline de render.

### 2.4 — Paridade linguagem (ADR-0107)

No vanilla, `tiling` é um valor de preenchimento (fill). No cristalino:
- `Tiling` integra via `Paint::Tiling` (o wrapper `Paint` já serve de "fill" em `ShapeElem`). Outros elementos (`Block`, `Box`, `Text`) continuam com `Option<Color>` ou `Option<Paint>` conforme o caso; não existe um enum `Fill` isolado.
- A paridade não exige que `Tiling` seja construído pelo usuário neste passo (isso é P396).
- A paridade exige que o **tipo exista no pipeline** — eval conhece, layout aceita, repr mostra.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `tiling.md`

Novo em `00_nucleo/prompts/entities/tiling.md`:

- **Paridade**: `Tiling` ≡ vanilla `TilingElem` morfologicamente (fill pattern).
- **Substrato**: tipo entity L1 puro; zero I/O; Arc-wrapped para cheap clone.
- **Corpo**: `TilingBody` enum com 3 variants (Image/Gradient/Color); Gradient placeholder.
- **Relative**: `Self` (default) | `Parent`.
- **Size**: `Option<Size2D>` — `None` = auto (bounds do body).
- **Spacing**: `Option<Size2D>` — `None` = zero.
- **Scope-out**: `Gradient` em `TilingBody` é placeholder — consumer inicial só Image/Color.
- **Fill integration**: `Tiling` integra em `Paint::Tiling` em `entities/paint.rs`. Não existe enum `Fill` isolado no cristalino; `ShapeElem::fill` já usa `Option<Paint>`.
- **Teste**: construção `Tiling::new(Color::Rgb(...))` → `Value::Tiling` → repr → cast.

### A.2 — Prompt L0 `value.md` (extensão)

Secção aditiva em `00_nucleo/prompts/entities/value.md`:

- `Value::Tiling(Arc<Tiling>)` — novo variant.
- Derives: `Clone`, `PartialEq` (via Tiling equality).
- Não é `Copy` (Arc).
- Repr: `"tiling(...)"` ou struct detalhada.

### A.3 — CHECKPOINT

Parar. Apresentar `tiling.md` + extensão `value.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

### B.1 — Tipo entity `Tiling`

1. Criar `01_core/src/entities/tiling.rs`:
   - `Tiling` struct com fields.
   - `TilingBody` enum (Image/Gradient/Color).
   - `TilingRelative` enum (Self/Parent).
   - Derives: `Debug`, `Clone`, `PartialEq` (Gradient placeholder equality = false).
   - Constructor: `Tiling::new(body: TilingBody) -> Self`.

2. Criar `01_core/src/entities/tiling.md` (doc inline) ou doc-comments.

### B.2 — Variant `Value::Tiling`

1. `entities/value.rs`: adicionar `Tiling(Arc<Tiling>)` ao enum `Value`.
2. Atualizar `PartialEq` match.
3. Atualizar `Repr` match (repr string).
4. Atualizar `Cast` match (identity + Color fallback).

### B.3 — Fill integration

1. `entities/paint.rs`: adicionar `Paint::Tiling(Tiling)` ao enum `Paint` (substrato real do cristalino; não existe enum `Fill` isolado).
2. Atualizar consumers de `Paint` (em especial `emit_stroke_paint` em `export/stream.rs`) com braço `Tiling` — **fallback a Color** via `Tiling::to_color()` (paridade ADR-0054 graded).

### B.4 — Export stub

1. `export/stream.rs`: adicionar braço `Paint::Tiling` em `emit_stroke_paint` → emite `Color` fallback (paridade ADR-0054 graded — pattern fill PDF é scope-out futuro).

### B.5 — Testes

1. **Unit `entities/tiling.rs`** (4-6 tests):
   - `tiling_new_color` — construção com Color.
   - `tiling_new_image` — construção com Image (mock).
   - `tiling_equality` — PartialEq struct.
   - `tiling_clone` — Arc clone O(1).
   - `tiling_relative_default` — Self default.
   - `tiling_size_none` — auto default.

2. **Unit `entities/value.rs`** (3-4 tests):
   - `value_tiling_variant` — discriminação.
   - `value_tiling_repr` — repr string.
   - `value_tiling_cast_from_color` — Color → TilingBody::Color.
   - `value_tiling_partial_eq` — equality com outro Tiling.

3. **Unit `entities/paint.rs`** (2-3 tests):
   - `paint_tiling_variant` — `Paint::Tiling` exists.
   - `paint_tiling_to_color_fallback` — `Paint::Tiling(...).to_color()` retorna cor representativa.

4. **Integration/E2E** (2-3 tests):
   - `tiling_como_fill_em_box` — `#box(fill: tiling(red))` paridade morfológica (não render — apenas parse+eval aceita).
   - `tiling_repr_em_documento` — repr no output.

### B.6 — Linhagem

- `@prompt` aponta para `tiling.md` + `value.md`.
- `@prompt-hash` via `--fix-hashes`.
- Referência cruzada: ADR-0017 §"Variant novo" + ADR-0107 §"Paridade linguagem".

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar `native_tiling` — é P396 (consumer + stdlib).
- **Não** implementar render PDF de pattern fill — é scope-out ADR-0054 graded (emite Color fallback).
- **Não** implementar `Gradient` tipo real — é placeholder em `TilingBody`; Gradient próprio é passo futuro.
- **Não** tocar em `document`/`title`/`asset` — é P396 ou P397, fila limpa.
- **Não** adicionar `Value::Bytes`/`Decimal`/`Duration`/`Version` — são S, fluem depois deste M.
- **Não** quebrar invariantes de camada (L1 puro, zero I/O).

---

## 6. Critérios de aceitação

1. `Value::Tiling(Arc<Tiling>)` compila e participa de `match` exhaustivo em eval/layout/export.
2. `Tiling` struct tem `TilingBody` (Image/Gradient/Color) + `TilingRelative` + size/spacing.
3. `Paint::Tiling` existe em `entities/paint.rs`; consumers de `Paint` (em especial `emit_stroke_paint`) têm braço fallback `Color`.
4. Export emite Color fallback para Tiling (scope-out graded documentado).
5. Testes verdes (≥12 unit + 2-3 integration); lint zero; hashes propagados.
6. Inventário 148: `Value::Tiling` transita `ausente` → `implementado` (tipo); `tiling()` permanece `ausente` (consumer P396).
7. L0 salvo e hashado antes do código (protocolo de nucleação).

---

## 7. O que pode sair errado

- **`ImageSource` não é Arc internamente.** Mitigação: verificar P72-74; se não for, wrap em Arc no TilingBody::Image.
- **`Fill` enum não existe isolado — o cristalino usa `Option<Color>`/`Option<Paint>` conforme o elemento.** Medição ADR-0108: `rg 'enum Fill' 01_core/src/entities/` devolve 0 hits; `Fill` ocorre como `Style::Fill(Color)` (`style.rs:43`). Mitigação confirmada: integrar `Tiling` via `Paint::Tiling` em vez de criar enum `Fill` separado; evita refactor cross-module desnecessário.
- **Export não tem ponto de injeção para Fill.** Mitigação: adicionar braço no helper de fill do export (paralelo a Color).
- **Tentação de já fazer `native_tiling` junto.** Mitigação: um passo de cada vez; este é tipo, P396 é consumer.

---

## 8. Referências

- `typst-sonda-ausentes-ordem-passo-389.md` §2D — confirmação de `Value::Tiling` como trava real.
- ADR-0017 — trava arquitetural enum fechado.
- ADR-0107 — paridade linguagem vs mecânica.
- ADR-0029 — pureza L1.
- ADR-0054 — graded scope-out (Gradient placeholder, PDF pattern fill).
- P72-74 — `Image`/`ImageSource` baseline.
- P102 — `Color` como tipo visual primitivo (paradigma paralelo).

---

## 9. Nota sobre o Tekt

Este passo é o **portão ADR-0017** — não é feature user-facing direta, é **infraestrutura de tipos**. O custo é M (não S) porque toca o enum `Value` fechado, que afeta ~6 módulos com match exhaustivos. O benefício é **abrir o funil** — após P395, todos os tipos S pendentes (Bytes, Decimal, Duration, Version) fluem sem parar, e `tiling()`/gradient vira M de implementação pura (sem design de tipo).

Registar o tempo de ciclo (L0 → hash → código → teste → lint) como baseline de saúde do pipeline para passos de tipo (M puro vs XS de helper).
