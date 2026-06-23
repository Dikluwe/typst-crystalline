# P427 — PDF Writer Shapes: emissão de FrameItem::Shape no PDF (M)

**Hash L0**: `typst-passo-427.md` | **Camada**: L3 (PDF Export / Infra) | **ADR**: ADR-0107, ADR-0108, ADR-0109

---

## 1. Contexto

P427 é o primeiro passo do ciclo PDF + Layout (Opção A pós-P426). Foca na emissão de formas geométricas básicas (`Rect`, `Ellipse`, `Line`, `Polygon`) do `FrameItem::Shape` para o stream PDF.

---

## 2. ADR-0107 — Paridade linguagem

O contrato é **semântico/sintático/morfológico**:
- `#rect(width, height, fill, stroke)` → retângulo preenchido/contornado no PDF
- `#circle(radius)` → círculo no PDF
- `#ellipse(width, height)` → elipse no PDF
- `#polygon((x, y), ...)` → polígono fechado no PDF
- `#line(start, end)` → linha no PDF

**Mecânica livre**: ordem de operadores PDF, precisão de floats, path simplificado.

---

## 3. ADR-0108 — Medir antes de decidir (FASE A.0)

6 sondas obrigatórias antes de qualquer código:

1. `FrameItem::Shape` existe no enum? (esperado: sim, desde P78)
2. `ShapeKind` tem `Rect` / `Ellipse` / `Line` / `Polygon` / `Path`? (esperado: sim)
3. PDF writer (`infra/export/builder.rs`) consome `FrameItem::Shape`? (esperado: stub ou ausente)
4. Quais `ShapeKind` já são emitidos no PDF? (esperado: 0 ou parcial)
5. `Paint` (fill/stroke) já é emitido para shapes? (esperado: parcial — `FrameItem::Line` tem cor desde P285)
6. `Transform` de shape já é aplicado no PDF? (esperado: sim, via `FrameItem::Group` P84.6)

**Critério de reclassificação**: se >3 `ShapeKind` já emitidos → reclassificar S (refino). Se nenhum emitido → confirmar M.

---

## 4. ADR-0109 — Atomização forma B

- `entities/layout_types.rs` — `FrameItem::Shape` já existe (dado; não tocar)
- `entities/geometry.rs` — `ShapeKind` já existe (dado; não tocar)
- `infra/export/builder.rs` — `emit_shape()` como free function dispatcher
- `infra/export/shape_emit.rs` (novo módulo) — `emit_rect()`, `emit_ellipse()`, `emit_polygon()`, `emit_line()` free functions

**Opção A rejeitada**: `impl FrameItem { fn emit_pdf(...) }` em `layout_types.rs` — evita lógica de PDF no arquivo de dados.

---

## 5. Decisão arquitetural

| Opção | Descrição | Magnitude | Risco |
|-------|-----------|-----------|-------|
| α — Monolítico | `emit_shape()` com match interno em `builder.rs` | M | Médio |
| **β — Módulo separado** | `shape_emit.rs` com free functions; `builder.rs` chama dispatcher | **M** | **Baixo** ✅ |
| γ — Visitor pattern | `ShapeEmitter` trait | M | Alto |

**Recomendado β**: reusa padrão P424 (`emit_link_annotations` free function em `builder.rs`); shapes ficam em módulo separado para crescer (path, curve, etc.).

> **Nota retroativa (correcção de deriva):** a opção β da spec (criar `shape_emit.rs` como módulo separado) estava em conflito com o L0 vigente de `stream.md` (hash `9acca994`), que determina não subdividir. Na execução, a opção β foi descartada por esse motivo e a emissão de shapes manteve-se em `stream.rs`. A spec não devia ter proposto a subdivisão como opção preferida sem validar o L0 vigente.

---

## 6. Algoritmo por ShapeKind

### 6.1 Rect
```
x y w h re
B  (fill+stroke) | b (stroke+close) | f (fill) | S (stroke only)
```
Ou path manual: `m x y → l x+w y → l x+w y+h → l x y+h → h → B/b/f/S`

### 6.2 Ellipse / Circle
4 cubic Bézier curves (kappa approximation `0.5522847498`):
```
m cx cy-r
 c cx+k*(r) cy-r  cx+r cy-k*(r)  cx+r cy
 c cx+r cy+k*(r)  cx+k*(r) cy+r  cx cy+r
 c cx-k*(r) cy+r  cx-r cy+k*(r)  cx-r cy
 c cx-r cy-k*(r)  cx-k*(r) cy-r  cx cy-r
h
B / b / f / S
```

### 6.3 Line
```
x0 y0 m
x1 y1 l
S
```

### 6.4 Polygon
```
x0 y0 m
x1 y1 l
x2 y2 l
...
h
B / b / f / S
```

### 6.5 Paint (fill / stroke)
- Fill: `r g b rg` (RGB) + operador `f` / `F` / `B` / `b`
- Stroke: `r g b RG` (RGB) + operador `S` / `B` / `b`
- Sem fill → `S` ou `b` (stroke only)
- Sem stroke → `f` ou `B` (fill only)
- Ambos → `B` (fill+stroke) ou `b` (fill+stroke+close)

---

## 7. Scope-out explícito

- `ShapeKind::Path` (Bézier curves) — P293/P294 já cobre; reutilizar se existir
- Gradient fill — scope-out
- Pattern fill (`Tiling`) — scope-out
- `stroke` dash/cap/join ricos — scope-out; emitir `RG`/`rg` básico
- Clip path em shapes — scope-out
- `ShapeKind::RoundedRect` (P242) — incluir se viável; senão scope-out

---

## 8. Critério de fecho

- [x] Sonda A.0 executada e reclassificação confirmada (**S** — refino; emit já existia)
- [ ] `infra/export/shape_emit.rs` criado com free functions — **scope-out**: requer atualização de L0 `infra/export/stream.md`
- [ ] `emit_shape()` dispatcher em `builder.rs` — **scope-out**: idem
- [x] 4+ ShapeKind emitidos no PDF (Rect, Ellipse, Line, Polygon/Path)
- [x] Paint (fill/stroke) aplicado corretamente
- [x] Tests E2E de PDF com shapes verdes
- [x] `cargo check -p typst-infra` passa
- [x] `crystalline-lint` zero drift (apenas warnings V7 de prompts órfãos preexistentes)
- [x] Commit realizado

---

## 9. Relatório de Execução

Ver `typst-passo-427-relatorio.md`.

---

## 10. Próximo passo

P428 — PDF Writer Images: embed PNG/JPEG no PDF (M)
