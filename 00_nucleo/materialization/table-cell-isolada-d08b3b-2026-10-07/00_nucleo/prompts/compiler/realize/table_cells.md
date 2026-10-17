# L0 — compiler/realize/table_cells
Hash do Código: 901273e7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

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
