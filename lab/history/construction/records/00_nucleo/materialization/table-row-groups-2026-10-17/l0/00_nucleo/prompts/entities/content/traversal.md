# L0 — entities/content/traversal
Hash do Código: bde1a1c7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:16ae092d990d3d26f03232afcabf63f3b5c08ffb5b6297989368f0346d14f2ab
- 00_nucleo/prompts/_nuclei/table/row-groups.toml sha256:472e9e168c439ce5e1d17557fd8dbb55047e2f3c2f259eada2ff8b763ae27ade


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

00_nucleo/diagnosticos/table-cell-realizacao-2026-10-07/before.json.gz e
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

00_nucleo/diagnosticos/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz contém 53 sondas, coletadas em 2026-10-07T14:47:56.340144+00:00 até 14:48:12.375245+00:00. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diff HEAD --stat/status/hashes/fontes/argv/cwd/ambiente e saídas completas registrados. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f. Não misturar before/refuters/complete como rodadas adicionais independentes.

isolated-default: vanilla expõe x/y/inset/align/fill/breakable auto, spans 1 e stroke dict vazio; Tekt omite esses defaults e publica duas metadata contra uma. isolated-x rejeita no Tekt, funciona no vanilla. raw-fields continua só body nos dois e raw-x rejeita nos dois. isolated-table-style confirma que defaults de Table não viram os da Cell avulsa. raw-vs-prepared mede false no vanilla e true no Tekt. nested-effects/body-effects/late-rule refutam execução única e origem lexical. fresh-isolated/table, fresh-block, fresh-styled e multi-fresh são aceitos indevidamente pelo Tekt; vanilla rejeita maximum show rule depth exceeded com span do constructor novo. identity/body/alias e fresh-conditional/hidden continuam válidos. multi-identity e multi-return-table mostram que devolver it deve permitir receita anterior ainda não consumida. isolated-query mantém cell is not locatable nos dois; location é none, sem localização falsa. cell-set é negativo separado, explicitamente excluído deste pacote.

Fonte vanilla lab/typst-original/crates/typst-library/src/model/table.rs:734–778 declara defaults e propriedades; typst-realize/src/lib.rs:367–386 prepara/guarda o argumento, :418–429 visita produto com profundidade, :491–505 distingue guards por elemento, :571–586 materializa propriedades. Tekt entities/elements/table_cell.rs:111–143 só projeta lexical ou plano posicionado; compiler/realize/table_cells.rs:110–129 reaplica Cell crua sem preparo; :280–304 termina snapshot inteiro; eval/show_rule_termination.rs:65–70 trata mesma forma como ponto-fixo; eval/rules/apply.rs:139–166 revoga regra globalmente durante callback. Classificação: campos, efeitos, ordem, terminação e erro causal são língua. A estrutura de guards e materialização upstream é mecânica e não é requisito. Não se infere intenção do upstream; a escolha nominal abaixo é inferência refutável pelos controles de aliases, duas regras, novas gerações e conteúdo ocultado.

### Responsabilidade exclusiva proposta

Preservar standalone, stage e guards em reconstruções; mapas/preorder visitam visual preparado exclusivo, conservando fonte lexical. Cru segue body. Nenhum double-walk do body escondido, preparo ou receita em entidades.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Travessia Source preserva span e visita somente body, não acrescenta callback para wrapper. map_content_pre e map_text seguem mesma regra sem emitir origem separada.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Reconstrução somente após mudança — 2026-10-10

Medição anterior à decisão: `00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/change-aware-red-gate.json.gz` registra HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado/status/diffstat, UTC, script/fixtures/interpositor/CPU/comandos e release SHA-256 `29745a9e80ca43160b34f4807949f00e803070392653df0fc5e214b2f42ac33a`: 483.823 pedidos glibc em local-set-1600, acima do gate pré-fixado de 470.000. `entities/content/traversal.rs:25-55,235-264` reconstrói sequências/carriers/estilos/Table/Cell mesmo se todos os callbacks retornam None; `entities/elements/table.rs:85-118` e `entities/elements/table_cell.rs:308-344` clonam metadados e contribuições visuais. O vanilla pinado visita referências e materializa apenas após mudança (`lab/typst-original/crates/typst-realize/src/lib.rs:242-291,353-410`). Isso é uma hipótese de custo, sem exigir sua vtable ou igualdade Rust (ADR-0107/0026/0109).

Decisão interna de fluxo contínuo (ADR-0127): manter a assinatura pública e o match exaustivo de map_content; propagar internamente Option<Content>, onde None comprova ausência de reconstrução e Some transporta o resultado materializado. Some do callback sempre é uma substituição, mesmo quando compara igual; não usar PartialEq, endereço ou hash para inferir mudança, nem executar callbacks duas vezes. Cada callback recebe o nó após os filhos, na mesma ordem, e erros interrompem a descida como antes; substituições não são reprocessadas.

As primeiras variantes que propagam a prova são Source, Located, Realized, ShowRecipe, Sequence, MathSequence, Styled, Par, SmallCaps, Table, TableCell e terminais já classificados pelo match. Source conserva a normalização de with_source_span, inclusive origem aninhada e span detached; Located mantém a supressão de callback no body imediato; Realized transforma somente visual. Variantes ainda não migradas usam a delegação existente e retornam conservadoramente Some. A lógica específica de Table e Cell permanece nos respectivos owners `00_nucleo/prompts/entities/elements/table.md` e `00_nucleo/prompts/entities/elements/table_cell.md`.

Helpers exclusivamente crate-private permitem map_content_if_changed e mapeamento de slices sem alocar até o primeiro filho alterado; então clonar apenas o prefixo e irmãos preservados para o resultado. Nenhum campo/método de trait/assinatura pública/fase muda. Aceitação: RED→GREEN de reuso de pais/irmãos e alocação; callback bottom-up uma vez, substituição igual explícita, erro sem visitar irmãos posteriores, Source/Located/Realized e vistas preparadas; tempo pareado e qualificação de linguagem/render/query contra o release anterior e vanilla pinado. O recorte não promete reuso de todos os contêineres, map_text ou map_content_pre.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `00_nucleo/diagnosticos/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/content/traversal.rs:55` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

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

T283: 01_core/src/entities/content/traversal.rs:83. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `00_nucleo/diagnosticos/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Unir braços None do mapeamento Source; decidir no corpo a mesma conjunção de spans. Manter None sem clone quando basta preservar origem; demais casos mantêm with_source_span, ordem e quantidade de callbacks. Sem reconstrução eager, igualdade ou visita adicional.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.


## RG1 — grupos Table ordenados e síntese estrutural — candidato 2026-10-17

### Medição anterior à decisão

Medição anterior: 00_nucleo/diagnosticos/table-row-groups-2026-10-17/before.json.gz, measurement.json.gz, supplement.json.gz, lines.json.gz, styles-phase.json.gz e svg.json.gz conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC/status/diffstat/fontes/binários/argv/cwd. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. table_grid.rs:126–150 funde grupos; row_groups.rs:22–35/70/355–366 conta entradas, table_cells.rs:1204 aplica receita antes do planner e :1259–1282 deriva prefixos. model/table.rs:307–316 sintetiza; typst-realize/src/lib.rs:367–385/558–568 sintetiza antes de receita; layout/grid/resolve.rs:1380–1457/1597–1650 preserva intervalos e cursores; typst-html/src/rules.rs:591–665 projeta papéis. TableHeader level/presença somem no Tekt, coordenadas de linha não são lidas em table_lines.rs:173–260, e supressão None em layout/grid/rules.rs:40–46 deixa traços. svg-analysis.json confere todos os strokes retos e mudanças pareadas (mesmo binário): top vermelho vanilla 82→41, Tekt 82→82; azul após None vanilla 41→0, Tekt 40→40. Estes são efeitos/células/roles/erros/geometria da língua; não obrigação de copiar vtables, estruturas Rust, bytes SVG/PDF ou inferir intenção upstream. O primeiro extrator SVG não reconhecia line/h/v; foi descartado como prova, conservado e substituído por extração completa registrada.

### Responsabilidade exclusiva

Migrar reconstruções literais Table/Header/Footer/linhas e fixtures aos campos aprovados. Map pré-ordem e público mantêm ordem e fronteira terminal, preservam nível/máscaras/Source. Somente Table com is_complete() visita visual; plano estrutural pending não é um segundo visual emitido. Não visitar simultaneamente filhos crus e células completas. Fonte lexical ordenada é preservada ao transformar visual; pending/raw conserva a travessia lexical. Nenhuma assinatura pública nova.

### Gate, aceitação e incompletude

Contrato candidato RG1: não vigente antes da confirmação humana do manifesto em 00_nucleo/materialization/table-row-groups-2026-10-17/review.md. A aprovação precisa abranger contratos públicos, compatibilidade e fase (ADR-0127 §2.1/§2.3/§2.4). Após aprovação, esta seção supersede apenas cláusulas históricas conflitantes de grupos Table/linhas e da fronteira estrutural/receitas. Outras propostas não são ratificadas. Não implementar campo/API novo antes do gate. Instalar L0/kernel/pins primeiro, testes funcionais RED antes do código, sem tratar erro de compilação/setup como RED; GREEN mais R1–R20 e corpus bilateral, build, workspace completo, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, formato, V1/V5/V15/V26 e reversa, diferencial SARIF/replay/diff-check. V16 externos continuam abertos; não alterar severity, criar allow/expect ou esconder um complemento em if-let só para zerar lint. V19 tem justificativa local e corpo compartilhado quando a política é a mesma. Núcleos são compartilhamento normativo; cada L0 conserva exatamente um consumer. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais; sondas/mutações usam target Cargo isolado e nenhuma variante mutante vai para target principal. Sem commit/push/merge/release ou pin novo.
