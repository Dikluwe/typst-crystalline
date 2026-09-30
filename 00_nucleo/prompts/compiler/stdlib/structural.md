# L0 — stdlib/structural
Hash do Código: f8499764

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/mod.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/mod.rs:1` (@prompt) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own only structural module declarations and direct reexports of constructors and internal bridges. Preserve bibliography, divider, document, footnote, heading, lists, markup, math, outline, par, quote, table_grid, table_lines and title composition, with existing test-only child modules.

The corresponding child L0s under `00_nucleo/prompts/compiler/stdlib/structural/` exclusively own argument validation, casts, defaults, Content construction, contextual realization and mathematical module composition. This hub owns no numbering table, math formula, outline geometry, header/footer parsing or show-rule policy. Keep the existing visibility of equation/math option helpers, outline page realization and public native constructors without wrappers or wildcard reexports.

Do not copy rendering into entities or remove native bindings as part of atomization. The global registry is a different owner; mathematical namespace construction remains with the math child.

## Rota de reexport dos helpers de matriz

**Medição anterior à decisão:** o working tree não commitado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-matrix-2026-09-30-before.json.gz`
mostra `mod.rs:105-114` a reexportar `MathMatrixOptions`, `math_matrix_augment`
e `apply_math_matrix_option` via o parent `math.rs`; consumidores em eval e
layout usam o bridge do stdlib. O filtro P1431 passa 29 testes na base.

**Decisão:** após a atomização, reexportar esses três nomes diretamente do
filho `math::matrix`, com as mesmas visibilidades e caminhos públicos de
`compiler::stdlib`. Este hub não recebe algoritmo nem wrapper. Não alterar
fase, diagnóstico, contrato público ou dispatch.

## Rota de reexport dos helpers de cases

**Medição anterior à decisão:** o working tree não commitado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-cases-2026-09-30-before.json.gz`
mostra `mod.rs:108-115` a reexportar `MathCasesOptions`,
`math_cases_delim` e `apply_math_cases_option` via `math.rs`; eval e layout
usam o bridge do stdlib. O filtro P1430 passa 22 testes na base.

**Decisão:** reexportar esses três nomes diretamente de `math::cases`, com as
mesmas visibilidades e caminhos de `compiler::stdlib`. Não adicionar wrapper,
algoritmo, dispatch ou fase ao hub.

## Rota de reexport de `math.accent`

**Medição anterior à decisão:** o working tree não commitado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-accent-2026-09-30-before.json.gz`
mostra `mod.rs:111-123` a reexportar `math_accent_content` e `native_accent`
via `math.rs`; eval e o registry usam esses nomes. O filtro P1429 passa 9
testes na base.

**Decisão:** reexportar os dois nomes diretamente de `math::accent` com a
mesma visibilidade e caminho público de `compiler::stdlib`. Não adicionar
wrapper, cast, algoritmo nem despacho a este hub.

## Rota de reexport de `math.cancel`

**Medição anterior à decisão:** o working tree não commitado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-cancel-2026-09-30-before.json.gz`
mostra `native_cancel` em `math.rs:395-529` e o hub a reexportá-lo via parent.
O filtro `native_cancel` passa 4 testes na base.

**Decisão:** reexportar `native_cancel` diretamente de `math::cancel`, com a
mesma visibilidade e caminho público de `compiler::stdlib`. Não acrescentar
wrapper, cast, algoritmo nem despacho a este hub.

## Rota de reexport de `sqrt_content`

**Medição anterior à decisão:** o working tree não commitado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-root-2026-09-30-before.json.gz`
mostra `sqrt_content` em `math.rs:46-48` e o hub a reexportá-lo pelo
parent; eval math mode consome o bridge de `compiler::stdlib`. O filtro
P1426 passa 12 testes na base.

**Decisão:** reexportar `sqrt_content` diretamente de `math::root`, com a
mesma visibilidade e caminho de `compiler::stdlib`. Não acrescentar
wrapper, cast, algoritmo ou despacho ao hub.

## Rota de reexport dos helpers de `math.equation`

**Medição anterior à decisão:** o working tree não commitado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-equation-2026-09-30-before.json.gz`
mostra `equation_content`, `equation_number_align`,
`equation_supplement` e `equation_alt` em `math.rs:46-112`, reexportados
pelo hub; eval e set rules usam o bridge de `compiler::stdlib`. O filtro
P1140 passa 85 testes antes do movimento.

**Decisão:** reexportar esses quatro helpers diretamente de
`math::equation`, com as mesmas visibilidades e caminhos. Não adicionar
wrapper, cast, algoritmo ou despacho ao hub.

## Rota de reexport de `math.class`

**Medição anterior à decisão:** o working tree não commitado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-class-2026-09-30-before.json.gz`
mostra a nativa em `math.rs:1069-1152` e o hub a reexportá-la via parent.
Os filtros `math_class` e P825 passam 29 e 15 testes na base.

**Decisão:** reexportar `native_math_class` diretamente de `math::class`,
com a mesma visibilidade e caminho de `compiler::stdlib`. Não adicionar
wrapper, cast, algoritmo ou despacho ao hub.

## Reexports diretos das famílias matemáticas restantes

**Medição anterior à decisão:** o snapshot
`00_nucleo/diagnosticos/atomizacao-stdlib-math-restante-2026-09-30-before.json.gz`
registra os bridges de `math.rs` reexportados por este hub. A suíte core
baseline passa 6.401 testes, com 2 ignorados.

**Decisão:** reexportar diretamente dos filhos as funções e bridges antes
reexportados via `math.rs`, preservando visibilidade e caminho
`compiler::stdlib`. Este hub não recebe casts, wrappers nem despacho
dinâmico; `make_math_module` permanece exportado pelo parent.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

## Política de hifenização e validação — 2026-09-28

### Medição anterior à decisão

`correcao-politica-hifenizacao-2026-09-28/before-pdf/resultados.json` conserva 52 entradas bilaterais, fontes, comandos e caixas, medidas no estado `initial.json`. As combinações absent/auto/false/true eram equivalentes no Tekt. O vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8` rejeita hyphenate integer/string, justify integer/auto e linebreaks fora de simple/optimized/auto; os diagnósticos literais e spans estão nos stderr do lote. Fonte: eval/rules.rs:2972-3010/3407-3420 descarta opções; stdlib/structural/par.rs:119-126 aceita/ignora; layout/break_selection.rs:39 não consulta política; vanilla text/mod.rs:545-565 declara Smart<bool> e model/par.rs:227-242/627-636 declara justify e Linebreaks. Isso mede comportamento e tipos, não demonstra a intenção histórica dos prompts.

### Decisão e escopo vigente

Correção interna de paridade ADR-0127: assinaturas públicas, entidades e fases permanecem; os owners abaixo legitimam apenas sua unidade. A restrição da atomização de não honrar propriedades descartadas é supersedida somente por este lote. O núcleo paragraph-break-policy fixa o invariante compartilhado; não é owner de código.

Adicionar regressões nativas de justify transportado, tipos inválidos e linebreaks validados. O teste antigo de justify mantém seu texto observável e passa a afirmar captura lexical; seu título deixa de prometer que se ignora. Nenhuma lógica produtiva muda neste hub.

## Composição de parágrafos — 2026-09-28

### Medição anterior à decisão

O lote before-pdf em correcao-composicao-paragrafos-2026-09-28 mede 34 entradas contra o pin 9f2b6e8715237cb086899a42873660fe744622e8, no estado initial.json e before/. Com 180pt e margem 8pt, a primeira linha justificada termina em 145.52pt no Tekt e 172.000002pt no vanilla; a última fica em 110.048pt em ambos. BEFORE/par[MIDDLE]/AFTER produz 2 linhas físicas contra 3. Empty-start põe AFTER em yMin 3.74 contra 18.14; empty-auto mede altura 24.568 contra 38.968. native-leading-two-lines demonstra que o escopo de leading precisa fechar antes de restaurar estilo. Fonte: line_commit.rs:195-236 já possui justificação; word_layout.rs usa flush ordinário em quebras; layout/mod.rs:1469 trata Par como transparente; style_scope.rs:18-77 restaura estilo depois do body; eval/rules.rs:663-705 agrupa fluxo para show. Agrupamentos XML do extrator não são linhas físicas: coordenadas coincidentes são medidas separadamente.

### Decisão vigente

Correção de paridade ADR-0127, em fluxo contínuo: sem nova assinatura pública, entidade, trait, flag ou fase. O núcleo pinado define invariantes compartilhados, não é owner de código. Preparação global e optimized ficam incompletos e pertencem à futura rodada-paragrafo-seletores, depois de medir escolhas/custos.

Atualizar o teste de representação de native_par para exigir o nó Par, preservando plain_text e os controles de erro/leading/justify. Adicionar controle de body vazio e Par dentro do Styled existente. Nenhuma lógica produtiva muda no hub.
