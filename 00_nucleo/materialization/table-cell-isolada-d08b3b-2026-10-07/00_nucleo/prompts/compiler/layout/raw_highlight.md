# L0 — raw highlight
Hash do Código: b6fb63f8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/raw-block-boundary.toml sha256:c983fdb9884f90548e110fe7c1709632fc00b6e2a1387ad9e80444009c3602fd
- 00_nucleo/prompts/_nuclei/layout/raw-defaults.toml sha256:2ce5a50363605eb1a122920b92773b1fc956c52817dd57830c2ce12bfe5f285e

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/raw.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/raw.rs:103`, `01_core/src/compiler/layout/raw.rs:117`, `01_core/src/compiler/layout/raw.rs:20`, `01_core/src/compiler/layout/raw.rs:41`, `01_core/src/compiler/layout/raw.rs:47`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own Raw rendering in `raw.rs`, not a separate renderer or language parser. Use embedded two_face syntax definitions and the existing Typst Light scope theme; only immutable lazy syntax/theme data is shared. Start from the current raw base style (clear bold/italic/fill, preserve the existing remaining fields), apply supported token foreground/bold/italic and restore the outer style after rendering. Block raw closes surrounding lines and starts token lines at the margin. Unknown syntax or highlighting failure uses the existing plain-token path. The current implementation uses `split_whitespace` for token emission, so the older whitespace-preservation claim is unresolved, not proven. Preserve the existing text/whitespace requirement as an open contract discrepancy; no fix, normalization expansion or weakening of the language requirement is authorized here.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

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


### Refutação de fill no primeiro GREEN

green.log (1/2 passou, snapshot tested-state) mostra fill None em Raw contra
vermelho no texto. StyleChain.fill:422 lê somente delta.fill; set/text.rs:57
guarda text.fill no canal custom, que o renderer Text já consome. O helper
de defaults Raw deve consultar text.fill nessa chain antes do fallback
TextStyle.fill, inclusive quando instalar o escopo para produto substituído.
Não corrigir o contrato global StyleChain neste recorte.

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

### Região inicial versus estilo Strong do conteúdo

refuters-second.json.gz mantém Strong-cell0.14302pt acima do vanilla,
mas plain-cell coincide até0.00002pt. raw.begin usa o top-edge do estilo
Strong interior para converter a origem legada; cell_body recompõe com o
ascender/top-edge do exterior da região. Sub-frame deve inicializar seu
line_assumed_ascent pelo estilo próprio na entrada, depois de guardar o
ascent do pai. Raw primeiro usa esse ascent regional para converter o topo,
não o estilo Strong/Emph interior. O helper continua restaurando o pai.
É correção de coordenada/contexto privado na fase existente, sem novo slot,
API ou tipo. Aceitação: o refutador Strong deve conservar a mesma origem e
altura do plain, com recompilação e suites/regressões.

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

O predicado terminal_block inspeciona exclusivamente visual preparado da Cell avulsa e positioned. Preservar todas as fórmulas de margens, fontes e regiões do reparo Raw atual; nenhum preparo/Engine no layout.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.
