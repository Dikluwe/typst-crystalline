# L0 — compiler/eval/rules/apply — Aplicação de show rules
Hash do Código: 78bdbaa7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/layout/raw-block-boundary.toml sha256:c983fdb9884f90548e110fe7c1709632fc00b6e2a1387ad9e80444009c3602fd
- 00_nucleo/prompts/_nuclei/layout/raw-defaults.toml sha256:2ce5a50363605eb1a122920b92773b1fc956c52817dd57830c2ce12bfe5f285e
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/show-occurrence.toml sha256:3c1058d01f7ca34817e561ef210e78eda38dcb993028414d0b30f296d35535d7
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:16ae092d990d3d26f03232afcabf63f3b5c08ffb5b6297989368f0346d14f2ab
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:1d697fae328277525e44cad674cecc537ceb23a1bb3ae11bb68a0a9575411ae4
- 00_nucleo/prompts/_nuclei/table/row-groups.toml sha256:472e9e168c439ce5e1d17557fd8dbb55047e2f3c2f259eada2ff8b763ae27ade

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/apply.rs`

## Medição antes da decisão

A working tree não commitada capturada em `00_nucleo/diagnosticos/atomizacao-eval-rules-2026-09-30-before.json.gz` contém as rotinas desta família em `01_core/src/compiler/eval/rules.rs`. A suíte L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem prova bilateral nova contra o vanilla. As medidas históricas e ressalvas do pin continuam em `00_nucleo/prompts/compiler/eval/rules.md`.

## Contrato de linguagem

Aplicar regras com precedência léxica, guardas RuleId, limite de profundidade, selector matching do owner próprio, rotas textuais e parágrafos realizados uma vez. Preservar erro e span causal.

As assinaturas, identidades, casts, defaults, precedência de erros e fase de avaliação observados permanecem. O parent mantém diagnóstico compartilhado e bindings estáticos; outros owners de selector matching, terminação e textual realization continuam exclusivos. Não duplicar esses algoritmos.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. Paridade em semântica, sintaxe e morfologia (ADR-0107/0108), ownership exclusivo e pins válidos (ADR-0129/0130).


## Realização com avaliação única

Quando defer_show_rules está ativo, capturar body, regras, styles, guards e
arquivo em ShowRecipe e devolver sem executar receita. Quando realizing_show_rules
está ativo, preparar identidade antes de chamar a receita; Located já preparado
conserva sua identidade. Transformações preservam a origem em Realized e passam
Located à receita. Show-set e matching continuam nos owners atuais; o visual
original não é reemitido apenas para manter query.

## Proteção por ocorrência na realização produtiva — revisão 2026-10-06

### Medição anterior à decisão

`00_nucleo/diagnosticos/show-recursao-auditoria-2026-10-06/measurement.json.gz`
registra oito controles, fonte/L0, comandos, HEAD/working tree/UTC e binários
contra o vanilla pinado 9f2b6e871. Quatro controles finitos coincidem na
aceitação. fresh_same_body, fresh_z, alias_plus_generated e text_and_generated
compilam no Tekt e são rejeitados no vanilla por maximum show rule depth exceeded.
`apply.rs:75-92`, `eval/tests.rs:1259-1273` e o owner
`00_nucleo/prompts/compiler/eval/show_rule_termination.md` descrevem a política
α de ponto-fixo por morfologia; o rótulo consciente não comprova ratificação
humana. `typst-realize/src/lib.rs:462-495` no oracle distingue regra já aplicada
ao elemento de elemento novo com corpo igual. Aceitação, efeitos, contagem e
conteúdo emitido são observáveis da linguagem, não igualdade mecânica do Rust.

### Decisão e responsabilidade exclusiva

Na realização produtiva de elementos localizáveis, proteger a aplicação de
cada regra por identidade da ocorrência Located existente. Alias de it mantém
a identidade protegida e pode ser emitido duas vezes sem aplicar novamente a
mesma regra. Construção nova tem identidade distinta mesmo com morfologia igual;
se o produto voltar a casar, realizar suas regras no ambiente lexical correto.
Não encerrar essa expansão só porque corpos ou morph_canon são iguais, nem
propagar uma guarda global de RuleId que torne todo descendente imune à regra.

A expansão de produtos conserva origens Realized, estilos, regras where/show-set,
ordem causal, namespace de contextos, erros e consultas. Seus registros de
proteção e profundidade pertencem à realização corrente, em RAM privada; não
usam estado global, não reavaliam o arquivo e não acrescentam entidade/trait/
assinatura pública ou nova fase. O limite existente de profundidade interrompe
a geração não finita com maximum show rule depth exceeded e hints correspondentes.
Não reciclar identidades, confundir alias com clone estrutural, contar a origem
consumida como emissão nem repetir efeitos de um original já guardado.

O helper de terminação legado permanece governado exclusivamente por
`00_nucleo/prompts/compiler/eval/show_rule_termination.md`; seu algoritmo não é
alterado por esta revisão. Matching continua em seu owner; este owner coordena
a aplicação e revisitação de produtos na realização produtiva.

### Aceitação e compatibilidade

As quatro gerações infinitas medidas devem rejeitar. Alias, alias duplo,
where seletivo, substituição finita, múltiplas regras e contextos/efeitos são
refutadores obrigatórios: devem conservar resultado, ordem e número de ocorrências
contra o pin atual. Suítes L1/L3, corpus de realização, macro e layout precisam
de revalidação; RED→GREEN não pode redefinir o controle adversarial como aceito.

Esta revisão retira a aceitação produtiva por ponto-fixo morfológico para
elementos novos. Documentos como show heading: it => heading[Z] passam de saída
Z a erro, enquanto show heading: it => it permanece válido. Por essa quebra de
comportamento expressamente documentado, materializar somente após confirmação
do dono no gate ADR-0127. Não introduzir uma flag nova para ocultar a diferença.

### Refinamento após controles adversariais

controls-first.json registra expansão duplicada de aliases dentro de block e colisão de RuleId numa regra local sobre it. core-first.log registra cinco expectativas produtivas da política revogada, distintas dos testes do helper legado. Antes de aplicar regras, avançar o alocador privado acima dos IDs capturados. Produtos Realized já concluídos delimitam seus visuais preparados: coletar as identidades desses visuais antes da travessia e não reaplicar receitas nem reemitir o produto memorizado nesses descendentes. Aliases ainda em realização conservam proteção em andamento; aliases fora de visual concluído podem reutilizar produto terminado. Não usar corpos iguais ou placeholders de Content para inferir essa fronteira. Em full_error, conservar a classificação adicional por histórico privado de formas ancestrais apenas como diagnóstico, sem usá-lo para terminar expansão.

core-third.log refuta usar a intercepção geral para inserções: P790 literal show com função volta a casar durante interpolação. Expor somente fronteira interna de crate para realizar localizáveis inseridos com snapshot lexical, reutilizando a mesma rotina por ocorrência; não executar regras de texto, parágrafos ou terminação não localizável nessa fronteira.

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

No fluxo produtivo de realização e antes de localizar/matchear cada nova inserção,
preparar Mathy externo com bare_math::prepare no escopo causal existente.
Reutilizar o matching e proteção por ocorrência. Fazer o mesmo para produtos
novos efetivamente gerados por show antes da revisitação; o corpo de Equation
que continua em contexto matemático não é inserção externa nova.

Alias de it preparado mantém Location e proteção. Devolver it.body fora de
math cria nova Equation de identidade distinta; não encerrar por igualdade de
corpo ou guarda global, e manter limite de profundidade e diagnóstico causal.
Captura deferida mantém o Mathy cru e snapshot; preparação produtiva não muda
valor de variável ou constructor durante eval. Não executar texto/parágrafo
ou receita duas vezes por inserir o wrapper, nem perder regra onde não havia
localizável no conteúdo cru. A fronteira realize_inserted_occurrences mantém
seu contrato interno e apenas inclui Mathy elegível em sua realização.

### Gate e aceitação

Somente materializar após confirmação humana do pacote math-bare-2026-10-06.md.
RED→GREEN e revalidação bilateral do escopo são obrigatórios. Esta cláusula
supersede restrições de fase anteriores apenas nas fronteiras aqui nomeadas,
sem ratificar paridade universal, D07, math label ou outros débitos.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

Possuir aplicação root-only de Table/Cell pelo motor de regras atual. Table/Cell e corpos pendentes são fronteiras de descida até o estágio definido pelo núcleo. Na realização produtiva, aplicar regra da Table antes de seu plano/cells; delegar default sobrevivente ao helper table_cells. Aplicar Cell com propriedades conhecidas antes de descer no body; snapshot it marca RecipeApplied para impedir sua regra novamente, e produto/body é realizado uma vez antes de Complete. Conservar proteção de Table locatable e produtos gerados; não atribuir Location à Cell. Matching, guards, regras textuais, termos e sinks continuam nos owners atuais. Preservar a política legada de terminação de não-localizáveis neste recorte; a geração fresca show-cell-reconstruct continua refutador fora da certificação D08B3-A, não é usada como GREEN.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.

## Raw defaults — medição pinada 2026-10-07 antes da decisão

raw-defaults-2026-10-07/before.json.gz e before-state.json.gz registram
HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, diffstat,
UTC, fontes, comandos e binários. Tekt 58456b9b54041983612e5946f89fef2cf4591b9f31a09d8dcce9ae5c8a9c0eea;
vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8 e binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
A medição dos 16 controles mostra Serif 11pt em Raw contra Mono 8.8pt,
20pt contra16pt sob tamanho global, show-set absoluto15pt prevalecendo,
show-font explicitamente Sans, show-size1em vanilla8.8pt/Tekt0pt,
substituiçãoABC mantendo defaults de Raw, e fill vermelho herdado.
Fonte pinada text/raw.rs:658–670 aplica defaults show-set antes dos overrides;
raw.rs:103–123 conserva fonte/tamanho e descarta fill; apply.rs:207–253
embrulha overrides mas perde defaults de Raw no produto; style_scope.rs:41–44
resolve StyleChain.size por abs, perdendo em. São estilos/geometria
observáveis da linguagem, não exigência de buffers ou bytes PDF.

### Decisão interna de paridade, fluxo contínuo ADR-0127

Usar a composição Styles/StyleDelta já existente, sem campo, trait ou
assinatura pública novos, sem antecipar coordenadas ou mudar fase do pipeline.
Os defaults de render ficam no owner Raw: fonte Mono, tamanho0.8 do exterior,
bold/italic desativados e fill exterior herdado. Na emissão direta de Raw
aplicar esses defaults e restaurar o exterior. Quando show de Raw produz
conteúdo (inclusive Text), marcar um escopo privado raw.defaults no Styles
exterior aos overrides, para que o mesmo owner instale defaults no render
antes dos overrides. Marcar raw.defaults.active somente nesse escopo,
e impedir aplicação dupla ao Raw devolvido por it. Styled resolve text.size
Length contra tamanho anterior apenas dentro desse escopo Raw, preservando
composição absoluta/relativa e restauração da chain. Nenhum parser ou
algoritmo de show/matching/guard é duplicado.
Para typc true/false, usar o foreground constant.language da paleta existente
#d73948 quando não houver gramática suportada; não transformar em Text nem
remover Raw morfológico. Outras gramáticas/whitespace continuam no owner atual.

### Aceitação e limites explícitos

RED→GREEN de fonte/tamanho/fill/restauração, overrides e substituição.
Recoletar os16 controles, glifos reais de células e regressões. Mensagens,
ordem, índices e targets continuam preservados; fonte subset e bytes não
são critério. Whitespace Raw/syntaxes customizadas e reconstrução fresca
de Raw dentro de show não são certificados por estes controles. A alegação
histórica P1055 de tamanho100% é refutada para este pin e substituída aqui.


## Raw bloco — medição 2026-10-07 antes da decisão

raw-block-spacing-2026-10-07/{before,before-additional}.json.gz e before-state
registram HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada,
UTC/diffstat/status/fontes/comandos/binários. Tekt43207349fba51afbdbcac3847c1a7d884e9ff845b6401022be22272679c00575;
vanilla pin9f2b6e8715237cb086899a42873660fe744622e8 e binário7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Raw prefix/suffix: yMin85.806243/95.064120 contra91.304203/108.594100.
Cell usa x70.86614 contra75.86614; Cell isolada acrescenta0.55208pt vertical.
Unknown-language colapsa abc/def emabcdef. SubstituiçãoABC perde a fronteira.
Show-size15 usa margem18pt no vanilla; show-em1 mantém13.2pt.
block-margins e show-margins above2/below7 são efetivos; par.spacing20
produzRaw98.104163/After122.194100. EmptyRaw temAfter101.90816.
Fonte pinada typst-layout/src/rules.rs:655–679 realiza Raw como Block;
container.rs:324–360 define margem/colapso. Raw consumer fecha linhas e
usamargem global; cursor.rs:88–132 já consome below pendente;
sub_frame.rs:315–325 usa avanço hipotético quando a linha terminal já fechou.
São geometria/texto observáveis, não algoritmo interno ou bytes.

### Decisão — correção interna de paridade ADR-0127

Seguir o núcleo raw-block-boundary pinado abaixo. Nenhum campo/trait/API
público ou fase nova. Raw possui begin/end físicos privados, local origin,
resolução de margens e classificação terminal do produto Raw. Styled orquestra
esses helpers para produto transformado, com markers do apply no Styles
existente. Apply conserva flag block e Length de tamanho/margens dos
overrides sem resolvê-los. Sub-frame terminal Raw mede a borda final real
menos seu topo, não leading/margem pendentes. Testes cobrem região local,
produto substituído e linhas distintas em fallback.

Ao iniciar, fechar prefixo material; usar baseline/descent anteriores e
margem acima colapsada, com baseline inicial geométrica. Primeiro Raw de
sub-frame vazio começa no topo local. Ao terminar, registrar borda inferior
real e deixar below no protocolo existente do Cursor. Nenhuma soma de
margem final à altura terminal. Unknown syntax mantém quebras explícitas
entre linhas de Raw bloco, sem normalizar espaços/tabs nem alterar inline.

### Aceitação e limites

RED→GREEN e recoleta dos14 controles +5 refutadores, tipografia anterior,
Cell/morfologia/math/query, suites/build/lint e preservação restrita.
Geometria observada decide; não introduzir tolerância posterior para apagar
resíduos. Whitespace/tabs/linhas vazias internas, syntax customizada,
reconstrução fresca e rich block styles/HTML continuam não certificados.

### Refutação e revisão da primeira fórmula de margem

after.json.gz/after-additional.json.gz (binário/snapshot/UTC completos)
refutam o primeiro reparo: consecutivos têm gap extra2.64pt; Cell prefix
continua5.49796pt acima; Cell isolada fica2.59598pt acima; margens explícitas
não chegam à chain, pois set.rs:976–1006 não reconhece Native block.
refuters-red.json.gz mostra Strong-cell perdendo altura (After83.27018
contra89.95610); plain-cellAfter87.36012. Também identifica negativos
preexistentes de parágrafo sizes-up/down, cada um comdelta10.8pt.

A margem Raw default é fraca e usa Raw80%:10.56pt, não13.2pt. Um parágrafo
exterior11pt contribui13.2pt, enquanto Raw consecutivo contribui10.56pt.
Usar prev_margin_is_parbreak como protocolo já existente de margem fraca
de parágrafo, incluindo Raw que herda essa margem: below explícito continua
forte e não muda esse ramo. Cursor colapsa margem fraca com par.spacing
actual, sem alterar o ramo de margens fortes, regiões/fracionárias/checkpoints.
Parágrafo anterior já fechado em sub-frame ainda participa da margem acima.
Primeiro Raw de sub-frame usa cursor inicial menos top-edge exterior, porque
cell_body.rs:253–260 recompõe sub-frame legado com ascender-top-edge.
Classificação terminal inclui Strong/Emph transparentes, sem wildcard
descartando dados: é predicado de fronteira, não projeção de enum.

Set proprietário aceita por identidade Native block e aliases somente
o subconjunto spacing/above/below com Length ou Auto; avalia uma vez,
publica delta apenas depois de validar o conjunto. Campos/tipos fora
desse subconjunto conservam fallback/advertência anterior e não são
certificados. Apply transporta essas propriedades e o tamanho como Length.
A margem de render usa o tamanho Raw efetivo (default ou show-set); o
parágrafo anterior contribui o exterior separado. Styles não mede glyphs.

A hipótese é refutável por qualquer negativo que permaneça nas coletas.
O parágrafo sizes-down já era negativo e não é declarado corrigido aqui;
não ampliar para regra de tamanho de parágrafo anterior sem medição própria.

### Refutadores finais de fronteira zero/auto e acesso a fontes

edges-red.json.gz prova: below0 sobrepõe After (74.95612 contra82.19410);
negativoAfter72.95612 contra77.19410; auto +spacing20 não sobrepõe auto
corretamente (Raw98.10418 contra91.30420); primeiro below0 deve vencer
above automático do Raw seguinte (def97.99011, não108.55011).
A margem explícita tem prioridade nos dois sentidos; ambas explícitas ou
ambas fracas colapsam por máximo. Auto de edge interrompe o shorthand
spacing e retoma par.spacing/default fraco. Below explícito<=0 deixa cursor
no fim real +below com baseline geométrica pendente; below fraco, inclusive
zero, passa ao ramo fraco do Cursor, que adiciona top-edge do texto seguinte.

suites-third.log refuta inicialização incondicional de FontMetrics: dois
controles place nontext (image/shape) passaram a adquirir font book.
Inicializar ascent regional somente no sub-frame legado cujo produto
terminal é Raw block. Geometric/nontext conserva a rota sem consulta nova.
Dados sem texto não dependem de fontes por esse reparo.

## D08B3-B — candidato, não vigente antes de ratificação

### Medição anterior à decisão

00_nucleo/diagnosticos/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz contém 53 sondas, coletadas em 2026-10-07T14:47:56.340144+00:00 até 14:48:12.375245+00:00. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diff HEAD --stat/status/hashes/fontes/argv/cwd/ambiente e saídas completas registrados. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f. Não misturar before/refuters/complete como rodadas adicionais independentes.

isolated-default: vanilla expõe x/y/inset/align/fill/breakable auto, spans 1 e stroke dict vazio; Tekt omite esses defaults e publica duas metadata contra uma. isolated-x rejeita no Tekt, funciona no vanilla. raw-fields continua só body nos dois e raw-x rejeita nos dois. isolated-table-style confirma que defaults de Table não viram os da Cell avulsa. raw-vs-prepared mede false no vanilla e true no Tekt. nested-effects/body-effects/late-rule refutam execução única e origem lexical. fresh-isolated/table, fresh-block, fresh-styled e multi-fresh são aceitos indevidamente pelo Tekt; vanilla rejeita maximum show rule depth exceeded com span do constructor novo. identity/body/alias e fresh-conditional/hidden continuam válidos. multi-identity e multi-return-table mostram que devolver it deve permitir receita anterior ainda não consumida. isolated-query mantém cell is not locatable nos dois; location é none, sem localização falsa. cell-set é negativo separado, explicitamente excluído deste pacote.

Fonte vanilla lab/typst-original/crates/typst-library/src/model/table.rs:734–778 declara defaults e propriedades; typst-realize/src/lib.rs:367–386 prepara/guarda o argumento, :418–429 visita produto com profundidade, :491–505 distingue guards por elemento, :571–586 materializa propriedades. Tekt entities/elements/table_cell.rs:111–143 só projeta lexical ou plano posicionado; compiler/realize/table_cells.rs:110–129 reaplica Cell crua sem preparo; :280–304 termina snapshot inteiro; eval/show_rule_termination.rs:65–70 trata mesma forma como ponto-fixo; eval/rules/apply.rs:139–166 revoga regra globalmente durante callback. Classificação: campos, efeitos, ordem, terminação e erro causal são língua. A estrutura de guards e materialização upstream é mecânica e não é requisito. Não se infere intenção do upstream; a escolha nominal abaixo é inferência refutável pelos controles de aliases, duas regras, novas gerações e conteúdo ocultado.

### Responsabilidade exclusiva proposta

Para Cell preparada, aplicar no máximo uma transformação de nó por visita, na precedência atual; selector observa getters já preparados. O argumento it recebe o RuleId consumido no carrier antes da função, não por mutação lexical. Preservar guards de aliases retornados e não atribuí-los a novo constructor. Permitir receitas anteriores não consumidas, inclusive quando a última devolve it ou [B #it]. Show-set segue o protocolo atual de estilos e não cria carrier em Styles. Guard global de parent Cell não impede uma nova ocorrência; guards de outros elementos ficam. Produtos escondidos não são realizados. Captura contextual e RAW/defaults/matemática/locatables não mudam. Delegar terminação ao seu owner sem repetir automaticamente a mesma forma de Cell preparada.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.

### Refutadores do binário — captura causal do produto de Cell

Medição anterior à decisão: table-cell-isolada-d08b3b-2026-10-07/after-second.json.gz mede growing com erro detached e block/styled com caret do wrapper externo. Invoke intercepta wrappers e markup ainda durante a função de Cell; isso pode consumir limite/realizar produto antes da fronteira emitida. Durante a função de Cell preparada, usar o protocolo existente defer_show_rules para capturar interceptações do produto, restaurando o modo em sucesso/erro. O produto é realizado pelo owner da inserção sob sua rota causal, incluindo wrappers; constructor cru/corpo lexical e regras de outros elementos fora da função conservam seu protocolo. Não adicionar estado/campo público, lógica de I/O ou modo do produto. Testes e refutadores devem confirmar efeitos uma vez, duas regras e conteúdos ocultados; a hipótese é refutada se estilos/escopo/caret/ordem regredirem.

## D08B3-C — consumo de stroke resolvido e fold de show-set

Medição anterior à decisão: table-cell-styles-d08b3c-2026-10-07/show-fold-before.json.gz compara duas declarações show-set Cell/Table: vanilla combina red e 2pt, Tekt perde red. geometry-final.json.gz / cell-stroke-two-sets e cell-stroke-constructor registra PDFs/argv/binaries/HEAD/working tree/UTC, cuja conversão pdftocairo SVG contém stroke preto 1pt Tekt contra red 2pt vanilla. Tekt borders.rs:43–48 lê e.stroke lexical; apply.rs:244,465 colapsa deltas cell.stroke com top-wins. Vanilla model/table.rs:765 #[fold] e layout/grid/resolve.rs:225–237 resolve componentes; typst-layout grid/layouter.rs:568–578 desenha o stroke já resolvido. É cor/espessura visível e campos de linguagem, sem exigir igualdade de bytes SVG/PDF ou número de comandos de desenho.

Decisão contínua ADR-0127: helper privado puro TableCellProperties::chain_styles combina somente Value::Stroke declarativo em cell.stroke, mantendo None explícito, demais chaves/topologia/ordem/guards. Apply usa esse helper na acumulação de show-set em ambas as rotas existentes; realizer usa ao aplicar deltas locais. Não armazenar Cell/plano em Styles. Borders, para TableCell preparada, consome somente o snapshot sobrevivente retornado pelo owner cell_body::prepared_default, inclusive None conhecido ou ausência de snapshot substituído; não faz fallback lexical/container. Grid e Table crua conservam prioridade histórica. Não mudar geometria de extremos/coalescência, assinaturas, fases, modos nem contratos públicos. RED→GREEN antes de código, cores/espessuras reais via SVG em unidades físicas, controles de default/None/substituição e corpus/suítes/lint/preservação obrigatórios. Alinhamento de linha fixa e resíduos métricos anteriores não são inferidos como resolvidos por este reparo.


## RG1 — grupos Table ordenados e síntese estrutural — candidato 2026-10-17

### Medição anterior à decisão

Medição anterior: 00_nucleo/diagnosticos/table-row-groups-2026-10-17/before.json.gz, measurement.json.gz, supplement.json.gz, lines.json.gz, styles-phase.json.gz e svg.json.gz conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC/status/diffstat/fontes/binários/argv/cwd. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. table_grid.rs:126–150 funde grupos; row_groups.rs:22–35/70/355–366 conta entradas, table_cells.rs:1204 aplica receita antes do planner e :1259–1282 deriva prefixos. model/table.rs:307–316 sintetiza; typst-realize/src/lib.rs:367–385/558–568 sintetiza antes de receita; layout/grid/resolve.rs:1380–1457/1597–1650 preserva intervalos e cursores; typst-html/src/rules.rs:591–665 projeta papéis. TableHeader level/presença somem no Tekt, coordenadas de linha não são lidas em table_lines.rs:173–260, e supressão None em layout/grid/rules.rs:40–46 deixa traços. svg-analysis.json confere todos os strokes retos e mudanças pareadas (mesmo binário): top vermelho vanilla 82→41, Tekt 82→82; azul após None vanilla 41→0, Tekt 40→40. Estes são efeitos/células/roles/erros/geometria da língua; não obrigação de copiar vtables, estruturas Rust, bytes SVG/PDF ou inferir intenção upstream. O primeiro extrator SVG não reconhecia line/h/v; foi descartado como prova, conservado e substituído por extração completa registrada.

### Responsabilidade exclusiva

Integrar a preparação estrutural Table no ponto causal após estilos declarativos show-set aplicáveis e antes de invocar transformação/receita Table. Reutilizar resolução de estilos e os owners de matching/guards; não duplicar aplicação de regras. Erro estrutural antecede panic da Table, mas Cell show/body continuam condicionados à sobrevivência. Respeitar defer_show_rules: capturar snapshot sem executar código; na realização preparar uma vez com ambiente causal correto. Não expor plano pending como visual nem alterar outras famílias. Revalidar literalNone/Empty, alias/it, produtos novos, show-set em ordens invertidas, query/fixpoint e efeitos ocultados. Nenhuma fachada pública nova.

### Gate, aceitação e incompletude

Contrato candidato RG1: não vigente antes da confirmação humana do manifesto em 00_nucleo/materialization/table-row-groups-2026-10-17/review.md. A aprovação precisa abranger contratos públicos, compatibilidade e fase (ADR-0127 §2.1/§2.3/§2.4). Após aprovação, esta seção supersede apenas cláusulas históricas conflitantes de grupos Table/linhas e da fronteira estrutural/receitas. Outras propostas não são ratificadas. Não implementar campo/API novo antes do gate. Instalar L0/kernel/pins primeiro, testes funcionais RED antes do código, sem tratar erro de compilação/setup como RED; GREEN mais R1–R20 e corpus bilateral, build, workspace completo, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, formato, V1/V5/V15/V26 e reversa, diferencial SARIF/replay/diff-check. V16 externos continuam abertos; não alterar severity, criar allow/expect ou esconder um complemento em if-let só para zerar lint. V19 tem justificativa local e corpo compartilhado quando a política é a mesma. Núcleos são compartilhamento normativo; cada L0 conserva exatamente um consumer. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais; sondas/mutações usam target Cargo isolado e nenhuma variante mutante vai para target principal. Sem commit/push/merge/release ou pin novo.
