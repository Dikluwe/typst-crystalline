# L0 — stdlib/structural/table_grid
Hash do Código: 4974214f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0


Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/table_grid.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/table_grid.rs:50`, `01_core/src/compiler/stdlib/structural/table_grid.rs:302`, `01_core/src/compiler/stdlib/structural/table_grid.rs:337` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own table/grid container, cell and header/footer argument parsing and carriers, including the local usize/align/inset/bool extractors. Preserve paired table/grid validation except the explicitly specified semantic table-header level exception. Cells preserve x/y/colspan/rowspan and placement carriers; table/grid geometry resolution belongs to layout. Auto or None becomes None for optional usize placement; Align is direct or parsed from Str with Align2D::from_string; uniform Length inset becomes Sides<Length>; Bool retains the requested default. Preserve fill:none and stroke:none on their supported container/cell routes; zero-thickness remains rejected rather than PDF hairline.

Headers/footers preserve repeat=true default and explicit false. table.header accepts strictly positive integer level (default 1) and cells: raw content becomes Header Column-scope TableCell, an Auto cell is promoted likewise, and explicit Header/Data classification is retained. table combines distinct levels into one logical group in order and rejects duplicate levels, preserving the default-two-header diagnostic. This semantic PDF exception does not authorize level on grid.header. Keep geometry/repeat compatibility.

Reuse default_hline_stroke from `00_nucleo/prompts/compiler/stdlib/structural/table_lines.md`; do not duplicate line construction/rendering. Keep the table/grid symmetrical surface except for the documented header semantics, with no I/O or dynamic dispatch.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

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

Somente native_table_cell captura presence depois de validar o argumento no seu value_span. Corpo mantém o cast atual. Aceitar auto apenas nos campos medidos; reject none fora de fill/stroke e auto em spans/stroke com mensagens e spans pinados. Não modificar extratores compartilhados de grid/container para obter isso. Fill/stroke none explícitos assentam presença; argumentos omitidos deixam Omitted/false. Stroke Length/Color conserva os componentes explicitamente fornecidos ao projetar repr. Header/footer promovem kind preservando presence. Não implementar ainda relative inset/dict/stroke por lados/paint ricos fora dos carriers existentes; esses casos continuam dívida declarada.

### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Contrato público por campo/tipos novos confirmado pelo humano em 2026-10-07; recibo table-cell-presenca-aprovacao-2026-10-07.json. Não altera fase/defaults de render.

### Refinamento medido de casts e limites — D08B2

Antes da decisão, cast-refuters.json (com pin/HEAD/working tree/binários/UTC)
mede x:-1 → number must be at least zero, spans zero/negativos → number must
be positive e x:float → expected integer or auto. Ancorar estes erros no
value_span; corrigir apenas table.cell, sem alterar o extrator de grid.
As cláusulas D08B2 supersedem as anteriores sobre equivalência none/omissão
e aceitação de auto/none em spans somente para table.cell.
Strokes Length/Color assentam somente componentes fornecidos, como a coleta
raw-options exige, sem mudar o extrator global.
Cast-refuters também mede vanilla aceitando stroke:0pt e rejeitando
align:string/inset:integer. Preservar por ora os caminhos legados Tekt desses
casos: não certificar sua paridade nem corrigir compatibilidade adicional
neste pacote. Inset relativo/dict e strokes/paint ricos seguem fora do escopo.

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

Native Table/Cell constrói valores crus, source_span=Args.span e resolved=None. Table registra rows_explicit pelo argumento, mesmo quando tracks normaliza para [Auto]. Constructors/header/footer preservam source/lexical presence. Assentar os componentes realmente fornecidos no stroke do container também, sem mexer em extrator global. Resolução/ocupação/defaults derivados/execução não pertencem a este parser. Casts e fronteira rica fora de escopo permanecem.

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

Construtores/casts novos inicializam standalone=None e guards vazios. Clones de Cell existente preservam os dois carriers nominais, source_span e presence. Não assar defaults da preparação no constructor, aceitar casts extras ou alterar assinaturas públicas e erros legados.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.

## D08B3-C — precisão de mensagens dos casts existentes

Medição antes da decisão: debug-first.json.gz / invalid-fill (HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC/argv/fontes/hashes completos) contém duas conjunções or onde vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8 usa “color, gradient, tiling, none, or auto”. refuters-before.json.gz / ctor-empty-inset-dict compara rejeição array no constructor: vanilla expected relative length, dictionary, or auto, found array; Tekt texto legado em português. Fonte Tekt table_grid.rs:475 e :491 delega a extrator legado :370; vanilla model/table.rs:751 e :759 possui casts desses campos. Erro é observável da linguagem, não mecânica. Decisão: corrigir somente a enumeração textual de fill e a mensagem de incompatibilidade do inset de table.cell, preservando value_span e casts/aceitações legadas; não modificar o extrator compartilhado de Grid, não ampliar rich casts. Fluxo contínuo ADR-0127, L0 primeiro, RED→GREEN, resselo restrito e revalidação. Nenhum campo/tipo/método/assinatura público ou fase/default/compatibilidade novo.

### Fold de declarações de Stroke — medição extra C

extra-first.json.gz (fontes/HEAD/estado/UTC/binários completos) compara two-stroke-sets: set stroke:red seguido de 2pt produz 2pt + red no getter avulso e posicionado vanilla. stroke-render-inherits-cell-style resolve o constructor 2pt com paint red da Cell antes do fallback da Table 3pt, embora stroke-constructor-fold avulso exponha somente o constructor 2pt. Fonte pinada model/table.rs:765 declara #[fold]; resolve.rs:225–237 usa stroke.resolve(styles), distinto do getter cru. Decisão: declarações sucessivas foldam componentes specified ao publicar Value::Stroke declarativo; nenhum carrier/callback é guardado em Styles. Constructor stroke permanece inteiro no getter avulso; resolução posicionada folda o constructor com a base capturada Cell e depois Table. Adicionar somente campo privado stroke_inherited: Option<Stroke> em TableCellProperties para transportar essa base causal, sem assá-la no getter lexical. ObservationEq inclui base; canonicalização ignora base derivada de render. Table já resolvida tem base None. Clone/aliases conservam base anterior; None explícito vence. RED→GREEN antes de código; correção privada contínua ADR-0127, nenhum contrato público/fase/default novo. A cláusula de precedência inteira do constructor vale para o getter avulso, não para a resolução fold de stroke, conforme medição.


## D08B3-C2 — candidato não vigente antes do gate ADR-0127

### Medição anterior à decisão

`00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz` registra oito sondas completas: fontes, comandos, UTC, HEAD, working tree/diffstat, hashes e binários. Pin 9f2b6e8715237cb086899a42873660fe744622e8. Executável Tekt congelado do C validado; `source-verification.json` confirma hashes produtivos/L0 iguais ao snapshot tested-final do C. O target da raiz não estava disponível nesta retomada; não foi fingido um rebuild.

bare/alias/spread-conflict: mensagens e hints iguais, caret/trace divergentes. No alias o vanilla usa a definição [a] (linha 1), não a chamada (linha 3). No spread usa [a], não o spread inteiro. local-body publica sequence com metadata+Cell preparada no Tekt, styled contendo Cell crua no vanilla; cores/efeitos coincidem no fixture. plain-func coincide cell; set-func/show-func/set-show-func medem cell/sequence contra styled. A primeira rodada `before` tinha str(function), foi invalidada para decidir morfologia; somente baseline usa repr(function).

Fonte: vanilla `typst-eval/src/code.rs:148` aplica Value.spanned; `typst-library/src/foundations/value.rs:210–216` só assenta Content/Func; `foundations/content/mod.rs:109–114` preserva span já assentado. `model/table.rs:436–437` transporta value.span ao fabricar Cell. `typst-eval/src/code.rs:26–57` conserva set/show como styled sobre a cauda. Tekt `eval/expression_dispatch.rs:125–136` realiza Cell no join; `stdlib/structural/table_grid.rs:160–175` guarda children sem origem; `realize/table_cells.rs:176–183` fabrica Cell com span detached; `layout/grid/row_groups.rs:109` usa esse span. É língua (diagnóstico/morfologia/efeitos), não igualdade Rust ou bytes PDF. Não inferir intenção upstream.

### Responsabilidade exclusiva candidata

Preservar Content::Source nos children e tratar wrapper transparentemente para reconhecimento de Cell/Header/Footer/linhas; não fabricar Cell nova quando o body já é Cell explícita; não substituir origem de valor por ocorrência do argumento.

A especificação detalhada e limites estão no contrato candidato do pacote C2. Antes de materializar, instalar este conteúdo no caminho canônico do owner, auditar seus núcleos e selar hashes somente depois de RED→GREEN. A fonte vigente permanece intacta até aprovação.

A cláusula C2 supersede a realização imediata da Cell no join descrita pelo refinamento A: capturar valor lexical e realizar somente na inserção efetiva. Essa mudança de fronteira/ordem também integra o gate ADR-0127 §2.3; preservar a ordem Table→placement→Cell→body do núcleo vigente.

Ratificação C2: dono confirmou `confirmo` em 2026-10-07; contrato Source e fronteira causal aprovados.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T267: 01_core/src/compiler/stdlib/structural/table_grid.rs:632. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `00_nucleo/diagnosticos/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Unir Some(Int) de table.header level e decidir no corpo o mesmo intervalo 1..=u32::MAX, preservando default, cast, mensagens, span e ordem das células.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.


## Tabelas: exaustividade, origem e argumentos — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC/status/git diff HEAD --stat, fontes/L0/configuração e hashes em `00_nucleo/diagnosticos/tekt-v16-table-cells-2026-10-17/before.json.gz` (snapshot inicial de dois owners preservado). contexts.json identifica dez V16 e as projeções latentes, antes do código. public-before.json usa APIs públicas reais com rlib/probe/binário/SHA/argv: Source(Cell) perde x/colspan/rowspan, kind e span no header; footer perde Source; níveis distintos são rejeitados somente quando seus membros levam Source; grupo de footer Source muda células a/b para ab/vazio. snapshot-before.json mede context_floor 77→0 sob Source nas duas restaurações. cli-before.json/cli-extra-before.json/cli-controls-before.json registram sondas bilaterais com binários SHA completos no pin 9f2b6e8715237cb086899a42873660fe744622e8. Footer com um argumento [a#strong[b]] gera duas células a/b no Tekt contra ab/vazio no vanilla; show table:none é indevidamente rejeitado. Controles [] ocultam header/footer sem executar panic de Cell; erros de colspan com alias conservam mensagem/caret. Header/footer colspan e grupos de nível distinto também expõem divergências de linhas, que este lote não dará por resolvidas. É semântica/morfologia/origem/erro, não obrigação de igualar estrutura Rust, bytes ou inferir intenção upstream.

### Prescrição exclusiva

native_table_header classifica content.source_body() para clonar Cell existente. Calcula a origem antes de mover o valor; origem de Cell já assentada não é substituída; detached recebe somente a origem comprovada do Source. Corpo raw usa origem do valor, ou args.span se não há origem do valor, ao criar a Cell. Preservar Source no resultado. Kind Auto é promovido, Header/Data ficam intactos por match nominal raso. table_header_levels coleta sobre source_body; o teste de kind Header vira match nominal Header versus Auto/Data.

native_table_footer converte cada argumento Content/Str em uma única Cell crua, exceto Cell explícita que é preservada através de Source. O corpo original permanece inteiro, mesmo Sequence; não achatar o argumento em células. Reutilizar Content::table_cell para defaults crus e assentar só source_span causal; preservar a origem Source no resultado. Não aceitar tipos novos, preparar propriedades ou executar show. Esta é a conversão TableItem::Cell da superfície medida em lab/typst-original/crates/typst-library/src/model/table.rs:395–442; não copia sua mecânica. Linhas/casts ricos de grupos continuam fora da certificação.

- **C10** — `01_core/src/compiler/stdlib/structural/table_grid.rs:234`, `native_table`: Só Length/Color reconstruem specified uniforme; os outros strokes válidos preservam seus componentes e erros continuam delegados ao cast existente. Excluir somente `Length, Color`; guardados no complemento `(nenhum)`.

- **C11** — `01_core/src/compiler/stdlib/structural/table_grid.rs:498`, `native_table_cell`: Preservar flags specified de stroke e delegação de align/inset ao extrator tipado. None/Auto mantêm seus ramos e value_span. Não ampliar casts ricos legados. Excluir somente `Length, Color`; guardados no complemento `(nenhum)`.

- **C12** — `01_core/src/compiler/stdlib/structural/table_grid.rs:516`, `native_table_cell`: Preservar flags specified de stroke e delegação de align/inset ao extrator tipado. None/Auto mantêm seus ramos e value_span. Não ampliar casts ricos legados. Excluir somente `Auto, None`; guardados no complemento `(nenhum)`.

- **C13** — `01_core/src/compiler/stdlib/structural/table_grid.rs:524`, `native_table_cell`: Preservar flags specified de stroke e delegação de align/inset ao extrator tipado. None/Auto mantêm seus ramos e value_span. Não ampliar casts ricos legados. Excluir somente `Auto, None`; guardados no complemento `(nenhum)`.

- **C14** — `01_core/src/compiler/stdlib/structural/table_grid.rs:728`, `collect`: Só Header/Sequence/Cell fornecem níveis pela representação atual. Os demais conservam nível legado 1. Descascar Source para a coleta, sem declarar resolvida a independência de nível do grupo e kind da célula. Excluir somente `TableHeader, Sequence, TableCell`; guardados no complemento `(nenhum)`.

- **C15** — `01_core/src/compiler/stdlib/structural/table_grid.rs:661`, `native_table_header`: Cell existente deve ser clonada por source_body; os demais conteúdos viram uma Cell crua. Preservar origem de valor, presence, propriedades, kind Data/Header e guards. Auto é promovido. Excluir somente `TableCell`; guardados no complemento `(nenhum)`.

### Aceitação e pendências

L0 antes de testes/código. Fluxo contínuo ADR-0127: correção interna de Source e casts da linguagem, sem campo/tipo/assinatura/trait público, nova flag/default, fase ou compatibilidade externa. Constructor só assenta Cell crua: não chama receita, with_defaults, placement, fontes ou layout. A ordem Table show→defaults/placement→Cell show→body permanece. Grupos V19 ficam justificados por caso e visíveis; não provar cobertura por executar um braço. Não introduzir allow/expect, captura @ ou if-let que esconda a exaustividade nova. Preservar erros/casts legados não alterados, guards que falham, callbacks e restauração em erro/sucesso.

Regressões RED→GREEN nos owners: propriedades/presence/kind/origem de Cell explícita e Source, casts de footer por argumento e não por fragmento, formas cruas não preparadas, groups Source e duas restaurações de snapshot, negativos de tipos/Auto/None e shadow de chave local inválida, None em transformação show sem abrir aceitação de outros valores. Revalidar efeitos ocultados/late-rule pelo CLI e observáveis estruturais do pin; não reduzir testes ou normalizar whitespace para fechar paridade.

Sondas Content/Value/TableCellKind com variantes futuras reais em workspace descartável, cfg(test) compilado: os 18 matches finais devem exigir E0004, contando apenas uma vez cada match. Inputs e erro-testemunha cfg(test) preservados. Cargo target privado, nunca compartilhar artefatos mutantes/medições com o target principal. Gates: build, suíte workspace completa, fmt, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings e quatro índices, V1/V5/V15/V26 estrito e reversa, replay, diferencial SARIF completo e diff-check. V16 esperado 141→131; V19 só dos grupos prescritos; V20 sem crescimento, remover profundidade das duas projeções opcionais. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais. Sem commit/push/merge/release ou pin CI novo.

Incompletude explícita: header/footer spanning e limites entre níveis ainda exigem o pacote table-row-group-contract, com medição/L0 próprios. Nível do grupo e kind/nível semântico da Cell não são o mesmo conceito; não ratificar a inferência atual como contrato completo. Não corrigir ocupação com comprimento de Vec nem usar Source/Styles como marcador de fronteira. Preservar Grid legada. Demais casts/paint/insets ricos e compromissos históricos não são certificados por fechar V16. O objetivo geral, math-stretch-composite e as duas obrigações de matching permanecem abertos.
