# Passo 291 — Relatório consolidado

**Tema**: Materialização da frente `P-style-leading-variant` —
extensão simétrica do enum `Style` com variant `Leading(Length)`,
paralela a P288/P289/P290. Fecha 1/2 da assimetria residual P290
§5.6. **Primeira aplicação prática da secção A.5' anti-reflexão**
(mitigação activa do risco P290 §7 risco quinário "sequência
reflexa") — paradigma consumer arquiteturalmente distinto
identificado.

**Data**: 2026-05-19
**Branch**: Tekt
**Magnitude**: XS-S (modificação cirúrgica: +1 variant atómico +1
arm em `push_styles`; **0 ADRs novas** — reaplicações de
ADR-0098/0099 sem nova promoção; +11 testes; **8º passo consecutivo**
a preservar hash `export.rs`).

---

## §1 — Validação contra spec (critérios §4)

| Critério §4 | Estado |
|---|---|
| `cargo test --workspace` verde | ✅ **2 783** testes (baseline P290: 2 773 → **+10**) |
| Delta esperado ~+8-12 | ✅ +10 (dentro do range) |
| `crystalline-lint` zero violations | ✅ Confirmado |
| Hash L0 `style.md` muda (+1 variant em B.3) | ✅ `3e40638a → 899414b5` (`style.rs → fda3a2dd`) |
| Hash L0 `content.md` preserved | ✅ Preservado |
| Hash L0 `stdlib.md` preserved | ✅ Preservado |
| Hash L0 `export.rs` **condicional** preserved se A.4 → (i) | ✅ **`66cb8ac3` preservado bit-exact pelo 8º passo consecutivo** — A.0 trivial confirmada empiricamente (zero hits) |
| **Regressão bit-exact validada** — P138 parse-driven | ✅ 2 773 testes P290 preservados; consumer P138 reusado sem alteração (`p291_leading_consumer_p138_flush_line_peek` verifica) |
| Tabela B.3 actualizada com `Leading(Length)` (9º variant) + nota assimetria residual (1 field restante: `font`) | ✅ Confirmado |
| Tabela B.4 linha 352 com nota cruzada P291 | ✅ Confirmado |
| Diagnóstico A.0+A.1+A.2+A.3+A.4+A.5+**A.5'** produzido | ✅ `diagnostico-style-leading-passo-291.md` (**6 secções** + 8 sub-secções A.1 + diagrama de fluxo genuíno) |
| **Sem promoção ADR meta nova** — ADR-0099 reaplica | ✅ Confirmado |
| Bug latente colateral (se descoberto) registado e fixado | ✅ A.5 — **nenhum bug detectado** (5 cenários fronteira: 0pt/11pt/0.65em/50pt/**-1pt negativo** todos passam; consumer P138 robusto) |
| **A.5' anti-reflexão produzida honestamente** | ✅ Comparação literal A.1.6 P288-P291 + elemento estructuralmente novo identificado (paradigma peek `current_line` per-line); **sequência NÃO é rubber-stamp** |

**Conformidade**: 14/14 critérios estritos cumpridos. P291 **acrescenta
critério novo** (A.5' anti-reflexão) ao padrão estabelecido em P289/P290.

---

## §2 — Resumo factual

### §2.1 — Variant atómico + cascade arm + A.5' anti-reflexão (NOVA)

**Antes P291**:
- `Style` enum: 8 variants pós-P290
  (Bold/Italic/Size/Fill/HeadingLevel/Lang/Weight/Tracking).
- `StyleDelta.leading: Option<Length>` parseado-mas-inerte do ponto
  de vista de `Styles` collection (escrito apenas via parse-driven
  `eval/rules.rs:298` desde P128/P138).
- 3 consumers activos de `chain.leading()`: top-wins propagation
  P136 (`layout/mod.rs:580`), **`flush_line` peek P138**
  (`cursor.rs:119-128`), TextStyle capture (`style_chain.rs:305`).
- Assimetria Tabela B.3 (8 variants) vs B.4 (10 fields) = 2 fields
  residuais (`leading`/`font`).
- ADR-0098 + ADR-0099 vigentes desde P288/P289.
- Risco quinário P290 §7 "sequência reflexa" antecipado mas sem
  mitigação activa.

**Pós-P291**:
- `Style` enum: **9 variants** (+`Leading(Length)`).
- **2ª fonte de entrada** para `delta.leading` via
  `Content::Styled(body, Styles::from_iter([Style::Leading(Length::em(0.65))]))`.
- Caminho parse-driven **intacto** — bit-exact preservado.
- Consumer P138 (`flush_line` peek) **reusado sem alteração** —
  paradigma per-line distinto vs P290 per-glyph.
- **0 ADRs novas** — ADR-0098 (N=8) + ADR-0099 (N=7) reaplicam.
- Assimetria residual: **1 field** (`font`).
- **A.5' anti-reflexão inaugurada** — mitigação operacional do
  risco quinário; **template para passos futuros**.

### §2.2 — Cobertura de mudança

| Sítio | Tipo | Mudança |
|---|---|---|
| `01_core/src/entities/style.rs:55-87` | extensão enum | +1 variant `Leading(Length)` com doc-comment ~20 LOC referenciando paradigma per-line + divergência consciente vanilla `par`↔cristalino `text` |
| `01_core/src/entities/style.rs:166-183` | test extensão | catalog actualizado 8 → 9 variants |
| `01_core/src/entities/style_chain.rs:158-167` | cascade arm | +1 arm `Style::Leading(l) => delta.leading = Some(*l)` (+10 LOC com doc paradigma consumer) |
| `00_nucleo/prompts/entities/style.md` | L0 | +9º variant em B.3 + nota A.5' anti-reflexão |
| `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | L0 cobertura | B.3 +1 linha; B.4 linha 352 nota P291; footnote ⁷⁷ ~95 LOC com secção A.5' |
| `00_nucleo/diagnosticos/diagnostico-style-leading-passo-291.md` | Diagnóstico Fase A | ~330 LOC ficheiro novo (**6 secções** A.0-A.5 + A.5') |
| `01_core/src/engine/layout/tests.rs:10640+` | testes P291 | ~140 LOC (11 testes em `p291_style_leading_tests` mod) |

Total: **2 sítios L1 produção + 2 ficheiros L0 documentação + 1
diagnóstico + 1 ficheiro de testes**. **Zero ficheiros tocados em
L3** (export.rs preservado bit-exact pelo 8º passo consecutivo).

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
    Style::Tracking(l)     => delta.tracking = Some(*l),              // P290
    // P291 — 2ª fonte de entrada para `delta.leading` (paralela à
    // parse-driven em `eval/rules.rs:298`). Consumer P138 distintivo:
    // `cursor.rs:119-128` em `flush_line` peek do último
    // `FrameItem::Text` da current_line (`iter().rev().find_map`) —
    // paradigma **per-line via peek**, distinto do per-glyph de P290
    // tracking. `export.rs` zero hits para leading (ADR-0098 vigente;
    // hash `66cb8ac3` preservado pelo 8º passo consecutivo).
    Style::Leading(l)      => delta.leading = Some(*l),
}
```

**Match continua exaustivo** — compilador detecta omissões por
construção (defesa cumulativa P288+P289+P290+P291).

### §2.4 — Paradigma consumer distinto P291 vs P288/P289/P290

| Passo | Variant | Consumer principal | Paradigma |
|---|---|---|---|
| P288 | `Lang(Lang)` | eval_markup localize_quotes; hyphenation `hypher`; smartquote consumer | **Cross-module** (eval + lang + layout) |
| P289 | `Weight(u16)` | `TextStyle::faux_bold_stroke_pt` P139 | **TextStyle method** (layout-time only) |
| P290 | `Tracking(Length)` | `cursor.rs:30` per-glyph horizontal advance + `export.rs:2139-2146` `Tc` emit | **Per-glyph horizontal** |
| **P291** | `Leading(Length)` | `cursor.rs:119-128` em `flush_line` peek do último `FrameItem::Text` da current_line | **Per-line via peek `current_line` antes do drain** |

**4 paradigmas arquiteturalmente distintos** — confirma que P288-P291
não são rubber-stamp. Apenas a *cascade arm* é trivial paralelo (por
design ADR-0098 — single source of truth).

---

## §3 — Fase A — decisões registadas (`diagnostico-style-leading-passo-291.md`)

### §3.1 — A.0 trivial confirmada empiricamente (não citada)

`grep "leading" 03_infra/src/export.rs` → **0 hits**. Antecipação
P290 §5.3 confirmada por **inspecção literal**, não citação.

| Critério | Veredicto |
|---|---|
| Emit consome leading? | ❌ Não (zero hits) |
| `FrameItem::Text` precisa novo field `leading`? | ❌ Não — `TextStyle.leading` já existe (P138) |
| `Frame`/`Region` consultam leading directamente? | ❌ Não (zero hits funcionais) |
| Hash `export.rs` esperado | **Preservado bit-exact** (8º passo consecutivo) |

### §3.2 — A.1 inventário literal (8 sub-secções com **A.1.6 distintivo**)

A.1.6 revela paradigma consumer arquiteturalmente novo:

```rust
// cursor.rs:119-128 (flush_line)
let line_leading_pt = self.regions.current.current_line
    .iter()
    .rev()
    .find_map(|item| match item {
        FrameItem::Text { style, .. } => {
            style.leading.map(|l| l.resolve_pt(self.font_size_pt.val()))
        }
        _ => None,
    })
    .unwrap_or(0.0);
```

**Peek do último `FrameItem::Text` da current_line antes do drain**
— paradigma "captura estilo persistente no momento da finalização da
unidade" (vs per-glyph P290 ou TextStyle method P289).

### §3.3 — A.2 estrutura — opção (a) forçada estructuralmente

**Decidido**: `Leading(Length)` paralelo a `StyleDelta.leading`.

A.1.2 confirma `Option<Length>` — opção (a) é **forçada
estructuralmente** (não preferência). `Length` é `Copy` (P127) →
`Style: Copy` intacto.

**Honestidade epistémica §A.2.3** (paralelo P287/P288/P289/P290):
variant **atómico** (1 campo `Length` required) — **não rico**.
Padrão N=4 "variant rico" **inalterado** pelo P291.

### §3.4 — A.3 cascade — opção (α)

`delta.leading = Some(*l)` paridade absoluta aos 8 arms anteriores.

### §3.5 — A.4 emit — opção (i) confirmada empiricamente

| Critério | Evidência |
|---|---|
| `export.rs` consulta `style.leading`? | A.0.1: 0 hits |
| Emit consome coordenadas Y resultantes (não leading directo)? | ✅ Sim — leading é resolvido em `flush_line` antes do emit |
| Reflectors directos? | Não |

**Hash `export.rs` preservado pelo 8º passo consecutivo**.

### §3.6 — A.5 detecção bugs latentes (5 cenários incluindo negativo)

| Cenário | Valor | Resultado |
|---|---:|:---:|
| Zero | `Length::pt(0.0)` | ✅ |
| Típico | `Length::pt(11.0)` | ✅ |
| Em-relativo | `Length::em(0.5)` | ✅ |
| Grande | `Length::pt(50.0)` | ✅ |
| **Negativo** | `Length::pt(-1.0)` | ✅ — `cursor.rs:124` passa valor literal sem clamp; paridade vanilla typst line collapse parcial |

**Nenhum bug latente detectado**. Padrão §8.4 P288 permanece N=1
estável.

### §3.7 — A.5' anti-reflexão (NOVA per P290 §7 risco quinário)

**Primeira aplicação prática** desta secção. Inaugura template para
passos futuros.

#### §3.7.1 — Comparação literal A.1.6 P288/P289/P290/P291

| Passo | Paradigma consumer principal |
|---|---|
| P288 lang | Cross-module (eval+lang+layout) |
| P289 weight | TextStyle method (`faux_bold_stroke_pt`) |
| P290 tracking | Per-glyph (horizontal advance + `Tc` emit) |
| **P291 leading** | **Per-line via peek `current_line.iter().rev().find_map(FrameItem::Text)`** |

**4 paradigmas arquiteturalmente distintos** — sequência NÃO é
rubber-stamp.

#### §3.7.2 — A.0 produzido empiricamente

P291 §A.0 verificou literalmente `grep "leading" export.rs` → 0 hits.
**Antecipação P290 §5.3 confirmada por inspecção, não citação**.
Distinção honesta preservada.

#### §3.7.3 — Elemento estructuralmente novo identificado

✅ **Identificado**: paradigma "peek último `FrameItem::Text` da
current_line antes do drain" em `cursor.rs:119-128` é
arquiteturalmente novo:

- **Não é** per-glyph (como tracking P290).
- **Não é** TextStyle method (como weight P289).
- **Não é** cross-module (como lang P288).
- **É** "peek o último item de tipo X numa colecção em construção,
  antes do drain" — padrão estructural distinto que materializa
  "estilo persistente captura no momento da finalização da unidade".

#### §3.7.4 — Decisão sobre P292 (font)

A.5' **NÃO revela sequência reflexa**. **P292 pode prosseguir** sem
intercalação ortogonal forçada.

P292 (font) tem complicação extra: `font: Option<FontList>`, e
`FontList` é wrapper que pode requerer tipo diferente vs
(Length/u16/Lang). P292 deve verificar A.1.2 com cuidado especial.

### §3.8 — Riscos mitigados

| Risco §7 | Status |
|---|---|
| Sequência reflexa | ✅ **Refutado por A.5'** — paradigma distinto identificado |
| A.0 antecipação falsa | ✅ Refutado — 0 hits confirmados empiricamente |
| Leading negativo bug | ✅ Verificação positiva |
| `StyleDelta.leading` tipo diferente | ✅ Refutado — `Option<Length>` confirmado |
| Rubber-stamp + ignorar A.5' | ✅ Mitigado — A.5' produzido genuinamente |

---

## §4 — Testes adicionados (+11)

| Local | Quantidade | Cobertura |
|---|---:|---|
| `entities/style.rs` (mod tests) | 1 | Catalog test 8 → 9 variants (`Style::Leading(Length::em(0.65))` incluído) |
| `engine/layout/tests.rs` (`p291_style_leading_tests`) | 10 | Variant ctor + PartialEq; `push_styles` cascade; `Styled` injection + TextStyle propagation; last-write wins; **5 fronteiras** (0pt/11pt/0.65em/50pt/**-1pt**); consumer P138 flush_line peek |
| **Total** | **11** | Paralelo a P290 +11 (mesma estrutura; novo: consumer P138 verification) |

**Resultado**: 11/11 verdes (`cargo test --lib p291`). Delta workspace
**+10** (catalog test substitui o anterior P290 8 → 9; 1 LOC novo).

---

## §5 — Observações pragmáticas

### §5.1 — A.5' anti-reflexão inaugurada — template para passos futuros

P290 §7 risco quinário registou *"P290 e seus sucessores P290.1/P290.2
entrarem em 'sequência reflexa' — automatização sem reflexão genuína."*
**P291 §A.5' é a primeira mitigação operacional activa** deste risco.

**Estrutura da A.5' inaugurada**:
1. Comparação literal A.1.6 de N passos anteriores (P288-P291).
2. Verificação que A.0 foi produzido empiricamente (não citado).
3. Identificação de pelo menos 1 elemento estructuralmente novo.
4. Decisão sobre passo seguinte (procede ou intercala ortogonal).

**Template para passos futuros**: P292 (font) deverá ter A.5'
paralela. Se A.5' identificar elemento estructuralmente novo (e.g.
`FontList` wrapper introduz consumer paradigm distinto), prossegue;
caso contrário, intercala passo ortogonal.

### §5.2 — Paradigma consumer distinto valida A.5' positivamente

P291 §A.5'.3 identifica paradigma novo: "peek `current_line.iter().rev()
.find_map(FrameItem::Text)` antes do drain". Este paradigma:
- Materializa "estilo persistente captura no momento da finalização
  da unidade" (semântica "last writer wins per-line").
- É arquitecturalmente distinto vs per-glyph (P290), TextStyle method
  (P289), cross-module (P288).
- **Confirma que P288-P291 são reaplicações arquiteturalmente
  distintas** com fonte de entrada paralela (por design ADR-0098).

A semelhança superficial (4 cascade arms idênticos em estrutura) é
**by design** — single source of truth via `StyleDelta`. A
diversidade arquitectural está no **consumo**, não na entrada.

### §5.3 — Hash `export.rs` preservado pelo 8º passo consecutivo

Sequência cumulativa:

| Passo | Razão preservação |
|---|---|
| N=1: P282 | Auditoria empírica refutou 6/6 suspeitas |
| N=2: P285 | Alteração simétrica via helper único |
| N=3: P286 | Reuso `FrameItem::Line` sem modificação |
| N=4: P287 | Consumer reusa `Content::Text` |
| N=5: P288 | `Style::Lang` extende parse sem tocar emit (formalização ADR-0098) |
| N=6: P289 | `Style::Weight` aplica ADR-0098 directamente |
| N=7: P290 | `Style::Tracking` — primeira com emit consumer real |
| **N=8: P291** | **`Style::Leading` — primeiro com consumer per-line via peek** |

Cada passo cumulativo **reforça** a invariante. ADR-0098 valida-se
como **invariante operacional vigente, não apenas pattern retrospectivo**.

### §5.4 — Nenhuma ADR meta nova promovida (P273.17 §0 vigente)

P288 → ADR-0098, P289 → ADR-0099. Cumulativos pós-P291:
- ADR-0098: **N=8** (reforço)
- ADR-0099: **N=7** (reaplicação)
- §8.3 refutação pragmática: **N=5 estável**
- §8.4 bug latente fixed: **N=1 estável**
- §8.5 patch cirúrgico sequencial: **N=4** mas P290 §8.5 **desqualifica
  passos cumulativos triviais** (anti-inflação)

**Decisão**: nenhuma ADR meta promovida. P273.17 §0 anti-padrão
"promoção mecânica sem gatilho novo" vigente. **Anti-padrão
estende-se implicitamente a "over-automatização"** — A.5' é
mitigação operacional desta extensão.

### §5.5 — Nenhum bug latente detectado (paralelo P289/P290)

P288 inaugurou padrão §8.4 (NBSP fix). P289+P290 testaram fronteiras
sem bugs. **P291 testou 5 fronteiras incluindo leading negativo —
nenhum bug detectado**:

- `Length::pt(-1.0)` (line collapse parcial vanilla legítimo)
  propaga gracefully via cascade.
- `cursor.rs:124` `style.leading.map(|l| l.resolve_pt(...))` passa
  valor literal sem clamp.

Padrão §8.4 P288 **permanece N=1 estável**.

### §5.6 — Honestidade epistémica: variant atómico (não rico)

Padrão N=4 "variant rico com `body` + cosméticos opcionais"
**inalterado** pelo P291 — mesma lógica P287/P288/P289/P290.

### §5.7 — Divergência arquitectural consciente preservada

Tabela A.3 linha 70 (cobertura) regista que cristalino captura
`leading` em `text` por conveniência temporária; vanilla tem em
`par`. P291 **preserva esta divergência** per spec §5 não-objectivo
("Não alterar relação arquitectural `text`↔`par`").

Doc-comment de `Style::Leading` regista a divergência literalmente
para que futuros passos (e.g. materialização `Content::Par`) saibam
o contexto histórico.

### §5.8 — Assimetria residual reduzida a 1/4

P288 fechou 1 (lang); P289 fechou mais 1 (weight); P290 fechou mais
1 (tracking); P291 fecha mais 1 (leading). Resta **1 dos originais 4**:

| Field `StyleDelta` | Caminho actual exclusivo | Passo candidato |
|---|---|---|
| `font: Option<FontList>` | P132B/P140B/P141/P146 — parse-driven | **P292** (verificar A.1.2 — FontList é wrapper) |

P292 candidato não-reservado. **A.5' P291 valida que P292 pode
prosseguir**, mas com **cuidado especial em A.1.2**: `FontList` é
`Vec<FontFamily>` wrapper que pode requerer estructura diferente vs
(Length/u16/Lang). Se `FontList` não for `Copy`, `Style: Copy` pode
ser ameaçado — P292 precisa adaptar opção A.2.

---

## §6 — Métricas

| Métrica | Valor |
|---------|-------|
| LOC L1 produção | ~32 (+1 variant 20 LOC com doc; +1 arm cascade 10 LOC com doc; +1 LOC em test catalog; +1 referência) |
| LOC L3 produção | **0** (zero impacto em export.rs — hash preservado pelo 8º passo) |
| LOC L0 modificado | ~430 (`style.md` +30; `cobertura.md` B.3+B.4+footnote⁷⁷ ~95; `diagnostico-style-leading-passo-291.md` ~330 ficheiro novo) |
| Testes adicionados | 11 (1 entity catalog + 10 layout: 4 ctor/cascade/inject/last-write + 5 fronteiras + 1 consumer P138) |
| Testes baseline P290 | 2 773 preserved bit-exact |
| Testes pós-P291 | **2 783** |
| Hash L0 `style.md` | `3e40638a → 899414b5` (`style.rs → fda3a2dd`) |
| Hash L0 `content.md` | **inalterado** |
| Hash L0 `stdlib.md` | **inalterado** |
| Hash L0 `export.rs` | **`66cb8ac3` preservado** (**8º passo consecutivo**) |
| Lint | zero violations |
| `Style` variants | 8 → **9** (+`Leading(Length)`) |
| Cascade arms `push_styles` | 8 → **9** (+1 LOC) |
| Caminhos de entrada para `delta.leading` | 1 → **2** (parse + Style::Leading) |
| Fechos de assimetria B.3↔B.4 | 3/5 (P288+P289+P290) → **4/5** (+ P291 leading) |
| Pendências resolvidas | **1** (1/2 assimetria residual P290 §5.6) |
| **ADRs novas** | **0** (reaplicações sem promoção) |
| **A.5' anti-reflexão produzida** | ✅ **Primeira aplicação prática** (inaugura template) |

---

## §7 — Conformidade Cristalina

- ✅ **ADR-0029 pureza física L1**: `Style::Leading(Length)` é
  variant atómico `Copy`; cascade arm é função pura sem I/O.
- ✅ **ADR-0038 Style enum divergência intencional**: P291 estende
  o enum dentro do mesmo paradigma.
- ✅ **ADR-0040 `#set text` activation P102**: caminho parse-driven
  preservado intacto.
- ✅ **ADR-0054 scope graded**: assimetria residual restante (1
  field `font`) documentada como scope-out **consciente**.
- ✅ **ADR-0065 inventariar-primeiro**: Fase A obrigatória produziu
  **6 secções A.0-A.5 + A.5'** (uma a mais que P289/P290).
- ✅ **ADR-0085 diagnóstico imutável**:
  `diagnostico-style-leading-passo-291.md` produzido com 6 secções +
  diagrama de fluxo + métricas + risco residual mitigado +
  **A.5' anti-reflexão obrigatória**.
- ✅ **ADR-0093 meta-metodologia evolução ADRs**: sem nova promoção
  — reaplicações cumulativas de ADRs vigentes.
- ✅ **ADR-0098** (P288): **8º passo consecutivo** a preservar
  `export.rs` — reforço cumulativo.
- ✅ **ADR-0099** (P289): **2ª reaplicação pós-formalização** —
  confirma robustez.
- ✅ **Anti-padrão over-formalização P273.17 §0**: **nenhuma ADR
  meta promovida**. **Anti-padrão estende-se implicitamente a
  "over-automatização"** — A.5' é mitigação operacional desta
  extensão.
- ✅ **Honestidade epistémica reforçada §5.6**: N=4 "variant rico"
  **inalterado** (Style::Leading atómico, não rico).
- ✅ **Bit-exact regression preservada por construção**: caminho
  parse-driven `eval_set_rule` intacto; 2 773 testes pré-P291
  preserved.
- ✅ **Divergência arquitectural consciente registada literalmente**:
  doc-comment de `Style::Leading` regista vanilla `par`↔cristalino
  `text` para contexto histórico futuro.

---

## §8 — Padrões emergentes (cumulação sem novas promoções + 1 NOVO)

### §8.1 — ADR-0099 reaplicada (N=7 cumulativo)

ADR-0099 ("Activação posterior de feature graded") formalizada em
P289 com 5 aplicações cumulativas (P285-P289). P290 foi 1ª
reaplicação pós-formalização (N=6). **P291 é a 7ª aplicação
cumulativa — 2ª reaplicação pós-formalização**:

| N | Passo | Tipo de activação |
|:---:|---|---|
| 1 | P285 | Campo novo em FrameItem |
| 2 | P286 | Campo opcional em Layouter |
| 3 | P287 | Variant leaf novo |
| 4 | P288 | Variant atómico novo (lang) |
| 5 | P289 | Variant atómico novo (weight) — formalização |
| 6 | P290 | Variant atómico novo (tracking) — 1ª reaplicação |
| **7** | **P291** | **Variant atómico novo (leading) — 2ª reaplicação** |

### §8.2 — ADR-0098 confirmada com consumer per-line (N=8 cumulativo)

ADR-0098 formalizada em P288. P290 foi primeira com emit consumer
real (N=7). **P291 é a 8ª aplicação cumulativa — primeira com
consumer per-line via peek**:

| N | Passo | Aplicação | Paradigma consumer |
|:---:|---|---|---|
| 1 | P282 | Auditoria empírica | n/a |
| 2 | P285 | Alteração simétrica via helper único | Linha simétrica |
| 3 | P286 | Reuso `FrameItem::Line` | Idem |
| 4 | P287 | Consumer reusa `Content::Text` | Texto |
| 5 | P288 | `Style::Lang` (formalização) | Cross-module |
| 6 | P289 | `Style::Weight` (1ª prática) | TextStyle method |
| 7 | P290 | `Style::Tracking` (1ª com emit consumer real) | Per-glyph |
| **8** | **P291** | **`Style::Leading` (1ª com consumer per-line via peek)** | **Per-line via peek `current_line`** |

ADR-0098 valida-se com **8 paradigmas consumer distintos** —
invariante operacional robusta cross-paradigm.

### §8.3 — "Refutação pragmática" N=5 estável

P291 segue defaults straight da spec (a) e (α) sem refutação
significativa nova. Padrão **permanece N=5 estável**.

### §8.4 — "Bug latente fixed durante materialização" N=1 estável

P291 testou 5 fronteiras (incluindo leading negativo) — **nenhum
bug detectado**. Padrão **permanece N=1 estável**.

### §8.5 — "Patch cirúrgico sequencial paralelo" N=4 — **continua desqualificado**

P291 atinge N=4 cumulativo (P288+P289+P290+P291). P290 §8.5
**explicitamente desqualifica passos cumulativos triviais**
(anti-inflação por construção). **Decisão**: padrão continua
desqualificado para formalização.

### §8.6 — "Verificação anti-reflexão A.5'" — **NOVO N=1 inaugural**

P291 inaugura padrão **N=1**: secção A.5' obrigatória que:
1. Compara literalmente A.1.6 de N passos anteriores.
2. Verifica que A.0 foi produzido empiricamente.
3. Identifica pelo menos 1 elemento estructuralmente novo.
4. Decide sobre passo seguinte (procede ou intercala ortogonal).

**Reaplicações candidatas**: P292 (font) — A.5' deverá verificar
se `FontList` introduz paradigma consumer arquiteturalmente novo
(provável dado `FontList` é wrapper). Aguardar N≥3-4 para considerar
formalização ADR meta.

**Sinergia com P273.17 §0**: A.5' materializa a extensão implícita
do anti-padrão "over-formalização" para "over-automatização" —
mitigação operacional do risco "sequência reflexa".

---

## §9 — Próximos passos sugeridos (estado pós-P291)

Cobertura agregada estimada: inalterada (~64%). P291 não altera
contagem user-facing (variant arquitectural) mas:
- **Fecha 1/2 da assimetria residual P290 §5.6** (4/5 fechados;
  1/5 restante = `font`).
- **Inaugura A.5' anti-reflexão** — primeira mitigação operacional
  do risco "sequência reflexa" P290 §7.
- **Confirma ADR-0098 com consumer per-line** (paradigma novo).
- **Confirma ADR-0099 com 2ª reaplicação pós-formalização**.

### Rank 1: completar fecho da assimetria

1. **`P-style-font-variant`** (S; P292) — adiciona
   `Style::Font(FontList)`. **Cuidado especial em A.1.2**: `FontList`
   é wrapper `Vec<FontFamily>` que provavelmente **não é `Copy`**.
   Se confirmado, opção A.2 (a) `Font(FontList)` pode forçar `Style`
   a perder `Copy` — registar como divergência ou usar `Arc<FontList>`.
   A.5' deverá identificar paradigma consumer arquiteturalmente
   novo (font selection via `FontVariant` resolver) ou intercalar
   ortogonal.

### Rank 2-4: features médias

2. **`P-math-accent-cancel`** (XS+S; Math 40% → 50%).
3. **`P-curve-geometry`** (S-M; ADR-0078 sub-fase b).
4. **`P-footnote-cluster`** (M; Model 60% → 70%).

### Padrões pós-P291 (operacional)

- **ADR-0098 + ADR-0099 vigentes** — Fase A inclui A.0 obrigatória.
- **Padrão §8.6 A.5' anti-reflexão N=1 inaugural** — P292 deve
  reaplicar; aguardar N≥3-4 para considerar formalização.
- **Padrão §8.3** "refutação pragmática" N=5 estável.
- **Padrão §8.5** "patch cirúrgico sequencial" N=4 estável
  desqualificado.

### Marco P292 antecipado

P292 (font) é o **último passo cumulativo da série P288-P292** se
proceder. Pós-P292, assimetria residual fechada 5/5 (0 restantes).
Sequência cirúrgica terminará naturalmente — não há mais campos
em `StyleDelta` sem variant `Style` correspondente.

---

## §10 — Referências cross-passos

- **P102 ADR-0040** — `#set text(...)` activation; precedente
  parse-driven path.
- **P127 ADR-0038** — `Length` type materializado.
- **P128 / P138** — Leading captured em `text` (divergência
  consciente vs vanilla `par`).
- **P136 Fase A DEBT-52** — Top-wins propagation; paradigma
  `TextStyle` capture.
- **P138** — `flush_line` peek `current_line` para leading — consumer
  reusado sem alteração em P291.
- **P281** — Unificação β-completa stream-builders (origem
  ADR-0098).
- **P282 §1.1** — Auditoria empírica (N=1 ADR-0098).
- **P285-P290 §8.1/§8.2** — Aplicações cumulativas ADR-0098 +
  ADR-0099.
- **P288** — Formalização ADR-0098 (N=5).
- **P289** — Formalização ADR-0099 (N=5).
- **P290 §7 risco quinário** — Antecipou "sequência reflexa"; P291
  §A.5' é primeira mitigação operacional activa.
- **P290 §8.5** — Desqualifica passos cumulativos triviais (anti-
  inflação por construção); P291 §8.5 cita.
- **P291 §A.5' / §5.1 / §8.6** — **Inaugura padrão "verificação
  anti-reflexão"** N=1.
- **ADR-0026 / ADR-0026-R1** — Enum vs vtable divergência.
- **ADR-0029** — Pureza física L1.
- **ADR-0038** — Style enum divergência intencional.
- **ADR-0054** — Scope graded (assimetria residual 1 field).
- **ADR-0065** — Inventariar-primeiro.
- **ADR-0085** — Diagnóstico imutável (45º consumo: P291 + 44 anteriores).
- **ADR-0093** — Meta-metodologia evolução ADRs.
- **ADR-0094** — Meta-operacional specs.
- **ADR-0098** — Single source of truth como invariante anti-bug
  (formalizada P288; **8º passo consecutivo de aplicação em P291**).
- **ADR-0099** — Activação posterior de feature graded (formalizada
  P289; **2ª reaplicação pós-formalização em P291**).
- **P273.17 §0** — Anti-padrão over-formalização (vigente; nenhuma
  ADR meta promovida em P291; **estende-se implicitamente a
  "over-automatização"** — A.5' é mitigação).

---

*P291 fecha 1/2 da assimetria residual P290 §5.6 ao adicionar
`Style::Leading(Length)` ao enum `Style` (8 → 9 variants) com arm
correspondente em `StyleChain::push_styles`. 2ª fonte de entrada
para `delta.leading` materializada — caminho parse-driven `eval_set_rule`
(P128/P138) + consumer P138 (`cursor.rs:119-128` em `flush_line` peek
do último `FrameItem::Text` da current_line) preservados sem
alteração. **Hash `export.rs 66cb8ac3` preservado bit-exact pelo 8º
passo consecutivo** (P282+P285+P286+P287+P288+P289+P290+P291) —
ADR-0098 valida-se com **8 paradigmas consumer arquiteturalmente
distintos**. **A.5' anti-reflexão inaugurada** como mitigação
operacional do risco "sequência reflexa" P290 §7 — comparação
literal de A.1.6 P288-P291 revela 4 paradigmas distintos (cross-
module / TextStyle method / per-glyph / **per-line via peek
`current_line`**); **sequência NÃO é rubber-stamp**; P292 pode
prosseguir sem intercalação ortogonal forçada. Anti-padrão P273.17
§0 estende-se implicitamente a "over-automatização" — A.5' é a
mitigação operacional. **Nenhuma ADR meta nova promovida** —
ADR-0098 (N=8 cumulativo) e ADR-0099 (N=7 cumulativo, 2ª reaplicação
pós-formalização) reaplicam. Padrão §8.6 "verificação anti-reflexão
A.5'" inaugurado N=1; aguardar N≥3-4 para considerar formalização.
Nenhum bug latente colateral detectado (A.5 verificou 5 fronteiras
incluindo leading negativo `-1pt` propaga gracefully via paridade
vanilla line collapse parcial). 11 testes P291 verdes (1 entity
catalog + 10 layout); baseline 2 773 → 2 783 (+10). Assimetria
residual passa de 3/5 (P290) para **4/5 fechados pós-P291** —
restam **1/5** (`font`) como passo próprio candidato P292
não-reservado. Honestidade epistémica preservada: variant atómico
(não rico); padrão N=4 inalterado. Divergência arquitectural
consciente registada literalmente no doc-comment (vanilla `par`↔
cristalino `text` por conveniência temporária; preservada per
spec §5 não-objectivo).*
