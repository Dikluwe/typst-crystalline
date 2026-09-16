# Prompt L0 — motor geral de layout
Hash do Código: ccd99ecb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4bf17f1455eef032ab3e30ea038edabed721e8378b913aaecf2b544bf288a917

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/mod.rs`
**Vanila ratificado:** `586e1bd43`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

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
fronteiras existentes. O recibo P1292 demonstra que o marcador deve realizar
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
selados.

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

O selo/runtime P1291 já foi materializado. P1292 preserva integralmente esta
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

## P1391 — arm interno de fechamento overlong certificado

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

Não há nova API pública, fase ou default. Ausência do certificado, prefixo
incompleto, outra célula de tamanho e fluxo ordinário nunca armam o estado.
11→39, 39→11, 11→40 e 40→11 devem manter uma página e `After=Tail`;
40→8 permanece opaco.

## P1392 — reutilização do arm por certificado cruzado

### Medição anterior à decisão

Em 40→8 e 40→7, a palavra inicial é overlong e fecha dentro do layout de
texto, pela mesma causalidade P1391. O vanila prevê a linha seguinte pelo
estilo exterior; sem o arm, o cristalino usa 40pt e pagina a cauda.

### Decisão interna

O mesmo booleano privado one-shot pode ser armado também depois de Sequence
certificar a forma P1392 completa. Inicialização, consumo, checkpoint e
isolamento permanecem idênticos a P1391; não se adiciona campo, API ou fase.
Formas não certificadas nunca armam o estado. P1391 e os controles de replay
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
separa medição pura e realização L3 por transcript selado. No baseline
`0a7de3dd9`, a entry point callback-aware cobre apenas math.

### Decisão proposta

Generalizar a entry point explícita de passagem para receber stores seladas de
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
