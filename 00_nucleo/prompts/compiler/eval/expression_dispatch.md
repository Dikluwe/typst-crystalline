# L0 — compiler/eval/expression_dispatch
Hash do Código: 54d4ea21



Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:825182e1dfe3591249a5efbc4c0ddbd937ebffde953c6595e0d1ed8176106435
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

## Medição anterior à decisão

No HEAD f7195cdf6 (árvore limpa, 2026-09-30), o bloco está em `01_core/src/compiler/eval/mod.rs:1651-2237`. Esta observação estabelece o limite da extração; não mede desempenho ou nova paridade bilateral.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/expression_dispatch.rs`. Dispatcher exaustivo e estático de Expr e helpers de expressão; preserva ordem, escopos, spans e diagnósticos. Manter contrato público, comportamento por defeito, fase do pipeline, compatibilidade, pureza e despacho estático.

## Aceitação

A suíte L1, o build e crystalline-lint passam. A separação não constitui prova de ganho de desempenho.

## D05 — captura lexical de show para equação literal

Medição antes da decisão: equation-where-2026-10-06/after.json.gz registra
show-block/show-inline aceitos sem transformação. expression_dispatch.rs:270–281
cria Equation e a retorna sem intercept_content; markup_stream.rs:119 só realiza
ocorrência inserida quando já está no modo de realização. Decisão: depois de
avaliar body uma vez e construir Equation, passar pela fronteira existente
rules::intercept_content, como os demais elementos de markup. A fronteira
captura receita lexical quando diferida e realiza quando solicitado, com guards
existentes; não adicionar avaliação global, fase nova ou novo wrapper. Preservar
Source/spans, captura de body para query e comportamento sem show. Fluxo contínuo
ADR-0127, mesma fase eval; teste literal RED→GREEN antes de código.

## D06 — captura das fronteiras lexicais existentes

Medição after.json.gz local-block-query retorna [2,0,2] versus [2,1,1];
after-additional.json.gz context-show-render produz fração versus EQ. Inspeção
de CodeBlock em expression_dispatch.rs:86–154 retorna Mathy sem guardar delta
local; Contextual:475–495 conserva scope da closure, mas não snapshot de show.

No join de cada valor Content de bloco de código, quando o estilo local mudou
e há math externo a inserir, guardar o Styles delta local existente no produto,
sem assar variável/constructor e sem espalhar set a expressões posteriores fora
do bloco. Capturar context externo com carrier Styled privado contendo um
ShowRecipe existente como Value::Content na chave privada eval.context.recipe;
sem campo/enum/assinatura pública nova. O snapshot conserva rules/styles/guards/
file/context_id_floor e não é executado ao capturar. A fachada L1 ratificada
realiza o produto contextual usando esse snapshot causal, sem regra retroativa.

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

No join de cada Content realmente emitido por CodeBlock, capturar ambiente local de inserção Table/Cell antes de restaurar scopes/styles/rules. LetBinding não é inserção de seu valor. Respeitar local-cell-alias/OUTER LOCAL e older-global-alias NEW, sem contador falso nem busca por nomes. Context captura snapshot Table/Cell pelos helpers L1 existentes; resultado contextual só realiza depois de produzir conteúdo. Preservar bloco matemático/D06, flow/diagnósticos/uma avaliação.

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

### Medição e correção suplementar interna — 2026-10-07

Proveniência: diagnosticos/table-cell-realizacao-d08b3-2026-10-07/geometry-fifth.json e local-bool-red.log, binário 59da863f, fonte/HEAD/estado/UTC/argv/glifos/testes.
local-bool-red.log mede FUNCcell em vez de FUNCsequence; geometry-fifth.json mede LOCAL/a quebrados. Em CodeBlock join efetivo, Cell crua precisa realizar regra local antes de compor outro body; empacotar receita diferida preservava Cell como body da Cell externa. Join Content com apply_show_rules=true delega imediatamente à fachada causal de Table/Cell; passagem original com apply_show_rules=false permanece sem preparo. O corpo continua valor lexical no let fora de join. Defaults de Cell isolada seguem D08B3-B; preservar it nominal, alias tardio, escopo e flags/restauração.
Não certificar antes da recoleta RED→GREEN e suites; nenhum campo/método público, flag ou mudança de fase suplementar.


## D08B3-C2 — candidato não vigente antes do gate ADR-0127

### Medição anterior à decisão

`00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz` registra oito sondas completas: fontes, comandos, UTC, HEAD, working tree/diffstat, hashes e binários. Pin 9f2b6e8715237cb086899a42873660fe744622e8. Executável Tekt congelado do C validado; `source-verification.json` confirma hashes produtivos/L0 iguais ao snapshot tested-final do C. O target da raiz não estava disponível nesta retomada; não foi fingido um rebuild.

bare/alias/spread-conflict: mensagens e hints iguais, caret/trace divergentes. No alias o vanilla usa a definição [a] (linha 1), não a chamada (linha 3). No spread usa [a], não o spread inteiro. local-body publica sequence com metadata+Cell preparada no Tekt, styled contendo Cell crua no vanilla; cores/efeitos coincidem no fixture. plain-func coincide cell; set-func/show-func/set-show-func medem cell/sequence contra styled. A primeira rodada `before` tinha str(function), foi invalidada para decidir morfologia; somente baseline usa repr(function).

Fonte: vanilla `typst-eval/src/code.rs:148` aplica Value.spanned; `typst-library/src/foundations/value.rs:210–216` só assenta Content/Func; `foundations/content/mod.rs:109–114` preserva span já assentado. `model/table.rs:436–437` transporta value.span ao fabricar Cell. `typst-eval/src/code.rs:26–57` conserva set/show como styled sobre a cauda. Tekt `eval/expression_dispatch.rs:125–136` realiza Cell no join; `stdlib/structural/table_grid.rs:160–175` guarda children sem origem; `realize/table_cells.rs:176–183` fabrica Cell com span detached; `layout/grid/row_groups.rs:109` usa esse span. É língua (diagnóstico/morfologia/efeitos), não igualdade Rust ou bytes PDF. Não inferir intenção upstream.

### Responsabilidade exclusiva candidata

Capturar primeiro span de Content no eval existente, preservando referências/spreads; CodeBlock preserva Styled lexical da cauda local e snapshot causal existente, sem executar receita ao guardar o valor. Respeitar pipeline/guards/escopos e não aplicar a correção cegamente a Flow/Math.

A especificação detalhada e limites estão no contrato candidato do pacote C2. Antes de materializar, instalar este conteúdo no caminho canônico do owner, auditar seus núcleos e selar hashes somente depois de RED→GREEN. A fonte vigente permanece intacta até aprovação.

A cláusula C2 supersede a realização imediata da Cell no join descrita pelo refinamento A: capturar valor lexical e realizar somente na inserção efetiva. Essa mudança de fronteira/ordem também integra o gate ADR-0127 §2.3; preservar a ordem Table→placement→Cell→body do núcleo vigente.

Ratificação C2: dono confirmou `confirmo` em 2026-10-07; contrato Source e fronteira causal aprovados.

### Refutação de abrangência C2

workspace-first.log refuta captura indiscriminada de todos os resultados Content: carriers matemáticos/nativos e grupos tipográficos perdem classificações existentes. Capturar Source somente na fronteira ContentBlock/CodeBlock, onde origem genérica ainda não é representada; constructor Table/Cell conserva span próprio. Outros constructors conservam seus carriers vigentes. Não alterar modelos nativos para esta dívida genérica.

### Captura de regras locais como delta — refutação C2

additional-debug/nested-local-text registra ausência da regra externa no produto interno (Tekt OUTER/LOCAL/a contra vanilla OUTER/LOCAL OUTER a). Snapshot local conserva somente regras introduzidas após a entrada no bloco; marcá-lo pelo canal privado eval.table.local=true. Na inserção, compor esse delta com regras causais atuais, por RuleId e ordem, sem congelar regras globais antigas ou duplicar id. Whole-context snapshots legados permanecem exclusivos e inalterados.

### Namespace privado da captura local C2

additional-debug2 refuta o snapshot local guardado em eval.context.recipe: consumidores de contexto completos podem substituir regras atuais antes de compor o delta. Usar chave privada dedicada eval.table.local.recipe, mantendo ShowRecipe existente e marca eval.table.local=true. Snapshot geral de context permanece em seu namespace anterior.


## C2-R1 — correção interna de paridade

Medição anterior à decisão: 00_nucleo/diagnosticos/performance-c2-2026-10-07/table-local-regression.json, três samples/r*-table-local/samples.json com fontes/argv/UTC/estado/hash do release ac889581 e vanilla pin9f2b6e871. Tamanho cresce10%, segunda coluna x60.880→38.999 com sobreposição. RED semântico em 00_nucleo/diagnosticos/table-local-c2r1-2026-10-07/red.log: tamanho10pt e separação Auto falham, tamanho local explícito11pt já passa.

Fonte upstream typst-eval/src/code.rs:32–43 aplica somente styles da declaração set à cauda. Tekt expression_dispatch.rs:132 colapsa raiz/defaults e não só declarações locais; style_chain.rs:304–315 contém size11/boldfalse/italicfalse na raiz. tracks.rs:328 reconhece somente Cell com resolved, deixando a Cell standalone Complete na projeção; measurement.rs não mede TableCell. É semântica/estilo/posição observável, não obrigação de reproduzir árvore ou algoritmo upstream. A causa conjunta é inferência refutável por fonte/tamanho/posição ainda divergentes após isolar estilos locais e medir o visual completo.

Classificação ADR-0127: reparo interno de paridade, sem API pública/trait/entidade/flag/fase nova. Fluxo contínuo autorizado por pedido de corrigir prioridade1; L0 primeiro, RED→GREEN, resselo restrito, builds/suítes/lint e recoleta física/temporal.

Responsabilidade: para captura local Table/Cell de CodeBlock, calcular delta de declarações desde insertion_styles pelo helper privado do owner 00_nucleo/prompts/entities/style_chain.md; Styled lexical recebe somente esse delta, acrescido dos dois canais privados C2. local_tail reconhece prefixo não vazio ou regras locais diferentes. O clone de entrada permanece ancestral por pushes/restauração lexical; ausência de ancestral viola invariante interna, não autoriza congelar defaults silenciosamente. Snapshot causal interno conserva styles/rules/guards/file/floor existentes; não reinterpretar whole-context nem bare math. Preservar shape Styled, efeitos uma vez, source/guard e comportamento das regras anteriores.


## Cauda lexical de set em CodeBlock — 2026-10-14

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/codeblock-set-2026-10-14/before/results.json.gz conserva
UTC, status/diffstat, fontes/owners, comandos, binários e saídas integrais,
contra pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8.
false-int/str/empty retornam valores crus; active-content perde Styled;
prefix-array junta arrays onde o pin rejeita array com content; prefix-int
aponta o erro para a string final em vez da regra set. sequential perde os
dois escopos; return-active omite o warning de conteúdo descartado. Célula
local já preserva Styled e constitui controle de regressão C2. A fonte
pinada typst-eval/src/code.rs:36–57 converte a cauda em conteúdo antes de
juntá-la ao prefixo; Tekt expression_dispatch.rs:91–226 faz join plano e só
transporta estilos em subconjuntos Table/Cell/Mathy. São tipo, operação,
morfologia, estilo e diagnóstico da linguagem, não estrutura Rust imposta.

### Decisão e responsabilidade

Cada SetRule sintática diretamente no CodeBlock separa um prefixo de valor
e uma cauda. Avaliar regra/condição/argumentos uma vez pelo owner set; guardar
somente suas declarações por local_delta_since do estado anterior, incluindo
valores iguais aos herdados. Avaliar a cauda em ordem normal, com joins de
valores dentro dela; converter o resultado da cauda para display Content
(inclusive None como Empty) e aplicar as declarações locais; depois juntar
ao prefixo. Erro do join prefixo/cauda tem o span da regra set; erro interno
à cauda mantém o span da expressão. False e set vazio não introduzem Styled,
mas a fronteira sintática continua convertendo a cauda para conteúdo.

Usar pilha privada de fronteiras para fechar caudas de dentro para fora;
nenhuma nova recursão por quantidade de sets. Não colapsar todas as declarações
no estilo final nem estilizar retroativamente o prefixo. Styled contíguos
sem conteúdo intermediário podem compor seus deltas com precedência interna,
preservando Source causal e canais privados. Marcar fluxo pelo canal
existente eval.flow.scope. Preservar carriers SetPage na sequência de saída,
fora do join de valores da cauda, sem descartar propriedades de página.

Table/Cell mantém realização apenas na inserção aprovada C2: a fronteira
Styled guarda o snapshot lexical de regras locais/guards/file/floor pelo
canal existente eval.table.local.recipe, sem executar receita ao armazenar.
Não transportar o mesmo delta de set novamente em cada célula. Regras show
introduzidas depois da fronteira preservam sua captura por ocorrência; as
anteriores permanecem por id/ordem. Math recebe o mesmo delta lexical pela
fronteira, sem Equation nova nem mudança de fase. Nenhum corpo/efeito é
reavaliado para reconhecer variante. Blocos sem set conservam seus caminhos.

Flow interrompe a coleta, mas as caudas já abertas devem produzir seu
conteúdo, incluindo vazio. Preservar return/break/continue e warnings causais;
restaurar scopes na saída normal ou com erro, sem alterar flow do chamador.

### Aceitação, gate e partes ainda incompletas

Correção interna de paridade ADR-0127, sem campo/tipo/assinatura pública,
flag/default ou mudança de fase: RED→GREEN e revalidação obrigatórios.
Testar tipos/Empty/false/mesmo estilo, prefixo e spans, deltas aninhados,
confinamento, uma avaliação, flow/warnings, Table/Cell e math. Medir produto
renderizado para estilos locais e revalidar controles C2/Math.

O passo codeblock-set-cauda é incompleto nas apresentações de Value cujo
owner display ainda tem escopo parcial: value-display-completo completa os
mapeamentos Raw/Module/Symbol conforme medição própria. codeblock-set-carriers
completa a morfologia pública de SetPage sem perder suas propriedades físicas;
codeblock-show-cauda completa show sem seletor e o tratamento lexical geral
da sua cauda. Campos de Styled (por exemplo child) pertencem ao owner de
field access e não são legitimados aqui. Essas pendências não justificam
certificar paridade universal; este owner não altera contratos de seus donos.


### Medição física suplementar antes da implementação

docs-before/results.json.gz conserva PDFs, XML MuPDF (fontes/tamanhos/quads),
texto e diagnósticos. style-scopes mede A/B/C/D/E/F em 11pt no Tekt contra
11/14/18/14/22/11pt no pin; same-style-alias mede M em 30pt contra 20pt,
confirmando que comparar apenas o valor efetivo do estilo perderia a
redeclaração lexical. local-math mede x em 11pt contra 20pt. local-cell
já mede Q20/R10 nos dois; local-cell-show/late e cell-hidden são refutadores
para ordem/ocultação das receitas C2. Não se normaliza extração textual nem
se exige identidade de bytes PDF para concluir preservação desses estilos.


### Refutador Raw da primeira implementação

refuters-after/results.json.gz, com estado/UTC/binários/saídas conservados,
mostra raw-rule-false/active imprimindo true em vez de RAW. A nova fronteira
passava Bool a Raw pelo helper de apresentação, mas omitia a intercepção
que markup_stream.rs:120–125 já fazia quando recebia Bool diretamente.
Antes de fechar a cauda, conteúdo Raw recém-criado pela apresentação de
Bool/Auto deve passar uma vez por rules::intercept_content, no Engine local,
preservando o protocolo de captura/guards do owner apply. Value::Content
já existente não recebe uma segunda intercepção. Isso restaura o percurso
existente, sem execução de receita Table/Cell nem nova fase. Teste RED próprio
para Bool/Auto, false/true e set aninhado, seguido da recoleta bilateral.


## R2-S — captura lexical de Sequence emitida por CodeBlock

Medição anterior: r2s-measurement.json.gz no diagnóstico
00_nucleo/diagnosticos/sequence-constructor-2026-10-17, pin bd5ebec285505889e8ff1d4f68804ab147a699f3,
registra local-rule AB contra LOCAL. Esta unidade em :198–245 só transporta
regras locais quando table_cells_needed; a restauração do Engine perde o
show Sequence de uma soma emitida pelo bloco. Contrato R2-S aprovado pelo
manifesto 46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327.

Depois de avaliar uma expressão Content uma vez no bloco, passar esse valor
pelo capturador Sequence com o local_engine antes de join/restauração. O
capturador só marca Empty/Sequence pertinentes, guarda rules/styles/guards/
file e não executa a receita. LetBinding não emite seu valor; conservar
CodeSetTail, local_delta, Table/math, flow, scopes e mensagens existentes.
Sem campo/trait/assinatura pública nova ou nova reavaliação. RED do corpus
antes do código; GREEN local e regressões completas sob 12 GiB/swap0/jobs1.

## R2-S — origem somente quando a captura é necessária

Medição antes da decisão: validation-attempt-4.json.gz, complete-workspace.log
e conditional-origin-before-sources.json.gz em 00_nucleo/diagnosticos/sequence-constructor-2026-10-17
conservam HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, estado não
commitado/UTC/diffstat/fontes/binários e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
A suíte de core passou 7228 testes, falhou 24 e ignorou 5. Os dois
controles de numbering local deixam de encontrar elementos internos;
os demais falham na geometria do prefixo/cauda após 1fr. A referência R1
validada antes desta etapa tinha zero falhas nesses testes.

markup_stream.rs passa content.with_source_span(child.span()) ao helper
mesmo quando não há regra Sequence ou quando o corpo não é Sequence.
O helper devolve esse novo carrier mesmo sem capturar. Dentro de um
ContentBlock, append_markup_sequence preserva Source, alterando as
fronteiras legadas de estilos e coluna que não precisavam de captura.
A adição de origem deve pertencer à operação efetiva de captura.

Correção interna da rota aprovada: a fachada crate-interna capture_at
recebe corpo cru e Span real; só acrescenta Source quando apply_show_rules
está ativo, existe snapshot pertinente e o corpo é Empty/Sequence.
capture conserva a assinatura interna atual e delega com Span detached;
capture_scope recebe Span explícito e só o usa dentro do ramo que cria
a receita marcada. Markup e CodeBlock passam o corpo sem embrulho prévio
e a origem real separadamente. Nenhuma regra pertinente ou corpo não
capturável devolve exatamente o conteúdo recebido; isso preserva os
escopos por defeito e evita alocar carriers inúteis. Não modificar os
algoritmos de layout nem os esperados dos 24 testes.

RED já observado na suíte integral; exigir GREEN de f2s, integração de
set/prefixo/1fr e controles Sequence antes da nova validação completa.
Manter origem de [] e chamadas, guards nominais, não localização e
coalescimento causal. Nenhuma assinatura Rust externa, campo público,
modo padrão ou fase adicional é criada.
