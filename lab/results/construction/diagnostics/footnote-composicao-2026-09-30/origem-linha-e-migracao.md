# Origem da linha de chamada e migração da nota

Auditoria de fonte, sem alteração de L0 ou código. Estado observado em
2026-09-30T11:43:34Z: HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`,
working tree não commitado. `git diff HEAD --stat` nessa hora abrangia 781
arquivos (19.673 inserções, 118.497 remoções); a pasta deste diagnóstico era
não rastreada. Os números das sondas pertencem aos snapshots e binários
registrados em `results.json` e `prototipo-refutado.json`, não ao HEAD sozinho.

## Onde a decisão se perde no cristalino

`paragraph_prepare.rs:200-222` prepara texto, espaços e estilos, mas retorna
`None` para `Footnote`. Assim, `paragraph_plan.rs:74-86,94-130` não consegue
selecionar o parágrafo completo que contém a chamada. A sequência recua para
`sequence.rs:761-769,827-829,947-958`, que entrega as partes uma a uma; o
`Content::Par` faz o mesmo recuo em `paragraph.rs:41-48`. Um `Text` individual
ainda pode passar por seleção própria em `text.rs:47-53`; na via incremental,
`word_layout.rs:45-64` e `space.rs:41-55` podem fechar uma linha antes de a
ocorrência seguinte chegar. Isso impede assumir que a linha portadora ainda
está integralmente em `current_line` quando `Footnote` é visitado.

`footnote.rs:38-51` resolve o número, emite o marcador inline e só então põe o
corpo em `pending_footnote_bodies`. Não mede nem admite a primeira região da
nota. `line_commit.rs:413-448` migra linha apenas quando sua própria tinta
excede o limite efetivo; `:681-690` drena `current_line` para `current_items`.
O corpo da nota é medido apenas em `footnote_flush.rs:91-100,122-170`, durante
o fechamento da região (`region_progress.rs:57-66`). A essa altura a decisão
de adiar só o corpo não pode, por si, migrar a linha de origem. A sonda
`defer.typ` confirma a separação de página (`relatorio.md:16-21`).

O **primeiro ponto correto de decisão** é a admissão da linha portadora,
antes do drain em `line_commit.rs:688-690`, com a ocorrência da nota já
associada causalmente à linha e a primeira parte do corpo medida contra a
região restante. Para textos que a seleção fecha antecipadamente, a associação
precisa começar antes em `paragraph_plan.rs:74-86,94-130` / no recuo de
`sequence.rs:761-769`: a unidade de replay deve abranger todo o conteúdo da
linha de origem. Um teste isolado que move só `current_line` em `footnote.rs`
não prova isso. O protótipo descrito em `proposta-base.md:72-81` ficou verde no
Layouter artificial, mas a compilação de `defer.typ` continuou separando a
chamada e o corpo; ele foi removido. Esta observação refuta aquele ponto local,
não prova que toda chamada passe pela mesma rota antecipada.

O `FlowSuffixCheckpoint` existente (`layout/mod.rs:728-812,1154-1205`) já
registra geometria da região, páginas, locator, runtime, numeração e o
comprimento da fila de notas; a restauração em `:1248-1306` trunca a cauda.
Pode servir de padrão para um ensaio iniciado **antes** da produção da linha,
quando os itens de origem ainda serão apenas acrescentados. Porém
`RegionTailCheckpoint::restore` (`:815-851`) não recria itens retirados de
`current_line` por um drain: se a linha existia antes do checkpoint e foi
comitada durante o ensaio, apenas truncar não a restaura. A transação de
`flow_suffix.rs:30-90` também repete ocorrências de conteúdo posteriores ao
seu próprio checkpoint; não representa automaticamente o prefixo `Alpha` da
linha da chamada. Uma correção deve ou começar o checkpoint antes de gerar
toda a linha e guardar seu conteúdo para replay, ou conservar uma linha/frame
de origem movível com os seus efeitos laterais correspondentes. Em ambos os
casos, rollback deve manter itens, estilos, posições/locator, contador,
decorações e ordem das notas; a fila não pode reemitir o mesmo corpo duas
vezes. Isso é requisito inferido da mutabilidade observada, não proposta de
novo contrato público.

## Condições precisas no vanilla pinado

No vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, o distribuidor
processa as notas **antes** de colocar o frame de origem em `items`
(`lab/typst-original/crates/typst-layout/src/flow/distribute.rs:502-529`).
`compose.rs:585-600` só oferece migração ao primeiro marcador encontrado no
frame e a proíbe se o frame for breakable. `compose.rs:659-675` cria uma região
de nota limitada, descontando o fluxo e o separador. `:678-715` pede migração
se (1) o primeiro frame da nota está vazio, (2) existe um frame posterior não
vazio, (3) a migração foi permitida e (4) `regions.may_progress()` é verdadeiro.
Se migrar não é possível mas uma página seguinte pode melhorar a situação
(`may_progress` ou `flow_need` não zero), enfileira apenas a nota. Caso contrário
prossegue para evitar espera sem progresso. `:717-731` guarda o primeiro frame
e os fragmentos restantes quando há admissão.

Por isso a co-localização em `defer.typ` não é regra incondicional do vanilla.
Em `multi-note.typ`, `prototipo-refutado.json:14-22` mostra segunda chamada na
primeira página e segundo corpo na segunda também no vanilla. O comportamento
relevante para aceitação é preservar a condição da **primeira** nota migrável,
sem prometer migração da segunda ou de origem breakable. A implementação do
cristalino pode diferir da mecânica do vanilla, desde que a semântica observável
seja medida nesses casos e nos casos sem progresso.
