# Diagnóstico — Fase A do Passo 290 (`P-style-tracking-variant`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-290.md`
**Origem**: P289 §5.6 (assimetria residual 3 fields); P290 = P289.1
(sequência directa paralela a P288→P289).

---

## A.0 — Potencial de reuso ADR-0098 (verificação **não-trivial** desta vez)

P289 §A.0 trivial (zero hits weight em export.rs). P290 §A.0 espera
**hits no formato `Tc` operator** dado P137 implementou tracking em
PDF emit. **Inspecção empírica obrigatória.**

### A.0.1 — Inspecção literal `grep "tracking\|Tc " 03_infra/src/export.rs`

| Linha | Conteúdo | Classificação |
|---|---|---|
| `:2139-2141` | `let tracking_pt = style.tracking.map(\|t\| t.resolve_pt(style.size.val())).unwrap_or(0.0);` | **Leitura via `style.tracking`** (i.e. `TextStyle.tracking` capturado em `FrameItem::Text.style`) |
| `:2142-2146` | `let tc_op = if tracking_pt.abs() > f64::EPSILON { format!("{:.2} Tc\n", tracking_pt) } else { String::new() };` | Emit `Tc` operator |

**2 hits** — ambos consomem `style.tracking` (não `chain.tracking()`).
**Paradigma P136 confirmado**: emit lê de `FrameItem::Text.style.tracking`,
**não** via chain directo.

### A.0.2 — Classificação ADR-0098

| Critério | Veredicto |
|---|---|
| Emit consome via `FrameItem::Text.style.tracking` (P136 paradigm)? | ✅ Sim |
| Emit consome via `chain.tracking()` directo? | ❌ Não (chain só lida com `StyleDelta` walk) |
| ADR-0098 vigente? | ✅ **Sim — confirmado empiricamente** |
| Hash `export.rs` esperado | **Preservado bit-exact** (7º passo consecutivo) |

### A.0.3 — Diferença material vs P289 A.0

P289 verificou ausência total de `weight` em export.rs (paradigma
trivial). P290 verifica **presença de `Tc` operator MAS canalizado
via `FrameItem::Text.style.tracking`** — não via `chain` directo.

Esta distinção materializa **A.0 como verificação não-trivial**:
ADR-0098 não exige ausência total de menção a um campo em emit; exige
que **o caminho de consumo seja via `FrameItem.style` capturado pelo
Layouter** (single source of truth pré-emit). Confirmação empírica
**valida ADR-0098 como mais subtil e robusta do que aparente**.

### A.0.4 — Citação literal ADR-0098

> "Qualquer feature nova que possa ser implementada reutilizando
> primitivas L1/L3 pré-existentes (`FrameItem` variants, emit
> helpers) **deve fazê-lo** em vez de criar emit ad-hoc."

P290 reusa o caminho **2ª fonte de entrada via `Style::Tracking(Length)`
→ `delta.tracking` → `chain.tracking()` → `TextStyle::from(&chain)`
→ `FrameItem::Text.style.tracking` → emit `Tc` operator (P137)**.
**Reuso estructural absoluto** desde primitivas pré-existentes.

---

## A.1 — Inventário literal do caminho actual `tracking`

### A.1.1 — `Style` enum pós-P289

`entities/style.rs:25-54` confirma 7 variants pós-P289:
`Bold` / `Italic` / `Size` / `Fill` / `HeadingLevel` / `Lang` / `Weight`.

P290 adicionará 8º variant `Tracking(Length)`.

### A.1.2 — `StyleDelta.tracking` tipo

`entities/style_chain.rs:46` (pós-P136 Fase A DEBT-52):

```rust
/// Espaçamento adicional entre glyphs (Passo 127, ADR-0038). Preserva
/// `Length` inteiro (`abs + em`); resolve para pt quando consumer
/// conhecer font-size. Inerte em layout.
pub tracking: Option<crate::entities::layout_types::Length>,
```

**Tipo `Option<Length>`**. `Length` é `Copy` (`layout_types.rs:601`
`#[derive(Debug, Clone, Copy, PartialEq)]`) → `Style::Tracking(Length)`
preserva `Style: Copy` intacto.

### A.1.3 — `push_styles` cascade match

`style_chain.rs:134-149` match exaustivo sobre 7 variants pós-P289.
**P290 adicionará 8º arm**:

```rust
Style::Tracking(l) => delta.tracking = Some(*l),  // P290
```

Match continua exaustivo — compilador detecta omissões.

### A.1.4 — `delta.tracking` write site único (parse-driven)

`eval/rules.rs:374` (P137):

```rust
"tracking" => {
    // Passo 127 (ADR-0038 anotada, DEBT-1 subset): captura
    // `Length` inteiro (`abs + em`). Não colapsa para pt —
    // consumer resolve com `font-size` quando existir.
    if let Value::Length(l) = val {
        delta.tracking = Some(l);
    }
}
```

**Único write site até P290**: `#set text(tracking: 0.1em)` ou
`tracking: 1pt` via parse. Sem caminho via `Styles` collection.

### A.1.5 — `delta.tracking` read site único

`style_chain.rs:216-223` (`fn tracking(&self)`):

```rust
pub fn tracking(&self) -> Option<Length> {
    let mut node = self.0.as_deref();
    while let Some(n) = node {
        if let Some(v) = n.delta.tracking { return Some(v); }
        node = n.parent.as_deref();
    }
    None
}
```

Walk up-the-chain — paralelo absoluto a `weight()`/`lang()`/etc.

### A.1.6 — Consumers actuais de `style.tracking`

| Consumer | Localização | Função |
|---|---|---|
| Top-wins propagation | `layout/mod.rs:579` | `self.style.tracking.or(node_style.tracking)` — P136 Fase A DEBT-52 |
| Layout consumer (extra width) | `layout/cursor.rs:30` | `tracking_extra` adiciona ao avanço por glyph — **P137 active consumer** |
| TextStyle capture | `style_chain.rs:295` | `tracking: chain.tracking()` em `TextStyle::from(&StyleChain)` |
| **PDF emit `Tc` operator** | `export.rs:2139-2146` | Lê `style.tracking.resolve_pt(style.size)` e emite `{:.2} Tc\n` se `> EPSILON` (P137) |

**4 consumers activos** — incluindo emit PDF. Diferença material vs
P289 weight: tracking tem **consumer em emit** (vs weight que só
tem consumer em layout-time via faux-bold). Mas o consumer emit é
via `FrameItem::Text.style.tracking` (P136 paradigm) — **não viola
ADR-0098** porque vai via `TextStyle` capturado, não via chain
directo.

### A.1.7 — `FrameItem::Text` emit (verificação literal A.0)

Confirmado em A.0.1: `export.rs:2139` lê `style.tracking` (i.e.
`TextStyle.tracking`). `TextStyle` é capturado em
`FrameItem::Text.style` durante `Layouter::layout_word`. P290
adiciona apenas 2ª fonte de entrada na cascade — `TextStyle::from(&chain)`
propaga automaticamente.

### A.1.8 — Diagrama de fluxo

```
#set text(tracking: 0.1em)                   Content::Styled(body,
       │                                       Styles::from_iter([Style::Tracking(Length::em(0.1))]))
       ▼                                            │
parse → eval_set_rule (rules.rs:374)              ▼
       │                                       push_styles → match arm
       ▼                                       Style::Tracking(l) => delta.tracking = Some(*l)  [P290]
delta.tracking = Some(Length::em(0.1))              │
       │ ◀──────────── 2ª fonte de entrada ─────────┘
       ▼
chain.push(delta) → StyleChain
       │
       ▼
chain.tracking() (style_chain.rs:216) ← read via walk up-the-chain
       │
       ├──► Top-wins: layout/mod.rs:579
       ├──► Layout consumer: cursor.rs:30 (tracking_extra glyph advance)
       ├──► TextStyle.tracking capture: style_chain.rs:295
       └──► FrameItem::Text.style.tracking (Layouter::layout_word push)
              │
              ▼
       export.rs:2139-2146 (emit) ← `Tc` operator se > EPSILON
       │
       ▼
       PDF stream: `{:.2} Tc` literal
```

**Distinção crítica vs P288/P289 §A.1.8**: P290 tem emit consumer
real (`Tc` operator), mas via `TextStyle.tracking` capturado em
`FrameItem::Text` — **não via chain directo**. ADR-0098 vigente.

---

## A.2 — Estrutura do variant `Style::Tracking`

### A.2.1 — Decisão

**Decidido**: opção **(a)** `Tracking(Length)` — paralelo absoluto
ao tipo `StyleDelta.tracking: Option<Length>`.

| Opção | Veredicto |
|---|---|
| (a) `Tracking(Length)` | ✅ Escolhida — `Length` é `Copy` → `Style: Copy` intacto; preserva semântica (`abs + em`) |
| (b) `Tracking(f64)` raw | ❌ Perde semântica `Em` vs `Pt`; divergência com `StyleDelta.tracking` |
| (c) `Tracking(LengthVariant)` | ❌ Indirecção sem ganho |

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
    Tracking(Length),            // P290 — paralelo a StyleDelta.tracking
}
```

### A.2.3 — Honestidade epistémica (paralelo P287/P288/P289)

`Style::Tracking(Length)` é variant **atómico** (1 campo `Length`
required). **Não qualifica** como "variant rico". Padrão N=4
"variant rico" **inalterado** pelo P290.

---

## A.3 — Integração com `StyleDelta`

### A.3.1 — Decisão

**Decidido**: opção **(α)** `delta.tracking = Some(*l)` — paridade
absoluta aos 7 arms existentes.

### A.3.2 — Implementação literal

```rust
// Em StyleChain::push_styles (style_chain.rs:138-149):
match style {
    Style::Bold(b)         => delta.bold = Some(*b),
    Style::Italic(i)       => delta.italic = Some(*i),
    Style::Size(pt)        => delta.size = Some(pt.val()),
    Style::Fill(c)         => delta.fill = Some(*c),
    Style::HeadingLevel(l) => delta.heading_level = Some(*l),
    Style::Lang(l)         => delta.lang = Some(*l),
    Style::Weight(w)       => delta.weight = Some(*w),
    Style::Tracking(l)     => delta.tracking = Some(*l),  // P290
}
```

**+1 LOC no match.** Match continua exaustivo.

---

## A.4 — Impacto em `FrameItem::Text` e emit (aplicação ADR-0098)

### A.4.1 — Análise empírica do gatilho

**Decidido**: opção **(i)** — confirmada empiricamente por A.0 + A.1.7.

| Critério | Evidência empírica |
|---|---|
| `export.rs` consulta `style.tracking`? | A.0.1: 2 hits — ambos via `style.tracking` (P136 paradigm) |
| `export.rs` consulta `chain.tracking()` directamente? | A.0.1: 0 hits — paradigma `TextStyle` capture preservado |
| `FrameItem::Text` precisa novo field `tracking`? | **Não** — `TextStyle.tracking` já existe (P127/P136) |
| Consumer P137 emit Tc operator activo? | **Sim** — `export.rs:2139-2146` |
| Reflectors directos a `chain`? | **Não** |

### A.4.2 — Aderência ADR-0098: hash `export.rs` preservado

**P290 confirma ADR-0098 pelo 7º passo consecutivo** (P282+P285+P286+
P287+P288+P289+P290). **Diferença distintiva**: P290 é a **primeira
aplicação onde emit consome literalmente o campo activado**, mas via
paradigma `TextStyle` capture — confirma **subtileza arquitectural
da ADR-0098**: o critério não é "ausência total em emit" mas "via
`FrameItem.style` capturado".

### A.4.3 — Padrões cumulativos sem promoção

| Padrão | N atual | Promoção em P290? |
|---|---:|---|
| §8.1 (ADR-0099 já formalizada P289) | 6 cumulativo (P285-P290) | ❌ Sem nova promoção; reaplicação directa |
| §8.2 (ADR-0098 já formalizada P288) | 7 cumulativo (P282+P285-P290) | ❌ Sem nova promoção; **7º passo consecutivo** reforça invariante |
| §8.3 "refutação pragmática" | 5 estável (P289 não adicionou) | ⏸ P290 também segue defaults straight |
| §8.4 "bug latente fixed" | 1 (P288 NBSP) | ⏸ A.5 testa 5 fronteiras — verificação |
| §8.5 "patch cirúrgico sequencial" | 2 (P288+P289) | ⏸ **N=3 com P290** mas §8.5 P289 desqualifica passos cumulativos triviais — não promover |

**Decisão**: nenhuma ADR meta promovida. P273.17 §0 anti-padrão
over-formalização vigente.

---

## A.5 — Detecção de bugs latentes (padrão P288 §8.4)

5 cenários fronteira (1 a mais que P289 — tracking negativo é caso
adicional crítico):

| Cenário | Valor | Expectativa |
|---|---:|---|
| Zero | `Length::pt(0.0)` | `chain.tracking() == Some(...)`; emit não emite `Tc` (EPSILON branch) |
| Pequeno positivo | `Length::pt(1.0)` | Emit `1.00 Tc` |
| Em-relativo | `Length::em(0.5)` | Resolve em runtime para `size * 0.5`; emit conforme |
| **Negativo** | `Length::pt(-0.5)` | Vanilla aceita (kerning artificial); emit `-0.50 Tc` |
| Grande | `Length::pt(10.0)` | Wrap natural deve continuar a funcionar |

### A.5.1 — Plano de testes

5 testes injection paralelos a P289:
- `p290_style_tracking_variant_basico` — ctor + PartialEq.
- `p290_push_styles_tracking_projecta_no_delta` — cascade arm.
- `p290_styled_tracking_injetado_chain_le_corretamente` — Styled
  wrapping + `chain.tracking()`.
- `p290_tracking_last_write_wins` — 2 `Style::Tracking` consecutivos.
- 5 testes fronteira (cenários A.5).

### A.5.2 — Atenção particular ao tracking negativo

Vanilla typst aceita tracking negativo como kerning artificial.
Cristalino deve preservar paridade — A.5 explicita teste dedicado
`p290_tracking_negative_propaga`.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P290 |
|---|---:|---:|
| `Style` variants | 7 (pós-P289) | **8** (+`Tracking(Length)`) |
| `push_styles` arms | 7 | **8** (+1 LOC) |
| Caminhos de entrada para `delta.tracking` | 1 (parse) | **2** (parse + Style::Tracking) |
| Consumers activos | 4 | 4 (inalterado — emit consumer P137 já existia) |
| Hash L0 `style.md` | actual | **muda** (+1 variant) |
| Hash L0 `content.md` / `stdlib.md` | inalterado | inalterado |
| Hash L0 `export.rs` | `66cb8ac3` | **preservado bit-exact** (**7º passo consecutivo**) |
| Fechos de assimetria B.3↔B.4 | 2/5 (P288 lang + P289 weight) | **3/5** (+ P290 tracking) |
| ADRs meta cumulativas | 2 (ADR-0098, ADR-0099) | 2 (sem novas; reaplicações sem promoção) |
| Padrão "variant rico" N | 4 | 4 (Tracking atómico, não rico) |
| Padrão §8.2 (ADR-0098) N | 6 (P289 limiar) | **7** (reforço cumulativo) |
| Padrão §8.1 (ADR-0099) N | 5 (P289 formalização) | **6** (1ª reaplicação pós-formalização) |

---

## §Risco residual mitigado

- **Risco principal** (A.0 violação nominal ADR-0098): **refutado
  empiricamente** — A.0.1 confirma paradigma `TextStyle` capture
  (P136); 2 hits em `export.rs` são via `style.tracking`, não via
  `chain.tracking()` directo.
- **Risco secundário** (`Length` não Copy): **refutado** — A.1.2
  confirma `Length` é `Copy`. `Style: Copy` preservado.
- **Risco terciário** (tracking negativo bug): verificação prevista
  em A.5 — `Length::pt(-0.5)` deve propagar transparentemente.
- **Risco quaternário** (promoção §8.5 indevida): refutado por A.4.3
  + spec §7 risco quaternário explícito (relatório §8.5 P289
  desqualifica passos cumulativos triviais).
- **Risco quinário** (sequência reflexa): mitigado por Fase A
  completa não-trivial em A.0 (verificação `Tc` operator empírica
  genuína, não rubber-stamp idêntico a P289).

---

## §Fecho da Fase A

Inventário literal (8 sub-secções A.1 + diagrama de fluxo) +
**A.0 NÃO-TRIVIAL verificada empiricamente** (paradigma `TextStyle`
capture confirmado; ADR-0098 vigente) + decisão variant (a) +
integração cascade (α) + impacto emit (i) registadas. **ADR-0098
robusta confirmada empiricamente** — primeira aplicação onde emit
consome literalmente o campo activado mas via paradigma `FrameItem.style`.
**Nenhuma promoção ADR meta nova** — ADR-0098 + ADR-0099 já cobrem
os padrões aplicáveis.

Procede-se a §3 do passo sem violação nominal ADR-0098.
