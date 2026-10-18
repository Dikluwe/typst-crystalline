# L0 — compiler/eval/tests
Hash do Código: f6c45ad5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/eval/source-validation.toml sha256:e29edf5bc71d0c7f8b45d0a2a0db43820151a7d80f7484197c56dc8b3830d774
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:fb383f227b0bcd8b1e61899942fa0383a4cda66093da58bda2e87844e2895d2a
- 00_nucleo/prompts/_nuclei/layout/typc-atom-display.toml sha256:2989f08ae6b4b379afb61d9673b94ab8538c45a97b98e566677ae0f719429dc3
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:f29e26ff07053f940cfe4ab5358fc9a6ac8d4d689528e169b5f8567492f52f46
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:1d697fae328277525e44cad674cecc537ceb23a1bb3ae11bb68a0a9575411ae4


**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/tests.rs:26` — `fn eval_for_test`.
- `01_core/src/compiler/eval/tests.rs:36` — `fn eval_for_test_keep_sink`.
- `01_core/src/compiler/eval/tests.rs:60` — `fn eval_for_test_with_registry`.
- `01_core/src/compiler/eval/tests.rs:85` — `fn eval_for_test_full_error`.
- `01_core/src/compiler/eval/tests.rs:112` — `fn eval_for_test_with_limits`.

## Contract

Own only the eval unit-test consumer, pure test Worlds, fixtures and test helpers,
not productive evaluation code. Test public semantics, syntax, morphology and
complete observable diagnostics/spans rather than Debug, pointer addresses,
render-byte identity or an expectation copied from the candidate.
For collection method calls, test that a receiver block with a local mutation is
evaluated once. Baseline measured on HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`
with the uncommitted tree captured in the item 6 experiment: vanilla pinned
`9f2b6e8715237cb086899a42873660fe744622e8` produces `(3, 1)` for
`({ n += 1; (1, 2, 3) }).len()`, while the prior Tekt binary produces `(3, 2)`.
This is a language-level observable; the source-level inference is duplicate
receiver evaluation in `instance_methods.rs:18` and `collection_methods.rs:18`.

Cover literals/units, expression/block/array/dictionary/spread order, scopes,
bindings/destructuring/assignment, closures/With, operators/short-circuit, loops,
imports/includes/modules, static/bound calls, field/content/snapshot methods and
Content construction. Positive tests retain values/effects; negatives retain
severity, exact message, hint/trace order and publicly available UTF-8 half-open
ranges. Preserve legitimately absent sources without inventing anchors.

Repeat applicable feature cases under default/html/a11y/html+a11y and the independent
bundle combinations. Disabled profiles do not count as exercised parity; local
homonyms remain resolvable. Test asset/std.asset/format.bundle against the real
active constructor identity, path normalization, UTF-8/arbitrary Bytes and schema
errors (including prohibited kind), not an old active-feature placeholder.
Actual target export obligations remain with L3/L4 tests.

Imports distinguish bare no-effect identifier warning from alias/field/items/
wildcard/file/dynamic/failure cases; warnings survive later errors. Module field
diagnostics use stored names and field-only spans; std aliases retain actual
identity and imported sources retain Source. Test global color membership and
PDF accessibility gates without giving this test prompt registry ownership.
PDF type/repr are function/pdf; emoji remains module.

Array repr tests cover boundary cardinalities (0/1/39/40/41/42/81/256), nested
escaping/Unicode and full payload conservation. Elision cannot leak into Args or
Content sequences. Named-module, alias, reexport and collision tests retain stored
names. JSON/TOML/YAML default/pretty/compact, CBOR, decoders/read and nonfinite/-0.0
cases preserve their own contracts, not generic warning/trace normalization.

Args tests preserve complete occurrence order, repeats, individual and aggregate
origins through direct/alias/With/spread/sink/linked calls and distinct equal-output
factories. Callback filter/map/join order and short-circuit remain discriminatory;
arguments.filter bool failures anchor at function origin while callback failures
retain their own causal origin. Queried-content tests retain authoritative ordered
snapshot fields through arrays/closures; language equality ignores label/location
as specified but detects relevant field/numbering changes. Do not replace it with
the internal observation relation.

Textual tests separate raw ContentBlock repr from realized flow: preserve Space
after Linebreak and between newline-separated CJK before interpolation, normalize
only later insertion. Test origin/collapse/discard independently from literal/regex
cross-node matching. Keep explicit-space, Latin/Korean, ideographic punctuation,
ambiguous quotes, comments, strong/dynamic, destructive Linebreak, style-chain and
nontext boundaries as controls. Local pre-normalized show replaces only its match;
combined local CJK show and sequential identical-rule override remain separate
unresolved parity cases, not GREEN credited by these tests.

Math tests preserve real syntax/qualified construction, arity/casts, presence and
identities. Keep primes slot/nesting distinctions, cases MathAlignPoint and lr body
order. norm/round/abs require their public MathLr bodies with ‖/‖, ⌊/⌉ and |/|;
do not require historical MathDelimited lowering. lr cases likewise retain public
MathLr while unrelated MathDelimited cases keep their own contract. Annotation
families require each actual public variant/body/annotation/explicit-none state,
not glyph equivalence or MathUnderover. Mathematical overline is not text overline.
Test func/has/at/fields alongside repr where contracted. before/after tests cover
construction/default inclusive and explicit false; contextual query tests belong
to introspection's exclusive test owner.

Keep regression families for bytes, gradients, datetime, int, counter and scope,
with pure Worlds and independently frozen expectations. Neutral rounding values
are fixture mechanics, not public contracts.
No parser/build/fixture/import failure may masquerade as the expected result.
Unknown/timeout/opaque observation fails closed. Repeat/reverse orders must be
stable and comparisons may not strip diagnostics to manufacture equality.

## Política de hifenização e validação — 2026-09-28

### Medição anterior à decisão

`correcao-politica-hifenizacao-2026-09-28/before-pdf/resultados.json` conserva 52 entradas bilaterais, fontes, comandos e caixas, medidas no estado `initial.json`. As combinações absent/auto/false/true eram equivalentes no Tekt. O vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8` rejeita hyphenate integer/string, justify integer/auto e linebreaks fora de simple/optimized/auto; os diagnósticos literais e spans estão nos stderr do lote. Fonte: eval/rules.rs:2972-3010/3407-3420 descarta opções; stdlib/structural/par.rs:119-126 aceita/ignora; layout/break_selection.rs:39 não consulta política; vanilla text/mod.rs:545-565 declara Smart<bool> e model/par.rs:227-242/627-636 declara justify e Linebreaks. Isso mede comportamento e tipos, não demonstra a intenção histórica dos prompts.

### Decisão e escopo vigente

Correção interna de paridade ADR-0127: assinaturas públicas, entidades e fases permanecem; os owners abaixo legitimam apenas sua unidade. A restrição da atomização de não honrar propriedades descartadas é supersedida somente por este lote. O núcleo paragraph-break-policy fixa o invariante compartilhado; não é owner de código.

Adicionar regressões dos tipos e diagnósticos nas formas set/native, valores válidos e transporte lexical, incluindo custom Auto e Bool. Migrar o canary de warning de hyphenate para stroke, ainda não capturado, sem eliminar sua asserção. Nenhuma tolerância, fixture de geometria ou asserção preexistente de linguagem é enfraquecida.

### Integração antes do ajuste de canary

first-core-integration.log registra 6316 passados e uma falha: o canary P134 exige warning de justify descartado. Preservar seu teste de propriedade versus target desconhecido usando first-line-indent ainda não consumido; incluir leading válido e confirmar que não avisa. Não remover a asserção de warning.

## Composição de parágrafos — 2026-09-28

### Medição anterior à decisão

O lote before-pdf em correcao-composicao-paragrafos-2026-09-28 mede 34 entradas contra o pin 9f2b6e8715237cb086899a42873660fe744622e8, no estado initial.json e before/. Com 180pt e margem 8pt, a primeira linha justificada termina em 145.52pt no Tekt e 172.000002pt no vanilla; a última fica em 110.048pt em ambos. BEFORE/par[MIDDLE]/AFTER produz 2 linhas físicas contra 3. Empty-start põe AFTER em yMin 3.74 contra 18.14; empty-auto mede altura 24.568 contra 38.968. native-leading-two-lines demonstra que o escopo de leading precisa fechar antes de restaurar estilo. Fonte: line_commit.rs:195-236 já possui justificação; word_layout.rs usa flush ordinário em quebras; layout/mod.rs:1469 trata Par como transparente; style_scope.rs:18-77 restaura estilo depois do body; eval/rules.rs:663-705 agrupa fluxo para show. Agrupamentos XML do extrator não são linhas físicas: coordenadas coincidentes são medidas separadamente.

### Decisão vigente

Correção de paridade ADR-0127, em fluxo contínuo: sem nova assinatura pública, entidade, trait, flag ou fase. O núcleo pinado define invariantes compartilhados, não é owner de código. Preparação global e optimized ficam incompletos e pertencem à futura rodada-paragrafo-seletores, depois de medir escolhas/custos.

Adicionar testes de repr/par nativo, preservação do body e realização show-par com estilos inline, verificando fronteiras reais sem Par extra por wrapper. Manter diagnósticos/política do lote anterior e asserções restantes.

### Refinamento de precedência medido antes da implementação

No estado precedence-state.json, regression-acceptance.json registra 191 verificações passando e duas falhas no controle show-par-sets-false. O Tekt emite extraordi-/interna- onde vanilla conserva palavras sem hifenização. show-par-full-tree.log mostra SetPage e o escopo global dentro de um Par sintetizado; a recipe fica exterior ao text.hyphenate true. eval/mod.rs:1532-1560 possui a proveniência exata dos escopos #set; rules.rs:660-727 apagou essa distinção ao tratar Styled sempre como inline. Não inferir origem pelo conteúdo do Styles.

Decisão interna: o fold dos escopos #set em eval_markup transporta marcador privado eval.flow.scope=true no Styles existente. Antes de formar o primeiro parágrafo material, realize_flow reconhece esse marcador e mantém o escopo exterior à realização dos seus parágrafos, preservando os carriers prévios SetPage/Empty/Space/SpaceFromNewline em ordem, sem os embrulhar em Par. Após conteúdo inline material o escopo permanece inline dentro do parágrafo corrente; wrappers de constructors/estilos sem marcador permanecem inline. Nenhuma entidade, assinatura, comportamento válido por defeito ou fase pública muda. Pin no núcleo compartilhado paragraph-composition documenta a distinção; não recorrer a heurística por chave de estilo. O consumer do hub só produz o marcador no fold; preparação e reconhecimento são do owner rules.

## Transporte de seleção — 2026-09-28

Medição antes da decisão: eval/rules.rs:3070 e native par.rs:131 validam sem transportar; before-pdf/resultados.json da rodada correcao-seletores-paragrafo-2026-09-28 refuta estratégias distinguíveis. Ampliar o teste existente de canais lexicais para par.linebreaks nas formas set/native simple/optimized/auto. Sem mudança de linguagem ou mecanismo público; casos inválidos existentes mantêm casts e spans.

## Remoção de hardcode funcional — 2026-09-28

### Medição anterior à decisão

`correcao-hardcode-paragrafo-2026-09-28/before-pdf/` mede 20 entradas contra
9f2b6e8715237cb086899a42873660fe744622e8 no estado `initial.json` e `before/`.
`eval/rules.rs:3483-3525` e `stdlib/text/constructor.rs:153-190` não capturam
`overhang`; auto/int/string são aceitos indevidamente. A fonte vanilla
`text/mod.rs:391-393` declara bool default true; `inline/line.rs:518-525`
condiciona a compensação ao estilo final. `paragraph_hyphen.rs:221-224`
sempre multiplica a largura por0.55. A suspeita inicial sobre `paragraph_select.rs:78-82` foi refutada antes de
implementar: vanilla `inline/linebreak.rs:551-559` usa attempt.justify, que
`inline/line.rs:138-139` desativa para mandatory. Conservar a fórmula atual
e testar a última linha natural. Isto mede comportamento, não intenção histórica.

### Decisão vigente

Correção interna/de paridade em fluxo contínuo ADR-0127, sem campo ou assinatura
pública. Não confundir constantes normativas de um algoritmo com configuração
ignorada. O fator0.55, custo-base135, distância5 e incremento0.15 vêm da fonte
pinada; identificar essas constantes pelo significado e referência, sem alegar
que renomear números implementa configuração. Custos customizados continuam
escopo pendente. Conservar identidade textual literal do hífen nesta rodada:
ela requer carrier/contrato separado, não heurística sobre glyph ou posição.

Adicionar controles set overhang bool true/false sem warning descartado e erro de auto/int/string com mensagem esperada; não mudar snapshots existentes.

## Custos textuais personalizados — 2026-09-28

### Medição anterior à decisão

`correcao-custos-paragrafo-2026-09-28/before-pdf/` registra 66 entradas no
estado `initial.json` e `before/`, contra pin 9f2b6e8715237cb086899a42873660fe744622e8.
`text/mod.rs:1507-1564` valida Dictionary, Ratio por campo e composição parcial;
`inline/mod.rs:240` lê costs do escopo compartilhado, não de cada fragmento;
`inline/linebreak.rs:29-30,961-962` aplica defaults 135/100 vezes o ratio.
O Tekt descarta costs em eval/rules e aceita sem transportar no constructor;
paragraph_hyphen e paragraph_select usam defaults fixos. Os erros e spans
estão no stderr vanilla, inclusive negativos aceitos. Não inferir intenção.

### Decisão vigente

Correção interna/de paridade ADR-0127 em fluxo contínuo, sem novo contrato
público, flag ou fase. Módulo privado compiler/text_costs possui owner único
para cast, transporte por componente e resolução dos custos compartilhados.
Hifenização/runt globais passam a consultar a configuração. Widow/orphan são validados e transportados; sua influência é limitada ao subconjunto regional certificado descrito abaixo.
Custos de fragmentos parciais não substituem o custo compartilhado; os quatro
canais validados são neutros para glyphs na whitelist Styled. Um wrapper integral
é fatorado pela realização existente antes de formar Par. Largura variável
permanece limitada. Não deduzir custo pelo glyph. Preservar todos os defaults.

Testar dicionário/tipos inválidos, prioridade normativa da primeira falha e custos válidos sem warning descartado. Inputs/mensagens derivados das sondas/fonte.

### Refinamento de custo compartilhado medido antes de implementar

after-first-resultados.json/proveniencia.json registra 66 entradas com o primeiro
binário de custos. Só restore e inline divergem entre os inputs válidos. A fonte
`inline/mod.rs:240` consulta shared.get(TextElem::costs); `eval/rules.rs` conserva
fatoração dos Styled que cobrem todo o material do fluxo antes de formar Par.
`paragraph_prepare.rs:45-78` recusa os novos canais mesmo quando eles não alteram
nenhum atributo de glyph: o fallback incremental produz escolhas divergentes.
A exclusão absoluta desses canais foi refutada; isto supersede o scope-out
anterior, sem promover fragmentos ao escopo exterior nem mudar fase de pipeline.

Permitir exclusivamente text.costs.hyphenation/runt/widow/orphan na whitelist
Styled da preparação. O custo lido pelo planejamento é o escopo compartilhado
exterior: push temporário da preparação é restaurado e não altera tal leitura.
Inline parcial não escolhe custos do parágrafo; Styles integral continua a ser
fatorado pelo mecanismo existente de realização e entra na chain antes de Par.
Não criar espaço ou fronteira de palavra por diferenças de costs. Manter
limitações físicas widow/orphan e contextos fora do subconjunto certificado.

## Proteção regional de viúvas/órfãs — 2026-09-28

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-viuvas-orfas-2026-09-28/before-pdf/` registra 78 entradas e `before-extra/` registra 24 controles, com estado completo em initial.json e before/, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. page-65-100%-0%-False deixa três linhas e uma no Tekt, mas duas e duas no vanilla; custos zero coincidem. Tiny/large-positive têm a mesma proteção vanilla. `lab/typst-original/crates/typst-layout/src/flow/collect.rs:196-229` mede habilitação estritamente positiva, primeiros dois/últimos dois e proteção integral de três linhas; `flow/distribute.rs:343-364` só avança por necessidade agrupada quando cabe na próxima região. paragraph_emit.rs emite linha por linha sem essa necessidade. A semântica observável é a distribuição de linhas/palavras em regiões, não os frames/Child do vanilla. Não inferir intenção do scope-out antigo.

### Decisão vigente

Correção interna/de paridade em fluxo contínuo ADR-0127: nenhum campo/trait/assinatura pública, flag ou fase nova. Supersede a exclusão anterior de widow/orphan exclusivamente para parágrafos preparados com métricas verticais uniformes, sem baseline offset, geometria paginada constante, sem colunas/células/subframes/notas/floats ou transação regional ativa. Os demais contextos conservam fallback e limites explícitos. Não promete algoritmo completo nem desempenho.

O módulo privado paragraph_pagination possui exclusivamente certificação vertical, agrupamento e admissão regional. Recebe ranges já escolhidos, não refaz seleção, não emite glyphs nem duplica a lógica de new_page. A certificação inteira precede emissão; todos os fragmentos devem ter size/arestas/leading iguais ao estilo exterior (negrito/itálico admissíveis se as métricas coincidirem), e finite nonnegative top/descent/leading. Bounds auto não precisam de proteção. Respeitar a baseline inicial e o colapso existente de block-chain antes de comparar necessidade real com page_bottom_limit. Novo espaço disponível é calculado com margens resolvidas para a página seguinte. Avançar só se há material na região atual e o grupo cabe na próxima. Grupos muito altos conservam progresso ordinário e conteúdo íntegro; nunca repetir new_page em laço.

Helpers privados de text_costs resolvem widow/orphan independentemente dos pesos hyphenation/runt, reutilizando FIELDS, com default Ratio 1 e teste estrito > 0. O default de estratégia do planejamento passa a Simple para parágrafo comum sem linebreaks, permitindo a preparação necessária à proteção padrão; a escolha automática por justify e todos os contextos recusados permanecem. Emitir sem alterar justificação, carriers, última linha aberta, fronteiras e restauração lexical. Preservar notas e colunas por não aplicar proteção incompleta nesses caminhos.

Aceitação: RED→GREEN com mesmas palavras distribuídas 2+2 versus 3+1, proteção integral de três, orphan desligado independentemente, grupo excessivo, restauração e default sem flag. Sondas bilaterais com caixas/palavras/páginas, tipos/spans anteriores preservados e controles de fallback. Não usar grupos XML como número físico de linhas. Resselo restrito aos owners, build e crystalline-lint sem erros.


## Realização com avaliação única

Acrescentar sentinelas de avaliação comum única mediante contagem de leituras de World e de query dos elementos original e gerados por show. Preservar helpers e demais testes; a contagem é estado local do MockWorld, sem global mutável.


## Realização com avaliação única

Testar execução comum única também com show; retorno it não duplica origem; duas headings produzidas com corpo igual conservam ocorrências distintas. Comparar observáveis de linguagem e índices causais, não igualdade morfológica como identidade.


## Realização com avaliação única

O adapter de testes legados projeta content visualmente para manter asserts antigos sobre estrutura de render. introspection_content conserva integralmente proveniência para testes de query/identidade; não usar projeção visual como índice semântico.


## Realização com avaliação única

As sentinelas de ocorrência consultam a árvore semântica preservada no módulo, enquanto assert visual usa a projeção do adapter legado.


## Realização com avaliação única

O adapter legado devolve a projeção visual da árvore realizada; conserva introspection_content integral para assert semântico de identidade. A projeção não altera o consumer produtivo.


## Realização com avaliação única

Receitas observam location da ocorrência original e campos do elemento com transparência dos carriers; devolver it conserva uma única ocorrência.


## Realização com avaliação única

Contadores seguem ordem causal de origem e produto, não ordem numérica da identidade. A sonda vanilla 9f2b6e871 de it duas vezes conserva uma ocorrência consultável e duas emissões visuais; testar distinção entre alias preparado e inserções comuns ainda não realizadas.


## Reaproveitamento do índice realizado — 2026-10-05

Adicionar sentinelas de memoização do resultado semântico: eval comum com show disponibiliza no Module o índice original/gerados; sem show o snapshot é ausente e permanece o fallback; conteúdo com colunas e show conserva a mesma sequência de corpos/labels/contadores numa reconstrução fresca. Testes observam o contrato de Module e índices da linguagem, não a igualdade visual de Content. O gateway público foi confirmado pelo usuário em `00_nucleo/prompts/entities/module.md`.

## Recursão por ocorrência — 2026-10-06

Medição: show-recursao-auditoria-2026-10-06/measurement.json.gz conserva quatro gerações infinitas aceitas indevidamente e quatro controles finitos contra o pin 9f2b6e871. Após confirmação do owner apply, acrescentar regressões produtivas para essas rejeições com mensagem e hints, alias duplo, where finito, múltiplas regras, efeitos e contexto lexical. Não alterar testes isolados do helper legado de ponto-fixo.

core-first.log mede cinco falhas em expectativas produtivas P348/P350c da política revogada. Atualizar essas expectativas para rejeição de geração ilimitada com os dois hints padrão e classificação adicional quando full_error. Preservar o teste produtivo a→b→c→it e todos os testes isolados do helper legado. controls-first.json refuta duplicação em block e perda da regra local por colisão de ID; acrescentar testes de alias em container, composição com regra anterior e regra local aplicada a it.

markup-span-red.json mede rejeição de heading de markup sem origem no Tekt e com range = Z no vanilla. Acrescentar controle de range causal real para esse construtor e para heading[Z], sem comparar números internos de Span nem retirar hints.

## Identidade de documento e expressão — gate confirmado 2026-10-06

### Evidência anterior à decisão

`00_nucleo/diagnosticos/diagnostic-paths-2026-10-06/identity-next-audit.json`
conserva comandos, fontes, UTC, HEAD, árvore alterada e hashes dos binários do
Tekt e do vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8. Os casos
separam erro/warning do documento de erro da expressão: o documento perde sua
fonte, enquanto a expressão recebe indevidamente o nome do documento.
`01_core/src/compiler/eval/mod.rs:609-615` usa world.main() para Source e base;
`04_wiring/src/eval.rs:66-95` repete esse id ao fornecer o primary ao formatter.
`03_infra/src/world/cache.rs:58-78` registra arquivos e recicla o contador;
`lab/typst-original/crates/typst-cli/src/eval.rs:159-179` e
`lab/typst-original/crates/typst-cli/src/world.rs:176-190` distinguem expressão,
stdin e ausência de entrada por identidade. A igualdade dos inteiros e o
interner global upstream são mecânica, não requisito (ADR-0107). A origem
mostrada, fonte, posição e resolução relativa são observáveis de linguagem.

### Decisão e escopo do gate

Testar a fachada Source-based com World puro: Source de expressão de id
distinto do main, erro/syntaxerror na própria expressão e preservação do Source
principal. Verificar read/import pela base world.main(), callbacks com sua
origem própria, scopes frescos e profiles de features. Manter controles das
fachadas string compatíveis. Mock não deve obter expectativa do candidato.

Gate ADR-0127 confirmado pelo dono em 2026-10-06 (resposta: confirmo), conforme
00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json.
Consumer exclusivo preservado; Hash do Código acompanha a materialização.

## Alcance de warnings vazios — 2026-10-06

Evidência: empty-markup-warning-2026-10-06/before.json mede caret de dois
caracteres para **/__ contra o pin, inclusive após Unicode e em adjacência.
Testar ranges UTF-8 publicados por eval, source id, mensagem/hint/status,
newline/trivia fora do range, dois constructs adjacentes e empty dentro de
content block. Conteúdo não vazio não ganha warning e delimitador aberto
continua erro. Não alterar expectativas de warnings genéricos do parser.

## Reparos independentes do gate de aspas — 2026-10-06

Medição anterior à decisão: smartquote-2026-10-06/{before,fields-before}.json.gz,
com pin/HEAD/working tree/UTC e binários exatos registrados.

Testar show com alias da função smartquote e chamada programática, exigindo
texto Q. O resultado do markup literal é guardado para o gate de fase, sem
reclassificar a rejeição original como sucesso. Fixtures puras atuais bastam.


## Aspa semântica e resolução tardia — ratificada em 2026-10-06

O contrato desta seção é vigente e supersede as cláusulas históricas conflitantes
das seções anteriores apenas para SmartQuote. A ratificação humana autoriza a
materialização em fluxo RED→GREEN, com ownership exclusivo e pins abaixo.

### Medição anterior à decisão

smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz
registra pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 dos binários,
HEAD/working tree, diff HEAD --stat, UTC, fontes, argv/cwd/ambiente e stdout/stderr.
PDF usa texto extraído, HTML usa DOM/texto e diagnósticos, não igualdade de bytes PDF.
A avaliação converte aspas de markup para Text; campos double/enabled são perdidos.
A variável contendo aspas mantém glyph inglês sob uso em alemão/francês ou
smartquote(enabled:false). Primes 6'/2" viram aspas no Tekt. Show da função
nativa e nome público foram corrigidos separadamente e passam antes deste gate;
markup ainda não é SmartQuote. Constructor vazio tem fields vazio no vanilla;
double:false é campo conhecido, mas double omitido é Unset. enabled:false
permanece SmartQuote no vanilla. set double:false e show filtrado têm positivos
novos. HTML literal funciona porque recebe Text; SmartQuote programática hoje
é rejeitada. Título literal renderiza aspas; script recebe string, rejeitando
Content antes da exportação nos dois binários.

Fontes: markup_stream.rs:79–128 decide glyph em eval; smartquote.rs de layout
alterna dois flags sem texto precedente; stdlib/text/smartquote.rs:112–117 apaga
elemento disabled; observation/elements.rs:242–250 compara os campos atuais.
Vanilla typst-eval/markup.rs:141–146 cria elemento com double explícito;
typst-layout/inline/collect.rs:196–211 resolve estilo/contexto na coleta;
typst-library/text/smartquote.rs:119–158 distingue prime/apóstrofo/nesting.

### Decisão ratificada e limites

Preservar identidade/campos de SmartQuote em eval e resolver glyph só na
composição de render de cada target. Paged usa sua unidade; HTML permanece
semântico e não chama layout paginado para aspas (ADR-0128). A lógica de
seleção contextual é helper puro L1 único, com estado por fluxo/compilação,
sem I/O, globais, vtable, registry de propriedades ou cópia do algoritmo de
memoização upstream. Stack/estado é mecânica escolhida do Tekt; linguagem,
campo conhecido/omitido, glyph e estilo são aceitação. Escaping HTML permanece
com seus owners. Sem novos modos/flags, unsafe, API de watch ou math phase.

Este gate muda double público bool→Option<bool>, adiciona enabled:Option<bool>,
expõe helper/contexto puro e move escolha de glyph eval→render. Confirmação humana recebida em 2026-10-06. O contrato ratificado abaixo
supersede cláusulas históricas conflitantes apenas para aspas; ADRs/núcleos
existentes e propostas alheias permanecem intactos.

RED→GREEN de [it's Ω]/["hi"] preservando SmartQuote e campo double; callback/
alias show da folha, native fields omitido versus false e enabled:false.
Atualizar testes históricos que exigiam glyph em conteúdo avaliado: render é
verificado pelo owner layout; eval verifica tipo/morfologia, campos e Source.
Não eliminar testes de idiomas/aspas; mover a observação ao alvo correto, com
mesmo esperado render. Code string literal permanece Value::Str.

## D05 — teste de show filtrado em equação literal

Medição: after.json.gz registra show sem efeito e expression_dispatch.rs:270–281
não chama a fronteira de captura. Testar alias math.equation.where(block: true)
em literal $ b $ e ramo inline intacto; corpo avaliado uma vez e guard de receita
existente. Não exigir nova fase, render baked ou mudança de representação.

## Inserção matemática automática — proposta sujeita ao gate ADR-0127

### Medição anterior à decisão — D06, 2026-10-06

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado,
UTC/diff/estado/fontes/binários nas coletas canônicas
00_nucleo/diagnosticos/math-bare-2026-10-06/{before,additional,refuters}.json.gz.
Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8; proveniência no relatório
00_nucleo/diagnosticos/math-bare-medicao-2026-10-06.md.

before: frac, mat, sqrt, lr e op nus têm uma Equation no vanilla e zero no Tekt;
math.bold([x]) e texto comum têm zero nos dois. nested e nested-equation têm
uma nos dois; reuse tem duas no vanilla. block-style/local-block/number-style
resolvem o escopo de inserção. show-generated e context-generated expõem as
mesmas lacunas. additional: Equation explícita preserva o render da fração;
it.body é rejeitado no vanilla por maximum show rule depth exceeded, enquanto
it é válido; o show posterior afeta apenas a segunda inserção. refuters mede
valor frac cru e label original versus Equation explícita.

lab/typst-original/crates/typst-realize/src/lib.rs:245–265,302–334 transforma
Mathy antes de show fora de math. 01_core/src/compiler/layout/equation.rs:680–689
projeta math nu em texto; compiler/realize/mod.rs:155–199 prepara show/identidade
sem normalização math. entities/content/traversal.rs:14–108 é bottom-up e
recursa no corpo matemático; rules/apply.rs:564–607 trata apenas localizáveis
já existentes. Estes observáveis são semântica/morfologia/render de linguagem,
não requisito de vtable, buffers, igualdade Rust ou bytes PDF.

### Responsabilidade exclusiva

Testar valores crus frac antes da inserção, frac/mat/root/lr/op automaticamente
consultáveis, texto/math.bold([x]) sem equação, duas inserções iguais distintas,
Equation explícita e conteúdo dentro de math sem wrapping extra. Medir contagens
no índice sem exigir formas internas Rust. Testar where(block), escopo local,
show direto/alias/where, it guardado, geração e erro it.body pelo limite existente,
regra posterior não retroativa e estilo/alt efetivo. Valores esperados vêm das
coletas pinadas. Labels não promovem propriedade à Equation automática.

Testes puros e seus helpers ficam neste consumer; testes de normalizador e
travessia ficam nos owners correspondentes. Fixtures de eval --in não expõem
variáveis do arquivo à expressão auxiliar: não usar unknown variable f como
prova de preservação do valor cru. Usar bloco avaliado diretamente.

### Gate e aceitação

Somente materializar após confirmação humana do pacote math-bare-2026-10-06.md.
RED→GREEN e revalidação bilateral do escopo são obrigatórios. Esta cláusula
supersede restrições de fase anteriores apenas nas fronteiras aqui nomeadas,
sem ratificar paridade universal, D07, math label ou outros débitos.

## D08B3-A — controles de execução do pacote ratificado

Medições anteriores: table-cell-realizacao-d08b3-2026-10-07/before.json.gz,
red.log, nested-reuse-red.log e local-bool-red.log registram estado, fontes
e REDs separados das correções. O manifesto 5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b
foi ratificado no recibo de aprovação. Este consumer cfg(test) suplementa os
43 owners do pacote com controles de campos/ordem, efeitos ocultados, alias
local, célula aninhada, contexto, booleano como Raw e span causal. Os testes
comparam observáveis da linguagem. Não legitimam código produtivo adicional
nem fecham defaults de Raw, célula isolada ou reconstrução recursiva fresca.
O registro de preservação declara explicitamente este consumer de testes e
seu L0 além dos 43 owners; não afirmar que nenhum arquivo externo mudou.

## D08B3-B — candidato, não vigente antes de ratificação

### Medição anterior à decisão

00_nucleo/diagnosticos/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz contém 53 sondas, coletadas em 2026-10-07T14:47:56.340144+00:00 até 14:48:12.375245+00:00. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diff HEAD --stat/status/hashes/fontes/argv/cwd/ambiente e saídas completas registrados. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f. Não misturar before/refuters/complete como rodadas adicionais independentes.

isolated-default: vanilla expõe x/y/inset/align/fill/breakable auto, spans 1 e stroke dict vazio; Tekt omite esses defaults e publica duas metadata contra uma. isolated-x rejeita no Tekt, funciona no vanilla. raw-fields continua só body nos dois e raw-x rejeita nos dois. isolated-table-style confirma que defaults de Table não viram os da Cell avulsa. raw-vs-prepared mede false no vanilla e true no Tekt. nested-effects/body-effects/late-rule refutam execução única e origem lexical. fresh-isolated/table, fresh-block, fresh-styled e multi-fresh são aceitos indevidamente pelo Tekt; vanilla rejeita maximum show rule depth exceeded com span do constructor novo. identity/body/alias e fresh-conditional/hidden continuam válidos. multi-identity e multi-return-table mostram que devolver it deve permitir receita anterior ainda não consumida. isolated-query mantém cell is not locatable nos dois; location é none, sem localização falsa. cell-set é negativo separado, explicitamente excluído deste pacote.

Fonte vanilla lab/typst-original/crates/typst-library/src/model/table.rs:734–778 declara defaults e propriedades; typst-realize/src/lib.rs:367–386 prepara/guarda o argumento, :418–429 visita produto com profundidade, :491–505 distingue guards por elemento, :571–586 materializa propriedades. Tekt entities/elements/table_cell.rs:111–143 só projeta lexical ou plano posicionado; compiler/realize/table_cells.rs:110–129 reaplica Cell crua sem preparo; :280–304 termina snapshot inteiro; eval/show_rule_termination.rs:65–70 trata mesma forma como ponto-fixo; eval/rules/apply.rs:139–166 revoga regra globalmente durante callback. Classificação: campos, efeitos, ordem, terminação e erro causal são língua. A estrutura de guards e materialização upstream é mecânica e não é requisito. Não se infere intenção do upstream; a escolha nominal abaixo é inferência refutável pelos controles de aliases, duas regras, novas gerações e conteúdo ocultado.

### Responsabilidade exclusiva proposta

Testes RED antes do reparo para defaults/getters/presença cru versus preparado; efeito uma vez; body ocultado/nested/reuse/late/local; it/alias e duas regras anteriores; fresh idêntica/crescente/wrappers/condicional/ocultada dentro e fora da Table; limites e spans. Usar helper existente de realização com Engine causal, sem mock que decida a regra no lugar do produto. Aceitação observa língua e ordem real de efeitos.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.

### Auto exposto pela Cell — reparo interno de apresentação, ADR-0127 contínuo

Medição anterior à decisão: table-cell-isolada-d08b3b-2026-10-07/after-second.json.gz, html-fields, em HEAD/estado/UTC/binários registrados: Tekt HTML X auto a como texto; vanilla code(data-lang:typc) e span constant.language #d73948. Upstream foundations/value.rs:191–208 exibe Auto/Bool como Raw inline typc; markup_stream.rs:210–215 só cobre Bool e seu fallback usa Text para Auto; layout/raw.rs:201–203 cobre só átomos true/false. HTML inline.rs não possui ramo Raw e conserva rejeição geral. É morfologia/estilo da linguagem, não byte de PDF; texto igual não prova paridade.

Antes da implementação, testes devem refutar a apresentação Auto como Text e a rejeição HTML de Raw atômico. Corrigir somente o mapeamento Auto para Raw(lang:typc,block:false), a classificação constant.language do átomo auto e a serialização HTML dos átomos typc auto/true/false com code e span constante, delegando escape/mode ao owner existente. Nenhum tipo/assinatura pública, fase, flag/default de produto ou gramática nova. A responsabilidade desta unidade limita-se à operação já owned correspondente: apresentação no markup, cor no Raw paginado, tags na inline HTML, testes no seu owner. Raw de outras linguagens/conteúdo arbitrário em HTML permanece fora da certificação. Confirmar RED→GREEN e igualdade do produto/estilos, com controles Table/Raw/query. Esta cláusula não autoriza mudanças nos outros tipos de Value ou implementação HTML geral de Raw.

## D08B3-C — defaults declarativos da Cell, fluxo contínuo

### Medição anterior à decisão

00_nucleo/diagnosticos/table-cell-styles-d08b3c-2026-10-07/before.json.gz registra 58 sondas, UTC 2026-10-07T15:24:58.038947+00:00 até 15:25:13.727872+00:00, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/status/fontes/argv/cwd/ambiente completos. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3. isolated/table-set, campos individuais, late/local/reuse/spread e rejeições refutam o warning de target vazio e a ausência dos defaults. padding-span-one/empty-fixed-rows fixam spans 1 em vazios; auto-table-fallback preserva Auto explícito como delegação à Table, não ao set da Cell. show-cell-set-self altera avulsa mas não fill conhecido da Cell posicionada. Stroke/alignment fazem fold. refuters-before.json.gz estende com header, partial inset, args/condição e snapshot em Table; alias-inside-table é válido no vanilla e recursa indevidamente no Tekt.

Fonte: upstream typst-library/src/model/table.rs:734–778 declara campos, e layout/grid/resolve.rs:203–278 lê estilos antes de resolver/sobrescrever posições/fill; defaults de padding TableCell packed em model/table.rs:785–800 pinam spans 1. Tekt rules/set.rs:23–27 usa text_str de target e não reconhece FieldAccess; table_cells.rs:262–310 usa apenas propriedades lexicais no placement/herança; grid_placement.rs:80 lê x/y/spans diretamente; TableCell standalone não possui propriedades efetivas privadas. É língua (campos/ordem/efeitos/estilo/erros); a representação privada abaixo é decisão Tekt, não obrigação de copiar Packed ou igualdade Rust. Hipótese refutável por qualquer negativo destas coletas, perda de fonte lexical, mutação de valor cru, efeito repetido, guard de alias perdido ou regressão de geometria/cache.

### Responsabilidade exclusiva

Escrever REDs válidos antes do reparo para setCell defaults/getters cru vs preparado, spans/padding e estilo na ocupação, alias/local/late, constructor/Auto/None/fold, condicional/spread/erro causal, show-set avulsa vs positioned e alias de it em Table. Aceitação por propriedades/efeitos/ordem e testes reais, não por espelhar structs privadas. Revalidar B3-B/Raw/math/query e builds/lint.

### Gate, aceitação e limites

Correção interna de paridade/mapeamento, ADR-0127 contínuo: nenhum contrato público, compatibilidade externa, flag/default de produto ou fase nova. Props e helpers são privados em carrier já não-construível externamente; lexicais não mudam. L0 editado primeiro, resselo restrito e RED→GREEN obrigatórios. Coletas bilaterais com erros/caret/efeitos, fonte/binaries/HEAD/UTC, geometria real; build debug/release/suítes e lint sem erros. Preservar árvore anterior sem commit/reset/resselo global. Subconjunto certificado: inteiros/autos, Color/None/Auto, Stroke uniforme/None, alinhamento, Length/inset parcial de Length/Auto, Bool/Auto e schemas já tipados. Gradient/tiling/functions, strokes por lado, inset percentuais e rich casts de constructor, grupos/spans multipágina/PDF-UA, set/text posicional e defaults gerais de Table continuam incompletos, com medição/L0 próprios antes de ampliar. Não certificar por warning fallback nem copiar defeitos upstream.

Precisão medida em refuters-before: stroke-constructor-fold e align-constructor-fold preservam o construtor inteiro contra set da Cell. self-set-does-not-change-placement muda colspan observado para 2 após a ocupação já feita com 1; não é prova de nova ocupação.

### Precisão dos spans observados após show-set (refutador C)

Medição anterior: refuters-before.json.gz / self-set-does-not-change-placement, vanilla devolve colspan 2 na célula real x0 e colspan 1 nos paddings x1/x2. Fonte pinada layout/grid/resolve.rs:218–244 captura spans de ocupação mas não assenta seus campos no conteúdo; model/table.rs:785–800 assenta 1 nos paddings. Logo ocupação e getter posterior são observáveis distintos. Decisão privada: a Cell posicionada conserva TableCellResolved como plano físico; o carrier standalone privado pode coexistir apenas para propriedades observadas da receita, mantendo campos resolvidos e sobrescrevendo spans lexicalmente omitidos pela declaração de show-set. Getters/canonicalização consultam essa projeção; placement do plano completo continua usando TableCellResolved. Padding sem fonte tem spans lexicais 1 para que show-set não os altere. Guards/stage continuam causais, sem novo contrato público nem mutação da fonte original. RED de span getter antes de implementar, GREEN mais geometria/regressões obrigatórios.

### Fold de declarações de Stroke — medição extra C

extra-first.json.gz (fontes/HEAD/estado/UTC/binários completos) compara two-stroke-sets: set stroke:red seguido de 2pt produz 2pt + red no getter avulso e posicionado vanilla. stroke-render-inherits-cell-style resolve o constructor 2pt com paint red da Cell antes do fallback da Table 3pt, embora stroke-constructor-fold avulso exponha somente o constructor 2pt. Fonte pinada model/table.rs:765 declara #[fold]; resolve.rs:225–237 usa stroke.resolve(styles), distinto do getter cru. Decisão: declarações sucessivas foldam componentes specified ao publicar Value::Stroke declarativo; nenhum carrier/callback é guardado em Styles. Constructor stroke permanece inteiro no getter avulso; resolução posicionada folda o constructor com a base capturada Cell e depois Table. Adicionar somente campo privado stroke_inherited: Option<Stroke> em TableCellProperties para transportar essa base causal, sem assá-la no getter lexical. ObservationEq inclui base; canonicalização ignora base derivada de render. Table já resolvida tem base None. Clone/aliases conservam base anterior; None explícito vence. RED→GREEN antes de código; correção privada contínua ADR-0127, nenhum contrato público/fase/default novo. A cláusula de precedência inteira do constructor vale para o getter avulso, não para a resolução fold de stroke, conforme medição.

### Show-set acumulado e bordas — precisão C

show-fold-before.json.gz e geometry-final.json.gz registram perda de red ao acumular show-set e desenho preto 1pt com campos conhecidos red 2pt. Aplicar helper privado puro chain_styles da Cell na acumulação de deltas cell.stroke; delegar componente de desenho ao owner borders, que lê snapshot sobrevivente do owner cell_body, sem fallback de uma Cell resolvida. Todos os outros estilos/guards/ordem permanecem. Esse helper somente folda Value::Stroke declarativo, nunca carrier/layout/callback; não cria API pública, campo/fase/default novo. RED→GREEN e recoleta real de stroke são obrigatórios.

## D08B3-C2 — testes de origem e forma ratificados

Medição anterior: table-origens-d08b3c2-2026-10-07/baseline.json.gz, HEAD/UTC/working tree/binários/fontes completos. Testar RED antes de implementação: local set/show deve conservar func styled; caret de bare/alias/spread usa origem do conteúdo e preserva trace. Testar forma do body cru, fontes distintas, reuso, metadata uma vez e corpo ocultado; aprovação C2 `confirmo` em 2026-10-07.

### Retificação de critério antigo C2

additional-debug/prior-local-body mede vanilla FUNCstyled contra critério anterior FUNCsequence. Atualizar somente essa expectativa de morfologia e nome do teste; demais critérios permanecem. Helpers de inspeção estruturais dos testes devem atravessar Source sem ignorar estilo/elemento filho (transparência, não supressão de assertions).

## Saneamento do CI — 2026-10-13

Estado medido: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree
não commitado. Fontes, hashes, UTC, status e diffstat estão em
`00_nucleo/diagnosticos/testes-ci-2026-10-13/before.json.gz`. O inventário
Clippy de entrada é `matematica-ci-2026-10-13/clippy-inventory.json`, conservado
no evidence.json.gz daquela rodada. Medir antes de decidir (ADR-0108).

Medição: tests.rs:3989–4022 usa métodos nativos proibidos como expectativas de
quatro inversas trigonométricas. O oráculo pinado, com SHA-256 ratificado,
mede os ângulos em graus no receipt angles.json (evidence.json.gz): 30, 60,
45 e 63.43494882292201, sujeitos à representação float registrada.
Em :6492 o teste descarta o módulo sem asserção; em :6501–6519 a asserção é
condicional ao conteúdo/item Text, podendo passar sem texto nem equação.
As fixtures singleton Range são coleções de spans, não iterações de offsets.

Decisão: substituir expectativas trigonométricas dependentes da plataforma
por constantes independentes em graus, tolerância 1e-12, conservando a
exigência Value::Angle e a ordem pública x,y. Exigir uma Equation presente
no teste de avaliação; no teste de layout exigir texto matemático 𝑥 e rejeitar
colchetes no texto observado, sem depender de FrameItem::Text específico.
Simplificar imports/bindings, Copy e borrows comprovadamente redundantes;
representar singleton Range com from_ref/once sem perder bounds; renomear
somente o módulo privado tests interno para regressions e usar alias privado
para a tupla de matriz. Manter todos os casos, spans, mensagens, fixtures e
assertions; nenhuma nova exceção de lint. A mudança do prefixo privado dos
nomes de testes não remove casos. Validar também os controles negativos de
conteúdo ausente/vazio para os testes reforçados.

Reparo interno de testes, ADR-0127 contínuo. O RED de qualidade é o Clippy
com -D warnings. Reexecutar os casos e validar linhagem; testes sem falha
funcional anterior não recebem alegação artificial de RED→GREEN funcional.

## Associação de labels produzidos por expressão — 2026-10-14

Medição anterior: labels-expressoes-2026-10-14/before/results.json.gz conserva
HEAD/estado/UTC/fontes/binários e pares Tekt/vanilla no pin 9f2b6e871.
Literal funciona; alias/chamada/closure/bloco são exibidos como texto, query
não encontra o alvo e órfão não avisa. Acrescentar REDs no consumer de testes
com World puro para nomes integrais, associação ao heading, espaços preservados,
aviso/range causal de órfão, confinamento no bloco, show label e regra textual
uma única vez, avaliação de expressão uma única vez. Esperados vêm dos
observáveis medidos; não suprimir whitespace/diagnósticos nem exigir árvore
interna idêntica ao vanilla. Os controles gerais de alvos não rotuláveis e
labels repetidos permanecem abertos no passo labels-associacao-alvos.

A primeira execução do candidato expôs espaços iniciais brutos provenientes
das newlines das declarações nas fixtures. fixture-controls.json confirma
fontes equivalentes com ponto e vírgula no oráculo. Usar essas fontes nos
testes de texto exato; não aparar saída nem alterar a semântica de espaços.

## Escopo de associação de labels — 2026-10-14

Medição anterior: labels-escopos-2026-10-14/before/results.json.gz, com
HEAD/estado/UTC/fontes/binários e pin 9f2b6e871. Parbreak bloqueia alvo; set/show
permitem atravessar escopo indevidamente, inclusive set igual/falso; set page
suprime aviso de órfão. Acrescentar REDs para literal/expressão, parbreak em
bloco e interpolado por alias, separadores preservados, set/show/set igual e
falso, page sem alvo, span exato do warning e novo alvo na cauda. Controles
let/code-block não criam fronteira externa. Manter expectativas completas
sem trim; não fechar labels repetidos, Empty explícito ou realização show
por esta mudança de seleção local. Reusar Worlds e helpers existentes.

## Condições e propagação de erros de set — 2026-10-14

Medição antes da decisão: set-condicoes-2026-10-14/{before,extra-before}/results.json.gz
com pin 9f2b6e871, HEAD/estado/UTC/fontes/binários. Condições são ignoradas
fora da Cell; argumentos executam sob false; Cell usa int no erro; heading
engole erro de argumento. Acrescentar testes puros para gate comum antes de
target/args/features, false sem mutações/carriers/warnings, condição uma vez
antes de argumentos, type error com nomes longos/ranges, erro causal, Cell e
alias, show-set e preservação de numeração. Cobrir erro de argumento ativo
em heading e no elemento registrado callout (extensão Tekt, sem falso oráculo
vanilla). Controles true/ausente continuam executando argumentos. Não aparar
mensagens nem trocar expectativas existentes. Perfil HTML desligado testa
curto-circuito/acesso negado, sem crédito de paridade de HTML ativo.


## Regressões CodeBlock com set — 2026-10-14

Medição: codeblock-set-2026-10-14/before/results.json.gz, HEAD/estado/UTC/
fontes/binários completos. Tipos crus, perda de estilos lexicais, join incorreto
com prefixo e warning return ausente foram medidos antes da decisão no owner
00_nucleo/prompts/compiler/eval/expression_dispatch.md. Escrever testes RED
para conversão de cauda (incluindo false/Empty), preservação de prefixo e
span, estilos efetivos locais/aninhados/mesmo valor, avaliação única, retorno,
loop e integração Table/Math; controles sem set conservam tipos. Não mudar
expectativas anteriores para fabricar aprovação. Aceitação não depende da
pilha privada escolhida nem da igualdade de bytes. Fluxo contínuo ADR-0127,
resselo restrito e suíte/sondas de produto; apresentações de outros Value,
show sem seletor e morfologia de SetPage continuam nas etapas explicitadas.

Refutação suplementar: refuters-after/raw-rule-false e raw-rule-active perdem
show raw ao apresentar Bool. Acrescentar RED independente para apresentação
Bool/Auto sob false/true/sets aninhados. O observador de estilos do teste deve
rejeitar variantes inesperadas em vez de silenciosamente ignorá-las.


### Retificação da expectativa de label sobre vazio

first-validation/workspace.log falhou no antigo controle
#heading[Alvo]#{set text(fill:red)}<alvo>, que esperava Alvo. A sonda nova
labels-first/results.json.gz mede o tipo/fields do alvo: o pin publica
styled com child vazio, e a referência é rejeitada como cannot reference
styled. O COUNT=1 do lote labels-escopos não provava que o heading era o alvo.
Também let retorna None e o pin associa sequence vazia, não heading. Portanto
a antiga expectativa deve ser corrigida, não a implementação da cauda para
reproduzir o teste incorreto. Manter novo heading após set e comentário como
controles de alvo anterior; testar separadamente o alvo vazio de CodeBlock.
O controle let com alvo vazio/query não é declarado resolvido: a associação
de None/Empty e publicação introspectiva de labels arbitrários ainda requer
os owners respectivos. Preservar os logs e a retificação no relatório;
não reduzir uma divergência de alvo a mera igualdade da contagem.

## Presença lexical Raw — ratificada em 2026-10-14

### Medição anterior à decisão

content-serialization-2026-10-14/{before,raw-controls}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes/binários e pin 9f2b6e8715237cb086899a42873660fe744622e8. RawElem raw.rs:21–25 perde omissão; native_raw markup.rs:78–136 colapsa ausência/false/none. Campos, igualdade e encoders distinguem esses valores no pin. Trata-se de semântica/morfologia, não mecânica Rust.

### Responsabilidade e aceitação

Testes RED→GREEN: campos omitidos/false/true, lang omitido/none/string, aliases, igualdade, at/has/fields, JSON/YAML e markup/apresentação; sem aparar saída.

Manifesto aprovado b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45, recibo em materialization/raw-presenca-2026-10-14/approval-receipt.json. Esta seção completa apenas a presença anteriormente incompleta de Raw e supersede as cláusulas de ausência de máscara. Preservar contratos alheios; sem fase/default/render novo. Validar testes RED→GREEN, suíte, build e linhagem.


## Contrato Default de EvalContext — pacote clippy-default-2026-10-16

Medição antes da decisão: compiler/eval/mod.rs:397–424 inicializa estados e limites explicitamente; o inventário registra ausência de Default. Fonte/owner atuais estão no snapshot deste pacote. A proposta pública está exclusivamente em `00_nucleo/prompts/compiler/eval.md`.

Após a confirmação do manifesto, acrescentar neste consumer os controles de uso por bound Default e isolamento entre instâncias: Paged, features vazias, full_error false, recipes habilitadas e orçamento vigente, contadores/locação/fluxo independentes. Um contexto mutado não contamina o próximo Default nem um contexto new. Confirmar RED pela ausência do impl e GREEN após delegação, sem relaxar nenhum teste de linguagem/diagnóstico existente. Não alegar nova paridade upstream nem mudar fixtures existentes. O pacote não autoriza código antes da confirmação.

Ratificação: dono aprovou o manifesto SHA-256 `11f3e138f5ba006d04ed4dd1e829e27c004778e691f4a1df63127b1644146d03` em 2026-10-16. Recibo: `00_nucleo/materialization/clippy-default-2026-10-16/approval-receipt.json`. O gate acima está satisfeito exclusivamente para os três impls Default e testes D1–D5 descritos; esta seção está vigente.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F001: 01_core/src/compiler/eval/tests.rs:7015 (has_line). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `00_nucleo/diagnosticos/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F001 — `has_line`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Text`, `TextShaped`, `Glyph`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.


## Helpers de projeção e estilos — V16, 2026-10-17

### Medição anterior à decisão

`01_core/src/compiler/eval/tests.rs:129,494,538,566,571,632,645,771,941,963,1098`
contém onze braços abertos auditados por AST, sem guards nesses matches.
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
UTC, diff HEAD --stat exato, fontes, configuração, hashes e parser pinado
estão em `00_nucleo/diagnosticos/tekt-v16-eval-helpers-2026-10-17/before.json.gz`,
`contexts.json` e `audit-recipe.json`. A enumeração atual mede 120 variantes
Content e 39 Value; o snapshot antes do código preserva as definições por hash.
A suíte anterior passou no mesmo estado executável, confirmado pelos hashes.
São helpers cfg(test), não decisões novas da linguagem. Não se infere intenção
upstream nem benefício de leitura por IA a partir de uma nota do linter.

### Prescrição e justificativa de V19 por decisão

Trocar exclusivamente o padrão `_` de cada braço abaixo pelo complemento
nominal **atual** do enum indicado em relação às variantes atendidas nos demais
braços. Usar o prefixo em escopo, payload `_`/`{ .. }` adequado e grupo plano,
sem capturar binding novo. O complemento integral e seus spans estão em
scope.json; enumerações futuras exigem nova decisão pelo compilador.

- **E01**, linha 129, `eval_for_test_with_registry`, Content: excluir do complemento `Located, Realized, ShowRecipe`. Projeção visual legada remove apenas Located/Realized/ShowRecipe; demais nós não são substituídos pelo adapter. O grupo V19 compartilha somente o corpo anterior deste braço; não exige despacho distinto por variante sem observável distinto.

- **E02**, linha 494, `go`, Content: excluir do complemento `Heading, Sequence, Styled`. Busca da primeira heading com numbering herdado; demais variantes não dão resultado nem abrem nova recursão. O grupo V19 compartilha somente o corpo anterior deste braço; não exige despacho distinto por variante sem observável distinto.

- **E03**, linha 538, `go`, Content: excluir do complemento `Heading, Sequence, Styled`. Coleta de headings na ordem atual; folhas excluídas não acrescentam itens. O grupo V19 compartilha somente o corpo anterior deste braço; não exige despacho distinto por variante sem observável distinto.

- **E04**, linha 566, `go`, Value: excluir do complemento `Str, None`. Valor diferente de Str/None conserva o padrão de numeração herdado, sem validá-lo como novo padrão. O grupo V19 compartilha somente o corpo anterior deste braço; não exige despacho distinto por variante sem observável distinto.

- **E05**, linha 571, `go`, Content: excluir do complemento `Equation, Sequence, Styled`. Busca de Equation com numbering herdado; os demais nós não produzem resultado. O grupo V19 compartilha somente o corpo anterior deste braço; não exige despacho distinto por variante sem observável distinto.

- **E06**, linha 632, `find_matrix_or_vec_delim`, Content: excluir do complemento `MathMatrix, MathVec, Sequence, MathSequence, Styled, Equation`. Busca delimitadores de Matrix/Vec na travessia atual; demais variantes não fornecem delimitador. O grupo V19 compartilha somente o corpo anterior deste braço; não exige despacho distinto por variante sem observável distinto.

- **E07**, linha 645, `find_op_limits`, Content: excluir do complemento `MathOp, Sequence, MathSequence, Styled, Equation`. Busca limits do MathOp na travessia atual; demais variantes não fornecem limits. O grupo V19 compartilha somente o corpo anterior deste braço; não exige despacho distinto por variante sem observável distinto.

- **E08**, linha 771, `go`, Content: excluir do complemento `Figure, Sequence, Styled, Label`. Coleta numbering de Figure com estilo e Label atuais; demais nós não acrescentam itens. O grupo V19 compartilha somente o corpo anterior deste braço; não exige despacho distinto por variante sem observável distinto.

- **E09**, linha 941, `has_dynamic`, Content: excluir do complemento `Dynamic, Sequence, Styled`. Detector de Dynamic nas travessias atuais; as demais variantes não satisfazem esse detector. O grupo V19 compartilha somente o corpo anterior deste braço; não exige despacho distinto por variante sem observável distinto.

- **E10**, linha 963, `find_styled_custom`, Content: excluir do complemento `Styled, Sequence`. Busca custom em Styled/Sequence com preferência e curto-circuito atuais; demais nós não fornecem propriedade. O grupo V19 compartilha somente o corpo anterior deste braço; não exige despacho distinto por variante sem observável distinto.

- **E11**, linha 1098, `find_custom_in_styled`, Content: excluir do complemento `Styled, Sequence`. Busca custom em Styled/Sequence, preservando primeira chave e ordem; demais nós não fornecem propriedade. O grupo V19 compartilha somente o corpo anterior deste braço; não exige despacho distinto por variante sem observável distinto.


Conservar byte a byte os demais tokens Rust após normalização por rustfmt:
corpos, guards, pattern payloads anteriores, chamadas, early returns, ordem,
short-circuit, estilos herdados e todas as fixtures/asserções existentes.
Não normalizar Source nem ampliar travessia neste lote; os helpers têm escopo
específico e a projeção semântica preservada continua separada do adapter visual.
Não converter match em if-let/matches!, não adicionar allow/expect, não mudar
limiares ou grupos preexistentes para calar V19. Esta nota é informativa e deve
continuar visível. Nenhum teste ou resultado ausente passa a ser aceito.

### Aceitação e limites

Manutenção interna de testes, ADR-0127 contínuo: sem contrato público, entidade,
default, fase, dependência ou compatibilidade nova. RED estrutural: onze V16
no parser pinado e sentinelas Content/Value sem E0004 nesses onze matches;
GREEN: V16 removidos e E0004 nos onze após inserir uma variante sentinela em
cada enum numa cópia descartável do workspace com cfg(test) compilado.
Não alegar RED funcional nem nova paridade: o comportamento é preservado.
Conferir replay dos padrões com corpos imutáveis, enum/assinaturas intactos,
SARIF diferencial completo (onze V16 a menos, onze V19 de complementos a mais,
V20 inalterado), suíte workspace com os mesmos esperados, build, fmt, Clippy
workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc
-D warnings e quatro índices HTML, V1/V5/V15/V26 estrito e linhagem reversa.
Resselo somente deste par. Medições com fontes congeladas, proveniência completa,
MemoryMax=12 GiB, MemorySwapMax=0, Cargo um job e comandos pesados sequenciais.
Os outros 52 V16 neste consumer e Source(Table) permanecem fora da materialização
desta seção; investigar cada um antes de prescrever correção. Não publicar release.


## Helpers matemáticos e estruturais — exaustividade V16, 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `00_nucleo/diagnosticos/tekt-v16-eval-math-2026-10-17/before.json.gz`. AST do parser pinado em contexts.json/all-contexts.json/wildcard-inventory.json; definições atuais por hash. A suíte anterior passou no mesmo estado executável, revalidado por hashes antes de decidir. Sem inferir intenção upstream ou benefício cognitivo da IA a partir do lint.

O inventário mede 52 V16 emitidos e 18 fallbacks nominais não emitidos (sete Content e onze MathAttachSlot). Guards em leaf/MathFrac/Quote e o payload Present(MathPrimes) são refinamentos: não representam cobertura incondicional dessas variantes. A propriedade pretendida é revisão diante de enum novo, sem alterar linguagem ou asserção.

### Prescrição exclusiva e V19 justificado por caso

Para cada caso abaixo, trocar somente o padrão `_` pelo complemento nominal **atual** das variantes cobertas incondicionalmente pelos braços anteriores. Content tem 120 variantes; MathAttachSlot tem Omitted/ExplicitNone/Present(Content). Não remover do complemento variantes atendidas somente sob guard. Manter guards, corpos, ordem de busca/slots, any/find_map, callbacks, retornos, erros, fixtures e esperados integralmente. Não trocar por if-let/matches!, não adicionar allow/expect, não alterar limiar. Cada grupo V19 continua informativo; não é prova de executar todas as alternativas.

- **M01**, `01_core/src/compiler/eval/tests.rs:1680`, `go`, Content, latente: excluir somente `Table, Sequence, Styled, Label`; manter sob fallback `(sem guard)`. Coleta de propriedade na travessia já especificada; demais variantes não acrescentam itens e a herança/ordem permanecem.

- **M02**, `01_core/src/compiler/eval/tests.rs:8128`, `has_math_root`, Content, emitido: excluir somente `MathRoot, Equation, MathSequence, Sequence`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M03**, `01_core/src/compiler/eval/tests.rs:8146`, `has_math_root_with_index`, Content, emitido: excluir somente `MathRoot, Equation, MathSequence, Sequence`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M04**, `01_core/src/compiler/eval/tests.rs:8405`, `eval_label_anexa_ao_bloco_anterior`, Content, latente: excluir somente `Label, Sequence`; manter sob fallback `(sem guard)`. Variantes que não satisfazem a asserção falham com o panic anterior, incluindo Quote sem attribution quando o guard falha; nunca aceitar ausência.

- **M05**, `01_core/src/compiler/eval/tests.rs:9348`, `tem_styled_sobre_label`, Content, emitido: excluir somente `Styled, Sequence`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M06**, `01_core/src/compiler/eval/tests.rs:10294`, `tem_context_block`, Content, emitido: excluir somente `ContextBlock, Sequence, Styled`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M07**, `01_core/src/compiler/eval/tests.rs:10312`, `tem_heading`, Content, emitido: excluir somente `Heading, Sequence, Styled`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M08**, `01_core/src/compiler/eval/tests.rs:10542`, `encontra_equation`, Content, emitido: excluir somente `Equation, Sequence, Styled`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M09**, `01_core/src/compiler/eval/tests.rs:10568`, `go`, Content, emitido: excluir somente `Strong, Sequence, Styled`; manter sob fallback `(sem guard)`. Coleta de propriedade na travessia já especificada; demais variantes não acrescentam itens e a herança/ordem permanecem.

- **M10**, `01_core/src/compiler/eval/tests.rs:10587`, `go`, Content, emitido: excluir somente `Emph, Sequence, Styled`; manter sob fallback `(sem guard)`. Coleta de propriedade na travessia já especificada; demais variantes não acrescentam itens e a herança/ordem permanecem.

- **M11**, `01_core/src/compiler/eval/tests.rs:10621`, `styled_bold_envolve_heading`, Content, emitido: excluir somente `Styled, Sequence`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M12**, `01_core/src/compiler/eval/tests.rs:10630`, `contem_heading`, Content, emitido: excluir somente `Heading, Styled, Sequence`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M13**, `01_core/src/compiler/eval/tests.rs:10653`, `go`, Content, emitido: excluir somente `Text, Styled, Sequence, Heading`; manter sob fallback `(sem guard)`. Coleta de propriedade na travessia já especificada; demais variantes não acrescentam itens e a herança/ordem permanecem.

- **M14**, `01_core/src/compiler/eval/tests.rs:10736`, `styled_bold_anywhere`, Content, emitido: excluir somente `Styled, Sequence, Heading`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M15**, `01_core/src/compiler/eval/tests.rs:10783`, `styled_bold_e_weight`, Content, emitido: excluir somente `Styled, Sequence`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M16**, `01_core/src/compiler/eval/tests.rs:10898`, `eval_terms_construtor_typst_lang`, Content, latente: excluir somente `Terms`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M17**, `01_core/src/compiler/eval/tests.rs:10900`, `eval_terms_construtor_typst_lang`, Content, emitido: excluir somente `Terms, Sequence`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M18**, `01_core/src/compiler/eval/tests.rs:10932`, `find_quote`, Content, emitido: excluir somente `Quote, Sequence`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M19**, `01_core/src/compiler/eval/tests.rs:10949`, `eval_quote_construtor_typst_lang`, Content, latente: excluir somente `Quote`; manter sob fallback `(sem guard)`. Variantes que não satisfazem a asserção falham com o panic anterior, incluindo Quote sem attribution quando o guard falha; nunca aceitar ausência.

- **M20**, `01_core/src/compiler/eval/tests.rs:10965`, `eval_quote_com_attribution_typst_lang`, Content, latente: excluir somente `(nenhuma variante inteira)`; manter sob fallback `Quote`. Variantes que não satisfazem a asserção falham com o panic anterior, incluindo Quote sem attribution quando o guard falha; nunca aceitar ausência. As variantes do guard permanecem no complemento para conservar a falha da condição.

- **M21**, `01_core/src/compiler/eval/tests.rs:10978`, `eval_quote_block_true_typst_lang`, Content, latente: excluir somente `Quote`; manter sob fallback `(sem guard)`. Variantes que não satisfazem a asserção falham com o panic anterior, incluindo Quote sem attribution quando o guard falha; nunca aceitar ausência.

- **M22**, `01_core/src/compiler/eval/tests.rs:10991`, `eval_quote_quotes_false_typst_lang`, Content, latente: excluir somente `Quote`; manter sob fallback `(sem guard)`. Variantes que não satisfazem a asserção falham com o panic anterior, incluindo Quote sem attribution quando o guard falha; nunca aceitar ausência.

- **M23**, `01_core/src/compiler/eval/tests.rs:11282`, `find_mathop_in`, MathAttachSlot, latente: excluir somente `Present`; manter sob fallback `(sem guard)`. Omitted e ExplicitNone não são conteúdo a visitar; conservar None e a ordem br/tr/b/t/bl/tl existente.

- **M24**, `01_core/src/compiler/eval/tests.rs:11286`, `find_mathop_in`, MathAttachSlot, latente: excluir somente `Present`; manter sob fallback `(sem guard)`. Omitted e ExplicitNone não são conteúdo a visitar; conservar None e a ordem br/tr/b/t/bl/tl existente.

- **M25**, `01_core/src/compiler/eval/tests.rs:11290`, `find_mathop_in`, MathAttachSlot, latente: excluir somente `Present`; manter sob fallback `(sem guard)`. Omitted e ExplicitNone não são conteúdo a visitar; conservar None e a ordem br/tr/b/t/bl/tl existente.

- **M26**, `01_core/src/compiler/eval/tests.rs:11294`, `find_mathop_in`, MathAttachSlot, latente: excluir somente `Present`; manter sob fallback `(sem guard)`. Omitted e ExplicitNone não são conteúdo a visitar; conservar None e a ordem br/tr/b/t/bl/tl existente.

- **M27**, `01_core/src/compiler/eval/tests.rs:11298`, `find_mathop_in`, MathAttachSlot, latente: excluir somente `Present`; manter sob fallback `(sem guard)`. Omitted e ExplicitNone não são conteúdo a visitar; conservar None e a ordem br/tr/b/t/bl/tl existente.

- **M28**, `01_core/src/compiler/eval/tests.rs:11302`, `find_mathop_in`, MathAttachSlot, latente: excluir somente `Present`; manter sob fallback `(sem guard)`. Omitted e ExplicitNone não são conteúdo a visitar; conservar None e a ordem br/tr/b/t/bl/tl existente.

- **M29**, `01_core/src/compiler/eval/tests.rs:11304`, `find_mathop_in`, Content, emitido: excluir somente `MathOp, Sequence, MathSequence, Equation, MathAttach`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M30**, `01_core/src/compiler/eval/tests.rs:11320`, `find_mathident_in`, MathAttachSlot, latente: excluir somente `Present`; manter sob fallback `(sem guard)`. Omitted e ExplicitNone não são conteúdo a visitar; conservar None e a ordem br/tr/b/t/bl/tl existente.

- **M31**, `01_core/src/compiler/eval/tests.rs:11324`, `find_mathident_in`, MathAttachSlot, latente: excluir somente `Present`; manter sob fallback `(sem guard)`. Omitted e ExplicitNone não são conteúdo a visitar; conservar None e a ordem br/tr/b/t/bl/tl existente.

- **M32**, `01_core/src/compiler/eval/tests.rs:11328`, `find_mathident_in`, MathAttachSlot, latente: excluir somente `Present`; manter sob fallback `(sem guard)`. Omitted e ExplicitNone não são conteúdo a visitar; conservar None e a ordem br/tr/b/t/bl/tl existente.

- **M33**, `01_core/src/compiler/eval/tests.rs:11332`, `find_mathident_in`, MathAttachSlot, latente: excluir somente `Present`; manter sob fallback `(sem guard)`. Omitted e ExplicitNone não são conteúdo a visitar; conservar None e a ordem br/tr/b/t/bl/tl existente.

- **M34**, `01_core/src/compiler/eval/tests.rs:11334`, `find_mathident_in`, Content, emitido: excluir somente `MathIdent, MathText, Sequence, MathSequence, Equation, MathAttach`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M35**, `01_core/src/compiler/eval/tests.rs:11345`, `find_mathstyled_body_in`, Content, emitido: excluir somente `MathStyled, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M36**, `01_core/src/compiler/eval/tests.rs:11435`, `leaf`, Content, emitido: excluir somente `Sequence, MathSequence, Equation`; manter sob fallback `MathIdent, MathText`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito. As variantes do guard permanecem no complemento para conservar a falha da condição.

- **M37**, `01_core/src/compiler/eval/tests.rs:11527`, `find_mathdelimited_in`, Content, emitido: excluir somente `MathDelimited, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M38**, `01_core/src/compiler/eval/tests.rs:11578`, `find_mathlr`, Content, emitido: excluir somente `MathLr, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M39**, `01_core/src/compiler/eval/tests.rs:11608`, `find_mathlr`, Content, emitido: excluir somente `MathLr, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M40**, `01_core/src/compiler/eval/tests.rs:11638`, `find_mathlr`, Content, emitido: excluir somente `MathLr, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M41**, `01_core/src/compiler/eval/tests.rs:11667`, `find_overbrace`, Content, emitido: excluir somente `MathOverbrace, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M42**, `01_core/src/compiler/eval/tests.rs:11695`, `find_underbrace`, Content, emitido: excluir somente `MathUnderbrace, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M43**, `01_core/src/compiler/eval/tests.rs:11742`, `find_underbracket`, Content, emitido: excluir somente `MathUnderbracket, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M44**, `01_core/src/compiler/eval/tests.rs:11789`, `find_underparen`, Content, emitido: excluir somente `MathUnderparen, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M45**, `01_core/src/compiler/eval/tests.rs:11836`, `find_undershell`, Content, emitido: excluir somente `MathUndershell, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M46**, `01_core/src/compiler/eval/tests.rs:11883`, `find_overbracket`, Content, emitido: excluir somente `MathOverbracket, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M47**, `01_core/src/compiler/eval/tests.rs:11911`, `find_overline`, Content, emitido: excluir somente `MathOverline, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M48**, `01_core/src/compiler/eval/tests.rs:11940`, `find_overparen`, Content, emitido: excluir somente `MathOverparen, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M49**, `01_core/src/compiler/eval/tests.rs:11968`, `find_overshell`, Content, emitido: excluir somente `MathOvershell, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M50**, `01_core/src/compiler/eval/tests.rs:12007`, `prime_count`, MathAttachSlot, latente: excluir somente `(nenhuma variante inteira)`; manter sob fallback `(sem guard)`. Omitted/ExplicitNone e Present de Content diferente de MathPrimes não fornecem contagem; o payload nominal mantém revisão futura dos dois enums.

- **M51**, `01_core/src/compiler/eval/tests.rs:12044`, `find_mathmatrix_in`, Content, emitido: excluir somente `MathMatrix, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M52**, `01_core/src/compiler/eval/tests.rs:12056`, `find_unlined_frac_in`, Content, emitido: excluir somente `MathDelimited, Sequence, MathSequence, Equation`; manter sob fallback `MathFrac`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito. As variantes do guard permanecem no complemento para conservar a falha da condição.

- **M53**, `01_core/src/compiler/eval/tests.rs:12111`, `find_mathaccent_in`, Content, emitido: excluir somente `MathAccent, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M54**, `01_core/src/compiler/eval/tests.rs:12343`, `p958_mathtexts`, Content, emitido: excluir somente `MathText, MathIdent, Sequence, MathSequence, Equation, MathDelimited, MathAttach`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M55**, `01_core/src/compiler/eval/tests.rs:12450`, `p962_find_styled`, Content, emitido: excluir somente `MathStyled, MathClassOverride, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M56**, `01_core/src/compiler/eval/tests.rs:12491`, `find`, Content, emitido: excluir somente `Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M57**, `01_core/src/compiler/eval/tests.rs:18768`, `find_mathunderover_in`, Content, emitido: excluir somente `MathUnderover, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M58**, `01_core/src/compiler/eval/tests.rs:18782`, `find_overbrace_in`, Content, emitido: excluir somente `MathOverbrace, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M59**, `01_core/src/compiler/eval/tests.rs:18796`, `find_underbrace_in`, Content, emitido: excluir somente `MathUnderbrace, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M60**, `01_core/src/compiler/eval/tests.rs:18810`, `find_underbracket_in`, Content, emitido: excluir somente `MathUnderbracket, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M61**, `01_core/src/compiler/eval/tests.rs:18824`, `find_overbracket_in`, Content, emitido: excluir somente `MathOverbracket, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M62**, `01_core/src/compiler/eval/tests.rs:19031`, `find_mathlr_body`, Content, emitido: excluir somente `MathLr, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M63**, `01_core/src/compiler/eval/tests.rs:19048`, `first_boundary`, Content, emitido: excluir somente `MathDelimited, Sequence, MathSequence, Text, MathText, MathIdent`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M64**, `01_core/src/compiler/eval/tests.rs:19065`, `last_boundary`, Content, emitido: excluir somente `MathDelimited, Sequence, MathSequence, Text, MathText, MathIdent`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M65**, `01_core/src/compiler/eval/tests.rs:19155`, `find_mathlimitsoverride_in`, Content, emitido: excluir somente `MathLimitsOverride, Sequence, MathSequence, Equation, MathAttach`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M66**, `01_core/src/compiler/eval/tests.rs:19291`, `find_mathdelimited_in`, Content, emitido: excluir somente `MathDelimited, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M67**, `01_core/src/compiler/eval/tests.rs:19302`, `has_linebreak`, Content, emitido: excluir somente `Linebreak, Sequence, MathSequence, Equation`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M68**, `01_core/src/compiler/eval/tests.rs:19418`, `p1043_math_callee_dot_double_isolada`, Content, emitido: excluir somente `MathAccent, MathSequence`; manter sob fallback `(sem guard)`. Busca ou predicado específico deste helper; variantes sem resultado partilham o fallback anterior, sem ampliar travessia, sinalizar falso sucesso ou mudar curto-circuito.

- **M69**, `01_core/src/compiler/eval/tests.rs:24262`, `break_policy_custom_values`, Content, emitido: excluir somente `Styled, Sequence, Par`; manter sob fallback `(sem guard)`. Coleta de propriedade na travessia já especificada; demais variantes não acrescentam itens e a herança/ordem permanecem.

- **M70**, `01_core/src/compiler/eval/tests.rs:24320`, `count`, Content, emitido: excluir somente `Par, Styled, Strong, Emph, Sequence`; manter sob fallback `(sem guard)`. Somente Par soma uma ocorrência e os wrappers explicitados recursam; as demais variantes contribuem zero neste contador de teste.


O caso prime_count substitui o catch-all por Omitted | ExplicitNone | Present
com complemento nominal de Content menos MathPrimes dentro do payload. Não
usar Present(_) como novo catch-all interno: a sonda deve exigir revisão diante
de Content novo e de MathAttachSlot novo. Os demais slots Present(Content)
continuam delegando a seus helpers. As asserções Quote/Label continuam falhando
com o mesmo texto; não converter qualquer erro em None ou falso sucesso.

### Aceitação e escopo

Correção interna de testes ADR-0127 contínuo, sem API pública, fase, default,
compatibilidade ou dependência nova. RED estrutural dos 52 avisos e sonda em
workspace descartável compilando lib cfg(test): zero E0004 nos 70 locais antes;
após, E0004 em todos com sentinelas Content/MathAttachSlot. Testar também
controles de guard recusado (leaf e fração lined) e asserções Quote existentes,
sem alterar seus esperados. Replay dos spans conserva tudo exceto padrões,
rustfmt, metadados e selos. SARIF diferencial completo: 52 V16 removidos,
notas V19 dos grupos prescritos, V20 e demais diagnósticos preservados após
mapear linhas. Não exigir remoção de V16 alheios para declarar este lote fechado.
Suíte workspace completa, build, fmt, Clippy all-targets -D warnings e
-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices HTML,
V1/V5/V15/V26 estrito, reversa e diff. Limite 12 GiB, swap zero, um job Cargo,
execução pesada sequencial, fontes congeladas e proveniência completa.


### Refinamento medido de prime_count — V20, 2026-10-17

candidate-state.json.gz/candidate-architecture.log do lote
tekt-v16-eval-math-2026-10-17 preservam o candidato, estado/UTC/hashes e a medição:
170 V16, 988 V19 e 872 V20. O complemento aninhado de Present(Content alternativo)
acrescentou um V20 à profundidade já existente do braço Present(MathPrimes).
A implementação direta foi, portanto, refutada pelo critério de não acrescentar
profundidade; isso não é falha funcional ou motivo para silenciar V20.

Superseder somente a forma do caso prime_count: manter assinatura e substituir
o match aninhado por dois matches rasos. O externo distingue Present(prime_content)
de Omitted | ExplicitNone; o interno retorna Some(elem.count) para MathPrimes e
None para o complemento nominal das outras 119 variantes atuais de Content.
Sem guard, captura @, clone, if-let, helper/API novo ou alteração dos demais
69 locais, fixtures/asserções e ordem. Esses dois complementos V19 são legítimos
porque compartilham somente o mesmo None em cada domínio. O conteúdo não é
validado duas vezes nem percorrido: só se lê count quando há MathPrimes.

A sonda mantém os 70 locais originais e acrescenta o match interno auxiliar:
variante MathAttachSlot nova exige revisão do externo e Content nova exige
revisão do interno. O resultado esperado é 71 E0004 selecionados no GREEN,
zero no RED inicial, sem outros erros. Comparar árvores por ordinal, descontando
somente o novo scrutinee prime_content. SARIF esperado do lote final: 52 V16
removidos, 71 V19 prescritos novos, um V20 antigo de prime_count removido e
nenhum V20 novo. Todos os outros diagnósticos preservados. Replay autoriza
somente esta reestruturação local adicional, conforme receita literal no recibo
de refinamento. Os gates de aceitação e limite de 12 GiB anteriores permanecem.


## TS1 — tamanho completo de texto (ratificado pelo humano, 2026-10-17)

### Medição anterior à decisão

`00_nucleo/diagnosticos/text-argumentos-2026-10-17/before.json.gz`, `before-measurement.json.gz`,
`morph-before-measurement.json.gz` e `audit.json` conservam HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status/diffstat,
fontes/L0/binários/argv e o pin9f2b6e8715237cb086899a42873660fe744622e8.
Em named-em, A mede0pt contra24pt; em affine,3pt contra27pt; nested sobre18pt
mede0pt contra36pt. As seis permutações Size/Color/body são aceitas pelo pin
mas rejeitadas pelo Tekt. Set posicional18pt é ignorado (12 contra18pt).
Fonte pinada text/mod.rs:290,303,924–930,1136–1142 e args.rs:178–247
estabelece busca tipada, preferência nomeada, body restante e composição.
style_chain.rs:513–517 e constructor.rs:123 descartam em; collapse em
style_chain.rs:394–395,419–422 aplica top-wins onde tamanho precisa fold.
É tamanho/escopo observável da linguagem; a representação Rust é escolha
interna. Não inferir intenção histórica. Refutar por restauração, Raw,
heading, math, capture/show, dupes/spans, zero/negativos ou métricas divergentes.
A sonda de igualdade refutou a hipótese de comparar os valores declarados:
no pin, text(size:2em)[A] == text(size:24pt)[A] é true. StyledElem::eq no pin
content/mod.rs:779–782 compara child. Esse defeito de igualdade requer sua
própria correção; o tipo completo não é prova de fechamento desse eixo.

### Ratificação

O humano aprovou TS1 e os dois contratos públicos da proposta de manifesto
771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba.
Implementação e testes S1–S16 sob 12 GiB/swap zero; sem release/merge/push.
Resselo somente após implementação e grafo válido. RG1 continua independente.

### Responsabilidade exclusiva adicional

Somente fixtures/adapters de testes: substituir Style::Size(Pt) por Size(Length absoluto), e expectativas de StyleDelta.size por Option<Length>. Conservar os observáveis linguísticos e os contratos funcionais existentes; não trocar geometria correta por snapshots da representação nova.


## Import/include — migração medida de expectativa P1306 (2026-10-17)

### Medição anterior à decisão

00_nucleo/diagnosticos/import-syntax-2026-10-17/legacy-expected-attempt/workspace/workspace.log
registra a falha isolada de p1306_named_module_contract_all_profiles em HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado e fontes
congeladas no validation-result.json daquele diretório. O teste exigia trace
vazio para imported-error, embora o erro surja em routes/error.typ. A fixture
exata está em legacy-module-probe/: import do arquivo, inner.absent, mensagem
module `std` does not contain `absent`, origem 59..65. legacy-module-probe.json
registra quatro perfis, UTC, hashes de fontes/binários/fixtures e stdout/stderr.
No pin 9f2b6e8715237cb086899a42873660fe744622e8 e no Tekt corrigido, o erro tem
Tracepoint::Import("routes/error.typ") na expressão importadora, range 2..36,
com mensagem/hints/origem iguais. Descontou-se apenas o literal de aviso de
arquivo vazio já conhecido; stderr restante idêntico. A regra é diagnóstico
observável da linguagem, não equivalência mecânica de Rust. O pressuposto de
trace vazio foi refutado pela sonda; não inferir intenção do teste antigo.

### Responsabilidade e aceitação

Migrar somente a expectativa de trace em p1306_named_module_contract_all_profiles.
Construir a Source da expressão uma vez e usá-la para conferir a origem do
trace. imported-error exige exatamente um Tracepoint::Import com caminho
routes/error.typ e range 2..36 da expressão. Todos os demais casos continuam
exigindo trace vazio. Conservar contagem unitária de erro, Severity::Error,
mensagem, hints vazios, origem/range do erro, side vazio, quatro perfis, ordem
normal/inversa e controles existentes. Não retirar assertivas para acomodar a
mudança. Sem novo teste público, supressão, lógica produtiva ou contrato Rust.
A política compartilhada vem do núcleo pinado source-validation e do owner
00_nucleo/prompts/compiler/eval/modules.md. Conferir teste legado focal,
regressão completa, Clippy/Rustdoc estritos, linhagem e SARIF diferencial no
novo source freeze sob MemoryMax12GiB/High10GiB/swap0/jobs1.

## A1 — reparo de paridade medido em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c; estado/UTC/fontes em
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz.
Vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3. before-pdf-probe.json.gz
no mesmo diagnóstico congela controles bilaterais de campo, admissão, callback
e init. A auditoria anterior paridade-novo-vanilla-2026-10-17/inventario.md
E02–E04 registra os limites de carrier e as fontes discriminantes; não se usa
dump/estrutura Rust como critério. Estes observáveis são linguagem.

### Responsabilidade exclusiva e aceitação

Adicionar regressões A1 de linguagem no módulo de testes existente: getters
text/h/v/box/block e presença de width/weak; bare text/bibliography/selector;
counter(math.equation) e alias; negativos de closure/partial/não-locatabilidade;
state init opcional. Esperados congelados do vanilla, não espelhados do código.
State funcional requer revisão corrigida: A1-S1 foi retirada por contrariar
init por objeto leitor. Heading tem seus próprios controles causais complementares.

L0 primeiro, RED funcional e GREEN bilateral. Não alterar contrato público,
default de produto, fase ou compatibilidade; a revisão corrigida de State é independente. Preservar
1:1 e núcleos pinados; resselo após grafo válido. Build/workspace/Clippy estrito,
Rustdoc/fmt/linhagem estrita e reversa, jobs sequenciais sob MemoryMax12GiB,
High8GiB/swap0/jobs1. Debug0/incremental0 somente no ambiente de validação para
reduzir picos; comparar consumo medido sem inferir ganho do produto.


## A1-S2 — leitura por objeto aprovada em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
UTC/status/diffstat/fontes em before.json.gz e final-freeze.json.gz de
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17.
state-counterproof.json.gz registra RED bilateral: init 1/10 da mesma chave
após +1 exige 2/11; callback não lido não executa; atualização reutilizada
executa por ocorrência. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3,
upstream introspection/state.rs:226–245, 419–455 e 477–520. O init do leitor
inicia a sequência; a ordem vem do documento, não do inteiro de Location.
O método atual trata Func como Set e o pós-passe eager perde essa semântica.
Estes são valores/erros de linguagem, não igualdade Rust ou algoritmo exigido.
A1-S1 foi retirada; gate A1-S2 confirmado pelo dono nesta sessão, revisão
00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md,
SHA-256 6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4.

### Responsabilidade exclusiva — supersede a fase histórica de State

Acrescentar controles de linguagem para update funcional direto e método guardado
em variável, com conteúdo inserido/reutilizado; a mera criação não executa.
O init pertence a cada leitor. Não chamar State.display prova vanilla: é extensão
Tekt. Estados com Set conservam paridade e controles anteriores.

### Aceitação e fronteiras

Gate A1-S2 ratificado para as cinco assinaturas e a fase de leitura. L0 primeiro,
RED funcional antes do código; GREEN contra o pin indicado para linguagem e
controles próprios das extensões Tekt. Não ampliar outros contratos públicos.
Build/workspace/doctests, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e
reversa; manter V16 sem aumento. MemoryMax12GiB/High8GiB/swap0/jobs1, tarefas
pesadas sequenciais. Medir consumo sem inferir ganho global só da representação.


## A1-H2 — suplemento antes das consultas, aprovado em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
final-freeze.json.gz e target-controls.json.gz no diagnóstico
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17 registram
UTC/diffstat/fontes/argv. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-raw-constructor-func passa no baseline/vanilla e falha no candidato:
Styled de transporte altera func. Query também vê wrapper causal temporário;
HTML ignora callback panic. Fonte walk.rs:930–945; pipeline/paged.rs:419–427 e
pipeline/html.rs:101–106 preparam a consulta pelo walk puro. São valores e erros
de linguagem, não um requisito de copiar algoritmo/estrutura do vanilla.
Gate A1-H2 confirmado pelo dono nesta sessão: revisão
00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md,
SHA-256 66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c.

### Responsabilidade exclusiva

Adicionar controles raw: func, supplement presente, numbering:none explícito,
campos em ordem e wrapper Styled comum opaco, incluindo texto. Comparar valores
da linguagem, preservando negativos de ausência e sem espelhar o helper.

### Aceitação e limites

H1–H8 da revisão ratificada, RED bilateral antes do código. Nenhum contrato
público além da fachada Heading está autorizado por H2; A1-S2 cobre separadamente
cinco helpers State. Sem novos campos públicos. Styled ordinário e demais
callbacks conservam suas fronteiras. Workspace/doctests/build/Clippy/Rustdoc,
fmt, V1/V5/V15/V26/reversa e V16 sem aumento; 12GiB/High8GiB/swap0/jobs1,
comandos pesados sequenciais. Ganho de RAM exige medição, não intenção.


## Concatenação de conteúdo — 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c e working tree/UTC/diffstat/fontes
em before.json.gz de 00_nucleo/diagnosticos/state-stack-2026-10-17.
morphology-before.json.gz/morphology-measured.json.gz comparam repr de loops e +
com vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3: o Tekt encaixa Sequence
a cada junção; vanilla concatena os filhos imediatos. debug-root.log localiza
a falha de pilha na varredura da cadeia produzida por 64 updates. São forma e
falha de linguagem, sem obrigação de copiar estrutura Rust ou aumentar stack.

### Responsabilidade exclusiva

Possuir testes E2E de eval para for/while, concatenação + e aliases com origem,
usando assert.eq(repr(...), ...) no nível da língua. Fixtures devem ser válidas
no oráculo; não acessar children como pré-requisito enquanto sua ausência atual
é dívida separada. O algoritmo não é owned por estes testes: é especificado em
00_nucleo/prompts/compiler/eval/operators/join.md.

Preservar os testes anteriores. Não reduzir profundidade ou quantidade do ensaio
original para fazê-lo passar. Confirmar RED→GREEN e revalidar sob 12GiB/swap0/jobs1,
sem contratos públicos, fase ou defaults novos (ADR-0127 contínuo).

## R1 — reflexão de sequência e vazio, fluxo contínuo ADR-0127

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado.
00_nucleo/diagnosticos/sequence-reflection-2026-10-17/before.json.gz e
before-sources.json.gz guardam UTC, diffstat, fontes e SHA dos binários. Pin
vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3. No upstream
lab/typst-original/crates/typst-library/src/foundations/content/mod.rs:724–728,
Sequence tem children requerido; :538–585 projeta has/at/fields. As sondas
empty-fields/empty-func/empty-children e sequence-fields/sequence-children/
sequence-single/sequence-for refutam o Tekt: campos ausentes e nome empty.
fields.rs:317–322 deixa Empty/Sequence no complemento sem campos;
identity.rs:24 usa empty; content_methods.rs:242 enumera somente body no
fallback. São morfologia e semântica da linguagem, não exigência de copiar
representação Rust. Text vazio continua texto; Styled comum permanece opaco.

### Responsabilidade exclusiva

Testar fields/children/has/at em Empty, Sequence com um ou vários filhos,
loops e aliases persistentes; erros de campos inexistentes usam sequence.
Controles negativos mantêm Text sem children e Styled opaco. As expectativas
vêm das sondas bilaterais antes do código; RED real antes da implementação.

### Aceitação e divisão explícita

R1 é incompleto para a função Sequence: R2 completará construção e admissão
em selector/where/show/counter, medindo argumentos, identidade e limites antes
de alterar seus owners. before.json.gz registra esses refutadores abertos.
Não anunciar paridade total de func() ou de sequência por este reparo.
A mensagem genérica de método inválido também permanece fora de R1.

Sem novos campos, tipos, traits ou assinaturas Rust públicas, fase ou modo
de produto. L0 antes de código, RED→GREEN, testes de regressão, workspace,
build, Clippy/Rustdoc estritos, ownership/núcleos/linhagem e reversa válidos;
V16 não aumenta. Resselo somente após V15/V26 válido. Validação sequencial
com MemoryMax12GiB/High8GiB/swap0/jobs1; registrar pico, não inferir ganho.
