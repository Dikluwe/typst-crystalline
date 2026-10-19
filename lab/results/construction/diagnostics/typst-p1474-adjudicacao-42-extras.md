# Diagnóstico — adjudicação das 42 rotas `CRYSTALLINE_ONLY` (P1474)

**Não é um Prompt L0.** Documento de classificação — ver `CLAUDE.md`
(organização de `00_nucleo/`). Fecha `typst-passo-1474.md` como diagnóstico:
nenhuma remoção, renomeação ou mudança de contrato acontece neste documento.

## Proveniência

- Commit da árvore no momento da medição: `b4b18eec9` (working tree limpo,
  `git status --short` sem alterações produtivas — só este diagnóstico e o
  próprio passo por escrever).
- Binários: vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`
  (`/usr/local/bin/typst`) e cristalino `target/debug/typst` (build de
  working tree sobre `b4b18eec9`, incluindo P1467-P1473 já integrados).
- A lista-base de 42 rotas vem de `typst-passo-1474.md` §2 (medida em
  2026-09-20, antes de P1467-P1473). Este diagnóstico **revalida
  bilateralmente cada uma** contra o estado atual (2026-09-21/22) — ver
  §1 sobre o que mudou desde a redação original.

## 0. O que mudou desde a redação original do passo

O passo foi escrito em 2026-09-20, antes de P1467-P1473 serem executados
nesta sessão. Verificação direta (não assumida): nenhuma das 42 rotas
desapareceu nem mudou de kind desde então — `type(<rota>)` continua
`function`/`symbol` para todas, confirmado por sonda bilateral repetida
nesta medição (§2). P1472 (materializado nesta sessão) tornou 17 das 18
rotas de "estilos/math globais" **redundantes** com uma rota qualificada
`math.*` equivalente (mesma função por `==`, confirmado) — isso não as fez
desaparecer da lista de 42, mas muda a análise de custo/benefício de as
manter (ver §3.1).

## 1. Revalidação da normalização 48/49 → 42

Não foi possível regenerar os catálogos automatizados completos (P1140.x)
nesta sessão — essa ferramenta produziu ~2.180 paths e não está
disponível como script executável neste checkout (só os `.json`
congelados de `00_nucleo/diagnosticos/superficie-linguagem-p1140.26*`
permanecem como artefacto). Em vez disso, cada uma das 42 rotas foi
sondada bilateralmente de forma direta e individual (`typst eval
"type(<rota>)"` em ambos os binários) — método mais lento mas com
proveniência mais explícita por linha, que é o que este passo pede (§3:
"rota | kind/repr cristalino | resultado vanilla | origem file:line").
As sete linhas de normalização do passo (`format.svg.pretty`, etc.) não
foram revalidadas aqui — são campos de `format.*` já auditados
exaustivamente em P1467-P1469 nesta mesma sessão, não pertencem às 42.

## 2. Tabela de adjudicação

### 2.1 Estilos/math globais (18 rotas)

Sonda: `type(<nome>)` vanilla vs cristalino; `math.<nome> == <nome>` e
`repr(math.<nome>) == repr(<nome>)` para confirmar identidade de função.

| rota | vanilla | cristalino | `math.<rota>` existe e é a mesma função? |
|---|---|---|---|
| `accent` | `unknown variable` | `function` | sim |
| `bb` | `unknown variable` | `function` | sim |
| `bold` | `unknown variable` | `function` | sim |
| `cal` | `unknown variable` | `function` | sim |
| `cancel` | `unknown variable` | `function` | sim |
| `display` | `unknown variable` | `function` | sim |
| `frak` | `unknown variable` | `function` | sim |
| `inline` | `unknown variable` | `function` | sim |
| `italic` | `unknown variable` | `function` | sim |
| `mono` | `unknown variable` | `function` | sim |
| `op` | `unknown variable` | `function` | sim |
| `sans` | `unknown variable` | `function` | sim |
| `scr` | `unknown variable` | `function` | sim |
| `script` | `unknown variable` | `function` | sim |
| `serif` | `unknown variable` | `function` | sim |
| `sscript` | `unknown variable` | `function` | sim |
| `underover` | `unknown variable` | `function` | **não** — `module math does not contain underover` |
| `upright` | `unknown variable` | `function` | sim |

**Achado — `underover` é um caso à parte.** `Content::MathUnderover`'s
próprio `elem_name()` interno já é literalmente `"math.underover"`
(`entities/content.rs:1468`), mas não há `dict.insert("underover", ...)`
em `make_math_module()` (`structural/math.rs`) — a rota qualificada nunca
foi registada, apesar do resto do sistema já tratar `underover` como
pertencente ao namespace `math`. Isto não é uma "extensão deliberada",
é uma lacuna real (a mesma classe de lacuna que P1472 fechou para as
outras 9 rotas) — `accent`/`cancel`/`op` já estavam qualificados antes
de P1472; só `underover` ficou de fora, aparentemente por descuido, não
decisão.

**Decisão recomendada:** para as 17 com equivalente `math.*` confirmado
(mesma função por identidade — `Func::PartialEq` compara por nome,
`entities/func.rs:384-397`), a regra §4 do passo aplica-se diretamente:
"Binding necessário apenas como implementação interna deixa de ser
global; o owner continua acessível por caminho privado/estático." Com
P1472 fechado, o caminho funcional (`math.<nome>`) já existe para todas
as 17 — a razão que bloqueava a remoção ("P1472 deve terminar antes")
já não se aplica. **Candidatas a "alias histórico a depreciar"** — não
removidas aqui; um passo futuro precisa de RED de ausência do global +
regressão de `math.<nome>` antes de qualquer remoção, conforme §5.3 do
passo. `underover` fica de fora dessa lista até ganhar `math.underover`
primeiro (passo futuro separado, mesmo padrão de P1472).

### 2.2 Aliases `calc` (3 rotas)

| rota | vanilla | cristalino | equivalente vanilla |
|---|---|---|---|
| `calc.deg` | `module calc does not contain deg` | `function` — converte radianos→graus (número puro) | nenhum — vanilla não converte números soltos, só valores `Angle` já tipados |
| `calc.log10` | `module calc does not contain log10` | `function` — `log10(x) == 2.0` para `x=100` | `calc.log(x, base: 10)` (medido: mesmo resultado) |
| `calc.rad` | `module calc does not contain rad` | `function` — converte graus→radianos (número puro); rejeita `Angle` (`esperava Int ou Float, recebeu angle`) | nenhum — mesma razão de `deg` |

Nenhuma decisão L0 prévia encontrada para estas três (não têm secção
dedicada em `compiler/stdlib/calc.md` que as declare extensão). São
conveniências matemáticas sobre números puros, sem conceito vanilla
equivalente (vanilla resolve graus/radianos inteiramente via o tipo
`Angle`, nunca por conversão de float solto). `calc.log10` é
estritamente redundante com `calc.log(x, base: 10)` já existente.

**Decisão recomendada:** `calc.log10` — candidata a "alias histórico a
depreciar" (redundante, sem uso documentado). `calc.deg`/`calc.rad` —
"requer nova sonda": não são redundantes com nada vanilla, podem ser
extensão deliberada (conversão de número puro é genuinamente útil e não
tem outro caminho no cristalino) ou vazamento; não investigado a fundo
aqui (uso interno/testes/documentação do repositório não verificado).

### 2.3 Aliases `counter`/`state` (9 rotas)

| rota | vanilla | cristalino | método moderno equivalente | mesmo resultado? |
|---|---|---|---|---|
| `counter_at` | `unknown variable` | `function` | `counter(k).at(sel)` | não comparado diretamente (ver nota) |
| `counter_display` | `unknown variable` | `function` | `counter(k).display(...)` | não comparado |
| `counter_final` | `unknown variable` | `function` | `counter(k).final()` | não comparado |
| `counter_step` | `unknown variable` | `function` | (sem equivalente método direto — ver nota) | — |
| `state_at` | `unknown variable` | `function` | `state(k,i).at(sel)` | não comparado |
| `state_display` | `unknown variable` | `function` | `state(k,i).display(...)` | não comparado |
| `state_final` | `unknown variable` | `function` | `state(k,i).final()` | não comparado |
| `state_update` | `unknown variable` | `function` | `state(k,i).update(v)` — confirmado, ambos produzem `{"func":"state.update"}` | sim (este par) |
| `state_update_with` | `unknown variable` | `function` | `state(k,i).update(fn)` (callback) | não comparado |

**Achado — não é um caso simples, ao contrário de §2.1/§2.4.**
`compiler/stdlib/state.md` (P1339) documenta explicitamente: *"Não
substituir a rota histórica por método moderno: `state_at`/`state_final`
globais mantêm `None` onde os métodos de `State` usam `init`."* — ou
seja, **há uma diferença de comportamento real e documentada**, não só
de nome. Confirmado por grep que `native_state_at`/`native_state_final`/
`native_counter_at`/`native_counter_final` são chamados internamente por
`Read::StateLegacyAt`/`Read::StateLegacyFinal`/`Read::CounterLegacyAt`/
`Read::CounterLegacyFinal` (`state.rs:465,475`, `counter.rs:608,618`) —
nomeados explicitamente "Legacy", parte de um mecanismo de replay/
validação de leituras registadas (P1339) que parece depender da forma
"legacy" continuar a existir **como função Rust**, independentemente de
estar exposta como global público.

**Decisão recomendada:** "requer nova sonda" para as 9 — a pergunta
certa não é "a rota é redundante" (pode não ser, dado o comportamento
`None` vs `init` documentado), é "a exposição **pública global** por
baixo do nome `state_at` etc. é necessária, ou só a função Rust interna
é que é (podendo tornar-se privada mantendo o mesmo comportamento
'Legacy')". Este diagnóstico não resolve essa pergunta — precisa de
leitura completa de P1339 e do mecanismo de replay antes de qualquer
decisão de remoção, e possivelmente de comparação byte-a-byte
`counter_at`/`state_at` vs os métodos modernos nos casos onde de facto
divergem (`None` vs `init`) para confirmar que a rota pública é só
espelho da interna, não uma segunda superfície com semântica própria que
algum documento externo possa depender.

### 2.4 Aliases globais `grid`/`table` (6 rotas)

| rota | vanilla | cristalino | `grid.cell`/`table.cell` produz o mesmo? |
|---|---|---|---|
| `grid_cell` | `unknown variable` | `function` | sim — `repr(grid_cell([x])) == repr(grid.cell([x])) == "grid.cell"` |
| `grid_header` | `unknown variable` | `function` | não comparado individualmente, mesma família |
| `grid_footer` | `unknown variable` | `function` | não comparado individualmente, mesma família |
| `table_cell` | `unknown variable` | `function` | sim — mesmo padrão de `grid_cell` |
| `table_header` | `unknown variable` | `function` | não comparado individualmente, mesma família |
| `table_footer` | `unknown variable` | `function` | não comparado individualmente, mesma família |

**Achado — já decidido, com L0 e teste próprios.**
`compiler/eval.md` (secção P1293) documenta explicitamente: *"Os seis
aliases flat cristalinos `grid_cell`, `grid_header`, `grid_footer`,
`table_cell`, `table_header` e `table_footer` conservam seus nomes
históricos; P1293 não cria aliases flat `*_hline`/`*_vline` nem
renomeia símbolos Rust."* — confirmado por teste locking
(`p1293_d_flat_aliases_keep_historical_names_and_lines_stay_namespaced`,
`eval/mod.rs`), que também confirma que `grid_hline`/`grid_vline`/
`table_hline`/`table_vline` foram **deliberadamente não** flatten'ados
(`stdlib.get(absent).is_none()`), provando que a presença dos seis não é
acidente de implementação — é escolha consciente e assimétrica (alguns
membros ganham alias flat, outros não).

**Decisão recomendada:** nenhuma ação — categoria "alias histórico
deliberado", já com L0 (`compiler/eval.md` P1293) e catálogo de exclusão
(`grid_hline` etc. confirmadamente ausentes prova que a exclusão do
numerador de paridade já é intencional, não um esquecimento). Cumpre
exatamente a regra §4 do passo: "Alias público deliberado só permanece
se o L0 o declarar como extensão."

### 2.5 `asset`, `lof`, `lot`, `replace` (4 rotas)

| rota | vanilla (sem feature) | vanilla (`--features bundle`) | cristalino |
|---|---|---|---|
| `asset` | `cannot access variable 'asset' because the 'bundle' feature is not enabled` | `function`, `repr(asset) == "asset"` | `function`, **sem gate de feature nenhum** |
| `lof` | `unknown variable` (idem com `--features bundle`/`html`/`a11y-extras`) | `unknown variable` | `function` |
| `lot` | `unknown variable` | `unknown variable` | `function` |
| `replace` | `unknown variable` | `unknown variable` | `function` |

**Achado — `asset` é um falso positivo do inventário, não uma extra.**
`asset` **existe no vanilla**, atrás de `Feature::Bundle` (medido
diretamente: `typst eval --features bundle 'type(asset)'` → `function`).
O catálogo original classificou-o como `CRYSTALLINE_ONLY` porque a sonda
vanilla não ativou `--features bundle`. O cristalino regista `asset`
como global **incondicional**
(`01_core/src/compiler/eval/mod.rs:8327`, sem nenhum `if features.contains(...)`),
divergindo do vanilla numa direção diferente da que o passo assumia: não
é "extra sem análogo", é **"análogo real, gate ausente"** — mais parecido
com um `WRONG_KIND`/gate incompleto do que com uma das quatro categorias
de extra do passo. Não investigado aqui o que `Feature::Bundle`
realmente controla nem o contrato de `asset` em si (fora do escopo desta
adjudicação — é sobre a *rota existir*, não sobre o *comportamento*).

`lof`/`lot`/`replace` confirmados como extensão deliberada com L0
dedicado: `compiler/stdlib/structural/outline.md` (`lof`/`lot`,
"`lof`/`lot` **não têm** [equivalente vanilla]", contrato completo com
`title:` e diagnósticos medidos) e `compiler/stdlib/text/case.md`
(`replace`, contrato completo com `count:` e diagnósticos medidos).
Porém, ao contrário de `lof`/`lot` (que só existem como conveniência
sintática sem forma alternativa), `replace` **é redundante com o método
`.replace(...)`** já existente e idêntico (`"abc".replace("a","x") ==
replace("abc","a","x")`, ambos `"xbc"`, confirmado) — o mesmo padrão de
`grid_cell`/`counter_at`, mas aqui **sem** decisão L0 que declare
explicitamente a duplicação como deliberada (o L0 de `replace` documenta
o contrato da função em si, não o porquê de também existir como global
bare além do método).

**Decisão recomendada:**
- `asset`: reclassificar fora das "42 extras" — é gate de feature
  ausente, não extensão. Recomendo um passo dedicado (fora de P1474) para
  medir `Feature::Bundle` no vanilla e decidir se o cristalino deve
  replicar o gate ou declarar `asset` como sempre-ativo por decisão
  própria (com L0 que o diga explicitamente — hoje não há).
- `lof`/`lot`: nenhuma ação — extensão deliberada, já com L0 completo.
- `replace`: "requer nova sonda" — parece redundante com `.replace()`,
  mas falta confirmar se algum documento/teste do repositório depende do
  nome bare antes de recomendar depreciação (mesmo cuidado do §2.3).

### 2.6 `math.registered` / `sym.registered` (2 rotas)

| rota | vanilla | cristalino |
|---|---|---|
| `math.registered` | `module math does not contain registered` | `symbol` |
| `sym.registered` | `module sym does not contain registered` | `symbol` |

**Achado — já decidido.** `compiler/stdlib/sym.md`: *"`sym.registered` é
extensão cristalina histórica, ausente do codex pinado. P1283 a
[preserva]."* `compiler/stdlib/structural/math.md` (P1283): *"
`math.registered`, herdado de `sym.registered`, permanece extensão
cristalina individualizada, sem crédito de paridade e sem remoção
automática."*

**Decisão recomendada:** nenhuma ação — categoria "extensão deliberada
sem crédito de paridade", já decidida e documentada em L0 (P1283), exatamente
como a regra §4 do passo exige para `registered`: *"não pode ser removido
se for mecanismo de introspecção cristalino ainda contratado; deve ser
marcado como extensão sem crédito"* — já está marcado assim.

## 3. Resumo por categoria

| Categoria (§1 do passo) | Rotas | Contagem |
|---|---:|---:|
| Extensão deliberada a preservar (já decidida, com L0) | `grid_cell`, `grid_header`, `grid_footer`, `table_cell`, `table_header`, `table_footer`, `lof`, `lot`, `math.registered`, `sym.registered` | 10 |
| Alias histórico — candidata a depreciar (requer passo próprio, RED de ausência + regressão do caminho canônico) | `accent`, `bb`, `bold`, `cal`, `cancel`, `display`, `frak`, `inline`, `italic`, `mono`, `op`, `sans`, `scr`, `script`, `serif`, `sscript`, `upright` (17, todos com `math.*` equivalente confirmado), `calc.log10` (redundante com `calc.log(x, base:10)`) | 18 |
| Lacuna real (não é extra — falta fechar antes de decidir) | `underover` (falta `math.underover`) | 1 |
| Falso positivo do inventário (não é extra — gate de feature ausente) | `asset` | 1 |
| Requer nova sonda (comportamento não totalmente esclarecido) | `counter_at`, `counter_display`, `counter_final`, `counter_step`, `state_at`, `state_display`, `state_final`, `state_update`, `state_update_with`, `calc.deg`, `calc.rad`, `replace` | 12 |

Soma: 10 + 18 + 1 + 1 + 12 = 42. Confere com a contagem original do passo.

## 4. Próximos passos (não executados aqui)

Por ordem de prontidão (mais pronta primeiro):

1. **Fechar `underover`** — passo pequeno, mesmo padrão de P1472: registar
   `math.underover` em `make_math_module()`. Depois disso, `underover`
   junta-se ao grupo de 18 "candidatas a depreciar".
2. **Depreciar as 18 candidatas do grupo "estilos globais + `calc.log10`"**
   — cada uma precisa de RED de ausência do global + regressão do
   caminho `math.*`/`calc.log(base:10)` canônico, por família, com gate
   ADR-0127 (quebra de compatibilidade).
3. **Investigar `Feature::Bundle`/`asset`** — medir o que a feature
   realmente controla no vanilla antes de decidir se o cristalino deve
   gatear `asset` ou declarar divergência deliberada com L0 próprio.
4. **Resolver as 12 "requer nova sonda"** — ler P1339 por completo para
   `counter_*`/`state_*` (9 rotas); decidir `calc.deg`/`calc.rad`/
   `replace` com investigação de uso real no repositório/testes antes de
   recomendar qualquer remoção.

Nenhuma destas ações foi tomada nesta sessão — este documento fecha
P1474 como diagnóstico, conforme autorizado explicitamente pelo próprio
passo ("P1474 pode fechar como diagnóstico sem remover nada").

## Adendo P1488 — retificação sem apagar o histórico (2026-09-24)

Este adendo não substitui a classificação histórica acima. Distingue a
medição posterior, a decisão já existente e o que ainda exige decisão nova.
Não autoriza a remoção de nenhum binding público.

### Proveniência

As sondas locais decorreram em 2026-09-24, entre 10:56:01 e 10:58:04
(-03:00), sobre HEAD e7670e5a1b97fca4e7d5ff236645720027a25669
(P1487). O vanilla ratificado foi /usr/local/bin/typst, SHA-256
eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f;
o cristalino foi target/debug/typst, SHA-256
81cbcb240620cfccac5677cbab236b9609cb7209252109f8ca63ba2b64275fa2.
O SHA do executável cristalino identifica o binário sondado, não certifica
sozinho que incorpora toda mudança não commitada da árvore.

A árvore estava não commitada antes deste adendo: 66 arquivos rastreados
modificados segundo git diff HEAD --name-only e 11 não rastreados segundo
git ls-files --others --exclude-standard. Lista exata dos rastreados:

~~~text
00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md
00_nucleo/prompts/compiler/eval.md
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/eval/tests.md
00_nucleo/prompts/compiler/introspect.md
00_nucleo/prompts/compiler/introspect/from_tags.md
00_nucleo/prompts/compiler/layout.md
00_nucleo/prompts/compiler/layout/cursor.md
00_nucleo/prompts/compiler/layout/html_frame.md
00_nucleo/prompts/compiler/stdlib/counter.md
00_nucleo/prompts/compiler/stdlib/format.md
00_nucleo/prompts/compiler/stdlib/structural.md
00_nucleo/prompts/compiler/stdlib/structural/document.md
00_nucleo/prompts/entities/compiler_features.md
00_nucleo/prompts/entities/content.md
00_nucleo/prompts/entities/counter_registry.md
00_nucleo/prompts/entities/layout_types.md
00_nucleo/prompts/entities/locator.md
00_nucleo/prompts/infra/export/builder.md
00_nucleo/prompts/infra/export/tests/mod.md
00_nucleo/prompts/infra/integration_tests.md
00_nucleo/prompts/infra/pipeline.md
00_nucleo/prompts/infra/pipeline/context_stabilization.md
00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md
00_nucleo/prompts/shell/cli.md
00_nucleo/prompts/shell/watch.md
00_nucleo/prompts/wiring.md
00_nucleo/prompts/wiring/tests/cli.md
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/mod.rs
01_core/src/compiler/eval/repr.rs
01_core/src/compiler/eval/rules.rs
01_core/src/compiler/eval/tests.rs
01_core/src/compiler/introspect.rs
01_core/src/compiler/introspect/from_tags.rs
01_core/src/compiler/layout/cursor.rs
01_core/src/compiler/layout/html_frame.rs
01_core/src/compiler/layout/mod.rs
01_core/src/compiler/stdlib/counter.rs
01_core/src/compiler/stdlib/format.rs
01_core/src/compiler/stdlib/foundations/query.rs
01_core/src/compiler/stdlib/structural/document.rs
01_core/src/compiler/stdlib/structural/mod.rs
01_core/src/entities/compiler_features.rs
01_core/src/entities/content.rs
01_core/src/entities/counter_registry.rs
01_core/src/entities/layout_types.rs
01_core/src/entities/locator.rs
02_shell/src/cli.rs
03_infra/src/export/builder.rs
03_infra/src/export/fonts.rs
03_infra/src/export/render.rs
03_infra/src/export/stream.rs
03_infra/src/export/svg.rs
03_infra/src/export/tests/mod.rs
03_infra/src/export/tests/p1481_pdfua_structure_tests.rs
03_infra/src/export/tests/p1483_pdf_spot_tests.rs
03_infra/src/integration_tests.rs
03_infra/src/layout_bidi.rs
03_infra/src/pipeline.rs
03_infra/src/pipeline/context_stabilization.rs
03_infra/src/shaper.rs
03_infra/src/watch.rs
03_infra/tests/p1297_watch_capability_contract.rs
04_wiring/src/main.rs
04_wiring/tests/cli.rs
~~~

Não rastreados no mesmo instantâneo:

~~~text
00_nucleo/diagnosticos/typst-p1484-matriz-bundle.md
00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md
00_nucleo/diagnosticos/typst-p1485-integracao-gate2.md
00_nucleo/diagnosticos/typst-p1486-page-labels.md
00_nucleo/materialization/typst-passo-1484.md
00_nucleo/materialization/typst-passo-1485.md
00_nucleo/materialization/typst-passo-1486.md
00_nucleo/materialization/typst-passo-1488.md
00_nucleo/prompts/infra/export/tests/p1486_pdf_page_labels_tests.md
03_infra/src/export/tests/p1486_pdf_page_labels_tests.rs
lab/parity/corpus/p1486-page-labels.typ
~~~

### Medição antes da retificação

1. Nos dois binários acima, typst eval 'type(math.underover)' falhou com
   module math does not contain underover. Typst eval 'type(underover)'
   falhou no vanilla com unknown variable underover e devolveu function no
   cristalino. O registro global está em
   01_core/src/compiler/eval/mod.rs:8391; os registros do módulo math em
   01_core/src/compiler/stdlib/structural/math.rs:2837-2861 não criam esse
   membro. Portanto, a conclusão de P1474 §2.1/§3/§4 de que falta
   math.underover como lacuna de paridade é falsa. Não adicionar o membro
   para reparar uma ausência que também existe no vanilla. O global
   underover é extra cristalina a adjudicar separadamente. Inferência:
   pode ser conveniência histórica; refutação seria L0 ou uso real que
   estabeleça outra finalidade. A medição não prova remoção segura.
2. No cristalino, typst eval de
   (math.cancel == cancel, repr(math.cancel), repr(cancel),
   repr(math.cancel([x])), repr(cancel([x]))) devolveu
   (false, cancel, cancel, cancel(body: [x]), cancel(body: [x]));
   no vanilla, repr(math.cancel([x])) também devolveu cancel(body: [x]).
   A forma JSON exata da primeira sonda foi
   [false,"cancel","cancel","cancel(body: [x])","cancel(body: [x])"].
   O qualificado usa nome nativo math.cancel em
   01_core/src/compiler/stdlib/structural/math.rs:2856-2861; o global
   usa cancel em 01_core/src/compiler/eval/mod.rs:8387-8390;
   Func::PartialEq compara nomes em
   01_core/src/entities/func.rs:375-396. A sonda completa de P1488
   encontrou igualdade true nas outras 16 das 17 rotas math globais;
   cancel é a exceção. Retificação: a alegação genérica de P1474 §2.1
   de identidade por == não vale para todas. A chamada amostrada coincide,
   mas não certifica todos os argumentos. Inferência de equivalência
   completa seria refutada por outra chamada, diagnóstico ou contexto
   que divergisse; manter decisão individual por binding.
3. P1474 §2.2/§3 classificou calc.log10 como candidato a depreciar e
   calc.deg/calc.rad como indeterminados. A medição vanilla/cristalino
   e a decisão explícita MANTER as três como extensões cristalinas já
   constam em 00_nucleo/diagnosticos/typst-passo-817-relatorio.md:107-116
   e :177-180; o comentário produtivo confirma em
   01_core/src/compiler/stdlib/calc.rs:83-91. Não reabrir essa decisão
   apenas por redundância aritmética ou ausência no vanilla. Inferência:
   o L0 calc vigente talvez não expresse a preservação tão claramente
   quanto o relatório P817; refutação é cláusula L0 incompatível ou nova
   decisão explícita do dono. Qualquer alteração exige reconciliação L0.
4. As nove rotas counter_*/state_* e replace têm contrato L0, ao
   contrário da hipótese de vazamento sem owner: ver
   00_nucleo/prompts/compiler/stdlib/counter.md:213-221,
   state.md:187-196 e text/case.md:29-43. Para state_at/state_final,
   state.md:20-32 ainda veda substituir a semântica global None pelo
   método moderno que usa init no replay. Preservar essas rotas na fase
   diagnóstica; não afirmar que o método canônico é substituto compatível.
   Inferência: exposição pública e helper Rust interno talvez possam
   separar-se; refutação é uso externo ou comportamento linguístico
   dependente do binding. Remoção exigiria L0 causal e ADR-0127.
5. O total 42 acima é fotografia histórica que inclui asset como extra.
   O gate de bundle foi especificado em
   00_nucleo/prompts/compiler/eval.md:180-212. P1487 mediu 47/48
   EXTRA_BINDING brutos em default/html, respectivamente
   (00_nucleo/diagnosticos/typst-p1487-recalibracao.md:143-156), contra
   48/49 históricos. Em P1488, dois passes completos dos seis perfis
   default, html, a11y-extras, html+a11y, bundle e html+bundle foram
   gerados sequencialmente com os extratores release P1487 e os binários
   acima em /tmp/p1488-inventory-codex-20260924.T6k2u0; 18/18 pares
   vanilla/crystal/inventory dos passes 1 e 2 foram idênticos byte a
   byte. Após excluir os 6/7 defaults format.* conforme o perfil, os
   seis catálogos atuais dão 41 rotas exatas, conjunto histórico das 42
   menos asset, sem rota nova. A lista atual não é uma autorização de
   depreciação: 10 extensões previamente deliberadas, 3 calc mantidas
   por P817, 10 rotas counter/state/replace contratadas, e 18 globais
   math/underover ainda sujeitos a decisão por binding. Inferência:
   presença extra no catálogo não implica defeito da linguagem;
   refutação para qualquer rótulo de intenção exige L0 e sonda
   funcional, não só contagem de chaves.

Nenhum código produtivo nem binding foi alterado por este adendo. O gate
de P1488 §2.4/ADR-0127 permanece: confirmar com o dono antes de qualquer
remoção pública, com L0 e plano de compatibilidade por owner, depois
testes RED→GREEN e revalidação.
