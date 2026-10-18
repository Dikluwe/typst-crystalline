# L0 — compiler/eval/rules/set — Avaliação de set rules
Hash do Código: 1247c824

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:2b8888460a1e6008abda0ff71603fe4dace79ff0a2a7aad598f8aabfab3e3776
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/layout/raw-block-boundary.toml sha256:c983fdb9884f90548e110fe7c1709632fc00b6e2a1387ad9e80444009c3602fd
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:7e41c96527dd5936bebe6916f2a9ea93477c057495845c7e8b52ce3c5917855a
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:e9038b453bbb1971c9a75a76eced99c35db742f57bebbb09b03c4922b3862800
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:32890de1247262b54fa39d7ae1d640a068774a28cf1fa1c8890d0508d999af51
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:1d697fae328277525e44cad674cecc537ceb23a1bb3ae11bb68a0a9575411ae4

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/set.rs`

## Medição antes da decisão

A working tree não commitada capturada em `00_nucleo/diagnosticos/atomizacao-eval-rules-2026-09-30-before.json.gz` contém as rotinas desta família em `01_core/src/compiler/eval/rules.rs`. A suíte L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem prova bilateral nova contra o vanilla. As medidas históricas e ressalvas do pin continuam em `00_nucleo/prompts/compiler/eval/rules.md`.

## Contrato de linguagem

Validar targets e argumentos, construir deltas de Styles sem mutação parcial, preservar ordem de avaliação e defaults vigentes. Incluir page, heading, par, text, format e demais schemas descritos no L0 pai. Feature gate HTML precede avaliação quando desativado.

As assinaturas, identidades, casts, defaults, precedência de erros e fase de avaliação observados permanecem. O parent mantém diagnóstico compartilhado e bindings estáticos; outros owners de selector matching, terminação e textual realization continuam exclusivos. Não duplicar esses algoritmos.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. Paridade em semântica, sintaxe e morfologia (ADR-0107/0108), ownership exclusivo e pins válidos (ADR-0129/0130).

## Delegação de branches P1500

O dispatcher conserva a ordem dos targets. Os braços `page` e `text` pertencem exclusivamente a `rules/set/page.md` e `rules/set/text.md`; este owner conserva os demais targets e o encaminhamento estático. A separação não altera defaults, contrato público nem fase.


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

Adicionar propriedade smartquote.double bool à tabela já existente de set,
com type_mismatch/span vigente e push_custom smartquote.double, como o positivo
compile-set-double-native mede. Manter enabled/alternative/quotes e validação.
Não preencher campos da folha a partir da chain nem alterar outros targets.

## D06 — block herdado, correção interna

Medição before.json.gz block-style/local-block/number-style mostra block herdado
no vanilla. set.rs:583–630 trata numbering e demais propriedades, mas omite
block. Para o target existente math.equation, avaliar argumento block uma vez,
validar booleano e transportar equation.block na chain lexical; omitido conserva
a chain. Não mudar assinatura/default/enum. Usar diagnóstico de tipo existente.
Owner de preparação resolve esse valor para a Equation automática ratificada.
A aceitação inclui block local não vazando, numbering de bloco e valor inválido.

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

## D08B3-C — defaults declarativos da Cell, fluxo contínuo

### Medição anterior à decisão

00_nucleo/diagnosticos/table-cell-styles-d08b3c-2026-10-07/before.json.gz registra 58 sondas, UTC 2026-10-07T15:24:58.038947+00:00 até 15:25:13.727872+00:00, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/status/fontes/argv/cwd/ambiente completos. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3. isolated/table-set, campos individuais, late/local/reuse/spread e rejeições refutam o warning de target vazio e a ausência dos defaults. padding-span-one/empty-fixed-rows fixam spans 1 em vazios; auto-table-fallback preserva Auto explícito como delegação à Table, não ao set da Cell. show-cell-set-self altera avulsa mas não fill conhecido da Cell posicionada. Stroke/alignment fazem fold. refuters-before.json.gz estende com header, partial inset, args/condição e snapshot em Table; alias-inside-table é válido no vanilla e recursa indevidamente no Tekt.

Fonte: upstream typst-library/src/model/table.rs:734–778 declara campos, e layout/grid/resolve.rs:203–278 lê estilos antes de resolver/sobrescrever posições/fill; defaults de padding TableCell packed em model/table.rs:785–800 pinam spans 1. Tekt rules/set.rs:23–27 usa text_str de target e não reconhece FieldAccess; table_cells.rs:262–310 usa apenas propriedades lexicais no placement/herança; grid_placement.rs:80 lê x/y/spans diretamente; TableCell standalone não possui propriedades efetivas privadas. É língua (campos/ordem/efeitos/estilo/erros); a representação privada abaixo é decisão Tekt, não obrigação de copiar Packed ou igualdade Rust. Hipótese refutável por qualquer negativo destas coletas, perda de fonte lexical, mutação de valor cru, efeito repetido, guard de alias perdido ou regressão de geometria/cache.

### Responsabilidade exclusiva

Reconhecer Native table.cell por identidade através de Ident/FieldAccess puro e aliases; reutilizar eval_args para avaliação em ordem e spreads. Condição falsa não avalia argumentos. Validar esquema nomeado x/y/colspan/rowspan/fill/stroke/align/inset/breakable, reject body/unknown/posicional com spans causais. Reutilizar casts do constructor já owned, sem avaliar declarações duas vezes ou duplicar schemas ricos. Publicar somente valores declarativos cell.<campo> após validação integral. StyleChain/custom contém declarações, nunca plano/carrier resolvido. Preservar branches de outros targets e avisos do subconjunto rico não certificado.

### Gate, aceitação e limites

Correção interna de paridade/mapeamento, ADR-0127 contínuo: nenhum contrato público, compatibilidade externa, flag/default de produto ou fase nova. Props e helpers são privados em carrier já não-construível externamente; lexicais não mudam. L0 editado primeiro, resselo restrito e RED→GREEN obrigatórios. Coletas bilaterais com erros/caret/efeitos, fonte/binaries/HEAD/UTC, geometria real; build debug/release/suítes e lint sem erros. Preservar árvore anterior sem commit/reset/resselo global. Subconjunto certificado: inteiros/autos, Color/None/Auto, Stroke uniforme/None, alinhamento, Length/inset parcial de Length/Auto, Bool/Auto e schemas já tipados. Gradient/tiling/functions, strokes por lado, inset percentuais e rich casts de constructor, grupos/spans multipágina/PDF-UA, set/text posicional e defaults gerais de Table continuam incompletos, com medição/L0 próprios antes de ampliar. Não certificar por warning fallback nem copiar defeitos upstream.

Precisão medida em refuters-before: stroke-constructor-fold e align-constructor-fold preservam o construtor inteiro contra set da Cell. self-set-does-not-change-placement muda colspan observado para 2 após a ocupação já feita com 1; não é prova de nova ocupação.

### Precisão dos spans observados após show-set (refutador C)

Medição anterior: refuters-before.json.gz / self-set-does-not-change-placement, vanilla devolve colspan 2 na célula real x0 e colspan 1 nos paddings x1/x2. Fonte pinada layout/grid/resolve.rs:218–244 captura spans de ocupação mas não assenta seus campos no conteúdo; model/table.rs:785–800 assenta 1 nos paddings. Logo ocupação e getter posterior são observáveis distintos. Decisão privada: a Cell posicionada conserva TableCellResolved como plano físico; o carrier standalone privado pode coexistir apenas para propriedades observadas da receita, mantendo campos resolvidos e sobrescrevendo spans lexicalmente omitidos pela declaração de show-set. Getters/canonicalização consultam essa projeção; placement do plano completo continua usando TableCellResolved. Padding sem fonte tem spans lexicais 1 para que show-set não os altere. Guards/stage continuam causais, sem novo contrato público nem mutação da fonte original. RED de span getter antes de implementar, GREEN mais geometria/regressões obrigatórios.

### Fold de declarações de Stroke — medição extra C

extra-first.json.gz (fontes/HEAD/estado/UTC/binários completos) compara two-stroke-sets: set stroke:red seguido de 2pt produz 2pt + red no getter avulso e posicionado vanilla. stroke-render-inherits-cell-style resolve o constructor 2pt com paint red da Cell antes do fallback da Table 3pt, embora stroke-constructor-fold avulso exponha somente o constructor 2pt. Fonte pinada model/table.rs:765 declara #[fold]; resolve.rs:225–237 usa stroke.resolve(styles), distinto do getter cru. Decisão: declarações sucessivas foldam componentes specified ao publicar Value::Stroke declarativo; nenhum carrier/callback é guardado em Styles. Constructor stroke permanece inteiro no getter avulso; resolução posicionada folda o constructor com a base capturada Cell e depois Table. Adicionar somente campo privado stroke_inherited: Option<Stroke> em TableCellProperties para transportar essa base causal, sem assá-la no getter lexical. ObservationEq inclui base; canonicalização ignora base derivada de render. Table já resolvida tem base None. Clone/aliases conservam base anterior; None explícito vence. RED→GREEN antes de código; correção privada contínua ADR-0127, nenhum contrato público/fase/default novo. A cláusula de precedência inteira do constructor vale para o getter avulso, não para a resolução fold de stroke, conforme medição.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: 00_nucleo/diagnosticos/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: Borrow do stroke herdado deve chegar a fold_stroke como Option<&Stroke>; publicar o resultado pelo From<Stroke> existente. Preservar captura e ordem do fold, sem mutação do payload de outro alias.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.

Ratificação Memória 5: o dono confirmou o manifesto SHA-256
6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237
com “confirmo” em 2026-10-08. O gate ADR-0127 deste pacote está satisfeito;
suas cláusulas propostas passam a vigorar nesta materialização.

## Condição comum antes do target — 2026-10-14

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/set-condicoes-2026-10-14/{before,extra-before}/results.json.gz
registra UTC, fontes, owners, status/diffstat, comandos e hashes dos binários,
contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8.
set text/page/par/block if false executa panic de argumentos. Condição com
mutação gera N=0 em false e N=2 em true, contra N=1 e N=12. show-set repete
o defeito. format.html desligado é acessado mesmo quando a condição falsa
pula o target no oráculo; condição inválida deve preceder esse acesso.
set.rs:44–54 só avalia condition no ramo Cell, depois da sondagem do target.
O erro de integer nesse ramo usa int indevidamente. A fonte pinada
lab/typst-original/crates/typst-eval/src/rules.rs:15–34 avalia/casta condição
antes do target e dos argumentos. Isso é ordem/efeito/diagnóstico da linguagem.

extra-before mostra heading if true e sem condição engolindo panic do
argumento, enquanto o pin rejeita. set.rs:193–194 e :1119–1120 substituem
Err por Value::None no heading e no elemento registrado; não é atalhamento.
O ramo de elemento registrado é extensão Tekt: preservar seu esquema aberto,
mas propagar erro causal de avaliação, conforme SourceResult e o contrato
comum de erros; teste puro com registro existente fornece o RED próprio.

### Decisão e responsabilidade exclusiva

Avaliar condition, quando presente, uma única vez na entrada de eval_set_rule,
antes de sondar/resolver target, verificar features ou avaliar argumentos.
Bool false devolve Value::None sem deltas/carriers/warnings de target ou
execução de argumentos. Bool true e ausência de condição prosseguem pelos
ramos atuais. Outro tipo gera expected boolean, found <nome longo> no span
da condição, reutilizando vanilla_type_name existente; erros de avaliação
propagam íntegros. Retirar a avaliação duplicada particular de Cell.

A cláusula anterior de feature gate HTML significa antes dos argumentos
quando o target é alcançado: condição falsa não acessa o binding nem ativa
feature. Gate e hints continuam iguais no caminho ativo. Esse controle prova
atalhamento, nunca paridade de HTML desligado. Show-set que delega a
capture_set_styles herda o mesmo gate; seu scratch/style restore permanece.
Fronteira lexical de markup após set falso continua governada por seu owner.

Nos dois pontos que convertiam erro de argumento em None (heading e elemento
registrado), usar propagação de SourceResult, preservando mensagem/span/trace.
Não alterar casts/defaults/schema de valores válidos nem ampliar targets.
Atualizar apenas este consumer e os testes proprietários. Correção interna
de paridade/propagação ADR-0127, sem tipo/assinatura pública, flag ou fase nova.

### Aceitação

RED→GREEN para false sem efeitos/diagnóstico de target, true com condição
antes de argumentos, condição exatamente uma vez, tipos/ranges/ordem de
falhas, Cell/alias e show-set, estilos anteriores preservados, erros ativos
em heading/elemento registrado. Recoleta CLI bilateral inclui gate de feature,
numeração, falha de condição/argumento e label após set falso. Build, suíte,
linhagem estrita/reversa e inventário de warnings. Sem alegação de paridade
universal de schemas/targets set ou seletores show especiais.

## Transformações — integração aprovada em 2026-10-14

Medição antes da decisão: corpus transformacoes-2026-10-14 e style-contract/results.json
registram o pin e os comandos/UTC. Fora de context há erro; aliases preservam
identidade; with não admite set/getter/show; dois set origin dobram por eixo.
A fixture de layout/tests.rs:21648 constrói matrix diretamente. Manifesto
aprovado 960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d.

Resolver aliases por identidade executável das quatro funções nativas após condição comum. Validar argumentos pela mesma construção tipada, sem executar layout/body, e assentar somente parâmetros declarados nos estilos custom existentes. Origin dobra por eixo sobre o herdado. Não aceitar with como target de set: o pin rejeita funções pré-aplicadas. Não promover o corpo artificial vazio a campo lexical de instâncias futuras.

Validar corpus e testes causais; não confundir construção com layout ou release.


## Continuação causal do gate — chamadas e set rules

Medição anterior à decisão: candidate-1/validation-green.log registra quatro
controles aprovados e falha real do teste de argumentos: std.html(panic(...))
executa panic antes do gate. before-set-probe.json.gz registra UTC, hashes,
fontes e saídas bilaterais no mesmo HEAD/árvore não commitada: std.html,
format.html, std.asset e format.bundle chamados avaliam argumentos antes do
gate; set std.html/std.format.html/alias apaga o erro do target e ignora os
argumentos. Variável/field ausente em set também é aceito indevidamente;
condition false conserva atalhamento. Html ativo em set por alias ignora panic
no Tekt e o executa no pin. São ordem causal, diagnóstico e efeito da linguagem.
collection_methods.rs:36 avalia argumentos antes de autenticar módulo; set.rs:
44 usa ok() na resolução e :326 autentica HTML pela sintaxe literal.

Após condition false/true/tipo já vigente, propagar o erro causal de resolução do target pure_path em vez de apagá-lo com ok(). Essa mudança inclui feature gate, variável ausente e field ausente; probes mostram aceitação indevida no Tekt e erro causal no pin. Não generalizar a resolução de targets não pure_path. Conservar target_function disponível por as_ref quando extraindo native_target. O braço existente de set HTML usa a identidade real FuncRepr::Format(FormatElementKind::Html, _) do target_function e não a grafia format.html; mover somente esse braço para antes do bloco restante de FieldAccess. Remover gate literal duplicado dentro desse braço: a resolução normal de target já autentica e aplica a feature. Assim aliases/std/raiz nativa usam o validador pretty existente e homônimos comuns não entram no braço por grafia. Preservar corpo de validação pretty, spans, condición falsa, demais formatos/targets e prioridades. Não introduzir API pública ou nova fase.

L0 precede esta continuação; correção interna/paridade em fluxo ADR-0127.
O núcleo builtin-module-identity continua pinado e sem alteração. O contrato
público e os campos externos permanecem iguais; nenhum fallback novo ou
supressão. Os controles e a matriz dos oito perfis permanecem; acrescentar
verificação bilateral dos efeitos e de set com/sem feature. A suite completa
regressa após o source freeze novo, sem reaproveitar gates de fonte alterada.

## A1 — callbacks de suplemento de Heading, 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c e working tree/UTC/hash
registrados em 00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz.
before-pdf-probe.json.gz: heading-supplement-panic termina em sucesso no Tekt
e em A1_HEADING no vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-supplement-valid aceita ambos, mas só o vanilla usa Custom na referência.
Fonte upstream model/heading.rs:249-289 sintetiza supplement por ocorrência e
model/reference.rs:393-415 aplica a função e display. Set.rs:286-320 ignora
supplement e limpa numbering; snapshot_fields heading.rs:55-68 fixa tradução.
Isto é callback/valor observável, não exigência de mecânica upstream.

### Decisão e responsabilidade exclusiva

No target heading, avaliar argumentos uma vez no fluxo já existente e
transportar supplement auto/none/Content/Str/Func como custom heading.supplement.
Não chamar callback em set. Uma set somente de supplement não desativa
numbering preexistente; só alterar numbering quando fornecido. Preservar
outros scope-outs de heading, sem declarar suporte universal a seus campos.

Correção de paridade em fluxo ADR-0127: sem nova API Rust, campo público ou
transferência de lógica existente entre eval/layout. O pós-walk com Engine já
é a fronteira de aplicação de callbacks. L0 primeiro, RED e controles bilaterais,
sem callbacks em encoder/walk/layout e sem alegar paridade de todos os campos.

## R2 — construção e seleção de Sequence, fluxo contínuo ADR-0127

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado.
00_nucleo/diagnosticos/sequence-constructor-2026-10-17/{before,expanded}.json.gz
e before-sources.json.gz conservam UTC, diffstat/status, binários/fontes e casos
bilaterais no pin bd5ebec285505889e8ff1d4f68804ab147a699f3. Vanilla
foundations/content/mod.rs:724–728 declara children requerido;
foundations/value.rs:644–650 define cast Content/string/symbol/none e
typst-macros/src/elem.rs:573–604 gera o constructor. Tekt
01_core/src/compiler/eval/bindings/field_access/content_methods.rs:358–410
ainda projeta fallback não chamável; o catálogo element_where não tem Sequence
e selector_matching.rs:272–337 não tem sua identidade. rules/set.rs:55–76
só recusa With de transforms e não reconhece set da Sequence. São semântica,
morfologia e diagnósticos de língua, não exigência de copiar storage upstream.
A mensagem de children() marcada pendente em R1 já coincide no baseline atual: o
getter recém-corrigido permite que o owner existente produza o erro correto.
A medição nova retifica essa pendência; não editar código de diagnóstico.

### Responsabilidade exclusiva

Reconhecer somente o callback native_sequence no target real de set. Set
vazio é no-op e todos os argumentos extras são erro em ordem causal; children
é requerido no constructor e não é settable. Avaliar argumentos uma vez antes
da validação (panic de valor prevalece). Preservar a condição false antes do
target/argumentos e rejeitar a aplicação parcial desta identidade antes dos
argumentos, pelo mesmo diagnóstico existente para With de transforms. Não
classificar nomes locais nem tocar outros targets/defaults/deltas.

### Aceitação e limites

Esta seção completa a responsabilidade R2 nomeada no L0 de R1. Corrigir
continuamente tabelas/composição/callback interno, sem novos campos/traits/
assinaturas Rust externas, modo padrão, fase ou quebra de compatibilidade.
L0 antes de teste e código, RED funcional, GREEN e corpus bilateral. Não
certificar guardas globais de show, todos os casts/elementos, nem paridade
universal de render. Qualquer refutador residual deve permanecer no relatório
e ganhar owner/L0 antes de intervenção, sem mudar testes para esconder o caso.

Validar build/workspace/doctests em invocação separada, Clippy/Rustdoc estritos,
fmt, V1/V5/V15/V26 e reversa, SARIF diferencial sem erro/warning novo. V19 pode
registrar agrupamento nominal prescrito sem supressão, testando alternativas.
Heavy jobs sequenciais, MemoryMax12GiB/High8GiB/swap0/jobs1, debug0/incremental0
só no ambiente de validação. Resselo após V15/V26 válido e proveniência exata.
