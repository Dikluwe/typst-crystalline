# L0 — compiler/eval/markup_stream
Hash do Código: f2fbd2c6



Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/markup-sequence-children.toml sha256:db4336e052b1d265683446ad4357ca819596185058ab10a5679ac0376ee4211f
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:825182e1dfe3591249a5efbc4c0ddbd937ebffde953c6595e0d1ed8176106435
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/eval/pending-text.toml sha256:3f300957a00ac2f02354331585c19b935428e05d5b8ac2115a533764262250c8
- 00_nucleo/prompts/_nuclei/eval/textual-consumed-slots.toml sha256:faaf206c9f4c20eba8b18b463ebe7836cb9671fd8672501c068eef2751f88d3c
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:f225f3039d4663e9bfcb408d26cf1cd6e557f089013bc2d95090ce16c52080c6
- 00_nucleo/prompts/_nuclei/layout/typc-atom-display.toml sha256:2989f08ae6b4b379afb61d9673b94ab8538c45a97b98e566677ae0f719429dc3
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

## Medição anterior à decisão

No HEAD f7195cdf6 (árvore limpa, 2026-09-30), o bloco está em `01_core/src/compiler/eval/mod.rs:1351-1650,2238-2261`. Esta observação estabelece o limite da extração; não mede desempenho ou nova paridade bilateral.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/markup_stream.rs`. Coordenação da sequência de markup, avaliação do corpo e apresentação de valores; preserva escopos, ordem e spans. Manter contrato público, comportamento por defeito, fase do pipeline, compatibilidade, pureza e despacho estático.

## Aceitação

A suíte L1, o build e crystalline-lint passam. A separação não constitui prova de ganho de desempenho.

## Inserção de aliases na realização — 2026-10-06

Medição: core-second.log registra ocorrência local devolvendo Original onde o oracle em controls.py devolve Local. markup_stream.rs:178-184 insere valores sem aplicar o snapshot de regras locais: a receita exterior perde esse snapshot ao devolver. Na realização produtiva, interceptar o conteúdo inserido no próprio ambiente lexical antes de append_markup_sequence; a proteção por ocorrência e produtos concluídos é delegada ao owner apply, não duplicada aqui. Fora de realizing_show_rules preservar o caminho existente. Preservar spans já conhecidos; erros sem origem desta inserção recebem o span real da expressão, sem procura por mensagens.

core-third.log registra a regressão P790 quando a intercepção geral repete a regra textual. A inserção produtiva delega exclusivamente a fronteira interna realize_inserted_occurrences do owner apply; literal/regex/text e preparação de parágrafos permanecem no caminho existente. Esta delimitação supersede a indicação genérica intercept_content acima.


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

SyntaxKind::SmartQuote cria Content::smartquote(kind.double) com propriedade
double explícita, sem consultar lang/quotes/adjacência/enabled para escolher
glyph. Preservar Source/spans, snapshots léxicos de Styles e sequência.
Aplicar fronteira estrutural de show existente aos SmartQuote, com guards,
identidade causal e captura, sem encaminhar a folha como Text para regras
textuais. Raw Text e strings code continuam literais. Aliases/receitas usam
fronteira realize_inserted_occurrences existente; não executar show duas vezes.
As decisões contextuais saem desta unidade; byte offsets ainda usados por
outros observáveis não são removidos indiscriminadamente.

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

Ao inserir valor Table/Cell em markup, capturar ou realizar no Engine lexical corrente, delegando table_cells/apply. Valor guardado permanece cru; cada inserção de alias recebe seu ambiente real, não o snapshot global da definição. Outros nativos/texto/aspas/matemática conservam seus caminhos; não aplicar Cell rule duas vezes nem antecipar corpo ocultado.

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

Proveniência: lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/geometry-fifth.json e local-bool-red.log, binário 59da863f, fonte/HEAD/estado/UTC/argv/glifos/testes.
geometry-fifth.json compara glifos de it.breakable: vanilla inline 8.8pt monoespaçado, Tekt texto 11pt. Vanilla foundations/value.rs:191–208 usa Raw(lang:typc,block:false) para Bool; markup_stream.rs:239–253 usa repr como Text. Corrigir só entrada Bool da tabela de apresentação para Content::raw(repr,Some(typc),false), mantendo demais tipos. Preservar intercepção e snapshot show de Raw também nessa apresentação (sem executar constructor para classificar). RED local-bool-red.log mostra zero Raw vs true/false. Correção interna de paridade/mapeamento, sem tipo/assinatura/fase nova. Render de tipos não medidos continua aberto.
Não certificar antes da recoleta RED→GREEN e suites; nenhum campo/método público, flag ou mudança de fase suplementar.

### Auto exposto pela Cell — reparo interno de apresentação, ADR-0127 contínuo

Medição anterior à decisão: table-cell-isolada-d08b3b-2026-10-07/after-second.json.gz, html-fields, em HEAD/estado/UTC/binários registrados: Tekt HTML X auto a como texto; vanilla code(data-lang:typc) e span constant.language #d73948. Upstream foundations/value.rs:191–208 exibe Auto/Bool como Raw inline typc; markup_stream.rs:210–215 só cobre Bool e seu fallback usa Text para Auto; layout/raw.rs:201–203 cobre só átomos true/false. HTML inline.rs não possui ramo Raw e conserva rejeição geral. É morfologia/estilo da linguagem, não byte de PDF; texto igual não prova paridade.

Antes da implementação, testes devem refutar a apresentação Auto como Text e a rejeição HTML de Raw atômico. Corrigir somente o mapeamento Auto para Raw(lang:typc,block:false), a classificação constant.language do átomo auto e a serialização HTML dos átomos typc auto/true/false com code e span constante, delegando escape/mode ao owner existente. Nenhum tipo/assinatura pública, fase, flag/default de produto ou gramática nova. A responsabilidade desta unidade limita-se à operação já owned correspondente: apresentação no markup, cor no Raw paginado, tags na inline HTML, testes no seu owner. Raw de outras linguagens/conteúdo arbitrário em HTML permanece fora da certificação. Confirmar RED→GREEN e igualdade do produto/estilos, com controles Table/Raw/query. Esta cláusula não autoriza mudanças nos outros tipos de Value ou implementação HTML geral de Raw.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Assentar origem do token Text na saída da intercepção lexical antes de construir ContentBlock. A origem inicial do texto precede a do bloco; por isso [a] aponta para a e não os delimitadores. Source permanece transparente aos grupos textuais e marcadores.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

### Origem lexical restrita C2

workspace-first.log refuta wrapping de todo Text do documento. Assentar origem de Text apenas quando este markup pertence diretamente a ContentBlock, identificado por Span/CST original, nunca por igualdade/nome/texto. Fonte literal inicial é transportada; documentos raiz preservam stream tipográfico anterior.


### Sequência com origem e crescimento de receita — refutação debug C2

depth-recheck.json.gz conserva os dois aborts após descascar Source em walk. append_markup_sequence:195–204 aplaina Sequence, mas o novo Source retém a árvore Sequence inteira no ramo preserve_source; [X#it.body] ganha uma camada por reconstrução. Decisão interna: ao inserir Source cujo body é Sequence, continuar a mesma concatenação plana, transportando a primeira origem como fallback em cada filho sem substituir origens já conhecidas. Source de outros elementos permanece; Styled/ShowRecipe não são descascados. O wrapper é transporte, não fronteira semântica de concatenação. Reexecutar os dez controles debug, caret/alias/spread/body, morfologia e corpus; sem alterar limite de show nem tamanho da pilha.

## Labels produzidos por expressão — 2026-10-14

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
lab/results/construction/diagnostics/labels-expressoes-2026-10-14/before/results.json.gz
registra UTC, status/diffstat, fontes, owners, comandos e hashes dos binários.
Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. alias/direct/closure/block
imprimem <alvo> e COUNT=0 no Tekt; o literal e o vanilla retornam COUNT=1.
O caso ref com é/😀 falha antes de exportar. Orphan não emite o aviso devido.
markup_stream.rs:100–135 associa somente SyntaxKind::Label; :138–140 manda
Value::Label à apresentação genérica, cujo fallback usa repr. A fonte pinada
lab/typst-original/crates/typst-eval/src/markup.rs:57–81 trata Value::Label no
fluxo. Esta é semântica de associação e observável diagnóstico, não mecânica.

### Decisão e aceitação

Após avaliar cada expressão uma única vez, Value::Label deve usar a mesma
operação privada de associação retroativa que o literal, antes de qualquer
conversão de apresentação ou realização de conteúdo interpolado. Preservar
nome integral, conteúdo anterior, espaços finais e origem da expressão no
aviso de label órfão. O label não produz texto. Reutilizar intercept_labelled
com suas guardas; não reaplicar regras textuais/construtores do alvo. Manter
atualização dos snapshots textuais e fronteiras de estilos após associação.
Escopo do bloco permanece local; um label órfão interno não rotula o irmão
externo. Não mudar value_to_display_content nem seus consumidores de math.
Testar aliases, chamadas, retorno de closure/bloco, Unicode/delimitadores,
efeito de avaliação única, show por label e texto, espaços e avisos/spans.
Recoletar query/ref pelo CLI e resolver o destino no leitor PDF independente.

Correção interna de paridade ADR-0127, sem assinatura/tipo público, flag ou
fase nova. L0 primeiro, RED→GREEN e resselo restrito. Esta etapa completa o
encaminhamento de valores Label; a associação geral ainda é incompleta:
o passo labels-associacao-alvos completará a seleção através de elementos
não rotuláveis e a substituição/aviso de labels repetidos, ambos expostos
pelos controles parbreak/duplicate desta coleta. Não declarar esses controles
fechados por esta etapa, nem promover diferenças do Tekt sem prova.

## Seleção do alvo dentro do escopo de markup — 2026-10-14

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/labels-escopos-2026-10-14/before/results.json.gz
registra UTC, fontes/owners/binários, comandos, status e diffstat, contra
vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8.
Parbreak direto, em bloco e por alias dá COUNT=0 no Tekt contra 1 no pin.
Set, set repetido/condicional falso, show e show-set permitem anexar ao
heading exterior no Tekt, mas no pin produzem warning órfão e COUNT=0.
Set page fornece um alvo sintético e suprime o warning órfão. Let e set
local dentro de code block são controles sem fronteira no markup exterior.
markup_stream.rs:195–222 remove só espaços/Empty e não conhece o início
do escopo de associação. O fold de estilos :156–177 detecta mudanças de
valor, insuficiente para reconhecer um set redundante ou falso. No pin,
typst-eval/src/markup.rs:39–56 inicia cauda lexical para SetRule/ShowRule;
model/par.rs:725–735 define Parbreak como não rotulável.

### Decisão interna

Manter um índice privado do início da cauda de associação, atualizado após
avaliar cada nó sintático SetRule/ShowRule do fluxo atual, incluindo carriers
sintéticos que ele emite. Não inferir fronteira por diferença de estilos,
nomes de funções ou conteúdo de texto. Um set dentro de expressão/bloco não
cria fronteira exterior. Novo conteúdo dentro da cauda pode receber label.
A associação pesquisa somente essa cauda, atravessando Space,
SpaceFromNewline, Parbreak e Empty já ignorado, com transparência de Source.
Não remover, reordenar ou reconstruir os itens atravessados: manter índices,
snapshots, estilos, origem, espaços e parágrafos. Só o alvo é substituído
pelo Label e entregue à intercepção existente. Sem alvo, emitir o aviso no
span da expressão/literal e conservar toda a sequência.

RED→GREEN deve cobrir parbreak direto/interpolado, escopos set/show/same/false,
set page órfão, conteúdo novo após a fronteira, let e code-block controles,
mensagem/severidade/span e preservação dos separadores. Recolher CLI com
query e diagnósticos completos. Correção interna ADR-0127, sem API/tipo,
flag ou mudança de fase; resselo restrito e validação integral.

O passo labels-associacao-alvos ainda está incompleto: esta etapa
labels-escopos completa seleção e fronteiras. A etapa labels-repetidos
completará substituição/aviso e ordem de efeitos de show para rótulos
consecutivos; não aplicar reparo que troque só o nome depois de executar
uma receita que deveria ter sido descartada. Empty explicitamente interpolado
e realização de heading em show também permanecem medidos e pendentes.


## R2-S — realização nominal de Sequence, ratificada em 2026-10-17

### Medição anterior à decisão

As coletas before/expanded/controls.json.gz e candidate.log em
lab/results/construction/diagnostics/sequence-constructor-2026-10-17 fixam HEAD
b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado,
UTC/diffstat/fontes/binários e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
Oito REDs iniciais; cinco passaram no candidato, show e recursão ainda não.
markup_stream.rs:240–260 aplana antes de selecionar a sequência; realize/mod.rs:
170–210 visita filhos antes de receita; apply.rs:140–170 delega não-localizáveis
ao ponto-fixo. O vanilla aceita alias e rejeita nova Sequence com mesmos
filhos. São efeitos, escopo, valores e diagnósticos de linguagem.

O dono aprovou a revisão
lab/history/construction/records/00_nucleo/materialization/sequence-realization-2026-10-17/review.md, manifesto
46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327,
nesta sessão. O gate ADR-0127 §2.3 cobre a ordem de realização desta nova
fronteira antes dos filhos/Table/math; não cobre novo contrato público.

### Responsabilidade exclusiva

Capturar fronteiras lexicais de Sequence somente quando há regra Sequence
ativa. Interpolações Sequence/Empty entram inteiras num recipe marcado antes
do append aplanador. Ao montar markup, formar recipes por cauda de show/set,
com snapshots de regras/styles/guards/file, sem retroatividade ao prefixo.
Intercalar com as fronteiras de estilo mantendo o seu nesting. Preservar
normalização textual e slots/índices; a nova captura não executa receita.

### Aceitação

S1–S20 ratificados, RED antes de código, GREEN bilateral e regressões; não
reduzir expectativas por limite mecânico. Todos os owners 1:1 e núcleos pinados
válidos antes do resselo. Build/workspace/doctests separados, Clippy/Rustdoc
estritos, fmt/linhagem/reversa e SARIF sem novo erro/warning. 12GiB/High8GiB/
swap0/jobs1 e comandos pesados sequenciais. Sem publicação/release.

## R2-S16 — origem da Sequence de ContentBlock

Medição: r2s-scope.json.gz conserva UTC/estado/binários no pin
bd5ebec285505889e8ff1d4f68804ab147a699f3. fresh-empty-show tem o
mesmo limite e hints, mas Tekt ancora o markup interno de [] (2:13)
e vanilla ancora o ContentBlock completo (2:12). markup_stream.rs:61–71
já encontra o pai ContentBlock para captura de origem; :212–219 usa
node.span do markup para o transporte Sequence. expression_dispatch.rs:
52–54 tenta usar span do bloco, mas Source conserva a origem anterior.

Correção interna: quando o markup tem pai ContentBlock, usar o span
do pai na captura da fronteira Sequence; demais corpos usam node.span.
Reutilizar o mesmo lookup já existente para determinar capture_origin.
Preservar todos os spans individuais dos filhos e a regra global de
Source que conserva a origem canônica; não trocar tipos, fase ou API.
RED da localização [] completa e controles de Source antes do código;
GREEN mantém limite/hints e os testes do bloco.

## Divisão explícita — pendências após R2/R2-S

Medição e proveniência: r2s-scope.json.gz/text-before.json.gz no diagnóstico
lab/results/construction/diagnostics/sequence-constructor-2026-10-17. Este pacote é
incompleto no universo de morfologia e regras textuais. R3-Symbol deve
completar identidade/igualdade do cast de Symbol (text/true no Tekt contra
symbol/false no vanilla); exige proposta/gate próprio se criar variante
pública, que R2-S não autoriza. R3-Markup deve preservar a Sequence
interpolada como filho imediato no valor de ContentBlock sem regra de
Sequence, hoje aplanada. R3-Textual deve medir/corrigir a precedência dos
literais em escopos distintos (C vs B), no owner matching. R3-Diagnostics
deve completar rastros genéricos de show de filhos emitidos. Estes nomes
identificam os pacotes de conclusão; não são prompts L0 nem autorização
para código adicional. Não classificar tais diferenças como mecânica
aceita ou paridade já demonstrada.

## R2-S — slots internos não são Empty explícito

Medição anterior à decisão: empty-slots-before.json.gz em
lab/results/construction/diagnostics/sequence-constructor-2026-10-17 registra fontes,
binários, UTC, HEAD/working tree/diffstat e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
Body simples produz EBODY contra BODY; filhos none/[] produzem EEA contra
EA; com regra textual A→AA aparecem Es adicionais. markup_stream.rs:
205–225 monta cauda incluindo slots Empty consumidos/normalizados;
textual.rs:104–120 monta pacotes com os mesmos slots; sequences.rs:
114–125 os visita como ocorrências reais.

Correção interna dentro da captura ratificada: ao montar o corpo da cauda
Sequence no owner markup e o corpo do pacote pendente no owner textual,
remover somente slots Content::Empty nus do vetor. As ocorrências Empty
explícitas sob regra Sequence já possuem captura/Source própria antes
de entrar nesse vetor. Conservar esses carriers, Text vazio e arrays
de filhos do construtor; não filtrar Empty arbitrariamente no visitor
de domínio. Corpus body/[]/none/construtor vazio/filho none/filho []/prefixo/
textual deve coincidir com o vanilla PDF, inclusive documento sem corpo.
L0 primeiro, RED em módulo antes do código; revalidar PDF e HTML separadamente.

## R2-S — raiz de pacote textual e EOF causal

Medição anterior à decisão: validation-attempt-2.json.gz, trace-recipes,
trace-markup2 e trace-textual2 em lab/results/construction/diagnostics/sequence-constructor-2026-10-17
mostram 11 testes verdes e dois REDs. eof-before.json.gz mede as fontes
exatas sem acrescentar newline: Styled com EOF produz Y nos dois; sem
EOF produz AB nos dois. A expectativa unitária Y usava a fonte sem EOF,
diferente da sonda anterior. Preservar ambos os controles externos; o
unitário Y passa a incluir o EOF medido, não a relaxar uma regra de show.

Alias textual sem EOF produz AAAAAAAAB no Tekt e AAB no vanilla.
sequences.rs:112–116 pula a raiz de todo pacote textual, mas a construção
Content::sequence normaliza um único item para o próprio item. Quando
esse item é ShowRecipe marcado, pular a raiz deixa a receita original
pendente e realiza outra captura nos seus filhos. O owner genérico volta
a executá-la e repete regras textuais. trace-textual2 registra essa
receita marcada ainda opaca no coalescimento após o percurso.

Correção interna: skip_root pula somente uma raiz crua Empty/Sequence
artificial de pacote, nunca ShowRecipe/Styled ou outra raiz normalizada.
Uma receita marcada na raiz efetiva deve decidir antes de seus filhos,
como qualquer outra ocorrência. Manter comparação causal estrita de
estilos/regras/guards/arquivo no coalescimento e todos os controles de
slots. Os logs cfg(test) temporários foram arquivados e serão removidos
do código antes da validação final. Sem campo/tipo/API/fase nova.

## R2-S13 — texto emitido e receitas escalares redundantes

Medição antes da decisão: validation-attempt-3.json.gz e emission-after.json.gz
em lab/results/construction/diagnostics/sequence-constructor-2026-10-17 preservam estado
working tree, UTC, fontes/argv/binários/diffstat no pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
Sequence emitindo [A] ou text("A") sob A→AA produz AAAA contra AA.
O controle text("A") sem Sequence já produz AAAA no release de pilha
SHA 2ab1495d555ff4d61e67eb14bef67597f7f668a0e3bce3a8129623011443441c
(native-before.json.gz). textual.rs:142–161 compara marca/guard de
Sequence como se distinguissem escopo textual e conserva receitas
genéricas escalares que aplicam o mesmo conjunto literal antes do pai.
trace-textual2/instrumentation-sources registram esses campos privados.

Responsabilidade do owner Sequence: fornecer helper crate-interno que
compara o transporte textual excluindo somente a chave privada
eval.sequence.occurrence do delta colapsado e guards cujo RuleId pertence
a regra Sequence no snapshot. Guards desconhecidos e de outras famílias
permanecem. Nenhuma filtragem de estilos reais nem assinatura externa.

Responsabilidade do owner textual: conservar agrupamento lexical estrito
TextualSnapshot::same_group. No coalescimento, pacotes textuais com
mesmo Arc de regras, arquivo e transporte textual equivalente são uma
única realização. Para receita genérica não textual, só eliminar o
transporte redundante se o corpo bruto for Text (permitido Source),
as regras ordenadas têm exatamente os mesmos RuleIds do pai, o arquivo
e o transporte textual coincidem, e nenhuma regra de elemento casa
com esse Text. Nesse caso não há callback de elemento a executar e
o pai já possui a aplicação literal/regex correspondente; devolver o
corpo com sua Source. Não eliminar receita Styled/estrutural/contextual
ou receita com regra de elemento aplicável. Não retirar regras do Engine
nem executar callbacks no coalescimento. IDs são a identidade já vigente
no mesmo contexto; não comparar só nomes, Debug ou texto do produto.

RED→GREEN de emissão [A]/text("A") e controle escalar nativo, com os
esperados do pin; controles de guard de outra família e estilo real
continuam opacos. Revalidar os 13 controles anteriores sem relaxamento.
Esta é correção interna de aplicação única já ratificada em S13, sem
novo tipo/campo público, fase ou mudança da política dos demais elementos.

## R2-S — origem somente quando a captura é necessária

Medição antes da decisão: validation-attempt-4.json.gz, complete-workspace.log
e conditional-origin-before-sources.json.gz em lab/results/construction/diagnostics/sequence-constructor-2026-10-17
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


## R3-Markup — filhos imediatos do valor de markup (2026-10-18)

### Medição anterior à decisão

lab/results/construction/diagnostics/sequence-r3-2026-10-18/before.json.gz e
before-sources.json.gz registram UTC, HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c,
working tree não commitado/diffstat/status, fontes e SHA dos executáveis.
Vanilla pin bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, oráculo SHA
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
Em [#([A]+[B]) C], o Tekt expõe quatro filhos A/B/espaço/C e o pin
três filhos Sequence(A,B)/espaço/C. Alias, dois siblings, soma posterior
e for apresentam a mesma perda; singleton coincide. Sob show Sequence
identidade o corpo já preservado coincide. append_markup_sequence em
01_core/src/compiler/eval/markup_stream.rs:286–311 recursa sempre nos
filhos Sequence, inclusive quando preserve_source identifica ContentBlock.
A fonte pinada lab/typst-original/crates/typst-eval/src/markup.rs:78
apensa o valor display inteiro e foundations/content/mod.rs:240–250
normaliza somente coleções de zero/um filhos. É morfologia da linguagem,
não obrigação de copiar Vec ou a mecânica de compartilhamento Rust.

### Decisão, limites e aceitação

Correção interna/de paridade ADR-0127 em fluxo contínuo: nenhuma API pública,
novo campo/trait, fase, modo ou default de produto. O núcleo pinado distingue
a coleção do valor e a composição do fluxo. Não mexer em Content::sequence,
concat_content, alias/Source, regras locais, captura anterior ao aplanamento,
Table/Cell, índices de estilos/labels/snapshots ou normalização de espaços.
A hipótese de perda na coleta é refutada se os filhos imediatos não forem
restaurados ou se singleton/+ e o produto visual mudarem.

Antes do código: RED funcional de filhos/tipos, alias imutável, dois irmãos,
singleton, + posterior, for e modo de fluxo. Depois: GREEN, controles CJK,
show/text/labels/Source e recursão Strong64/256, suíte completa, build,
Clippy/Rustdoc estritos, fmt, linhagem bidirecional e diferencial SARIF.
Teto12GiB/High8GiB/swap0/jobs1; comandos pesados sequenciais. Sem publicação.
As novas medições de None omitido, metadata textual vazando em children,
repr/campos Styled, precedência literal/regex e Symbol continuam separadas:
R3-DisplayNone, R3-TextualTransport, R3-Reflection, R3-Textual e R3-Symbol
completarão esses subconjuntos; este reparo não os certifica como corretos.

Responsabilidade exclusiva: quando preserve_source é verdadeiro, apensar o
Content inteiro sem descascar Source ou Sequence. Esse modo já é obtido pela
origem CST do ContentBlock; não inferir pelo texto ou nome do elemento.
No modo de fluxo anterior, Source é transparente e Sequence continua recursiva,
com a captura causal já feita pelo caller. Styled e os demais carriers/unidades
permanecem itens opacos de append. Enumerar nominalmente o complemento do match
Source/Sequence, cujo corpo único é push: grupo V19 é política de preservação
do item, não equivalência entre elementos. Sem wildcard/supressão/if-let para
ocultar V16. O forwarding de append não emitia V16 no baseline; esta explicitação protege a decisão sem receber crédito de redução de finding.


## R3 — slots consumidos e apresentação de None (2026-10-18)

### Medição anterior à decisão

lab/results/construction/diagnostics/sequence-r3-2026-10-18/{before,markup-after,
textual-before,transport-before,transport-before-sources}.json.gz conserva
HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado,
UTC/diffstat/fontes, executáveis e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
Sob show literal Z que não casa, nested.children expõe um pacote Sequence
mais dois Empty no Tekt; no pin são os três filhos originais. empty-child
expõe pacote mais Empty no Tekt, mas []/A no pin. markup_stream.rs:201–249
reconstitui o vetor incluindo slots que textual.rs:103–129 zerou; matching.rs:
90–95 também zera os siblings consumidos. São detalhes de transporte vazando
para a morfologia pública. CodeBlock/None interpolado desaparece; [#none#[]A]
tem dois filhos no Tekt e três no pin, e X#none;B sob show Empty resulta XB
contra XEB. A tabela value_to_display_content omite None por contrato histórico.
No pin typst-library/src/foundations/value.rs:191–210 apresenta None como
Content::empty; typst-eval/src/markup.rs:39–56 trata SetRule/ShowRule como
fronteiras próprias e :78 apensa o valor das demais expressões. Não copiar
buffers Rust nem inferir intenção a partir da implementação.

### Decisão e limites

Correção interna de paridade ADR-0127, sem contrato Rust externo, novo campo,
modo/default ou fase. Máscara privada Vec<bool>, sem Location falsa ou carrier
público novo. A hipótese é refutada por perda de Empty/Text vazio genuíno,
deslocamento de fronteira/estilo, repetição de callback ou índices inconsistentes.
A apresentação de None muda apenas na inserção do markup, não no helper genérico
usado por Math/CodeSetTail. SetRule/ShowRule continuam suas fronteiras próprias.

RED funcional antes de código: children/fields sob receita literal não casada,
filhos []/Text vazio, valor None de literal/let/assert/condicional, aplicação do
show Empty a None e set vazio/estilo sem Empty artificial. GREEN e corpus
bilateral, controles R2-S/EOF/slots/estilos/CJK/loops, build/suítes completas,
Clippy/Rustdoc estritos, formato/linhagem/reversa/diferencial SARIF. 12GiB,
High8GiB, swap zero, jobs1 e jobs pesados sequenciais; sem publicação.

A preservação de fronteiras do valor de markup não fecha a morfologia pública
de Styled/Styles, repr de labels ou a formação do valor de módulo/include.
R3-Reflection e o lote A3/include completarão esses eixos com medição própria.
Precedência/guardas textuais e show-set literal continuam no lote R3-Textual;
nenhuma mudança dessa política é legitimada por este transporte.

Responsabilidade exclusiva: na inserção, Value::None de expressão ordinária
produz Empty com o Span real da expressão antes das capturas existentes.
None de SetRule/ShowRule não é inserido como filho adicional. O match Value
é exaustivo; o complemento nominal é forwarding ao helper anterior, uma só
política V19. Não mudar esse helper compartilhado nem coerções dos demais tipos.
Receber máscara do coordenador textual; nos folds de estilos/Sequence, drenar
corpo e máscara juntos e excluir slots consumidos. Produto novo é marcado false.
Na montagem final, remover somente os consumidos restantes. Preservar a filtragem
anterior dos slots Empty nus na cauda de captura Sequence; Empty do autor em
ContentBlock conserva sua Source, e None pertinente é capturado antes do fluxo.


### Limite adicional medido — grupos textuais em múltiplas caudas

transport-after.json.gz no diagnóstico R3 confirma a remoção dos Empty
consumidos e a presença dos None reais. style-tail ainda expõe um grupo
sintético Sequence(A,Space) como um filho, antes do Styled; rule-tail também
conserva forma de cauda divergente. O problema é adicional à máscara de consumo,
não somente uma diferença de repr. O pacote R3-TextualGroups completará a
projeção linguística dos grupos sintéticos em múltiplos escopos e sua igualdade/
seleção, preservando a Sequence escrita pelo autor. R3-Reflection completará
Styles e Styled públicos. Não considerar esses controles fechados nem achatar
Sequence genericamente para escondê-los. Este adendo registra o recorte ainda
incompleto; não autoriza modificar os owners dessas projeções sem L0 próprio.

## R3-T1 — realização causal ratificada (2026-10-18)

### Medição anterior à decisão

lab/results/construction/diagnostics/textual-realization-2026-10-18/{before,scope,refuters}.json.gz
preserva HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não
commitado, UTC, status/diffstat, fontes/argv/hashes dos binários e pin vanilla
bd5ebec285505889e8ff1d4f68804ab147a699f3. before-sources.json.gz contém
fontes/L0 vigentes completos. Não há código candidato implementado nesta medição.
matching.rs:118–150 aplica regex reversa e depois literais em declaração;
:194–207 volta a expor produtos a passes subsequentes. textual.rs:102–123
captura snapshot completo. realize/mod.rs:177–217 visita receitas bottom-up.
apply.rs:620–648 e matching.rs:218–225 chamam funções sem capturar os filhos.
expression_dispatch.rs:125–129 já distingue insertion_rules do estado local.

Exemplos medidos: literais idênticos C/B; empate literal/regex R/L; overlap
X/YB; primeiro efeito LATER/FIRST; função encadeada inversa BBBB/BB;
produto separado AB/X; produto+vizinho X/AB; Strong ocultando filho dá
panic HIDDEN/Q; global capturado seguido de global nova dá L/R, mas local
capturado dá L/L. Entrada CJK local 水\n果 dá 水果/X. A fonte pinada
foundations/selector.rs:110–125 unifica literal escapado com regex;
typst-realize/src/lib.rs:1328–1405 escolhe o início e carrega revogação
no produto. Esses dados medem linguagem/efeitos; não ratificam mecânica Rust,
igualdade de buffers ou bytes PDF. É inferência causal que a separação de
escopos e a visita anterior aos filhos expliquem o conjunto; os controles de
acceptance.json, igualdade/fields/estilo/query e erros podem refutá-la.

### Decisão ratificada

Aplicar o núcleo textual-recipe-frontier pinado neste L0. A confirmação humana
do manifesto 47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302
ratificou esta ordem. Ele supersede neste escopo as
cláusulas históricas de execução provisória de show em ContentBlock e da
visita bottom-up de receitas pendentes, sem alterar a ordem de avaliação
de expressões comuns. Não reavaliar o programa, antecipar State/Counter,
assentar render em morfologia nem criar contrato Rust público.

### Responsabilidade exclusiva desta unidade

Registrar o frame lexical ao entrar no fluxo, distinguir a raiz do documento da fabricação de ContentBlock e registrar deltas de show nas suas caudas. Capturar receitas sem executar show textual provisório no valor. Herança global participa na inserção, delta local acompanha somente sua cauda; não inferir o delta pela diferença final de texto. Preservar authored Sequence, Source, None explícito, slots consumidos e fronteiras de #set/#show. A formação completa de Styled/Styles públicos e de grupos sintéticos em múltiplas caudas segue R3-TextualGroups/R3-Reflection, não se encerra por esta mudança.

### Aceitação desta revisão

RED→GREEN das famílias de acceptance.json, corpus R2-S/R3 e controles de uma avaliação. Revalidar PDF e HTML separadamente; build/workspace/doctests, Clippy/Rustdoc estritos, fmt, V1/V5/V15/V26 e linhagem reversa; diferencial SARIF sem novo erro/warning. Teto 12 GiB/High8 GiB/swap0/jobs1 e comandos pesados sequenciais. Não alegar ganho de RAM antes de ensaio pareado nem zerar o gate com supressão. Sem commit/push/release.

### R3-T1 — integração revelada pela suíte ampla

A regressão codeblock_set_display_raw_keeps_show_rules_once do candidato3
perdeu a regra Raw na inserção de um bool convertido dentro de CodeBlock:
o packet interno herdou Raw, mas a realização nova começa sem ambiente global.
Ao inserir um valor em markup diferido, capturar também o delta lexical do
frame de inserção com capture_local_output, depois do transporte Sequence.
O frame da raiz fornece as regras ali declaradas; bindings continuam sem emissão.
O packet interno então herda o ambiente atual e não congela a global antiga.
Preservar concatenação, slots, origens e distinção entre local e global.
