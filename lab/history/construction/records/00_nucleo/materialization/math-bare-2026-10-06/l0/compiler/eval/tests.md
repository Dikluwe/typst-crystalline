# L0 — compiler/eval/tests
Hash do Código: 85659501

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:fb383f227b0bcd8b1e61899942fa0383a4cda66093da58bda2e87844e2895d2a
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8

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
