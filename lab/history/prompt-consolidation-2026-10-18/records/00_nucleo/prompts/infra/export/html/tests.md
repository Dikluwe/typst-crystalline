# L0 — html/tests
Hash do Código: dbf5cea8

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/typc-atom-display.toml sha256:2989f08ae6b4b379afb61d9673b94ab8538c45a97b98e566677ae0f719429dc3
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

## Consumer evidence

`03_infra/src/export/html/tests.rs:13` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Regression suite ownership

Own the existing cfg(test) HTML suite and its fixtures/assertions. Cover serialization
modes, ordered DOM/attributes, void/presence errors, styled raw/title, whitespace,
explicit roots, ruby, tables/spans, protected pretty contexts and frame resolution.
Exercise public paths and private helpers through existing restricted access.
Expected behavior comes from the productive owners, not a second normative encoding
contract here. Byte assertions are encoder regressions, not universal vanilla parity.
Do not add network, layout or I/O merely to reorganize this suite.


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

RED→GREEN de SmartQuote markup-like/nativa na exportação HTML sem Paged,
idioma após captura, disabled, nesting em span, title, irmão separado por
styles e boundaries. Verificar texto DOM e escaping por modo; preservar
raw/style/string rejection, headers e HtmlFrame. Não adaptar lang=en existente
como igualdade bilateral ou esconder guarda Unsupported de outros Content.

## D08B2 — presença lexical ratificada em 2026-10-07

### Medição anterior à decisão

lab/results/construction/diagnostics/table-cell-realizacao-2026-10-07/before.json.gz e
presence-refuters.json conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree, diffstat, UTC, binários e argumentos contra o pin 9f2b6e8715237cb086899a42873660fe744622e8.
Upstream model/table.rs:734–778 possui campos Smart/Option; o Tekt
entities/elements/table_cell.rs:40–51 conserva apenas Option de valor.
X:auto e x omitido colapsam; fill:none e stroke:none perdem presença.
Os refutadores medem auto permitido em x/y/fill/align/inset/breakable,
rejeitado em colspan/rowspan/stroke; none permitido só em fill/stroke.
São observáveis da linguagem, sem obrigação de copiar a estrutura upstream.

### Contrato ratificado e responsabilidade exclusiva

Fixtures TableCell especificam presence coerente com seus argumentos explícitos. Preservar todos os observáveis/assertions de render existentes; não relaxar tolerâncias nem fabricar defaults conhecidos. Testes não autorizam mudanças produtivas adicionais.

### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Contrato público por campo/tipos novos confirmado pelo humano em 2026-10-07; recibo table-cell-presenca-aprovacao-2026-10-07.json. Não altera fase/defaults de render.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/
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

Fixtures/testes preservam observáveis e tolerâncias. Ajustar constructors para novos campos crus/defaults; adicionar RED→GREEN observando fields, order, efeitos/alias/ocultação/HTML/geometry. Testes não autorizam comportamento além do contrato ratificado.

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

## D08B3-B — candidato, não vigente antes de ratificação

### Medição anterior à decisão

lab/results/construction/diagnostics/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz contém 53 sondas, coletadas em 2026-10-07T14:47:56.340144+00:00 até 14:48:12.375245+00:00. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diff HEAD --stat/status/hashes/fontes/argv/cwd/ambiente e saídas completas registrados. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f. Não misturar before/refuters/complete como rodadas adicionais independentes.

isolated-default: vanilla expõe x/y/inset/align/fill/breakable auto, spans 1 e stroke dict vazio; Tekt omite esses defaults e publica duas metadata contra uma. isolated-x rejeita no Tekt, funciona no vanilla. raw-fields continua só body nos dois e raw-x rejeita nos dois. isolated-table-style confirma que defaults de Table não viram os da Cell avulsa. raw-vs-prepared mede false no vanilla e true no Tekt. nested-effects/body-effects/late-rule refutam execução única e origem lexical. fresh-isolated/table, fresh-block, fresh-styled e multi-fresh são aceitos indevidamente pelo Tekt; vanilla rejeita maximum show rule depth exceeded com span do constructor novo. identity/body/alias e fresh-conditional/hidden continuam válidos. multi-identity e multi-return-table mostram que devolver it deve permitir receita anterior ainda não consumida. isolated-query mantém cell is not locatable nos dois; location é none, sem localização falsa. cell-set é negativo separado, explicitamente excluído deste pacote.

Fonte vanilla lab/typst-original/crates/typst-library/src/model/table.rs:734–778 declara defaults e propriedades; typst-realize/src/lib.rs:367–386 prepara/guarda o argumento, :418–429 visita produto com profundidade, :491–505 distingue guards por elemento, :571–586 materializa propriedades. Tekt entities/elements/table_cell.rs:111–143 só projeta lexical ou plano posicionado; compiler/realize/table_cells.rs:110–129 reaplica Cell crua sem preparo; :280–304 termina snapshot inteiro; eval/show_rule_termination.rs:65–70 trata mesma forma como ponto-fixo; eval/rules/apply.rs:139–166 revoga regra globalmente durante callback. Classificação: campos, efeitos, ordem, terminação e erro causal são língua. A estrutura de guards e materialização upstream é mecânica e não é requisito. Não se infere intenção do upstream; a escolha nominal abaixo é inferência refutável pelos controles de aliases, duas regras, novas gerações e conteúdo ocultado.

### Responsabilidade exclusiva proposta

Adaptar literals ao campo standalone=None sem alterar sentido dos controles antigos. Acrescentar apenas controles necessários ao comportamento owned de visual/clone/maps/representação/HTML/geometria, sem espelhar a implementação.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.

### Auto exposto pela Cell — reparo interno de apresentação, ADR-0127 contínuo

Medição anterior à decisão: table-cell-isolada-d08b3b-2026-10-07/after-second.json.gz, html-fields, em HEAD/estado/UTC/binários registrados: Tekt HTML X auto a como texto; vanilla code(data-lang:typc) e span constant.language #d73948. Upstream foundations/value.rs:191–208 exibe Auto/Bool como Raw inline typc; markup_stream.rs:210–215 só cobre Bool e seu fallback usa Text para Auto; layout/raw.rs:201–203 cobre só átomos true/false. HTML inline.rs não possui ramo Raw e conserva rejeição geral. É morfologia/estilo da linguagem, não byte de PDF; texto igual não prova paridade.

Antes da implementação, testes devem refutar a apresentação Auto como Text e a rejeição HTML de Raw atômico. Corrigir somente o mapeamento Auto para Raw(lang:typc,block:false), a classificação constant.language do átomo auto e a serialização HTML dos átomos typc auto/true/false com code e span constante, delegando escape/mode ao owner existente. Nenhum tipo/assinatura pública, fase, flag/default de produto ou gramática nova. A responsabilidade desta unidade limita-se à operação já owned correspondente: apresentação no markup, cor no Raw paginado, tags na inline HTML, testes no seu owner. Raw de outras linguagens/conteúdo arbitrário em HTML permanece fora da certificação. Confirmar RED→GREEN e igualdade do produto/estilos, com controles Table/Raw/query. Esta cláusula não autoriza mudanças nos outros tipos de Value ou implementação HTML geral de Raw.
