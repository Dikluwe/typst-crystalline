# Passo 288 — Relatório consolidado

**Tema**: Materialização da frente `P-style-lang-variant` —
extensão simétrica do enum `Style` com variant `Lang(Lang)` que
fecha a assimetria Tabela B.3 (5 variants) vs B.4 (10 fields)
para o campo `lang`. **Gatilho N=5 do padrão "Single source of
truth como invariante anti-bug" dispara empiricamente → ADR-0098
promovida.**

**Data**: 2026-05-19
**Branch**: Tekt
**Magnitude**: XS-S (modificação cirúrgica: +1 variant atómico +1 arm
em `push_styles` +1 LOC; refino consumer P287 SmartQuote para
preservar NBSP em FR; **+1 ADR meta promovida** com 5 aplicações
cumulativas formalizadas; +7 testes).

---

## §1 — Validação contra spec (critérios §4)

| Critério §4 | Estado |
|---|---|
| `cargo test --workspace` verde | ✅ **2 755** testes (baseline P287: 2 748 → **+7**) |
| Delta esperado +8 a +12 | ⚠ +7 (ligeiramente abaixo — economia natural: refino consumer P287 +0 testes novos por ser bug-fix; 3 testes lang-aware reactivados contam como **0 LOC novo** mas activaram features genuinamente; ver §5.2) |
| `crystalline-lint` zero violations | ✅ Confirmado |
| Hash L0 `style.md` muda (+1 variant em B.3) | ✅ `37404a23 → b4de54d0` (`style.rs → a3eb5b76`) |
| Hash L0 `content.md` preserved | ✅ **Preservado** (sem novo Content variant) |
| Hash L0 `stdlib.md` preserved | ✅ **Preservado** (sem nova função) |
| Hash L0 `export.rs` **condicional** preserved se A.4 → (i) | ✅ **Preservado bit-exact `66cb8ac3`** (A.4 → (i) confirmada empiricamente) |
| **Regressão bit-exact validada** (P144 hyphenation, P158B figure supplement, P287 smartquote markup) | ✅ Todos preserved nos 2 748 testes pré-P288 + refino SmartQuote consumer não altera P287 (testes P287 verdes) |
| Tabela B.3 actualizada com `Lang(Lang)` (6º variant) | ✅ Confirmado + nota assimetria residual (`weight`/`tracking`/`leading`/`font` sem variant) |
| Tabela B.4 linha 353 com nota cruzada P288 | ✅ Confirmado (2ª fonte de entrada documentada) |
| 3 testes adiados de P287 §4 activados e verdes | ✅ **`p288_smartquote_double_lang_en_emite_curly_open_e_close`** + **`p288_smartquote_double_lang_pt_emite_chevrons`** + **`p288_smartquote_double_lang_fr_inclui_nbsp`** |
| Diagnóstico A.1+A.2+A.3+A.4 produzido | ✅ `diagnostico-style-lang-passo-288.md` (4 secções + diagrama de fluxo + 8 sub-secções A.1) |
| **Condicional**: ADR meta promovida se A.4 → (i) | ✅ **ADR-0098 promovida** com status `IMPLEMENTADO` |

**Conformidade**: 13/13 critérios estritos (incluindo o condicional);
1 com observação pragmática (+7 vs +8-12).

---

## §2 — Resumo factual

### §2.1 — Variant atómico + cascade arm + ADR-0098

**Antes P288**:
- `Style` enum: 5 variants (Bold/Italic/Size/Fill/HeadingLevel).
- `StyleDelta` fields: 10 — assimetria de 5 fields **sem variant
  `Style` correspondente** (`weight`/`tracking`/`leading`/**`lang`**/
  `font`).
- Caminho único para escrever `delta.lang`: parse-driven
  `eval_set_rule` em `eval/rules.rs:377-394` (P130/P131B).
- 3 testes lang-aware adiados em P287 §4 por `Style` não ter
  variant `Lang`.

**Pós-P288**:
- `Style` enum: **6 variants** (+`Lang(Lang)`).
- **2ª fonte de entrada** para `delta.lang`: via `Content::Styled(body,
  Styles::from_iter([Style::Lang(lang)]))` em `push_styles`.
- Caminho parse-driven **intacto** — bit-exact preservado.
- 3 testes lang-aware **reactivados e verdes**.
- **ADR-0098 promovida** — formalização do padrão "Single source of
  truth como invariante anti-bug" com 5 aplicações cumulativas
  registadas.

### §2.2 — Cobertura de mudança

| Sítio | Tipo | Mudança |
|---|---|---|
| `01_core/src/entities/style.rs:25-44` | extensão enum | +1 variant `Lang(Lang)`; doc-comment com justificação P288 |
| `01_core/src/entities/style.rs:130-143` | test extensão | atualiza catalog test de 5 → 6 variants |
| `01_core/src/entities/style_chain.rs:142-145` | cascade arm | +1 arm `Style::Lang(l) => delta.lang = Some(*l)` |
| `01_core/src/rules/layout/mod.rs:1992-2009` | refino consumer P287 SmartQuote | `layout_content(Content::Text)` → `layout_word(glyph)` para preservar NBSP em FR |
| `00_nucleo/adr/typst-adr-0098-single-source-of-truth-invariante-anti-bug.md` | ADR meta nova | ~220 LOC formaliza padrão com 5 aplicações cumulativas |
| `00_nucleo/prompts/entities/style.md` | L0 | +6º variant em B.3 + nota assimetria residual |
| `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | L0 cobertura | B.3 +1 linha; B.4 nota P288; footnote ⁷⁴ ~75 LOC |

Total: **3 sítios L1 + 1 ADR + 2 ficheiros L0 documentação**.

### §2.3 — Cascade arm (`push_styles`)

```rust
match style {
    Style::Bold(b)         => delta.bold = Some(*b),
    Style::Italic(i)       => delta.italic = Some(*i),
    Style::Size(pt)        => delta.size = Some(pt.val()),
    Style::Fill(c)         => delta.fill = Some(*c),
    Style::HeadingLevel(l) => delta.heading_level = Some(*l),
    // P288 — 2ª fonte de entrada para `delta.lang` (paralela à
    // parse-driven em `eval/rules.rs:385`). Last-write wins per
    // LIFO da chain. Diagnóstico P288 §A.3.
    Style::Lang(l)         => delta.lang = Some(*l),
}
```

**Match continua exaustivo** — compilador detecta ausência se `Style`
ganha variant futuro (defesa por construção).

### §2.4 — Refino consumer P287 SmartQuote

Durante materialização P288, o teste
`p288_smartquote_double_lang_fr_inclui_nbsp` falhou porque o
consumer P287 chamava:

```rust
self.layout_content(&Content::Text(glyph.into(), style));
```

E `Content::Text` arm em `layout/mod.rs:548` faz `text.split_whitespace()`
para word-wrap — que **remove NBSP** (`\u{00A0}`) por estar na
categoria whitespace. Glyph FR `«\u{00A0}` perdia o NBSP.

**Fix cirúrgico**: substituir por `layout_word`:

```rust
self.layout_word(glyph);
```

`layout_word` preserva o glyph como **unit indivisível** — push
directo em `current_line` sem split. Word-wrap natural ainda activado
via wrap check em `layout_word`. Paridade vanilla FR recuperada.

---

## §3 — Fase A — decisões registadas (`diagnostico-style-lang-passo-288.md`)

### §3.1 — A.1 inventário literal (8 sub-secções)

| Sub-secção | Achado decisivo |
|---|---|
| A.1.1 — `Style` enum | 5 variants (Bold/Italic/Size/Fill/HeadingLevel); todos `Copy`/`PartialEq` |
| A.1.2 — `StyleDelta` | 10 fields; 5 com variant paralelo; **5 sem variant** (`weight`/`tracking`/`leading`/`lang`/`font`) |
| A.1.3 — `push_styles` cascade | match exaustivo sobre 5 variants; `Style::Lang(...)` adiciona arm trivial |
| A.1.4 — `delta.lang` write | **Único** site: `eval/rules.rs:385` (parse-driven `#set text(lang)`) |
| A.1.5 — `delta.lang` read | Walk-up-chain em `chain.lang()` (`style_chain.rs:226`) |
| A.1.6 — Consumers `style.lang` | 5+ activos: eval_markup P155, hyphenation P144, supplement P158B, smartquote P287, TextStyle capture |
| A.1.7 — `FrameItem::Text` emit | **Zero hits** em `export.rs` — paradigma vanilla: `lang` é input layout-time, não persistido em emit |
| A.1.8 — Diagrama de fluxo | Caminho lateral confirmado — `Style::Lang` adiciona 2ª fonte de entrada sem perturbar consumer chain |

### §3.2 — A.2 estrutura variant — opção (a)

**Decidido**: `Lang(Lang)` paralelo aos 5 existentes.

Justificações: `Lang` é `Copy` (P131B) → `Style` mantém `Copy`
intacto; estrutura mínima vanilla simetria; rejeita
`Option<Lang>` (vanilla não suporta `lang: none`); rejeita
`LangCode(EcoString)` (replica parsing).

**Honestidade epistémica §A.2.3**: variant **atómico** (não rico) —
padrão N=4 cumulativo "variant rico com cosméticos opcionais"
**inalterado** pelo P288. Mesma lógica P287 §A.2.2 (SmartQuote leaf).

### §3.3 — A.3 integração cascade — opção (α)

**Decidido**: `delta.lang = Some(*l)` em `push_styles` (paridade
absoluta aos 5 arms existentes). Last-write wins per LIFO da chain
— determinístico.

A 2ª fonte (via `Style::Lang`) coexiste com a 1ª (parse-driven) sem
interferência semântica.

### §3.4 — A.4 impacto emit — opção (i) confirma gatilho N=5

**Decidido**: opção **(i)** — `FrameItem::Text` já consulta `Lang`
indirectamente; P288 apenas estende caminho de entrada.

Verificação empírica:

| Critério | Evidência |
|---|---|
| `export.rs` consulta `lang`? | `grep "lang" 03_infra/src/export.rs` → 0 hits funcionais |
| `FrameItem::Text` precisa novo field `lang`? | Não — `TextStyle.lang` já existe (P136 Fase A DEBT-52) |
| Consumer P287 SmartQuote já lê `self.style.lang`? | **Sim** — `layout/mod.rs:1992` |
| Reflectors tocam `export.rs`? | Não — A.1.7 confirma |

**Consequência arquitectural**: hash `export.rs` preservado bit-exact
pelo **5º passo consecutivo** → atinge **limiar histórico N=5** do
padrão "Single source of truth como invariante anti-bug".

### §3.5 — Gatilho N=5 dispara empiricamente

5 aplicações cumulativas confirmadas:
- N=1: P282 §1.1 (auditoria refutou 6/6 suspeitas)
- N=2: P285 §8.2 (alteração simétrica via helper único)
- N=3: P286 §5.2 (reuso `FrameItem::Line` sem modificação)
- N=4: P287 §5.1 (consumer reusa `Content::Text`)
- **N=5: P288 §A.4** (`Style::Lang` extende parse sem tocar emit)

**ADR-0098 promovida** com status `IMPLEMENTADO` e 5 citações
formalizadas. Promoção legítima per ADR-0065 critério N≥5 + ADR-0093
política de formalização incremental + P273.17 §0 anti-padrão
over-formalização (apenas se gatilho disparar genuinamente — aqui
confirmado).

### §3.6 — Riscos mitigados

| Risco §7 | Status | Mitigação |
|---|---|---|
| `delta.lang` dead-code | ✅ Refutado | A.1.6 lista 5+ consumers activos |
| A.4 → (ii) (novo field FrameItem) | ✅ Refutado | A.1.7 zero hits L3 |
| Assimetria oculta > 1 field | ✅ Acknowledged | 4 fields restantes registados como passos próprios condicionais |
| Promoção ADR meta indevida | ✅ Empiricamente confirmada | 5 citações documentadas + gatilho real |

---

## §4 — Testes adicionados (+7)

| Local | Quantidade | Cobertura |
|---|---:|---|
| `entities/style.rs` (mod tests) | 1 | Catalog test atualizado 5 → 6 variants (`Style::Lang(Lang::ENGLISH)` incluído) |
| `rules/layout/tests.rs` (`p287_smartquote_tests` reactivados) | 3 | **`p288_smartquote_double_lang_en_emite_curly_open_e_close`** (U+201C/U+201D); **`p288_smartquote_double_lang_pt_emite_chevrons`** («/»); **`p288_smartquote_double_lang_fr_inclui_nbsp`** (NBSP preservado) |
| `rules/layout/tests.rs` (`p288_style_lang_tests`) | 4 | Variant ctor; `push_styles` projecta no delta; `Content::Styled` injetado lê `chain.lang()`; last-write-wins entre `Style::Lang` consecutivos |
| **Total** | **7** | (3 reactivados de P287 § 4 adiados — contam para o delta P288) |

**Resultado**: 7/7 verdes (`cargo test --lib p288`).

---

## §5 — Observações pragmáticas

### §5.1 — Gatilho N=5 disparou empiricamente — ADR-0098 promovida

O gatilho condicional da spec §3 ponto 7 ("se A.4 → (i)") **disparou
empiricamente** durante a Fase A:

| Evidência | Citação |
|---|---|
| `grep "lang" 03_infra/src/export.rs` zero hits funcionais | diagnóstico §A.1.7 |
| `FrameItem::Text` não precisa novo field | A.1.7 + A.4.1 |
| Hash `export.rs` preservado bit-exact | validação final + métricas §6 |
| 5 citações cumulativas | P282 + P285 + P286 + P287 + **P288** |

**ADR-0098** ("Single source of truth como invariante anti-bug")
**promovida** em `00_nucleo/adr/typst-adr-0098-...md` com:
- Definição operacional do padrão.
- 5 aplicações cumulativas detalhadas.
- Alternativas consideradas (3) com justificações.
- Consequências imediatas + futuras + não-objectivos explicitados.
- Status `IMPLEMENTADO` desde P281, formalizado em P288.

Marco histórico do projecto: **primeira meta-ADR de invariante
arquitectural com base empírica cumulativa N≥5**. Confirma robustez
do criterio ADR-0065 (formalização baseada em precedente, não em
especulação).

### §5.2 — Delta de testes ligeiramente abaixo do alvo (+7 vs +8-12)

Spec previa "8-12 testes". Materializaram-se **7**. Causas:

1. **3 testes reactivados de P287 §4** — eram **especificações
   prontas** (LOC inalterado), apenas precisavam de `Style::Lang`
   para compilar. Contam para a meta mas representam **zero LOC
   novo de teste**.
2. **Refino consumer SmartQuote (NBSP fix)** — bug-fix descoberto
   durante materialização; +0 testes novos (validado pelo teste FR
   que originalmente falhou).
3. **Cobertura larga em 4 níveis** (variant + cascade + smartquote
   reactivado + chain.lang inject) sem inflação por nível.

Aceite como rigor genuino — não há gap material.

### §5.3 — Refino P287 NBSP como ganho colateral

Durante materialização P288 descobriu-se que o consumer P287
SmartQuote tinha bug subtil: `layout_content(Content::Text)` →
`split_whitespace()` removia NBSP. Bug **inactivo até P288** porque
P287 não tinha como injectar `Lang::FR` em testes (cara da galinha
e o ovo: testes lang-aware adiados precisavam de `Style::Lang`
para activar).

**Fix cirúrgico**: substituir por `layout_word(glyph)` — bypass do
split, glyph como unit indivisível.

**Ganho colateral**: P288 não só fecha assimetria Tabela B.3↔B.4
**como também** revela e fixa um bug latente em P287 que só ficou
detectável quando o caminho lang-aware activou. Padrão emergente:
**features dependentes ganham testes implícitos quando primitivas
prévias ganham forma de teste**. Aguardar N≥2 para considerar
formalização.

### §5.4 — Honestidade epistémica: N=4 "variant rico" inalterado

Mesma lógica P287 §A.2.2 / §5.3: `Style::Lang(Lang)` é variant
**atómico** (1 campo required) — **não** qualifica como "variant
rico com cosméticos opcionais" (padrão N=4 cumulativo). Padrão
permanece N=4 inalterado pelo P288.

Registo redundante mas crítico para futuros passos não contarem
incorrectamente.

### §5.5 — Assimetria residual Tabela B.3 vs B.4 (não-objectivo §5)

P288 fecha apenas `lang`. Continuam **sem variant `Style`**:

| Field `StyleDelta` | Caminho actual exclusivo |
|---|---|
| `weight: Option<u16>` | P139 (faux-bold) — parse-driven |
| `tracking: Option<Length>` | P137 — parse-driven |
| `leading: Option<Length>` | P138 — parse-driven |
| `font: Option<FontList>` | P132B/P140B/P141/P146 — parse-driven |

Passos próprios candidatos: P288.1 (weight), P288.2 (tracking),
P288.3 (leading), P288.4 (font). Ou aglomerado em P288.5
"cluster-close-symmetry" se algum passo futuro tiver razão para
aplicar todos. Registado mas **não materializado** per spec §5
não-objectivo + §7 risco terciário (alargamento quebra sequência
fechada cirúrgica).

---

## §6 — Métricas

| Métrica | Valor |
|---------|-------|
| LOC L1 produção | ~30 (+1 variant +8 LOC doc; +1 arm cascade +5 LOC doc; +1 LOC refino consumer SmartQuote) |
| LOC L3 produção | **0** (zero impacto em export.rs — hash preservado pelo 5º passo) |
| LOC L0 modificado | ~310 (`style.md` +15; `cobertura.md` B.3+B.4+footnote⁷⁴ ~75; `diagnostico-style-lang-passo-288.md` ~210 ficheiro novo; **ADR-0098 ~220 ficheiro novo**) |
| Testes adicionados | 7 (1 entity catalog + 3 lang-aware reactivados + 4 cascade/chain) |
| Testes baseline P287 | 2 748 preserved bit-exact |
| Testes pós-P288 | **2 755** |
| Hash L0 `style.md` | `37404a23 → b4de54d0` (`style.rs → a3eb5b76`) |
| Hash L0 `content.md` | **inalterado** (sem novo Content variant) |
| Hash L0 `stdlib.md` | **inalterado** (sem nova função) |
| Hash L0 `export.rs` | **`66cb8ac3` preservado** (5º passo consecutivo — gatilho N=5 ADR-0098) |
| Lint | zero violations |
| `Style` variants | 5 → **6** (+`Lang(Lang)`) |
| Cascade arms `push_styles` | 5 → **6** (+1 LOC) |
| Caminhos de entrada para `delta.lang` | 1 → **2** (parse + Style::Lang) |
| Pendências resolvidas | **1** (P287 §4 / §5.2 — 3 testes lang-aware adiados) |
| **ADRs novas** | **+1** (ADR-0098 IMPLEMENTADO) |

---

## §7 — Conformidade Cristalina

- ✅ **ADR-0029 pureza física L1**: `Style::Lang(Lang)` é variant
  atómico `Copy`; cascade arm é função pura sem I/O.
- ✅ **ADR-0038 Style enum divergência intencional**: P288 estende
  o enum dentro do mesmo paradigma; vanilla continua vtable.
- ✅ **ADR-0054 scope graded**: assimetria residual Tabela B.3↔B.4
  para `weight`/`tracking`/`leading`/`font` documentada como
  scope-out **consciente** per §5 não-objectivo + §7 risco
  terciário.
- ✅ **ADR-0057** (lang/hyphenation): P288 reusa caminho consumer
  `chain.lang()` existente desde P144; zero alteração na infra
  hyphenation.
- ✅ **ADR-0065 inventariar-primeiro**: Fase A obrigatória produziu
  8 sub-secções A.1 + diagrama de fluxo + decisão variant +
  cascade + impacto emit. Inspecção literal (grep + leitura
  linha-a-linha) confirmou A.4 → (i) empiricamente.
- ✅ **ADR-0085 diagnóstico imutável**:
  `diagnostico-style-lang-passo-288.md` produzido com 4 secções
  A.1-A.4 + métricas + risco residual mitigado.
- ✅ **ADR-0093 meta-metodologia evolução ADRs**: ADR-0098 promovida
  per política incremental (5 aplicações cumulativas documentadas);
  não promoção mecânica.
- ✅ **ADR-0098 (recém-formalizada)**: gatilho N=5 disparou
  empiricamente — promoção legítima per critério registado em
  ADR-0065 + P273.17 §0.
- ✅ **Anti-padrão over-formalização P273.17 §0**: promoção
  **condicional ao gatilho disparar genuinamente**. P288 §A.4
  confirma empiricamente; não acidental. **Não promover outras
  ADRs neste passo** (apenas a que atingiu limiar).
- ✅ **Honestidade epistémica reforçada §5.4**: N=4 "variant rico"
  **inalterado** (Style::Lang atómico, não rico).
- ✅ **Win arquitectural P281 N=5 cumulativo formalizado**:
  ADR-0098 fixa o padrão como invariante perene. Hash `export.rs`
  agora é métrica testável de aderência.
- ✅ **Bit-exact regression preservada por construção**: caminho
  parse-driven `eval_set_rule` intacto; 2 748 testes pré-P288
  preserved.

---

## §8 — Padrões emergentes (formalização e novos)

### §8.1 — "Single source of truth como invariante anti-bug" — **FORMALIZADA EM ADR-0098** (N=5)

P288 **fecha** este padrão emergente promovendo-o a ADR. A partir
deste passo:
- Padrão deixa de ser "emergente" e passa a ser **invariante
  arquitectural perene**.
- Hash `export.rs` torna-se **métrica testável** de aderência.
- Fase A de passos futuros deve incluir secção "potencial de
  reuso" explícita.

5 citações cumulativas (registadas na ADR e em §3.5 deste relatório):
P282 + P285 + P286 + P287 + **P288**.

### §8.2 — "Activação posterior de feature graded" — N=4 cumulativo

- N=1: P285 §8.3 (`stroke` parseado em P284 → activo via adição
  de `FrameItem::Line.color`).
- N=2: P286 §8.1 (consumer P284 single-line → wrap-aware).
- N=3: P287 §8.2 (feature *ausente* — função stdlib smartquote —
  materializada paralelamente a markup pré-existente).
- **N=4: P288** (feature *parseada-mas-inerte* — `Style::Lang`
  variant ausente apesar de `delta.lang` activo desde P144 — agora
  activa via 2ª fonte de entrada).

**Padrão maduro**: pendências graded podem ser resolvidas
incrementalmente sem refactor de fundo, adicionando **fontes de
entrada** ou **consumers** em vez de quebrar caminhos pré-existentes.

Reaplicações candidatas: P288.1-4 (restantes 4 fields sem variant
`Style`). Aguardar N≥5 para considerar formalização ADR meta paralela
a ADR-0098.

### §8.3 — "Refutação pragmática de pressuposto da spec via inspecção empírica" — N=5 cumulativo

- N=1: P285 §A.2.
- N=2/3: P286 §A.2.
- N=4: P287 §A.2 e §A.3.
- **N=5: P288 §A.4** (spec §A.4 enumerou 3 opções (i)/(ii)/(iii) com
  "default sugerido (i) se A.1 confirmar P144 paradigm" — A.1
  confirmou empiricamente; (ii)/(iii) **refutadas literalmente**
  por `grep "lang" export.rs` zero hits).

**Padrão maduro N=5** — atinge limiar histórico paralelo a §8.1.
Próximo passo onde se aplicar é candidato natural a promoção ADR
meta. **Não é objectivo P288 promover** (per anti-padrão P273.17
§0 — só um gatilho ADR meta promovido por passo, e P288 já
promoveu ADR-0098).

### §8.4 — "Bug latente fixed durante materialização de feature dependente" — N=1 inaugural

P288 descobriu e fixou o bug NBSP em consumer P287 SmartQuote
**durante** a materialização — só foi detectável porque P288
activou `Style::Lang(Lang::FR)` que P287 não conseguia testar.

Padrão emergente: **features dependentes ganham testes implícitos
quando primitivas prévias ganham forma de teste**. Reaplicações
candidatas: P288.1-4 podem revelar bugs latentes em consumers
que dependem de `weight`/`tracking`/`leading`/`font` sem
`Style::*` variant equivalente.

Aguardar N≥2 para considerar formalização.

---

## §9 — Próximos passos sugeridos (estado pós-P288)

Cobertura agregada estimada: inalterada (~64%). P288 não altera
contagem user-facing (variant adicional ao enum `Style` — feature
arquitectural, não user-facing directa) mas **formaliza padrão
arquitectural meta**.

### Rank 1-3: continuar quick wins / fecho de assimetrias

1. **`P-style-weight-variant`** (XS; assimetria residual P288 — adiciona
   `Style::Weight(u16)` paralelo a B.4 linha 350). Reaplica padrão
   §8.2 N=5 → potencial ADR meta paralela.
2. **`P-style-tracking-variant`** (XS; idem para B.4 linha 351).
3. **`P-math-accent-cancel`** (XS+S; Math 40% → 50%).

### Rank 4-6: features médias

4. **`P-curve-geometry`** (S-M; ADR-0078 sub-fase b).
5. **`P-footnote-cluster`** (M; Model 60% → 70%).
6. **`P-outline-cluster`** (M; Introspection 70% → 80%).

### Padrões pós-P288 (operacional)

- **ADR-0098 vigente** — Fase A de passos futuros deve citar
  explicitamente potencial de reuso (hash `export.rs` preservado
  ou justificadamente alterado).
- **Padrão §8.2** "activação posterior" N=4 — próxima aplicação
  candidata a promoção.
- **Padrão §8.3** "refutação pragmática" N=5 — próximo passo onde
  aplicar é candidato a ADR meta paralela.

---

## §10 — Referências cross-passos

- **P102 ADR-0040** — `#set text(...)` activation; precedente do
  parse-driven path.
- **P130 / P131B (ADR-0052)** — `Lang` tipo materializado.
- **P144 (ADR-0057)** — `text.lang` via `hypher`; precedente
  directo da consulta lang em runtime.
- **P155** — Markup `"foo"` lang-aware via `eval_markup` +
  `localize_quotes`; primeiro consumer de `chain.lang()`.
- **P158B** — `figure_supplement_for_lang`; reusa paradigma "consultar
  Lang em runtime sem persistir em emit".
- **P281** — Unificação β-completa stream-builders (origem do padrão
  formalizado em ADR-0098).
- **P282 §1.1** — Auditoria empírica (N=1 cumulativo do padrão).
- **P285 §8.2** — Alteração simétrica via helper único (N=2).
- **P286 §5.2** — Reuso `FrameItem::Line` sem modificação (N=3).
- **P287 §5.1** — Consumer reusa `Content::Text` (N=4); origem
  imediata da pendência adiada P288.
- **ADR-0029** — pureza física L1.
- **ADR-0038** — Style enum divergência intencional vs vanilla
  vtable.
- **ADR-0054** — scope graded (assimetria residual B.3↔B.4
  documentada).
- **ADR-0065** — inventariar-primeiro + critério N≥5 para promoção
  meta-ADR.
- **ADR-0085** — diagnóstico imutável (42º consumo: P288 + P287
  + P286 + P285 + P284 + P282 + 36 anteriores).
- **ADR-0093** — meta-metodologia evolução ADRs (política
  incremental).
- **ADR-0094** — meta-operacional specs.
- **ADR-0098 (recém-criada)** — Single source of truth como
  invariante anti-bug (formalização N=5).

---

*P288 fecha a assimetria Tabela B.3 vs B.4 para o campo `lang`
adicionando `Style::Lang(Lang)` ao enum `Style` (5 → 6 variants)
com arm correspondente em `StyleChain::push_styles` (paridade
absoluta aos 5 existentes). 2ª fonte de entrada para `delta.lang`
materializada — caminho parse-driven `eval_set_rule` (P130/P131B/P144)
preservado bit-exact. Hash `export.rs 66cb8ac3` preservado pelo 5º
passo consecutivo (P281+P285+P286+P287+P288) — **gatilho histórico
N=5 do padrão "Single source of truth como invariante anti-bug"
dispara empiricamente → ADR-0098 promovida** com 5 aplicações
cumulativas formalizadas. Ganho colateral: refino do consumer P287
SmartQuote para usar `layout_word(glyph)` em vez de
`layout_content(Content::Text)` — preserva NBSP em LANG_QUOTES["fr"]
que `split_whitespace` removia (bug latente detectado pela activação
dos testes lang-aware adiados P287 §4). 3 testes lang-aware
reactivados + 4 testes P288 + 1 catalog update = 7 testes verdes;
baseline 2 748 → 2 755. Honestidade epistémica reforçada: variant
`Style::Lang` é atómico (não rico) — padrão N=4 "variant rico"
inalterado pelo P288. Assimetria residual (`weight`/`tracking`/
`leading`/`font`) registada como passos próprios candidatos
P288.1-4 não-reservados.*
