# Diagnóstico — Fase A do Passo 295 (`P-footnote-cluster`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-295.md`
**Origem**: P282 §6 rank #7 (M); P294 §10 candidato; Tabela A.6
linha 178 (ausente); Tabela C linha 387 (bloqueador histórico).
**Tipo**: **Cluster M ortogonal genuíno** — 1º pós-série
cumulativa P288-P292 + extensões P293-P294. Magnitude reset.
**Hipótese adoptada**: **HE Fase 1 (marker only)** — variant
minimal `(a)` + stdlib + Layouter marker `[N]` superscript. Fase
2 (nota rodapé) + Fase 3 (overflow multi-página) ficam como
sub-passos P295.1/P295.2 candidatos.

---

## A.0.0 — Verificação literal de bloqueador histórico (N=3 §8.7')

### A.0.0.1 — Tabela C linha 387 vs A.9 linha 228 (contradição)

**Tabela C linha 387**: regista bloqueador duplo
`Content::Footnote` + `locate runtime` (ADR-0017 adiada).
**Tabela A.9 linha 228**: sugere `here()`/`locate()`
implementado P208B+C (M9c).

**Verificação literal**:

```text
grep "fn query\|fn page\|fn flat_counter_at\|fn formatted_counter_at"
01_core/src/entities/introspector.rs
```

Resultados (sumário):
- `query(selector)` ✅ funcional desde P175 (M9 sub-passo 5).
- `flat_counter_at(key, location)` ✅ desde P185B (M9).
- `formatted_counter_at(key, location)` ✅ desde P177 (M9 sub-7).
- `page(location)` / `pages(location)` ✅ desde P207D (M9c).
- `bib_number_for_key(key)` ✅ desde P181F (M9).

**Conclusão**: Tabela C linha 387 está **factualmente
desactualizada** quanto à segunda parte. `locate` runtime
infrastructure **funcional**.

**Bloqueador restante (real)**: ausência de `Content::Footnote`
variant — único bloqueador residual.

### A.0.0.2 — Refutação significativa registada (§8.3 N=7 candidato)

P293 §7.3 refutou §8.3 N=6 candidato anterior em favor de §8.7'
inaugural. P294 §7.1 manteve §8.3 N=6 candidato genuíno mas
adiou-o per P273.17 §0.

**P295 dispara potencial N=7** porque a refutação **é factual**
(documento Tabela C registado mas inconsistente com Tabela A.9
e implementação concreta). **Mas magnitude da refutação P295 é
menor** que P294 (corrige uma linha de tabela vs invalida toda
a estrutura de uma spec).

**Decisão de não promover §8.3 N=7 em P295**:
- Refutação é factual mas trivial (linha desactualizada de uma
  tabela administrativa).
- §8.7' N=3 reaplicação genuína tem **prioridade metodológica
  maior** (template inaugural P293 precisa consolidação).
- P273.17 §0: uma ADR meta por passo.
- §8.3 N=7 candidato preservado para P296+ se reaplicação for
  significativa.

### A.0.0.3 — Hipótese decidida HE Fase 1 marker only

Spec §A.0.0 lista 5 hipóteses (HA cluster completo; HB Fase 1;
HC numeração estática; HD Counter reuso; HE multi-fase).

**Decisão HE Fase 1 marker only**:
1. Variant `Content::Footnote { body: Box<Content> }` minimal
   (A.2 → opção (a)). Sem campo `numbering` cosmético — padrão
   "variant rico" N=4 cumulativo **inalterado** (não qualificar
   gratuitamente).
2. `native_footnote(body)` stdlib em `structural.rs` (paralelo
   `native_cite` P159A).
3. Layouter consumer **Fase 1 only**: produz `[N]` superscript
   inline; body **armazenado mas não renderizado** nesta fase.
4. Numeração: walker counter simples interno ao Layouter (sem
   Introspector/Counter machinery — magnitude reduzida).
5. Fase 2 (nota rodapé) + Fase 3 (overflow) → P295.1/P295.2
   sub-passos candidatos registados em §9.

**Razão**: Fase 2 requer 2-pass layout (medir corpo footnote
antes de definir altura page) — magnitude L isoladamente. P295
deve estabelecer **variant + path de entrada** primeiro.

### A.0.0.4 — Honestidade epistémica

A.0.0 N=3 reaplica genuinamente template §8.7'. Refutação
factual modesta vs P293 (hipótese não-listada) ou P294 (toda
estrutura invalidada). **§8.7' N=3 candidato a promoção** mas
adiado per P273.17 §0 — preferência por consolidação adicional.

---

## A.0 — Reuso ADR-0098 (preservação esperada)

| Verificação | Esperado pós-P295 |
|---|---|
| Hash `export.rs` | `66cb8ac3` preservado bit-exact (**12º passo consecutivo**) |
| `FrameItem::Text` precisa novo field? | **Não** — marker `[N]` emite como `FrameItem::Text` standard |
| `FrameItem` variant novo? | **Não** (Fase 1 marker only) |
| ADR-0098 vigente? | ✅ N=12 cumulativo |

Fase 2 (nota rodapé) **poderá** exigir novo `FrameItem::Footnote`
variant — fora do scope P295.

---

## A.1 — Inventário literal

### A.1.1 — `Content` enum pós-P293/P294

Match `pub enum Content` em `01_core/src/entities/content.rs:42`.
N variants confirmados via inspecção literal (P294 não adicionou
variant). `Content::Footnote` **confirmado ausente** via
`grep "footnote\|Footnote" 01_core/src/entities/content.rs` →
0 hits.

### A.1.2 — `FrameItem` enum

`Text`, `Glyph`, `Image`, `Line`, `Shape`, `Group`. **Footnote
marker emite como `FrameItem::Text`** (Fase 1) — sem variant
novo necessário.

### A.1.3 — Match arms exhaustive sobre `Content`

Para `Content::Footnote { body }` defesa compilador exige arms
em:

| Local | Linha | Operação |
|---|---|---|
| `content.rs:1336+` | `is_empty()` | `body.is_empty()` |
| `content.rs:1556+` | `plain_text()` | "[N] " + `body.plain_text()` |
| `content.rs:1729+` | `PartialEq` | `body == body` |
| `content.rs:2200+` | `map_content()` | recurse em body |
| `content.rs:2513+` | `map_text()` | recurse em body |
| `layout/mod.rs` | `layout_content` | marker `[N]` superscript |
| `walker/from_tags.rs` (?) | possivelmente | dependendo se Tagged |
| Hash impl (?) | possivelmente | dependendo se Content: Hash |

Defesa compilador identificará todos os sítios em fase de
implementação.

### A.1.4 — Vanilla `FootnoteElem`

`lab/.../model/footnote.rs:62-86`:

```rust
#[elem(scope, Locatable, Tagged, Count)]
pub struct FootnoteElem {
    #[default(Numbering::Pattern("1"))]
    pub numbering: Numbering,
    #[required]
    pub body: FootnoteBody,
}

pub enum FootnoteBody {
    Content(Content),
    Reference(Label),
}
```

**2 fields vanilla**: `numbering` (default `"1"`) + `body`
(required). `FootnoteBody` é union: corpo directo OU referência
a outra footnote.

**Cristalino P295 simplifications per ADR-0054 graded**:
- `numbering` field **scope-out** (cosmético; numeração default
  `"1"` arabic implícita).
- `FootnoteBody::Reference(Label)` **scope-out** (multi-ref
  footnotes — frente futura P295.X).
- Apenas `body: Box<Content>` (paridade `FootnoteBody::Content`).

### A.1.5 — Counter machinery (P60)

Counter L1 em `01_core/src/entities/counter*.rs` + walker em
P175+. **Reusável** para footnote numbering em Fase 2/3 (não em
Fase 1 — walker counter simples basta).

### A.1.6 — Consumer Layouter para `cite` (precedente)

`layout/mod.rs:1015`:

```rust
Content::Cite { key, supplement, form } => {
    let entry = self.introspector.bib_entry_for_key(key);
    let text = match (resolved_form, entry) {
        (CitationForm::Normal, _) => {
            self.introspector
                .bib_number_for_key(key)
                .map(|n| format!("[{}]", n))
                .unwrap_or_else(|| format!("[{}]", key))
        }
        ...
    };
    self.layout_content(&Content::text(text));
}
```

**Pattern**: lookup via Introspector → format string →
`layout_content` recursivo. P295 marker reusa pattern simplificado
(walker counter em vez de Introspector lookup).

### A.1.7 — Emit consumers

`export.rs` zero hits em footnote/Footnote — emit é agnóstico.
Marker chega como `FrameItem::Text` standard.

### A.1.8 — Diagrama de fluxo P295 Fase 1

```
#footnote[corpo]
      │
      ▼
parse → eval_call (native_footnote)
      │
      ▼
Content::Footnote { body: Box<corpo> }
      │
      ▼
Layouter::layout_content (Footnote arm)         [P295 NOVA]
      ├── counter += 1                          [walker simples]
      ├── marker = format!("[{}]", counter)
      └── self.layout_content(&Content::text(marker))
      │
      ▼
FrameItem::Text { pos, text: "[1]", style }
      │
      ▼
export.rs emit standard — INALTERADO
      │
      ▼
PDF "[1]" visível inline
```

**Body é armazenado mas não renderizado** em Fase 1 (P295.1
futuro renderiza no rodapé).

---

## A.2 — Estrutura variant (decisão (a) minimal)

**Decisão A.2 → opção (a)**: `Footnote { body: Box<Content> }`.

**Razões**:
- Não qualificar gratuitamente padrão "variant rico" N=5
  (P273.17 §0 anti-padrão).
- `numbering` cosmético é divergência consciente per ADR-0054
  graded.
- `FootnoteBody::Reference` scope-out per ADR-0054 graded.

**Padrão "variant rico" N=4 preservado inalterado**.

---

## A.3 — Numeração (walker counter, sem Introspector)

**Decisão A.3 → walker counter interno**. Layouter tem state
`footnote_counter: u32` (default 0). Em cada arm `Content::Footnote`:
- `counter += 1`.
- marker = `format!("[{}]", counter)`.

Magnitude reduzida vs Counter/Introspector full machinery.
Trade-off: footnote counter não reseta cross-page nem cross-document.
Aceitável para Fase 1.

---

## A.4 — Layouter consumer (Fase 1 only)

**Decisão A.4 → marker only**.

**Pseudo-code**:

```rust
Content::Footnote { body: _ } => {
    self.footnote_counter += 1;
    let n = self.footnote_counter;
    let marker = format!("[{}]", n);
    self.layout_content(&Content::text(marker));
    // body armazenado mas não renderizado em Fase 1 — P295.1
    // (nota rodapé) renderizará via 2-pass layout.
}
```

Cenários **fora do scope P295**:
- Renderização body no rodapé.
- Reset counter cross-page.
- Overflow multi-página.
- Reference Footnote (`footnote(<label>)`).

---

## A.5 — Detecção de bugs latentes

6 cenários fronteira:

| Cenário | Resultado esperado P295 |
|---|---|
| Footnote vazia `#footnote[]` | marker `[N]`; body vazio armazenado |
| Footnote nested `#footnote[#footnote[a]]` | 2 markers `[N]`+`[N+1]`; body inner armazenado |
| Footnote no início do doc | counter arranca em 1 |
| Footnote em math context `$x_#footnote[a]$` | marker emite em math — paridade Cite/Quote em math; sem regressão |
| Footnote dentro de figure caption | marker emite; counter incrementa coerente |
| Footnote em show-rule (loop infinito?) | show-rules sobre footnote não materializados; sem risco P295 |

### A.5.1 — Nenhum bug latente esperado

Walker counter simples; sem state cross-cutting. Padrão §8.4 N=1
estável.

---

## A.5' — Anti-reflexão N=5 cumulativo §8.6

### A.5'.1 — Comparação A.1.6 P288-P295

| Passo | Tipo | Paradigma consumer |
|---|---|---|
| P288 lang | cumulativo | Cross-module |
| P289 weight | cumulativo | TextStyle method |
| P290 tracking | cumulativo | Per-glyph + Tc emit |
| P291 leading | cumulativo | Per-line peek |
| P292 font | cumulativo | FontBook indirect |
| P293 cubic | ortogonal | Variant inerte activação |
| P294 quadratic | ortogonal | Transform-on-build |
| **P295 footnote** | **ortogonal M** | **Walker counter + marker inline (Fase 1)** |

**8 paradigmas arquiteturalmente distintos** em 8 passos. P295
é o **1º cluster M** pós-série; magnitude reset.

### A.5'.2 — A.0.0 N=3 reaplica template §8.7'

P293 inaugurou (H6 não-listada). P294 reaplicou com refutação
maior (toda estrutura invalidada). P295 reaplica com refutação
**factual modesta** (linha de tabela administrativa). Tendência:
A.0.0 é robusto mas **genuinidade da refutação diminui** com
cumulativo — sinal de **risco de "sequência reflexa"** §7
septenário.

### A.5'.3 — Elementos estructuralmente novos

5 elementos identificados:

1. **Cluster M magnitude** — 1º pós-série cumulativa cirúrgica.
2. **Refutação factual de bloqueador histórico** (linha 387 vs
   228).
3. **Walker counter Layouter** — paradigma novo (não usado em
   P288-P294).
4. **Marker only (Fase 1)** — decisão honesta de sub-passar
   nota rodapé; documentado em §9.
5. **Reaplicação template §8.7' N=3 limiar** — candidato a
   promoção mas adiado.

### A.5'.4 — Decisão sobre promoção ADR meta

Candidatos disparados:
- **§8.7' N=3** — atinge limiar tentativo N≥3.
- **§8.3 N=7** — candidato genuíno mas refutação trivial.
- **§8.6 N=5** — limiar passado mas anti-padrão adia.
- **"variant rico" N=4** — preservado inalterado.

**Decisão**: **0 ADRs meta novas**. §8.7' N=3 candidato genuíno
**adiado** porque:
- A.0.0 P295 refutação é factual mas magnitude pequena.
- P273.17 §0: uma ADR meta por passo.
- Próxima reaplicação A.0.0 (P296+) com refutação significativa
  → promoção §8.7' N=4 mais robusta.

Anti-padrão "sequência reflexa" §7 septenário: A.0.0 N=3
consecutivo pode degenerar em rubber-stamp se refutações são
sempre factuais-modestas. **P295 documenta o risco**; P296+
vigilar genuinidade.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P295 |
|---|---:|---:|
| `Content` variants | 64 | **65** (+1 Footnote) |
| `FrameItem` variants | 6 | 6 (inalterado) |
| Stdlib funções `model` | N | N+1 (+`native_footnote`) |
| Hash L0 `content.md` | actual | **muda** (+1 variant) |
| Hash L0 `stdlib.md` | actual | inalterado (política única) |
| Hash L0 `export.md` | `31a37c57` | inalterado |
| Hash `export.rs` | `66cb8ac3` | **preservado bit-exact** (**12º passo**) |
| Padrão §8.6 A.5' N | 4 | **5** (P291+P292+P293+P294+P295) |
| Padrão §8.7' A.0.0 N | 2 | **3** (reaplica) |
| Padrão §8.3 N | 6 candidato adiado | 7 candidato adiado |
| Padrão "variant rico" N | 4 | 4 (inalterado — A.2 → (a)) |
| ADRs novas | 0 | 0 |

---

## §Risco residual mitigado

- **Risco principal** (bloqueador histórico): ✅ refutado A.0.0.1.
- **Risco secundário** (Fase 2 complexidade): ✅ scope-out
  explícito em A.4; sub-passos registados.
- **Risco terciário** (qualifica N=5 "variant rico"): ✅
  refutado A.2 → (a) minimal.
- **Risco quaternário** (promoção apressada §8.3): ✅ adiado.
- **Risco quinário** (paradigma 2-fase dispara §8.X): ✅
  Fase 1 é single-flow simples.
- **Risco senário** (cluster M bugs adjacentes): ⚖ verificado
  via testes regressão; A.5 nenhum bug latente.
- **Risco septenário** (sequência reflexa A.0.0): ⚖ documentado
  em A.5'.2; vigilância P296+.

---

## §Fecho da Fase A

Inventário literal completo + **A.0.0 N=3 reaplica §8.7' com
refutação factual modesta** + decisão **HE Fase 1 marker only**
+ A.2 → (a) minimal preservando "variant rico" N=4 + A.4
walker counter simples + A.5 sem bugs + **A.5' N=5 cumulativo
com 5 elementos novos**. **0 ADRs meta novas** — §8.7' N=3,
§8.3 N=7 ambos adiados per P273.17 §0.

**MARCO P295**:
- **1º cluster M pós-série cumulativa cirúrgica** P288-P294 —
  magnitude reset.
- **A.0.0 N=3** com refutação factual (Tabela C 387 vs A.9 228).
- **Hash `export.rs` preservado pelo 12º passo consecutivo**
  (P282→P295) — ADR-0098 robusta sobre 12 features distintas.
- **Sub-passos registados** P295.1 (nota rodapé) + P295.2
  (overflow) — magnitude L+M futuras.
- **Risco "sequência reflexa" §7 septenário documentado** —
  vigilância A.0.0 P296+.

Procede-se a §3 da spec (com plano HE Fase 1 marker only).
