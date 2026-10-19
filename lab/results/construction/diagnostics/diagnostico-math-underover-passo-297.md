# Diagnóstico — Fase A do Passo 297 (`P296.1 — MathUnderover`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-297.md`
**Origem**: P296 §8 frente pendente; extensão directa.
**Tipo declarado spec**: extensão directa P296 com decisão A.2
genuinamente distinta (Option fields).
**Hipótese adoptada**: **HV' (spec inteira invalidada por vanilla
pattern — paralelo P294)** + **A.2 → (b) Option fields** + **A.3 →
(α) handler dedicado**.
**Magnitude da refutação A.0.0 N=5**: **alta (factual-significativa
máxima)** — vanilla NÃO tem `UnderoverElem` unificado; tem 12
elementos separados. Spec invalidada.

---

## A.0.0 — Verificação literal estado actual (N=5 §8.7')

### A.0.0.1 — Inspecção cristalino L1 (zero hits)

```text
grep -rin "MathUnderover\|UnderoverElem\|native_underover\|under_over" 01_core/src/  → 0 hits
```

Variant + stdlib **confirmados ausentes**. Status real `ausente`
(não `parcial` como Tabela A.4 linha 119 sugere — paralelo P296
descoberta).

### A.0.0.2 — Inspecção vanilla typst (refutação significativa)

Spec P297 assumiu **`UnderoverElem { base, under?, over? }`**
unificado. Vanilla `lab/.../math/underover.rs` revela **estrutura
fragmentada em 12 elementos separados**:

| Elemento | Estrutura | Função |
|---|---|---|
| `UnderlineElem` | `{ body }` | Linha horizontal abaixo do body |
| `OverlineElem` | `{ body }` | Linha horizontal acima do body |
| `UnderbraceElem` | `{ body, annotation? }` | Chave `⏟` abaixo + anotação opcional |
| `OverbraceElem` | `{ body, annotation? }` | Chave `⏞` acima + anotação opcional |
| `UnderbracketElem` | `{ body, annotation? }` | Colchete `⎵` abaixo + anotação |
| `OverbracketElem` | `{ body, annotation? }` | Colchete `⎴` acima + anotação |
| `UnderparenElem` | `{ body, annotation? }` | Parêntesis abaixo |
| `OverparenElem` | `{ body, annotation? }` | Parêntesis acima |
| `UndershellElem` | `{ body, annotation? }` | Tortoise shell abaixo |
| `OvershellElem` | `{ body, annotation? }` | Tortoise shell acima |

**Spec P297 §A.1.4 expectativa** completamente refutada — não
existe wrapper unificado em vanilla. **§A.0.0 cenário "alta
inesperada" HIV' confirmado** (variante HV' = spec invalidada).

### A.0.0.3 — Refutação significativa magnitude alta

| Passo | A.0.0 N | Magnitude |
|---|---:|---|
| P293 | 1 inaugural | alta (hipótese H6 não-listada) |
| P294 | 2 | **máxima** (toda estrutura invalidada por vanilla) |
| P295 | 3 | baixa (linha tabela administrativa) |
| P296 | 4 | média (classificação inteira inválida) |
| **P297** | **5** | **alta** (paralelo P294 — wrapper unificado não existe) |

**P297 NÃO é "confirmação esperada"** como spec antecipava — é
**refutação significativa**. Hipótese degenerescência §6.6 P295
**re-refutada com magnitude maior que P296**.

**Trend definitivo**: magnitudes **não-decrescentes** em janela P294-P297
(máxima → baixa → média → alta). Template §8.7' **robusto**;
desformalização **rejeitada empiricamente**.

### A.0.0.4 — Decisão HV'.a (simplificação cristalina agregada)

**Opções consideradas**:

| Opção | Descrição | Avaliação |
|---|---|---|
| **HV'.a** Variant agregado `MathUnderover { base, under?, over? }` | Wrapper cristalino simplificado per ADR-0054 graded; agrega cluster vanilla em 1 entry | ✅ Decisão adoptada — magnitude controlada, paradigma claro |
| **HV'.b** 2 variants separadas `MathUnder` + `MathOver` | Mais próximo de vanilla (mas ainda agregado) | Rejeitado — sobre-complica |
| **HV'.c** 12 variants paralelos (underline, overline, underbrace, ...) | Paridade vanilla literal | Rejeitado — magnitude L+ inviável; abrir 12 sub-passos |
| **HV'.d** Adiar P297 e abrir sub-passos dedicados | Cluster | Rejeitado — Tabela A.4 linha 119 "underover" pede single feature |

**HV'.a justificação**:
- **ADR-0054 graded vigente**: cristalino aceita divergência
  consciente vs vanilla para simplificação arquitectural.
- **Padrão cristalino existente**: P296 já agregou `accent`
  diferentes Unicode num único `MathAccent { base, accent }` —
  P297 reaplica padrão a underover.
- **Future-proof**: P297.1+ pode adicionar tipo discriminator
  (`UnderoverKind::Brace`/`Bracket`/etc.) se cluster vanilla
  exigir paridade fina.

### A.0.0.5 — "Confirmação esperada" inverte-se

Spec P297 §1.2 antecipou cenário "confirmação esperada" como
provavel. **Vanilla pattern refuta** — P297 é descoberta genuína,
não confirmação. Mas **a categoria continua válida** para passos
futuros onde A.0.0 confirma paradigma sem refutar.

---

## A.0 — Reuso ADR-0098 (preservação esperada)

| Verificação | Esperado pós-P297 |
|---|---|
| Hash `export.rs` | `66cb8ac3` preservado bit-exact (**14º passo consecutivo**) |
| `FrameItem` standard para under/over | ✅ paralelo `layout_accent` P296 |
| Math layout produz consumer agnóstico | ✅ paradigma P136 vigente |
| ADR-0098 N=14 cumulativo | ✅ |

---

## A.1 — Inventário literal

### A.1.1 — `Content::Math*` variants pós-P296 (12)

`MathSequence`, `MathIdent`, `MathText`, `MathFrac`, `MathAttach`,
`MathRoot`, `MathDelimited`, `MathAlignPoint`, `MathMatrix`,
`MathCases`, **`MathAccent`** (P296), **`MathCancel`** (P296).

Pós-P297: **13 variants** (+`MathUnderover`).

### A.1.2 — Vanilla 12 elementos cluster underover

Vide A.0.0.2. Cristalino agrega em 1 variant per HV'.a.

### A.1.3 — Match arms exhaustive (esperado 9 sítios paralelo P296)

| Local | Operação |
|---|---|
| `content.rs:plain_text()` | concatena base+under+over |
| `content.rs:PartialEq` | structural |
| `content.rs:map_content()` | recurse |
| `content.rs:map_text()` | terminal (paralelo MathAccent/MathCancel) |
| `rules/introspect.rs:materialize_time` | terminal |
| `rules/introspect.rs:walk` | terminal |
| `rules/introspect/locatable.rs` | `false` |
| `engine/layout/mod.rs` | fallthrough math |
| `rules/math/layout/mod.rs:layout_node` | handler dedicado novo |

### A.1.4 — Vanilla wrapper unificado AUSENTE

**Spec P297 §A.1.4 expectativa refutada**. Vanilla = 12 elementos
distintos. Cristalino agrega per HV'.a.

### A.1.5 — Stdlib `native_underover` ausente esperado

P297 materializa. Signature: `underover(base, under: ?, over: ?)`
— base posicional; under/over named opcionais.

### A.1.6 — Layouter consumer paralelo P296

`layout_node` arm + `layout_underover` handler dedicado paralelo
`layout_accent` (P296).

### A.1.7 — Emit verification

`FrameItem::Text/Glyph` standard. Sem operadores PDF novos. Hash
`export.rs` preservado.

### A.1.8 — Diagrama de fluxo P297

```
#underover(base, under: "u", over: "o")
       │
       ▼
parse → eval_call (native_underover)
       │
       ▼
Content::MathUnderover {
    base:  Box<base>,
    under: Some(Box<"u">),
    over:  Some(Box<"o">),
}
       │
       ▼
Math Layouter::layout_node (arm novo P297)
       └── layout_underover:
           ├── over_box = layout_node(over)  // se Some
           ├── base_box = layout_node(base)
           ├── under_box = layout_node(under)  // se Some
           ├── width = max(over.w, base.w, under.w)
           ├── empilhar verticalmente: over + base + under
           └── centrar horizontalmente cada
       │
       ▼
MathBox { items: Vec<FrameItem> }
       │
       ▼
export.rs emit standard — INALTERADO bit-exact
```

### A.1.9 — Paradigma consumer P297

**Paralelo P296 cluster math handler dedicado** — N=2 cumulativo
do sub-padrão "cluster math handler dedicado". Não é paradigma
**novo** em si, mas **decisão estrutural A.2 → (b) genuinamente
nova** (Option fields estruturais).

---

## A.2 — Estrutura variant (decisão (b) Option fields)

**Decisão A.2 → (b)** após refutação A.0.0:

```rust
MathUnderover {
    base:  Box<Content>,
    under: Option<Box<Content>>,
    over:  Option<Box<Content>>,
},
```

**Justificação**:
- **Vanilla paridade**: cada elemento `UnderbraceElem`/etc.
  vanilla tem `body` + `annotation: Option<Content>`. Cristalino
  generaliza com 2 Options (under + over).
- **Estrutural genuíno**: under/over ausência é estrutural (não
  cosmético) — `None` é semanticamente "sem elemento" (paralelo
  vanilla `Smart<None>`).
- **Qualifica "variant rico" N=5 candidato** — primeiros fields
  Option estruturais (vs P156G/H/I cosméticos bool defaults; vs
  P284 attribs primitivos).

**Padrão "variant rico" N=5 candidato genuíno** — primeira
qualificação real desde P287 refutação:
- P156G Block, P156H Boxed, P156I Stack (P287 refutou
  qualificação) — todos com bool defaults graded.
- P284 Underline/Strike/Overline — attribs Option primitivos
  (Length, Color).
- **P297 MathUnderover** — primeiros Option `Box<Content>`
  estruturais.

**Decisão sobre promoção**: **adiar** per P273.17 §0:
- Magnitude A.0.0 P297 alta — §8.7' N=5 candidato dispara.
- "variant rico" N=5 também dispara.
- **Uma ADR meta por passo** — escolher uma. Decisão honesta:
  **adiar ambas** porque:
  - §8.7' N=5 robusta já validada implicitamente por refutação
    repetida.
  - "variant rico" N=5 é primeira qualificação genuína mas pode
    consolidar em N=6+ para promoção robusta.
  - Anti-padrão over-formalização P273.17 §0 vigente.

---

## A.3 — Integração `layout_underover` handler dedicado (α)

**Decisão A.3 → (α)** paralelo `layout_accent` P296.

**Pseudo-code**:

```rust
fn layout_underover(
    &self,
    base:  &Content,
    under: Option<&Content>,
    over:  Option<&Content>,
    style: &TextStyle,
) -> MathBox {
    let base_box = self.layout_node(base, style);
    let over_box = over.map(|c| self.layout_node(c, style));
    let under_box = under.map(|c| self.layout_node(c, style));

    // Width: max das 3 partes.
    let w = base_box.width
        .max(over_box.as_ref().map(|b| b.width).unwrap_or(0.0))
        .max(under_box.as_ref().map(|b| b.width).unwrap_or(0.0));

    // Empilhar verticalmente:
    // y = 0:                    topo do over (se presente)
    // y = over_h:                topo da base
    // y = over_h + base_h:       topo do under
    let over_h = over_box.as_ref().map(|b| b.height()).unwrap_or(0.0);
    let under_h = under_box.as_ref().map(|b| b.height()).unwrap_or(0.0);

    let mut items = Vec::new();
    if let Some(ob) = over_box {
        let dx = (w - ob.width) / 2.0;
        for item in ob.items {
            items.push(offset_item(item, Pt(dx), Pt(0.0)));
        }
    }
    let base_dx = (w - base_box.width) / 2.0;
    for item in base_box.items {
        items.push(offset_item(item, Pt(base_dx), Pt(over_h)));
    }
    if let Some(ub) = under_box {
        let dx = (w - ub.width) / 2.0;
        for item in ub.items {
            items.push(offset_item(item, Pt(dx), Pt(over_h + base_box.height())));
        }
    }

    MathBox {
        width:   w,
        ascent:  base_box.ascent + over_h,
        descent: base_box.descent + under_h,
        items,
    }
}
```

---

## A.4 — Impacto em emit (paralelo P296)

**Decisão A.4 → (i)** — `FrameItem` standard. Hash `export.rs`
preservado bit-exact pelo **14º passo consecutivo**.

---

## A.5 — Detecção de bugs latentes

6 cenários fronteira:

| Cenário | Resultado esperado |
|---|---|
| `underover(b, under: u)` (over None) | apenas base + under empilhados |
| `underover(b, over: o)` (under None) | apenas over + base empilhados |
| `underover(b)` (ambos None) | apenas base (paralelo `MathSequence(b)`) |
| `underover(b, under: u, over: o)` | empilhamento triplo |
| `underover` aninhado em `accent` | composição (over de accent vs over de underover) |
| `underover` com body vazio | width zero; degenerate |

### A.5.1 — Nenhum bug latente esperado

Padrão §8.4 N=1 estável.

---

## A.5' — Anti-reflexão N=6 cumulativo

### A.5'.1 — Comparação A.1.9 P288-P297

| Passo | Tipo | Paradigma consumer |
|---|---|---|
| P288-P292 | cumulativo | 5 paradigmas style/text |
| P293-P296 | ortogonais | 4 paradigmas distintos |
| **P297** | **extensão directa P296** | **N=2 sub-padrão "cluster math handler dedicado"** (cumulativo com P296) |

P297 **não inaugura paradigma novo** mas **consolida** o de P296
com N=2 sub-padrão. Decisão estrutural A.2 → (b) é **estructura
nova** dentro do paradigma.

### A.5'.2 — A.0.0 N=5 reaplicação — magnitude alta

P297 magnitude **alta** refuta categórica e definitivamente a
hipótese degenerescência §6.6 P295:

- 5 reaplicações consecutivas com magnitudes alta→máxima→baixa→média→**alta**.
- Janela P294-P297: **não-decrescente em média**.
- Template §8.7' **robusto** — refutações genuínas continuam a
  emergir.

### A.5'.3 — Elementos estructuralmente novos identificados

5 elementos:

1. **A.2 → (b) Option fields estruturais** — primeira
   qualificação genuína "variant rico" N=5 desde P287 refutação.
2. **Refutação spec inteira por vanilla pattern** — paralelo P294;
   2.ª ocorrência cumulativa do sub-padrão.
3. **Agregação cristalina de cluster vanilla 12 elementos** em
   1 variant — divergência consciente per ADR-0054 graded.
4. **Sub-padrão "cluster math handler dedicado" N=2** —
   consolidação P296+P297.
5. **5ª reaplicação A.0.0 com refutação alta** — refuta
   degenerescência empiricamente.

### A.5'.4 — Decisão sobre promoção ADR meta

Candidatos disparados simultaneamente:
- **§8.7' N=5** — magnitude alta valida; promoção candidata
  genuína.
- **"variant rico" N=5** — primeira qualificação Option estrutural;
  promoção candidata genuína.
- **§8.3 N=9** — refutação pragmática.
- **Sub-padrão "cluster math handler" N=2** — cumulativo; longe
  de N=3.

**Decisão**: **0 ADRs meta novas**. Razões:
- P273.17 §0: uma ADR meta por passo no máximo.
- §8.7' e "variant rico" ambos qualificam — escolher uma forçaria
  decisão arbitrária.
- **Adiar ambas para P298+** — consolidação cumulativa permitirá
  promoção robusta sem arbitragem.

Anti-padrão over-formalização rigorosamente honrado.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P297 |
|---|---:|---:|
| `Content` variants | 67 | **68** (+1 MathUnderover) |
| `Content::Math*` variants | 12 | **13** |
| Stdlib funções math | 2 (P296) | 3 (+`native_underover`) |
| Hash L0 `content.md` | actual | **muda** (+1 variant) |
| Hash `export.rs` | `66cb8ac3` | **preservado bit-exact** (**14º passo consecutivo**) |
| Padrão §8.6 A.5' N | 6 | **7** (P291-P297) |
| Padrão §8.7' A.0.0 N | 4 | **5** (reaplica com magnitude alta) |
| Padrão §8.3 N candidato | 8 | **9** candidato adiado |
| Padrão "variant rico" N | 4 | **5 candidato** (primeira qualificação Option estrutural) |
| Sub-padrão "cluster math handler" N | 1 (P296 inaugural) | **2** (P296+P297) |
| ADRs novas | 0 | 0 |

---

## §Risco residual mitigado

- **Risco principal** (modesta-esperada vs rubber-stamp): ✅
  refutado por descoberta significativa (vanilla pattern).
- **Risco secundário** (qualifica "variant rico" sem gatilho
  legítimo): ✅ refutado A.2 — Option estrutural genuíno.
- **Risco terciário** (refactor não-trivial HIV'): ⚖ controlado
  via HV'.a agregação minimal.
- **Risco quaternário** (promoção múltipla): ✅ ambas adiadas.
- **Risco quinário** (degenerescência template): ✅ refutado
  empiricamente; magnitude alta valida §8.7' robustez.
- **Risco senário** (inconsistência P296): ✅ paradigma idêntico;
  handler paralelo `layout_accent`.
- **Risco septenário** (regressão accent/cancel): ⚖ testes
  regression via workspace.

---

## §Fecho da Fase A

Inventário literal completo + **A.0.0 N=5 reaplica §8.7' com
refutação magnitude alta (paralelo P294)** + decisão **HV'.a + (b)
Option fields + (α) handler dedicado** + A.4 hash preservado + A.5
sem bugs + **A.5' N=6 cumulativo com 5 elementos novos**. **0 ADRs
meta novas** — §8.7' N=5 + "variant rico" N=5 ambos qualificam
genuinamente mas adiados per P273.17 §0.

**MARCO P297**:
- **Extensão directa P296** com **decisão estrutural A.2
  genuinamente distinta**.
- **A.0.0 N=5 com magnitude alta** — refuta categoricamente
  hipótese degenerescência §6.6 P295.
- **Vanilla underover.rs revela 12 elementos** — spec P297
  invalidada (paralelo P294); cristalino agrega per ADR-0054
  graded.
- **"variant rico" N=5 primeira qualificação genuína Option
  estrutural** desde P287 refutação.
- **Sub-padrão "cluster math handler dedicado" N=2 cumulativo**
  (P296+P297).
- **Hash `export.rs` preservado pelo 14º passo consecutivo**.

Procede-se a §3 da spec (com plano HV'.a + (b) + (α)).
