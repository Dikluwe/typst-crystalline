# L0 — compiler/eval/rules/apply — Aplicação de show rules
Hash do Código: 3335624f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/show-occurrence.toml sha256:3c1058d01f7ca34817e561ef210e78eda38dcb993028414d0b30f296d35535d7
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

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
