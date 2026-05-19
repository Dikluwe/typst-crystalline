# Diagnóstico — Fase A do Passo 296 (`P-math-accent-cancel`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-296.md`
**Origem**: P292 §9.2 rank #1; P294 §10; P295 §9.
**Tipo declarado spec**: frente ortogonal genuinamente nova; magnitude
XS+S.
**Hipótese adoptada**: **HIV** (ambos features ausentes apesar de
"parcial") com **A.2 → (a) minimal** + **A.3 → (α) variants +
Layouter arms**.
**Magnitude da refutação A.0.0 N=4**: **média (factual-significativa)** —
refuta hipótese degenerescência P295 §10 com magnitude maior que P295
mas menor que P294. **§8.7' template valida com refutação real**;
promoção N=4 ainda adiada per P273.17 §0.

---

## A.0.0 — Verificação literal estado "parcial" (N=4 §8.7')

### A.0.0.1 — Inspecção literal do código (zero hits)

```text
grep -rn "MathAccent\|MathCancel" 01_core/src/   → 0 hits
grep -rn "native_accent\|native_cancel" 01_core/src/  → 0 hits
grep -rin "accent\|cancel" 01_core/src/  → 0 hits
```

**Variants Math actuais (10)** em `Content`:
`MathSequence`, `MathIdent`, `MathText`, `MathFrac`, `MathAttach`,
`MathRoot`, `MathDelimited`, `MathAlignPoint`, `MathMatrix`,
`MathCases`.

`MathAccent` e `MathCancel` **confirmados ausentes**.

### A.0.0.2 — Tabela A.4 linhas 118-119 vs realidade

**Tabela A.4 linha 118**: `accent(c, mark)` — `parcial`.
**Tabela A.4 linha 119**: `cancel`, `underover`, `op` — `parcial`.

**Realidade empírica**: ambas as features **completamente ausentes**
em L1. Sem stdlib, sem variant, sem layout handler, sem fallback.

**Classificação correcta pré-P296**: **`ausente`**, não `parcial`.

### A.0.0.3 — Magnitude da refutação (teste empírico P295 §10)

P295 §10 hipotetizou que A.0.0 podia estar a degenerar em ritual
procedimental. Magnitude decrescente:
- P293: alta (hipótese H6 não-listada)
- P294: máxima (spec inteira invalidada por vanilla pattern)
- P295: baixa (linha tabela administrativa desactualizada)

**P296 magnitude**: **média (factual-significativa)**:
- **Não é** apenas "linha desactualizada" — é classificação inteira
  errada.
- **Não é** invalidação de spec — spec **antecipou ambiguidade**
  via HI-HV.
- A refutação confirma que A.0.0 **continua a gerar valor empírico**
  — sem inspecção literal, P296 teria assumido "parcial heurístico"
  e implementado refinos em vez de from-scratch.

**Conclusão**: hipótese degenerescência P295 §10 **refutada**.
§8.7' template **não degenerou**. **Promoção N=4 ainda adiada** por
P273.17 §0 (preferência por consolidação maior).

### A.0.0.4 — Hipótese decidida: HIV + (a) + (α)

- **HIV**: features ausentes apesar de "parcial" tabela.
- **A.2 → (a) minimal**: `MathAccent { base, accent }` +
  `MathCancel { body }`. Sem cosméticos (`size`, `length`,
  `inverted`, `cross`, `angle`, `stroke`) — scope-out per ADR-0054
  graded.
- **A.3 → (α) variants + layout_math arms**: paralelo `MathFrac`
  (P37) + `MathRoot` (P37+).

**Padrão "variant rico" N=4 preservado inalterado** — não qualificar
gratuitamente N=5 (anti-padrão P273.17 §0).

---

## A.0 — Reuso ADR-0098 (preservação esperada)

| Verificação | Esperado pós-P296 |
|---|---|
| Hash `export.rs` | `66cb8ac3` preservado bit-exact (**13º passo consecutivo**) |
| `FrameItem` variant novo? | **Não** — accent emite `FrameItem::Glyph` + posicionamento; cancel emite `FrameItem::Line` |
| Math layout produz consumer agnóstico | ✅ paradigma P136 vigente |
| ADR-0098 N=13 cumulativo | ✅ |

---

## A.1 — Inventário literal

### A.1.1 — Variants `Math*` actuais (10)

Vide A.0.0.1. Pós-P296: **12 variants** (+`MathAccent` +`MathCancel`).

### A.1.2 — Vanilla `AccentElem` + `CancelElem`

`lab/.../math/accent.rs`:
```rust
pub struct AccentElem {
    #[required] pub base: Content,
    #[required] pub accent: Accent,
    #[default(Rel::one())] pub size: Rel<Length>,
    #[default(false)] pub dotless: Smart<bool>,
}
```

`lab/.../math/cancel.rs`:
```rust
pub struct CancelElem {
    #[required] pub body: Content,
    #[default(Rel::new(Ratio::one(), Em::new(0.3).into()))]
    pub length: Rel<Length>,
    #[default(false)] pub inverted: bool,
    #[default(false)] pub cross: bool,
    pub angle: Smart<CancelAngle>,
    pub stroke: Stroke,
}
```

Cristalino P296 simplifications per ADR-0054 graded:
- `AccentElem.size` + `dotless` → scope-out.
- `CancelElem.length` + `inverted` + `cross` + `angle` + `stroke` →
  scope-out.

### A.1.3 — Match arms exhaustive

Para `MathAccent`/`MathCancel` defesa compilador exige arms em:

| Local | Operação |
|---|---|
| `content.rs:is_empty()` | body/base empty |
| `content.rs:plain_text()` | concatenar base + accent / body |
| `content.rs:PartialEq` | structural |
| `content.rs:map_content()` | recurse |
| `content.rs:map_text()` | recurse |
| `rules/introspect.rs:materialize_time` | terminal (paralelo MathFrac) |
| `rules/introspect.rs:walk` | walk em children (paralelo MathFrac) |
| `rules/introspect/locatable.rs` | `false` (paralelo MathFrac) |
| `rules/layout/mod.rs` | fallthrough math (paralelo MathFrac) |
| `rules/math/layout/mod.rs:layout_node` | handler dedicado novo |

Defesa cumulativa via compiler errors identifica todos.

### A.1.4 — Stdlib actual

`native_accent` + `native_cancel` **ausentes**. Vou adicionar em
`01_core/src/rules/stdlib/structural.rs` (paralelo
`native_quote`/`native_cite`).

### A.1.5 — Layouter consumer

`rules/math/layout/mod.rs:255 layout_node` despacha math variants.
**Arms a adicionar**: `MathAccent` + `MathCancel`.

### A.1.6 — Emit (ADR-0098 invariante)

`export.rs` é agnóstico — math layout produz `FrameItem::Text/Glyph/Line`
standard. **Hash preservado**.

### A.1.7 — Diagrama de fluxo P296

```
#accent(a, hat) / #cancel(x)
       │
       ▼
parse → eval_call (native_accent / native_cancel)
       │
       ▼
Content::MathAccent { base, accent } / Content::MathCancel { body }
       │
       ▼
Math Layouter::layout_node (arm novo P296)
       ├── MathAccent: layout_node(base) + posicionar(accent) above
       └── MathCancel: layout_node(body) + FrameItem::Line diagonal
       │
       ▼
MathBox { items: Vec<FrameItem> }
       │
       ▼
FrameItem standard (Text/Glyph/Line)
       │
       ▼
export.rs emit standard — INALTERADO
```

### A.1.8 — Paradigma consumer P296

**9.º paradigma** consecutivo:
- P288-P292: cumulativo style.
- P293 curve: variant inerte activação.
- P294 quadratic: transform-on-build.
- P295 footnote: walker counter.
- **P296 math accent/cancel: math layout handler dedicado**.

Genuinamente novo (math layout não tocado em P288-P295).

---

## A.2 — Estrutura variants (decisão (a) minimal)

**Decisão A.2 → (a)**:

```rust
MathAccent {
    base:   Box<Content>,
    accent: Box<Content>,
},
MathCancel {
    body: Box<Content>,
},
```

**Razões**:
- Padrão "variant rico" N=4 **preservado** (anti-padrão N=5 gratuito).
- Cosméticos vanilla (size/length/inverted/cross/angle/stroke)
  scope-out per ADR-0054 graded.
- `inverted: bool` / `cross: bool` são genuíno toggle (não cosmético
  puro) — mas materialização futura via P296.1 dedicado.

---

## A.3 — Integração com math layout (decisão (α))

**Decisão A.3 → (α)** variants novos + handlers `layout_node` arm.

**Pseudo-code accent**:

```rust
Content::MathAccent { base, accent } => {
    let base_box = self.layout_node(base, style);
    let accent_box = self.layout_node(accent, style);
    // Posicionar accent centrado horizontalmente acima da base.
    let dx = (base_box.width - accent_box.width) / 2.0;
    let dy = -base_box.ascent;  // acima do topo
    // Compor verticalmente via MathBox merge.
    ...
}
```

**Pseudo-code cancel**:

```rust
Content::MathCancel { body } => {
    let body_box = self.layout_node(body, style);
    let line = FrameItem::Line {
        start: Point { x: Pt(0.0),                    y: Pt(body_box.height()) },
        end:   Point { x: Pt(body_box.width),         y: Pt(0.0) },
        thickness: 0.5,
        color: None,
    };
    // Anexar linha a body_box.items.
    ...
}
```

---

## A.4 — Impacto em emit (ADR-0098 N=13 cumulativo)

**Decisão A.4 → (i)** — math layout produz `FrameItem` standard
(Text/Glyph/Line). Hash `export.rs` preservado bit-exact pelo
**13º passo consecutivo**.

---

## A.5 — Detecção de bugs latentes

6 cenários fronteira:

| Cenário | Resultado esperado |
|---|---|
| `accent(a, hat)` simples | base `a` + glyph `^` acima |
| `accent("ab", hat)` base multi-letter | acento sobre 1.ª letra apenas (heurístico minimal) ou centrado (vanilla) |
| `cancel(x)` body simples | diagonal sobre `x` |
| `cancel("")` body vazio | linha de tamanho zero (no-op observable) |
| `cancel(frac(1,2))` body composto | linha sobre bbox completo |
| accent + cancel combinados (`accent(cancel(x), hat)`) | nesting estructural correcto |

### A.5.1 — Sem bugs latentes esperados

Padrão §8.4 N=1 estável.

---

## A.5' — Anti-reflexão N=5 cumulativo (teste empírico §8.7')

### A.5'.1 — Comparação A.1.8 P288-P296

| Passo | Tipo | Paradigma consumer |
|---|---|---|
| P288-P292 | cumulativo | 5 paradigmas style/text |
| P293 cubic | ortogonal | Variant inerte activação |
| P294 quadratic | ortogonal | Transform-on-build |
| P295 footnote | ortogonal M | Walker counter |
| **P296 math accent/cancel** | **ortogonal XS+S** | **Math layout handler dedicado** |

**9 paradigmas arquiteturalmente distintos** em 9 passos. P296 é
o **1.º paradigma math layout** pós-série.

### A.5'.2 — A.0.0 N=4 reaplicação — magnitude média

P296 refuta hipótese degenerescência §6.6 P295. Magnitude:
- **Não factual-modesta** (vs P295: tabela só desactualizada).
- **Não significativa máxima** (vs P294: spec inteira invalidada).
- **Média**: classificação Tabela A.4 inválida; features
  completamente ausentes apesar de "parcial".

§8.7' template **valida com refutação real genuína**.

### A.5'.3 — Elementos estructuralmente novos identificados

5 elementos:

1. **Math layout handler dedicado** — paradigma novo (P288-P295
   não tocaram em math layout).
2. **Refutação classificação inteira** — Tabela A.4 inválida (não
   apenas linha imprecisa P295).
3. **Hipótese principal não antecipada na spec** — primeira spec
   onde A.0.0 abre HI-HV plausíveis sem preferência. Decidido HIV
   após inspecção.
4. **Teste empírico hipótese degenerescência §6.6 P295** — refutada
   por P296.
5. **Math é 9.º paradigma consecutivo distinto** — sequência
   P288-P296 robusta.

### A.5'.4 — Decisão sobre promoção ADR meta

Candidatos disparados:
- **§8.7' N=4** — reaplicação genuína; template valida. Limiar
  N≥3 ultrapassado; **adiado** porque P273.17 §0 (uma ADR meta por
  passo) + preferência por consolidação mais ampla.
- **§8.3 N=8** — refutação pragmática genuína. **Adiado** mesma
  razão.
- **§8.6 N=6** — anti-reflexão cumulativa. **Não promovido** per
  anti-padrão.

**Decisão**: **0 ADRs meta novas**. §8.7' N=4 + §8.3 N=8 candidatos
genuínos preservados para P297+ se reaplicação for ainda mais
robusta.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P296 |
|---|---:|---:|
| `Content` variants | 65 | **67** (+2 MathAccent, MathCancel) |
| Stdlib funções math | N | N+2 (+`native_accent`, `native_cancel`) |
| Hash L0 `content.md` | actual | **muda** (+2 variants) |
| Hash L0 `stdlib.md` | actual | inalterado (política única) |
| Hash L0 `export.md` | `31a37c57` | inalterado |
| Hash `export.rs` | `66cb8ac3` | **preservado bit-exact** (**13º passo consecutivo**) |
| Padrão §8.6 A.5' N | 5 | **6** (P291-P296) |
| Padrão §8.7' A.0.0 N | 3 | **4** (reaplica) |
| Padrão §8.3 N candidato | 7 | **8** candidato adiado |
| Padrão "variant rico" N | 4 | 4 (inalterado — A.2 → (a)) |
| ADRs novas | 0 | 0 |

---

## §Risco residual mitigado

- **Risco principal** (degenerescência §6.6 P295): ✅ **refutado**
  por A.0.0 N=4 com refutação média (não factual-modesta).
- **Risco secundário** (HIV trabalho maior): ⚖ controlado via
  scope-out cosméticos.
- **Risco terciário** ("variant rico" N=5 disparo): ✅ refutado A.2
  → (a) minimal.
- **Risco quaternário** (emit muda inesperadamente): ✅ refutado A.0
  + A.1.6.
- **Risco quinário** (regressão math pre-P296): ⚖ testes regression
  via workspace.
- **Risco senário** (sequência reflexa desformalizada): ✅ decisão
  consciente preserva §8.7' — magnitude média valida template.

---

## §Fecho da Fase A

Inventário literal completo + **A.0.0 N=4 reaplica §8.7' com
refutação média (factual-significativa)** + decisão **HIV + (a)
minimal + (α) variants + layout_math arms** + A.4 hash preservado
+ A.5 sem bugs + **A.5' N=5 cumulativo com 5 elementos novos**.
**0 ADRs meta novas** — §8.7' N=4 + §8.3 N=8 candidatos genuínos
adiados.

**MARCO P296**:
- **3.º passo ortogonal pós-série cumulativa P288-P292** —
  paradigma math layout (9.º distinto).
- **A.0.0 N=4 com magnitude média** — refuta hipótese
  degenerescência §6.6 P295. Template §8.7' valida com refutação
  real.
- **Classificação Tabela A.4 corrigida** — `parcial` (errado) →
  `ausente` pré-P296 → `implementado` pós-P296.
- **Hash `export.rs` preservado pelo 13º passo consecutivo** —
  ADR-0098 robusta sobre 13 features distintas.

Procede-se a §3 da spec (com plano HIV + (a) + (α)).
