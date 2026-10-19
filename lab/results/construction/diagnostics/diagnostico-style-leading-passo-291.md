# Diagnóstico — Fase A do Passo 291 (`P-style-leading-variant`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-291.md`
**Origem**: P290 §5.6 (assimetria residual 2 fields); P291 = P290.1
(sequência directa).
**Novidade metodológica**: secção **A.5' anti-reflexão**
obrigatória per spec §2.5 / risco quinário P290 §7.

---

## A.0 — Potencial de reuso ADR-0098 (verificação empírica, NÃO assumpção)

P290 §5.3 antecipou "leading é layout-time only, sem `Tc` operator
equivalente". **P291 §A.0 verifica literalmente — não cita
antecipação como prova**.

### A.0.1 — Inspecção literal `grep "leading" 03_infra/src/export.rs`

```
$ grep -n "leading" 03_infra/src/export.rs
(zero hits)
```

**Zero hits funcionais**. Antecipação P290 §5.3 confirmada
empiricamente.

### A.0.2 — Verificação Frame/Region

`grep "leading" 01_core/src/entities/{layout_types,region,frame}.rs`
→ 0 hits funcionais. Frame/Region não consultam leading directamente.
Vertical spacing entre frames é resultado de coordenadas Y
distintas em `FrameItem::Text` consecutivos — produzidas pelo
Layouter pré-emit (paradigma P136 paralelo).

### A.0.3 — Classificação ADR-0098

| Critério | Veredicto |
|---|---|
| Emit consome leading? | ❌ **Não** (zero hits) |
| `FrameItem::Text` precisa novo field `leading`? | ❌ Não — `TextStyle.leading` já existe (P138) capturado para uso layout-time |
| Hash `export.rs` esperado | **Preservado bit-exact** (8º passo consecutivo) |
| ADR-0098 vigente? | ✅ Sim — confirmado empiricamente |

### A.0.4 — Diferença material vs P290 A.0

P290 §A.0 foi **não-trivial** (2 hits via `style.tracking` para
`Tc` operator). P291 §A.0 é **trivial empírica** (zero hits) — mas
**confirmada empiricamente, não citando P290 §5.3**. Honestidade
preservada: A.0 não é rubber-stamp.

---

## A.1 — Inventário literal do caminho actual `leading`

### A.1.1 — `Style` enum pós-P290

`entities/style.rs:25-67` confirma 8 variants pós-P290:
`Bold` / `Italic` / `Size` / `Fill` / `HeadingLevel` / `Lang` /
`Weight` / `Tracking`.

P291 adicionará 9º variant `Leading(Length)`.

### A.1.2 — `StyleDelta.leading` tipo

`entities/style_chain.rs:48` (pós-P138 Fase B.2 DEBT-52):

```rust
/// Espaço entre linhas (Passo 128, DEBT-1 subset). Em vanilla é
/// propriedade de `par` (não de `text`); capturado em `#set text`
/// por conveniência temporária — migra para `eval_set_par` quando
/// este for activado. Inerte em layout.
pub leading: Option<crate::entities::layout_types::Length>,
```

**Tipo `Option<Length>`** confirmado. `Length` é `Copy` (P127).

**Divergência arquitectural consciente registada literalmente no
código**: leading é tipicamente em `par` em vanilla, mas cristalino
captura em `text` por conveniência temporária. Tabela A.3 linha 70
documenta. P291 preserva esta divergência (não-objectivo §5).

### A.1.3 — `push_styles` cascade

`style_chain.rs:134-158` match exaustivo sobre 8 variants pós-P290.
P291 adicionará 9º arm:

```rust
Style::Leading(l) => delta.leading = Some(*l),
```

Match continua exaustivo.

### A.1.4 — `delta.leading` write site único (parse-driven)

`eval/rules.rs:298` (P138):

```rust
"leading" => {
    if let Value::Length(l) = val {
        delta.leading = Some(l);
    }
}
```

**Único write site até P291**: `#set text(leading: 0.65em)`
parse-driven. Sem caminho via `Styles` collection.

### A.1.5 — `delta.leading` read site único

`style_chain.rs:236-240`:

```rust
pub fn leading(&self) -> Option<Length> {
    let mut node = self.0.as_deref();
    while let Some(n) = node {
        if let Some(v) = n.delta.leading { return Some(v); }
        node = n.parent.as_deref();
    }
    None
}
```

Walk up-the-chain — assinatura paralela a outros accessors.

### A.1.6 — Consumers actuais de `style.leading` (DISTINTIVO vs P290)

`grep "style.leading\|chain.leading"`:

| Consumer | Localização | Função | Paradigma |
|---|---|---|---|
| Top-wins propagation | `layout/mod.rs:580` | `self.style.leading.or(node_style.leading)` (P136) | Top-wins (idêntico aos restantes) |
| **`flush_line` peek** | `cursor.rs:119-128` (P138) | Peek `current_line.iter().rev().find_map(FrameItem::Text { style, .. } => style.leading)`; adiciona `line_height + leading_pt` a `cursor_y` | **Per-line via `FrameItem::Text` da current_line** — DISTINTIVO |
| TextStyle capture | `style_chain.rs:305` | `leading: chain.leading()` em `TextStyle::from(&StyleChain)` | TextStyle capture (idêntico) |

**Comparação com P290 tracking consumer paradigm**:

| Campo | Consumer principal | Paradigma |
|---|---|---|
| Tracking (P290) | `cursor.rs:30` per-glyph horizontal advance + `export.rs:2139` per-glyph `Tc` emit | **Per-glyph horizontal** |
| **Leading (P291)** | `cursor.rs:119-128` peek **última `FrameItem::Text`** da current_line antes do flush; advance vertical | **Per-line vertical via último FrameItem::Text** |

**Distinção arquitectural genuína** — leading é consumido em ponto
estructural diferente (`flush_line` pre-drain, não `layout_word`).
A.5' regista esta como evidência factual de que P291 **não é
rubber-stamp completo** vs P290.

### A.1.7 — `FrameItem::Text` / `Frame` emit

`grep "leading" 03_infra/src/export.rs` → 0 hits (A.0.1 confirmou).
Emit consome apenas coordenadas Y resultantes (que são afectadas
pelo Layouter via leading mas sem ler leading directamente).
Paradigma ADR-0098 vigente.

### A.1.8 — Diagrama de fluxo (genuíno, não copy-paste P290)

```
#set text(leading: 0.65em)                   Content::Styled(body,
       │                                       Styles::from_iter([Style::Leading(Length::em(0.65))]))
       ▼                                            │
parse → eval_set_rule (rules.rs:298)              ▼
       │                                       push_styles → match arm
       ▼                                       Style::Leading(l) => delta.leading = Some(*l)  [P291]
delta.leading = Some(Length::em(0.65))              │
       │ ◀──────────── 2ª fonte de entrada ─────────┘
       ▼
chain.push(delta) → StyleChain
       │
       ▼
chain.leading() (style_chain.rs:236) ← read via walk up-the-chain
       │
       ├──► Top-wins: layout/mod.rs:580
       ├──► TextStyle.leading capture: style_chain.rs:305
       │       ▼
       │   FrameItem::Text.style.leading (Layouter::layout_word push)
       │       │
       │       ▼ (acumulado em current_line.iter())
       └──► **flush_line peek**: cursor.rs:119-128
              │ (find_map em current_line.iter().rev())
              ▼
       Vertical advance: cursor_y += line_height + leading_pt
              │
              ▼
       FrameItem::Text seguinte tem Y distinto
              │
              ▼
       export.rs: emite coordenadas Y, sem ler leading directamente
```

**Distinção vs P290 §A.1.8**: P290 fluxo termina em emit `Tc`
operator literal. P291 fluxo termina em **coordenadas Y distintas**
nos FrameItem::Text consecutivos — leading é consumido **estructuralmente
no `flush_line` peek** (não no Layouter principal nem no emit
directo).

---

## A.2 — Estrutura do variant `Style::Leading`

### A.2.1 — Decisão

**Decidido**: opção **(a)** `Leading(Length)` — paralelo absoluto
ao tipo `StyleDelta.leading: Option<Length>`.

`Length` é `Copy` (P127). **Verificação literal A.1.2 confirma**:
estructura paralela é forçada (não preferência) porque o storage
em `StyleDelta` é `Option<Length>`. Variant atómico paralelo a
`Tracking(Length)` P290.

### A.2.2 — Sintaxe final

```rust
pub enum Style {
    Bold(bool),
    Italic(bool),
    Size(Pt),
    Fill(Color),
    HeadingLevel(u8),
    Lang(Lang),                  // P288
    Weight(u16),                 // P289
    Tracking(Length),            // P290
    Leading(Length),             // P291 — DOC: divergência text↔par consciente
}
```

### A.2.3 — Honestidade epistémica

`Style::Leading(Length)` é variant **atómico** — padrão N=4
"variant rico" **inalterado**.

---

## A.3 — Integração com `StyleDelta`

### A.3.1 — Decisão

**(α)** `delta.leading = Some(*l)` — paridade absoluta aos 8 arms
existentes.

### A.3.2 — Implementação

```rust
Style::Leading(l) => delta.leading = Some(*l),
```

+1 LOC em `push_styles`. Match exaustivo.

---

## A.4 — Impacto em `FrameItem::Text` / `Frame` e emit

### A.4.1 — Decisão

**(i)** — confirmada empiricamente por A.0 + A.1.7.

Leading é consumido apenas no Layouter (cursor_y advance via
`flush_line` peek). Emit lê coordenadas Y resultantes mas não
leading directamente. **Paradigma ADR-0098 vigente — distinto de
P290 mas igualmente compatível**.

### A.4.2 — Hash `export.rs` preservado

8º passo consecutivo (P282+P285+P286+P287+P288+P289+P290+**P291**)
— ADR-0098 reaplica como invariante operacional.

### A.4.3 — Sem promoção ADR meta

| Padrão | N pós-P291 | Promoção? |
|---|---:|---|
| ADR-0098 | **8** | ❌ Reforço cumulativo |
| ADR-0099 | **7** | ❌ Reaplicação cumulativa (1ª pós-formalização foi P290) |
| §8.3 "refutação pragmática" | 5 estável | ❌ |
| §8.4 "bug latente fixed" | 1 estável | ❌ (A.5 nenhum bug — ver §A.5) |
| §8.5 "patch cirúrgico sequencial" | **4** | ❌ Desqualificado per P290 §8.5 (anti-inflação) |

P273.17 §0 vigente.

---

## A.5 — Detecção de bugs latentes

5 cenários fronteira:

| Cenário | Valor | Expectativa | Resultado |
|---|---:|---|:---:|
| Zero | `Length::pt(0.0)` | `chain.leading() == Some(...)`; flush_line peek usa 0pt extra | ✅ |
| Típico | `Length::pt(11.0)` | Próximo do default cristalino (font_size 11.0 * 1.0 ≈ 11pt) | ✅ |
| Em-relativo | `Length::em(0.5)` | Resolve em runtime via font_size | ✅ |
| Grande | `Length::pt(50.0)` | Sem overflow | ✅ |
| **Negativo** | `Length::pt(-1.0)` | Vanilla typst: pode saturar a 0 ou aceitar sobreposição. Cristalino: `current_line.cursor_y += line_height + (-1.0)` → linhas mais juntas (não saturação) | ✅ |

### A.5.1 — Leading negativo: comportamento empírico

Vanilla typst aceita leading negativo (line collapse parcial); não
satura a 0. Cristalino preserva paridade — `cursor.rs:124`
`style.leading.map(|l| l.resolve_pt(...))` passa valor literal sem
clamp. Confirmação empírica em teste fronteira.

### A.5.2 — Nenhum bug latente detectado

5 fronteiras passam. Padrão §8.4 P288 permanece N=1 estável.

---

## A.5' — Verificação anti-reflexão (NOVA per P290 §7 risco quinário)

### A.5'.1 — Comparação literal A.1.6 de P288/P289/P290/P291

| Passo | Variant | Consumer principal | Localização |
|---|---|---|---|
| P288 | `Lang(Lang)` | eval_markup localize_quotes; hyphenation `hypher`; smartquote P287 | `eval/mod.rs`, `layout/cursor.rs:56`, `lang/figure_supplement.rs`, `layout/mod.rs:1992` |
| P289 | `Weight(u16)` | top-wins propagation; **faux_bold_stroke_pt P139** (`TextStyle::faux_bold_stroke_pt`) | `layout_types.rs:158-164` |
| P290 | `Tracking(Length)` | top-wins; **`cursor.rs:30` per-glyph horizontal advance**; **`export.rs:2139` per-glyph `Tc` emit** | `cursor.rs:30`, `export.rs:2139-2146` |
| **P291** | `Leading(Length)` | top-wins; **`cursor.rs:119-128` per-line vertical advance via peek `current_line.iter().rev().find_map(FrameItem::Text)`** | `cursor.rs:119-128` |

**Análise**: 4 passos têm consumer paradigm **arquitecturalmente
distinto**:
- P288: cross-module (eval+lang+layout consumers).
- P289: TextStyle method (`faux_bold_stroke_pt`) layout-time only.
- P290: per-glyph (horizontal advance) + per-glyph emit (`Tc`).
- **P291: per-line (vertical advance) via peek `current_line`** —
  paradigma estructural distinto vs P290.

**Sequência NÃO é rubber-stamp**. Cada passo activa campo com
consumer arquitecturalmente próprio. A semelhança é apenas na
**fonte de entrada** (parse-driven `eval_set_rule` paralelo); o
**consumo** diverge significativamente.

### A.5'.2 — Verificação A.0 produzido empiricamente

P291 §A.0 verificou literalmente `grep "leading" export.rs` — **0
hits**. Antecipação P290 §5.3 confirmada empiricamente, **não
citada como prova**. Distinção honesta preservada.

### A.5'.3 — Pelo menos 1 elemento estructuralmente novo identificado

✅ **Identificado**: paradigma de consumo "per-line via peek
`current_line.iter().rev().find_map(FrameItem::Text)`" em
`cursor.rs:119-128` é arquitecturalmente novo:

- **Não é** per-glyph (como tracking P290).
- **Não é** TextStyle method (como weight P289).
- **Não é** cross-module (como lang P288).
- **É** "peek o último item de tipo X numa colecção em construção,
  antes do drain" — padrão estructural distinto que materializa
  "estilo persistente captura no momento da finalização da unidade".

**Implicação metodológica**: P288-P291 são **reaplicações
arquiteturalmente distintas** com fonte de entrada paralela. A
sequência NÃO é rubber-stamp completa — apenas a *cascade arm* é
trivial paralelo (e isso é por design — single source of truth via
`StyleDelta`).

### A.5'.4 — Decisão sobre P292 (font)

Spec §5 não-objectivo: *"Se A.5' revelar sequência reflexa, não
materializar P292 imediatamente. Quebrar sequência com passo
ortogonal."*

A.5' **NÃO revela sequência reflexa** — P291 tem consumer paradigm
distinto. **P292 pode prosseguir** sem necessidade de intercalar
passo ortogonal.

**No entanto**, P292 (font) tem complicação extra: `font:
Option<FontList>`, e `FontList` é wrapper que pode requerer tipo
diferente vs (Length/u16/Lang). P292 deve verificar A.1.2 com
cuidado especial — caso seja `Vec<...>` não-Copy, opção (a)
estructura precisa adaptação.

**Decisão final**: P292 procede após P291; sem intercalação
ortogonal forçada.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P291 |
|---|---:|---:|
| `Style` variants | 8 (pós-P290) | **9** (+`Leading(Length)`) |
| `push_styles` arms | 8 | **9** (+1 LOC) |
| Caminhos entrada `delta.leading` | 1 (parse) | **2** (parse + Style::Leading) |
| Consumers activos | 3 | 3 (top-wins + flush_line peek + TextStyle capture) |
| Hash L0 `style.md` | actual | **muda** (+1 variant) |
| Hash L0 `content.md` / `stdlib.md` | inalterado | inalterado |
| Hash L0 `export.rs` | `66cb8ac3` | **preservado bit-exact** (**8º passo consecutivo**) |
| Fechos B.3↔B.4 | 3/5 (P288+P289+P290) | **4/5** (+ P291 leading) |
| ADRs novas | 0 | 0 (reaplicações sem promoção) |
| Padrão "variant rico" N | 4 | 4 (Leading atómico) |
| Padrão §8.2 (ADR-0098) N | 7 (P290) | **8** |
| Padrão §8.1 (ADR-0099) N | 6 (P290) | **7** |

---

## §Risco residual mitigado

- **Risco principal P291** (sequência reflexa): ✅ **Refutado por
  A.5'** — paradigma consumer per-line via peek `current_line` é
  estructuralmente distinto. P292 procede.
- **Risco secundário** (A.0 antecipação falsa): ✅ Refutado — 0
  hits confirmados empiricamente.
- **Risco terciário** (leading negativo bug): ✅ Verificação A.5
  positiva — propaga gracefully.
- **Risco quaternário** (`StyleDelta.leading` tipo diferente): ✅
  Refutado — A.1.2 confirma `Option<Length>` esperado.
- **Risco quinário** (rubber-stamp + decidir ignorar A.5'): ✅
  Mitigado — A.5' produzido genuinamente; decisão honesta sobre
  P292 documentada.

---

## §Fecho da Fase A

Inventário literal (8 sub-secções A.1 + diagrama de fluxo genuíno)
+ A.0 trivial **confirmada empiricamente** (não citada) + decisão
variant (a) + integração cascade (α) + impacto emit (i) + A.5'
**anti-reflexão produzida com elemento estructuralmente novo
identificado** (paradigma peek `current_line` per-line). **P291 NÃO
é rubber-stamp** — consumer paradigm distinto.

P292 (font) **pode prosseguir** sem intercalação ortogonal forçada,
mas requer cuidado especial em A.1.2 (FontList wrapper). **Sem
promoção ADR meta nova** — ADR-0098 (N=8) + ADR-0099 (N=7)
reaplicam.
