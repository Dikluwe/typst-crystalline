# Passo 289 — Relatório consolidado

**Tema**: Materialização da frente `P-style-weight-variant` —
extensão simétrica do enum `Style` com variant `Weight(u16)`,
paralela arquitectural a `Style::Lang(Lang)` P288. Fecha 1/4 da
assimetria residual P288 §7 risco terciário. **Gatilho N=5 do
padrão §8.2 "Activação posterior de feature graded" dispara
empiricamente → ADR-0099 promovida** em paralelo a ADR-0098 (P288).
**Primeira aplicação prática directa de ADR-0098** como invariante
testável.

**Data**: 2026-05-19
**Branch**: Tekt
**Magnitude**: XS-S (modificação cirúrgica: +1 variant atómico +1
arm em `push_styles` +1 LOC; **+1 ADR meta promovida** com 5
aplicações cumulativas; +9 testes; **6º passo consecutivo** a
preservar hash `export.rs`).

---

## §1 — Validação contra spec (critérios §4)

| Critério §4 | Estado |
|---|---|
| `cargo test --workspace` verde | ✅ **2 763** testes (baseline P288: 2 755 → **+8**) |
| Delta esperado ~+6-10 | ✅ +8 (dentro do range) |
| `crystalline-lint` zero violations | ✅ Confirmado |
| Hash L0 `style.md` muda (+1 variant em B.3) | ✅ `b4de54d0 → 4cb5f241` (`style.rs → 87396557`) |
| Hash L0 `content.md` preserved | ✅ Preservado |
| Hash L0 `stdlib.md` preserved | ✅ Preservado |
| Hash L0 `export.rs` **condicional** preserved se A.4 → (i) | ✅ **Preservado `66cb8ac3` pelo 6º passo consecutivo** — confirma ADR-0098 robusta como invariante operacional vigente |
| **Regressão bit-exact validada** — P139 faux-bold pré-P289 | ✅ 2 755 testes P288 preservados; consumer P139 reusado sem alteração (`p289_weight_faux_bold_stroke_consumer_p139_consome_chain_weight` verifica activação directa) |
| Tabela B.3 actualizada com `Weight(...)` (7º variant) + nota assimetria residual (3 fields restantes) | ✅ Confirmado |
| Tabela B.4 linha 350 com nota cruzada P289 | ✅ Confirmado |
| Diagnóstico A.0+A.1+A.2+A.3+A.4+A.5 produzido | ✅ `diagnostico-style-weight-passo-289.md` (5 secções + 8 sub-secções A.1 + diagrama de fluxo) |
| **Condicional**: ADR-009X promovida se A.4 → (i) confirma 5 aplicações cumulativas §8.2 P288 | ✅ **ADR-0099 promovida** com status `IMPLEMENTADO` |
| Bug latente colateral (se descoberto) registado e fixado | ✅ A.5 — **nenhum bug detectado** (4 cenários fronteira 100/700/900/450 todos passam; consumer P139 robusto) |

**Conformidade**: 13/13 critérios estritos (incluindo o condicional);
0 com observação pragmática. **Spec executada literalmente sem
divergências.**

---

## §2 — Resumo factual

### §2.1 — Variant atómico + cascade arm + ADR-0099

**Antes P289**:
- `Style` enum: 6 variants pós-P288 (Bold/Italic/Size/Fill/HeadingLevel/Lang).
- `StyleDelta.weight: Option<u16>` parseado-mas-inerte do ponto de
  vista de `Styles` collection (escrito apenas via parse-driven
  `eval/rules.rs:350-368` desde P126/P129).
- 3 consumers activos de `chain.weight()`: top-wins propagation P136
  (`layout/mod.rs:578`), faux-bold P139
  (`TextStyle::faux_bold_stroke_pt`), TextStyle capture
  (`style_chain.rs:287`).
- Assimetria Tabela B.3 (6 variants) vs B.4 (10 fields) = 4 fields
  residuais (`weight`/`tracking`/`leading`/`font`).

**Pós-P289**:
- `Style` enum: **7 variants** (+`Weight(u16)`).
- **2ª fonte de entrada** para `delta.weight` via
  `Content::Styled(body, Styles::from_iter([Style::Weight(700)]))`.
- Caminho parse-driven **intacto** — bit-exact preservado.
- Consumer P139 **reusado sem alteração** — primeira aplicação prática
  directa de ADR-0098.
- **ADR-0099 promovida** — formalização do padrão "Activação
  posterior de feature graded" com 5 aplicações cumulativas
  (P285+P286+P287+P288+**P289**).
- Assimetria residual: **3 fields** (`tracking`/`leading`/`font`).

### §2.2 — Cobertura de mudança

| Sítio | Tipo | Mudança |
|---|---|---|
| `01_core/src/entities/style.rs:37-54` | extensão enum | +1 variant `Weight(u16)` com doc-comment ~10 LOC referenciando ADR-0098 |
| `01_core/src/entities/style.rs:141-156` | test extensão | catalog actualizado 6 → 7 variants |
| `01_core/src/entities/style_chain.rs:142-149` | cascade arm | +1 arm `Style::Weight(w) => delta.weight = Some(*w)` (+8 LOC com doc) |
| `00_nucleo/adr/typst-adr-0099-activacao-posterior-feature-graded.md` | ADR meta nova | ~190 LOC formaliza padrão com 5 aplicações + sinergia ADR-0098 |
| `00_nucleo/prompts/entities/style.md` | L0 | +7º variant em B.3 + nota assimetria residual (3 fields) |
| `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | L0 cobertura | B.3 +1 linha; B.4 linha 350 nota P289; footnote ⁷⁵ ~90 LOC |
| `00_nucleo/diagnosticos/diagnostico-style-weight-passo-289.md` | Diagnóstico Fase A | ~250 LOC ficheiro novo (5 secções A.0-A.5) |
| `01_core/src/engine/layout/tests.rs:10355+` | testes P289 | ~120 LOC (9 testes em `p289_style_weight_tests` mod) |

Total: **3 sítios L1 produção + 1 ADR meta + 2 ficheiros L0 documentação + 2 diagnósticos**.

### §2.3 — Cascade arm (`push_styles`)

```rust
match style {
    Style::Bold(b)         => delta.bold = Some(*b),
    Style::Italic(i)       => delta.italic = Some(*i),
    Style::Size(pt)        => delta.size = Some(pt.val()),
    Style::Fill(c)         => delta.fill = Some(*c),
    Style::HeadingLevel(l) => delta.heading_level = Some(*l),
    Style::Lang(l)         => delta.lang = Some(*l),                  // P288
    // P289 — 2ª fonte de entrada para `delta.weight` (paralela à
    // parse-driven em `eval/rules.rs:361/365`). Consumer faux-bold P139
    // (`TextStyle::faux_bold_stroke_pt`) reusado sem alteração —
    // ADR-0098 aderência (hash `export.rs` preservado pelo 6º passo
    // consecutivo). Diagnóstico P289 §A.3 + §A.4.
    Style::Weight(w)       => delta.weight = Some(*w),
}
```

**Match continua exaustivo** — compilador detecta omissões por
construção (defesa cumulativa P288 + P289).

### §2.4 — Aplicação prática directa de ADR-0098

P289 é a **primeira aplicação directa de ADR-0098** desde a sua
formalização em P288 (que foi formalização retrospectiva da
acumulação P282/P285/P286/P287/P288). A Fase A inclui **secção A.0
obrigatória** (nova pós-ADR-0098) que verifica empiricamente:

```
$ grep -n "weight\|Weight" 03_infra/src/export.rs
(zero hits)
```

**Confirmação**: hash `export.rs` preservado bit-exact pelo 6º passo
consecutivo. ADR-0098 funciona como **invariante operacional vigente** —
não apenas documentação retrospectiva, mas critério antecipatório.

---

## §3 — Fase A — decisões registadas (`diagnostico-style-weight-passo-289.md`)

### §3.1 — A.0 (NOVA) — Potencial de reuso ADR-0098

Per directiva operacional P288 §9: Fase A de passos futuros deve
citar explicitamente potencial de reuso. **Esta é a primeira
secção A.0 produzida** no ciclo cristalino.

| Verificação | Evidência |
|---|---|
| `grep weight 03_infra/src/export.rs` | 0 hits funcionais |
| `FrameItem::Text` precisa novo field `weight`? | Não — `TextStyle.weight` já existe (P136) dead-code em emit |
| Hash `export.rs` esperado | **Preservado** bit-exact pelo 6º passo |
| Citação literal ADR-0098 | "Qualquer feature nova que possa ser implementada reutilizando primitivas L1/L3 pré-existentes deve fazê-lo" |

### §3.2 — A.1 inventário literal (8 sub-secções paralelas a P288 §A.1)

| Sub-secção | Achado decisivo |
|---|---|
| A.1.1 — `Style` enum pós-P288 | 6 variants confirmadas (P289 → 7) |
| A.1.2 — `StyleDelta.weight` | `Option<u16>` raw (P126/P129; range 0-1000 CSS/OpenType) |
| A.1.3 — `push_styles` cascade | match exaustivo sobre 6 variants; arm trivial para `Weight(w)` |
| A.1.4 — `delta.weight` write | **Único** site: `eval/rules.rs:350-368` (parse `#set text(weight: ...)` aceita Int ou nome simbólico) |
| A.1.5 — `delta.weight` read | `chain.weight()` walk up-the-chain |
| A.1.6 — Consumers | 3 activos (top-wins P136, faux-bold P139, TextStyle capture) |
| A.1.7 — `FrameItem::Text` emit | **Zero hits** — paridade absoluta `lang` P288 |
| A.1.8 — Diagrama de fluxo | Idêntico a P288 §A.1.8; 7ª fonte de entrada parallel |

### §3.3 — A.2 estrutura variant — opção (a) refinada

**Decidido**: `Weight(u16)` paralelo a `HeadingLevel(u8)`.

Justificações: `u16` é `Copy` → `Style` mantém `Copy`; storage raw
em `StyleDelta` (não `FontWeight` wrapper). Rejeita `Weight(FontWeight)`
(criar wrapper criaria divergência com storage); rejeita alias type.

**Honestidade epistémica §A.2.3** (paralelo P287/P288): variant
**atómico** (1 campo `u16` required) — **não rico**. Padrão N=4
"variant rico com cosméticos opcionais" **inalterado** pelo P289.

### §3.4 — A.3 cascade — opção (α)

`delta.weight = Some(*w)` paralelo absoluto aos 6 arms anteriores.
Last-write wins per LIFO da chain — comportamento determinístico.

### §3.5 — A.4 emit — opção (i) confirma N=5 §8.2

| Critério | Evidência |
|---|---|
| `export.rs` consulta `weight`? | A.0.1: 0 hits |
| `FrameItem::Text` precisa novo field? | Não — `TextStyle.weight` (P136) dead-code em emit |
| Consumer faux-bold P139 já lê `chain.weight()`? | **Sim** — A.1.6 lista 3 consumers activos |
| Reflectors em `export.rs`? | Não |

**Hash `export.rs` preservado pelo 6º passo consecutivo**
(P282+P285+P286+P287+P288+P289).

### §3.6 — Gatilho N=5 padrão §8.2 dispara empiricamente

5 aplicações cumulativas do padrão "Activação posterior de feature
graded como padrão de pendência":

| N | Passo | Citação |
|:---:|---|---|
| 1 | P285 §8.3 | `stroke` em decorações P284 → activo via `FrameItem::Line.color` |
| 2 | P286 §8.1 | Consumer P284 single-line → wrap-aware via `decoration_lines_collector` |
| 3 | P287 §8.2 | Feature ausente (smartquote stdlib) → materializada paralela a markup |
| 4 | P288 §8.2 | `Style::Lang` parseado-mas-inerte → 2ª fonte de entrada |
| **5** | **P289** | `Style::Weight` parseado-mas-inerte → 2ª fonte de entrada |

**Limiar histórico N=5 atingido**. **ADR-0099 promovida** com
status `IMPLEMENTADO`.

### §3.7 — A.4.4 decisão "apenas uma ADR meta por passo" (P273.17 §0)

Padrões em limiar simultaneamente neste passo:

| Padrão | N | Promoção? |
|---|---:|---|
| §8.2 "activação posterior" | 5 | ✅ Dispara empiricamente — ADR-0099 |
| §8.3 "refutação pragmática" | 5 (estável pós-P288) | ⏸ P289 segue defaults straight — refutação não significativa nova; adiado |
| §8.4 "bug latente fixed" | 1 (P288) | Não atinge limiar |

### §3.8 — A.5 detecção de bugs latentes (padrão P288 §8.4)

4 cenários fronteira testados: 100 (thin), 700 (bold), 900 (black),
450 (non-canonical). **Nenhum bug latente detectado** — consumer
P139 `faux_bold_stroke_pt` robusto a todos os valores:

```rust
let factor = ((w as f64 - 400.0) / 300.0).max(0.0);
factor * self.size.val() * k
```

Formula linear `max(0)` cobre fronteiras gracefully.

### §3.9 — Riscos mitigados

| Risco | Status |
|---|---|
| A.4 → ii (novo field FrameItem) | ✅ Refutado por A.0.1 + A.1.7 zero hits |
| Assimetria oculta >1 field | ✅ Acknowledged — 3 fields restantes registados como passos próprios |
| Promoção indevida ADR meta | ✅ Empiricamente confirmada (5 citações documentadas) |
| Bug latente A.5 | ✅ Verificação positiva — nenhum bug encontrado |
| 2 ADRs meta simultâneas | ✅ A.4.4 escolheu §8.2 (mais evidência empírica); §8.3 adiado |

---

## §4 — Testes adicionados (+8)

| Local | Quantidade | Cobertura |
|---|---:|---|
| `entities/style.rs` (mod tests) | 1 | Catalog test 6 → 7 variants (`Style::Weight(700)` incluído) |
| `engine/layout/tests.rs` (`p289_style_weight_tests`) | 8 | Variant ctor + PartialEq; `push_styles` cascade; `Styled` injection + TextStyle propagation; last-write wins; 3 fronteiras (100/900/450); consumer P139 faux-bold stroke > 0 |
| **Total** | **9** (1 entity + 8 layout) | Paralelo absoluto a P288 mas com `+4 fronteiras` para detecção A.5 + `+1 verificação consumer P139` |

**Resultado**: 9/9 verdes (`cargo test --lib p289`). Delta workspace
**+8** (porque o catalog test substitui o anterior P288 6 → 7 mas
representa apenas 1 LOC novo).

---

## §5 — Observações pragmáticas

### §5.1 — Gatilho N=5 §8.2 disparou empiricamente — ADR-0099 promovida

Padrão §8.2 P288 ("Activação posterior de feature graded como padrão
de pendência") atingiu limiar N=5 com 5 citações cumulativas
documentadas:

| N | Passo | Tipo de activação |
|:---:|---|---|
| 1 | P285 | Campo novo em FrameItem (activa stroke parseado) |
| 2 | P286 | Campo opcional em Layouter (activa wrap) |
| 3 | P287 | Variant leaf novo (materializa feature ausente) |
| 4 | P288 | Variant atómico novo (activa parseado-mas-inerte) |
| 5 | P289 | Variant atómico novo (activa parseado-mas-inerte — paralelo absoluto P288) |

**ADR-0099** ("Activação posterior de feature graded como padrão de
pendência") promovida em
`00_nucleo/adr/typst-adr-0099-activacao-posterior-feature-graded.md`
com:
- Definição operacional do padrão.
- 5 aplicações cumulativas detalhadas.
- 3 alternativas consideradas com justificações.
- Consequências imediatas + futuras + sinergia com ADR-0098.
- Status `IMPLEMENTADO` desde P285, formalizado em P289.

**Marco histórico**: **2ª meta-ADR de invariante arquitectural** em
passos consecutivos (P288 → ADR-0098; P289 → ADR-0099). Sinergia
documentada na própria ADR-0099 §"Sinergia com ADR-0098":
- ADR-0098 garante **estrutura de emit** estável (reuso L1/L3
  pré-existente).
- ADR-0099 garante **estrutura de consumer** estável (extensão
  minimalista de caminhos pré-existentes).
- Juntas definem o **paradigma cirúrgico** pós-P281.

### §5.2 — Primeira aplicação prática directa de ADR-0098

Antes de P289, ADR-0098 era documentação retrospectiva (formalização
em P288 da acumulação P282-P288). **P289 é o primeiro passo que aplica
ADR-0098 prospectivamente** — Fase A inclui secção A.0 obrigatória
que verifica empiricamente:

1. `grep weight 03_infra/src/export.rs` → 0 hits funcionais (esperado).
2. `FrameItem::Text` precisa novo field? Não.
3. Hash `export.rs` esperado preservado.

**Confirma robustez**: ADR-0098 funciona como **invariante
operacional vigente**, não apenas pattern retrospectivo. A.0 torna-se
template para Fase A de passos futuros.

### §3 — Hash `export.rs` preservado pelo 6º passo consecutivo

Sequência cumulativa:

| Passo | Razão preservação |
|---|---|
| N=1: P282 | Auditoria empírica refutou 6/6 suspeitas de divergência |
| N=2: P285 | Alteração simétrica via helper único `line_rg_prefix` |
| N=3: P286 | Reuso `FrameItem::Line` sem modificação |
| N=4: P287 | Consumer reusa `Content::Text` → `FrameItem::Text` |
| N=5: P288 | `Style::Lang` extende parse sem tocar emit (formalização ADR-0098) |
| **N=6: P289** | `Style::Weight` aplica ADR-0098 directamente (primeira aplicação prática) |

Cada passo cumulativo **reforça** a invariante. Próximo passo que
preserve `export.rs` é a 7ª aplicação — ADR-0098 ganha mais
robustez.

### §5.4 — Nenhum bug latente colateral detectado (paralelo P288 §8.4 N=1)

P288 inaugurou o padrão "bug latente fixed durante materialização"
(NBSP em consumer SmartQuote). P289 testou cenários fronteira A.5
(100/700/900/450) — **nenhum bug detectado**. Consumer P139
`faux_bold_stroke_pt` é robusto by construction (formula linear
`max(0)`).

Padrão emergente §8.4 P288 **permanece N=1** (não cumula com P289).
Aguardar N≥2 em passo futuro para considerar formalização.

### §5.5 — Honestidade epistémica: variant atómico (não rico)

Padrão N=4 "variant rico com `body` + cosméticos opcionais"
**inalterado** pelo P289 — mesma lógica P287/P288. `Style::Weight(u16)`
é variant atómico paralelo a `HeadingLevel(u8)`.

Registo redundante mas crítico para futuros passos não contarem
incorrectamente N=5 acidental.

### §5.6 — Assimetria residual: 3/4 ainda abertos

P288 § fechou 1 (lang); P289 fecha mais 1 (weight). Restam **3**:

| Field `StyleDelta` | Caminho actual exclusivo | Passo candidato |
|---|---|---|
| `tracking: Option<Length>` | P137 — parse-driven | P289.1 |
| `leading: Option<Length>` | P138 — parse-driven | P289.2 |
| `font: Option<FontList>` | P132B/P140B/P141/P146 — parse-driven | P289.3 |

Cada um vai a passo próprio cirúrgico paralelo a P288/P289. Não-
objectivo §5 deste passo. Próxima oportunidade de reaplicação do
padrão ADR-0099 sem necessidade de nova promoção (limiar já atingido).

---

## §6 — Métricas

| Métrica | Valor |
|---------|-------|
| LOC L1 produção | ~25 (+1 variant 10 LOC com doc; +1 arm cascade 8 LOC com doc; +1 LOC em test catalog) |
| LOC L3 produção | **0** (zero impacto em export.rs — hash preservado pelo 6º passo) |
| LOC L0 modificado | ~350 (`style.md` +20; `cobertura.md` B.3+B.4+footnote⁷⁵ ~90; `diagnostico-style-weight-passo-289.md` ~250 ficheiro novo; **ADR-0099 ~190 ficheiro novo**) |
| Testes adicionados | 9 (1 entity catalog + 8 layout: 4 ctor/cascade/inject/last-write + 3 fronteiras + 1 consumer P139) |
| Testes baseline P288 | 2 755 preserved bit-exact |
| Testes pós-P289 | **2 763** |
| Hash L0 `style.md` | `b4de54d0 → 4cb5f241` (`style.rs → 87396557`) |
| Hash L0 `content.md` | **inalterado** |
| Hash L0 `stdlib.md` | **inalterado** |
| Hash L0 `export.rs` | **`66cb8ac3` preservado** (6º passo consecutivo — confirma ADR-0098 operacional) |
| Lint | zero violations |
| `Style` variants | 6 → **7** (+`Weight(u16)`) |
| Cascade arms `push_styles` | 6 → **7** (+1 LOC) |
| Caminhos de entrada para `delta.weight` | 1 → **2** (parse + Style::Weight) |
| Fechos de assimetria B.3↔B.4 | 1/5 (P288 lang) → **2/5** (+ P289 weight) |
| Pendências resolvidas | **1** (1/4 assimetria residual P288 §7) |
| **ADRs novas** | **+1** (ADR-0099 IMPLEMENTADO) |

---

## §7 — Conformidade Cristalina

- ✅ **ADR-0029 pureza física L1**: `Style::Weight(u16)` é variant
  atómico `Copy`; cascade arm é função pura sem I/O.
- ✅ **ADR-0038 Style enum divergência intencional**: P289 estende
  o enum dentro do mesmo paradigma; vanilla continua vtable.
- ✅ **ADR-0040 `#set text` activation P102**: caminho parse-driven
  preservado intacto; P289 adiciona 2ª fonte sem perturbar 1ª.
- ✅ **ADR-0054 scope graded**: assimetria residual restante (3
  fields) documentada como scope-out **consciente** com passos
  candidatos P289.1-3.
- ✅ **ADR-0057** (P144 hyphenation): paralelo arquitectural
  preservado entre `Style::Lang` (P288) e `Style::Weight` (P289).
- ✅ **ADR-0065 inventariar-primeiro**: Fase A obrigatória produziu
  **5 secções A.0-A.5** (uma a mais que P288). A.0 é template para
  passos futuros.
- ✅ **ADR-0085 diagnóstico imutável**:
  `diagnostico-style-weight-passo-289.md` produzido com 5 secções +
  diagrama de fluxo + métricas + risco residual mitigado.
- ✅ **ADR-0093 meta-metodologia evolução ADRs**: ADR-0099 promovida
  per política incremental (5 aplicações cumulativas documentadas);
  não promoção mecânica.
- ✅ **ADR-0098** (P288): **primeira aplicação prática directa** em
  P289 — confirma invariante operacional vigente. Hash preservado
  pelo 6º passo consecutivo valida testabilidade da ADR.
- ✅ **ADR-0099** (recém-formalizada): gatilho N=5 disparou
  empiricamente — promoção legítima per critério registado em
  ADR-0065 + P273.17 §0.
- ✅ **Anti-padrão over-formalização P273.17 §0**: promoção
  **condicional ao gatilho disparar genuinamente** (confirmado
  empiricamente). **Apenas uma ADR meta por passo** — §8.3
  "refutação pragmática" adiada para passo próprio onde refutação
  significativa nova ocorra.
- ✅ **Honestidade epistémica reforçada §5.5**: N=4 "variant rico"
  **inalterado** (Style::Weight atómico, não rico).
- ✅ **ADR-0098 + ADR-0099 sinergia**: paradigma cirúrgico pós-P281
  formalizado em 2 meta-ADRs complementares.
- ✅ **Bit-exact regression preservada por construção**: caminho
  parse-driven `eval_set_rule` intacto; 2 755 testes pré-P289
  preserved.

---

## §8 — Padrões emergentes (formalização e novos)

### §8.1 — "Activação posterior de feature graded" — **FORMALIZADA EM ADR-0099** (N=5)

P289 **fecha** este padrão emergente promovendo-o a ADR. A partir
deste passo:
- Padrão deixa de ser "emergente" e passa a ser **invariante
  meta-processual perene**.
- Pendências graded são candidatos naturais a activação posterior.
- **Patch arquitectural sucinto** torna-se métrica de aderência
  (activações >100 LOC L1 ou que tocam L3 devem ser examinadas).

5 citações cumulativas registadas na ADR e em §3.6 deste relatório:
P285 + P286 + P287 + P288 + **P289**.

### §8.2 — "Single source of truth como invariante anti-bug" — ADR-0098 confirmada em P289 (N=6 cumulativo)

ADR-0098 foi formalizada em P288 com 5 aplicações cumulativas
(P282-P288). **P289 é a 6ª aplicação cumulativa** — primeira
aplicação prática directa pós-formalização:

| N | Passo | Aplicação |
|:---:|---|---|
| 1 | P282 | Auditoria empírica (origem) |
| 2 | P285 | Alteração simétrica via helper único |
| 3 | P286 | Reuso `FrameItem::Line` sem modificação |
| 4 | P287 | Consumer reusa `Content::Text` |
| 5 | P288 | `Style::Lang` extende parse sem tocar emit (formalização) |
| **6** | **P289** | `Style::Weight` aplica ADR-0098 directamente (primeira aplicação prática) |

ADR-0098 valida-se como **invariante operacional vigente** — não
apenas documentação retrospectiva. Hash `export.rs` preservado pelo
6º passo consecutivo confirma testabilidade.

### §8.3 — "Refutação pragmática de pressuposto da spec via inspecção empírica" — N=5 estável

P287 atingiu N=5; P288 reforçou N=5 com 2 refutações simultâneas
(A.2 e A.3). **P289 não adiciona refutação significativa** — segue
defaults straight da spec (a) e (α). Padrão **permanece N=5
estável**; promoção ADR meta adiada para passo próprio onde
refutação significativa nova ocorra.

### §8.4 — "Bug latente fixed durante materialização de feature dependente" — N=1 estável

P288 inaugurou N=1 (NBSP em consumer SmartQuote). P289 testou 4
cenários fronteira A.5 — **nenhum bug detectado**. Padrão **permanece
N=1 estável**; aguardar N≥2 em passo futuro.

### §8.5 — "Patch cirúrgico sequencial paralelo" — N=2 cumulativo (P288+P289)

**Novo padrão emergente N=2**: P288 e P289 partilham estrutura
arquitectural quase idêntica:
- +1 variant atómico no enum `Style`.
- +1 arm em `push_styles`.
- +1 ADR meta promovida (cada passo).
- Hash `export.rs` preservado.

Característica distintiva: passos cumulativos com **especificações
quase-idênticas** (P288.md e P289.md diferem apenas no campo —
`lang` vs `weight`). Reaplicação candidatas: P289.1 (`tracking`),
P289.2 (`leading`), P289.3 (`font`). Aguardar N≥4 para considerar
formalização (passos próprios cumulativos não bastam — seria
inflação por construção).

---

## §9 — Próximos passos sugeridos (estado pós-P289)

Cobertura agregada estimada: inalterada (~64%). P289 não altera
contagem user-facing (variant arquitectural) mas:
- **Fecha 1/4 da assimetria residual** (2/5 = lang+weight fechados; 3/5 restantes).
- **Formaliza 2ª meta-ADR** consecutiva (ADR-0099 após ADR-0098).
- **Sinergia ADR-0098 + ADR-0099** define paradigma cirúrgico pós-P281.

### Rank 1-3: continuar fecho da assimetria (paralelo P288/P289)

1. **`P-style-tracking-variant`** (XS; P289.1) — adiciona
   `Style::Tracking(Length)` paralelo a `Style::Lang(Lang)` P288 e
   `Style::Weight(u16)` P289. Reaplica ADR-0099 (não promove nova
   meta-ADR — limiar já atingido). Hash `export.rs` esperado
   preservado pelo 7º passo consecutivo.
2. **`P-style-leading-variant`** (XS; P289.2) — idem para
   `leading: Option<Length>`.
3. **`P-style-font-variant`** (XS; P289.3) — idem para
   `font: Option<FontList>`. Possivelmente mais complexo se
   `FontList` requer tipo wrapper.

### Rank 4-6: features médias

4. **`P-math-accent-cancel`** (XS+S; Math 40% → 50%).
5. **`P-curve-geometry`** (S-M; ADR-0078 sub-fase b).
6. **`P-footnote-cluster`** (M; Model 60% → 70%).

### Padrões pós-P289 (operacional)

- **ADR-0098 + ADR-0099 vigentes** — Fase A de passos futuros deve
  incluir secção A.0 (potencial reuso ADR-0098) + cita padrão
  ADR-0099 quando aplicável.
- **Padrão §8.3** "refutação pragmática" N=5 estável — próximo passo
  onde refutação significativa nova ocorra é candidato a ADR meta.
- **Padrão §8.5** "patch cirúrgico sequencial paralelo" N=2 inaugural
  — aguardar N≥4.

---

## §10 — Referências cross-passos

- **P102 ADR-0040** — `#set text(...)` activation; precedente
  parse-driven path.
- **P126 / P129** — `delta.weight` parse aceita Int + nome simbólico
  via `FontWeight::from_name`.
- **P136 Fase A DEBT-52** — Top-wins propagation em
  `layout/mod.rs:578`.
- **P139** — Faux-bold consumer `TextStyle::faux_bold_stroke_pt` —
  reusado sem alteração em P289.
- **P281** — Unificação β-completa stream-builders (origem
  ADR-0098).
- **P282 §1.1** — Auditoria empírica (N=1 cumulativo ADR-0098).
- **P285 §8.2/§8.3** — N=2 ADR-0098 + N=1 ADR-0099.
- **P286 §5.2/§8.1** — N=3 ADR-0098 + N=2 ADR-0099.
- **P287 §5.1/§8.2** — N=4 ADR-0098 + N=3 ADR-0099.
- **P288 §5.1/§8.2** — N=5 ADR-0098 (formalização) + N=4 ADR-0099.
- **P289 §A.4/§5.1** — N=6 ADR-0098 (primeira aplicação prática) +
  **N=5 ADR-0099 (formalização)**.
- **ADR-0026 / ADR-0026-R1** — Enum vs vtable divergência.
- **ADR-0029** — Pureza física L1.
- **ADR-0038** — Style enum divergência intencional vs vanilla
  vtable.
- **ADR-0054** — Scope graded (assimetria residual 3 fields
  documentada como scope-out consciente).
- **ADR-0057** — `text.lang` via `hypher` (precedente paralelo
  P144).
- **ADR-0065** — Inventariar-primeiro + critério N≥5 para promoção
  meta-ADR.
- **ADR-0085** — Diagnóstico imutável (43º consumo: P289 + P288 +
  P287 + P286 + P285 + P284 + P282 + 36 anteriores).
- **ADR-0093** — Meta-metodologia evolução ADRs.
- **ADR-0094** — Meta-operacional specs.
- **ADR-0098** — Single source of truth como invariante anti-bug
  (formalizada P288; primeira aplicação prática P289).
- **ADR-0099 (recém-criada)** — Activação posterior de feature graded
  como padrão de pendência (formalização N=5 em P289).

---

*P289 fecha 1/4 da assimetria residual P288 §7 risco terciário ao
adicionar `Style::Weight(u16)` ao enum `Style` (6 → 7 variants)
com arm correspondente em `StyleChain::push_styles` (paralelo
arquitectural absoluto ao P288 `Style::Lang(Lang)`). 2ª fonte de
entrada para `delta.weight` materializada — caminho parse-driven
`eval_set_rule` (P126/P129) + consumer faux-bold P139 preservados
sem alteração. Hash `export.rs 66cb8ac3` preservado bit-exact pelo
6º passo consecutivo (P282+P285+P286+P287+P288+P289) — **primeira
aplicação prática directa de ADR-0098** confirma invariante
operacional vigente. **Gatilho N=5 padrão §8.2 dispara empiricamente
→ ADR-0099 promovida** com 5 aplicações cumulativas formalizadas
(P285+P286+P287+P288+P289). Marco arquitectural: **2ª meta-ADR em
passos consecutivos** (P288 → ADR-0098; P289 → ADR-0099). Sinergia
ADR-0098 (emit estável) + ADR-0099 (consumer estável) define
paradigma cirúrgico pós-P281. Nenhum bug latente colateral detectado
(A.5 verificou 4 fronteiras 100/700/900/450). 9 testes P289 verdes
(1 entity catalog + 8 layout); baseline 2 755 → 2 763 (+8).
Assimetria residual passa de 4/5 (P288) para **3/5** (`tracking`/
`leading`/`font` continuam abertos como passos próprios candidatos
P289.1-3 não-reservados — reaplicações de ADR-0099 sem necessidade
de nova promoção). Honestidade epistémica preservada: variant
atómico (não rico) — padrão N=4 inalterado.*
