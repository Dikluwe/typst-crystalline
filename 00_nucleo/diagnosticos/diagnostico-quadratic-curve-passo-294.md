# Diagnóstico — Fase A do Passo 294 (`P-quadratic-curve`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-294.md`
**Origem**: P293 §9 frente pendente; P293 §10 candidato rank 1.
**Tipo declarado spec**: extensão directa P293 mas estructuralmente
diferente; materialização from-scratch (variant novo + emit novo).
**Tipo após A.0.0**: **refutação significativa da spec** — vanilla
typst **NÃO** tem variant `QuadraticTo` interno; converte q→c em
construct-time. **A.0.0 N=2 reaplica §8.7' com refutação genuína**.

---

## A.0.0 — Clarificação de scope (N=2 reaplicação §8.7')

### A.0.0.1 — Verificação literal do estado actual

#### Inspecção 1: `01_core/src/entities/geometry.rs:11-21`

`PathItem` enum (4 variants):

```rust
pub enum PathItem {
    MoveTo(Point),
    LineTo(Point),
    CubicTo(Point, Point, Point),  // c1, c2, end (tuple posicional)
    ClosePath,
}
```

`QuadraticTo` **confirmado ausente** (spec §A.0.0 expected).

#### Inspecção 2: emit cubic em `export.rs`

Match arm cubic em 3 sítios (paradigma P136 + ADR-0098):
- Linha 2375 — top-level `build_page_stream`.
- Linha 2457 — local `emit_shape_path_local`.
- Linha 2629 — clip path.

Todos emitem `{cx1} {cy1} {cx2} {cy2} {ex} {ey} c\n`. **Operator `v`
e `y` confirmados ausentes**.

#### Inspecção 3: `path_bbox` (P277) — `bezier_cubic_bbox`

`geometry.rs:151-186` — algoritmo análitico cubic via raízes de
B'(t)=0 (P277). **Não suporta** quadratic; algoritmo similar mais
simples teoricamente disponível.

#### Inspecção 4 (CRÍTICA): vanilla typst `lab/.../shapes.rs:215-220`

```rust
/// Add a quadratic curve segment.
fn quad(&mut self, control: Point, end: Point) {
    let c1 = control_q2c(self.last_point, control);
    let c2 = control_q2c(end, control);
    self.cubic(c1, c2, end);
}

/// Convert a quadratic control point into a cubic one.
fn control_q2c(p: Point, c: Point) -> Point {
    (p + 2.0 * c) / 3.0
}
```

**Vanilla typst converte quadratic → cubic em construct-time** via
fórmula matemática exacta:
- C₁ = (P₀ + 2·Q) / 3
- C₂ = (P₂ + 2·Q) / 3

**onde Q = control point quadrático único; P₀ = last_point; P₂ =
end**. A curva resultante é **matemáticamente idêntica** à Bézier
quadrática original.

Vanilla `Curve` (storage interno) **não tem variant `QuadraticTo`**.
PDF emit do vanilla também não usa operator `v` ou `y` — tudo é
emitido como `c` operator (cubic) porque o storage já é cubic.

### A.0.0.2 — Refutação significativa da spec P294

Spec P294 §1.1 + §1.2 + §2 §A.0/A.2/A.3/A.4 todas assumem:
- (i) `PathItem::QuadraticTo(control, end)` variant adicionado ao enum.
- (ii) Match arms estendidos em 4 sítios (path_bbox + 3 emit).
- (iii) Algoritmo `bezier_quadratic_bbox` análitico paralelo P277.
- (iv) Operator PDF `v` ou `y` emit.
- (v) Hash `export.rs` muda intencionalmente (1ª quebra desde P281).

**Refutação empírica via inspecção 4**:
- Vanilla **não materializa variant interno** — converte na
  construção.
- Vanilla **não emite operator `v`/`y`** — usa `c` operator.
- `path_bbox` (P277) **reusa-se sem alteração** — depois da
  conversão, é cubic ordinária.
- Hash `export.rs` **preservado bit-exact** — sem novo arm.

### A.0.0.3 — Hipótese H1 nova (não listada na spec)

| Hipótese | Veredicto |
|---|---|
| Spec §1.1 H1+H2 (materialização from-scratch) | ❌ Refutado por vanilla pattern |
| **H1' (NOVA) — Conversão q→c em construct-time, sem novo variant** | ✅ **Adoptado** — paridade vanilla bit-exact |

**A spec foi escrita assumindo divergência arquitectural de
vanilla**. A inspecção literal revela que vanilla **já resolveu**
o problema com elegância — converter q→c preserva geometria e
permite código unificado.

### A.0.0.4 — Decisão sobre continuação P294

**Cenário H1'** (refutação significativa de spec via empirical
A.0.0 N=2):

1. **NÃO adicionar variant `QuadraticTo`** ao enum `PathItem`.
   Preservar 4 variants.
2. **NÃO estender match arms** em `path_bbox` ou em emit (3 sítios).
   Hash `export.rs 66cb8ac3` **preservado bit-exact** (11º passo
   consecutivo).
3. **Modificar apenas `native_curve`** (P293 §3.1) — remover
   scope-out `Err` em `"quadratic"` kind e **substituir** por:
   - Leitura de control + end coords.
   - Aplicação fórmula `control_q2c` localmente em
     `native_curve` para calcular C₁ e C₂.
   - Push `PathItem::CubicTo(C₁, C₂, end)`.
4. **Manter `last_point` tracking** dentro de `native_curve` para
   conhecer P₀ na fórmula. Walker simples: percorrer `path_items`
   construídos até agora, último Point construído = `last_point`.
5. Testes: ~7-9 testes que verificam conversão correcta + emit cubic.

### A.0.0.5 — Honestidade epistémica registada

P294 spec **partiu de assumption falsa** (vanilla teria variant
quadratic interno). A.0.0 N=2 reaplicação descobriu vanilla pattern
via inspecção `lab/`.

**Refutação significativa genuína** (vs P293 §A.2.3 onde refutação
foi estructuralmente forçada). **§8.3 N=6 cumulativo CONFIRMADO**:
- P293 §7.3 refutou §8.3 candidato N=6 em favor de §8.7' inaugural.
- **P294 promove §8.3 N=6** porque a refutação aqui é
  **mais significativa** que a de P293 (refuta toda a estrutura
  proposta, não apenas hipótese H1-H5).

**Mas P273.17 §0 ainda vigente**: uma ADR meta por passo.
Decisão de promoção condicional em §A.4.2.

---

## A.0 — Hash `export.rs` (preservação literal, não alteração)

**Spec esperava**: hash muda intencionalmente; **1ª quebra desde
P281**.

**Realidade pós-A.0.0**: hash **preservado bit-exact** —
**11º passo consecutivo**. Spec invalidada por A.0.0.

| Verificação | Esperado pós-P294 |
|---|---|
| Hash `export.rs` | `66cb8ac3` preservado |
| Ramos cubic/line/move/close | inalterados |
| Novo operator PDF | **nenhum** — q→c conversão emite `c` existente |
| ADR-0098 aplicável | ✅ vigente (N=11 cumulativo) |

---

## A.1 — Inventário literal do caminho `PathItem`

### A.1.1 — Variants actuais

4 variants confirmados: `MoveTo`, `LineTo`, `CubicTo`, `ClosePath`.
**Inalterados pós-P294**.

### A.1.2 — Estrutura `PathItem::CubicTo(Point, Point, Point)`

Tuple variant posicional (control1, control2, end).

### A.1.3 — Match arms exhaustive sobre `PathItem`

Identificados via `grep`:

1. `01_core/src/entities/geometry.rs:214-230` — `path_bbox` walker.
2. `03_infra/src/export.rs:2365-2381` — emit top-level.
3. `03_infra/src/export.rs:2451-2461` — emit local Group.
4. `03_infra/src/export.rs:2621-2635` — emit clip path.

**Pós-P294**: **inalterados** — H1' não toca em nenhum.

### A.1.4 — Constructors actuais

- `native_polygon`: MoveTo + N×LineTo + ClosePath.
- `native_curve` (P293): move/line/cubic/close + scope-out `"quadratic"` (Err).

**Pós-P294**: `native_curve` ganha arm `"quadratic"` que **converte
para `CubicTo`** internamente.

### A.1.5 — `path_bbox` (P277)

`bezier_cubic_bbox` análitico via raízes derivada cubic.

**Pós-P294**: **inalterado** — quadratic é convertida para cubic
antes de chegar a `path_bbox`.

### A.1.6 — Emit consumer (`export.rs`)

Operator `c` em 3 sítios. **Inalterado pós-P294**.

### A.1.7 — Vanilla typst comparação

`curve.quadratic(control, end)` element em vanilla expressa-se via
struct `CurveQuad { control: Smart<Option<...>>, end: Axes<Rel<Length>>, relative: bool }`.

Internamente em `layout-typst-layout/shapes.rs:216-220`:
```rust
fn quad(&mut self, control: Point, end: Point) {
    let c1 = control_q2c(self.last_point, control);
    let c2 = control_q2c(end, control);
    self.cubic(c1, c2, end);
}
```

**Storage interno final**: cubic only. Vanilla **não armazena**
representação quadrática; é descartada após conversão.

### A.1.8 — Paradigma consumer P294

**7º paradigma** identificado nos últimos 7 passos:
- P288 lang: cross-module.
- P289 weight: TextStyle method.
- P290 tracking: per-glyph + Tc emit.
- P291 leading: per-line peek.
- P292 font: 2 layers (TextStyle + FontBook).
- P293 curve: activação posterior variant inerte.
- **P294 quadratic: conversão q→c em construct-time, zero novo
  variant, hash preservado**.

### A.1.9 — Diagrama de fluxo P294

```
#curve(("move", (0, 0)), ("quadratic", (5, 10), (10, 0)))
       │
       ▼
parse → eval_call (native_curve)         [P294 ALTERAR APENAS AQUI]
       │
       ▼
Walker: kind = "quadratic"
   ├── last_point ← último MoveTo ou end de segmento anterior
   ├── extract control (Q) e end (P₂) das coords
   ├── C₁ = (last_point + 2·Q) / 3        [control_q2c P294]
   ├── C₂ = (P₂ + 2·Q) / 3                [control_q2c P294]
   └── path_items.push(PathItem::CubicTo(C₁, C₂, P₂))
       │
       ▼
Content::Shape { kind: ShapeKind::Path(vec![..., CubicTo, ...]) }
       │
       ▼
Layouter / path_bbox (P277, bezier_cubic_bbox) — INALTERADO
       │
       ▼
FrameItem::Shape — INALTERADO
       │
       ▼
export.rs emit `c` operator — INALTERADO bit-exact
       │
       ▼
PDF `{c1x} {c1y} {c2x} {c2y} {ex} {ey} c\n`
```

**Apenas** `native_curve` muda. Resto do sistema **bit-exact**.

---

## A.2 — Decisão estrutural

### A.2.1 — Decisão H1'

**NÃO** adicionar variant `QuadraticTo`. Preservar 4 variants do
`PathItem`. **Conversão q→c em construct-time** dentro de
`native_curve`.

### A.2.2 — Sintaxe cristalino aproximada

```typst
#curve(
    ("move", (0pt, 0pt)),
    ("quadratic", (5pt, 10pt), (10pt, 0pt)),     // (control, end)
    ("close",),
)
```

Argumentos:
- arr[0] = `"quadratic"` (kind string).
- arr[1] = control point Q (array [cx, cy] ou [cx, cy] em pt).
- arr[2] = end point P₂ (array [ex, ey] ou [ex, ey] em pt).

### A.2.3 — Honestidade epistémica vs spec

Spec §A.2 propôs 3 opções (a/b/c) **todas** assumindo variant novo.
A.0.0 inspecção empírica **refuta a premissa comum**: vanilla
**não tem variant**. **(a)/(b)/(c) todas inadequadas** vs
**H1' (β-style sem variant)**.

### A.2.4 — Tracking de `last_point` em `native_curve`

Cristalino precisa calcular `last_point` antes da fórmula `q2c`.

**Solução simples**: durante walker over args, manter `last_point:
Option<Point>` actualizado ao processar cada segmento. Para `"move"`
e `"line"`: `last_point = Some(target)`. Para `"cubic"`: `last_point
= Some(end)`. Para `"close"`: `last_point` preserved (close não
move). Para `"quadratic"`: usar `last_point` actual para calcular C₁,
C₂; depois `last_point = Some(end)`.

**Caso fronteira**: `"quadratic"` antes de qualquer `"move"` (sem
last_point). Solução: usar `Point::ZERO` como fallback (paridade
implícita vanilla — `last_point` arranca em (0,0) antes do primeiro
component).

---

## A.3 — Algoritmo de bbox para quadratic

### A.3.1 — Decisão (β) — sem extensão de path_bbox

`path_bbox` (P277) **reusa-se sem alteração**. Quadratic é
convertida para CubicTo equivalente; bbox cubic via
`bezier_cubic_bbox` **calcula bbox correcto** porque a curva é
matemáticamente idêntica.

### A.3.2 — Bbox correctness preservada

Vanilla `point_to_kurbo + CubicBez::new + bounding_box` em
`shapes.rs:230-236` confirma: após conversão q→c, bbox cubic é
suficiente. **Não há erro numérico** — fórmula `q2c` é exacta
em `f64`.

### A.3.3 — Algoritmo análitico quadratic-only (não usado)

Para registo histórico (caso H1' fosse refutado e variant
materializado em passo futuro):

```
B(t) = (1-t)² P₀ + 2(1-t)t P₁ + t² P₂
B'(t) = 2(P₁-P₀) + 2t(P₀ - 2P₁ + P₂)
B'(t)=0 ⟹ t* = (P₀ - P₁) / (P₀ - 2P₁ + P₂)
Candidatos: {P₀, P₂, B(t*) se t* ∈ (0,1)}
```

**Não materializado** em P294 — pendente.

---

## A.4 — Impacto em emit e ADR-0098

### A.4.1 — Decisão (ii) — emit inalterado

Hash `export.rs 66cb8ac3` **preservado bit-exact** (11º passo
consecutivo). ADR-0098 §"single source of truth" honrada.

### A.4.2 — Padrões cumulativos pós-P294

| Padrão | N pós-P294 | Promoção? |
|---|---:|---|
| ADR-0098 | **11** | ❌ Reforço cumulativo (11º passo) |
| ADR-0099 | **10** | ❌ Reaplicação cumulativa (5ª pós-formalização); P294 caso paralelo (activação posterior do scope-out P293) |
| §8.3 "refutação pragmática" | **6** | ⚖ **A.0.0 N=2 refuta toda a estrutura da spec** — refutação genuína mais significativa que P293; **candidato genuíno** |
| §8.7' "A.0.0 template" | **2** | ⚖ Reaplicação N=2 com refutação maior; aproxima limiar N≥3 |
| §8.6 "A.5' anti-reflexão" | **4** | ⚖ Cumulativo P291+P292+P293+P294; aproxima limiar mas decisão de não promover preserva anti-padrão |

**Decisão**: 0 ADRs meta promovidas em P294. Razões:
- §8.3 candidato N=6 — **promoção adiada** porque P273.17 §0 limita
  a uma meta por passo; se promover §8.3 hoje, perde-se chance de
  consolidar §8.7' em P295+.
- §8.7' N=2 — limiar N≥3 não atingido; aguarda mais aplicações.
- §8.6 N=4 — limiar N≥3-4 atingido mas decisão honesta documenta
  que padrão é metodológico interno (não ADR-promovível).

**Resultado P294**: **0 ADRs meta novas**. Spec invalidada por A.0.0
N=2 (refutação significativa). Padrão §8.3 N=6 sólido.

---

## A.5 — Detecção de bugs latentes

5 cenários quadratic fronteira:

| Cenário | Construção | Comportamento esperado |
|---|---|---|
| Control coincidente com start | `move(p) + quadratic(p, end)` | C₁=p, C₂=(end+2p)/3 — curva degenerada para linha |
| Control coincidente com end | `move(p) + quadratic(end, end)` | C₁=(p+2·end)/3, C₂=end — curva degenerada |
| Control colinear | `move((0,0)) + quadratic((5,0), (10,0))` | C₁=(10/3, 0), C₂=(20/3, 0) — linha recta |
| Quadratic sem move anterior | `quadratic(c, end)` only | last_point fallback (0,0); curva válida |
| Quadratic encadeada | `move + quadratic + quadratic + close` | last_point actualiza correctamente |

### A.5.1 — Nenhum bug latente detectado

Fórmula `q2c` é exacta. P277 `bezier_cubic_bbox` robust. Padrão
§8.4 N=1 estável.

---

## A.5' — Verificação anti-reflexão N=4 cumulativo

### A.5'.1 — Comparação literal A.1.8 P288-P294

| Passo | Tipo | Paradigma consumer principal |
|---|---|---|
| P288 lang | cumulativo style | Cross-module |
| P289 weight | cumulativo style | TextStyle method |
| P290 tracking | cumulativo style | Per-glyph + Tc emit |
| P291 leading | cumulativo style | Per-line peek |
| P292 font | cumulativo style | FontBook indirect resolution |
| P293 curve | ortogonal | PathItem variant inerte activação |
| **P294 quadratic** | **ortogonal extension** | **Conversão q→c construct-time; zero novo variant; emit preservado** |

**7 paradigmas arquiteturalmente distintos** em 7 passos. P294
distingue-se de P293: P293 activou variant existente; **P294 evita
criação de variant** que a spec assumia necessário.

### A.5'.2 — A.0.0 N=2 reaplicação com refutação maior

P293 inaugurou A.0.0 com descoberta H6 (variant existente mas
inerte). **P294 reaplica A.0.0 com refutação mais significativa**:
não apenas hipótese não-listada, mas **toda a estrutura proposta
pela spec invalidada** por inspecção vanilla.

### A.5'.3 — Elementos estructuralmente novos identificados

✅ **5 elementos novos** (vs P293 N=3 com 4 elementos):

1. **Refutação significativa da spec inteira** — vanilla pattern
   refuta §1.1 + §1.2 + §A.2 + §A.3 + §A.4 simultaneamente.
2. **Hash `export.rs` preservação inesperada** — spec esperava 1ª
   quebra desde P281; A.0.0 descobre que preservação é **mais
   correcta arquitecturalmente** (paridade vanilla).
3. **Conversão matemática em construct-time** — paradigma novo
   "transform-on-build" vs "store-and-emit" anterior.
4. **§8.3 N=6 confirmado genuinamente** — refutação aqui é mais
   significativa que a P293 que refutou §8.3 candidato anterior.
5. **`last_point` tracking dentro de stdlib** — primeira vez que
   `native_curve` precisa estado walker; cristalino sem
   `last_point` field até P294.

### A.5'.4 — Decisão sobre passo seguinte

P295 fica aberto. P294 fecha frente quadratic via conversão q→c.

**Não-promoção §8.3 N=6**: candidato genuíno mas P273.17 §0 vigente.
Próximo passo onde §8.3 reaplicar com refutação significativa →
promoção N=7+.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P294 |
|---|---:|---:|
| `PathItem` variants | 4 | 4 (inalterado — H1' preserva) |
| `Content` variants | 64 | 64 (inalterado) |
| `ShapeKind` variants | 5 | 5 (inalterado) |
| Stdlib funções shape/path | 6 (polygon/rect/line/circle/ellipse/curve) | 6 (inalterado; apenas activação do scope-out `"quadratic"`) |
| Caminhos entrada para `PathItem::CubicTo` | 1 (via `cubic` kind P293) | **2** (+`quadratic` kind via conversão q→c) |
| Hash L0 `geometry.md` | actual | inalterado (PathItem inalterado) |
| Hash L0 `stdlib.md` | actual | inalterado (política única) |
| Hash L0 `export.md` | actual | inalterado |
| Hash L0 `export.rs` | `66cb8ac3` | **preservado bit-exact** (**11º passo consecutivo**) |
| Padrão §8.3 N | 5 estável (P293 não-promoveu N=6) | **6 genuíno mas adiado** (P273.17 §0) |
| Padrão §8.6 A.5' N | 3 | **4** (P291+P292+P293+P294) |
| Padrão §8.7' A.0.0 N | 1 inaugural | **2 reaplica** |
| ADRs novas | 0 | 0 |

---

## §Risco residual mitigado

- **Risco principal P294 (spec §7 #1)** — quebra hash export.rs: ✅
  **refutado** — H1' preserva hash bit-exact.
- **Risco secundário spec §7 #2** — divergência numérica q→c: ✅
  **refutado** — fórmula exacta em `f64`.
- **Risco terciário spec §7 #3** — `path_bbox` não generaliza: ✅
  **refutado** — `path_bbox` reusado intacto.
- **Risco quaternário spec §7 #4** — ADR-0099 dupla natureza: ⚖
  Documentado A.4.2; P294 é activação posterior do scope-out P293
  (não materialização nova).
- **Risco quinário spec §7 #5** — gatilhos meta múltiplos: ✅
  Decisão honesta documentada A.4.2 — adiar todas; §8.3 N=6 candidato
  genuíno preservado para P295+.
- **Risco senário spec §7 #6** — variant ou emit já existem: ✅
  Refutado por inspecção; mas descoberta empírica vanilla **invalida
  toda a abordagem da spec**.

---

## §Fecho da Fase A

Inventário literal completo + **A.0.0 N=2 reaplicação §8.7' com
refutação significativa de spec inteira** + decisão H1' (sem variant;
conversão q→c) + A.4 hash preservado + A.5 sem bugs + **A.5' N=4
cumulativo com 5 elementos estructuralmente novos**. **0 ADRs meta
novas promovidas** — §8.3 N=6 candidato genuíno adiado per P273.17
§0.

**MARCO P294**:
- **2º passo ortogonal pós-série cumulativa** (P293+P294).
- **A.0.0 N=2 com refutação significativa de toda a estrutura
  proposta** — mais forte que P293 H6 (não-listada vs invalidada).
- **Hash `export.rs` preservado pelo 11º passo consecutivo** —
  consequência da descoberta empírica, não objectivo declarado.
- **Spec P294 invalidada arquitecturalmente** — relatório registar
  desvio honesto + razões.

Procede-se a §3 da spec (com plano H1' alterado).
