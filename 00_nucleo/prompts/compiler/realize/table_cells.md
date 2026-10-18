# L0 — compiler/realize/table_cells
Hash do Código: e8f14bab

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:1d697fae328277525e44cad674cecc537ceb23a1bb3ae11bb68a0a9575411ae4


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/realize/table_cells.rs`

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

Possuir somente a realização de Table e células na inserção. Ordem: Table show-root (via apply owner) → resolução lógica pelo placement owner → Cell show-root (via apply owner) → realização do produto/corpo (via owners existentes). Restaurar contexto causal em todo retorno. Elementos/receitas ocultados não são visitados, inclusive heading original. Tabela substituída não constrói plano. Planos completos são novos por inserção de fonte crua; contribuição já aceita/visual completo é reutilizada sem efeitos extras. Respeitar aliases globais tardios e join local: constructor cru não assa OLD; inserção local do CodeBlock pode produzir LOCAL antes de entrar como body de nova célula sob OUTER. Não distinguir esses casos por nome de variável/AST ad hoc. Resolver herança tipada: x/y/spans do plano, inset/align/fill/stroke e breakable conforme presence/rows; None explícito vence herança. Alignment fold por eixo, Stroke fold por componentes specified; unidades de inset permanecem Length, sem resolver largura física. Table sem rows explícitas/children tem zero células; rows explícitas fixam mínimo, gaps e última linha recebem vazios exceto slots merged. Header/footer inteiros e conflitos são delegados aos owners existentes, que passam a aceitar contexto de tabela. Publicar TableGridResolved com cells row-major, visual de cada célula Complete; delegar sintaxe/selectors/terminação e localização de produtos a seus owners, sem segunda interpretação, vtable, I/O, globals ou FixedMetrics para geometria.

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

### Refinamento interno medido — 2026-10-07

Medição anterior à decisão: 00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/{after-second,refuters-second}.json.gz, pin/binários/HEAD/estado/fontes/UTC/argv/resultados completos.
after-second/show-body difere por espaço e show-bare quebra CELL/a em linhas; tracks.rs mede wrapper sem ramo TableCell no measurement owner. A visualização conserva o snapshot Complete da Cell quando a regra o emite; não o reduz ao body. Assim o renderer distingue default sobrevivente de substituição total sem campo/flag novo. Contexto original sem apply_show_rules não prepara. Snapshot contextual recebido na StyleChain é restaurado antes de realizar Table.
A aceitação é bilateral, efeitos uma vez e geometria; não normalizar whitespace nem alterar contratos ratificados.

### Refutação e decisão interna antes de código — 2026-10-07

Medições em 00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/{after-fourth,refuters-fourth}.json.gz, nested-reuse-red.log e effect-order-red.json: fontes/binários/HEAD/estado/UTC/resultados.
nested-reuse-red.log tem dois REDs válidos: nested executa uma receita em vez de duas, índice de reuse tem zero metadados em vez de dois. Uma Cell recém-preparada em Table aninhada é ocorrência independente: não herda revogação global da regra Cell externa. Restaurar guards em sucesso/erro; remover só guards de seletores Cell ao aplicar a regra de nova Cell preparada. Snapshot RecipeApplied do próprio it permanece terminal, evitando revisitar a própria ocorrência; política α de Cell crua/fresca isolada continua D08B3-B. effect-order-red.json refuta alteração de ordem de execução: SPAN é o primeiro panic nos dois; callbacks permanecem row-major, índice é vista distinta.
Supersede apenas a projeção/guarda aqui medida, sem contrato público ou fase adicional. Refutação: diferença nos testes/recoletas, efeitos repetidos, fonte ocultada visível ou regressão de cache/geometry.

## D08B3-B — candidato, não vigente antes de ratificação

### Medição anterior à decisão

00_nucleo/diagnosticos/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz contém 53 sondas, coletadas em 2026-10-07T14:47:56.340144+00:00 até 14:48:12.375245+00:00. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diff HEAD --stat/status/hashes/fontes/argv/cwd/ambiente e saídas completas registrados. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f. Não misturar before/refuters/complete como rodadas adicionais independentes.

isolated-default: vanilla expõe x/y/inset/align/fill/breakable auto, spans 1 e stroke dict vazio; Tekt omite esses defaults e publica duas metadata contra uma. isolated-x rejeita no Tekt, funciona no vanilla. raw-fields continua só body nos dois e raw-x rejeita nos dois. isolated-table-style confirma que defaults de Table não viram os da Cell avulsa. raw-vs-prepared mede false no vanilla e true no Tekt. nested-effects/body-effects/late-rule refutam execução única e origem lexical. fresh-isolated/table, fresh-block, fresh-styled e multi-fresh são aceitos indevidamente pelo Tekt; vanilla rejeita maximum show rule depth exceeded com span do constructor novo. identity/body/alias e fresh-conditional/hidden continuam válidos. multi-identity e multi-return-table mostram que devolver it deve permitir receita anterior ainda não consumida. isolated-query mantém cell is not locatable nos dois; location é none, sem localização falsa. cell-set é negativo separado, explicitamente excluído deste pacote.

Fonte vanilla lab/typst-original/crates/typst-library/src/model/table.rs:734–778 declara defaults e propriedades; typst-realize/src/lib.rs:367–386 prepara/guarda o argumento, :418–429 visita produto com profundidade, :491–505 distingue guards por elemento, :571–586 materializa propriedades. Tekt entities/elements/table_cell.rs:111–143 só projeta lexical ou plano posicionado; compiler/realize/table_cells.rs:110–129 reaplica Cell crua sem preparo; :280–304 termina snapshot inteiro; eval/show_rule_termination.rs:65–70 trata mesma forma como ponto-fixo; eval/rules/apply.rs:139–166 revoga regra globalmente durante callback. Classificação: campos, efeitos, ordem, terminação e erro causal são língua. A estrutura de guards e materialização upstream é mecânica e não é requisito. Não se infere intenção do upstream; a escolha nominal abaixo é inferência refutável pelos controles de aliases, duas regras, novas gerações e conteúdo ocultado.

### Responsabilidade exclusiva proposta

Possuir preparo de Cell isolada na fachada de inserção existente, antes de selector/receita; preservar body lexical e preparar só produto efetivamente emitido. Estado Complete é terminal, não repete efeito. RecipeApplied conserva guards por regra e ainda admite receitas anteriores não consumidas; não tratar snapshot inteiro como revogado. Célula recém-criada é nova ocorrência sem guards herdados da Cell externa; conservar guards de outros elementos e restaurar ambiente em erro/sucesso. Delegar um passo de receita ao apply; visitar produto sob limite causal existente de show, incluindo wrappers e Table aninhada. Aliases de it não reaplicam a própria regra; duas emissões cruas são duas inserções. Sem show sobrevivente, realizar o corpo uma vez e publicar visual Complete. Planos Table A mantêm posições e herança, incorporando guards nominais das Cells; nenhuma Func no Layouter, nova avaliação da fonte ou mecanismo de Location. Não publicar estados intermediários. O erro de profundidade usa origem causal do conteúdo novo e hints existentes. Não contar replay/cache como geração nova.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.

### Refutadores do binário — preparo direto antes do mapper genérico

Medição anterior à decisão: after-debug.json.gz (binário debug congelado, aborts fresh) e after-second.json.gz (release 761089843eab00316c4af04f0611838df99a521b92fc77bbd409cf8f7dd5aa82, HEAD/árvore/UTC completos) em table-cell-isolada-d08b3b-2026-10-07. O primeiro refuta depender da pilha do mapper genérico para cada geração; o segundo mostra falta de caret em growing e caret do wrapper em block/styled. Selecionar raízes Table/Cell/Styled/ShowRecipe antes de chamar map_content_pre; Sequence/Block/Boxed transparentes preservam campos e visitam body/filhos diretamente. O mapper genérico continua catálogo de descendentes dos demais elementos. Não duplicar corpos nem mover lógica de layout. Erros da aplicação/preparo da Cell preservam span existente; se detached, recebem source_span da Cell nova. O limite causal permanece o existente, sem aumentar pilha/limite ou depender de flags de ambiente.

## D08B3-C — defaults declarativos da Cell, fluxo contínuo

### Medição anterior à decisão

00_nucleo/diagnosticos/table-cell-styles-d08b3c-2026-10-07/before.json.gz registra 58 sondas, UTC 2026-10-07T15:24:58.038947+00:00 até 15:25:13.727872+00:00, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/status/fontes/argv/cwd/ambiente completos. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3. isolated/table-set, campos individuais, late/local/reuse/spread e rejeições refutam o warning de target vazio e a ausência dos defaults. padding-span-one/empty-fixed-rows fixam spans 1 em vazios; auto-table-fallback preserva Auto explícito como delegação à Table, não ao set da Cell. show-cell-set-self altera avulsa mas não fill conhecido da Cell posicionada. Stroke/alignment fazem fold. refuters-before.json.gz estende com header, partial inset, args/condição e snapshot em Table; alias-inside-table é válido no vanilla e recursa indevidamente no Tekt.

Fonte: upstream typst-library/src/model/table.rs:734–778 declara campos, e layout/grid/resolve.rs:203–278 lê estilos antes de resolver/sobrescrever posições/fill; defaults de padding TableCell packed em model/table.rs:785–800 pinam spans 1. Tekt rules/set.rs:23–27 usa text_str de target e não reconhece FieldAccess; table_cells.rs:262–310 usa apenas propriedades lexicais no placement/herança; grid_placement.rs:80 lê x/y/spans diretamente; TableCell standalone não possui propriedades efetivas privadas. É língua (campos/ordem/efeitos/estilo/erros); a representação privada abaixo é decisão Tekt, não obrigação de copiar Packed ou igualdade Rust. Hipótese refutável por qualquer negativo destas coletas, perda de fonte lexical, mutação de valor cru, efeito repetido, guard de alias perdido ou regressão de geometria/cache.

### Responsabilidade exclusiva

Preparar propriedades efetivas a partir de fonte lexical e StyleChain na inserção; preservar snapshots já preparados. Usar dados privados fechados no carrier standalone já existente, com campos tipados equivalentes às nove propriedades e presence; inset de estilo usa Sides<Option<Length>> para omissão parcial. Obter valores de declarações sem chamar Func: valores já foram avaliados no set. Constructor/Auto/None explícitos vencem estilos, stroke/align fold com Table por componente/eixo (constructor explícito vence o estilo Cell inteiro), inset parcial de estilo conserva omissão e faz fold com Table. Para Table, preparar somente argumentos efetivos (children/header/footer) antes de table_groups/placement, usando clone logístico local; fonte original da Table e das Cells não é substituída. Padding gerado recebe spans 1 e estilos das outras propriedades. Propagação lógica usa props privadas; geometria permanece no layout. Cell avulsa recebe show-set de Cell antes da receita/getters; Cell positioned já conhecida não re-resolve propriedades por show-set de si. Aliases de it que entram em Table preservam guards consumidos, novos constructors ficam sem guards. Nenhum corpo ocultado é realizado para buscar defaults. Restaurar ambiente em erro/sucesso.

### Gate, aceitação e limites

Correção interna de paridade/mapeamento, ADR-0127 contínuo: nenhum contrato público, compatibilidade externa, flag/default de produto ou fase nova. Props e helpers são privados em carrier já não-construível externamente; lexicais não mudam. L0 editado primeiro, resselo restrito e RED→GREEN obrigatórios. Coletas bilaterais com erros/caret/efeitos, fonte/binaries/HEAD/UTC, geometria real; build debug/release/suítes e lint sem erros. Preservar árvore anterior sem commit/reset/resselo global. Subconjunto certificado: inteiros/autos, Color/None/Auto, Stroke uniforme/None, alinhamento, Length/inset parcial de Length/Auto, Bool/Auto e schemas já tipados. Gradient/tiling/functions, strokes por lado, inset percentuais e rich casts de constructor, grupos/spans multipágina/PDF-UA, set/text posicional e defaults gerais de Table continuam incompletos, com medição/L0 próprios antes de ampliar. Não certificar por warning fallback nem copiar defeitos upstream.

Precisão medida em refuters-before: stroke-constructor-fold e align-constructor-fold preservam o construtor inteiro contra set da Cell. self-set-does-not-change-placement muda colspan observado para 2 após a ocupação já feita com 1; não é prova de nova ocupação.

### Precisão dos spans observados após show-set (refutador C)

Medição anterior: refuters-before.json.gz / self-set-does-not-change-placement, vanilla devolve colspan 2 na célula real x0 e colspan 1 nos paddings x1/x2. Fonte pinada layout/grid/resolve.rs:218–244 captura spans de ocupação mas não assenta seus campos no conteúdo; model/table.rs:785–800 assenta 1 nos paddings. Logo ocupação e getter posterior são observáveis distintos. Decisão privada: a Cell posicionada conserva TableCellResolved como plano físico; o carrier standalone privado pode coexistir apenas para propriedades observadas da receita, mantendo campos resolvidos e sobrescrevendo spans lexicalmente omitidos pela declaração de show-set. Getters/canonicalização consultam essa projeção; placement do plano completo continua usando TableCellResolved. Padding sem fonte tem spans lexicais 1 para que show-set não os altere. Guards/stage continuam causais, sem novo contrato público nem mutação da fonte original. RED de span getter antes de implementar, GREEN mais geometria/regressões obrigatórios.

### Fold de declarações de Stroke — medição extra C

extra-first.json.gz (fontes/HEAD/estado/UTC/binários completos) compara two-stroke-sets: set stroke:red seguido de 2pt produz 2pt + red no getter avulso e posicionado vanilla. stroke-render-inherits-cell-style resolve o constructor 2pt com paint red da Cell antes do fallback da Table 3pt, embora stroke-constructor-fold avulso exponha somente o constructor 2pt. Fonte pinada model/table.rs:765 declara #[fold]; resolve.rs:225–237 usa stroke.resolve(styles), distinto do getter cru. Decisão: declarações sucessivas foldam componentes specified ao publicar Value::Stroke declarativo; nenhum carrier/callback é guardado em Styles. Constructor stroke permanece inteiro no getter avulso; resolução posicionada folda o constructor com a base capturada Cell e depois Table. Adicionar somente campo privado stroke_inherited: Option<Stroke> em TableCellProperties para transportar essa base causal, sem assá-la no getter lexical. ObservationEq inclui base; canonicalização ignora base derivada de render. Table já resolvida tem base None. Clone/aliases conservam base anterior; None explícito vence. RED→GREEN antes de código; correção privada contínua ADR-0127, nenhum contrato público/fase/default novo. A cláusula de precedência inteira do constructor vale para o getter avulso, não para a resolução fold de stroke, conforme medição.

### Show-set acumulado e bordas — precisão C

show-fold-before.json.gz e geometry-final.json.gz registram perda de red ao acumular show-set e desenho preto 1pt com campos conhecidos red 2pt. Aplicar helper privado puro chain_styles da Cell na acumulação de deltas cell.stroke; delegar componente de desenho ao owner borders, que lê o plano externo conhecido, sem fallback de uma Cell resolvida. Todos os outros estilos/guards/ordem permanecem. Esse helper somente folda Value::Stroke declarativo, nunca carrier/layout/callback; não cria API pública, campo/fase/default novo. RED→GREEN e recoleta real de stroke são obrigatórios.

## D08B3-C — tinta do plano da Table separada do default show da Cell

Medição anterior à decisão: table-cell-styles-d08b3c-2026-10-07/physical-before.json.gz conserva PDF/SVG, fontes, HEAD/working tree, UTC/argv/hashes completos no pin 9f2b6e8715237cb086899a42873660fe744622e8. fill-with-body-recipe e fill-with-hidden-recipe conservam retângulo red no vanilla, ausente Tekt. stroke-with-body-recipe conserva red 2pt vanilla contra black 1pt Tekt; default-with-body-recipe conserva black 1pt nos dois. Fonte vanilla layout/grid/resolve.rs:218–237 e :270–282 retorna Cell.fill/stroke antes de seu body passar por show; typst-layout grid/layouter.rs desenha esses campos independentemente do default show. Tekt cell_body.rs:61–63 condiciona fill à sobrevivência do snapshot, borders.rs:43–48 lê stroke lexical. O refinamento A mediu espaçamento/ab, não tinta: sua generalização de sobrevivência para fill/stroke é refutada e supersedida por esta cláusula.

Decisão contínua interna ADR-0127: TableCell posicionada usa fill/stroke/breakable do plano externo TableCellResolved, independentemente de recipe it/body/substituição; None conhecido vence qualquer fallback de container. Apenas inset/alignment derivados do default show continuam dependentes do snapshot sobrevivente reconhecido por prepared_default, com geometria/medidas sob owners existentes. Borders consome diretamente stroke externo conhecido; Cell body emite o fill externo antes do produto visual. Não executa conteúdo ocultado nem muda fonte lexical, API pública, fases, render default novo ou política Grid/crua. Mantém bounds/coalescência, clipping/caudas, ordem de efeitos e estado causal. RED→GREEN de tinta com produto substituído, células conhecidas None e regressão visual SVG real; builds/suítes/lint/preservação necessários.


## D08B3-C2 — candidato não vigente antes do gate ADR-0127

### Medição anterior à decisão

`00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz` registra oito sondas completas: fontes, comandos, UTC, HEAD, working tree/diffstat, hashes e binários. Pin 9f2b6e8715237cb086899a42873660fe744622e8. Executável Tekt congelado do C validado; `source-verification.json` confirma hashes produtivos/L0 iguais ao snapshot tested-final do C. O target da raiz não estava disponível nesta retomada; não foi fingido um rebuild.

bare/alias/spread-conflict: mensagens e hints iguais, caret/trace divergentes. No alias o vanilla usa a definição [a] (linha 1), não a chamada (linha 3). No spread usa [a], não o spread inteiro. local-body publica sequence com metadata+Cell preparada no Tekt, styled contendo Cell crua no vanilla; cores/efeitos coincidem no fixture. plain-func coincide cell; set-func/show-func/set-show-func medem cell/sequence contra styled. A primeira rodada `before` tinha str(function), foi invalidada para decidir morfologia; somente baseline usa repr(function).

Fonte: vanilla `typst-eval/src/code.rs:148` aplica Value.spanned; `typst-library/src/foundations/value.rs:210–216` só assenta Content/Func; `foundations/content/mod.rs:109–114` preserva span já assentado. `model/table.rs:436–437` transporta value.span ao fabricar Cell. `typst-eval/src/code.rs:26–57` conserva set/show como styled sobre a cauda. Tekt `eval/expression_dispatch.rs:125–136` realiza Cell no join; `stdlib/structural/table_grid.rs:160–175` guarda children sem origem; `realize/table_cells.rs:176–183` fabrica Cell com span detached; `layout/grid/row_groups.rs:109` usa esse span. É língua (diagnóstico/morfologia/efeitos), não igualdade Rust ou bytes PDF. Não inferir intenção upstream.

### Responsabilidade exclusiva candidata

Ao resolver argumento bare, ler span primeiro assentado em SourceContent para a Cell implícita, preservar body lexical original, preparar defaults/placement antes do callback; transparência de Source ao caminhar receita/Cell/Table. Não executar body lexical e visual em paralelo.

A especificação detalhada e limites estão no contrato candidato do pacote C2. Antes de materializar, instalar este conteúdo no caminho canônico do owner, auditar seus núcleos e selar hashes somente depois de RED→GREEN. A fonte vigente permanece intacta até aprovação.

A cláusula C2 supersede a realização imediata da Cell no join descrita pelo refinamento A: capturar valor lexical e realizar somente na inserção efetiva. Essa mudança de fronteira/ordem também integra o gate ADR-0127 §2.3; preservar a ordem Table→placement→Cell→body do núcleo vigente.

Ratificação C2: dono confirmou `confirmo` em 2026-10-07; contrato Source e fronteira causal aprovados.

### Composição de snapshot local C2

additional-debug/nested-local-text refuta substituição exclusiva de rules locais: regra externa atual deve continuar elegível no produto interno. Snapshot marcado eval.table.local=true transporta delta lexical, composto em ordem com rules atuais por RuleId, sem id duplicado. Snapshot completo eval.context.recipe sem marca conserva restauração exclusiva legada. Usar helper privado puro em ambas as restaurações da unidade. Preservar guards nominais, file/styles/context floor e restauração em erro/sucesso.

A captura lexical local usa eval.table.local.recipe dedicado; consultá-lo antes do snapshot geral eval.context.recipe somente nesta unidade. Consumers gerais de contexto não restauram nem executam esse delta. Não alterar comportamento de snapshots completos legados.

### Trace limitado à resolução lógica — refutação final C2

verified-isolated.json.gz/fresh-table acrescenta trace table indevido ao erro de profundidade de show. O trace causal de table cobre somente groups/placement do plano lógico (alias-conflict), antes de callbacks. Erros de show/body preservam seus próprios traces e não ganham chamada Table tardia. Não filtrar mensagens ou apagar traces existentes: separar estruturalmente as fronteiras.


### Traversia Source sem frame semântico — refutação debug C2

complete-depth.json.gz registra fresh-growing-isolated/table com abort por stack overflow em debug; os oito outros controles conservam erro de profundidade. A nova origem adicionou recursão física ao caminhar cada produto, embora não seja elemento semântico. Decisão interna: walk descasca Source antes de walk_node/map_content_pre, preserva primeira origem no retorno sem abrir frame recursivo só para Source. Mantém route/check_show_depth e limite atuais; não aumentar pilha nem diminuir limite. Reexecutar todos os dez controles em debug e release, mais corpus e suítes.

## Reuso do primeiro default quando show-set de Cell não atua — 2026-10-09

### Medição antes da decisão

`00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/{residual-attribution-after-bare-math,default-call-counts-before}.json.gz` registra release SHA-256 `265e97e7ac40bde81caf839ea7ddffbb471fc7c914aebf91f15ab76d74772bbd`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, status/diffstat, UTC, fixtures, comandos e scripts. Em 1.600 células, `local-set` executa 6.400 chamadas a `with_defaults`, 3.200 a `realize_cell` e 4.802 a `native_table_cell`; `no-fill` executa 3.200, 1.600 e 1.601. A avaliação local leva 134,44 ms contra 114,54 ms no controle. Pilhas disjuntas de realização da tabela são 1.758 contra 633. `table_cells.rs:203-218` calcula defaults, testa regras show-set da própria Cell e recalcula os mesmos defaults mesmo quando nenhuma regra Style aplicável alterou a chain. `native_table_cell` em `stdlib/structural/table_grid.rs:394-510` usa argumentos e não muta ctx/world. A inferência de economia será refutada se a contagem nativa, o excesso de tempo ou a saída qualificada não melhorarem.

### Decisão interna e aceitação

Conservar o primeiro produto de `with_defaults` quando nenhum `Transformation::Style` de show rule elegível (guard ausente e selector correspondente ao candidato já preparado) foi aplicado. Se uma ou mais regras de estilo da própria Cell corresponderem, recalcular defaults a partir da Cell lexical sob a chain composta, na mesma ordem vigente, e restaurar a chain inclusive em erro. Não testar igualdade de `StyleChain`/Content para decidir; usar a ocorrência efetiva da regra. Não alterar seleção, guard, propriedades, estágios, origem, chamadas de show body, placement, defaults de Table ou fase do pipeline. É uma remoção de trabalho idêntico no mesmo owner, sem contrato público, flag, default de produto ou compatibilidade nova (ADR-0127 contínuo).

Teste RED→GREEN da escolha de produto sem executar o recálculo quando nenhuma regra casou, e de recálculo quando casou, inclusive erro; os testes de Cell show-set/getters e regras locais existentes preservam a semântica. Gate externo pré-código: em `local-set-1600`, reduzir em pelo menos 1.600 chamadas a `native_table_cell` relativamente às 4.802 anteriores, mantendo `no-fill` sem aumento; medir tempo pareado antigo/novo/vanilla, alocações e 23 fixtures com texto/páginas/caixas/raster/avisos idênticos. Se o excesso local de tempo não cair ou qualquer observável mudar, retirar a intervenção. Suites, build, lint e resselo restrito após qualificação.

### Resultado da tentativa e decisão

Os recibos `default-call-counts-after.json.gz`, `measurements-cell-defaults.json.gz`, `alloc-cell-defaults-{before,after}.json.gz` e `qualification-cell-defaults.json.gz` no diagnóstico acima guardam o estado completo, UTC, fontes e binários. No release candidato SHA-256 `3d52e418a7343ce4a202735ffe72fba1de65ce9d6fc5b310aaac1482547d1241`, `native_table_cell` caiu 4.802→3.202 em `local-set-1600`; `no-fill-1600` permaneceu 1.601. As solicitações glibc locais caíram 607.135→579.935. Na janela pareada antigo/novo/vanilla, a mediana local Tekt caiu 267,54→264,65 ms e o excesso sobre `no-fill` 49,65→47,41 ms; a razão pareada novo/antigo foi −0,7%, negativa nas três rodadas. São ganhos modestos; a variação entre janelas impede afirmar que o número absoluto seja permanente. As 23 fixtures anteriores e dois novos casos de show-set correspondente/não correspondente mantiveram exatamente texto, páginas, caixas, raster e avisos entre releases. Manter a eliminação do recálculo redundante sob os gates finais de suíte, build e lint; não declarar paridade geral de TableCell nem benefício de layout.

## Chaves estáticas dos oito defaults de Cell — 2026-10-09

### Medição antes da decisão

No mesmo owner, `table_cells.rs:162-168` constrói `format!("cell.{key}")` em cada consulta de x/y/colspan/rowspan/fill/stroke/align/breakable. Os nomes são invariáveis; a string temporária existe apenas para lookup. `stacks-after-cell-defaults.json.gz` amostrou uma em 64 solicitações glibc no release SHA-256 `3d52e418a7343ce4a202735ffe72fba1de65ce9d6fc5b310aaac1482547d1241`: pilhas contendo simultaneamente `with_defaults` e `alloc::fmt::format` foram 700 em `local-set-1600`, 400 em `no-fill-1600` e 601 em `block-no-set-1600` (inclusivas, não custo exclusivo). `static-cell-keys-red-gate.json.gz` fixou antes do código o limite de 565.000 pedidos locais; o baseline falhou com 579.935. Ambos os recibos guardam HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado/status/diffstat, UTC, fixtures, comandos, CPU, fontes e hash de binário/script. A inferência de ganho será refutada se alocações não caírem, se o excesso de tempo aumentar ou se a saída mudar.

### Decisão interna e aceitação

Substituir apenas a formação dinâmica das oito chaves por pares literais `(campo, chave)`, na mesma ordem e com o mesmo `StyleChain::custom` top-wins, inserção em Args e `native_table_cell` existente. `cell.inset` continua consultado no ramo separado; não pré-calcular propriedades, não alterar casts/erros, origem, show-set, stage, default de produto, API pública ou fase. Esta é correção mecânica local ao owner (ADR-0127 contínuo), não escolha de nova representação de TableCell.

Teste RED→GREEN no módulo para a projeção integral e ordenada das oito chaves, com uma chave ausente e `cell.inset` fora deste conjunto; gates externos: solicitações `local-set-1600` ≤565.000, redução de excesso de tempo Tekt local versus `no-fill` na janela pareada com o release anterior e vanilla pinado, e 25 fixtures sem diferenças de texto/páginas/caixas/raster/avisos. Se gate de tempo ou saída falhar, retirar o experimento e registrar a refutação; suites/build/lint/resselo restrito após qualificação.

### Resultado medido

`static-cell-keys-green-gate.json.gz` registra 541.535 pedidos locais no release candidato SHA-256 `14d023d36fb765eb041f24f440eaad3bc5d67183325b611e4fd98502bd26815e`, contra 579.935 no RED: menos 38.400 pedidos, passando o limite pré-fixado. `measurements-static-cell-keys.json.gz` mediu antigo/candidato/vanilla na mesma janela 2026-10-09 18:47:58–18:48:36 UTC: local Tekt 256,34→250,17 ms, no-fill 210,63→207,49 ms, excesso 45,71→42,68 ms; razão pareada candidato/antigo no local −1,7%, negativa nas três rodadas, e razão local Tekt/vanilla +47,8%→+44,8%. Os três excessos por rodada também caíram; 400 células ficaram praticamente estáveis. `qualification-static-cell-keys.json.gz` encontrou zero mudança em texto, páginas, caixas, raster e avisos nas 25 fixtures antigo/candidato, incluindo os controles de show-set. São medições de janelas e amostras finitas, não garantia de milissegundos absolutos. Manter a troca sob suítes/build/lint finais, sem estender a transformação a outras chaves dinâmicas.

## Candidato de selector de estilo sob demanda — 2026-10-10

### Medição anterior à decisão

`00_nucleo/diagnosticos/table-cell-candidate-2026-10-10/before.json.gz` e
`allocation-equal-red.json.gz` registram HEAD, árvore não commitada, diffstat, UTC,
hashes, argv e fontes das fixtures. Baseline release SHA-256
584d5c24c243d52c3f80999fe96241c01c98cf69c33eb4f3cb00144d5dc5a1ba.
`table_cells.rs:230-243,246-252` cria Arc de clone da Cell antes de descobrir
se existe Transformation::Style não guardada. Solicitações glibc: local-1600
326348, no-fill-1600 251077, inline-1600 457431. São pedidos, não heap vivo.
Apenas presença inclusiva de clone no perfil não prova custo exclusivo.

### Decisão interna e aceitação

Adiar a construção do candidato até a primeira regra Style com guard ausente,
reutilizando o mesmo snapshot para todas as regras elegíveis dessa passagem.
Cada passagem conserva snapshot próprio, anterior à aplicação de estilos. Não
usar chain alterada para reconstruir candidato, nem antecipar selectors. Ordem,
matching, defaults, body, guards, spans, estágios e restauração permanecem iguais.
Usar armazenamento local, sem cache global, vtable ou alteração pública.
Mudança privada segue ADR-0127 contínuo; enum permanece fechado.

Teste RED→GREEN no módulo: passagem sem regra elegível não cria snapshot,
primeira elegível cria e seguintes reutilizam. Gate de recurso RED: todas as
fixtures selecionadas devem reduzir pedidos contra o baseline; gate linguístico:
controles existentes de show-set correspondente/não correspondente, query, texto,
páginas, caixas, raster e warnings idênticos antigo/candidato. Medir tempo/RSS
pareado sem sonda. Retirar se observáveis mudarem ou regressão persistir.


## Seletores de show — exaustividade e agrupamento, 2026-10-16

### Medição anterior à decisão

01_core/src/compiler/realize/table_cells.rs:34 (captures_rule, emitted-v16). O enum em 01_core/src/entities/show.rs:63-96 contém Text, NodeKind, NativeElement, DynKind, Regex, Where, And, Or e Label. Fonte/linhas e funções atuais foram confrontadas com os avisos SARIF da revisão pinada e com o inventário AST anterior, verificando presença literal dos matches na árvore atual. Proveniência: 00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16/before.json.gz conserva HEAD, working tree não commitado, UTC, hashes/fontes, status e git diff HEAD --stat; classification.json distingue avisos emitidos de predicado latente. Classificação: padrões/exaustividade são mecânica; seleção, texto, efeitos e erro de linguagem permanecem conforme os contratos vigentes. Não há nova medição de paridade ou inferência de ganho de leitura/performance.

### Decisão interna

captures_rule conserva reconhecimento por endereço das nativas table/table.cell, recursão Where/And/Or (any) e seleção exclusiva FuncRepr::Native no ramo interno. Nomear Text/NodeKind/DynKind/Regex/Label como false no match externo de Selector. Não modificar o fallback interno FuncRepr, que pertence a outra família, nem fontes/efeitos/defaults/placement/células/show. Teste S4: nove variantes, aliases table e cell, homônimos table/cell apontando a grid ou heading, Where aninhado, And/Or mistos e vazios, NativeWithEngine rejeitado. Não avaliar filtros nem disparar constructor para decidir captura.

Alternativas com o mesmo resultado conservam um corpo compartilhado. Grupos explícitos informam a política vigente e exigem decisão quando Selector cresce; não duplicar corpos para zerar V19. V19 permanece informativo, sem afirmar déficit de cobertura ou fator de multiplicação de testes. Guard e políticas recursivas merecem controles próprios; uma execução do corpo não prova todas as alternativas. Nenhum allow/expect, exceção, ajuste de severidade ou helper genérico para esconder V16.

### Aceitação e limites

Fluxo contínuo ADR-0127: nenhuma assinatura/campo/trait público, default de produto, compatibilidade ou fase nova. Não reabrir propostas históricas deste owner. L0 antes dos testes/código e resselo restrito bilateral. Escrever os controles no módulo e confirmar o comportamento anterior; o RED estrutural é a ausência de E0004 nos seis pontos na cópia real do core com uma variante sentinela acrescentada apenas ao enum copiado. Após nominalização, os seis pontos devem exigir E0004 nessa cópia, sem stubs nem mutação da fonte produtiva. Os cinco V16 selecionados devem desaparecer da emissão, preservando diagnósticos externos e notas V19. Reexecutar testes reais, build, formato, Clippy estrito, Rustdoc e linhagem direta/reversa sob 12 GiB, swap zero e um job Cargo. Não declarar gate global verde enquanto V16 externos permanecerem.


## Tabelas: exaustividade, origem e argumentos — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC/status/git diff HEAD --stat, fontes/L0/configuração e hashes em `00_nucleo/diagnosticos/tekt-v16-table-cells-2026-10-17/before.json.gz` (snapshot inicial de dois owners preservado). contexts.json identifica dez V16 e as projeções latentes, antes do código. public-before.json usa APIs públicas reais com rlib/probe/binário/SHA/argv: Source(Cell) perde x/colspan/rowspan, kind e span no header; footer perde Source; níveis distintos são rejeitados somente quando seus membros levam Source; grupo de footer Source muda células a/b para ab/vazio. snapshot-before.json mede context_floor 77→0 sob Source nas duas restaurações. cli-before.json/cli-extra-before.json/cli-controls-before.json registram sondas bilaterais com binários SHA completos no pin 9f2b6e8715237cb086899a42873660fe744622e8. Footer com um argumento [a#strong[b]] gera duas células a/b no Tekt contra ab/vazio no vanilla; show table:none é indevidamente rejeitado. Controles [] ocultam header/footer sem executar panic de Cell; erros de colspan com alias conservam mensagem/caret. Header/footer colspan e grupos de nível distinto também expõem divergências de linhas, que este lote não dará por resolvidas. É semântica/morfologia/origem/erro, não obrigação de igualar estrutura Rust, bytes ou inferir intenção upstream.

### Prescrição exclusiva

Consolidar as duas projeções opcionais em helper privado captured_recipe: lookup eval.table.local.recipe antes de eval.context.recipe, ? para ausência, match nominal Value::Content versus demais Value, e match nominal Content::ShowRecipe versus demais Content após source_body. Valor local inválido continua sombreando o geral; não buscar fallback adicional. Clonar o Arc uma vez, conservar captured_rules, file, guards, styles/context floor e sua restauração. As duas projeções antigas aninhadas são removidas.

argument_defaults só preenche source_span detached da Cell existente quando body.Source prova uma origem; não troca uma origem já assentada. group_defaults discrimina Sequence por body.source_body(), prepara os membros com o owner vigente e conserva o Source do grupo no resultado. O clone logístico não é uma segunda emissão. Não modificar o guard interno FuncRepr de captures_rule: o L0 anterior o delimitou à ABI Native, e este lote conserva a decisão.

- **C03** — `01_core/src/compiler/realize/table_cells.rs:54`, `needs_preparation`: Somente Table não resolvida e Cell não Complete levantam needs e interceptam a caminhada. Variantes guardadas também ficam no complemento para quando o guard falha. Excluir somente ``; guardados no complemento `Table, TableCell`.

- **C04** — `01_core/src/compiler/realize/table_cells.rs:239`, `walk_node`: Só as raízes discriminadas interceptam o walker. As outras retornam Ok(None) para o catálogo genérico; Table/Located guardados permanecem no complemento. Excluir somente `Source, ShowRecipe, Styled, TableCell, Sequence, Block, Boxed`; guardados no complemento `Table, Located`.

- **C05** — `01_core/src/compiler/realize/table_cells.rs:573`, `finish_cell_product`: Styled e Cell têm finalização própria; os demais delegam ao walker, com o mesmo contexto e limite causal. Excluir somente `Styled, TableCell`; guardados no complemento `(nenhum)`.

- **C06** — `01_core/src/compiler/realize/table_cells.rs:597`, `table`: Somente raízes/snapshots discriminados interceptam a preparação da tabela. Os demais seguem mapper genérico; casos guardados continuam no complemento. Excluir somente `ShowRecipe, Styled, TableCell`; guardados no complemento `Table, Located`.

- **C07** — `01_core/src/compiler/realize/table_cells.rs:91`, `realize`: Duas restaurações usam o mesmo lookup local-recipe antes de context-recipe. Consolidar projeção emprestada rasa, reconhecendo Source(ShowRecipe), sem reavaliar receita ou mudar ordem/restauração.

- **C08** — `01_core/src/compiler/realize/table_cells.rs:191`, `walk_node`: Duas restaurações usam o mesmo lookup local-recipe antes de context-recipe. Consolidar projeção emprestada rasa, reconhecendo Source(ShowRecipe), sem reavaliar receita ou mudar ordem/restauração.

- **C09** — `01_core/src/compiler/realize/table_cells.rs:368`, `argument_defaults`: Header/Footer e Cell explícita preservam identidade/propriedades. Demais corpos recebem Cell crua implícita, com a primeira origem comprovada. Não preparar conteúdo ocultado. Excluir somente `TableHeader, TableFooter, TableCell`; guardados no complemento `(nenhum)`.

### Aceitação e pendências

L0 antes de testes/código. Fluxo contínuo ADR-0127: correção interna de Source e casts da linguagem, sem campo/tipo/assinatura/trait público, nova flag/default, fase ou compatibilidade externa. Constructor só assenta Cell crua: não chama receita, with_defaults, placement, fontes ou layout. A ordem Table show→defaults/placement→Cell show→body permanece. Grupos V19 ficam justificados por caso e visíveis; não provar cobertura por executar um braço. Não introduzir allow/expect, captura @ ou if-let que esconda a exaustividade nova. Preservar erros/casts legados não alterados, guards que falham, callbacks e restauração em erro/sucesso.

Regressões RED→GREEN nos owners: propriedades/presence/kind/origem de Cell explícita e Source, casts de footer por argumento e não por fragmento, formas cruas não preparadas, groups Source e duas restaurações de snapshot, negativos de tipos/Auto/None e shadow de chave local inválida, None em transformação show sem abrir aceitação de outros valores. Revalidar efeitos ocultados/late-rule pelo CLI e observáveis estruturais do pin; não reduzir testes ou normalizar whitespace para fechar paridade.

Sondas Content/Value/TableCellKind com variantes futuras reais em workspace descartável, cfg(test) compilado: os 18 matches finais devem exigir E0004, contando apenas uma vez cada match. Inputs e erro-testemunha cfg(test) preservados. Cargo target privado, nunca compartilhar artefatos mutantes/medições com o target principal. Gates: build, suíte workspace completa, fmt, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings e quatro índices, V1/V5/V15/V26 estrito e reversa, replay, diferencial SARIF completo e diff-check. V16 esperado 141→131; V19 só dos grupos prescritos; V20 sem crescimento, remover profundidade das duas projeções opcionais. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais. Sem commit/push/merge/release ou pin CI novo.

Incompletude explícita: header/footer spanning e limites entre níveis ainda exigem o pacote table-row-group-contract, com medição/L0 próprios. Nível do grupo e kind/nível semântico da Cell não são o mesmo conceito; não ratificar a inferência atual como contrato completo. Não corrigir ocupação com comprimento de Vec nem usar Source/Styles como marcador de fronteira. Preservar Grid legada. Demais casts/paint/insets ricos e compromissos históricos não são certificados por fechar V16. O objetivo geral, math-stretch-composite e as duas obrigações de matching permanecem abertos.

### R3-T1 — integração revelada pela suíte ampla

O candidato3 abortou d08b3_reused_table_indexes_emitted_metadata. O backtrace
(table-stack-trace.log) mostra table→walk(ShowRecipe)→table reintroduzindo o
produto em cada alias de it após a captura causal dos callbacks. Durante a
visita do produto de Table, proteger a Location original real no contexto
privado já existente e restaurar a pilha mesmo em erro. Um alias protegido
pula só a receita da própria Table e ainda prepara seu plano/Cells por emissão,
preservando dois metadados e Table→placement→Cell→body. Não criar Location.
Packets novos com baseline causal delegam ao coordenador recipes; packets
legados conservam sua política vigente. Não realizar filhos ocultados antes da
raiz. Os testes existentes de reuse/nested/fresh/error são refutadores.

### R3-T1 — refinamento por regra e fronteira

A proteção da Location de Table é por regra: ao entrar no produto conservar
ativos os IDs das receitas Table selecionadas (e show-set aplicado); um alias
protegido filtra somente esses IDs, admitindo regras anteriores/locais. Preparar
Cells independentemente em cada emissão. Restaurar guards e Locations em todo
retorno. A falha de metadata zero do candidato4 é RED desta integração.

### R3-T1 — guards herdados no transporte legado

O backtrace5b confirma a recursão por walk(ShowRecipe)→table: o packet legado
substitui guards ativos por seu snapshot anterior, perdendo a revogação atual.
Ao instalar guards de recipe/contexto/Styled nesta unidade, unir por identidade
os guards capturados aos ativos da inserção e restaurar a lista anterior em
todo retorno. Não alterar captured_rules nem o reset nominal de guards Cell de
uma ocorrência nova. A proteção só vale para o alias da Location já conhecida.

### R3-T1 — integração do fluxo emitido

O candidato6 terminou com metadata zero e quatro tabelas emitidas em vez das
duas emissões esperadas. Retirar a realização bottom-up própria dos packets
ShowRecipe, inclusive legados: delegar sua execução ao coordenador recipes,
que instala/restaura o ambiente e visita somente a raiz/produto emitidos. O
caminho legado reaplicava apply_show_rules ao corpo já visitado, reintroduzindo
o resultado cacheado da Table. Os snapshots de contexto/Styled e o preparo
Table/Cell continuam nesta unidade, com guards herdados unidos por identidade.


## Context — retirada do filtro restritivo, 2026-10-18

Medição anterior: before.json.gz em 00_nucleo/diagnosticos/context-paridade-2026-10-18,
HEAD b7e7c83223e5daa67bbdf124c36b5b7f93200731, vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3,
com fontes, UTC, argv e identidades. Global/local textual retorna A/A contra R/L.
O único caller produtivo de captures_rule era capture_context_recipe no hub.
O filtro exclusivo Equation/Table impede transporte de regras válidas e o novo
transporte lexical geral deixa este predicado sem uso (green-text1.log).

Decisão interna ADR-0127: superseder somente o limite histórico de captura
contextual e as cláusulas de manutenção deste filtro. Remover captures_rule e seu
teste unitário que fixa a whitelist obsoleta; os testes funcionais Context do
owner 00_nucleo/prompts/infra/pipeline/tests.md substituem esse critério de
captura. Não alterar reconhecimento semântico dos seletores, matching, catálogo
Mathy, preparo Table/Cell, aliases, wrappers ou interfaces públicas. Captura
conserva regras; não declara que elas casam qualquer elemento. Nenhum lint é
silenciado e nenhuma expectativa de linguagem é reduzida.
