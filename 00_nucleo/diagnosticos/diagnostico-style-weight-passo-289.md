# Diagnóstico — Fase A do Passo 289 (`P-style-weight-variant`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-289.md`
**Origem**: P288 §7 risco terciário (assimetria residual 4 fields);
P289 = P288.1 (sequência directa).

---

## A.0 — Potencial de reuso ADR-0098 (nova secção obrigatória pós-P288)

Per directiva operacional P288 §9: Fase A de passos futuros deve
citar explicitamente potencial de reuso. **Esta é a primeira
aplicação prática da ADR-0098** desde a sua formalização em P288.

### A.0.1 — Verificação empírica de aderência

```
$ grep -n "weight\|Weight" 03_infra/src/export.rs
(zero hits)
```

**Zero hits funcionais** em `export.rs` para `weight`/`Weight`.
Confirma aderência ADR-0098: feature `weight` é consumida em
runtime layout (faux-bold P139) sem persistir em emit estructural.

### A.0.2 — Expectativa de hash

Hash `export.rs` esperado **preservado bit-exact** pelo 6º passo
consecutivo (P282+P285+P286+P287+P288+**P289**). Validação final
deve confirmar `66cb8ac3` inalterado.

### A.0.3 — Citação literal ADR-0098

> "Qualquer feature nova que possa ser implementada reutilizando
> primitivas L1/L3 pré-existentes (`FrameItem` variants, emit
> helpers) **deve fazê-lo** em vez de criar emit ad-hoc."

P289 reusa o caminho consumer existente
(`chain.weight()` → `layout/mod.rs:578` `self.style.weight.or(...)`
→ `faux_bold_stroke_pt` em `layout_types.rs:158-164`). **Reuso
estructural absoluto** — sem criar emit, helper ou consumer novo.

---

## A.1 — Inventário literal do caminho actual `weight`

### A.1.1 — `Style` enum pós-P288

`entities/style.rs:25-44` confirma 6 variants:
`Bold(bool)` / `Italic(bool)` / `Size(Pt)` / `Fill(Color)` /
`HeadingLevel(u8)` / `Lang(Lang)` (P288).

P289 adicionará 7º variant `Weight(u16)`.

### A.1.2 — `StyleDelta.weight` tipo

`entities/style_chain.rs:42` confirma literalmente:

```rust
/// Peso da fonte (Passo 126, ADR-0038). Valor raw `u16` (CSS/OpenType
/// 0-1000). Capturado pelo eval mas ainda inerte em layout.
pub weight: Option<u16>,
```

**Tipo raw `u16`** (não `FontWeight` wrapper). Range 0-1000
documentado (CSS/OpenType paridade vanilla). **`FontWeight` é tipo
de helper usado no parse para canonicalizar nomes simbólicos**
(`bold`/`thin`/etc.) via `FontWeight::from_name(s) → u16`, mas o
storage final é raw `u16`.

### A.1.3 — `push_styles` cascade match

`style_chain.rs:134-146` match exaustivo sobre 6 variants pós-P288.
**P289 adicionará 7º arm**:

```rust
Style::Weight(w) => delta.weight = Some(*w),  // P289
```

Match continua exaustivo — compilador detecta omissões.

### A.1.4 — `delta.weight` write site único (parse-driven)

`eval/rules.rs:350-368` (P126/P129):

```rust
"weight" => {
    if let Value::Int(n) = val {
        if let Ok(w) = u16::try_from(n) {
            delta.weight = Some(w);
        }
    } else if let Value::Str(s) = val {
        if let Some(fw) = FontWeight::from_name(s.as_str()) {
            delta.weight = Some(fw.to_number());
        }
    }
}
```

**Único write site até P289**: `#set text(weight: N)` ou
`#set text(weight: "bold")` via parse. Sem caminho via `Styles`
collection (paralelo absoluto P288 `lang` pré-materialização).

### A.1.5 — `delta.weight` read site único

`style_chain.rs:200-204` (`fn weight(&self)`):

```rust
pub fn weight(&self) -> Option<u16> {
    let mut node = self.0.as_deref();
    while let Some(n) = node {
        if let Some(v) = n.delta.weight { return Some(v); }
        node = n.parent.as_deref();
    }
    None
}
```

Walk up-the-chain — paralelo absoluto aos outros accessors.

### A.1.6 — Consumers actuais de `style.weight`

| Consumer | Localização | Função |
|---|---|---|
| Top-wins propagation | `layout/mod.rs:578` | `self.style.weight.or(node_style.weight)` — P136 Fase A DEBT-52 |
| Faux-bold stroke | `layout_types.rs:158-164` (`TextStyle::faux_bold_stroke_pt`) | P139 — `((weight - 400) / 300).max(0.0) * size * k` |
| TextStyle capture | `style_chain.rs:287` | `weight: chain.weight()` em `TextStyle::from(&StyleChain)` |

**3 consumers activos** — todos em layout-time, zero em emit. Padrão
ADR-0098 directamente confirmado (paralelo P288 §A.1.6 para `lang`).

### A.1.7 — `FrameItem::Text` emit

`grep "weight\|Weight" 03_infra/src/export.rs` → **zero hits
funcionais** (A.0.1 confirmou). `TextStyle.weight` (P136) é
dead-code em emit — paridade exacta P144 paradigm para `lang`
(P288 §A.1.7).

### A.1.8 — Diagrama de fluxo

```
#set text(weight: 700)                       Content::Styled(body,
       │                                       Styles::from_iter([Style::Weight(700)]))
       ▼                                            │
parse → eval_set_rule (rules.rs:361)              ▼
       │                                       push_styles → match arm
       ▼                                       Style::Weight(w) => delta.weight = Some(*w)  [P289]
delta.weight = Some(700)                            │
       │ ◀──────────── 2ª fonte de entrada ─────────┘
       ▼
chain.push(delta) → StyleChain
       │
       ▼
chain.weight() (style_chain.rs:200) ← read via walk up-the-chain
       │
       ├──► Top-wins: layout/mod.rs:578 (`self.style.weight.or(node_style.weight)`)
       ├──► Faux-bold: faux_bold_stroke_pt (P139)
       └──► TextStyle.weight (FrameItem::Text — dead-code em emit)
              │
              ▼
       export.rs (emit_text_pdf) → IGNORADO (sem branch weight)
```

**Diagrama idêntico a P288 §A.1.8** — confirma simetria
arquitectural absoluta. `Style::Weight` é a 7ª fonte de entrada
parallelo aos 6 anteriores.

---

## A.2 — Estrutura do variant `Style::Weight`

### A.2.1 — Decisão

**Decidido**: opção **(a)** `Weight(u16)` — paralelo arquitectural
absoluto a `HeadingLevel(u8)` e ao storage actual de
`StyleDelta.weight`.

| Opção | Veredicto |
|---|---|
| (a) `Weight(u16)` | ✅ Escolhida — paralelo `HeadingLevel(u8)`, simétrico ao storage `StyleDelta`; `u16` é `Copy` → `Style` mantém `Copy` |
| (b) `Weight(FontWeight)` | ❌ `FontWeight` é tipo helper de parse (P129); storage final é raw `u16`. Wrapping no variant criaria divergência entre `Style` e `StyleDelta` |
| (c) `Weight(W)` alias | ❌ Sem valor adicional |

### A.2.2 — Sintaxe final

```rust
pub enum Style {
    Bold(bool),
    Italic(bool),
    Size(Pt),
    Fill(Color),
    HeadingLevel(u8),
    Lang(Lang),          // P288
    Weight(u16),         // P289 — paralelo a HeadingLevel(u8); storage raw
}
```

**Style continua `Copy`** porque `u16` é `Copy`.

### A.2.3 — Honestidade epistémica (paralelo P287 §A.2.2 / P288 §A.2.3)

`Style::Weight(u16)` é variant **atómico** (1 campo `u16` required).
**Não qualifica** como "variant rico com `body` + cosméticos
opcionais" (padrão N=4 cumulativo P156G/H/I+P284 sobre `Content`).

Padrão N=4 "variant rico" **inalterado** pelo P289. Registo redundante
mas crítico para evitar contagem incorrecta em passos futuros.

---

## A.3 — Integração com `StyleDelta`

### A.3.1 — Decisão

**Decidido**: opção **(α)** `delta.weight = Some(*w)` — paridade
absoluta aos 6 arms existentes (P288 Lang inclusive).

### A.3.2 — Implementação literal

```rust
// Em StyleChain::push_styles (style_chain.rs:138-147):
match style {
    Style::Bold(b)         => delta.bold = Some(*b),
    Style::Italic(i)       => delta.italic = Some(*i),
    Style::Size(pt)        => delta.size = Some(pt.val()),
    Style::Fill(c)         => delta.fill = Some(*c),
    Style::HeadingLevel(l) => delta.heading_level = Some(*l),
    Style::Lang(l)         => delta.lang = Some(*l),
    Style::Weight(w)       => delta.weight = Some(*w),  // P289
}
```

**+1 LOC no match.** Match continua exaustivo.

### A.3.3 — Convivência com parse-driven path

Last-write wins per LIFO da chain — comportamento determinístico
paralelo absoluto P288 §A.3.3.

---

## A.4 — Impacto em `FrameItem::Text` e emit (aplicação ADR-0098)

### A.4.1 — Análise empírica do gatilho

**Decidido**: opção **(i)** confirmada empiricamente por A.0 + A.1.7.

| Critério | Evidência empírica |
|---|---|
| `export.rs` consulta `weight`? | A.0.1: 0 hits funcionais |
| `FrameItem::Text` precisa novo field `weight`? | **Não** — `TextStyle.weight` já existe (P136) e é dead-code em emit |
| Consumer faux-bold P139 já lê `chain.weight()`? | **Sim** — A.1.6 lista 3 consumers activos |
| Reflectors em `export.rs`? | **Não** — A.0.1 zero hits |

### A.4.2 — Aderência ADR-0098: hash `export.rs` preservado

P289 cita ADR-0098 §"Decisão": reutiliza primitivas L1 (`chain.weight()`
+ consumer `faux_bold_stroke_pt`) sem criar emit/helper/consumer novo.

**Hash `export.rs` preservado bit-exact pelo 6º passo consecutivo**:

| Passo | Aplicação ADR-0098 |
|---|---|
| N=1: P282 | Origem auditoria empírica (pré-formalização) |
| N=2: P285 | `FrameItem::Line.color` simétrico via helper único |
| N=3: P286 | Reuso `FrameItem::Line` sem modificação |
| N=4: P287 | Consumer reusa `Content::Text` → `FrameItem::Text` |
| N=5: P288 | `Style::Lang` extende parse sem tocar emit (formalização ADR-0098) |
| **N=6: P289** | `Style::Weight` aplica ADR-0098 directamente (primeira aplicação pós-formalização) |

**P289 é a primeira aplicação directa de ADR-0098 como invariante
testável** — confirma robustez da formalização. Quebra dispararia
investigação obrigatória; preservação confirma o padrão como
operacional vigente.

### A.4.3 — Gatilho N=5 padrão §8.2 "activação posterior de feature graded"

5 aplicações cumulativas do padrão §8.2 P288:

| N | Passo | Citação |
|:---:|---|---|
| 1 | P285 §8.3 | `stroke` parseado em P284 → activo via `FrameItem::Line.color` |
| 2 | P286 §8.1 | Consumer P284 single-line → wrap-aware via `decoration_lines_collector` |
| 3 | P287 §8.2 | Feature **ausente** (smartquote stdlib) → materializada paralela a markup pré-existente |
| 4 | P288 §8.2 | `Style::Lang` parseado-mas-inerte (variant ausente apesar de `StyleDelta.lang` activo) → 2ª fonte de entrada |
| **5** | **P289** | `Style::Weight` parseado-mas-inerte (variant ausente apesar de `StyleDelta.weight` activo e `faux_bold` consumer P139) → 2ª fonte de entrada |

**Limiar histórico N=5 atingido** — paralelo absoluto a ADR-0098.
Padrão §8.2 candidato a formalização como **ADR-0099** "Activação
posterior de feature graded como padrão de pendência".

### A.4.4 — Decisão promoção ADR meta (per spec §5 "apenas uma por passo")

Per spec §5 + §7 risco quinário: **apenas uma ADR meta por passo**.
Padrões em limiar simultaneamente neste passo:

| Padrão | N atual | Promoção condicional? |
|---|---:|---|
| §8.2 "activação posterior" | **5** (P285+P286+P287+P288+P289) | ✅ Dispara empiricamente |
| §8.3 "refutação pragmática spec" | 5 (P285+P286×2+P287+P288) | Estável pós-P288; P289 não adiciona refutação significativa |
| §8.4 "bug latente fixed durante materialização" | 1 (P288 NBSP) | Não atinge limiar |

**Decisão**: promover **ADR-0099** "Activação posterior de feature
graded como padrão de pendência". §8.3 fica para passo próprio onde
o padrão atingir refutação genuína nova (P289 segue convenções P288
sem refutação adicional significativa — §A.2 (a) e §A.3 (α) são
defaults straight da spec).

---

## A.5 — Detecção de bugs latentes (padrão P288 §8.4)

Per padrão emergente P288 §8.4 N=1 inaugural: features dependentes
ganham testes implícitos quando primitivas prévias ganham forma de
teste. P289 adiciona testes-fronteira para `Style::Weight`:

| Cenário | Valor | Expectativa |
|---|---:|---|
| Thin | 100 | `chain.weight() == Some(100)` |
| Regular | 400 | (default vanilla; consumer P139 retorna stroke=0) |
| Bold | 700 | (default bold vanilla; consumer P139 stroke > 0) |
| Black | 900 | `chain.weight() == Some(900)` |
| Non-canonical | 450 | Aceite (vanilla preserva); consumer P139 calcula stroke proporcional |

### A.5.1 — Bug latente detectado?

Antes de materializar, não há sinal específico. Mas o padrão P288
§8.4 estabelece que **a activação deve revelá-lo se existir**.
Verificação durante materialização — registar em §5.3 do relatório
se bug for descoberto.

### A.5.2 — Plano de tests fronteira

4 testes injection (paralelo absoluto P288):
- `p289_style_weight_variant_basico` — ctor + PartialEq.
- `p289_push_styles_weight_projecta_no_delta` — cascade arm.
- `p289_styled_weight_injetado_chain_le_corretamente` — Styled
  wrapping + `chain.weight()`.
- `p289_weight_last_write_wins` — 2 `Style::Weight` consecutivos.

3 testes fronteira (per A.5 cenários):
- `p289_weight_thin_100_propaga` — Style::Weight(100) lido como tal.
- `p289_weight_black_900_propaga` — idem 900.
- `p289_weight_non_canonical_450_aceite` — paridade vanilla.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P289 |
|---|---:|---:|
| `Style` variants | 6 (pós-P288) | **7** (+`Weight(u16)`) |
| `push_styles` arms | 6 | **7** (+1 LOC) |
| Caminhos de entrada para `delta.weight` | 1 (parse) | **2** (parse + Style::Weight) |
| Consumers activos de `chain.weight()` | 3 | 3 (inalterado) |
| Hash L0 `style.md` | actual | **muda** (+1 variant) |
| Hash L0 `content.md` | inalterado | inalterado |
| Hash L0 `stdlib.md` | inalterado | inalterado |
| Hash L0 `export.rs` | `66cb8ac3` | **preservado bit-exact** (6º passo consecutivo) |
| Fechos de assimetria B.3↔B.4 | 1/5 (P288 lang) | **2/5** (P288 lang + P289 weight) |
| Padrão §8.2 "activação posterior" N | 4 (P288) | **5** (limiar — promoção ADR-0099 dispara) |
| Padrão "variant rico" N | 4 | 4 (Weight atómico, não rico) |
| ADRs meta cumulativas | 1 (ADR-0098) | **2** (ADR-0098 + **ADR-0099**) |

---

## §Risco residual mitigado

- **Risco principal** (A.4 → ii): refutado por A.0.1 + A.1.7 zero hits.
- **Risco secundário** (assimetria oculta nos 3 restantes): registado
  como passos próprios (P289.1-3 candidatos: tracking, leading, font).
- **Risco terciário** (promoção indevida): empiricamente confirmado
  N=5 (5 citações documentadas em A.4.3); legítimo per ADR-0065.
- **Risco quaternário** (bug latente A.5): verificação prevista
  durante materialização; registar em §5.3 do relatório se descoberto.
- **Risco quinário** (2 ADRs meta simultâneas): A.4.4 escolheu §8.2
  (mais evidência empírica directa neste passo); §8.3 adiado.

---

## §Fecho da Fase A

Inventário literal (8 sub-secções A.1) + decisão variant (a) +
integração cascade (α) + **confirmação empírica ADR-0098 + N=5
§8.2** registadas. **ADR-0099 promovida em P289** com 5 citações
cumulativas formalizadas. **Apenas uma ADR meta neste passo**
(§8.3 adiado).

Procede-se a §3 do passo + materialização da ADR-0099.
