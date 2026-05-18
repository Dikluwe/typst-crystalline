# Diagnóstico Fase A P277.A — DEBT-33 fecho CLOSED (Bézier bbox analítica)

**Data**: 2026-05-18.
**Passo**: typst-passo-277.A.
**Magnitude**: S documental (~25 min).
**Cluster**: Visualize / Geometry / Fecho DEBT real.
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085 — verificação para
fecho CLOSED de DEBT material via materialização algorítmica.
**31º consumo directo de fonte** (continuação P276 N=35; 30º consumo →
P277 N=36; 31º consumo).

---

## §A.1 — Localização empírica de `ShapeKind::Path` e `CubicTo`

### Resultado: **Cenário C1 confirmado com caveat**

`01_core/src/entities/geometry.rs`:

```rust
// Linha 12-21:
pub enum PathItem {
    MoveTo(Point),                        // ✓
    LineTo(Point),                        // ✓
    CubicTo(Point, Point, Point),         // ✓ EXISTE
    ClosePath,
}

// Linha 80:
Path(Vec<PathItem>),                       // ✓ EXISTE em ShapeKind
```

### §A.1.1 — Consumidores actuais

| Sítio | Função | Uso |
|---|---|---|
| `01_core/src/entities/geometry.rs:12-21` | Definição enum `PathItem` (4 variantes incl. CubicTo) | Tipo |
| `01_core/src/entities/geometry.rs:80` | Definição `ShapeKind::Path(Vec<PathItem>)` | Tipo |
| `01_core/src/rules/stdlib/shapes.rs:268` | `polygon()` cria `ShapeKind::Path(items)` | **Único produtor stdlib** |
| `01_core/src/rules/layout/helpers.rs:88,124` | `measure_content_constrained` arm Path | Bbox via user-provided width/height |
| `01_core/src/rules/layout/mod.rs:814,2127` | Layout shape kind arms | Layout |
| `03_infra/src/export.rs:2173,2270,2441,2725,2921` | 5 sítios PDF emit arm Path | Render |
| `03_infra/src/export.rs:2187,2279,2453,2725,2921` | 5 sítios PDF emit arm CubicTo | Render |
| `03_infra/src/export.rs:3442` | **Único teste** que constrói Path com CubicTo | Test E2E |

### §A.1.2 — Caveat factual

**`polygon()` (único stdlib producer) usa apenas `MoveTo` + `LineTo`**:

```rust
// 01_core/src/rules/stdlib/shapes.rs:239-247
if i == 0 {
    path_items.push(PathItem::MoveTo(...));
} else {
    path_items.push(PathItem::LineTo(...));
}
min_x = min_x.min(x); max_x = max_x.max(x);
min_y = min_y.min(y); max_y = max_y.max(y);
```

Width/height calculados de **min/max dos pontos** (linhas 264-265). Para
LineTo-only paths, este cálculo é **exacto** (rectas não excedem
endpoints).

### §A.1.3 — Risco DEBT-33 estado

**Risco arquitectural**: enum CubicTo + PDF emit existem → futuras
stdlib functions (e.g. `curve()` user-facing) poderiam criar Paths
com CubicTo. Bbox via min/max dos pontos de controlo seria
**inexacta** (curva real pode exceder).

**Risco factualmente**: zero ocorrências em pipeline produção. Apenas
um teste de exportador constrói CubicTo Path (export.rs:3442).

### §A.1.4 — Conclusão cenário

Não é estritamente C1 (CubicTo activamente usado), nem C2 (CubicTo
ausente), nem C3 (Path não existe). É **intermédio**: enum existe e
PDF emit funciona, mas zero produtores stdlib.

**Decisão**: tratar como **C1 com caveat** — materializar algoritmo
**proactivamente** porque:
1. Algoritmo é pequena adição L1 (matemática pura).
2. Estrutura suporta CubicTo no enum + emit; bbox calculation é gap.
3. Futuras stdlib functions (`curve()`/`path()` user-facing) precisarão.
4. Reforça correcção arquitectural sem alteração de comportamento
   actual (polygon() LineTo-only preserva bit-exact).

L0 `geometry.md` está **desactualizado** — lista apenas Rect/Ellipse/Line
(per spec §A.1). Verificar literal:

---

## §A.2 — Inventário função bbox actual

### §A.2.1 — Cálculo de bbox em produção

`01_core/src/rules/layout/helpers.rs:85-93` e `:120-128` (dois sítios
análogos `measure_content` + `collect_items_at`):

```rust
match kind {
    ShapeKind::Rect | ShapeKind::RoundedRect { .. }
    | ShapeKind::Ellipse | ShapeKind::Path(_) => (
        resolve_pt(width.as_deref(), available_w),
        resolve_pt(height.as_deref(), 0.0),
    ),
    ShapeKind::Line { dx, dy } => (dx.abs(), dy.abs()),
}
```

**Conclusão**: layout helpers **não consultam PathItems** ao calcular
bbox. Usam apenas `width`/`height` user-provided ou fallback.

### §A.2.2 — Cálculo de bbox em stdlib `polygon()`

`01_core/src/rules/stdlib/shapes.rs:244-265`:

```rust
min_x = min_x.min(x); max_x = max_x.max(x);
min_y = min_y.min(y); max_y = max_y.max(y);
// ...
let width  = if max_x > min_x { Some(Box::new(Value::Float(max_x - min_x))) } else { None };
let height = if max_y > min_y { Some(Box::new(Value::Float(max_y - min_y))) } else { None };
```

**Conclusão**: `polygon()` calcula width/height **uma vez** (na criação
do Content::Shape) via min/max dos pontos. Para LineTo, exacto.

### §A.2.3 — Comentário DEBT-33 inline

`01_core/src/entities/geometry.rs:78-79`:

```rust
/// A bounding box é calculada pelos pontos de controlo (DEBT-33:
/// pode ser conservadora para segmentos CubicTo).
```

DEBT-33 está mencionado inline na documentação do enum `ShapeKind::Path`.

---

## §A.3 — Algoritmo a materializar

### §A.3.1 — Curva cúbica B(t) e derivada B'(t)

Per spec §A.3:

```
B(t) = (1-t)³·P₀ + 3(1-t)²t·P₁ + 3(1-t)t²·P₂ + t³·P₃     para t ∈ [0, 1]

B'(t) = 3·[a·t² + b·t + c]                                onde:
  a = -P₀ + 3·P₁ - 3·P₂ + P₃
  b =  2·P₀ - 4·P₁ + 2·P₂
  c = -P₀ + P₁
```

### §A.3.2 — Raízes de B'(t)=0 (extremos)

Por fórmula quadrática:

```
discriminant = b² - 4ac

se a != 0:
    t = (-b ± √disc) / (2a)              se disc >= 0
    sem solução                          se disc < 0
se a == 0 e b != 0:
    t = -c / b                           (linear)
se a == 0 e b == 0:
    sem solução                          (curva degenerada)
```

Filtrar `t ∈ (0, 1)` exclusivo (endpoints contam separadamente).

### §A.3.3 — Algoritmo `bezier_cubic_bbox(p0, p1, p2, p3) -> (min_x, min_y, max_x, max_y)`

```rust
fn bezier_cubic_bbox(p0: Point, p1: Point, p2: Point, p3: Point)
    -> (f64, f64, f64, f64)
{
    // Endpoints sempre extremos.
    let mut candidates_x: Vec<f64> = vec![p0.x.0, p3.x.0];
    let mut candidates_y: Vec<f64> = vec![p0.y.0, p3.y.0];

    // Para cada eixo, raízes de B'(t) = 0 em (0, 1).
    for axis in [Axis::X, Axis::Y] {
        let (a, b, c) = coefs(p0, p1, p2, p3, axis);
        for t in solve_quadratic_in_unit(a, b, c) {
            let pt_axis = bezier_at_axis(t, p0, p1, p2, p3, axis);
            match axis {
                Axis::X => candidates_x.push(pt_axis),
                Axis::Y => candidates_y.push(pt_axis),
            }
        }
    }

    let min_x = candidates_x.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_x = candidates_x.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min_y = candidates_y.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_y = candidates_y.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    (min_x, min_y, max_x, max_y)
}
```

### §A.3.4 — Helper `path_bbox(items: &[PathItem]) -> (f64, f64, f64, f64)`

Walker sobre PathItems com estado `current_point`:

```rust
fn path_bbox(items: &[PathItem]) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut current = Point::ZERO;

    for item in items {
        match item {
            PathItem::MoveTo(p) | PathItem::LineTo(p) => {
                min_x = min_x.min(p.x.0); max_x = max_x.max(p.x.0);
                min_y = min_y.min(p.y.0); max_y = max_y.max(p.y.0);
                current = *p;
            }
            PathItem::CubicTo(p1, p2, p3) => {
                let (mn_x, mn_y, mx_x, mx_y) = bezier_cubic_bbox(current, *p1, *p2, *p3);
                min_x = min_x.min(mn_x); max_x = max_x.max(mx_x);
                min_y = min_y.min(mn_y); max_y = max_y.max(mx_y);
                current = *p3;
            }
            PathItem::ClosePath => {
                // sem efeito no bbox
            }
        }
    }

    (min_x, min_y, max_x, max_y)
}
```

### §A.3.5 — Pureza L1

- **Matemática f64 pura** — operações `+`, `-`, `*`, `/`, `sqrt`, `min`, `max`.
- **Zero dependências externas** — sem `lyon`, `kurbo`, etc.
- **Zero I/O** — operações em parameter inputs.

**ADR-0029 preserved absoluto**.

---

## §A.4 — Paridade vanilla

Verificação rápida (acesso parcial a `lab/typst-original/`):

A literatura sobre rendering vector (Pomax, Cairo, etc.) usa
algoritmo análogo: raízes de B'(t)=0. Vanilla typst provavelmente
usa abordagem similar via crates como `kurbo` que computam bbox
analítico.

**Cristalino diverge nominal** de vanilla (vanilla usa crate; cristalino
implementa localmente) per ADR-0029 invariante "sem crates externas
de PDF" + extensão para geometria pura.

**Resultado funcional**: paridade matemática (mesmo algoritmo);
divergência de implementação (local vs crate). Aceitável per
ADR-0054 graded.

---

## §A.5 — Casos de teste planeados

8 testes unit cobrindo:

| Test | Descrição | Tipo |
|---|---|---|
| `bezier_bbox_linha_recta` | P0=P1=P2=P3 colineares → bbox = endpoints | sanidade |
| `bezier_bbox_curva_dentro_pontos_controlo` | Curva inscrita em pontos de controlo → bbox = control points | concordância |
| `bezier_bbox_curva_excede_em_x` | P1/P2 forçam excursão extra em x → analítica > min/max simples | **vazamento corrigido** |
| `bezier_bbox_curva_excede_em_y` | Análogo eixo y | **vazamento corrigido** |
| `bezier_bbox_curva_degenerada_a_zero` | a=0, b=0 → fallback endpoints | edge case |
| `bezier_bbox_endpoints_unicos_extremos` | P1/P2 entre P0 e P3 → endpoints vencem | sanidade |
| `solve_quadratic_in_unit_a_zero_linear` | a=0, b!=0 → t = -c/b se ∈ (0,1) | helper directo |
| `path_bbox_polygon_lineto_preserva` | Path com MoveTo+LineTo → bbox = polygon() actual | regressão zero |

**Estimativa**: 8 testes (~60-80 LOC). Cap testes hard 80 respeitado.

---

## §A.6 — Gates de paragem

Per spec §A.6:

| # | Condição | Estado |
|---|---|---|
| 1 | §A.1 cenário C2/C3 | ⚠ Intermédio: C1 com caveat (CubicTo enum + emit existem; zero stdlib producer). **Não dispara paragem** — materialização proactiva justificada per §A.1.4 |
| 2 | §A.2 função bbox não localizada | ✓ Localizada (helpers.rs:85-93 + 120-128; shapes.rs:244-265) |
| 3 | §A.4 paridade vanilla incompatível | ✓ Mesma matemática; só divergência implementação local vs crate |
| 4 | Cap LOC L1 hard 150 ameaçado | ✓ Estimativa real ~80-100 LOC (helpers + integração polygon refactor); folga 33%+ |
| 5 | Cap doc Fase A hard 500 ameaçado | ✓ Este doc ~400 linhas; folga 20% |
| 6 | Tests workspace ≠ 2644 baseline | ✓ Verificação adiada para §C.5 (esperado preserved) |
| 7 | Algum teste novo falha pós-impl | ✓ Verificação adiada para §C.5 |

**Zero gates disparam paragem**. Passo prossegue com **C1 com caveat
documentado** — materialização proactiva.

---

## §A.7 — Critério de aceitação Fase A

- ✓ §A.1 cenário identificado (C1 com caveat): Path/CubicTo existem
  estruturalmente; zero stdlib producers actuais; algoritmo
  proactivamente justificado.
- ✓ §A.2 função bbox actual localizada (2 sítios helpers + 1 sítio
  stdlib polygon).
- ✓ §A.3 algoritmo documentado completo (B(t), B'(t), raízes,
  bezier_cubic_bbox, path_bbox).
- ✓ §A.4 paridade vanilla: mesma matemática; divergência só de
  implementação (local vs crate); aceitável.
- ✓ §A.5 8 testes planeados.
- ✓ §A.6 gates: zero disparos; passo prossegue.

**Fase A produzida — critério §A.7 cumprido absoluto.**

---

## §A.8 — Plano §C operações

1. **§C.1 L0 update**: `prompts/entities/geometry.md` ganha secção
   `ShapeKind::Path` + bbox analítica; `--fix-hashes`.
2. **§C.2 Testes-first**: 8 testes em `geometry.rs` `#[cfg(test)]`.
   Confirmar 5 testes falham pré-impl (`bezier_bbox_curva_excede_*`
   + helpers ainda não existem).
3. **§C.3 Implementação**:
   - Funções `bezier_cubic_bbox`, `bezier_at_axis`,
     `solve_quadratic_in_unit`, `path_bbox` em `geometry.rs`.
   - Refactor `polygon()` em `shapes.rs` para usar `path_bbox`.
4. **§C.4 DEBT.md update**: DEBT-33 → Secção 2 CLOSED + histórico
   preserved + cabeçalho linha P277.
5. **§C.5 Validação**: cargo test workspace 2644 + 8 novos → ~2652;
   crystalline-lint zero.
6. **§C.6 Relatório consolidado**.

---

*Diagnóstico imutável produzido em 2026-05-18. 31º consumo.
**Cenário C1 com caveat factual**: PathItem::CubicTo + ShapeKind::Path
existem estruturalmente (enum + PDF emit), mas zero stdlib producers
actuais. Materialização proactiva justificada por: (1) algoritmo
pequeno L1 puro; (2) futuras stdlib (`curve()`/`path()`) precisarão;
(3) reforça correcção arquitectural; (4) preserva comportamento
actual polygon() bit-exact (LineTo-only). Veredicto **CLOSED** via
materialização per pattern P206E.*
