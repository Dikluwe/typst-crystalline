# Prompt L0 — entities/corners
Hash do Código: 6ff888b6

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/corners.rs`
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0037
(coesão por domínio), ADR-0079 PROPOSTO (Layout Fase 5 roadmap;
Categoria A.4 P231 graded → P242 materializado parcial),
ADR-0081 IMPLEMENTADO parcial 3/5 (M9d / M7+5 P242).

## Contexto

`Corners<T>` é um tipo geométrico genérico que agrupa quatro
valores indexados por canto (`top_left` / `top_right` /
`bottom_right` / `bottom_left`). Materializado em P242 (M9d /
M7+5, ADR-0081 IMPLEMENTADO parcial 3/5) como suporte estrutural
para `ShapeKind::RoundedRect { radii: Corners<Length> }` +
refino `Content::Block.radius` + `Content::Boxed.radius`
`Option<Length>` → `Corners<Length>` (per-corner).

Réplica simplificada de
`lab/typst-original/crates/typst-library/src/layout/corners.rs`,
reduzida ao essencial para o consumer cristalino actual.

## Forma

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Corners<T> {
    pub top_left:     T,
    pub top_right:    T,
    pub bottom_right: T,
    pub bottom_left:  T,
}
```

**Ordem dos campos** sentido horário começando top-left
(paridade vanilla). Útil para iteração que percorre cantos em
sequência natural (e.g. Bezier path generation no PDF exporter
P242).

## Métodos

```rust
impl<T> Corners<T> {
    /// Constrói `Corners` com cada canto independente.
    pub fn new(top_left: T, top_right: T, bottom_right: T, bottom_left: T) -> Self;
}

impl<T: Clone> Corners<T> {
    /// Constrói `Corners` com o mesmo valor em todos os cantos.
    pub fn uniform(value: T) -> Self;
}

impl<T: Default> Default for Corners<T>;
```

## Casos canónicos

```
Corners::new(1.0, 2.0, 3.0, 4.0)
  → { top_left: 1.0, top_right: 2.0, bottom_right: 3.0, bottom_left: 4.0 }

Corners::uniform(5.0)
  → { top_left: 5.0, top_right: 5.0, bottom_right: 5.0, bottom_left: 5.0 }

Corners::<f64>::default()
  → { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
```

## Sub-padrão #14 "Tipo entity em ficheiro próprio" N=5 → 6 cumulativo

Aplicações cumulativas pré-P242:
- P156C — `Sides<T>` em `entities/sides.rs`.
- P156E — `Parity` em `entities/parity.rs`.
- P156I — `Dir` em `entities/dir.rs`.
- P159A — `BibEntry` em `entities/bib_entry.rs`.
- P159C — `CitationForm` em `entities/citation_form.rs`.
- **P242 — `Corners<T>` em `entities/corners.rs`** (N=6 cumulativo).

Coerência arquitectural: tipos entity com semantic estrutural
isolada ganham ficheiro próprio.

## Ver também

- `entities/sides.rs` — `Sides<T>` (ortogonal: lados vs cantos).
- `entities/geometry.rs` — `ShapeKind::RoundedRect { radii:
  Corners<Length> }` (consumer principal P242).
- `entities/content.rs` — `Content::Block.radius:
  Corners<Length>` + `Content::Boxed.radius` (refino P242 face
  P231 `Option<Length>`).
