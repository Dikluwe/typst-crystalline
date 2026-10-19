# L0 — entities/content/traversal
Hash do Código: c72f6552

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0


**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/traversal.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own bottom-up content transformation and text transformation through
`map_content` and `map_text`. Traversal remains exhaustive, static,
order-preserving, and pure. Variant-specific recursion delegates to the owning
element. Replacement nodes are not recursively reprocessed unless the
existing contract already requires it.

## Acceptance

Existing traversal tests pass unchanged and the module includes local
tests.


## Realização com avaliação única

map_content recursa no body de ShowRecipe preservando snapshot; Located e Realized são ocorrências terminais para aplicação de receitas, evitando reaplicação aos corpos já realizados. Consumers que expandem contexto e estilos precisam de travessia explícita dos carriers quando pertinente.


## Realização com avaliação única

map_text transforma somente a projeção visual, conservando snapshots de origem e identidade.


## Realização com avaliação única

Travessia visual genérica recursa nos carriers. Located transforma descendentes e visita uma única ocorrência de raiz: o corpo raiz não recebe callback adicional. Realized transforma somente visual, conservando origem imutável. Isso permite expansão contextual/regex/estilos sem duplicar a ocorrência transportada.


## Realização com avaliação única

Located recursa nos descendentes sem callback duplicado de raiz; Realized recursa no produto e nunca transforma a origem consumida pela travessia visual.


## Realização com avaliação única

O callback de raiz desativado existe somente para a raiz do corpo transportado; descendants continuam sujeitos ao callback.

## Inserção matemática automática — proposta sujeita ao gate ADR-0127

### Medição anterior à decisão — D06, 2026-10-06

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado,
UTC/diff/estado/fontes/binários nas coletas canônicas
lab/results/construction/diagnostics/math-bare-2026-10-06/{before,additional,refuters}.json.gz.
Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8; proveniência no relatório
lab/results/construction/diagnostics/math-bare-medicao-2026-10-06.md.

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

Preservar integralmente o comportamento público de map_content e map_text.
Adicionar helper pub(crate) de travessia pré-ordem, com o mesmo callback tipado
FnMut(&Content)->SourceResult<Option<Content>>, sem assinatura pública nova.
A substituição Some é terminal: não visitar seus filhos ou reprocessar o
produto. None delega a recursão exaustiva estática existente e retorna em ordem.
A implementação compartilha a enumeração de filhos do owner, sem segunda tabela
de containers no compiler, vtable, registry ou dispatcher por nome.

Localização preparada continua sendo um carrier único, e origem Realized nunca
é transformada pela travessia visual. Esta unidade não decide o que é Mathy,
blockness, estilos ou aplicação de regras; isso é responsabilidade do owner
compiler/realize/bare_math. Testar ordem, fronteira de substituição, erros e
conservação da travessia pública antes/depois, incluindo Located/Realized.

### Gate e aceitação

Somente materializar após confirmação humana do pacote math-bare-2026-10-06.md.
RED→GREEN e revalidação bilateral do escopo são obrigatórios. Esta cláusula
supersede restrições de fase anteriores apenas nas fronteiras aqui nomeadas,
sem ratificar paridade universal, D07, math label ou outros débitos.

### Refinamento interno da travessia pré-ordem

Inspeção em traversal.rs:30–289 e Element::map_content mostra que a delegação
existente é bottom-up até nos filhos do elemento. O helper privado pré-ordem
possui enumeração estática própria neste mesmo owner e clona campos não-content;
não duplica catálogo de containers no compiler nem modifica métodos públicos
dos elementos. Carriers Dynamic de trait legado são fronteira opaca neste helper
privado; map_content público conserva a delegação dinâmica já existente. Esse
limite não certifica math dentro de Dynamic. Elementos internos com prefixo/campos
privados usam remapped/with_body existentes.

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

Reconstruções de TableCell preservam presence; transformações afetam body/texto sem assentar propriedades. Travessia e fase existentes ficam.

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

Table/Cell preparados possuem vista visual exclusiva: map_content/map_text/pre-order trabalham cells/visual, conservando origens raw e planos/props/stage. Raw mantém descida anterior; durante preparação os hooks da realização delimitam Table/Cell pendentes. Não fazer render/Engine no domínio nem visitar source e visual como dois irmãos emitidos.

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

Preservar standalone, stage e guards em reconstruções; mapas/preorder visitam visual preparado exclusivo, conservando fonte lexical. Cru segue body. Nenhum double-walk do body escondido, preparo ou receita em entidades.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Travessia Source preserva span e visita somente body, não acrescenta callback para wrapper. map_content_pre e map_text seguem mesma regra sem emitir origem separada.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Reconstrução somente após mudança — 2026-10-10

Medição anterior à decisão: `lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/change-aware-red-gate.json.gz` registra HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado/status/diffstat, UTC, script/fixtures/interpositor/CPU/comandos e release SHA-256 `29745a9e80ca43160b34f4807949f00e803070392653df0fc5e214b2f42ac33a`: 483.823 pedidos glibc em local-set-1600, acima do gate pré-fixado de 470.000. `entities/content/traversal.rs:25-55,235-264` reconstrói sequências/carriers/estilos/Table/Cell mesmo se todos os callbacks retornam None; `entities/elements/table.rs:85-118` e `entities/elements/table_cell.rs:308-344` clonam metadados e contribuições visuais. O vanilla pinado visita referências e materializa apenas após mudança (`lab/typst-original/crates/typst-realize/src/lib.rs:242-291,353-410`). Isso é uma hipótese de custo, sem exigir sua vtable ou igualdade Rust (ADR-0107/0026/0109).

Decisão interna de fluxo contínuo (ADR-0127): manter a assinatura pública e o match exaustivo de map_content; propagar internamente Option<Content>, onde None comprova ausência de reconstrução e Some transporta o resultado materializado. Some do callback sempre é uma substituição, mesmo quando compara igual; não usar PartialEq, endereço ou hash para inferir mudança, nem executar callbacks duas vezes. Cada callback recebe o nó após os filhos, na mesma ordem, e erros interrompem a descida como antes; substituições não são reprocessadas.

As primeiras variantes que propagam a prova são Source, Located, Realized, ShowRecipe, Sequence, MathSequence, Styled, Par, SmallCaps, Table, TableCell e terminais já classificados pelo match. Source conserva a normalização de with_source_span, inclusive origem aninhada e span detached; Located mantém a supressão de callback no body imediato; Realized transforma somente visual. Variantes ainda não migradas usam a delegação existente e retornam conservadoramente Some. A lógica específica de Table e Cell permanece nos respectivos owners `00_nucleo/prompts/core/entities/elements/table.md` e `00_nucleo/prompts/core/entities/elements/table_cell.md`.

Helpers exclusivamente crate-private permitem map_content_if_changed e mapeamento de slices sem alocar até o primeiro filho alterado; então clonar apenas o prefixo e irmãos preservados para o resultado. Nenhum campo/método de trait/assinatura pública/fase muda. Aceitação: RED→GREEN de reuso de pais/irmãos e alocação; callback bottom-up uma vez, substituição igual explícita, erro sem visitar irmãos posteriores, Source/Located/Realized e vistas preparadas; tempo pareado e qualificação de linguagem/render/query contra o release anterior e vanilla pinado. O recorte não promete reuso de todos os contêineres, map_text ou map_content_pre.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/content/traversal.rs:55` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.

Neste dispatcher, estender a prova às variantes Block, Boxed, Strong, Emph, Equation e a toda a família Math existente, conservando carriers Source/Located/Realized/ShowRecipe e regras já aprovadas. Nenhum corpo da lógica de elemento migra para o dispatcher.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.

## Transformações tipadas — preservação aprovada

Medição: `01_core/src/entities/content/traversal.rs:906` reconstrói matrix/body;
corpus transformacoes-2026-10-14 mostra perda de identidade e presença.
Manifesto aprovado 960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d.
Reconstrução deve copiar operation integralmente e transformar apenas body,
sem normalizar parâmetros, assar defaults ou reduzir variantes à matriz.
Testar mapas com zero/false/auto e valores dependentes do estilo.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T283: 01_core/src/entities/content/traversal.rs:83. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Unir braços None do mapeamento Source; decidir no corpo a mesma conjunção de spans. Manter None sem clone quando basta preservar origem; demais casos mantêm with_source_span, ordem e quantidade de callbacks. Sem reconstrução eager, igualdade ou visita adicional.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.


## I2 — cobertura estática bibliográfica (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/tekt-v16-bibliography-static-2026-10-17/before.json.gz
registra UTC, status, diff HEAD --stat, os textos anteriores e 2130 hashes
conferidos. bibliography.rs:21–79 enumera só parte dos containers e omite
Heading, Repeat, Columns e TableCell crua; :87–266 repete o catálogo parcial.
traversal.rs:337–1105 já possui enumeração estática pré-ordem completa, com
substituição terminal; :89–107 distingue Source, Located e Realized. O mapper
TableCell seleciona standalone.visual, resolved.visual ou body; visual() possui
outra prioridade, esclarecida no refinamento abaixo.
introspect/walk.rs:1206–1268 confirma os corpos de grid/header/footer/cell,
attribution de Quote, title de Bibliography e supplement de Cite; :1396
estabelece Document/Asset como recursos separados. São perdas internas de
conteúdo, não razão para copiar igualdade, bytes ou mecânica upstream.

A hipótese é que os conteúdos estáticos atualmente ignorados perdem chaves e
conversão Ref→Cite. Testes RED devem confirmar antes do código. A hipótese é
refutada se o caminho já conservar a chave/citação; não alterar esperados para
forçar essa classificação. Carriers internos usam a prova metamórfica da I1;
sem inferir paridade universal ou corrigir os limites do walker de math/HTML.
### Responsabilidade exclusiva

Adicionar somente helper interno pub(crate) visit_content_static, por
empréstimo, com closure FnMut(&Content)->bool: false poda o subgrafo; true
visita filhos estáticos. Sem alocar clones/coleções para navegar. Source,
ShowRecipe, Located e Realized são transparentes ao callback: body/visual
somente, nunca origem de Realized nem snapshots capturados. A enumeração de
Content é exaustiva, sem wildcard terminal; variantes novas exigem decisão.

Usar os campos Content navegáveis do mapeamento público: corpos simples,
seqüências, campos opcionais de Figure/Quote/captions/footnotes/bibliografia,
math (inclusive slots presentes e matrizes), HTML body, PageRun body e campos
estáticos de marginais/supplement/background/foreground; raw Table visita
children/header/footer/caption, preparada visita cells/caption; TableCell
visita exclusivamente visual() já definido. Outline visita title e realized_body;
OutlineEntry visita element/fill/prefix; Document body/title são navegáveis
apenas quando o callback não podar. Ref, SetPage, Value, callbacks, estados,
recursos e Dynamic/ContextBlock são terminais nesta vista estática. Não avaliar
Func nem chamar dyn_map_content. Não modificar map_content/map_text/pre-order
nem métodos públicos dos elementos. Nenhum tipo/trait/campo público novo.

### Aceitação

Testes locais de ordem/poda, transparência de origem, seleção exclusiva da
vista preparada e ausência de efeitos Dynamic. O helper é leitura estrutural,
não emite ocorrência/Location nem aplica regras. Variantes fictícias devem
provocar E0004 na enumeração de filhos. Dados/leaves não são tratados como
containers via introspecção Debug ou strings de nomes.

### Gate e validação

Correção interna ADR-0127: L0 antes de testes/código e resselo restrito aos
dois owners após grafo V15/V26 válido. Sem campo/assinatura pública, flag,
default ou mudança de fase; sem dependência ou publicação. Não silenciar V16,
V19 ou V20. Justificar no L0 o grupo nominal de terminais da visita (folhas
ou dados/callbacks opacos, sem Content estático ativo). Build, RED→GREEN,
I1, workspace completo, fmt, Clippy all-targets -D warnings e
-D unfulfilled_lint_expectations, Rustdoc estrito/quatro índices, linhagem
V1/V5/V15/V26/reversa, diff --check e freeze das fontes. Pesados seqüenciais,
MemoryMax12GiB/High10GiB/swap0/jobs1. Relatório com estado exato, argv,
hashes e UTC. I2 substitui a dívida de cobertura dos containers estáticos da
bibliografia da I1; demais frentes e objetivo global permanecem abertos.

### Confirmação RED e refinamento de empréstimo

red-result.json / red-state.json.gz registram a execução de 2026-10-17T20:02:52.979326+00:00 a
2026-10-17T20:06:42.623098+00:00: 0 passou, 2 falharam, sem erro de compilação, fontes
inalteradas durante a medição. As 18 posições perdem chave e conversão.
A navegação de OutlineEntry.prefix usa o novo accessor interno prefix_ref
do seu owner, pois prefix() faz clone; não mudar a interface pública.
O grupo nominal de 38 terminais da visita é: Empty, Text, Space, SpaceFromNewline, Parbreak, Raw, MathIdent, MathText, MathPrimes, MathAlignPoint, Linebreak, Ref, CounterDisplay, CounterUpdate, Image, Shape, Curve, SetPage, Flush, Divider, Asset, SmartQuote, PdfAttach, HSpace, VSpace, Pagebreak, Colbreak, GridHLine, GridVLine, TableHLine, TableVLine, Metadata, State, StateUpdate, StateDisplay, CounterDisplayCallback, ContextBlock, Dynamic.
Esses casos não possuem Content estático navegável nesta vista; Ref mantém
supplement opaco como no map_content vigente, valores/estados são dados e
Dynamic/ContextBlock não são avaliados. V19 deve continuar visível e
justificado; novas variantes exigem decisão no match exaustivo.

### Refinamento I2 — vista visual selecionada de Cell

selected-visual-attempt registra o primeiro GREEN: 5 passaram e o controle
de coexistência falhou na coleta (resolved contra a expectativa standalone).
A expectativa confundia a prioridade do mapper genérico com a vista ativa.
Fonte reavaliada: TableCellElem::visual, table_cell.rs:261–268, usa resolved,
senão standalone, senão body; finish_cell_product em realize/table_cells.rs:
1065–1080 publica o corpo realizado em resolved quando presente. O L0 da
entidade §correção de spans descreve coexistência: standalone conserva
propriedades observadas enquanto resolved mantém o plano ativo. A cláusula
de prioridade standalone do mapper é outra operação, não redefine visual().
A medição anterior que atribuía essa prioridade ao visual estava incorreta.

Testar separadamente resolved-only, standalone-only e coexistência. O ativo
é resolved quando existe, senão standalone; manter a origem raw e o carrier
inativo intactos. Registrar novo RED por Ref ainda presente no visual ativo,
sem mudar nenhum teste antigo de mapper nem corrigir seu contrato neste lote.
A conversão recebe arm específico de TableCell: copiar só o carrier ativo,
converter exclusivamente seu visual (ou raw se nenhum preparo), preservando
campos, presença, stage/guards/props e o outro carrier. Coleta usa visual()
existente. Nenhuma mudança em visual(), fases ou API. O desacordo dos
mappers genéricos com a vista ativa fica registrado para auditoria própria;
não usar a fixture anterior para mudar o produto.

### Grupos uniformes V19 de I2

v19-prescription-attempt/architecture-delta.json registra quatro notas novas
no visitante, e não só a de terminais prevista inicialmente. A previsão de
contagem estava incompleta; não é mudança do código ou falha funcional. Os
quatro grupos já correspondem aos campos/contratos prescritos acima:

- Source/ShowRecipe/Located/Realized (4): unreachable no match de filhos, pois
  os quatro if-let de transparência retornam antes. Validar o despacho dos
  quatro, não executar esse corpo impossível para fabricar cobertura.
- Sequence/MathSequence (2): mesmo Arc<[Content]>, filhos em ordem.
- Par/SmallCaps (2): mesmo Box<Content>, somente body.
- Os 38 terminais listados acima: nenhuma descida estática nesta vista.

Conservar as quatro notas, sem supressão ou cópia de corpos para esconder
V19. O coletor antigo também tinha uma nota ContextBlock/Dynamic que some
quando sua tabela parcial é removida. Diferencial esperado: um V16 removido,
um V19 antigo removido e quatro V19 novos (saldo +3), V20 intacto. A execução
de um grupo não prova cada alternativa. I1 cobre carriers, a sonda pública
cobre Sequence/Par e controles estruturais adicionais cobrem MathSequence/
SmallCaps, sem alegar que o walker emita eventos dentro de todo math.


## Pilha da reconstrução pré-ordem — 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
before.json.gz e stress-candidate.json.gz em
lab/results/construction/diagnostics/state-stack-2026-10-17 conservam fonte/UTC/diffstat,
argumentos, limites e hashes dos binários. GDB debug-root.log encontra frame
map_content_pre repetido com distância de 104664 bytes entre níveis;
traversal.rs:670–1440 mantém locais de muitas reconstruções no mesmo frame.
64 Strong reais continuam abortando no candidato que já corrige a concatenação;
vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3 aceita e produz X. Assim corrigir
sequências artificiais é necessário, mas insuficiente para o caso de profundidade
legítima. Não inferir resultado do release Tekt a partir do debug.

### Responsabilidade exclusiva e preservação

Isolar a reconstrução de cada braço recursivo de map_content_pre em frame próprio,
por helper privado estático que recebe FnOnce tipada e retorna SourceResult<Content>.
#[inline(never)] impede reintegrar todos os locais no frame do dispatcher. Não
usar dyn/vtable, cache global, threads, stacker, unsafe ou nova configuração de
pilha. A enumeração nominal/exaustiva, os filhos e suas ordens permanecem idênticos.
Não trocar por visita rasa, podar dados legítimos, achatar wrappers ou alterar a
vista de TableCell nesta intervenção.

Source continua transparente sem callback extra. Callback ocorre antes dos filhos
uma vez por nó; Some substitui terminalmente, None recursa; erros interrompem
os mesmos irmãos posteriores. As reconstruções existentes preservam dados,
spans, snapshots, slots e vistas preparadas, inclusive suas limitações documentadas.
O helper apenas delimita armazenamento temporário nativo; não transforma conteúdo
por conta própria. Métodos públicos e map_content/map_text/static visitor ficam
intactos. Fluxo contínuo ADR-0127: nenhuma API/trait/estrutura pública, fase ou
default de produto novo.

Teste local RED antes da implementação com RUST_MIN_STACK=2097152 somente no
processo de teste: percorrer e reconstruir 64 Strong, conferir ordem/preordem e
substituição/erro terminal. A configuração produtiva não muda. GREEN do mesmo
controle e suíte existente; CLI original 64×2048 e Strong64 na pilha padrão,
GDB pós-fix e RAM/RSS medidos. Gates workspace/build/Clippy/Rustdoc estritos,
V1/V5/V15/V26/reversa, sem novos erros Tekt, fmt/diff e fontes congeladas sob
12GiB/High8GiB/swap0/jobs1. Não afirmar profundidade ilimitada: outras travessias
recursivas, destruction/eq/render e custo de nós reais exigem medição própria.


### Continuação medida: frames do mapper bottom-up

stress-expanded.json.gz registra Strong256: vanilla rejeita com maximum show
rule depth exceeded; Tekt aborta. expanded-trace.log localiza agora map_content_inner,
via StrongElem::map_content_if_changed e wrap_page_columns. A passagem pré-ordem
já deixou o programa avançar; resta o mesmo problema de locais compartilhados
na passagem bottom-up. Não classificar o caso negativo upstream como sucesso
obrigatório nem copiar seu limite numérico sem decisão de produto. O contrato
Tekt de profundidade vigente não é alterado: exigir término sem SIGABRT e
registrar separadamente valor/diagnóstico que a política atual produzir.

Aplicar o mesmo isolamento estático a cada braço recursivo de map_content_inner,
com FnOnce privada retornando SourceResult<Option<Content>>. Preservar exatamente
a prova None de ausência de mudança, Some inclusive igual, ordem bottom-up,
transform_root e supressão da raiz de Located, Source/Realized/ShowRecipe e
delegação Dynamic. Não antecipar ou repetir transform, não reconstruir alias
sem mudança, não alterar children selecionados nem usar igualdade como prova.
O pipeline, seus limites existentes e map_text/static visitor conservam contratos.

RED local com 128 Strong e pilha de teste 2MiB, antes do código: None deve
reusar a raiz; substituição igual da folha deve reconstruir o caminho; conferir
ordem e o mesmo conteúdo. GREEN nos mesmos parâmetros, regressões existentes e
CLI Strong256; o caso upstream negativo fica distinguido dos seis positivos.
Mesmo fluxo contínuo ADR-0127 e gates/RAM acima, sem API pública ou default novo.


### Resultado observado sem alteração de política

stress-both-mappers.json.gz: seis controles positivos concluem com a pilha padrão;
Strong256 também produz X no Tekt, enquanto vanilla rejeita pelo limite de show.
Os mappers não são donos de limites de show/layout e não introduzirão uma quota
para simular o aborto ou copiar a contagem upstream. A fonte 01_core/src/entities/world_types.rs permanece com seus guardas próprios.
Seu owner canônico é 00_nucleo/prompts/core/entities/world_types.md; a leitura desse L0 não
encontrou a especificação de Route/guardas. Registrar essa lacuna documental
para auditoria própria, sem legitimar limites a partir do código neste mapper.
ADR-0041 registra o teto de show 64; não se encontrou nesta leitura contrato
completo para a divergência de Strong256.
Este caso prova ausência de aborto neste recorte e registra uma divergência de
limite operacional; não é certificado como paridade de rejeição nem segurança
para profundidade ilimitada.
