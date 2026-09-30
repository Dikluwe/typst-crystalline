# L0 — compiler/eval/tests
Hash do Código: a983a88c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020

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
