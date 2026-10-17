# L0 — compiler/eval/markup_stream
Hash do Código: 1f6faec7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/layout/typc-atom-display.toml sha256:2989f08ae6b4b379afb61d9673b94ab8538c45a97b98e566677ae0f719429dc3
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:16ae092d990d3d26f03232afcabf63f3b5c08ffb5b6297989368f0346d14f2ab


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

Proveniência: diagnosticos/table-cell-realizacao-d08b3-2026-10-07/geometry-fifth.json e local-bool-red.log, binário 59da863f, fonte/HEAD/estado/UTC/argv/glifos/testes.
geometry-fifth.json compara glifos de it.breakable: vanilla inline 8.8pt monoespaçado, Tekt texto 11pt. Vanilla foundations/value.rs:191–208 usa Raw(lang:typc,block:false) para Bool; markup_stream.rs:239–253 usa repr como Text. Corrigir só entrada Bool da tabela de apresentação para Content::raw(repr,Some(typc),false), mantendo demais tipos. Preservar intercepção e snapshot show de Raw também nessa apresentação (sem executar constructor para classificar). RED local-bool-red.log mostra zero Raw vs true/false. Correção interna de paridade/mapeamento, sem tipo/assinatura/fase nova. Render de tipos não medidos continua aberto.
Não certificar antes da recoleta RED→GREEN e suites; nenhum campo/método público, flag ou mudança de fase suplementar.

### Auto exposto pela Cell — reparo interno de apresentação, ADR-0127 contínuo

Medição anterior à decisão: table-cell-isolada-d08b3b-2026-10-07/after-second.json.gz, html-fields, em HEAD/estado/UTC/binários registrados: Tekt HTML X auto a como texto; vanilla code(data-lang:typc) e span constant.language #d73948. Upstream foundations/value.rs:191–208 exibe Auto/Bool como Raw inline typc; markup_stream.rs:210–215 só cobre Bool e seu fallback usa Text para Auto; layout/raw.rs:201–203 cobre só átomos true/false. HTML inline.rs não possui ramo Raw e conserva rejeição geral. É morfologia/estilo da linguagem, não byte de PDF; texto igual não prova paridade.

Antes da implementação, testes devem refutar a apresentação Auto como Text e a rejeição HTML de Raw atômico. Corrigir somente o mapeamento Auto para Raw(lang:typc,block:false), a classificação constant.language do átomo auto e a serialização HTML dos átomos typc auto/true/false com code e span constante, delegando escape/mode ao owner existente. Nenhum tipo/assinatura pública, fase, flag/default de produto ou gramática nova. A responsabilidade desta unidade limita-se à operação já owned correspondente: apresentação no markup, cor no Raw paginado, tags na inline HTML, testes no seu owner. Raw de outras linguagens/conteúdo arbitrário em HTML permanece fora da certificação. Confirmar RED→GREEN e igualdade do produto/estilos, com controles Table/Raw/query. Esta cláusula não autoriza mudanças nos outros tipos de Value ou implementação HTML geral de Raw.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Assentar origem do token Text na saída da intercepção lexical antes de construir ContentBlock. A origem inicial do texto precede a do bloco; por isso [a] aponta para a e não os delimitadores. Source permanece transparente aos grupos textuais e marcadores.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

### Origem lexical restrita C2

workspace-first.log refuta wrapping de todo Text do documento. Assentar origem de Text apenas quando este markup pertence diretamente a ContentBlock, identificado por Span/CST original, nunca por igualdade/nome/texto. Fonte literal inicial é transportada; documentos raiz preservam stream tipográfico anterior.


### Sequência com origem e crescimento de receita — refutação debug C2

depth-recheck.json.gz conserva os dois aborts após descascar Source em walk. append_markup_sequence:195–204 aplaina Sequence, mas o novo Source retém a árvore Sequence inteira no ramo preserve_source; [X#it.body] ganha uma camada por reconstrução. Decisão interna: ao inserir Source cujo body é Sequence, continuar a mesma concatenação plana, transportando a primeira origem como fallback em cada filho sem substituir origens já conhecidas. Source de outros elementos permanece; Styled/ShowRecipe não são descascados. O wrapper é transporte, não fronteira semântica de concatenação. Reexecutar os dez controles debug, caret/alias/spread/body, morfologia e corpus; sem alterar limite de show nem tamanho da pilha.

## Labels produzidos por expressão — 2026-10-14

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
00_nucleo/diagnosticos/labels-expressoes-2026-10-14/before/results.json.gz
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
00_nucleo/diagnosticos/labels-escopos-2026-10-14/before/results.json.gz
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


## RG1 — refinamento compartilhado de fase — candidato 2026-10-17

### Medição anterior à decisão

Medição anterior: 00_nucleo/diagnosticos/table-row-groups-2026-10-17/before.json.gz, measurement.json.gz, supplement.json.gz, lines.json.gz, styles-phase.json.gz e svg.json.gz conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC/status/diffstat/fontes/binários/argv/cwd. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. table_grid.rs:126–150 funde grupos; row_groups.rs:22–35/70/355–366 conta entradas, table_cells.rs:1204 aplica receita antes do planner e :1259–1282 deriva prefixos. model/table.rs:307–316 sintetiza; typst-realize/src/lib.rs:367–385/558–568 sintetiza antes de receita; layout/grid/resolve.rs:1380–1457/1597–1650 preserva intervalos e cursores; typst-html/src/rules.rs:591–665 projeta papéis. TableHeader level/presença somem no Tekt, coordenadas de linha não são lidas em table_lines.rs:173–260, e supressão None em layout/grid/rules.rs:40–46 deixa traços. svg-analysis.json confere todos os strokes retos e mudanças pareadas (mesmo binário): top vermelho vanilla 82→41, Tekt 82→82; azul após None vanilla 41→0, Tekt 40→40. Estes são efeitos/células/roles/erros/geometria da língua; não obrigação de copiar vtables, estruturas Rust, bytes SVG/PDF ou inferir intenção upstream. O primeiro extrator SVG não reconhecia line/h/v; foi descartado como prova, conservado e substituído por extração completa registrada.

### Responsabilidade exclusiva

Atualizar somente o pin do núcleo compartilhado. A distinção entre validação estrutural da Table e receita Cell/body substitui a leitura histórica de “Table antes de tudo”; este consumer conserva sua responsabilidade e código, sem novas operações locais, contratos próprios ou alargamento de escopo. Visual completo continua sendo a única vista publicada.

### Gate

Contrato candidato RG1: não vigente antes da confirmação humana do manifesto em 00_nucleo/materialization/table-row-groups-2026-10-17/review.md. A aprovação precisa abranger contratos públicos, compatibilidade e fase (ADR-0127 §2.1/§2.3/§2.4). Após aprovação, esta seção supersede apenas cláusulas históricas conflitantes de grupos Table/linhas e da fronteira estrutural/receitas. Outras propostas não são ratificadas. Não implementar campo/API novo antes do gate. Instalar L0/kernel/pins primeiro, testes funcionais RED antes do código, sem tratar erro de compilação/setup como RED; GREEN mais R1–R20 e corpus bilateral, build, workspace completo, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, formato, V1/V5/V15/V26 e reversa, diferencial SARIF/replay/diff-check. V16 externos continuam abertos; não alterar severity, criar allow/expect ou esconder um complemento em if-let só para zerar lint. V19 tem justificativa local e corpo compartilhado quando a política é a mesma. Núcleos são compartilhamento normativo; cada L0 conserva exatamente um consumer. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais; sondas/mutações usam target Cargo isolado e nenhuma variante mutante vai para target principal. Sem commit/push/merge/release ou pin novo.
