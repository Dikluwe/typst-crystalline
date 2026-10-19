# Proposta de correção na base da composição de notas

Esta proposta parte das sondas e da proveniência de `results.json`, medidas
contra o vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8` e o
cristalino compilado indicado naquele arquivo. A árvore atual pode diferir
desse binário; repetir RED antes de decidir que uma mudança fechou o defeito.

## Causa observável e fronteira correta

Em `defer.typ`, o vanilla põe a linha da chamada e o início do corpo na
segunda página; o cristalino conserva a chamada na primeira e só adia o
corpo. `01_core/src/compiler/layout/footnote.rs:47-51` emite o marcador antes
de enfileirar o corpo. `01_core/src/compiler/layout/footnote_flush.rs:128-169`
decide cabimento só quando a região fecha. Nesse momento já não existe uma
decisão local capaz de mover a linha completa que contém o marcador. Essa é
uma inferência causal da fonte e da sonda, não atribuição de intenção ao
upstream.

Em `oversize.typ`, o vanilla conserva as 21 linhas em duas páginas, e a
extração do cristalino só recupera as primeiras 11. O flush cristalino chama
`layout_sub_frame` com `height: None` e `unconstrained_height: true` em
`footnote_flush.rs:140-149`, então força o frame único maior que a página em
`:153-166`. `sub_frame.rs:204-300` devolve um único conjunto de itens e
restaura o estado do pai; fornecer uma altura finita por si só não cria
fragmentos nem estado de continuação. O vanilla usa região limitada, migra a
origem quando o primeiro fragmento não cabe e mantém os fragmentos seguintes
em fila (`lab/typst-original/crates/typst-layout/src/flow/compose.rs:659-731`).

## Contratos L0 a revisar antes da implementação

1. `prompts/compiler/layout/footnote.md`: o marcador e o corpo passam a ser
   registrados como uma ocorrência causal; a emissão inline não pode assumir
   admissão irrevogável da linha de origem. A identidade/numeração não se
   repetem na continuação.
2. `prompts/compiler/footnote_overflow_columns.md`: trocar o fallback que
   força corpo inteiro por admissão do primeiro fragmento e fila privada de
   continuação. Uma região deve consumir conteúdo ou avançar para outra;
   conteúdo remanescente jamais deve desaparecer para obter terminação.
3. `prompts/compiler/layout/line_commit.md` e
   `prompts/compiler/layout/region_progress.md`: a decisão de primeira região
   ocorre antes de confirmar a linha portadora. Se a primeira parte da nota
   não cabe, mover a linha de origem inteira, preservando seus itens, estilos,
   posições causais e ordem. Não mover apenas o marcador no meio da linha.
4. `prompts/compiler/layout/sub_frame.md`: a API atual de subframe único deve
   permanecer estável para os outros consumers. Um percurso privado e
   limitado para fragmentos deve devolver parte emitível, resto e prova de
   progresso, sem deixar páginas/itens do ensaio no pai.
5. `prompts/compiler/layout.md` e `prompts/compiler/columns.md`: registrar
   apenas a coordenação de estado, rollback, terminal e transição por região;
   manter a política de fragmentação no owner do flush. Testar a página
   física e a coluna, inclusive a ordem de várias notas.

O desenho evita campo público em `entities`, assinatura pública nova, mudança
de fase e novo default deliberado. Assim, a correção de paridade pode seguir
em fluxo contínuo pela ADR-0127: editar cada L0 antes do respectivo consumer,
resselar apenas os owners afetados, observar RED→GREEN e revalidar. Se a
implementação provar necessário um carrier público, um método de trait
público, outra fase do pipeline ou incompatibilidade, apresentar esse
contrato ao dono e parar antes do código correspondente.

## Sequência de refutação

Primeiro reproduzir `fit`, `defer` e `oversize` no estado atual, incluindo
páginas e extração de todas as linhas. Corrigir a admissão da linha de origem
com `defer` RED→GREEN, mantendo `fit` e notas em colunas. Depois acrescentar
fragmentação com `oversize` RED→GREEN, exigindo linhas 1–21 em ordem, nenhuma
duplicação e progresso quando a primeira região está vazia. Cobrir nota
seguida de outra nota, corpo com estilos/links e nota maior que uma coluna.
Verificar texto, páginas e morfologia conforme ADR-0107; bytes PDF e forma
interna do algoritmo não são critérios de paridade.

## Refutação do protótipo local — 2026-09-30

Foi tentada uma pré-medição no owner `footnote.rs` e a migração do buffer
`current_line` antes de emitir o marcador. Um teste direto do Layouter com
`cursor_y` artificial ficou RED e depois GREEN; esse GREEN era insuficiente.
O CLI debug recompilado com o protótipo continuou a produzir, para o
`defer.typ` original, `Alpha1 omega.` na página 1 e o corpo apenas na página
2. O problema real atravessa a composição prévia do parágrafo: a linha já
estava confirmada fora de `current_line` quando a unidade de footnote tentou
movê-la. O protótipo e seu teste artificial foram removidos do código.

Uma sonda adicional `multi-note.typ` com duas notas na mesma linha e pressão
regional mostrou segunda chamada na página 1 e segundo corpo na página 2
em ambos os binários. Logo, a co-localização vanilla observada em `defer` não
pode ser generalizada sem verificar as condições de migração da primeira nota.
`prototipo-refutado.json` registra hora, HEAD, SHA-256 dos binários e sondas,
páginas e texto. O binário debug foi compilado antes da reversão do protótipo;
o snapshot de árvore adjacente é posterior a ela. Esta medição refuta a
abordagem local, mas não serve para fechar paridade da árvore final.

A próxima correção precisa decidir a admissão da linha portadora na unidade
que ainda a detém, coordenada com o owner de transição regional; requer L0
próprio desses owners antes de novo código. Não basta pré-medir no braço
`Footnote` nem mudar só o flush do corpo.
