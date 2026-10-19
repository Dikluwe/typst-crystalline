# Passo 292 — Relatório consolidado

**Tema**: Materialização da frente `P-style-font-variant` —
**último passo da série cirúrgica P288-P292**. Adiciona variant
`Style::Font(FontList)` ao enum `Style`, fechando **5/5 da
assimetria residual P289 §5.6**. Decisão arquitectural genuinamente
não-trivial (A.2): `FontList` não é `Copy` → `Style` perde `Copy`
derive — inventário literal A.2.0 confirma perda inofensiva.
**Marco arquitectural**: pós-P292, série cumulativa termina
naturalmente; próximo passo será ortogonal por construção.

**Data**: 2026-05-19
**Branch**: Tekt
**Magnitude**: S (modificação cirúrgica: +1 variant + 1 derive
change `Copy` → removed + 1 cascade arm com `.clone()`; **0 ADRs
novas** — reaplicações de ADR-0098/0099 sem nova promoção; +11
testes; **9º passo consecutivo** a preservar hash `export.rs`).

---

## §1 — Validação contra spec (critérios §4)

| Critério §4 | Estado |
|---|---|
| `cargo test --workspace` verde | ✅ **2 793** testes (baseline P291: 2 783 → **+10**) |
| Delta esperado ~+10-15 | ✅ +10 (dentro do range) |
| `crystalline-lint` zero violations | ✅ Confirmado |
| Hash L0 `style.md` muda (+1 variant em B.3) | ✅ `899414b5 → eb0d8fd9` (`style.rs → dfe68a89`) |
| Hash L0 `content.md` preserved | ✅ Preservado |
| Hash L0 `stdlib.md` preserved | ✅ Preservado |
| Hash L0 `export.rs` **condicional** preserved se A.4 → (i)/(iv) | ✅ **`66cb8ac3` preservado bit-exact pelo 9º passo consecutivo** — A.0 não-trivial confirmou paradigma `TextStyle` capture (P136) com emit consumer real (multifont `Tf` operator); ADR-0098 vigente |
| **Regressão bit-exact validada** — P140B/P141/P146 parse-driven | ✅ 2 783 testes P291 preservados; **perda de `Style: Copy` confirmada inofensiva** (A.2.0 antecipou empiricamente) |
| Tabela B.3 actualizada com `Font(FontList)` (10º variant) + nota assimetria fechada 5/5 | ✅ Confirmado |
| Tabela B.4 linha 354 com nota cruzada P292 | ✅ Confirmado |
| Diagnóstico A.0+A.1+A.2+A.3+A.4+A.5+A.5' produzido | ✅ `diagnostico-style-font-passo-292.md` (**6 secções** + 8 sub-secções A.1 + diagrama de fluxo + **A.5' N=2** reaplicação P291) |
| **Promoção ADR meta condicional ao gatilho disparar genuinamente** | ✅ **Nenhuma promoção** — A.2.3 confirma refutação estructuralmente forçada (não genuína); §8.3 N=5 estável; §8.6 N=2 < tentativo N≥3-4 |
| Bug latente colateral (se descoberto em A.5) registado | ✅ A.5 — **invariante estructural descoberta** (`FontList::new(vec![])` retorna `None` — non-empty by construction; não é bug, é design) |
| **Marco arquitectural registado**: série P288-P292 termina | ✅ Documentado em L0 + tabelas + diagnóstico + teste simbólico |

**Conformidade**: 13/13 critérios estritos cumpridos. P292 é o
**último passo da série cumulativa P288-P292**.

---

## §2 — Resumo factual

### §2.1 — Variant atómico + cascade arm + derive change

**Antes P292**:
- `Style` enum: 9 variants pós-P291 (Bold/Italic/Size/Fill/HeadingLevel/Lang/Weight/Tracking/Leading).
- `Style` derive: `#[derive(Debug, Clone, Copy, PartialEq)]`.
- `StyleDelta.font: Option<FontList>` parseado-mas-inerte do ponto
  de vista de `Styles` collection (escrito apenas via parse-driven
  `eval/rules.rs:395+` desde P140B/P141/P146).
- 4 consumers activos de `chain.font()`: top-wins propagation P136,
  TextStyle capture, **multifont emit P140B+P141+P146**
  (`export.rs:2169-2174`), FontBook lookup (implícito).
- Assimetria Tabela B.3 (9 variants) vs B.4 (10 fields) = 1 field
  residual (`font`).

**Pós-P292**:
- `Style` enum: **10 variants** (+`Font(FontList)`).
- `Style` derive: `#[derive(Debug, Clone, PartialEq)]` — **`Copy`
  removido** por `FontList: !Copy`.
- **2ª fonte de entrada** para `delta.font` via
  `Content::Styled(body, Styles::from_iter([Style::Font(FontList::single("Inter"))]))`.
- Caminho parse-driven **intacto** — bit-exact preservado.
- Consumers reusados sem alteração — **2ª aplicação prática de
  ADR-0098 com emit consumer real** (P290 foi a 1ª; P292 reforça).
- **0 ADRs novas** — ADR-0098 (N=9) + ADR-0099 (N=8) reaplicam.
- **Assimetria residual fechada 5/5** — série cirúrgica P288-P292
  termina naturalmente.

### §2.2 — Cobertura de mudança

| Sítio | Tipo | Mudança |
|---|---|---|
| `01_core/src/entities/style.rs:21` | import | +`FontList` |
| `01_core/src/entities/style.rs:25-32` | derive change | `Copy` removido; doc-comment justifica |
| `01_core/src/entities/style.rs:95-122` | extensão enum | +1 variant `Font(FontList)` com doc-comment ~27 LOC |
| `01_core/src/entities/style.rs:175-195` | test extensão | catalog actualizado 9 → 10 variants |
| `01_core/src/entities/style_chain.rs:174-189` | cascade arm | +1 arm `Style::Font(f) => delta.font = Some(f.clone())` (+15 LOC com doc) |
| `00_nucleo/prompts/entities/style.md` | L0 | +10º variant em B.3 + nota marco arquitectural série fechada + derive change documentado |
| `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | L0 cobertura | B.3 +1 linha; B.4 linha 354 nota P292; footnote ⁷⁸ ~120 LOC com marco arquitectural |
| `00_nucleo/diagnosticos/diagnostico-style-font-passo-292.md` | Diagnóstico Fase A | ~360 LOC ficheiro novo (6 secções A.0-A.5+A.5' N=2) |
| `01_core/src/engine/layout/tests.rs:10770+` | testes P292 | ~140 LOC (11 testes em `p292_style_font_tests` mod incluindo marco simbólico) |

Total: **2 sítios L1 produção + 2 ficheiros L0 documentação + 1
diagnóstico + 1 ficheiro de testes**. **Zero ficheiros tocados em
L3** (export.rs preservado bit-exact pelo 9º passo consecutivo).

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
    Style::Leading(l)      => delta.leading = Some(*l),               // P291
    // P292 — distinção sintáctica crítica: `.clone()` em vez de `*f`
    // por `FontList: !Copy`. Paralelo a `chain.font()` que clona em
    // walk (style_chain.rs:264). Style enum perde `Copy` derive
    // (A.2.0 confirma inofensivo).
    Style::Font(f)         => delta.font = Some(f.clone()),
}
```

**Match continua exaustivo** — compilador detecta omissões por
construção (defesa cumulativa P288-P292). **Cascade arm pós-P292
tem 10 arms** — 5 originais + 5 P288-P292.

### §2.4 — Distintivo arquitectural P292 vs P288-P291

P292 é **o único da série** onde:

1. **Tipo armazenado não é `Copy`** — `FontList(Vec<FontFamily>)`.
2. **`Style` enum perde derive `Copy`** — primeira vez na série.
3. **Cascade arm usa `.clone()`** em vez de `*` — distinção
   sintáctica única.
4. **Decisão A.2 não-trivial** — 6 opções (a)-(f) com trade-offs
   reais; default sugerido pela spec era condicional.
5. **A.2.0 novo inventário** — `grep "*style"` literal para confirmar
   perda de Copy inofensiva (template para passos futuros).
6. **Paradigma consumer "indirect resolution via FontBook" 2
   layers** — TextStyle capture + lookup `fonts.iter().position(|f|
   match name)`; arquitecturalmente novo vs P288-P291.

---

## §3 — Fase A — decisões registadas (`diagnostico-style-font-passo-292.md`)

### §3.1 — A.0 não-trivial confirmada empiricamente

`grep "font" 03_infra/src/export.rs` revelou 3+ hits:

| Linha | Conteúdo | Classificação |
|---|---|---|
| `:2155` | `"{q_open}BT\n/{font_ref} {:.1} Tf\n..."` | Helvetica path; font_ref via style.bold/italic |
| `:2163` | `"BT\n/F1 {:.1} Tf\n..."` | CIDFont single (hardcoded /F1) |
| `:2169-2174` | `let fi = style.font.as_ref().and_then(\|fl\| ...); format!("BT\n/F{} {:.1} Tf\n...", fi+1, ...)` | **Multifont path lê `style.font`** (paradigma `TextStyle` capture P136) |

**Padrão confirmado**: emit multifont consome via `style.font`
(linha 2169), **não** via `chain.font()` directo. **Paradigma P136
vigente** — ADR-0098 OK. **2ª aplicação prática de ADR-0098 com
emit consumer real** (P290 foi a 1ª).

### §3.2 — A.1 inventário literal (8 sub-secções com A.1.2 CRÍTICO)

| Sub-secção | Achado decisivo |
|---|---|
| A.1.1 — `Style` enum pós-P291 | 9 variants confirmadas (P292 → 10) |
| **A.1.2 — `StyleDelta.font`** | `Option<FontList>`; **`pub struct FontList(Vec<FontFamily>)` — NÃO é `Copy`**; derive `Debug, Clone, PartialEq, Eq, Hash` |
| A.1.3 — `push_styles` cascade | match exaustivo sobre 9 variants; arm `Font(f)` com `.clone()` |
| A.1.4 — `delta.font` write | **Único** site: `eval/rules.rs:395+` (parse aceita Str/Array; rejeita Dict per ADR-0054bis) |
| A.1.5 — `delta.font` read | `chain.font()` walk **com `.clone()`** (style_chain.rs:264; comentário documenta divergência) |
| A.1.6 — Consumers | **4 activos** (top-wins, TextStyle capture, **multifont emit real**, FontBook resolver implícito) |
| A.1.7 — `FrameItem::Text` emit | Multifont path lê `style.font.as_ref()` directamente (paradigma P136) |
| A.1.8 — Diagrama de fluxo | Distintivo: termina em `/F{i+1} Tf` literal + **indirect resolution via FontBook** |

### §3.3 — A.2 decisão genuinamente não-trivial (primeira vez na série)

| Opção | Veredicto |
|---|---|
| **(a) `Font(FontList)` com perda Copy** | ✅ **Escolhida** — `Style: Copy` perdido mas A.2.0 confirma inofensivo |
| (b) `Font(Arc<FontList>)` | ❌ Indirecção desnecessária |
| (c) `Font(Box<FontList>)` | ❌ Box: !Copy igual a (a) |
| (d) `Font(EcoString)` | ❌ Divergência semântica grave (perde array fallback) |
| (e) Aceitar perda explícita | = (a) — escolhida implicitamente |
| (f) `Font(EcoVec<EcoString>)` | ❌ Adiciona dependência sem ganho |

**A.2.0 inventário literal** (template novo para passos futuros):
`grep "*style" 01_core/src/` → **0 hits funcionais**. Ninguém
desreferencia `Style` directamente. `Style: Copy` é **apenas trait
derive** — não é usado dependencialmente. **Perda de Copy é
estructuralmente inofensiva**.

### §3.4 — A.2.3 honestidade epistémica crítica

**Refutação significativa do paradigma "todos Copy"?** Sim, mas
**estructuralmente forçada** por tipo `FontList: !Copy` já
existente (P136 estabeleceu). **Não conta como refutação pragmática
N=6 do padrão §8.3** porque não há decisão genuína da spec — o tipo
já existia e a estructura é a única opção sensata. §8.3 N=5
**estável** — sem promoção.

### §3.5 — A.3 cascade — opção (α) com `.clone()`

`delta.font = Some(f.clone())` — paridade aos 9 arms existentes
**com `.clone()` em vez de `*f`** por necessidade material.
Distinção sintáctica única na série P288-P292.

### §3.6 — A.4 emit — opção (i) confirmada empiricamente

| Critério | Evidência |
|---|---|
| `export.rs` consulta `style.font`? | A.0.1: 3+ hits (linha 2169) — via `TextStyle` capture |
| `export.rs` consulta `chain.font()` directo? | Não (0 hits) |
| `FrameItem::Text` precisa novo field `font`? | Não — `TextStyle.font` já existe (P140B/P141/P146) |
| Reflectors directos? | Não |

**Hash `export.rs` preservado pelo 9º passo consecutivo**.

### §3.7 — A.5 detecção bugs (5 cenários incluindo invariante FontList vazia)

| Cenário | Construção | Resultado |
|---|---|:---:|
| Single | `FontList::single("Inter")` | ✅ |
| Multi (fallback) | `FontList::new(vec![FF("Inter"), FF("Arial"), FF("Helvetica")])` | ✅ |
| **Vazia (invariante)** | `FontList::new(vec![])` retorna **`None`** (non-empty by construction; réplica vanilla "font fallback list must not be empty") | ✅ Invariante estructural — não é bug |
| Last-write wins | 2 `Style::Font` consecutivos | ✅ Último ganha |
| Missing font name | `FontList::single("NonExistentFontXYZ")` | ✅ Propaga; resolution defer ao FontBook |

**Nenhum bug latente detectado**. **Invariante "FontList non-empty
by construction" descoberta** durante A.5 — não é bug, é design.
Padrão §8.4 P288 permanece **N=1 estável**.

### §3.8 — A.5' anti-reflexão N=2 (reaplicação P291 §A.5' inaugural)

**5 paradigmas consumer arquiteturalmente distintos** identificados:

| Passo | Paradigma consumer principal |
|---|---|
| P288 lang | Cross-module (eval+lang+layout) |
| P289 weight | TextStyle method (`faux_bold_stroke_pt`) |
| P290 tracking | Per-glyph + Tc emit |
| P291 leading | Per-line via peek `current_line` |
| **P292 font** | **2 layers — TextStyle capture + indirect resolution via FontBook** |

**2 elementos estructuralmente novos identificados** (vs P291 N=1
com 1 elemento):
1. **A.2 não-trivial em si** — `Style` perde `Copy` derive (primeira
   vez na série); inventário A.2.0 obrigatório.
2. **Paradigma "indirect resolution via global registry"**:
   `FontBook::select` resolve `FontFamily` → font index pré-emit;
   arquitecturalmente novo vs P288-P291.

**Decisão sobre passo seguinte**: P293 será **ortogonal por
construção** (não por decisão) — assimetria fechada 5/5, não há mais
campos para reaplicação cumulativa.

### §3.9 — Riscos mitigados

| Risco | Status |
|---|---|
| `Style: !Copy` impacto cumulativo | ✅ A.2.0 0 call sites — inofensivo |
| Tipo inesperado | ✅ A.1.2 confirma `Option<FontList>` |
| Emit idiossincrático | ✅ A.4.1 paradigma P136 confirmado |
| FontList vazia bug | ✅ A.5 invariante estructural — não é bug |
| Sequência reflexa | ✅ A.5' N=2 com 2 elementos novos |
| Promoção indevida | ✅ A.2.3 confirma forçada (não genuína) |

---

## §4 — Testes adicionados (+11)

| Local | Quantidade | Cobertura |
|---|---:|---|
| `entities/style.rs` (mod tests) | 1 | Catalog test 9 → 10 variants (`Style::Font(FontList::single("Inter"))` incluído) |
| `engine/layout/tests.rs` (`p292_style_font_tests`) | 10 | Variant ctor + PartialEq; `push_styles` cascade com `.clone()`; `Styled` injection + TextStyle propagation; last-write wins; **5 cenários A.5** (single/multi/non-empty invariant/missing/textstyle capture); **1 marco simbólico "série P288-P292 fechada"** (constroi `Styles` com todos os 5 variants e verifica `chain.X().is_some()` para cada um) |
| **Total** | **11** | (1 entity + 10 layout) |

**Resultado**: 11/11 verdes (`cargo test --lib p292`). Delta
workspace **+10** (catalog test substitui o anterior P291 9 → 10;
1 LOC novo).

### §4.1 — Teste simbólico "série fechada"

```rust
#[test]
fn p292_marco_arquitectural_serie_p288_a_p292_fechada() {
    use crate::entities::lang::Lang;
    use crate::entities::layout_types::Length as L;

    let all_5 = Styles::from_iter([
        Style::Lang(Lang::ENGLISH),                              // P288
        Style::Weight(700),                                       // P289
        Style::Tracking(L::pt(0.5)),                              // P290
        Style::Leading(L::em(0.65)),                              // P291
        Style::Font(FontList::single(EcoString::from("Inter"))),  // P292
    ]);
    let chain = StyleChain::empty().push_styles(&all_5);
    assert!(chain.lang().is_some());
    assert!(chain.weight().is_some());
    assert!(chain.tracking().is_some());
    assert!(chain.leading().is_some());
    assert!(chain.font().is_some());
}
```

Este teste é **simbólico** — atesta que os 5 variants P288-P292
todos integram correctamente na cascade pré-existente e a chain
resolve todos via accessors paralelos.

---

## §5 — Observações pragmáticas

### §5.1 — Marco arquitectural: série cirúrgica P288-P292 termina naturalmente

**5 passos cumulativos consecutivos** materializaram 5 variants
`Style` correspondentes aos 5 fields `StyleDelta` sem representação
em `Styles` collection:

| Passo | Variant adicionado | Field `StyleDelta` activado | Paradigma consumer |
|---|---|---|---|
| P288 | `Lang(Lang)` | `lang: Option<Lang>` | Cross-module |
| P289 | `Weight(u16)` | `weight: Option<u16>` | TextStyle method |
| P290 | `Tracking(Length)` | `tracking: Option<Length>` | Per-glyph + Tc emit |
| P291 | `Leading(Length)` | `leading: Option<Length>` | Per-line via peek |
| **P292** | **`Font(FontList)`** | **`font: Option<FontList>`** | **2 layers — TextStyle + FontBook lookup** |

**Pós-P292**: não há mais campos `StyleDelta` ortogonais para
reaplicação cumulativa. Assimetria Tabela B.3 vs B.4 **fechada
estructuralmente 5/5**. **Próximo passo (P293) será ortogonal por
construção** — não há decisão a tomar sobre continuar a série
porque a série terminou naturalmente por exaustão da assimetria.

Candidatos P293 listados no relatório P291 §9: math-accent-cancel
(XS+S), curve-geometry (S-M), footnote-cluster (M).

### §5.2 — Primeira decisão arquitectural genuinamente não-trivial da série

P288-P291 todas escolheram opção (a) trivial paralela porque o tipo
armazenado em `StyleDelta` era `Copy` (Lang, u16, Length). **P292 é
diferente**: `FontList(Vec<FontFamily>)` **não é Copy**.

A spec P292 §A.2 enumera **6 opções** (a)-(f) com trade-offs reais.
**Não há default sugerido** — decisão genuína condicional a A.1.2.

**Decisão final**: opção (a) `Font(FontList)` com perda explícita
de `Style: Copy` derive. **Inventário literal A.2.0** (novo na
série) confirma 0 call sites afectados — perda estructuralmente
inofensiva.

Isto valida o critério "decisão genuína sempre que possível, default
trivial quando estructuralmente forçada" — P288-P291 foram trivial
forçadas (todos `Copy`); **P292 foi genuína** porque havia trade-off
real.

### §5.3 — A.2.0 inventário literal como template novo

A.2.0 ("Inventário call sites `Style: Copy`") é **secção nova
inaugurada em P292**. Procedimento:
1. `grep "*style" 01_core/src/ -r` literal.
2. Classificar cada hit por necessidade real de Copy.
3. Decisão sobre perda de derive baseada em evidência empírica.

**Resultado P292**: 0 hits funcionais — perda inofensiva confirmada
empiricamente, não assumida.

**Template para passos futuros**: sempre que decisão arquitectural
envolver perda de trait derive, A.2.0 deve confirmar empiricamente
o impacto cumulativo antes de prosseguir.

### §5.4 — Hash `export.rs` preservado pelo 9º passo consecutivo

Sequência cumulativa:

| Passo | Razão preservação |
|---|---|
| N=1: P282 | Auditoria empírica refutou 6/6 suspeitas |
| N=2: P285 | Alteração simétrica via helper único `line_rg_prefix` |
| N=3: P286 | Reuso `FrameItem::Line` sem modificação |
| N=4: P287 | Consumer reusa `Content::Text` |
| N=5: P288 | `Style::Lang` extende parse sem tocar emit (formalização ADR-0098) |
| N=6: P289 | `Style::Weight` aplica ADR-0098 directamente |
| N=7: P290 | `Style::Tracking` — primeira com emit consumer real |
| N=8: P291 | `Style::Leading` — primeiro com consumer per-line via peek |
| **N=9: P292** | **`Style::Font` — primeiro com derive change + indirect resolution paradigm** |

Cada passo cumulativo **reforça** a invariante ADR-0098. P292 é o
9º passo consecutivo e cobre **3 paradigmas distintos de consumer
em emit** (P290 per-glyph, P291 per-line via peek, P292 indirect
resolution via FontBook) — todos compatíveis com ADR-0098 vigente.

### §5.5 — A.5' N=2 confirma robustez do padrão §8.6 P291

P291 inaugurou A.5' anti-reflexão N=1. **P292 N=2 confirma**:
- Verificação anti-reflexão **continua a identificar elementos
  estructuralmente novos** mesmo na 2ª aplicação.
- O padrão **não degenera com a sua própria reaplicação** — porque
  a inspecção literal sempre revela diferenças factuais.

**P292 N=2 identifica 2 elementos novos** (vs P291 N=1 com 1
elemento) — mais robusto.

§8.6 padrão **N=2 cumulativo**; tentativo N≥3-4 para formalização.
P293 ortogonal terá A.5' diferente (não comparação cumulativa P288-
P293) — pode terminar a sequência §8.6 naturalmente.

### §5.6 — Nenhuma ADR meta nova promovida (P273.17 §0 vigente)

Padrões cumulativos pós-P292:
- ADR-0098: **N=9** (reforço)
- ADR-0099: **N=8** (reaplicação)
- §8.3 "refutação pragmática": **N=5 estável** (A.2.3 confirma
  forçada)
- §8.4 "bug latente fixed": **N=1 estável**
- §8.5 "patch cirúrgico sequencial": **N=5 desqualificado**
  (anti-inflação)
- §8.6 "A.5' anti-reflexão": **N=2 cumulativo**

**Decisão**: nenhuma ADR meta promovida. P273.17 §0 anti-padrão
"promoção mecânica sem gatilho novo" vigente. **Anti-padrão
estende-se implicitamente a "over-automatização"** — A.5' é
mitigação operacional.

### §5.7 — Honestidade epistémica: variant atómico (não rico)

Padrão N=4 "variant rico com `body` + cosméticos opcionais"
**inalterado** pelo P292. `Style::Font(FontList)` é variant atómico
com 1 campo required (Vec é container, mas o variant em si é leaf).

### §5.8 — Invariante FontList non-empty by construction descoberta

Cenário A.5 "FontList vazia" revelou **invariante estructural**:
`FontList::new(vec![])` retorna `Option<Self> = None`. **Não é bug**
— réplica semântica vanilla "font fallback list must not be empty"
documentada literalmente em `font_list.rs:55+63`.

P292 §A.5.1 ajustou o teste para validar a invariante:
```rust
let attempt = FontList::new(vec![]);
assert!(attempt.is_none(),
    "FontList::new(vec![]) deve retornar None — non-empty by construction");
```

Este teste **atesta a invariante estructural** — não permite
regressões futuras (e.g. alguém alterar o construtor para aceitar
vazio).

---

## §6 — Métricas

| Métrica | Valor |
|---------|-------|
| LOC L1 produção | ~40 (+1 import; +1 variant 27 LOC com doc; +1 arm cascade 15 LOC com doc; +1 LOC test catalog; +1 derive change) |
| LOC L3 produção | **0** (zero impacto em export.rs — hash preservado pelo 9º passo) |
| LOC L0 modificado | ~500 (`style.md` +30 + nota marco arquitectural; `cobertura.md` B.3+B.4+footnote⁷⁸ ~120; `diagnostico-style-font-passo-292.md` ~360 ficheiro novo) |
| Testes adicionados | 11 (1 entity catalog + 10 layout: 4 ctor/cascade/inject/last-write + 5 cenários A.5 + 1 marco simbólico) |
| Testes baseline P291 | 2 783 preserved bit-exact |
| Testes pós-P292 | **2 793** |
| Hash L0 `style.md` | `899414b5 → eb0d8fd9` (`style.rs → dfe68a89`) |
| Hash L0 `content.md` | **inalterado** |
| Hash L0 `stdlib.md` | **inalterado** |
| Hash L0 `export.rs` | **`66cb8ac3` preservado** (**9º passo consecutivo**) |
| Lint | zero violations |
| `Style` variants | 9 → **10** (+`Font(FontList)`) |
| `Style` derives | `Debug, Clone, Copy, PartialEq` → **`Debug, Clone, PartialEq`** (-Copy) |
| Cascade arms `push_styles` | 9 → **10** (+1 LOC com `.clone()`) |
| Caminhos de entrada para `delta.font` | 1 → **2** (parse + Style::Font) |
| Fechos de assimetria B.3↔B.4 | 4/5 (P288-P291) → **5/5** (+ P292 font) — **fechada completamente** |
| Pendências resolvidas | **1** (1/1 final assimetria residual; **série P288-P292 termina**) |
| **ADRs novas** | **0** (reaplicações cumulativas sem promoção) |
| **Marco arquitectural** | ✅ Registado em L0 + tabelas + diagnóstico + teste simbólico |

---

## §7 — Conformidade Cristalina

- ✅ **ADR-0029 pureza física L1**: `Style::Font(FontList)` é
  variant atómico; cascade arm é função pura sem I/O.
- ✅ **ADR-0038 Style enum divergência intencional**: P292 estende
  o enum dentro do mesmo paradigma; derive change documentado.
- ✅ **ADR-0040 `#set text` activation P102**: caminho parse-driven
  preservado intacto.
- ✅ **ADR-0054 scope graded**: `font` dict (gap 8) continua
  scope-out per ADR-0054bis condicional.
- ✅ **ADR-0054bis** (font dict; gap 8 DEBT-52): **respeitada
  literalmente** — parse aceita Str/Array, rejeita Dict.
- ✅ **ADR-0055bis** (FontVariant variant-aware): **respeitada** —
  não materializada (passo distinto).
- ✅ **ADR-0065 inventariar-primeiro**: Fase A obrigatória produziu
  **6 secções A.0-A.5 + A.5'** + secção nova **A.2.0** (inventário
  call sites Copy).
- ✅ **ADR-0085 diagnóstico imutável**:
  `diagnostico-style-font-passo-292.md` produzido.
- ✅ **ADR-0093 meta-metodologia evolução ADRs**: sem nova promoção
  — A.2.3 confirma refutação estructuralmente forçada (não genuína).
- ✅ **ADR-0098** (P288): **9º passo consecutivo** a preservar
  `export.rs` — 3º paradigma consumer em emit (P290 per-glyph + P291
  per-line + **P292 indirect resolution**).
- ✅ **ADR-0099** (P289): **3ª reaplicação pós-formalização** —
  confirma robustez.
- ✅ **Anti-padrão over-formalização P273.17 §0**: **nenhuma ADR
  meta promovida** apesar de potencial §8.3 N=6 e §8.6 N=2.
- ✅ **Honestidade epistémica reforçada §A.2.3**: refutação
  estructuralmente forçada **não conta** como genuína; §8.3 N=5
  estável.
- ✅ **Bit-exact regression preservada por construção**: caminho
  parse-driven `eval_set_rule` intacto; 2 783 testes pré-P292
  preserved.
- ✅ **Inventário literal A.2.0 inaugural**: template novo para
  passos futuros que envolvam decisão de trait derives.

---

## §8 — Padrões emergentes (cumulação sem novas promoções; marco arquitectural)

### §8.1 — ADR-0099 reaplicada N=8 cumulativo (3ª pós-formalização)

ADR-0099 ("Activação posterior de feature graded") formalizada em
P289 com 5 aplicações cumulativas. P290 foi 1ª reaplicação
pós-formalização. P291 foi 2ª. **P292 é a 3ª reaplicação
pós-formalização**:

| N | Passo | Tipo de activação |
|:---:|---|---|
| 1-5 | P285-P289 | Originais (formalização) |
| 6 | P290 | 1ª reaplicação (tracking) |
| 7 | P291 | 2ª reaplicação (leading) |
| **8** | **P292** | **3ª reaplicação (font) — última da série** |

### §8.2 — ADR-0098 confirmada com 3º paradigma emit consumer N=9 cumulativo

P282-P289 = 5 aplicações originais. P290 = 1ª com emit consumer
real (per-glyph). P291 = consumer per-line (sem emit real). **P292
= 3º paradigma consumer com emit real (indirect resolution via
FontBook)**:

| N | Passo | Aplicação | Paradigma consumer |
|:---:|---|---|---|
| 1-5 | P282-P288 | Originais (formalização) | n/a a per-glyph |
| 6 | P289 | Weight (TextStyle method) | n/a emit |
| 7 | P290 | Tracking (per-glyph + Tc emit) | **1º paradigma emit** |
| 8 | P291 | Leading (per-line via peek) | sem emit real |
| **9** | **P292** | **Font (2 layers + indirect FontBook)** | **3º paradigma emit** |

ADR-0098 valida-se com **3 paradigmas emit distintos** + **5
paradigmas consumer distintos** — invariante operacional robusta
cross-paradigm.

### §8.3 — "Refutação pragmática" N=5 estável (refutação forçada não conta)

P287 atingiu N=5. P288-P291 todas trivial forçadas. **P292 tem
"refutação significativa estructuralmente forçada"** (perda de
`Style: Copy`) mas **A.2.3 explicitamente exclui** do padrão N=5:
não é refutação genuína da spec, é adaptação estructural forçada
por tipo já existente.

**Decisão metodológica**: distinção entre "refutação genuína vs
spec" (conta para §8.3) e "adaptação estructural forçada" (não
conta). P292 inaugura este critério distintivo.

§8.3 **permanece N=5 estável**. **Sem promoção**.

### §8.4 — "Bug latente fixed" N=1 estável (invariante descoberta não conta)

P288 inaugurou N=1 (NBSP fix). **P292 descobre invariante
estructural** (FontList non-empty by construction) — não é bug
latente, é design literal documentado.

**Distinção**: padrão §8.4 conta "bugs descobertos durante
materialização que requerem fix" (P288 NBSP). **Invariantes
estructurais descobertas** (que não requerem fix porque são por
design) **não contam**.

§8.4 **permanece N=1 estável**.

### §8.5 — "Patch cirúrgico sequencial paralelo" N=5 — **desqualificação confirmada**

P288+P289+P290+P291+P292 = 5 passos cumulativos. P290 §8.5
desqualificou padrão para formalização (anti-inflação por
construção). **P292 atinge N=5 mas continua desqualificado** —
**série termina sem promoção** porque toda a série é fundamentalmente
N=5 por design.

Padrão §8.5 **encerra com a série**: não há "passo cumulativo
adicional" pós-P292 porque a assimetria está fechada. **N=5
permanente desqualificado**.

### §8.6 — A.5' anti-reflexão N=2 cumulativo (P291+P292)

P291 inaugurou N=1 com 1 elemento estructuralmente novo (peek
`current_line` per-line). **P292 N=2** com **2 elementos
estructuralmente novos** identificados:
1. A.2 não-trivial em si (Style perde Copy).
2. Paradigma "indirect resolution via FontBook".

**P292 N=2 confirma robustez do padrão** — verificação
anti-reflexão não degenera com reaplicação.

P293 será ortogonal (não cumulativo) — A.5' P293 terá comparação
diferente (não cross-P288-P293 mas vs padrão específico do P293).
Padrão §8.6 **N=2 cumulativo**; tentativo N≥3-4 para formalização.
**Aguardar P294+ para considerar promoção**.

### §8.7 — **NOVO: Inventário literal A.2.0 como verificação empírica obrigatória** — N=1 inaugural

P292 inaugura padrão **N=1**: secção A.2.0 obrigatória sempre que
decisão arquitectural envolva perda de trait derive:
1. `grep` literal por uso dependente do trait.
2. Classificar cada hit por necessidade real.
3. Decisão sobre derive change baseada em evidência empírica.

**Reaplicações candidatas**: futuros passos que considerem perder
`Copy`/`PartialEq`/`Hash` em tipos core. Aguardar N≥3 para
considerar formalização.

**Sinergia com A.5' anti-reflexão**: A.2.0 é variante "trait derive"
da verificação anti-reflexão — ambas são **mitigações empíricas**
de assumpções estructurais.

---

## §9 — Próximos passos sugeridos (estado pós-P292)

Cobertura agregada estimada: inalterada (~64%). P292 fecha
**marco arquitectural** sem alterar contagem user-facing.

### §9.1 — Série P288-P292 terminada

**Por construção**, não há mais "passo cumulativo paralelo" possível
— assimetria 5/5 fechada estructuralmente. **Próximo passo será
ortogonal por construção**, não por decisão.

### §9.2 — Rank 1-3: candidatos ortogonais (listados em P291 §9)

1. **`P-math-accent-cancel`** (XS+S; Math 40% → 50%) — primitives
   Math distintos.
2. **`P-curve-geometry`** (S-M; ADR-0078 sub-fase b) — Curve
   primitive.
3. **`P-footnote-cluster`** (M; Model 60% → 70%) — Footnote
   materialization.

### §9.3 — Padrões pós-P292 (operacional)

- **ADR-0098 + ADR-0099 vigentes** — Fase A inclui A.0 obrigatória.
- **§8.6 A.5' anti-reflexão N=2** — passos cumulativos future
  poderão atingir N≥3-4 se outra série emergir.
- **§8.7 A.2.0 inventário literal N=1 inaugural** — template para
  passos com decisão de trait derive.
- **Anti-padrão P273.17 §0 vigente** — extensão "over-automatização"
  documentada implicitamente.

### §9.4 — Marco do projecto

P292 marca **conclusão da 1ª série cirúrgica cumulativa explícita**
do projecto cristalino:
- 5 passos consecutivos (P288-P292).
- 5 variants adicionados ao enum `Style` (Lang/Weight/Tracking/
  Leading/Font).
- 5 paradigmas consumer arquiteturalmente distintos confirmados.
- 9 passos consecutivos preservando `export.rs` (P282-P292).
- 2 ADRs meta formalizadas (ADR-0098 P288, ADR-0099 P289).
- 0 ADRs meta promovidas em reaplicações (P290-P292).

**Lição metodológica**: séries cirúrgicas cumulativas podem
materializar features paralelas **sem inflação ADR** se cada passo:
1. Confirmar empiricamente o paradigma (Fase A obrigatória).
2. Reaplicar ADRs vigentes sem promoção (P273.17 §0 vigente).
3. Identificar elementos estructuralmente novos (A.5' anti-reflexão).
4. Terminar naturalmente por exaustão da assimetria.

---

## §10 — Referências cross-passos

- **P102 ADR-0040** — `#set text(...)` activation; precedente
  parse-driven path.
- **P136 Fase A DEBT-52** — `TextStyle` perdeu Copy por FontList;
  precedente para P292.
- **P140B / P141 / P146** — Multi-doc + array fallback parse;
  consumer reusado sem alteração em P292.
- **P281** — Unificação β-completa stream-builders (origem
  ADR-0098).
- **P282 §1.1** — Auditoria empírica (N=1 ADR-0098).
- **P285-P291 §8.1/§8.2** — Aplicações cumulativas ADR-0098 +
  ADR-0099.
- **P288** — Início da série cirúrgica; formalização ADR-0098.
- **P289** — Formalização ADR-0099.
- **P290 §7 risco quinário** — Antecipou "sequência reflexa".
- **P290 §8.5** — Desqualifica passos cumulativos triviais.
- **P291 §A.5' / §8.6** — Inaugurou padrão "verificação
  anti-reflexão" N=1.
- **P292 §A.2.0 / §8.7** — **Inaugura padrão "inventário literal
  trait derive"** N=1 (este passo).
- **P292 §A.5' N=2** — 2ª aplicação anti-reflexão (este passo).
- **ADR-0026 / ADR-0026-R1** — Enum vs vtable divergência.
- **ADR-0029** — Pureza física L1.
- **ADR-0038** — Style enum divergência intencional.
- **ADR-0054** — Scope graded.
- **ADR-0054bis** (gap 8 font dict) — Scope-out respeitado.
- **ADR-0055bis** (FontVariant variant-aware) — Scope-out respeitado.
- **ADR-0065** — Inventariar-primeiro.
- **ADR-0085** — Diagnóstico imutável (46º consumo: P292 + 45
  anteriores).
- **ADR-0093** — Meta-metodologia evolução ADRs.
- **ADR-0094** — Meta-operacional specs.
- **ADR-0098** — Single source of truth como invariante anti-bug
  (formalizada P288; **9º passo consecutivo de aplicação em P292**).
- **ADR-0099** — Activação posterior de feature graded (formalizada
  P289; **3ª reaplicação pós-formalização em P292**).
- **P273.17 §0** — Anti-padrão over-formalização (vigente; nenhuma
  ADR meta promovida em P292).

---

*P292 fecha **5/5 final da assimetria residual P289 §5.6** ao
adicionar `Style::Font(FontList)` ao enum `Style` (9 → 10 variants)
com arm correspondente em `StyleChain::push_styles` (paralelo
arquitectural a P288-P291 **com distinção sintáctica crítica**:
`.clone()` em vez de `*f` por `FontList: !Copy`). **Decisão
arquitectural genuinamente não-trivial pela primeira vez na série**:
`Style` enum perde `Copy` derive (`#[derive(Debug, Clone, Copy,
PartialEq)]` → `#[derive(Debug, Clone, PartialEq)]`) — **inventário
literal A.2.0 inaugural** confirma 0 call sites afectados (perda
estructuralmente inofensiva). 2ª fonte de entrada para `delta.font`
materializada; caminho parse-driven (P140B/P141/P146) + consumers
(top-wins, TextStyle capture, **multifont emit real**, FontBook
lookup) preservados sem alteração. **Hash `export.rs 66cb8ac3`
preservado bit-exact pelo 9º passo consecutivo** (P282+P285+P286+
P287+P288+P289+P290+P291+P292) — **3º paradigma emit consumer
distinto** confirmado (per-glyph P290, per-line P291, **indirect
resolution via FontBook P292**). **A.5' N=2 anti-reflexão** com
**2 elementos estructuralmente novos** identificados (vs P291 N=1
com 1 elemento) — sequência continua NÃO ser rubber-stamp.
**Nenhuma ADR meta nova promovida** — ADR-0098 (N=9) + ADR-0099
(N=8) reaplicam; A.2.3 confirma refutação estructuralmente forçada
(não genuína), §8.3 N=5 estável. **MARCO ARQUITECTURAL**: série
cirúrgica P288-P292 termina naturalmente — não há mais campos
`StyleDelta` ortogonais para reaplicação cumulativa. P293 será
**ortogonal por construção** (não por decisão) — candidatos
listados (math-accent-cancel, curve-geometry, footnote-cluster).
**Padrão §8.7 inaugurado N=1**: inventário literal A.2.0 como
verificação empírica obrigatória para passos com decisão de trait
derive. Invariante estructural descoberta em A.5: `FontList::new(
vec![])` retorna `None` (non-empty by construction; réplica vanilla
"font fallback list must not be empty") — não é bug, é design. 11
testes P292 verdes (1 entity catalog + 10 layout incluindo **marco
simbólico "série P288-P292 fechada"**); baseline 2 783 → 2 793
(+10). Assimetria 5/5 fechada; **série cumulativa termina**.*
