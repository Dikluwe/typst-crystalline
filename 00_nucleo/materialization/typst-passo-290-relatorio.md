# Passo 290 — Relatório consolidado

**Tema**: Materialização da frente `P-style-tracking-variant` —
extensão simétrica do enum `Style` com variant `Tracking(Length)`,
paralela arquitectural a `Style::Lang(Lang)` P288 e
`Style::Weight(u16)` P289. Fecha 1/3 da assimetria residual P289 §5.6.
**Primeira aplicação de ADR-0098 com emit consumer real** (`Tc`
operator P137 em PDF stream) — confirma invariante robusta mesmo
quando emit consome literalmente o campo activado.

**Data**: 2026-05-19
**Branch**: Tekt
**Magnitude**: XS-S (modificação cirúrgica: +1 variant atómico
`Tracking(Length)` +1 arm em `push_styles`; **0 ADRs novas** —
reaplicações de ADR-0098/ADR-0099 sem nova promoção; +11 testes;
**7º passo consecutivo** a preservar hash `export.rs`).

---

## §1 — Validação contra spec (critérios §4)

| Critério §4 | Estado |
|---|---|
| `cargo test --workspace` verde | ✅ **2 773** testes (baseline P289: 2 763 → **+10**) |
| Delta esperado ~+8-12 | ✅ +10 (dentro do range) |
| `crystalline-lint` zero violations | ✅ Confirmado |
| Hash L0 `style.md` muda (+1 variant em B.3) | ✅ `4cb5f241 → 3e40638a` (`style.rs → 661944c0`) |
| Hash L0 `content.md` preserved | ✅ Preservado |
| Hash L0 `stdlib.md` preserved | ✅ Preservado |
| Hash L0 `export.rs` **condicional** preserved se A.4 → (i) | ✅ **`66cb8ac3` preservado bit-exact pelo 7º passo consecutivo** — A.0 não-trivial confirma paradigma `TextStyle` capture (P136); ADR-0098 vigente |
| **Regressão bit-exact validada** — P137 parse-driven | ✅ 2 763 testes P289 preservados; consumer P137 reusado sem alteração (`p290_tracking_consumer_p137_cursor_extra` verifica activação directa) |
| Tabela B.3 actualizada com `Tracking(Length)` (8º variant) + nota assimetria residual (2 fields restantes) | ✅ Confirmado |
| Tabela B.4 linha 351 com nota cruzada P290 | ✅ Confirmado |
| Diagnóstico A.0+A.1+A.2+A.3+A.4+A.5 produzido | ✅ `diagnostico-style-tracking-passo-290.md` (5 secções + 8 sub-secções A.1 + diagrama de fluxo + **A.0 não-trivial**) |
| **Sem promoção ADR meta nova** — ADR-0099 reaplica sem promoção | ✅ Confirmado — P273.17 §0 anti-padrão vigente; ADR-0098/0099 já cobrem |
| Bug latente colateral (se descoberto) registado e fixado | ✅ A.5 — **nenhum bug detectado** (5 cenários fronteira: 0pt/1pt/0.5em/**-0.5pt negativo**/10pt todos passam; consumer P137 robusto) |

**Conformidade**: 13/13 critérios estritos cumpridos. **Spec
executada literalmente sem divergências** — paralelo P289 com
elevação de risco em A.0 mitigada com sucesso.

---

## §2 — Resumo factual

### §2.1 — Variant atómico + cascade arm (sem nova ADR)

**Antes P290**:
- `Style` enum: 7 variants pós-P289
  (Bold/Italic/Size/Fill/HeadingLevel/Lang/Weight).
- `StyleDelta.tracking: Option<Length>` parseado-mas-inerte do ponto
  de vista de `Styles` collection (escrito apenas via parse-driven
  `eval/rules.rs:374` desde P127/P137).
- 4 consumers activos de `chain.tracking()`: top-wins propagation
  P136 (`layout/mod.rs:579`), cursor extra P137 (`cursor.rs:30`),
  TextStyle capture (`style_chain.rs:295`), **emit `Tc` operator P137
  via `FrameItem::Text.style.tracking`** (`export.rs:2139-2146`).
- Assimetria Tabela B.3 (7 variants) vs B.4 (10 fields) = 3 fields
  residuais (`tracking`/`leading`/`font`).
- ADR-0098 + ADR-0099 vigentes desde P288/P289.

**Pós-P290**:
- `Style` enum: **8 variants** (+`Tracking(Length)`).
- **2ª fonte de entrada** para `delta.tracking` via
  `Content::Styled(body, Styles::from_iter([Style::Tracking(Length::em(0.1))]))`.
- Caminho parse-driven **intacto** — bit-exact preservado.
- Consumer P137 (`cursor_extra` + `Tc` operator emit) **reusado sem
  alteração** — **primeira aplicação de ADR-0098 com emit consumer
  real**.
- **0 ADRs novas** — ADR-0099 reaplica (6ª aplicação cumulativa).
- Assimetria residual: **2 fields** (`leading`/`font`).

### §2.2 — Cobertura de mudança

| Sítio | Tipo | Mudança |
|---|---|---|
| `01_core/src/entities/style.rs:21` | import | `Color, Length, Pt` (+`Length`) |
| `01_core/src/entities/style.rs:55-67` | extensão enum | +1 variant `Tracking(Length)` com doc-comment ~13 LOC referenciando paradigma `TextStyle` capture + ADR-0098 |
| `01_core/src/entities/style.rs:153-170` | test extensão | catalog actualizado 7 → 8 variants |
| `01_core/src/entities/style_chain.rs:149-158` | cascade arm | +1 arm `Style::Tracking(l) => delta.tracking = Some(*l)` (+10 LOC com doc emit consumer paradigm) |
| `00_nucleo/prompts/entities/style.md` | L0 | +8º variant em B.3 + nota distinção P290 vs P288/P289 |
| `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | L0 cobertura | B.3 +1 linha; B.4 linha 351 nota P290; footnote ⁷⁶ ~85 LOC |
| `00_nucleo/diagnosticos/diagnostico-style-tracking-passo-290.md` | Diagnóstico Fase A | ~280 LOC ficheiro novo (5 secções A.0-A.5 com A.0 não-trivial empírica) |
| `01_core/src/rules/layout/tests.rs:10500+` | testes P290 | ~140 LOC (11 testes em `p290_style_tracking_tests` mod) |

Total: **2 sítios L1 produção + 2 ficheiros L0 documentação + 1
diagnóstico + 1 ficheiro de testes**. **Zero ficheiros tocados em
L3** (export.rs preservado bit-exact).

### §2.3 — Cascade arm (`push_styles`)

```rust
match style {
    Style::Bold(b)         => delta.bold = Some(*b),
    Style::Italic(i)       => delta.italic = Some(*i),
    Style::Size(pt)        => delta.size = Some(pt.val()),
    Style::Fill(c)         => delta.fill = Some(*c),
    Style::HeadingLevel(l) => delta.heading_level = Some(*l),
    Style::Lang(l)         => delta.lang = Some(*l),                  // P288
    Style::Weight(w)       => delta.weight = Some(*w),                // P289
    // P290 — 2ª fonte de entrada para `delta.tracking` (paralela à
    // parse-driven em `eval/rules.rs:374`). Consumers P137
    // (`cursor.rs:30` tracking_extra + `export.rs:2139-2146` `Tc`
    // operator) reusados sem alteração — ADR-0098 aderência confirmada
    // **mesmo com emit consumer real** (paradigma `TextStyle` capture
    // via `FrameItem::Text.style.tracking`; hash `export.rs 66cb8ac3`
    // preservado pelo 7º passo consecutivo).
    Style::Tracking(l)     => delta.tracking = Some(*l),
}
```

**Match continua exaustivo** — compilador detecta omissões por
construção (defesa cumulativa P288+P289+P290).

### §2.4 — Distinção arquitectural crítica P290 vs P288/P289

**P290 é a primeira aplicação prática de ADR-0098 onde emit
consome literalmente o campo activado**:

| Passo | Consumer principal | Emit em `export.rs`? |
|---|---|---|
| P288 (`lang`) | `eval_markup` localize_quotes; hyphenation P144; smartquote P287 | **0 hits funcionais** (lang dead-code em emit) |
| P289 (`weight`) | `faux_bold_stroke_pt` P139 (layout-time only) | **0 hits funcionais** (weight dead-code em emit) |
| **P290 (`tracking`)** | `cursor_extra` P137 (layout-time) + **`Tc` operator P137 (emit)** | **2 hits funcionais** em `export.rs:2139-2146` — **mas via `style.tracking` (paradigma `TextStyle` capture)**, não `chain.tracking()` directo |

**Subtileza arquitectural validada**: ADR-0098 não exige "ausência
total em emit" — exige "consumo via `FrameItem.style` capturado pelo
Layouter" (single source of truth pré-emit). P290 é o teste
empírico mais robusto da invariante até agora.

---

## §3 — Fase A — decisões registadas (`diagnostico-style-tracking-passo-290.md`)

### §3.1 — A.0 (NÃO-TRIVIAL desta vez) — Potencial de reuso ADR-0098

P289 §A.0 trivial (zero hits weight em export.rs). **P290 §A.0
verificação empírica obrigatória não-trivial** porque P137 implementou
`Tc` operator em PDF emit.

`grep "tracking\|Tc " 03_infra/src/export.rs`:

| Linha | Conteúdo | Classificação |
|---|---|---|
| `:2139-2141` | `let tracking_pt = style.tracking.map(\|t\| t.resolve_pt(style.size.val())).unwrap_or(0.0);` | **Leitura via `style.tracking`** (i.e. `TextStyle.tracking` capturado em `FrameItem::Text.style`) |
| `:2142-2146` | `let tc_op = if tracking_pt.abs() > f64::EPSILON { format!("{:.2} Tc\n", tracking_pt) } else { String::new() };` | Emit `Tc` operator |

**2 hits** — ambos via `style.tracking` (não `chain.tracking()`
directo). **Paradigma P136 confirmado**: emit lê de
`FrameItem::Text.style.tracking`, não via chain.

**Diferença material vs P289 A.0**: P289 esperava zero hits literais.
P290 espera **hits no formato `Tc` operator** mas verificados como
consumo via `TextStyle` capture (paradigma P136 + P137). A.0
documenta a distinção explicitamente — **ADR-0098 vigente apesar
de emit consumer real**.

### §3.2 — A.1 inventário literal (8 sub-secções paralelas a P289 §A.1)

| Sub-secção | Achado decisivo |
|---|---|
| A.1.1 — `Style` enum pós-P289 | 7 variants confirmadas (P290 → 8) |
| A.1.2 — `StyleDelta.tracking` | `Option<Length>` (P127/P137); `Length` é `Copy` (`layout_types.rs:601`) |
| A.1.3 — `push_styles` cascade | match exaustivo sobre 7 variants; arm trivial para `Tracking(l)` |
| A.1.4 — `delta.tracking` write | **Único** site: `eval/rules.rs:374` (parse `#set text(tracking: ...)`) |
| A.1.5 — `delta.tracking` read | `chain.tracking()` walk up-the-chain |
| A.1.6 — Consumers | **4 activos** (top-wins P136, cursor extra P137, TextStyle capture, **emit `Tc` operator P137**) |
| A.1.7 — `FrameItem::Text` emit | 2 hits em export.rs **via `style.tracking`** (P136 paradigm) — não viola ADR-0098 |
| A.1.8 — Diagrama de fluxo | Idêntico a P288/P289 §A.1.8 mas com nota explícita "emit consome via `TextStyle` capture, não via chain directo" |

### §3.3 — A.2 estrutura variant — opção (a)

**Decidido**: `Tracking(Length)` paralelo a `StyleDelta.tracking`.

Justificações: `Length` é `Copy` (P127 `#[derive(Copy)]`) → `Style:
Copy` intacto; preserva semântica (`abs + em`). Rejeita
`Tracking(f64)` (perde semântica `Em` vs `Pt`); rejeita
`Tracking(LengthVariant)` (indirecção sem ganho).

**Honestidade epistémica §A.2.3** (paralelo P287/P288/P289):
`Style::Tracking(Length)` é variant **atómico** (1 campo `Length`
required) — **não rico**. Padrão N=4 "variant rico" **inalterado**
pelo P290.

### §3.4 — A.3 cascade — opção (α)

`delta.tracking = Some(*l)` paralelo absoluto aos 7 arms anteriores.
Last-write wins per LIFO da chain — comportamento determinístico.

### §3.5 — A.4 emit — opção (i) confirmada empiricamente

| Critério | Evidência |
|---|---|
| `export.rs` consulta `style.tracking`? | A.0.1: 2 hits via `TextStyle.tracking` (P136 paradigm) |
| `export.rs` consulta `chain.tracking()` directo? | A.0.1: 0 hits |
| `FrameItem::Text` precisa novo field `tracking`? | Não — `TextStyle.tracking` já existe (P127/P136) |
| Consumer P137 emit Tc operator activo? | **Sim** — `export.rs:2139-2146` |
| Reflectors directos a `chain`? | Não |

**Hash `export.rs` preservado pelo 7º passo consecutivo**
(P282+P285+P286+P287+P288+P289+**P290**) — confirma ADR-0098
robusta como invariante operacional vigente.

### §3.6 — A.4.3 decisão sem promoção ADR meta nova

Padrões cumulativos pós-P290:

| Padrão | N pós-P290 | Promoção? |
|---|---:|---|
| ADR-0098 (formalizada P288) | **7** | ❌ Reforço cumulativo; sem nova promoção |
| ADR-0099 (formalizada P289) | **6** | ❌ Primeira reaplicação pós-formalização; sem nova promoção |
| §8.3 "refutação pragmática" | 5 estável | ❌ P290 segue defaults straight |
| §8.4 "bug latente fixed" | 1 estável | ❌ A.5 nenhum bug |
| §8.5 "patch cirúrgico sequencial" | **3** (P288+P289+P290) | ❌ P289 §8.5 **desqualifica passos cumulativos triviais** (anti-inflação por construção) |

**Decisão**: nenhuma ADR meta promovida. P273.17 §0 anti-padrão
over-formalização vigente.

### §3.7 — A.5 detecção bugs latentes (5 cenários — 1 a mais que P289)

| Cenário | Valor | Expectativa | Resultado |
|---|---:|---|:---:|
| Zero | `Length::pt(0.0)` | `chain.tracking() == Some(...)`; emit não emite `Tc` | ✅ |
| Pequeno positivo | `Length::pt(1.0)` | Emit `1.00 Tc` | ✅ |
| Em-relativo | `Length::em(0.5)` | Resolve em runtime | ✅ |
| **Negativo** | `Length::pt(-0.5)` | Vanilla aceita (kerning artificial) | ✅ |
| Grande | `Length::pt(10.0)` | Sem overflow | ✅ |

**Atenção particular ao tracking negativo**: passou — `Length::pt(-0.5)`
propaga gracefully via `chain.tracking()` → `TextStyle::from` →
emit `-0.50 Tc`. Paridade vanilla preservada. Padrão §8.4 P288
"bug latente fixed" **permanece N=1 estável** (nenhum bug colateral
detectado em P290).

### §3.8 — Riscos mitigados

| Risco | Status |
|---|---|
| A.0 violação nominal ADR-0098 | ✅ **Refutado empiricamente** (paradigma `TextStyle` capture confirmado) |
| `Length` não Copy | ✅ Refutado (`Length: Copy` confirmado em `layout_types.rs:601`) |
| Tracking negativo bug | ✅ Verificação positiva (`Length::pt(-0.5)` propaga gracefully) |
| Promoção §8.5 indevida | ✅ Refutado por A.4.3 + spec §7 risco quaternário |
| Sequência reflexa | ✅ Mitigado por Fase A não-trivial em A.0 (verificação `Tc` operator empírica genuína) |

---

## §4 — Testes adicionados (+11)

| Local | Quantidade | Cobertura |
|---|---:|---|
| `entities/style.rs` (mod tests) | 1 | Catalog test 7 → 8 variants (`Style::Tracking(Length::pt(0.5))` incluído) |
| `rules/layout/tests.rs` (`p290_style_tracking_tests`) | 10 | Variant ctor + PartialEq; `push_styles` cascade; `Styled` injection + TextStyle propagation; last-write wins; 5 fronteiras (0pt/1pt/0.5em/**-0.5pt**/10pt); consumer P137 cursor_extra |
| **Total** | **11** (1 entity + 10 layout) | Paralelo a P289 +9 com `+1 fronteira` (tracking negativo) + `+1 consumer P137 verification` |

**Resultado**: 11/11 verdes (`cargo test --lib p290`). Delta workspace
**+10** (porque catalog test substitui o anterior P289 7 → 8 mas
representa apenas 1 LOC novo).

---

## §5 — Observações pragmáticas

### §5.1 — Primeira aplicação prática de ADR-0098 com emit consumer real

P282-P289 foram **8 aplicações cumulativas** de ADR-0098 mas em
todas o consumer era em layout-time (eval_markup, hyphenation,
faux-bold) ou totalmente ausente (deco wrap aware). **P290 é a 1ª
aplicação onde emit consome literalmente o campo activado** — `Tc`
operator em PDF stream `export.rs:2139-2146`.

**Confirmação empírica subtil**: ADR-0098 não é "ausência total em
emit" mas "via `FrameItem.style` capturado pelo Layouter" (single
source of truth pré-emit). P290 valida ADR-0098 como **mais robusta
e subtil**:
- Emit pode consumir um campo activado.
- Desde que o consumo seja via `FrameItem::Text.style.tracking`
  (capturado pelo Layouter durante `layout_word`), e não via
  `chain.tracking()` directo.
- Hash `export.rs` preservado bit-exact pelo **7º passo consecutivo**.

Esta validação eleva ADR-0098 de "invariante retrospectiva" para
"invariante operacional testada empiricamente em consumo emit
real". Marco arquitectural genuíno.

### §5.2 — Sem promoção ADR meta nova (P273.17 §0 vigente)

P288 → ADR-0098, P289 → ADR-0099. Cumulativos pós-P290:
- ADR-0098: **N=7** (reforço)
- ADR-0099: **N=6** (reaplicação)
- §8.3 refutação pragmática: **N=5 estável**
- §8.4 bug latente fixed: **N=1 estável**
- §8.5 patch cirúrgico: **N=3** mas P289 §8.5 desqualifica passos
  cumulativos triviais (anti-inflação)

**Decisão**: nenhuma ADR meta promovida. P273.17 §0 anti-padrão
"promoção mecânica sem gatilho novo" vigente. P290 é reaplicação
cumulativa que **reforça** as ADRs existentes, não adiciona padrão
novo.

### §5.3 — A.0 não-trivial como template

P289 §A.0 trivial (zero hits weight). P290 §A.0 estabelece **template
para passos futuros onde A.0 é não-trivial**:

1. `grep` literal em `export.rs` para o campo activado.
2. Classificar cada hit: via `FrameItem.style` (paradigma P136 — OK)
   vs `chain.<campo>()` directo (violação nominal ADR-0098).
3. Documentar distinção empírica na A.0.

P290.1 (`leading`) e P290.2 (`font`) provavelmente terão A.0
trivial (zero hits — `leading` e `font` são dead-code em emit no
estado actual). Mas o template está disponível.

### §5.4 — Nenhum bug latente detectado (paralelo P289 §5.4)

P288 inaugurou padrão §8.4 (NBSP fix). P289 testou 4 fronteiras
weight — nenhum bug. **P290 testou 5 fronteiras incluindo
tracking negativo — nenhum bug detectado**:

- `Length::pt(-0.5)` (kerning artificial vanilla legítimo) propaga
  gracefully via cascade.
- `chain.tracking()` retorna `Some(Length::pt(-0.5))`.
- `TextStyle::from(&chain).tracking` propaga.
- Emit `-0.50 Tc` correcto.

Padrão §8.4 P288 **permanece N=1 estável** (não cumula com P290).

### §5.5 — Honestidade epistémica: variant atómico (não rico)

Padrão N=4 "variant rico com `body` + cosméticos opcionais"
**inalterado** pelo P290 — mesma lógica P287/P288/P289.
`Style::Tracking(Length)` é variant atómico paralelo aos restantes.

Registo redundante mas crítico para futuros passos não contarem
incorrectamente.

### §5.6 — Assimetria residual: 2/4 abertos

P288 fechou 1 (lang); P289 fechou mais 1 (weight); P290 fecha
mais 1 (tracking). Restam **2** dos originais 4:

| Field `StyleDelta` | Caminho actual exclusivo | Passo candidato |
|---|---|---|
| `leading: Option<Length>` | P138 — parse-driven | P290.1 |
| `font: Option<FontList>` | P132B/P140B/P141/P146 — parse-driven | P290.2 (possivelmente complexo; `FontList` wrapper) |

Cada um vai a passo próprio cirúrgico paralelo a P288/P289/P290.
Não-objectivo §5 deste passo. Reaplicações de ADR-0099 sem nova
promoção.

---

## §6 — Métricas

| Métrica | Valor |
|---------|-------|
| LOC L1 produção | ~30 (+1 variant 13 LOC com doc; +1 arm cascade 10 LOC com doc; +1 LOC em test catalog; +1 import) |
| LOC L3 produção | **0** (zero impacto em export.rs — hash preservado pelo 7º passo) |
| LOC L0 modificado | ~350 (`style.md` +25; `cobertura.md` B.3+B.4+footnote⁷⁶ ~95; `diagnostico-style-tracking-passo-290.md` ~280 ficheiro novo) |
| Testes adicionados | 11 (1 entity catalog + 10 layout: 4 ctor/cascade/inject/last-write + 5 fronteiras + 1 consumer P137) |
| Testes baseline P289 | 2 763 preserved bit-exact |
| Testes pós-P290 | **2 773** |
| Hash L0 `style.md` | `4cb5f241 → 3e40638a` (`style.rs → 661944c0`) |
| Hash L0 `content.md` | **inalterado** |
| Hash L0 `stdlib.md` | **inalterado** |
| Hash L0 `export.rs` | **`66cb8ac3` preservado** (**7º passo consecutivo** — ADR-0098 confirmada operacional) |
| Lint | zero violations |
| `Style` variants | 7 → **8** (+`Tracking(Length)`) |
| Cascade arms `push_styles` | 7 → **8** (+1 LOC) |
| Caminhos de entrada para `delta.tracking` | 1 → **2** (parse + Style::Tracking) |
| Fechos de assimetria B.3↔B.4 | 2/5 (P288 lang + P289 weight) → **3/5** (+ P290 tracking) |
| Pendências resolvidas | **1** (1/3 assimetria residual P289 §5.6) |
| **ADRs novas** | **0** (reaplicações sem promoção) |

---

## §7 — Conformidade Cristalina

- ✅ **ADR-0029 pureza física L1**: `Style::Tracking(Length)` é
  variant atómico `Copy`; cascade arm é função pura sem I/O.
- ✅ **ADR-0038 Style enum divergência intencional**: P290 estende
  o enum dentro do mesmo paradigma; vanilla continua vtable.
- ✅ **ADR-0040 `#set text` activation P102**: caminho parse-driven
  preservado intacto; P290 adiciona 2ª fonte sem perturbar 1ª.
- ✅ **ADR-0054 scope graded**: assimetria residual restante (2
  fields) documentada como scope-out **consciente** com passos
  candidatos P290.1-2.
- ✅ **ADR-0065 inventariar-primeiro**: Fase A obrigatória produziu
  **5 secções A.0-A.5** (paralelo P289 mas com **A.0 não-trivial
  empírica**).
- ✅ **ADR-0085 diagnóstico imutável**:
  `diagnostico-style-tracking-passo-290.md` produzido com 5 secções
  + diagrama de fluxo + métricas + risco residual mitigado.
- ✅ **ADR-0093 meta-metodologia evolução ADRs**: sem nova promoção
  — reaplicações cumulativas de ADRs vigentes (P273.17 §0
  anti-padrão).
- ✅ **ADR-0098** (P288): **primeira aplicação prática com emit
  consumer real** em P290 — confirma invariante robusta. Hash
  preservado pelo 7º passo consecutivo eleva ADR-0098 de "invariante
  retrospectiva" para "invariante operacional testada empiricamente".
- ✅ **ADR-0099** (P289): **primeira reaplicação pós-formalização**
  em P290 — confirma robustez do padrão "activação posterior de
  feature graded".
- ✅ **Anti-padrão over-formalização P273.17 §0**: **nenhuma ADR
  meta promovida** — reaplicações cumulativas sem gatilho novo
  genuíno.
- ✅ **Honestidade epistémica reforçada §5.5**: N=4 "variant rico"
  **inalterado** (Style::Tracking atómico, não rico).
- ✅ **Bit-exact regression preservada por construção**: caminho
  parse-driven `eval_set_rule` intacto; 2 763 testes pré-P290
  preserved.

---

## §8 — Padrões emergentes (cumulação sem novas promoções)

### §8.1 — ADR-0099 reaplicada (N=6 cumulativo)

ADR-0099 ("Activação posterior de feature graded") formalizada em
P289 com 5 aplicações cumulativas (P285-P289). **P290 é a 6ª
aplicação cumulativa** — primeira reaplicação pós-formalização:

| N | Passo | Tipo de activação |
|:---:|---|---|
| 1 | P285 | Campo novo em FrameItem |
| 2 | P286 | Campo opcional em Layouter |
| 3 | P287 | Variant leaf novo |
| 4 | P288 | Variant atómico novo (lang) |
| 5 | P289 | Variant atómico novo (weight) — formalização |
| **6** | **P290** | Variant atómico novo (tracking) — **1ª reaplicação pós-formalização** |

Confirma robustez do padrão como meta-processual operacional.

### §8.2 — ADR-0098 confirmada com emit consumer real (N=7 cumulativo)

ADR-0098 formalizada em P288 (N=5 cumulativo). P289 confirmou em
aplicação prática trivial (N=6). **P290 é a 7ª aplicação cumulativa
— primeira com emit consumer real**:

| N | Passo | Aplicação | Emit consumer? |
|:---:|---|---|:---:|
| 1 | P282 | Auditoria empírica | n/a |
| 2 | P285 | Alteração simétrica via helper único | Sim (linha) |
| 3 | P286 | Reuso `FrameItem::Line` | Idem N=2 |
| 4 | P287 | Consumer reusa `Content::Text` | Idem texto |
| 5 | P288 | `Style::Lang` (formalização ADR-0098) | **Não** (dead-code) |
| 6 | P289 | `Style::Weight` (primeira aplicação prática) | **Não** (dead-code) |
| **7** | **P290** | **`Style::Tracking` — primeira com emit real** | **Sim — `Tc` operator P137** |

**Marco**: ADR-0098 testada empiricamente em consumo emit real.
Hash `export.rs` preservado pelo 7º passo consecutivo confirma
**invariante operacional robusto, não apenas pattern
retrospectivo**.

### §8.3 — "Refutação pragmática" N=5 estável

P287 atingiu N=5 com refutações genuínas. P288/P289/P290 seguem
defaults straight da spec (a) e (α) sem refutação significativa
nova. Padrão **permanece N=5 estável**.

### §8.4 — "Bug latente fixed durante materialização" N=1 estável

P288 inaugurou N=1 (NBSP fix). P289 testou 4 fronteiras sem bugs;
P290 testou 5 fronteiras incluindo tracking negativo — **nenhum
bug detectado**. Padrão **permanece N=1 estável**.

### §8.5 — "Patch cirúrgico sequencial paralelo" N=3 — **desqualificado para formalização**

P288+P289 inaugurou padrão N=2 com nota anti-inflação. **P290 atinge
N=3 mecanicamente** (3 passos cumulativos com estrutura quase-idêntica)
mas P289 §8.5 **explicitamente desqualifica passos cumulativos
triviais** como reaplicações genuínas para esse padrão:

> "passos próprios cumulativos não bastam — seria inflação por
> construção"

**Decisão**: padrão **não promovido em P290** mesmo com N=3.
Critério de promoção requer **N≥4 com "reaplicação não-trivial"**
(passos onde a estrutura paralela revela algo arquitectural novo,
não apenas continua a sequência cirúrgica). P290.1 (`leading`) e
P290.2 (`font`) provavelmente continuam triviais — padrão pode ficar
desqualificado de forma estável.

---

## §9 — Próximos passos sugeridos (estado pós-P290)

Cobertura agregada estimada: inalterada (~64%). P290 não altera
contagem user-facing (variant arquitectural) mas:
- **Fecha 1/3 da assimetria residual P289 §5.6** (3/5 fechados;
  2/5 restantes).
- **Confirma ADR-0098 com emit consumer real** — marco arquitectural
  empírico genuíno.
- **Reaplica ADR-0099 pela 1ª vez pós-formalização** — confirma
  robustez do padrão.

### Rank 1-2: completar fecho da assimetria (paralelo P288/P289/P290)

1. **`P-style-leading-variant`** (XS; P290.1) — adiciona
   `Style::Leading(Length)` paralelo a `Style::Tracking(Length)` P290.
   Provavelmente A.0 trivial (zero hits em export.rs — `leading` é
   layout-time only, sem `Tc` operator equivalente). Reaplica ADR-0099
   (não promove). Hash `export.rs` esperado preservado pelo 8º passo
   consecutivo.
2. **`P-style-font-variant`** (XS-S; P290.2) — idem para
   `font: Option<FontList>`. **Possivelmente mais complexo** —
   `FontList` é wrapper (`Vec<FontFamily>`); pode precisar de tipo
   diferente vs (Length/u16/Lang). Verificar A.1.2.

### Rank 3-5: features médias

3. **`P-math-accent-cancel`** (XS+S; Math 40% → 50%).
4. **`P-curve-geometry`** (S-M; ADR-0078 sub-fase b).
5. **`P-footnote-cluster`** (M; Model 60% → 70%).

### Padrões pós-P290 (operacional)

- **ADR-0098 + ADR-0099 vigentes** — Fase A de passos futuros
  inclui secção A.0 obrigatória.
- **Padrão §8.3** "refutação pragmática" N=5 estável.
- **Padrão §8.5** "patch cirúrgico sequencial paralelo" N=3 **estável
  desqualificado** — anti-inflação por construção; promoção requer
  reaplicação não-trivial (próximo passo P290.1 provavelmente
  trivial → padrão pode ficar desqualificado de forma estável).
- **Próxima oportunidade ADR meta nova**: quando padrão emergente
  novo (não cumulativo de P288-P290) atingir N≥5 com gatilho
  genuíno.

---

## §10 — Referências cross-passos

- **P102 ADR-0040** — `#set text(...)` activation; precedente
  parse-driven path.
- **P127 ADR-0038** — `Length` type materializado (P127 introduziu
  storage `abs + em`).
- **P136 Fase A DEBT-52** — Top-wins propagation em
  `layout/mod.rs:579` (paradigma `TextStyle` capture).
- **P137** — Tracking `Tc` operator emit em PDF — reusado sem
  alteração em P290.
- **P138** — Leading (próximo candidato P290.1).
- **P281** — Unificação β-completa stream-builders (origem
  ADR-0098).
- **P282 §1.1** — Auditoria empírica (N=1 ADR-0098).
- **P285-P289 §8.1/§8.2** — Aplicações cumulativas ADR-0098 +
  ADR-0099.
- **P288** — Formalização ADR-0098 (N=5 limiar).
- **P289 §A.4/§5.1** — Primeira aplicação prática trivial de
  ADR-0098 + formalização ADR-0099 (N=5).
- **P290 §A.4 / §5.1** — **Primeira aplicação prática de ADR-0098
  com emit consumer real** + 1ª reaplicação ADR-0099 pós-formalização.
- **ADR-0026 / ADR-0026-R1** — Enum vs vtable divergência.
- **ADR-0029** — Pureza física L1.
- **ADR-0038** — Style enum divergência intencional.
- **ADR-0054** — Scope graded (assimetria residual 2 fields).
- **ADR-0065** — Inventariar-primeiro.
- **ADR-0085** — Diagnóstico imutável (44º consumo: P290 + P289 +
  P288 + 41 anteriores).
- **ADR-0093** — Meta-metodologia evolução ADRs.
- **ADR-0094** — Meta-operacional specs.
- **ADR-0098** — Single source of truth como invariante anti-bug
  (formalizada P288; primeira aplicação prática trivial P289;
  **primeira aplicação prática com emit consumer real P290**).
- **ADR-0099** — Activação posterior de feature graded (formalizada
  P289; **primeira reaplicação pós-formalização P290**).
- **P273.17 §0** — Anti-padrão over-formalização (vigente; nenhuma
  ADR meta promovida em P290).

---

*P290 fecha 1/3 da assimetria residual P289 §5.6 ao adicionar
`Style::Tracking(Length)` ao enum `Style` (7 → 8 variants) com arm
correspondente em `StyleChain::push_styles` (paralelo arquitectural
absoluto ao P288 `Style::Lang(Lang)` e P289 `Style::Weight(u16)`).
2ª fonte de entrada para `delta.tracking` materializada — caminho
parse-driven `eval_set_rule` (P127/P137) + consumers P137 (`cursor_extra`
+ emit `Tc` operator) preservados sem alteração. **Hash `export.rs
66cb8ac3` preservado bit-exact pelo 7º passo consecutivo
(P282+P285+P286+P287+P288+P289+P290) — primeira aplicação de
ADR-0098 com emit consumer real**, confirma invariante robusta como
operacional vigente (não apenas pattern retrospectivo). Subtileza
arquitectural validada empiricamente: ADR-0098 critério é "consumo
via `FrameItem.style` capturado pelo Layouter" (single source of
truth pré-emit), não "ausência total em emit". **Nenhuma ADR meta
nova promovida** — ADR-0098 (N=7 cumulativo) e ADR-0099 (N=6
cumulativo) reaplicam sem nova promoção; P273.17 §0 anti-padrão
vigente. Padrão §8.5 P289 atinge N=3 mas **desqualificado** para
formalização (anti-inflação por construção). Nenhum bug latente
colateral detectado (A.5 verificou 5 fronteiras incluindo tracking
negativo `-0.5pt` propaga gracefully). 11 testes P290 verdes (1
entity catalog + 10 layout); baseline 2 763 → 2 773 (+10).
Assimetria residual passa de 3/5 (P289) para **3/5 fechados pós-P290**
— restam **2/5** (`leading`/`font`) como passos próprios candidatos
P290.1-2 não-reservados (reaplicações ADR-0099 sem nova promoção).
Honestidade epistémica preservada: variant atómico (não rico) —
padrão N=4 inalterado.*
