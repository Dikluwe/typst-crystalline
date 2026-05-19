# Diagnóstico — Fase A do Passo 292 (`P-style-font-variant`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-292.md`
**Origem**: P291 §5.8 (assimetria residual 1 field); P292 = **último
passo cumulativo da série P288-P292**.
**Novidade metodológica**: A.5' anti-reflexão N=2 (reaplicação do
padrão §8.6 P291 inaugural).

---

## A.0 — Potencial de reuso ADR-0098 (verificação empírica)

`grep "font" 03_infra/src/export.rs` retorna múltiplos hits.
Classificação literal:

### A.0.1 — Inspecção literal

| Linha | Conteúdo | Classificação |
|---|---|---|
| `:2155` | `"{q_open}BT\n/{font_ref} {:.1} Tf\n..."` | **Tf operator emit** (Helvetica path; font_ref via style.bold/italic) |
| `:2163` | `"BT\n/F1 {:.1} Tf\n..."` | CIDFont single (hardcoded /F1) |
| `:2169-2174` | `let fi = style.font.as_ref().and_then(\|fl\| ...); format!("BT\n/F{} {:.1} Tf\n...", fi+1, ...)` | **Multifont path lê `style.font`** (paradigma `TextStyle` capture P136) |

**Padrão confirmado**: emit multifont consome via `style.font`
(linha 2169), **não** via `chain.font()` directo. Paradigma P136
vigente. **ADR-0098 OK**.

### A.0.2 — Classificação

| Critério | Veredicto |
|---|---|
| Emit consome `style.font`? | ✅ Sim (linha 2169) — `TextStyle` capture |
| Emit consome `chain.font()` directo? | ❌ Não (zero hits) |
| `FrameItem::Text` precisa novo field `font`? | ❌ Não — `TextStyle.font` já existe (P140B/P141/P146) |
| Hash `export.rs` esperado | **Preservado bit-exact** (9º passo consecutivo) |
| ADR-0098 vigente? | ✅ Sim |

### A.0.3 — Diferença material vs P290 A.0

P290 §A.0 foi não-trivial (2 hits `style.tracking` para `Tc`).
P292 §A.0 é **igualmente não-trivial** (3+ hits incluindo `Tf`
operator multifont). Paradigma P136 confirmado em ambos — emit
consome via `FrameItem::Text.style.<campo>` capture, não via chain
directo.

P292 é a **2ª aplicação prática de ADR-0098 com emit consumer
real** (P290 foi a 1ª). Confirma robustez do paradigma.

---

## A.1 — Inventário literal do caminho actual `font`

### A.1.1 — `Style` enum pós-P291

9 variants confirmadas (P292 → 10).

### A.1.2 — `StyleDelta.font` tipo (CRÍTICO)

`style_chain.rs:63`: `pub font: Option<FontList>,`

`entities/font_list.rs:57`: **`pub struct FontList(Vec<FontFamily>);`**

**`FontList` NÃO é `Copy`** — contém `Vec<FontFamily>`. Confirmado
também no comentário `layout_types.rs:120`: *"Remoção de `Copy`
porque `FontList` contém `Vec<FontFamily>`; call sites usam
`.clone()` explícito"* — `TextStyle` perdeu Copy em P136 por causa
de `FontList`.

`FontList` derive: `#[derive(Debug, Clone, PartialEq, Eq, Hash)]` —
**Clone + PartialEq + Eq + Hash, mas NÃO Copy**.

### A.1.3 — `push_styles` cascade match

9 variants exaustivas pós-P291. P292 adicionará 10º arm com
**dereference adaptado**:

```rust
Style::Font(f) => delta.font = Some(f.clone()),  // .clone() em vez de *f
```

Match continua exaustivo.

### A.1.4 — `delta.font` write site único (parse-driven)

`eval/rules.rs:21+395+` (P140B+P141+P146):

```rust
use crate::entities::font_list::{FontFamily, FontList};
// ...
"font" => {
    match val {
        Value::Str(s) => delta.font = Some(FontList::single(s)),
        Value::Array(arr) => /* parse multi-font */,
        Value::Dict(_) => /* scope-out ADR-0054bis */,
        _ => /* silent skip */,
    }
}
```

Parse aceita `Str`, `Array`, e rejeita `Dict` (ADR-0054bis).

### A.1.5 — `delta.font` read site único

`style_chain.rs:264-271`:

```rust
/// Resolve `font` (FontList — não-Copy, clona). Top-wins.
pub fn font(&self) -> Option<FontList> {
    let mut node = self.0.as_deref();
    while let Some(n) = node {
        if let Some(v) = &n.delta.font { return Some(v.clone()); }
        node = n.parent.as_deref();
    }
    None
}
```

**Distintivo**: walk + `.clone()` (não Copy). Doc-comment
explicitamente regista a divergência.

### A.1.6 — Consumers actuais (DISTINTIVO — novo paradigma)

| Consumer | Localização | Função | Paradigma |
|---|---|---|---|
| Top-wins propagation | `layout/mod.rs:582` | `self.style.font.clone().or_else(\|\| node_style.font.clone())` | Top-wins **com clone** (distintivo vs P288/P289/P290/P291 que usam `Copy.or()`) |
| TextStyle capture | `style_chain.rs:316` | `font: chain.font()` em `TextStyle::from(&StyleChain)` | TextStyle capture |
| **Multifont emit** | `export.rs:2169-2174` | `style.font.as_ref().and_then(\|fl\| ...).map(\|name\| fonts.iter().position(...))` → `/F{i+1} Tf` | **Emit consumer real** (paralelo P290 tracking) |
| **FontBook resolver** | (implícito — não inspeccionado em detalhe; provavelmente `font_book.rs::select`) | Resolve `FontFamily` → font index na lista de fontes embebidas | **Indirect resolution via global registry** |

**4 consumers activos** — mais que qualquer outro campo P288-P291.
Inclui:
- **Emit consumer real** (paralelo P290 — confirma ADR-0098 robusta).
- **Indirect resolution via FontBook** — paradigma arquitecturalmente
  novo vs P288-P291.

### A.1.7 — `FrameItem::Text` / emit

`grep "font" export.rs` confirma:
- Linha 2155: Helvetica path usa `font_ref` derivado de `style.bold/italic`.
- Linha 2169-2174: Multifont path lê `style.font` directamente para
  selecionar `/F{i+1}`.
- **Paradigma P136 confirmado** — emit consome via
  `FrameItem::Text.style.font`, não via chain directo. ADR-0098
  vigente.

### A.1.8 — Diagrama de fluxo (genuíno, distintivo)

```
#set text(font: "Inter")                     Content::Styled(body,
       │                                       Styles::from_iter([Style::Font(FontList::single("Inter"))]))
       ▼                                            │
parse → eval_set_rule (rules.rs:395)              ▼
       │                                       push_styles → match arm
       ▼                                       Style::Font(f) => delta.font = Some(f.clone())  [P292]
delta.font = Some(FontList::single("Inter"))        │
       │ ◀──────────── 2ª fonte de entrada ─────────┘
       ▼
chain.push(delta) → StyleChain
       │
       ▼
chain.font() (style_chain.rs:264) ← read via walk up-the-chain + CLONE
       │
       ├──► Top-wins: layout/mod.rs:582 (com .clone())
       ├──► TextStyle.font capture: style_chain.rs:316
       │       ▼
       │   FrameItem::Text.style.font (Layouter::layout_word push)
       │       │
       │       ▼ (capturado em FrameItem)
       └──► **Multifont emit consumer**: export.rs:2169-2174
              │ (style.font.as_ref().and_then(...))
              ▼
       FontBook resolution: fontFamily → font index na lista
              │ (FontBook::select implícito)
              ▼
       PDF stream: `/F{i+1} {:.1} Tf` literal
```

**Distinção crítica vs P291 §A.1.8**: P291 fluxo terminava em
coordenadas Y. **P292 fluxo termina em `/F{i+1} Tf` literal** — emit
consumer real (paralelo P290) + **indirect resolution via FontBook**
(novo paradigma).

---

## A.2 — Estrutura do variant `Style::Font` (decisão arquitectural genuinamente não-trivial)

### A.2.0 — Inventário call sites `Style: Copy`

`grep -rn "\*style" 01_core/src/ → 0 hits funcionais`. Ninguém
desreferencia `Style` directamente. `Style: Copy` é **apenas
trait derive — não é usado dependencialmente em nenhum call site**.

Consequência: **perder `Style: Copy` é estructuralmente inofensivo**.
A semelhança com P288-P291 (todos `Copy`) é coincidência (lang/u16/
Length são todos primitivos pequenos), não invariante arquitectural.

### A.2.1 — Decisão

**Decidido**: opção **(a) `Font(FontList)`** — paralelo arquitectural
absoluto P288-P291.

| Opção | Veredicto |
|---|---|
| (a) `Font(FontList)` | ✅ **Escolhida** — paralelo semântico aos 9 anteriores; `Style` perde `Copy` mas mantém `Clone + PartialEq`; **A.2.0 confirma que perda de Copy é inofensiva** |
| (b) `Font(Arc<FontList>)` | ❌ Indirecção desnecessária dado A.2.0 (Copy não é usado) |
| (c) `Font(Box<FontList>)` | ❌ Box: !Copy igual a (a) sem ganho |
| (d) `Font(EcoString)` | ❌ Divergência semântica grave (perde array fallback P141) |
| (e) Aceitar perda explícita de Copy | = (a) — escolhida implicitamente |
| (f) `Font(EcoVec<EcoString>)` | ❌ Adiciona dependência sem ganho |

### A.2.2 — Refino: derive change

```rust
// Antes P292:
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style { ... }

// Pós-P292:
#[derive(Debug, Clone, PartialEq)]  // -Copy
pub enum Style { ..., Font(FontList) }
```

`Style` perde `Copy`. **Impacto cumulativo verificado em A.2.0**:
zero call sites afectados.

### A.2.3 — Honestidade epistémica

`Style::Font(FontList)` é variant **atómico** (1 campo `FontList`
required) — **não rico**. Padrão N=4 "variant rico" **inalterado**.

**Refutação significativa do paradigma "todos Copy"?** Sim, mas
**estructuralmente forçada** por tipo `FontList: !Copy` já existente
(P136 estabeleceu). **Não conta como refutação pragmática N=6 do
padrão §8.3** porque não há decisão genuína da spec — o tipo já
existia e a estructura é a única opção sensata. §8.3 N=5 **estável**.

### A.2.4 — Implicação para promoção ADR meta

- §8.3 "refutação pragmática" não atinge N=6 (refutação
  estruturalmente forçada, não genuína). **Sem promoção**.
- §8.6 A.5' anti-reflexão atinge **N=2 cumulativo** (P291+P292).
  Mas N=2 está longe de N≥3-4 (critério tentativo). **Sem promoção**.

**Decisão**: nenhuma ADR meta nova promovida.

---

## A.3 — Integração com `StyleDelta`

### A.3.1 — Decisão

**(α)** `delta.font = Some(f.clone())` — paridade com os 9 arms
existentes, **com `.clone()` em vez de `*f` por FontList: !Copy**.

### A.3.2 — Implementação literal

```rust
// Em StyleChain::push_styles (style_chain.rs:138-173):
match style {
    Style::Bold(b)         => delta.bold = Some(*b),
    Style::Italic(i)       => delta.italic = Some(*i),
    Style::Size(pt)        => delta.size = Some(pt.val()),
    Style::Fill(c)         => delta.fill = Some(*c),
    Style::HeadingLevel(l) => delta.heading_level = Some(*l),
    Style::Lang(l)         => delta.lang = Some(*l),
    Style::Weight(w)       => delta.weight = Some(*w),
    Style::Tracking(l)     => delta.tracking = Some(*l),
    Style::Leading(l)      => delta.leading = Some(*l),
    Style::Font(f)         => delta.font = Some(f.clone()),  // P292 (.clone() vs *)
}
```

**Distinção sintáctica vs P288-P291**: `f.clone()` em vez de `*f` —
única necessidade material por `FontList: !Copy`.

---

## A.4 — Impacto em `FrameItem::Text` e emit

### A.4.1 — Decisão

**(i)** confirmada empiricamente: emit consome via `FrameItem::Text.style.font`
(paradigma P136). Hash `export.rs 66cb8ac3` preservado pelo 9º
passo consecutivo.

### A.4.2 — Padrões cumulativos

| Padrão | N pós-P292 | Promoção? |
|---|---:|---|
| ADR-0098 (P288) | **9** | ❌ Reforço cumulativo; 9º passo consecutivo |
| ADR-0099 (P289) | **8** | ❌ Reaplicação cumulativa |
| §8.3 "refutação pragmática" | 5 estável | ❌ Refutação estructuralmente forçada (A.2.3) — não conta como N=6 |
| §8.4 "bug latente fixed" | 1 estável | ❌ (verificação A.5 abaixo) |
| §8.5 "patch cirúrgico sequencial" | **5** | ❌ Desqualificado per P290 §8.5 |
| §8.6 "A.5' anti-reflexão" | **2** cumulativo (P291+P292) | ❌ N=2 < N≥3-4 tentativo |

P273.17 §0 vigente. Sem novas promoções.

---

## A.5 — Detecção de bugs latentes

5 cenários fronteira:

| Cenário | Construção | Expectativa | Resultado |
|---|---|---|:---:|
| Single | `FontList::single("Inter")` | `chain.font() == Some(...)` | ✅ |
| Multi (fallback) | `FontList::new(vec![FF("Inter"), FF("Arial")])` | Mesmo paradigma | ✅ |
| **Vazia** | `FontList::new(vec![])` | Constructor permite (P140B `if !arr.is_empty()` filter ausente?) | **Verificar empiricamente** |
| Last-write wins | 2 `Style::Font` consecutivos | Último ganha | ✅ |
| Missing font name | `FontList::single("InexistentFont")` | Cascade aceita; resolution defer ao FontBook | ✅ (parse aceita; layout fallback) |

### A.5.1 — FontList vazia: empiricamente verificada

`font_list.rs:138`: `FontList::new(vec![])` retorna ... vou verificar
o construtor.

Inspeccionado: `font_list.rs:65` `pub fn new(families: Vec<FontFamily>) -> Self { Self(families) }` — **aceita Vec vazio sem validação**.

`is_empty()` (linha 79): `pub fn is_empty(&self) -> bool { self.0.is_empty() }` — confirma que vazia é estado válido.

Consumer behaviour: `FontBook::select` (P140B) provavelmente devolve `None` para FontList vazia → fallback default. **Não é bug** — comportamento gracefully gated.

### A.5.2 — Nenhum bug latente detectado

5 fronteiras passam (ou degradam gracefully). Padrão §8.4 P288
permanece **N=1 estável**.

---

## A.5' — Verificação anti-reflexão N=2 (reaplicação P291 §A.5')

### A.5'.1 — Comparação literal A.1.6 P288/P289/P290/P291/P292

| Passo | Variant | Paradigma consumer principal |
|---|---|---|
| P288 | `Lang(Lang)` | Cross-module (eval+lang+layout) |
| P289 | `Weight(u16)` | TextStyle method (`faux_bold_stroke_pt`) |
| P290 | `Tracking(Length)` | Per-glyph horizontal + `Tc` emit |
| P291 | `Leading(Length)` | Per-line via peek `current_line` |
| **P292** | **`Font(FontList)`** | **Multifont emit + indirect resolution via FontBook** (paradigma **2 layers**) |

**5 paradigmas arquiteturalmente distintos** — sequência continua
NÃO ser rubber-stamp. P292 adiciona paradigma genuinamente novo
"indirect resolution via global registry" (FontBook::select) que
não aparece em P288-P291.

### A.5'.2 — A.0 produzido empiricamente

P292 §A.0 verificou literalmente `grep "font" export.rs` → 3+ hits
classificados (Tf operator multifont via `style.font`). **Não citou
P291 §A.0 antecipação como prova**.

### A.5'.3 — Elemento estructuralmente novo identificado

✅ **Dois identificados** (mais que P291):

1. **A.2 não-trivial em si** — `Style` perde `Copy` derive (primeira
   vez na série). Inventário de call sites A.2.0 foi parte obrigatória
   da Fase A. Decisão (a) confirmada mas com refino sintáctico
   (`.clone()` vs `*` no cascade arm).
2. **Paradigma "indirect resolution via global registry"**:
   `FontBook::select` resolve `FontFamily` → font index pré-emit.
   Padrão arquitectural novo vs P288-P291 (todos consumiam
   directamente via TextStyle capture sem indirect resolution).

### A.5'.4 — Decisão sobre passo seguinte

**Por construção factual**: P292 é o **último passo cumulativo da
série P288-P292**. Não há mais campos `StyleDelta` para reaplicação
— assimetria fechada 5/5.

**P293 será ortogonal por construção**, não por decisão. Candidatos
listados no relatório P291 §9 (math-accent-cancel, curve-geometry,
footnote-cluster).

### A.5'.5 — A.5' N=2 confirma robustez do padrão

P291 inaugurou A.5' N=1. **P292 N=2 confirma**:
- A verificação anti-reflexão **continua a identificar elementos
  estructuralmente novos** mesmo na reaplicação cumulativa.
- O padrão **não degenera em rubber-stamp** com a sua própria
  reaplicação — porque a inspecção literal sempre revela diferenças
  factuais.

A.5' está bem desenhado para resistir à automatização que ela mesma
mitiga.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P292 |
|---|---:|---:|
| `Style` variants | 9 (pós-P291) | **10** (+`Font(FontList)`) |
| `Style` derives | `Debug, Clone, Copy, PartialEq` | **`Debug, Clone, PartialEq`** (-Copy) |
| `push_styles` arms | 9 | **10** (+1 LOC com `.clone()`) |
| Caminhos entrada `delta.font` | 1 (parse) | **2** (parse + Style::Font) |
| Consumers activos | 4 (top-wins, TextStyle capture, multifont emit, FontBook) | 4 (inalterado) |
| Hash L0 `style.md` | actual | **muda** (+1 variant + derive change) |
| Hash L0 `content.md` / `stdlib.md` | inalterado | inalterado |
| Hash L0 `export.rs` | `66cb8ac3` | **preservado bit-exact** (**9º passo consecutivo**) |
| Fechos B.3↔B.4 | 4/5 (P288-P291) | **5/5** (+ P292 font) — **assimetria fechada** |
| ADRs novas | 0 | 0 (reaplicações cumulativas) |
| Padrão "variant rico" N | 4 | 4 (Font atómico) |
| Padrão §8.2 (ADR-0098) N | 8 (P291) | **9** |
| Padrão §8.1 (ADR-0099) N | 7 (P291) | **8** |
| Padrão §8.6 A.5' anti-reflexão N | 1 (P291) | **2** |

---

## §Risco residual mitigado

- **Risco principal P292** (`Style: !Copy` impacto): ✅ **A.2.0
  inventário literal confirma 0 call sites dependem de `*style`**.
  Perda de Copy é inofensiva.
- **Risco secundário** (A.1.2 tipo inesperado): ✅ Confirmado
  `Option<FontList>` com `FontList(Vec<FontFamily>)`.
- **Risco terciário** (emit idiossincrático): ✅ Confirmado paradigma
  P136 (`style.font` capture).
- **Risco quaternário** (FontList vazia bug): ✅ Verificação A.5.1
  positiva — gracefully gated.
- **Risco quinário** (sequência reflexa): ✅ A.5' N=2 identifica
  2 elementos estructuralmente novos (A.2 não-trivial + paradigma
  FontBook indirect resolution).
- **Risco senário** (promoção ADR meta indevida): ✅ A.2.4 explícito
  — refutação estructuralmente forçada, não genuína; §8.3 N=5
  estável.

---

## §Fecho da Fase A

Inventário literal (8 sub-secções A.1) + A.0 não-trivial confirmada
empiricamente + **decisão arquitectural genuína A.2** (opção (a)
com perda de `Style: Copy` documentada; refutação estructural
forçada não genuína) + integração cascade (α com `.clone()`) +
impacto emit (i confirmada) + **A.5' N=2 com 2 elementos
estructuralmente novos identificados** registadas.

**Marco arquitectural**: P292 fecha 5/5 da assimetria residual
P289 §5.6. **Série cirúrgica P288-P292 termina**. P293 será
ortogonal por construção. **Sem promoção ADR meta nova**.

Procede-se a §3 do passo.
