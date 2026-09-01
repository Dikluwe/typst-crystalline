# Prompt L0 — motor geral de layout
Hash do Código: 2ccc7ac0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4bf17f1455eef032ab3e30ea038edabed721e8378b913aaecf2b544bf288a917

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/mod.rs`
**Vanilla ratificado:** `a51e02804`
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
