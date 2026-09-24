# Prompt L0 — motor geral de layout
Hash do Código: bbfda1ff

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/mod.rs`
**Vanila ratificado:** `9f2b6e8715237cb086899a42873660fe744622e8`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## P1494 — despacho da folha de espaço com origem newline (RATIFICADO; gate ADR-0127 confirmado em 2026-09-24)

### Medição anterior à decisão

No baseline `6da06e849d75972651a342ad6aad337313451c79`, o despacho
exaustivo de `Layouter::layout_content` trata `Content::Space` em
`01_core/src/compiler/layout/mod.rs:1865-1881`: só avança o cursor se a
linha já tiver material e usa a largura de espaço vigente. O diagnóstico
P1494 mede que a origem de newline altera a junção CJK antes de layout;
layout não dispõe do contexto textual de show/regex.

### Decisão ratificada

Despachar `Content::SpaceFromNewline` pelo mesmo comportamento de espaço
comum caso essa folha alcance layout: mesma supressão inicial, avanço,
oportunidade de justificação e flush condicional. Não executar regra de
junção CJK, colapso de origem ou show/regex no layouter; a normalização
textual cabe ao owner de eval antes do alvo. Não alterar assinatura,
geometria de outros elementos nem fase deste consumer. O braço exaustivo
será materializado após o gate ADR-0127 P1494, confirmado em 2026-09-24.

## Medição e contrato

O consumer define Layouter/PageConfig, entrypoints de layout, dispatcher
exaustivo e composição final de páginas. Domínios atomizados delegam por
funções estáticas descendentes. Métricas, introspecção e imagens são injetadas;
L1 não importa L3. Cursor, regiões, páginas, fixups e conteúdo diferido
preservam referencial e causalidade.

## Aceitação

Empty, texto, parágrafos, paginação, páginas auto, dispatch e composição final
são cobertos pela suíte de layout. Mudança pública/default/fase para no gate
ADR-0127.

## P1470 — exemplo executável do entry point com introspector

A documentação de `layout_with_introspector` contém um doctest Rust completo:
constrói `Content` textual, produz o `TagIntrospector` pela API pública, executa
o entry point e verifica o texto público do documento. O exemplo não pode ser
marcado `ignore`, `no_run` nem depender de fixture, I/O ou item privado.

## P1424 — layout intrínseco de frame HTML (APROVADO NO GATE ADR-0127 EM 2026-09-16)

### Medição anterior à decisão

No vanilla, `html.frame` é transparente em layout paginado
(`typst-html/src/rules.rs:88-91`). Durante conversão HTML, seu body é relaid out
sob target `Paged` numa região ilimitada e o frame resultante usa dimensões
intrínsecas (`typst-html/src/convert.rs:140-154`). A entry point pública vigente
produz `PagedDocument` com canvas/página e não representa este fragmento.

### Contrato proposto

O dispatcher exaustivo delega `Content::HtmlFrame(e)` a uma unidade
descendente que, no caminho paginado ordinário, chama layout do `e.body` sem
emitir wrapper nem alterar geometria. Para o caller L3 HTML, expor uma entry
point pura de layout intrínseco que recebe body, estilos/metrics/sizer e devolve
um frame/page transparente recortado ao conteúdo, sem margens, fundo, header,
footer, numeração ou canvas A4.

O layout interno observa target paged e conserva nesting: frame dentro de
frame não cria uma segunda ilha. Erros e callbacks pendentes seguem os canais
vigentes; não executar Func dentro do layouter nem importar L3. A assinatura
pública nova e o novo braço foram confirmados pelo dono em 2026-09-16 e ficam
limitados pelo escopo de P1424.

## P1374 — parcela fracionária interna proposta

### Medição anterior à decisão

O Block fracionário precisa receber do flow uma parcela já calculada; o
Layouter vigente não possui esse estado. Resolver dentro do Block confundiria
o peso local com o conjunto de siblings e impediria a razão 1fr/2fr observada
no vanila.

### Decisão proposta

Adicionar somente estado interno e não-público para a parcela vertical da
ocorrência fracionária corrente, guarda de preflight e sentinela consumível da
linha terminal da distribuição. Sequence instala e restaura a parcela ao
redor do dispatch do Block; Block apenas a consome. A sentinela permite ao
Cursor distinguir a tinta terminal confirmada do avanço para uma baseline
hipotética seguinte e é consumida no primeiro fechamento. Esses estados
entram em snapshots/checkpoints capazes de atravessar replay e nunca vazam
entre siblings, subframes ou páginas. Ausência de parcela não equivale a zero.
Não criar nova fase pública ou dispatcher dinâmico. O carrier público de Block
permanece sujeito ao gate ADR-0127 P1374.

## P1339 — sincronização das ocorrências Strong/Emph

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`:
`compiler/layout/mod.rs:1484-1501` é o ponto único de Locator/Position;
`:1523-1525` o chama antes do dispatch real. Strong/Emph em `:1894-1913`
descem no body por layout_content, mas a medição pura de tamanho em
`:2640-2666` usa measure_content_constrained e não emite ocorrências.
A promoção aprovada de NativeElement exige preservar essa distinção.

### Decisão

Strong/Emph locatáveis avançam exatamente no ponto canônico pré-dispatch,
seguindo a classificação de `compiler/introspect/locatable.md`. Não adicionar
incremento nos braços dos elementos nem nas sondas de tamanho. Styled
exterior continua sem avanço; seu filho Strong/Emph avança uma única vez.
O render de bold/italic e as fórmulas de medida não mudam.

Verificar que as Locations do walk e do layout pertencem às mesmas
ocorrências em corpo vazio, nested, labels e elementos subsequentes;
Position associa-se ao nó correto. Medição especulativa e replay de flow
não podem confirmar outra ocorrência para o mesmo evento. Preservar a
transação P1292 e o protocolo de callbacks math P1291, sem nova fase,
assinatura ou execução de Func. Se os braços já satisfizerem a regra,
não fazer refactor produtivo além de testes/linhagem necessários.

## P1292 — dispatch de `Flush` em forma B

### Medição anterior à decisão

O dispatcher não possui `Content::Flush`; floats são drenados somente em
fronteiras existentes. O registro P1292 demonstra que o marcador deve realizar
o prefixo de floats no ponto exato antes do conteúdo seguinte, sem antecipar
floats posteriores.

### Decisão

Declarar o módulo descendente `flush` e adicionar ao match exaustivo somente
`Content::Flush(e) => flush::layout(self, e)`. Toda decisão de efeito vive no
owner `compiler/layout/flush.md`; este hub não absorve o corpo, não altera o
owner de `place` e não introduz despacho dinâmico. O arm não é incluído no
fallback matemático nem tratado como Empty.

## P1292 amendment-10 — checkpoint atômico do sufixo de flow

### Medição anterior à decisão

O contrato v10 estabilizou três páginas e o anchor do float-prefixo, mas uma
ocorrência de Place não-float dentro do Block posterior ao marker ficou na
página 2 enquanto a linha que a continha migrou para p3. Um reproducer
black-box independente obteve no candidato `AFTER_MARKER` em p2 e
`AFTER_FLOW` em p3; no vanilla ratificado ambos aparecem em p3, exatamente uma
vez. A migração somente de `current_line` deixa escapar efeitos que já foram
anexados a `current_items` antes da decisão final de fitting.

Isto refuta checkpoint restrito a texto/linha e refuta um caso especial em
Block: o efeito escapado é Place não-float, mas a unidade causal é todo o
sufixo de layout produzido depois da fronteira, qualquer que seja o tipo do
child.

### Decisão — transação regional do Layouter

Este owner define o checkpoint interno e não-público do estado mutável do
Layouter. Depois de o prefixo do marker estar realizado e suas reservas
estabilizadas, o Cursor estabelece uma fronteira de retomada antes de qualquer
efeito do flow posterior. Todo efeito regional posterior pertence a uma única
transação de sufixo até a linha/unidade de flow ser aceita.

O checkpoint captura, por valor lógico ou por comprimentos de cauda, todo
estado capaz de tornar esse sufixo visível ou afetar fitting: posição e região
correntes; buffers e métricas da linha; cauda de `current_items`; cauda de
items/frame da página ou sub-frame ativo; extensões inline e geometria
pendente; fronteiras de floats/deferred locais criados no sufixo; e o ponto de
replay por ocorrência. Novos buffers regionais futuros entram na mesma
transação por obrigação, não podem escapar por terem sido atualizados antes de
`current_line` estabilizar.

Estado anterior à fronteira não é clonado, migrado nem desfeito. Páginas e
items já confirmados, prefix floats realizados, suas reservas top/bottom e
todo conteúdo anterior permanecem imutáveis. A transação registra somente a
cauda posterior; floats criados depois do marker continuam sufixo e nunca são
promovidos ao prefixo.

Se a unidade posterior cabe, commit torna a cauda definitiva. Se a região
efetiva a rejeita, rollback remove atomicamente toda a cauda pós-fronteira,
restaura cursor/métricas/buffers ao checkpoint, avança pela regra normal e
reexecuta as mesmas ocorrências na nova região. `current_line` e
`current_items` nunca são migrados separadamente. Cada ocorrência produz no
máximo um efeito confirmado; replay não duplica Place, texto, float posterior,
tag, link ou item diferido.

O Layouter oferece a transação; `compiler/layout/cursor.md` decide
commit/rollback/avanço; `compiler/layout/place.md` continua dono de fitting e
reservas; `compiler/layout/flush.md` continua dono apenas da sentinela. O
checkpoint não examina o conteúdo futuro, não prevê altura/tipo, não cria fase
ou passagem e não altera API pública.

### Refutadores e aceitação

Refutam: snapshot apenas de linha; truncar `current_line` sem a cauda de
`current_items`; recompor o documento/prefixo integral; mover items anteriores;
duplicar efeitos; inspecionar o próximo Content; especializar Block; ou deixar
um Place não-float pós-marker na região rejeitada.

No caso focal, PREFLOW permanece p1; FLOAT_BEFORE permanece p2; AFTER_MARKER,
AFTER_FLOW e FLOAT_AFTER aparecem exatamente uma vez em p3. No-floats,
no-flush, nested, clearance explícito e prefix identity preservam os resultados
validados.

### Amendment-11 — limite do checkpoint face ao flow ordinário

Medição bilateral posterior mostrou que o controle sem Flush e o controle sem
float já divergiam do vanilla do mesmo modo: faltava o gap Block→flow de
`1.2em` e Place não-float ancorava no topo. Portanto o checkpoint não adquire
autoria sobre essas fórmulas ordinárias.

Este owner continua dono somente da capacidade transacional e dos campos
privados necessários para capturar/restaurar a cauda. Qualquer estado de
origem de replay só pode existir durante uma transação de marker realmente
rejeitada e deve restaurar exatamente a semântica ordinária fornecida pelos
owners Block e Place. Fora dessa causalidade, o Layouter não traduz
`current_items` por crescimento de cauda, não procura items “novos” e não
redefine baseline/gap de flow.

O fluxo sem marker e o fluxo sem float são controles obrigatórios: não entram
na transação e devem resultar exclusivamente das fórmulas dos owners
`compiler/layout/block.md` e `compiler/layout/place.md`. Um reparo que os torna
verdes por ajuste em Cursor viola ownership mesmo que preserve o vetor
transacional.

## P1286 — dispatch PDF condicionado ao gate

### Medição anterior à decisão

O dispatcher exaustivo ainda não possui braços para attachment/artifact.
`PagedDocument` é montado neste consumer; logo um side-channel de attachments
coletado durante layout precisa ser drenado aqui. O caminho atual de Formula
já demonstra o envelope `FrameItem::Semantic`; ele não pode ser reutilizado
como Formula para artifact.

### Decisão proposta

Após confirmação dos contratos públicos em `entities/content.md` e
`entities/layout_types.md`, o match central permanece magro e delega:

- `Content::PdfAttach` à futura free function dona da feature, que registra o
  mesmo `Arc<PdfAttachElem>` no side-channel em ordem documental e não cria
  frame nem desloca cursor;
- `Content::PdfArtifact` à futura free function dona da feature, que faz
  layout do body uma vez e o envolve em `FrameItem::Semantic` com
  `SemanticKind::Artifact(kind)`, sem alterar dimensões, posição ou texto.

A composição final move o side-channel para `PagedDocument.attachments`.
Não há introspecção global nova, segunda passagem nem mudança da ordem
eval→layout→export. Os dois ficheiros futuros de feature só recebem Prompt L0
proprietário 1:1 após a confirmação; este owner não os legitima. O gate é
obrigatório pelos tipos públicos já listados, não por mudança de fase.

## P1291 — passagem pura de requests math (VIGENTE; PRESERVADA POR P1292)

### Medição anterior à decisão

O caller de math em `01_core/src/compiler/layout/equation.rs:107-120` constrói
`MathLayouter` somente com métricas, modo block e estilo. Neste ponto o
`Layouter` possui região efetiva e `StyleChain`, mas não deve adquirir
capacidade de executar `Func`. Os precedentes P240/P241/P1159 mantêm o
Layouter puro e realizam callbacks na pipeline entre passagens.

### Contrato vigente

Adicionar uma entry point explícita de passagem que recebe
`&SealedMathCallbacks` e devolve `MathLayoutPassOutcome`. O `Layouter` cria um
estado de transcript em memória e o compartilha com todos os `MathLayouter`s
daquela passagem. Ao entrar numa equação, `equation.rs` fornece a `Location`
corrente da equação,
`regions.effective().height`, a `StyleChain` léxica e o mesmo estado; não
fornece função executável.

A entry point compatível devolve `SourceResult<PagedDocument>` e continua
válida para árvores sem requests. Se a passagem retornar `Pending`, descarta
as páginas provisórias e devolve `Err(vec![diagnostic])`, conforme
`SourceResult<T> = Result<T, Vec<SourceDiagnostic>>`, **sem valor e sem
documento**; não existe `PagedDocument` vazio/de-falha capaz de chegar a um
exporter. A entry point de passagem não marca a primeira medição como erro:
retorna `Pending(requests)` ao caller L3, que é obrigado a realizá-las e
repetir.

Nenhum estado de eval é guardado em `Content`, `Region`, `PagedDocument` ou
cache de métricas. Request e store pertencem à invocação e são retornados por
valor. `PagedDocument` só sai na variante `Complete`, depois de o transcript
estar integralmente resolvido, totalmente consumido e estável.

O escopo da passagem é a tentativa concreta criada por cada `Layouter::new`,
não a invocação externa inteira. O loop interno que pode refazer layout por TOC
cria `MathCallbackPassState` novo para cada tentativa, descarta junto com o
documento os transcripts das iterações rejeitadas e conserva somente o estado
da tentativa candidata. Se essa candidata possui pendências, devolve apenas
`Pending`; caso contrário pode devolver `Complete`. Esse ciclo de callbacks é
ortogonal a `compiler::introspect::run_fixpoint` e não é fundido a ele.

O confirmação/runtime P1291 já foi materializado. P1292 preserva integralmente esta
ligação e não cria nova fase. Contrato detalhado:
`compiler/math/layout/callbacks.md`; implementação da capacidade:
`infra/pipeline.md`.

## P1373 — medição intrínseca de Block relativo

### Medição anterior à decisão

Depois de `BlockElem::height` passar ao contrato confirmado
`Option<Rel<Length>>`, o check focal encontrou em
`layout/mod.rs:2588-2618` o segundo consumer produtivo: o braço de
`measure_content_constrained` ainda chamava `Length::resolve_pt` diretamente.
Esse braço mede Block para grid e fitting; não é proprietário da fórmula de
altura.

### Decisão

Ao medir `Content::Block`, o motor delega a resolução relativa ao helper
estático descendente de `compiler/layout/block.rs`, fornecendo font-size e
`available_height()` externos. O valor resolvido continua sendo apenas o
mínimo entre altura declarada e altura natural com inset. Não duplicar a
fórmula, não usar `max_width` como eixo vertical e não resolver contra a altura
do body. Alturas absolutas mantêm as expectativas existentes; width relativa
e height fracionária permanecem no P1374.

## P1391 — arm interno de fechamento overlong validado

### Medição anterior à decisão

Nos pares 39/40→11, a primeira palavra size-only é horizontalmente overlong e
o layout de texto fecha sua linha antes de Sequence alcançar o segundo
wrapper. O fechamento ordinário usa o estilo 39/40pt para a baseline seguinte;
o vanila usa o estilo exterior de 11pt. O carrier posterior P1389 chega tarde
demais para selecionar a fórmula correta.

### Decisão interna

Layouter mantém um booleano privado one-shot, armado exclusivamente por
Sequence após o preflight certificar a forma P1391 completa. O primeiro
`flush_line` consome o estado e pede a fórmula singleton exterior já definida
por Cursor P1389. O estado inicia falso, integra checkpoint/rollback e é
isolado em sub-frame; não sobrevive ao fechamento que o consome.

Não há nova API pública, fase ou default. Ausência da validação, prefixo
incompleto, outra célula de tamanho e fluxo ordinário nunca armam o estado.
11→39, 39→11, 11→40 e 40→11 devem manter uma página e `After=Tail`;
40→8 permanece opaco.

## P1392 — reutilização do arm por validado cruzado

### Medição anterior à decisão

Em 40→8 e 40→7, a palavra inicial é overlong e fecha dentro do layout de
texto, pela mesma causalidade P1391. O vanila prevê a linha seguinte pelo
estilo exterior; sem o arm, o cristalino usa 40pt e pagina a cauda.

### Decisão interna

O mesmo booleano privado one-shot pode ser armado também depois de Sequence
certificar a forma P1392 completa. Inicialização, consumo, checkpoint e
isolamento permanecem idênticos a P1391; não se adiciona campo, API ou fase.
Formas não validadas nunca armam o estado. P1391 e os controles de replay
continuam invariantes.

# P1402 — fallback de layout para entry

Uma `OutlineEntry` construída diretamente é transparente ao layout nesta fatia:
desce no corpo outlinable. A geração visual completa e os helpers contextuais
permanecem fora do escopo; isso não altera o layout vigente de `outline()`.

# P1405 — consumo do carrier PDF label

Ao encontrar `Content::Styled` cuja propriedade custom privada
`pdf.marker.tag` seja a string `label`, layoutar o body uma única vez em
sub-frame inline, conservar o avanço do cursor e recolocar os items produzidos
num `FrameItem::Semantic { kind: Label, placement: Inline }`. Outros Styled
seguem o push/pop vigente sem branch semântico. O carrier é visualmente
transparente e não altera medição.
# P1406 — delegação de link interno de outline (PROPOSTO; gate autorizado)

Medição anterior à decisão: `Content::Link` cristalino representa somente URL,
mas o show default de `outline.entry` no vanila ratificado liga o corpo à
Location causal. `FrameItem::Link` já suporta `LinkTarget::Destination` e o
layout de referências já o emite.

O arm central de `Content::Styled` reconhece somente o carrier privado e tipado
de destino produzido pela realização P1406 e delega a captura visual ao módulo
`layout/outline.rs`. Não interpreta a regra do outline, não generaliza
`link()` e não reconhece texto por convenção fora da chave privada. O módulo da
feature envolve os itens dispostos em `LinkTarget::Destination`; todo outro
Styled conserva o caminho vigente. Confirmação humana explícita recebida em
2026-09-15.

## P1407 — carrier privado para expansão adiada de `repeat`

Medição anterior à decisão: no baseline `ed7340ada`, `Repeat` é disposto antes
de uma caixa `width: 1fr` registrar sua parcela, e a largura correta só aparece
no fechamento da linha. A sonda limitada de 100 pt rendeu 1 cópia no cristalino
contra 23 no vanila ratificado.

Layouter conserva uma lista privada e finita de peças repetíveis pendentes. O
registro contém somente faixa de itens, origem, largura resolvida, gap e
justify; não expõe API nem altera `Region`. Checkpoint/rollback trunca a lista
pela cauda e a inicialização começa vazia. O owner `repeat.rs` decide a fórmula
e o teto; Cursor fornece a parcela, e consumidores que selam containers devem
resolver os pendentes antes de drenar os itens da linha.

## P1408 — largura natural privada de caixas fracionárias

A prova bilateral `1fr:2fr` mostrou que descontar previamente a largura
natural dos bodies desloca a segunda caixa 2,73 pt para a direita. O vanila
atribui a cada caixa uma parcela do espaço total disponível e dispõe o body
dentro dela.

Layouter conserva, em lista privada alinhada às fronteiras `pending_fr`, a
largura natural já ocupada por cada entrada: zero para `h(Nfr)` e a largura
exterior natural para `box(width: Nfr)`. Cursor soma essas larguras ao restante
antes de calcular parcelas e subtrai a largura natural da parcela ao deslocar
o suffix. Checkpoint/rollback trunca a lista pela cauda, e fechamento ou região
infinita a limpa junto com `pending_fr`. Nenhum campo de `Region`, assinatura
pública ou quantidade de itens proporcional ao peso é introduzido.

## P1414 — estado privado da numeração de linhas (PROPOSTO; gate ADR-0127)

Medição anterior à decisão: o Cursor já fecha linhas físicas depois do wrap e
da eventual migração de página, mas Layouter não conserva progresso de
`par.line`. O vanila reinicia por página quando pedido e mantém progresso no
documento caso contrário.

Layouter acrescenta somente estado privado para o próximo número documental e
a página causal da última linha numerada. Inicializa em zero/ausente, integra
checkpoint/rollback e sublayouts sem vazar contagem especulativa. Cursor é o
único consumer e incrementa apenas depois de estabilizar uma linha com itens.
Não criar counter público, marker em Content, segunda passagem ou estado
global. A ampliação de comportamento padrão aguarda confirmação humana.

## P1415 — transcript puro de callbacks de linha (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O Layouter P1414 conhece a linha física somente depois de wrap, fitting e
baseline, mas L1 não possui nem pode possuir `Engine`. O precedente P1291 já
separa medição pura e realização L3 por transcript validado. No baseline
`0a7de3dd9`, a entry point callback-aware cobre apenas math.

### Decisão proposta

Generalizar a entry point explícita de passagem para receber stores validadas de
math e de `par.line` e devolver um outcome combinado. O estado privado da
tentativa registra requests de linha em ordem causal com `Func`, inteiro
lógico, `StyleChain` e span; não executa a função nem conserva documento no
outcome `Pending`. Resoluções contêm `Content` já realizado e são consumidas
uma única vez pela mesma ocorrência na passagem seguinte.

Tentativa interna rejeitada por TOC, replay ou fitting descarta seu transcript.
`Complete` exige todos os stores consumidos e nenhum request novo. Divergência
de ordem/quantidade devolve `Pending`, nunca documento parcial. Entry points
compatíveis permanecem para árvores sem request e falham fechado ao encontrar
callback que não possam realizar.

O store é local à invocação, integra o ciclo já limitado de callbacks e não
entra em `Content`, `Region`, `PagedDocument`, cache global ou introspector.
Não há `Engine`, `World` ou `EvalContext` em L1. A nova passagem pública e a
mudança de fase aguardam confirmação humana explícita.

## P1416 — contexto privado de ordinal transversal

### Medição anterior à decisão

O modo segmentado cria regiões irmãs independentes, mas P1415 conserva um
único contador progressivo; por isso `#columns()` soma linhas das colunas em
vez de contar o ordinal transversal uniforme observado no vanila.

### Decisão interna

Layouter pode manter um contexto opcional, privado e escalar com: limite
absoluto de ordinais já emitidos e compensações horizontais até os bordos
inicial/final do contentor. `columns.rs` instala/restaura o valor por segmento;
Cursor somente consulta. O campo inicia ausente, participa do checkpoint e
não entra em subframes sem instalação explícita.

O contexto não guarda frames, páginas, callbacks ou vetores, não muda Region,
entrypoint ou fase. Ele existe somente durante `#columns()` segmentado do
recorte P1416; colunas de página e fluxo contínuo permanecem nos caminhos
anteriores.

## P1417 — buffer privado de candidatas marginais

### Medição anterior à decisão

No baseline `af587e30b`, o estado escalar P1416 não representa a ordem de
baselines heterogêneas. A fonte pinada decide a ocorrência marginal depois de
ordenar marcadores e mede a altura do número no estilo exterior comum.

### Decisão interna

Layouter acrescenta um contexto booleano privado e uma cauda de candidatas de
pattern. Cada candidata contém somente baseline, página, pattern e
`StyleChain`; não contém frame, callback, Engine ou documento. O owner de
`#columns()` segmentado instala/restaura o contexto, consolida e drena a cauda;
checkpoint registra o contexto e comprimento para rollback.

O vetor é linear nas linhas numeráveis da composição ativa e não é usado para
callbacks. Em colunas de página e fora de colunas segmentadas permanece vazio;
o comportamento P1414/P1415 imediato não muda.

## P1418 — estilo compartilhado privado de coluna de página

### Medição anterior à decisão

No baseline `573c42c14`, a cauda P1417 já contém chain causal suficiente para
capturar o estilo da primeira ocorrência, mas colunas de página não a ativam.
O estilo local da segunda coluna diverge do estilo compartilhado observado no
vanila.

### Decisão interna

Durante uma composição de colunas de página, Layouter mantém um
`Option<TextStyle>` privado capturado da primeira candidata disponível e o
índice inicial da cauda da coluna corrente. Ambos integram checkpoint/rollback,
são instalados e restaurados pelo owner de columns e não entram em entidade ou
contrato público. `close_current_column` drena a cauda por coluna; o modo
segmentado direto usa estado local equivalente. A cauda fica vazia ao sair.

Callbacks não usam esse estilo nem essa cauda. Primeira coluna sem candidata e
configurações locais divergentes permanecem `Unknown`.

## P1436 — encaminhamento geral de `MathLr` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

O dispatcher geral agrupa variantes `Math*` como conteúdo inline entregue ao
layouter matemático; um novo braço ausente quebra a exaustividade, enquanto
realizar `lr` aqui duplicaria o owner especializado.

### Decisão proposta

Encaminhar `Content::MathLr` pelo mesmo caminho inline das demais variantes
matemáticas. Este owner não interpreta size, cercas ou cascata; toda lógica de
produto permanece em `compiler/math/layout/delimited.rs`.

## P1437 — encaminhamento geral de `MathMid` (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

O dispatcher geral agrupa variantes `Math*` como conteúdo inline entregue ao
layouter matemático; a nova variante exige braço exaustivo, mas realizar `mid`
neste nível duplicaria seu owner especializado.

### Decisão proposta

Encaminhar `Content::MathMid` pelo mesmo caminho inline das variantes math.
Este owner não interpreta classe `Relation`, grupo `lr` nem stretch; a lógica
permanece em `compiler/math/layout/delimited.rs` e no spacing matemático.

## P1439 — encaminhamento geral de `MathOverbrace` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

O dispatcher geral encaminha variantes `Math*` ao layouter matemático; a nova
variante exige braço exaustivo, mas sua realização não pertence a este nível.

### Decisão proposta

Encaminhar `Content::MathOverbrace` pelo caminho inline math. Este owner não
interpreta U+23DE, annotation, classe ou stretch; a lógica permanece em
`compiler/math/layout/underover.rs` e no spacing matemático.

## P1451 — encaminhamento geral de `MathUndershell` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O dispatcher geral exige braço exaustivo para a identidade nova, mas não é
dono da realização geométrica.

### Decisão proposta

Encaminhar `Content::MathUndershell` pelo caminho inline math; U+23E1,
annotation, classe e stretch permanecem nos owners matemáticos.

## P1450 — encaminhamento geral de `MathUnderparen` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O dispatcher geral exige braço exaustivo para a identidade nova, mas não é
dono da realização geométrica.

### Decisão proposta

Encaminhar `Content::MathUnderparen` pelo caminho inline math; U+23DD,
annotation, classe e stretch permanecem nos owners matemáticos.

## P1440 — encaminhamento geral de `MathOverbracket` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O dispatcher geral encaminha variantes `Math*` ao layouter matemático; a nova
variante exige braço exaustivo, mas sua realização não pertence a este nível.

### Decisão proposta

Encaminhar `Content::MathOverbracket` pelo caminho inline math. Este owner não
interpreta U+23B4, annotation, classe ou stretch; a lógica permanece em
`compiler/math/layout/underover.rs` e no spacing matemático.

## P1441 — encaminhamento genérico de `MathOverline` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

Sem braço explícito, a variante nova pode cair em caminho genérico ou ser
confundida com a decoração textual homônima.

### Decisão proposta

Encaminhar `Content::MathOverline` pelo caminho inline math. Este owner não
interpreta cramped, constantes, classe nem posição da regra; a lógica
permanece em `compiler/math/layout/underline.rs` e no spacing matemático.

## P1442 — encaminhamento geral de `MathOverparen` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O dispatcher geral encaminha variantes `Math*` ao layouter matemático; a nova
variante exige braço exaustivo, mas sua realização não pertence a este nível.

### Decisão proposta

Encaminhar `Content::MathOverparen` pelo caminho inline math. Este owner não
interpreta U+23DC, annotation, classe ou stretch; a lógica permanece em
`compiler/math/layout/underover.rs` e no spacing matemático.

## P1443 — encaminhamento geral de `MathOvershell` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O dispatcher geral encaminha variantes `Math*` ao layouter matemático; a nova
variante exige braço exaustivo, mas sua realização não pertence a este nível.

### Decisão proposta

Encaminhar `Content::MathOvershell` pelo caminho inline math. Este owner não
interpreta U+23E0, annotation, classe ou stretch; a lógica permanece em
`compiler/math/layout/underover.rs` e no spacing matemático.

## P1444 — encaminhamento geral de `MathPrimes` (MATERIALIZADO)

### Medição anterior à decisão

O dispatcher geral precisa reconhecer a nova variante math, mas não é dono da
geometria dos glyphs agrupados.

### Decisão proposta

Encaminhar `Content::MathPrimes` pelo caminho inline math. Este owner não
interpreta count, codepoint, sobreposição ou attachment; a realização pertence
a `compiler/math/layout/attach.rs`.
## P1446 — roteamento geral de `MathStretch` (PROPOSTA; gate ADR-0127)

Fora de `Equation`, `MathStretch` segue o mesmo fallback matemático dos demais
carriers. Dentro da equação, somente `MathLayouter` o resolve. O Layouter
geral não interpreta `size`, não estica frames e não duplica a lógica da
feature.

## P1448 — encaminhamento geral de `MathUnderbrace` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O dispatcher geral encaminha variantes `Math*` ao layouter matemático; a nova
variante exige braço exaustivo, mas sua realização não pertence a este nível.

### Decisão proposta

Encaminhar `Content::MathUnderbrace` pelo caminho inline math. Este owner não
interpreta U+23DF, annotation, classe ou stretch; a lógica permanece em
`compiler/math/layout/underover.rs` e no spacing matemático.

## P1449 — encaminhamento geral de `MathUnderbracket` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O dispatcher geral encaminha variantes `Math*` ao layouter matemático; a nova
variante exige braço exaustivo, mas sua realização não pertence a este nível.

### Decisão proposta

Encaminhar `Content::MathUnderbracket` pelo caminho inline math. Este owner não
interpreta U+23B5, annotation, classe ou stretch; a lógica permanece em
`compiler/math/layout/underover.rs` e no spacing matemático.

## P1462 — fachada de opções documentais de formato (MATERIALIZADA)

O descendente privado `format_options` reexporta a função pura
pública `collect_document_format_options` definida e possuída por
`00_nucleo/prompts/compiler/layout/format_options.md`. A fachada não conhece
CLI, exporters ou `World`, não duplica a travessia e não incorpora a lógica ao
dispatcher de render.

A nova entry point L1 é superfície pública entre camadas. A mudança foi aprovada
e materializada em 2026-09-19. Nenhuma fase ou geometria do layout muda neste owner.

## P1481 — idioma documental no resultado paginado (IMPLEMENTADO; confirmado em 2026-09-23)

Medição: o vanilla escreve `/Lang (fr)` para o documento que define
`#set text(lang: "fr")` e pinta somente `rect` (fixture
`p1481-no-text-fr.typ`, SHA-256
`8e1b27cf49c91d2edbe132aa69f893bb554bca7141121fe87a544ed219455b38`).
O frame cristalino não conserva a regra textual depois do layout. O
vanilla sem regra escreve `/Lang (en)`; na fixture de idioma local
inicial `p1481-local-first.typ`, escreve `/Lang (fr)` e marca o texto
seguinte em inglês como override local.

Contrato confirmado: capturar o primeiro idioma efetivo da sequência
documental e transportá-lo em `PagedDocument.document_language`.
Capturar regra top-level mesmo se não houver glifos; conservar overrides
locais em `TextStyle.lang`. O campo público exige aprovação
ADR-0127. A captura ocorre no layout existente: nenhuma decisão de
idioma é movida entre eval e layout, nenhuma fonte externa é consultada.

## P1485.2 — origem de Locations em layout multi-documento (ratificado; ADR-0127)

### Medição anterior à decisão

Na working tree P1485 não commitada sobre
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`,
`layout_with_introspector_and_metrics_and_callbacks`
(`01_core/src/compiler/layout/mod.rs:3327-3450`) recebe o
introspector global por valor e cria `Layouter::new` em cada
iteração (`:3423-3424`); `Layouter.locator` inicia sempre em zero
(`:1108`). O introspector do bundle já conhece os dois headings
em Locations globais distintas. Com `#set heading(numbering: "1.")`,
o vanilla pinado extrai `1. Alpha` de `a.pdf` e `2. Beta` de
`b.pdf`; o cristalino extrai `1. Alpha` e `1. Beta`.
A diferença é semântica da linguagem, não byte de PDF.
Classificação ADR-0107: o texto numerado é semântica da
linguagem; o valor numérico de `Location` e a forma da entrypoint
são mecânica. O vanilla mede a continuidade, mas não legitima
copiar seu `SplitLocator`. Inferência: iniciar cada iteração no
checkpoint do walk global conserva a associação a counters/refs.
Refutação: heading aninhado, fixpoint ou callback produzir Location
distinta da introspecção apesar do seed correto.


### Contrato público ratificado

Adicionar uma entrypoint irmã
`layout_with_introspector_and_metrics_and_callbacks_from` com a
mesma assinatura vigente mais `location_start: LocatorSeed`.
A entrypoint existente delega ao início zero e conserva resultados
e API. A nova instala `Locator::from_seed(location_start)` em
**cada** nova iteração de `Layouter` antes de `layout_content`,
incluindo fixpoint, replay e callbacks; não avança artificialmente
o conteúdo, não insere marcadores invisíveis e não altera a ordem
de páginas. O seed é fornecido pela entrypoint proprietária de
`compiler/introspect.md`, nunca por contagem ad hoc em L3.

Aceitação: dois bodies paginados e um body HTML intercalado mantêm
Locations/numeração globais; layout individual pela API antiga é
idêntico ao anterior; fixpoint, labels, refs, counters, outline,
equações e contextos não observam colisões nem offsets trocados.
Repetir o mesmo body/seed no mesmo fixpoint é determinístico.
A assinatura pública L1 teve o gate ADR-0127 confirmado
pelo dono em 2026-09-24.
## P1486 — número lógico inicial da página

### Medição anterior à decisão

`01_core/src/compiler/layout/mod.rs:2650-2665` fecha a última
`Page` após calcular `page_number = self.pages.len() + 1`.
`entities/layout_types.md` P1486 introduz o campo público já
confirmado em 2026-09-24; a realização posterior do número lógico
pertence a `infra/pipeline.md` P1486. Número mostrado no leitor PDF é
observável; a inicialização física antes do snapshot é mecânica.

### Decisão

A página final nasce com `logical_number: page_number`. L1 não
consulta o `PageStore` realizado nem antecipa `counter(page).update`;
L3 substitui pelo número lógico validado antes do exportador.
Nenhuma mudança de geometria, texto desenhado ou fase de layout.

## P1486.1 — margem automática usa o número lógico realizado

### Medição anterior à decisão

`00_nucleo/diagnosticos/typst-p1486-page-labels.md` registra a sonda
bilateral: com `counter(page).update(10)` e seleção 1+3, ambos os leitores
mostram rótulos PDF `10, 12`, mas `pdftotext` extrai margem `1, 3` no
cristalino e `10, 12` no vanilla pinado. Neste owner, `finish` usa
`page_number = self.pages.len() + 1` para `format_counter` e para o
pending de padrões compostos (`01_core/src/compiler/layout/mod.rs:2516-2584,
2670-2695`), ignorando `runtime.known_page_store.logical_number_for_page`.
O texto da margem é observável de linguagem; o momento de injeção da store
é mecânica. A inferência de que a store final corrige a margem sozinha foi
refutada pela sonda. Uma página sem store realizada deve manter o fallback
físico; divergência de paginação na passagem final refutaria um snapshot
anterior não alinhado.

### Decisão

Ao desenhar a margem automática de `Numbering::Pattern`, usar o número
lógico da `known_page_store` para a página local quando disponível, e o
físico apenas antes da realização. Para padrões compostos, preservar o
adiamento mas usar o número lógico da página e o último número lógico
realizado como total; sem store, conservar a fórmula física anterior.
Recalcular a largura e a posição a partir do texto novo, sem substituir
camadas explícitas. A inicialização física de `Page.logical_number` é
provisória e continua inalterada. Esta correção de paridade supersede só
a cláusula histórica acima que excluía mudança do texto desenhado; não
introduz API pública, modo novo nem outra fase de layout.


## P1491 — geometria causal de baseline para `measure`

### Medição anterior à decisão

Em 2026-09-24T14:12:42-03:00, no HEAD
`b169a78c6c6bc17764b294f501ac09b63d678b03`, a redução vigente de
`measure_content_real` perde a baseline e, quando há texto, substitui a altura
do subframe pela aresta textual. O vanilla pinado mede: rect 10×20 → baseline
20; texto → baseline igual à altura; box 20 com shift 3 → baseline 17; vazio →
zero; composto texto+box → altura 20 e baseline 17. HTML conserva essas
relações. Isso refuta `baseline = height` e a altura escolhida só por
`plain_text()`.

### Decisão proprietária

Manter `pub fn measure_content_real(...) -> (f64, f64)` sem alteração. Um
helper de visibilidade interna a `compiler` reutiliza exatamente o mesmo
Layouter isolado e devolve também a baseline. A altura/baseline vêm das
extensões da linha composta: arestas tipográficas reais para texto e
`line_inline_ascent`/`line_inline_descent` para frames inline. Conteúdo sem
baseline natural usa a borda inferior do frame; vazio usa zero. O par público
legado delega ao helper e descarta apenas baseline.

Não inferir pelo variant de `Content`, não assar 20pt, não criar campo em
`Frame`, `Region`, entidade ou trait, e não mover fase. Métricas continuam
injetadas. Width, sombra, diagnóstico e rejeição de `measure(width:/height:)`
permanecem. O owner `boxed.md` torna o shift já transportado parte das
extensões inline; este owner apenas extrai a geometria resultante.

## P1493 — fachada intrínseca com posições HTML (RATIFICADO; gate ADR-0127 confirmado em 2026-09-24)

### Medição anterior à decisão

A fachada `layout_html_frame` devolve somente `Page`; a posição dos markers
causais é descartada antes de L3 poder estabilizar leituras. Alterar a função
existente quebraria callers P1424.

### Contrato ratificado

Conservar `layout_html_frame -> Page` como fachada de compatibilidade.
Adicionar `layout_html_frame_detailed(...) -> HtmlFrameLayout`, com campos
públicos `page: Page` e `positions: Vec<(u64, HtmlPosition)>`. A chave `u64`
é o id causal do `ContextBlock` que executa `here()`. A sequência é
determinística e produzida pelo owner descendente. A fachada não conhece
pipeline, World, exportador ou target externo e não executa Func.

A Page intrínseca pode conservar `logical_number: 1` apenas como mecânica de
layout; esse número não integra `HtmlPosition` nem qualquer resposta da
linguagem. Carriers geométricos privados são removidos antes da Page pública e
não alteram itens, dimensões nem SVG. A adição da fachada pública entre L1 e L3
foi confirmada pelo dono em 2026-09-24.

## P1493 — neutralidade de fluxo após revisão adversarial

Medição posterior ao gate: presença física do carrier em `current_line` fazia
decisões cruas de vazio tratarem um `ContextBlock` no-op como conteúdo;
espaço, parbreak, flush e transições de container podiam criar avanço
fantasma.

Em toda decisão de fluxo, somente o carrier privado direto é imaterial. Ele
continua na árvore para receber a geometria, mas não satisfaz predicados de
linha ou região com conteúdo. Wrappers preexistentes preservam exatamente a
materialidade que possuem na fachada legada; conter apenas o carrier não os
torna vazios. Fora da passagem
detalhada não existe carrier e o comportamento permanece idêntico.
