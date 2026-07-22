# Relatório — typst-passo-842: `layout` (define) — 8 achados (#32–#39)

**Data**: 2026-07-22
**Proveniência**: commit HEAD `d70617970aa427eec4fad8ead2faf8f6989da6e7` (P841), árvore limpa no arranque (`git status --porcelain` vazio). Todas as medições "antes" correram nesse estado; as medições "depois" correram com working tree não commitado (lista de ficheiros alterados no fim deste relatório) e release rebuildado (`cargo build --release`, 19.81s).
**Binários**: vanilla `lab/typst-original/target/release/typst compile <f> <out>`; cristalino `./target/release/typst <f> -o <out>`. Fixtures em `temp/p842/`. Extração de texto/posições via `pdftotext`/`pdftotext -bbox`.
**Baseline confirmada no arranque**: `cargo test -p typst-core` = 4570 passed / 0 failed; `cargo test -p typst-infra` = 698 passed / 0 failed.

---

## Nota de coordenação com P841 (#36/#37) — pedida pelo prompt

**#36 (`Sub` de `Angle`) e #37 (`Add` de `Fraction`) já estavam fechados em P841** (commit `d70617970`): braços em `01_core/src/engine/eval/operators.rs:386-392` com testes `p841_sub_angle`/`p841_add_fraction`. **Não foram reimplementados neste passo** — só confirmados por sonda no arranque (HEAD de P841, sem nenhuma alteração deste passo):

| Caso | Vanilla | Cristalino (P841) |
|---|---|---|
| `#repr(90deg - 45deg)` | `45deg` (exit 0) | `45deg` (exit 0) ✓ |
| `#repr(1fr + 2fr)` | `3fr` (exit 0) | `3fr` (exit 0) ✓ |

Sem trabalho duplicado nem lacuna entre os dois passos: P841 fechou #31/#36/#37; P842 trata #32–#35, #38, #39.

---

## #32 (L1) — `type(50%)`/`type(30% + 1em)` devolviam `length` — **FECHADO**

### Medição (antes)

| Caso | Vanilla | Cristalino |
|---|---|---|
| `type(50%)` | `ratio` | `length` |
| `type(50% + 0pt)` | `relative` | `length` |
| `type(30% + 1em)` | `relative` | `length` |

**Refutação da premissa do prompt**: o prompt (e o L0 antigo) afirmavam que
"Relative puro é Ratio no vanilla quando a parte absoluta é zero". Medido:
`type(50% + 0pt)` → `relative` — o tipo no vanilla depende da **construção**
(`Ratio + Length` constrói `Rel`, mesmo com abs zero), não do valor.

### Causa raiz

Dois níveis: (1) `Value::type_of` mapeava `Relative → Type::Length`
(`entities/value.rs:341`) com comentário que afirmava paridade — errado;
(2) mais fundo: o literal percentual (`eval/mod.rs`, `Unit::Percent`) era
avaliado para `Value::Relative` com abs zero (P469), fundindo os tipos
`ratio`/`relative` — por isso `type(50%)` também dava `length`.

### Implementação

- `entities/value.rs`: novo `Type::Relative` (name `"relative"`); `type_of`:
  `Relative → Type::Relative`; comentário errado substituído pela medição.
- `eval/mod.rs`: `Unit::Percent → Value::Ratio(Ratio::from_percent(v))`;
  binding global `relative` registado (vanilla: `type(30% + 1em) == relative`
  → `true`).
- `eval/operators.rs`: braços de `Ratio` para a tabela medida no vanilla —
  `Ratio ± Ratio → Ratio`; `Ratio ± Length ↔ Relative` (ambas as ordens);
  `Ratio × Float → Ratio`; `Ratio × Fraction → Fraction` (`100% * 2fr = 2fr`,
  `type(50% * 2fr) = fraction`); `Ratio / Int|Float → Ratio`.
  (`Ratio × Int`, `Ratio / Ratio`, `Neg`, `Eq ↔ Relative`, ordenação já
  existiam.)
- Consumers que esperavam percentagem como `Value::Relative` ganharam braço
  `Value::Ratio` equivalente (não-regressão medida): `rgb`/`linear-rgb`
  (`stdlib/foundations.rs`), `extract_ratio_arg` (`stdlib/color.rs`),
  `extract_length` (`stdlib/layout.rs` — `Ratio` truncado para zero, mesmo
  scope-out P475 que `Relative` já tinha). O braço `Relative` foi mantido em
  todos os sítios (Relative ainda é construído por outras vias).
- Testes P469 que afirmavam a representação antiga atualizados (a mudança de
  representação é deliberada e fundamentada na medição — decisão minha,
  registada aqui); nota P685 "fora de escopo" revogada.

### Validação (depois, release rebuildado)

Tabela verbatim completa (`temp/p842/l1_ratio_arith.typ`), vanilla ==
cristalino em todos os itens: `ratio 80% relative 50% + 1pt relative ratio
20% ratio 100% ratio ratio 25% float 2.0 -50% ratio true ratio 1em 50%`.
Não-regressão de consumers (`regressao_ratio.typ`): `box(width: 50%)`,
`rect(width: 50%)`, `rgb(50%,0%,0%)` → `#800000`, `rgb(...,50%)` →
`#80000080`, `luma(50%)`, `luma(150%)` → `luma(100%)` (fallback paritário),
`block(inset: 5%)`, `color.saturate(red, 50%)` → `rgb("#ff271b")`,
`polygon` com coords pt — tudo verbatim igual ao vanilla.

Testes novos: `p842_l1_type_ratio_puro`, `p842_l1_type_relative_abs_zero`,
`p842_l1_type_relative_misto`, `p842_l1_relative_binding_global`,
`p842_l1_ratio_aritmetica_preserva_tipo`, `p842_l1_ratio_vezes_fraction`.

---

## #33 (L2) — `measure` rejeita `width:`/`height:` — **CONFIRM-ONLY (sem alteração)**

Scope-out consciente (ADR-0054), confirmado vivo e fiel à medição de P831:
cristalino exit 1 com `error: measure: named arg `width` não suportado
(paridade graded; refino futuro candidato NÃO-reservado per ADR-0054)`
(`stdlib/layout.rs:1761`), vanilla exit 0. A entrada reflete a medição —
nada a corrigir; não implementado por exigir decisão do dono.

---

## #34 (L3) — `measure` devolve métricas diferentes — **SONDA COMPLETA, causa identificada; SEM correção (pendência para o dono)**

### Sonda (conteúdos progressivamente mais simples, `temp/p842/l3_measure_*.typ`)

| Conteúdo | Vanilla | Cristalino |
|---|---|---|
| `measure([hello])` | `(22.19pt, 7.24pt)` | `(33pt, 14.85pt)` |
| `measure([x])` | `(5.39pt, 7.24pt)` | `(6.6pt, 14.85pt)` |
| `measure([abcd])` | `(20.83pt, 7.24pt)` | `(26.4pt, 14.85pt)` |
| `measure([a b])` | `(13.2pt, 7.24pt)` | `(19.8pt, 14.85pt)` |
| `measure([])` | `(0pt, 0pt)` | `(0pt, 0pt)` |
| `measure([x])` com `#set text(size: 20pt)` | `(9.8pt, 13.16pt)` | `(12pt, 27pt)` |

### Isolamento da causa

- Largura cristalina = **0.6 × size por codepoint** ("hello" = 5×6.6 = 33pt)
  — a heurística monoespaçada de `FixedMetrics.advance`
  (`engine/layout/metrics.rs:215`); vanilla usa métricas reais da fonte
  (shaping).
- Altura cristalina = **1.35 × size** (11pt → 14.85; 20pt → 27) = top-edge
  cap-height (0.7em, `FixedMetrics.text_edges`) + leading default (0.65em);
  vanilla devolve top-edge real da fonte (≈0.658em na Libertinus Serif).
- Ponto exato: `measure_content_real` (`engine/layout/mod.rs:1773-1804`)
  constrói o `Layouter` de medição com **`FixedMetrics` hard-coded**. A
  divergência já estava **documentada no código desde P712**
  (comentário "Divergência mecânica documentada, não de língua (ADR-0107):
  L1 não tem acesso a métricas de fonte reais (`FallbackFontMetrics` é L3)").

### Porque não foi corrigido neste passo

A correção real exige injetar métricas de fonte L3 no caminho de eval
(`eval_func_call` → `measure_content_real` corre em L1, que por construção
não conhece L3). É uma mudança arquitetural de fronteira de camadas, não um
bug local — exige decisão do dono (provável candidato: portar o padrão de
injeção já usado em `layout_with_introspector_and_metrics` para o eval, ou
resolver `measure` numa fase pós-eval com métricas disponíveis). Sonda
registada com a causa exata (`file:line`) para o passo que a decidir.

**Sem alteração de código neste achado** (sem teste novo — nada mudou para
testar; o comportamento permanece o documentado em P712).

---

## #35 (L4) — `#context (10pt)` não renderizava — **FECHADO**

### Medição (antes / depois, `temp/p842/l4_ctx_*.typ`)

| `#context (<valor>)` | Vanilla | Cristalino antes | Cristalino depois |
|---|---|---|---|
| `(10pt)` | `10pt` | *(vazio)* | `10pt` ✓ |
| `(50%)` | `50%` | *(vazio)* | `50%` ✓ |
| `(30% + 1em)` | `30% + 1em` | *(vazio)* | `30% + 1em` ✓ |
| `(45deg)` | `45deg` | *(vazio)* | `45deg` ✓ |
| `(2fr)` | `2fr` | *(vazio)* | `2fr` ✓ |
| `(3)` / `(2.5)` | `3` / `2.5` | `3` / `2.5` (já ok) | sem alteração |

### Implementação

`value_to_content` (`engine/stdlib/state.rs`) ganhou o braço
`Value::Length | Ratio | Relative | Angle | Fraction` usando o display já
usado em `repr` (para unidades coincide com o display vanilla) — mesmo
padrão do braço `Value::Type` de P821. Cobre também o caminho via
`03_infra/src/pipeline.rs:156` (a função é partilhada).

Teste novo: `p842_l4_value_to_content_tipos_numericos_geometricos`.

---

## #38 (L7) — `#h(1fr)` rejeitado — **FECHADO**

### Medição vanilla (antes; `temp/p842/l7_h_*.typ`, página 200pt×100pt, margem 0)

- `A#h(1fr)B` → B em `xMax=200.0` (fr consome todo o restante da linha);
- `A#h(1fr)B#h(2fr)C` → razão 1:2 (B em xMin 67.239, C em 192.894);
- `A#h(10pt)B#h(1fr)C` → fixo e fração combinam.

Cristalino antes: exit 1 `error: h() espera amount como length, recebeu
fraction`. Não existia mecanismo de distribuição fracionária no layout de
linha (o fr de `grid` é de tracks, mecanismo distinto).

### Implementação (decisões minhas, registadas)

- Novo enum `Spacing { Absolute(Length), Fractional(f64) }` em
  `entities/elements/h_space.rs` (paridade vanilla `layout/spacing.rs`);
  `HSpaceElem.amount: Spacing`. Construtores `Content::h_space` (Absolute) e
  `Content::h_space_fraction` (Fractional).
- `build_spacing` (`stdlib/layout.rs`) aceita `Fraction` (com a mesma
  validação de não-negativo); `native_h` propaga; **`native_v` rejeita
  `Fraction` com a mensagem pré-P842 verbatim** — distribuição vertical
  fracionária é outro mecanismo, scope-out registado aqui.
- Layout (`engine/layout/h_space.rs`): `Fractional` regista
  `(current_line.len(), fr)` no novo campo `Region::pending_fr`
  (`entities/region.rs`) em vez de mover o cursor.
- Expansão: `Layouter::expand_fr_spacings` (`engine/layout/cursor.rs`),
  chamada no início de `flush_line` **e** de `finish` (a última linha também
  expande — medido). `remaining = (width - margin) - line_content_right`,
  truncado a ≥ 0; cada fr translada os items à sua direita só pelo seu
  próprio share (item à direita de vários fr recebe a soma, uma parcela por
  iteração); após expansão `cursor_x = right_margin`. Corre antes do
  collector de decorações, do leading e do alinhamento RTL.
- Medição (`measure_content_constrained`): `Fractional` mede `(0, 0)` — sem
  restante definido numa medição isolada.

### Validação (depois, release rebuildado, `pdftotext -bbox`)

| Caso | Vanilla | Cristalino depois |
|---|---|---|
| `A#h(1fr)B` | B: xMin 193.532, xMax 200.0 | B: xMin 193.532, xMax 200.0 ✓ |
| `A#h(1fr)B#h(2fr)C` | B: 67.238670, C: 192.894 | B: 67.239, C: 192.894 ✓ |
| `A#h(10pt)B#h(1fr)C` | B: 17.645, C: 192.894 | B: 17.645, C: 192.894 ✓ |

Posições iguais à centésima de ponto (o pipeline de produção usa métricas
reais; os testes unitários usam `FixedMetrics` com tolerâncias anotadas).

Testes novos: `p842_l7_h_1fr_expande_espaco_restante`,
`p842_l7_h_fr_distribuicao_proporcional`, `p842_l7_h_fr_com_length_fixo`,
`p842_l7_native_h_aceita_fraction`,
`p842_l7_native_v_rejeita_fraction_com_mensagem_previa`.

Nota: no teste do caso misto mediu-se um desvio de ~0.6pt no gap do
`h(10pt)` com `FixedMetrics` — **pré-existente** (caminho absoluto inalterado
neste passo; o teste P156D `layout_hspace_avanca_cursor_x` sempre usou
threshold). Registado; não investigado por ser fora do achado.

---

## #39 (L8) — mensagem de `2 * ltr` divergia — **FECHADO**

### Medição vanilla por operador (antes de generalizar; `temp/p842/l8_probe_*.typ`)

| Expressão | Vanilla | Cristalino antes |
|---|---|---|
| `2 * ltr` | `cannot multiply integer with direction` | `cannot apply Mul to int and direction` |
| `ltr * 2` | `cannot multiply direction with integer` | (idem genérico) |
| `1em + ltr` | `cannot add length and direction` | `cannot apply Add to length and direction` |
| `1pt - ltr` | `cannot subtract direction from length` | `cannot apply Sub to length and direction` |
| `1 / ltr` | `cannot divide integer by direction` | (genérico) |
| `ltr < 2` | `cannot compare direction and integer` | (genérico com `Lt`) |
| `true + 1` | `cannot add boolean and integer` | `cannot apply Add to bool and int` |
| `(50% + 1pt) + ltr` | `cannot add relative length and direction` | (genérico) |

Fonte dos formatos: `foundations/ops.rs:170,214,284,340,500` — `Sub`
inverte a ordem dos operandos; nomes **longos** de tipo (`integer`,
`boolean`, `string`, `relative length`, …).

### Implementação

Fronteira genérica de `eval_binary_op` reescrita: `binary_mismatch` por
operador (Add/Sub/Mul/Div com os formatos verbatim; restantes ops mantêm o
formato pré-P842) + fronteira de comparação com `cannot compare {a} and {b}`
+ helper `vanilla_type_name` (nomes longos). O único teste que afirmava o
formato antigo (`p722_dict_vezes_int_erro_fronteira`) atualizado para a
mensagem vanilla (`cannot multiply dictionary with integer`). A divergência
de mensagem do operador `in` (§P706 do L0) mantém-se — não fazia parte do
achado.

Validação (depois): `2 * ltr` →
`error: cannot multiply integer with direction` ✓ (exit 1, como o vanilla).

Teste novo: `p842_l8_fronteira_mensagens_vanilla` (7 pares, incluindo ordem
invertida e nomes longos).

### Achado lateral (fora de escopo, registado — NÃO corrigido)

Medido durante a sonda: `none + 1` e `1 + none` **compilam no vanilla**
(devolvem `1` — `None` é identidade em `add`); o cristalino rejeita
(`cannot add none and integer`). Divergência comportamental, não de
mensagem — fica para passo dedicado.

---

## Consolidação

### Contagens de testes (comando: `cargo test -p <crate>`)

| Crate | Antes | Depois | Delta |
|---|---|---|---|
| `typst-core` | 4570 passed / 0 failed | **4583 passed / 0 failed** | +13 (6×`p842_l1`, 1×`p842_l4`, 5×`p842_l7`, 1×`p842_l8`) |
| `typst-infra` | 698 passed / 0 failed | **698 passed / 0 failed** | 0 |

### Lint

`crystalline-lint --fix-hashes .` executado (hashes sincronizados após as
edições de L0; efeito colateral esperado: `@prompt-hash` atualizado nos
headers de todos os ficheiros que referenciam os L0 editados).
`crystalline-lint .` → **exit 0** (0 drift; warnings V7 de prompts órfãos
pré-existentes, não introduzidos neste passo).

### L0 atualizados

`prompts/entities/value.md` (Type::Relative, tabela type_of, comentário
errado revogado), `prompts/engine/eval/ops.md` (literal percentual → Ratio,
braços Ratio, fronteira verbatim #39, scope-out misto revogado),
`prompts/engine/eval.md` (binding `relative`),
`prompts/engine/stdlib/state.md` (value_to_content),
`prompts/engine/stdlib/layout.md` (h() com Fraction, v() scope-out),
`prompts/engine/stdlib/foundations.md` (rgb/luma Ratio),
`prompts/engine/stdlib/color.md` (extract_ratio_arg Ratio),
`prompts/entities/elements/h_space.md` (enum Spacing),
`prompts/entities/region.md` (pending_fr), `prompts/engine/layout.md`
(§P842 — expansão fr na linha).

### Ficheiros de código alterados (substância)

`01_core/src/entities/value.rs`, `entities/elements/h_space.rs`,
`entities/content.rs`, `entities/region.rs`,
`engine/eval/mod.rs`, `engine/eval/operators.rs`, `engine/eval/tests.rs`,
`engine/stdlib/state.rs`, `engine/stdlib/layout.rs`,
`engine/stdlib/foundations.rs`, `engine/stdlib/color.rs`,
`engine/stdlib/shapes.rs` (só comentário), `engine/stdlib/mod.rs` (só
testes), `engine/layout/h_space.rs`, `engine/layout/cursor.rs`,
`engine/layout/mod.rs`, `engine/layout/tests.rs` — mais atualizações de
`@prompt-hash` (1 linha) em ficheiros que referenciam os L0 editados.

### Pendências para o dono (decisões PARAM aqui)

1. **#34**: injetar métricas de fonte reais no caminho de `measure`
   (arquitetural — L1↔L3; causa exata em `layout/mod.rs:1773-1804`).
2. **`v(1fr)`**: distribuição vertical fracionária (scope-out registado;
   mensagem de rejeição preservada).
3. **`none + 1`**: vanilla trata `None` como identidade em `add` (achado
   lateral da sonda #39).
4. **Ratio em contextos de layout** (`h(50%)`, `pad(50%)` truncados para
   zero — scope-out P475 mantido; antes era `Relative` truncado, agora é
   `Ratio` truncado — mesmo observável).
5. **Div mista `Length/Relative`, `Ratio/Relative`** (scope-out P713
   mantido; `Ratio` agora é produzível por sintaxe, mas sem consumidor
   medido).

### Desvios/limitações a rever antes do commit

- Representação do literal percentual mudou de `Value::Relative` para
  `Value::Ratio` — blast radius controlado pelos braços novos e pela suíte
  (4570→4583, 0 falhas) + sonda de regressão verbatim, mas consumidores não
  cobertos por testes que façam match exaustivo de `Value::Relative`
  esperando percentagens podem agora ver `Ratio` cair no braço `other`
  (mudando mensagens de erro de "found relative length" para "found
  ratio" — aliás mais próximo do vanilla).
- `pending_fr` é por-região; `h(Nfr)` dentro de sub-frames isolados
  (`layout_sub_frame`, `measure`) expande contra a largura dessa região —
  comportamento razoável, medido só no fluxo principal.
- Interação `h(Nfr)` + RTL: expansão corre antes do alinhamento RTL (o fr
  consome o slack que o RTL align usaria); caso não medido nos binários.
