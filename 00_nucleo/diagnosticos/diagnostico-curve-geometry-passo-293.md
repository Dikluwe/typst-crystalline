# Diagnóstico — Fase A do Passo 293 (`P-curve-geometry`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-293.md`
**Origem**: P292 §9.2 ranking; P293 é o **primeiro passo ortogonal
pós-série cumulativa P288-P292**.
**Novidade metodológica**: secção A.0.0 inaugural ("clarificação de
scope") obrigatória porque spec partiu de referência arquitectural
ambígua P282 §6.

---

## A.0.0 — Clarificação de scope (inaugural)

### A.0.0.1 — Verificação literal do estado actual

#### Inspecção 1: `01_core/src/entities/geometry.rs`

`PathItem` enum (`geometry.rs:11-21`):

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum PathItem {
    MoveTo(Point),
    LineTo(Point),
    CubicTo(Point, Point, Point),  // ← (control1, control2, end)
    ClosePath,
}
```

**4 variants** — incluindo `CubicTo` para curvas Bézier cúbicas
(P277 DEBT-33 fecho).

`ShapeKind::Path(Vec<PathItem>)` (`geometry.rs:81`) — aceita lista
livre de PathItem.

#### Inspecção 2: `01_core/src/rules/stdlib/shapes.rs`

`native_polygon` (linha 225-260) — única função stdlib que produz
`PathItem`:

```rust
path_items.push(PathItem::MoveTo(Point { ... }));    // 1º vertex
path_items.push(PathItem::LineTo(Point { ... }));    // restantes
// ...
path_items.push(PathItem::ClosePath);
```

**Usa apenas 3 dos 4 variants** — **`CubicTo` é completamente
ignorado**. Não há `native_path`, `native_curve`, ou método
auxiliar que produza `CubicTo`.

#### Inspecção 3: `03_infra/src/export.rs`

`grep "CubicTo" 03_infra/src/export.rs` → **3 hits funcionais**:
- Linha 2375: emit top-level `q ... c ... S Q\n` (PDF operator `c`
  para cubic Bézier).
- Linha 2457: emit `emit_shape_path_local` (em Group).
- Linha 2629: emit clip path.

Formato PDF: `{cx1:.2} {cy1:.2} {cx2:.2} {cy2:.2} {ex:.2} {ey:.2} c\n`
— PDF spec standard para cubic Bézier (3 control points).

**Emit consumer COMPLETAMENTE materializado** — só falta caminho de
entrada via stdlib.

#### Inspecção 4: `lab/typst-original/.../visualize/curve.rs`

Vanilla `CurveElem` expõe:
- `curve.move((x, y))` — MoveTo.
- `curve.line((x, y))` — LineTo.
- `curve.quadratic(c, end)` — Quadratic Bézier (2 control points).
- **`curve.cubic(c1, c2, end)` — Cubic Bézier (3 control points).**
- `curve.close()` — ClosePath.

Cristalino tem `CubicTo` no enum (paralelo `curve.cubic`); **mas
não tem `Quadratic`** (paralelo `curve.quadratic`). Tabela A.7
linha 194 lista feature como `implementado⁺` (aproximação por
ADR-0054).

### A.0.0.2 — Hipóteses + descoberta nova H6

| Hipótese (spec) | Veredicto após inspecção |
|---|---|
| H1 — Cubic operations | ❌ Descobre-se nada (CubicTo *existe* mas inerte) |
| H2 — PDF emit cubic | ❌ Já implementado (linhas 2375/2457/2629) |
| H3 — Features adjacentes | ❌ Frente é genuinamente curve, não desvia |
| H4 — Refino numérico | ❌ Bbox já O(1) analítica (P277) |
| H5 — Mix | ❌ Não há mix material |
| **H6 NOVA** | ✅ **Activação posterior de `PathItem::CubicTo` via stdlib** — variant inerte vs cascade entrada |

**Descoberta empírica**: as 5 hipóteses da spec **não capturam o
estado real**. A hipótese genuína é **H6 — activação posterior de
feature graded** análoga arquitectural directa a P285-P292 mas para
`PathItem` em vez de `Style`.

### A.0.0.3 — Decisão sobre continuação P293

**Cenário H6** (não listado na spec → cenário novo): P293 prossegue
com scope **clarificado empiricamente como activação posterior de
`PathItem::CubicTo`**. Materialização:

1. Adicionar `native_curve` stdlib que aceita argumentos para
   construir `Vec<PathItem>` incluindo `CubicTo`.
2. Registar em `make_stdlib`.
3. Testes unitários + integração PDF.

**Hash `export.rs` preservado** — emit já existe.

### A.0.0.4 — Honestidade epistémica registada

P293 spec partiu de "ADR-0078 sub-fase b" (P282 §6) — **referência
arquitectural ambígua**. ADR-0078 cobre column flow, não curve.
A.0.0 inaugural inspeccionou literalmente e descobriu **hipótese H6
não listada**: activação posterior de `CubicTo` (variant existe,
emit existe, mas stdlib não constrói).

**Refutação significativa genuína da spec** (vs P292 §A.2.3 onde
refutação foi estructuralmente forçada). **§8.3 candidato a N=6
cumulativo** — promoção condicional em §3 ponto 3.

**Template A.0.0 inaugurado**: passos futuros com scope ambíguo na
origem devem inspeccionar literalmente antes de assumir hipótese.

---

## A.0 — Potencial de reuso ADR-0098

### A.0.1 — Verificação literal

`grep "curve\|cubic\|CubicTo" 03_infra/src/export.rs`:

| Hits | Classificação |
|---|---|
| 3 hits em `CubicTo` (linhas 2375/2457/2629) | **Emit consumer PDF já materializado** desde antes (provavelmente P78/P277 ambiente Path) |
| 0 hits em `curve` literal | n/a |

**Paradigma consumer**: emit PDF lê `PathItem::CubicTo(p1, p2, p3)`
directamente via match arm na lista `Vec<PathItem>` capturada em
`FrameItem::Shape { kind: ShapeKind::Path(...), ... }`. **Paradigma
P136 vigente** (capture em FrameItem; emit consume sem chain).

### A.0.2 — Classificação ADR-0098

| Critério | Veredicto |
|---|---|
| Emit consome via `FrameItem::Shape` capture? | ✅ Sim |
| Emit consome via chain directo? | ❌ Não |
| `FrameItem::Shape` precisa novo field? | ❌ Não — `kind: ShapeKind::Path(Vec<PathItem>)` já existe |
| Hash `export.rs` esperado | **Preservado bit-exact** (10º passo consecutivo) |
| ADR-0098 vigente? | ✅ Sim |

---

## A.1 — Inventário literal do caminho actual

### A.1.1 — `PathItem` enum

4 variants confirmados (A.0.0.1). `Vec<FontFamily>`-style storage
em `ShapeKind::Path(Vec<PathItem>)`.

### A.1.2 — Operações stdlib actualmente expostas

| Função | Construção | `CubicTo` usado? |
|---|---|---|
| `native_polygon` | Sequence de LineTo + ClosePath | ❌ Não |
| `native_rect` | Sem PathItem (usa `Rect` variant directo) | n/a |
| `native_line` | Sem PathItem (usa `Line { dx, dy }` variant) | n/a |
| `native_circle` / `native_ellipse` | Sem PathItem | n/a |
| **`native_curve`** | **Não existe** | n/a |
| **`native_path`** | **Não existe** | n/a |

**Conclusão**: nenhuma stdlib constrói `CubicTo` actualmente.

### A.1.3 — Emit actual em `export.rs`

`emit_shape_path_local` + outros 2 sítios (top-level + clip mask)
todos com match exaustivo sobre `PathItem`:

```rust
match item {
    PathItem::MoveTo(p) => ops.push_str(&format!("{:.2} {:.2} m\n", ...)),
    PathItem::LineTo(p) => ops.push_str(&format!("{:.2} {:.2} l\n", ...)),
    PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
        "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
        cx1, cy1, cx2, cy2, ex, ey,
    )),
    PathItem::ClosePath => ops.push_str("h\n"),
}
```

**Match exhaustive** — `CubicTo` arm já implementado correctamente.

### A.1.4 — Vanilla typst comparação

Vanilla expõe `curve.move/line/quadratic/cubic/close`. Cristalino
pós-P293:
- `curve.move`: → MoveTo (P293 materializar).
- `curve.line`: → LineTo (P293).
- `curve.cubic`: → CubicTo (P293).
- `curve.close`: → ClosePath (P293).
- `curve.quadratic`: ❌ **scope-out P293** (cristalino não tem
  variant `QuadraticTo`; passo dedicado futuro).

### A.1.5 — DEBT-33 P277 detalhe

P277 fechou "Bézier bbox analítica" — `path_bbox` em `geometry.rs:198`
calcula AABB analítica para sequências PathItem com CubicTo. Pure
math f64. **Não toca em stdlib nem emit** — P293 reusa sem alteração.

### A.1.6 — Consumers actuais (paradigma)

| Consumer | Paradigma |
|---|---|
| `path_bbox` (P277) | Função pura `Vec<PathItem>` → `(f64, f64, f64, f64)` |
| Emit PDF (`export.rs`) | Match exhaustive sobre PathItem |
| **`native_curve` (P293)** | **Constructor stdlib novo — caminho de entrada** |

**Paradigma consumer P293**: **6º paradigma arquitecturalmente
distinto** identificado nos últimos 6 passos:
- P288 lang: cross-module.
- P289 weight: TextStyle method.
- P290 tracking: per-glyph + Tc emit.
- P291 leading: per-line peek.
- P292 font: 2 layers (TextStyle + FontBook).
- **P293 curve: PathItem variant inerte + cascade entry via stdlib**.

### A.1.7 — `FrameItem::Shape` emit

Confirmado A.0.1: `FrameItem::Shape { kind: ShapeKind::Path(items), ... }`
captura em `FrameItem`; emit lê via match. ADR-0098 vigente.

### A.1.8 — Diagrama de fluxo

```
#curve(curve.move((0, 0)), curve.cubic(c1, c2, (100, 50)), curve.close())
       │
       ▼
parse → eval_call (native_curve)             [P293 materializar]
       │
       ▼
Construct Vec<PathItem>:
  [MoveTo(0,0), CubicTo(c1, c2, (100, 50)), ClosePath]
       │
       ▼
Content::Shape { kind: ShapeKind::Path(vec), ... }
       │
       ▼
Layouter::layout_content (Shape arm)
       │
       ▼
FrameItem::Shape { kind: ShapeKind::Path(vec), ... } (capture)
       │
       ▼
export.rs match arm CubicTo (linhas 2375/2457/2629)
       │
       ▼
PDF `{cx1} {cy1} {cx2} {cy2} {ex} {ey} c\n`
```

Caminho 100% materializado **excepto** o constructor stdlib (passo
1). P293 adiciona apenas a entrada.

---

## A.2 — Decisão estrutural

### A.2.1 — Decisão

**Decidido**: adicionar **`native_curve` stdlib** que aceita variadic
arguments tipados como variants de PathItem.

**Sintaxe vanilla** (cristalino aproximação):
```typst
#curve(
    curve.move((x, y)),
    curve.line((x, y)),
    curve.cubic((c1x, c1y), (c2x, c2y), (ex, ey)),
    curve.close(),
)
```

**Sintaxe cristalino simplificada** (sem `curve.move`/`curve.cubic`
sub-funções; usa arrays/dicts para distinguir):

```typst
#curve(
    ("move", (0pt, 0pt)),
    ("cubic", (10pt, 0pt), (90pt, 50pt), (100pt, 50pt)),
    ("close",),
)
```

**Justificação simplificação**: cristalino não tem `scope` em
funções (`#elem(scope)` vanilla é proc macro). Replicar `curve.move`
exigiria materializar scope methods — fora do escopo P293.
Aproximação tuple/array é divergência consciente per ADR-0054
graded.

### A.2.2 — Honestidade epistémica

`native_curve` é **constructor stdlib**, não variant. Padrão N=4
"variant rico" **inalterado** pelo P293.

P293 é **6º paradigma consumer distinto** mas **não** é mais um
"Style variant" — é constructor para `Content::Shape` existente.
Reaplicação ADR-0099 com **paradigma novo**: feature graded é
**construção via stdlib** em vez de **cascade entry**.

---

## A.3 — Integração com tipos existentes

`native_curve` produz `Content::Shape { kind: ShapeKind::Path(items), ... }` —
**zero novos variants em `Content` ou `ShapeKind`**. Apenas adiciona
caminho de entrada via stdlib.

Paralelo P283 calc trig (extensão stdlib sem novos Content
variants).

---

## A.4 — Impacto em `FrameItem::Shape` e emit

### A.4.1 — Decisão

**(i)** confirmada empiricamente: emit já existe para CubicTo.
Hash `export.rs 66cb8ac3` preservado bit-exact pelo **10º passo
consecutivo** (P282+P285+P286+P287+P288+P289+P290+P291+P292+P293).

### A.4.2 — Padrões cumulativos

| Padrão | N pós-P293 | Promoção? |
|---|---:|---|
| ADR-0098 | **10** | ❌ Reforço cumulativo (10º passo consecutivo) |
| ADR-0099 | **9** | ❌ Reaplicação cumulativa (4ª pós-formalização) |
| §8.3 "refutação pragmática" | **6 candidato** | ⚖ **A.0.0 H6 inaugural descobre hipótese não listada** — refutação genuína vs spec. Decisão de promoção condicional em §3 ponto 3 |
| §8.4 "bug latente fixed" | 1 estável | ❌ (A.5 abaixo) |
| §8.5 "patch cirúrgico sequencial" | 5 desqualificado | ❌ (P293 ortogonal — não cumulativo) |
| §8.6 "A.5' anti-reflexão" | **3 cumulativo** | ⚖ P293 ortogonal — A.5' tem comparação diferente; ainda válida; tentativo N≥3-4 atingido |

**Decisão**: §8.3 N=6 é candidato genuíno. **Mas**:
- P273.17 §0: uma ADR meta por passo.
- §8.6 N=3 também candidato (próximo de limiar).

Vou **NÃO promover §8.3** porque a refutação H6 inaugurou A.0.0 e
o ganho metodológico já está documentado como template inaugural
no L0. **Anti-padrão over-formalização** vigente — A.0.0 inaugural
**inaugura padrão emergente novo §8.7' (template clarificação de
scope)** que ficará N=1 estável pós-P293.

§8.6 N=3 também aguarda — ortogonalidade do P293 muda o critério
de comparação.

**Resultado P293**: **0 ADRs meta novas** promovidas. Padrão §8.7'
(A.0.0 template) inaugurado N=1.

---

## A.5 — Detecção de bugs latentes

5 cenários Bézier fronteira:

| Cenário | Construção | Resultado esperado |
|---|---|---|
| Control coincidentes | `cubic(p, p, end)` | Path válido; emit produz mesma curva (caso degenerado paramétrico) |
| Curva fechada | `move(p) + cubic(c1, c2, p) + close` | ClosePath emite `h` correctamente |
| Auto-intersecção | `cubic((10, 0), (-10, 0), (10, 0))` | PDF aceita; bbox via P277 calcula extremos |
| Coords muito grandes | `cubic((1e6, 1e6), ..., (1e6, 1e6))` | Sem overflow (P277 algoritmo robusto) |
| Items vazios | `Vec<PathItem>` vazio | `Content::Shape` aceita; emit produz path PDF vazio (no-op) |

### A.5.1 — Nenhum bug latente detectado

P277 P293 trabalho cumulativo robusto. Padrão §8.4 permanece N=1
estável.

---

## A.5' — Verificação anti-reflexão N=3 cumulativo (ortogonal vs cumulativo)

### A.5'.1 — Comparação literal A.1.6 P288-P293

| Passo | Tipo | Paradigma consumer principal |
|---|---|---|
| P288 lang | cumulativo style | Cross-module (eval+lang+layout) |
| P289 weight | cumulativo style | TextStyle method (faux_bold) |
| P290 tracking | cumulativo style | Per-glyph + Tc emit |
| P291 leading | cumulativo style | Per-line peek `current_line` |
| P292 font | cumulativo style | 2 layers (TextStyle + FontBook) |
| **P293 curve** | **ortogonal** (não cumulativo) | **PathItem variant inerte + stdlib constructor** |

**6 paradigmas arquiteturalmente distintos** em 6 passos consecutivos.
P293 é o **1º ortogonal** — paradigma genuinamente novo (variant
em enum diferente; constructor stdlib em vez de cascade arm).

### A.5'.2 — A.0 produzido empiricamente (com A.0.0 inaugural)

P293 inaugurou A.0.0 obrigatória — inspecção literal de
`geometry.rs`+`shapes.rs`+`export.rs`+vanilla. **Antecipação spec
(H1-H5) refutada por descoberta empírica (H6)**.

### A.5'.3 — Elementos estructuralmente novos identificados

✅ **Múltiplos identificados** (P293 N=3 com **3+ elementos** vs
P292 N=2 com 2 elementos):

1. **A.0.0 em si** — secção inaugural; primeiro passo onde scope da
   spec é genuinamente ambíguo. Template para passos futuros.
2. **H6 descoberta** — hipótese não listada na spec; padrão
   "activação posterior" aplicado a `PathItem` (não `Style`).
3. **Ortogonalidade vs cumulativo** — primeiro passo pós-série
   P288-P292; padrão consumer "constructor stdlib" vs "cascade
   arm" da série.
4. **Refutação genuína da spec** (não estructuralmente forçada) —
   refutação significativa de hipóteses spec é distinta de
   refutação por tipo existente (P292 §A.2.3).

### A.5'.4 — Decisão sobre passo seguinte

P293 é ortogonal por construção; **P294 pode ser** outra frente
ortogonal qualquer (math-accent-cancel, footnote-cluster, ou
extensão P293 com `QuadraticTo`).

**A.5' N=3 com 4 elementos novos confirma robustez do padrão §8.6**.
Tentativo N≥3-4 atingido — **mas não promover** em P293 porque:
- §8.7' (A.0.0 template) é mais urgente como padrão emergente.
- P273.17 §0: uma ADR meta por passo no máximo.
- §8.6 promoção fica para passo P294+ se padrão continuar relevante.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P293 |
|---|---:|---:|
| `Content` variants | 64 | 64 (inalterado) |
| `ShapeKind` variants | 5 | 5 (inalterado) |
| `PathItem` variants | 4 | 4 (inalterado — apenas activação) |
| `Style` variants | 10 (pós-P292) | 10 (inalterado — P293 é ortogonal a Style) |
| Stdlib funções shape/path | 5 (polygon/rect/line/circle/ellipse) | **6** (+`curve`) |
| Caminhos entrada para `PathItem::CubicTo` | **0** | **1** (via `native_curve`) |
| Hash L0 `style.md` | actual | inalterado (P293 não toca Style) |
| Hash L0 `content.md` | actual | inalterado |
| Hash L0 `stdlib.md` | actual | **muda** (+1 função `native_curve`) |
| Hash L0 `export.rs` | `66cb8ac3` | **preservado bit-exact** (**10º passo consecutivo**) |
| Padrão §8.6 A.5' N | 2 | **3** (P291+P292+P293) |
| Padrão §8.3 N | 5 estável | 5 estável (refutação significativa mas não promovida em P293) |
| Padrão §8.7' A.0.0 template N | n/a | **1 inaugural** |
| ADRs novas | 0 | 0 |

---

## §Risco residual mitigado

- **Risco principal P293** (scope ambíguo): ✅ A.0.0 clarificou
  empiricamente — H6 NOVA descoberta.
- **Risco secundário** (A.0.0 → H3 redirecção): ✅ Refutado — H6 é
  feature genuinamente curve.
- **Risco terciário** (H2 quebra hash export.rs): ✅ Refutado — H6
  preserva hash (emit já existe).
- **Risco quaternário** (§8.6 N=3 promoção): ✅ Decisão honesta
  documentada em §A.4.2 — não promover.
- **Risco quinário** (dependências novas): ✅ H6 é puro Rust;
  `kurbo` etc. não necessárias.
- **Risco senário** (sequência reflexa): ✅ A.5'.3 identifica
  4 elementos novos — refutado.

---

## §Fecho da Fase A

Inventário literal completo + **A.0.0 inaugural com H6 descoberta
empírica** + decisão estrutural (`native_curve` stdlib) + A.4 emit
preservado + A.5 sem bugs + **A.5' N=3 cumulativo com 4 elementos
estructuralmente novos** registadas. **0 ADRs meta novas promovidas**
— §8.3 N=6 candidato refutado em favor de §8.7' (A.0.0 template
inaugural N=1 emergente).

**MARCO P293**: primeiro passo ortogonal pós-série cumulativa. A.0.0
inaugural estabelece template para casos futuros de scope ambíguo.
Hash `export.rs` preservado pelo 10º passo consecutivo.

Procede-se a §3 da spec.
