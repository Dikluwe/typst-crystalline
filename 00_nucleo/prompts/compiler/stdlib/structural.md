# L0 — stdlib/structural
Hash do Código: b91a5319

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:79283c9047419768139e0227b9ed7e34146a85706ed47731655736545c95306b
- 00_nucleo/prompts/_nuclei/selectors/math-native-bridges.toml sha256:19eb838b3e56711fa151d416bd24394a0a6f5c8286bb9ab2d0dd9bc542b4aee7

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

## Seletor nativo de math.equation — 2026-10-06

### Evidência antes da decisão

`00_nucleo/diagnosticos/plano-debitos-2026-10-06/math-before.json.gz`
conserva pin/HEAD/árvore/UTC/fontes/comandos/SHA. Equação explícita é locatable,
mas query(math.equation), alias e selector(math.equation) são rejeitados.
`01_core/src/compiler/stdlib/foundations/selector.rs:269-283` reconhece apenas
heading/figure/table/metadata/outline.entry por identidade estática e omite
o constructor já existente de equation. `structural/math.rs:97-103` registra
o mesmo constructor nativo no módulo público; o índice já trata Equation.
O pin devolve a equação nesses controles. ElementKind::Equation já existe;
não é necessário ampliar enums ou reconstruir a biblioteca para reconhecer.

### Decisão e aceitação

O hub estrutural expõe somente um bridge interno pub(super) para
math::equation::native_math_equation, com acesso limitado ao subtree stdlib;
não ampliar visibilidade de módulos, API exportada de compiler::stdlib ou
assinaturas. O owner selector acrescenta a função à tabela estática de
ElementKind::Equation usando fn_addr_eq, mantendo alias pela função real.
Não usar nome do Func ou string math.equation como identidade. Não executar
constructors, World ou make_math_module em produção para descobrir binding.

Testar parse_selector_arg/native_selector, alias nativo com nome diferente e
func diferente com nome math.equation. CLI deve observar número de equações e
ordem/fields dos elementos já existentes. Não envolver matemática bare em
Equation, alterar fase, promover todas as funções ou alegar correção da
morfologia de matriz. .where é responsabilidade de element_where e fica
fora deste lote; table.cell também permanece independente. Correção interna
de mapeamento/paridade em fluxo contínuo ADR-0127.

## D05 — equation.where, correção interna de paridade

### Medição antes da decisão

Coleta equation-where-2026-10-06/before.json.gz registra pin
9f2b6e8715237cb086899a42873660fe744622e8, binários SHA-256, UTC, HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b e diff HEAD --stat/estado exato.
Tekt rejeita equation.where como função não-elemento; vanilla aceita catálogo
público, filtros block/label/body/numbering/alt, alias, composição e show.
Vanilla rejeita campos size/missing, posição, closure e aplicação parcial.
Fonte upstream: foundations/func.rs:433–462 valida field_id e mantém grupo;
math/equation.rs:58–170 separa campos públicos dos internos.
Tekt element_where.rs:63–490 omite equation; selector_matching.rs:269–310
omite Equation no matching nativo. O getter block/body e snapshot já existem.

### Decisão e aceitação

Supersede neste recorte a restrição D04 do bridge ao subtree stdlib: reexportar
o mesmo native_math_equation com pub(in crate::compiler), permitindo apenas
consumers internos eval. Não alterar visibilidade dos módulos math/equation,
assinatura do constructor ou namespace Typst; não criar wrappers/factories.
Não é API Rust exportada para usuários da crate.

Fluxo contínuo ADR-0127: correção de tabela interna, sem novo contrato público
ou fase. RED→GREEN dos consumers e revalidação bilateral dos controles;
preservar D04 e não realizar math bare/D06 ou morfologia de matriz/D07.

## D07 — correção de morfologia de matriz, fluxo ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
00_nucleo/diagnosticos/matrix-morfologia-2026-10-06/before.json.gz guarda
UTC/diff/fontes/binários e casos contra o pin 9f2b6e8715237cb086899a42873660fe744622e8.
Mat raw/query sai como {func:math.matrix} sem rows; getter/métodos perdem campos
e func-equality retorna false. Fonte upstream math/matrix.rs:91–221 declara
propriedades e rows. Tekt MathMatrixElem conserva rows/explicit em math_matrix.rs:45–52,
mas Element.get_field é default; content_methods.rs:151–191 não enumera mat;
cli.rs:1154–1348 cai no fallback body. É morfologia/semântica da linguagem,
não exigência de estrutura Rust ou bytes PDF; nenhuma intenção foi inferida.

### Responsabilidade exclusiva

Reexport privado pub(in crate::compiler) de math::matrix::native_math_mat, somente para identidade no getter func. Sem wrapper/execução de constructor, mudança de assinatura pública, algoritmo ou fase.

### Aceitação

RED→GREEN e coleta bilateral de fields/has/at/func, JSON/query, padding e
presença; build/lint e regressões D05/D06/query. Gate contínuo: contratos
públicos de tipos/traits/assinaturas e fase ficam iguais. Não certificar
mat selectors, render, Styled geral ou outros elementos não medidos.

## Integração condicional dos módulos de teste — 2026-10-13

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado. A duplicação cfg(test) ocorre em `01_core/src/compiler/stdlib/structural/p1429_tests.rs:5` / pai :38; `01_core/src/compiler/stdlib/structural/p1430_tests.rs:5` / pai :42; `01_core/src/compiler/stdlib/structural/p1431_tests.rs:5` / pai :46; `01_core/src/compiler/stdlib/structural/p1436_tests.rs:5` / pai :50; `01_core/src/compiler/stdlib/structural/p1437_tests.rs:5` / pai :54; `01_core/src/compiler/stdlib/structural/p1438_tests.rs:5` / pai :58; `01_core/src/compiler/stdlib/structural/p1439_tests.rs:5` / pai :62; `01_core/src/compiler/stdlib/structural/p1440_tests.rs:5` / pai :66; `01_core/src/compiler/stdlib/structural/p1441_tests.rs:5` / pai :70; `01_core/src/compiler/stdlib/structural/p1442_tests.rs:5` / pai :73; `01_core/src/compiler/stdlib/structural/p1443_tests.rs:5` / pai :76; `01_core/src/compiler/stdlib/structural/p1444_tests.rs:5` / pai :78; `01_core/src/compiler/stdlib/structural/p1445_tests.rs:5` / pai :80; `01_core/src/compiler/stdlib/structural/p1446_tests.rs:5` / pai :82; `01_core/src/compiler/stdlib/structural/p1447_tests.rs:5` / pai :84; `01_core/src/compiler/stdlib/structural/p1448_tests.rs:5` / pai :86; `01_core/src/compiler/stdlib/structural/p1449_tests.rs:5` / pai :88; `01_core/src/compiler/stdlib/structural/p1450_tests.rs:5` / pai :90; `01_core/src/compiler/stdlib/structural/p1451_tests.rs:5` / pai :92.
Fontes, L0, UTC, status e diffstat estão em
`00_nucleo/diagnosticos/cfg-test-ci-2026-10-13/before.json.gz` e
`refinement-before.json.gz`. O Clippy pinado reproduz os atributos duplicados.
A tentativa de retirar a condição interna mantém os testes Rust, mas o linter
arquitetural pinado não propaga a condição do pai e acusa V2 nos helpers.

Neste owner, retirar apenas o atributo externo cfg(test) das declarações
listadas acima. Manter os caminhos, nomes e visibilidades dos módulos e seus
atributos path. A condição interna `#![cfg(test)]` de cada arquivo listado
continua a excluir integralmente o módulo do build de produção, inclusive
seus imports e helpers. Os consumers de teste e seus L0 permanecem intactos.
Não acrescentar dependências, lógica, API, modo padrão ou fase. Validar a
mesma lista de testes, suíte core, build sem cfg(test), Clippy e arquitetura.

## Imports de produção e teste — 2026-10-13

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. 01_core/src/compiler/stdlib/structural/mod.rs:100,107,1079; 01_core/src/compiler/stdlib/structural/math/matrix.rs:81,163,406 registra a composição e os usos existentes. O inventário Clippy distingue imports sem uso na compilação produtiva; os usos de teste foram conferidos separadamente. Fontes, L0, UTC, status e diffstat estão em `00_nucleo/diagnosticos/imports-ci-2026-10-13/before.json.gz`. Não é medição de paridade ou desempenho.

### Decisão e aceitação

A reexportação native_math_class no hub privado existe somente sob cfg(test); o catálogo produtivo usa diretamente o filho class. Superseder a obrigação anterior de reexportar math_matrix_augment por este hub: seus usos estão no próprio filho matrix, então remover somente esse alias crate-interno. Preservar MathMatrixOptions, apply_math_matrix_option, a função original e suas chamadas.

Correção de composição interna, sem mudança de assinatura pública, defaults, fase ou comportamento da linguagem. Preservar cenários/assertivas, executar a suíte core, build, Clippy e linhagem. Não mascarar avisos com allow/expect.

## Complemento de casts de figure — Type e Symbol, 2026-10-17

Medição anterior à decisão: type-kind-probe.json e more-cast-probe.json em
00_nucleo/diagnosticos/tekt-v16-figure-casts-2026-10-17 registram o candidato
que passou 5/6 testes mas falhou em kind:int. foundations/value.rs:647-658 do
pin 9f2b6e8715237cb086899a42873660fe744622e8 mede Func aceitando Type.constructor
ou Symbol.func; content/element.rs:210-214 exige to_element; symbol.rs:133-137,
math/accent.rs:189-214 e math/lr.rs:144-198 classificam a conversão simbólica.
Há 37 tipos exercitados no CLI; Location sem construtor foi conferido na
fonte pinada. type-kind-classes.json guarda a classificação completa dos 38.
Símbolo aceito como corpo/caption é convertido pelo pin a SymbolElem; o nosso
Content não tem essa variante e o markup atual conserva o valor como Text.
Esse carrier/morfologia exige evolução pública SY1, não pode ser declarado
resolvido pela preservação do texto nesta etapa.

Decisão exclusiva deste owner:

Reexportar diretamente, somente pub(crate), is_known_math_accent do owner
math/accent e is_callable_lr_symbol do owner math/lr. Não mover tabelas,
classificação ou lógica ao hub; nenhuma API externa ou registro novo.

Aceitação: L0 primeiro, RED funcional de Type/Symbol/valor textual antes do código,
GREEN e revalidação integral desta etapa; 12 GiB/swap zero/jobs1. Fluxo contínuo
ADR-0127, sem contrato Rust público, fase ou default de produto novo. Resselo só
dos owners após V15/V26; preservar testes de matemática e classificadores originais.


## Identidade matemática nativa — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado; pin `9f2b6e8715237cb086899a42873660fe744622e8`.
`00_nucleo/diagnosticos/native-math-catalogue-2026-10-17/` fixa o estado
exato em before.json.gz: UTC, diff HEAD --stat e hashes de todos os consumers,
L0 e configuração protegidos. before-measurement.json.gz é reutilização
verificada da matriz final da etapa anterior: fontes e binário Tekt têm
identidade de hash com o baseline atual (baseline-reuse.json), não uma nova
execução. A matriz cobre os oito perfis de features e 3956 casos de funções;
26 rotas math ainda são recusadas em where/kind pelo Tekt e aceitas pelo pin.
before-batch.json.gz mede 227 casos detalhados de campos, exclusões, alias
parcial e counter, com comandos/binários/saídas e seus hashes.

batch-schema.json conserva blocos e SHA-256 das fontes primárias do pin,
file:line e declarações de ABI do Tekt; typst-macros/src/elem.rs:410–421
exclui internal/external de field_id. As anotações elem destas 26 funções
contêm Mathy e não Locatable. Isto é semântica/forma pública da linguagem,
não igualdade mecânica de Func nem inferência de intenção por render.

### Responsabilidade exclusiva e decisão

Este owner propaga diretamente as 23 pontes internas declaradas no owner
`00_nucleo/prompts/compiler/stdlib/structural/math.md`, de `math` ao hub
estrutural, com `pub(in crate::compiler)`. Reexportar os mesmos símbolos,
sem wrappers, schema local, lookup por nome ou mudança nos reexports públicos
atuais, inclusive o native_math_class público somente em cfg(test).

### Aceitação e limites

Invariantes compartilhadas: consumir o Núcleo Tekt pinado
`00_nucleo/prompts/_nuclei/selectors/math-native-bridges.toml`; as
responsabilidades exclusivas acima definem o segmento de cada owner.

Correção interna de tabela/composição em fluxo contínuo ADR-0127. Nenhum campo,
trait, assinatura Rust externa, compatibilidade, default ou fase muda. L0
antes dos testes/código; RED funcional por identidade ausente, GREEN dos 26
casos com filtros e exclusões; controles de With, homônimo e helpers que não
são elementos. Validar antes/depois toda a matriz, preservando observáveis
fora das identidades autorizadas e aliases efetivos (accent/cancel globais
cristalinos incluídos, sem crédito de paridade se não existem no pin).

O sucesso de where/kind não certifica getters, síntese, indexação, show,
layout ou counter. Os controles counter permanecem explícitos e a sua
admissão separada não é alterada neste lote. As quinze rotas restantes de
formatos/HTML/PDF têm investigação própria; a colisão de callback dos formatos
não é resolvida por nomes. FG1/RM1/SY1/RG1 preservam escopos e gates.

Verificar ownership/V26 antes do resselo restrito aos cinco pares; build,
suíte completa, fmt, Clippy/Rustdoc estritos, linhagem e diagnóstico global
sem nova violação, sob MemoryMax12GiB/High10GiB/swap0/jobs1 sequenciais.


### Posição das pontes no hub — correção Clippy

Medição anterior à decisão: candidate-1/strict-details.json em
00_nucleo/diagnosticos/native-math-catalogue-2026-10-17 registra
clippy::items_after_test_module no hub estrutural:137/1199. Fontes, hashes
e comandos congelados dessa tentativa estão em candidate-1/state.json.gz.
O conjunto de callbacks e seus corpos já passou os testes focados e a
matriz; a posição do item após o módulo inline de testes é a falha nova.

Manter o reexport interno deste lote antes da declaração de módulo de testes,
junto dos itens de composição. Mover apenas o item, preservando seus símbolos,
ABI, visibilidade e todos os itens preexistentes. Não adicionar allow/expect.
Revalidar a versão final ressela­da: fonte mudou, portanto as medições da
tentativa anterior são histórico e não substituem os gates da versão final.
