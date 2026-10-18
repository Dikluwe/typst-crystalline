# Passo de execução — recuo francês

Este documento coordena tarefas. **Não é L0 e não legitima código.**

Objetivo integral permanece Faça todas as correções. Esta etapa completa a
aplicação física de hanging-indent em parágrafos, com bibliografia e controles.
Frentes E01–E20 e contratos públicos pendentes não são encerrados por ela.

1. Medições em bibliography-hanging-2026-10-18:92 pares principais,26 extras
   corrigidos e10 de fronteiras, mais4 de repetição. A primeira coleta lexical
   tinha `[...]` literal, corrigida para `#[...]` antes de qualquer decisão.
2. L0s vigentes e hashes em owner-audit-before.json. Atualizar L0s e núcleo de
   seleção antes de teste/código; V15/V26 antes do resselo e implementação.
3. Separar comprimento lexical, contexto Par vs fragmento inline, offsets da
   primeira linha/continuação e classe causal de set. Não aplicar recuo a um
   bloco inline isolado, nem inferir h por fonte corrente de um run parcial.
4. Unidade privada paragraph_indent: Spec de offsets inicial/continuado e Dir,
   State local de escopo/linha, Shape(first, continuation) puro para seletores.
   Não mudar line_start_x/width globais para representar o recuo. Aplicar ao
   cursor uma vez depois da admissão; limites de wrap/RTL/justificação consultam
   a mesma forma. Nova região reabre aplicação da linha sem reiniciar parágrafo.
5. Bibliografia usa Block com inset inicial h e compensação privada da primeira
   linha -h; fila pendente consumida na admissão real, inclusive no caminho
   preparado. Sem HSpace que force fallback, conteúdo/Location falsos ou API
   pública. Default1.5em apenas Hanging; Columns/Plain preservam sua geometria.
   Override só por show-set que casou Bibliography, não set par ambiente.
6. Declarações de par.hanging interrompem o escopo mesmo com zero, repetidas ou
   com corpo vazio. Transportar ocorrência por canal privado de presença local
   se diff_styles apagar valores iguais; não reevaluar argumentos/callbacks.
7. Refatorar seletores com forma inicial/continuação para simple/optimized e
   hifenização. Guardas existentes continuam; expansão adicional RTL latina só
   sem BidiCursor ativo/tracking, identidade lógica None sem prova, outros
   scripts e contextos permanecem explicitamente fora da seleção global.
8. Testes RED funcionais antes do código, cobertura de sinais/abs+em,
   fronteiras/escopos/containers, estratégias e cópia Unicode, checkpoint,
   subframe, regiões e overrides. Comparar com dados anteriores; não corrigir
   métricas mistas com offset nem tratar recorte PDF extremo como perda lógica.
9. Validação sequencial sob12GiB/High8GiB/swap0/jobs1/debug0/incremental0.
   Oracle BD5 SHA9b2373 e linter CI1279707 explícitos. TMPDIR e TYPST_VANILLA
   obrigatórios para os testes de integração. Sem commit/push/release.

R3-T1 continua pendente da aprovação já solicitada. Stored Content alias e
outras receitas podem depender dessa frente; não mudar ordem de realização.
Metadata-only owners do núcleo recebem somente novo pin e header de linhagem.
