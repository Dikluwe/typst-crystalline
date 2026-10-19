# L0 — compiler/eval/rules/outline — Outlines sintetizados
Hash do Código: abc6aa74

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/outline.rs`

## Medição antes da decisão

A working tree não commitada capturada em `lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz` contém as rotinas desta família em `01_core/src/compiler/eval/rules.rs`. A suíte L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem prova bilateral nova contra o vanilla. As medidas históricas e ressalvas do pin continuam em `00_nucleo/prompts/core/compiler/eval/rules.md`.

## Contrato de linguagem

Realizar entradas de outline com snapshots causais e estilos do parent, sem reexecutar callbacks em layout, sem novos nós de query e preservando ordem/forma da linguagem.

As assinaturas, identidades, casts, defaults, precedência de erros e fase de avaliação observados permanecem. O parent mantém diagnóstico compartilhado e bindings estáticos; outros owners de selector matching, terminação e textual realization continuam exclusivos. Não duplicar esses algoritmos.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. Paridade em semântica, sintaxe e morfologia (ADR-0107/0108), ownership exclusivo e pins válidos (ADR-0129/0130).


## Realização com avaliação única

Realização pós-introspecção de outline reconhece Outline dentro de Located e conserva sua Location ao substituir seu corpo realizado. Snapshot lexical de regras continua no elemento Outline.


## Reuso sem transformação — 2026-10-05

A inspeção em `01_core/src/compiler/eval/rules/outline.rs:20–46` demonstra que map_content reconstrói carriers mesmo quando nenhuma receita de outline de headings é aplicada. A função pública mantém assinatura e fases: registrar internamente se aplicou realize_heading_outline e, se nenhuma aplicação ocorreu, devolver o Content de entrada conservando seus Arcs. Não usar igualdade visual para decidir ausência de transformação. Se alguma aplicação ocorreu, devolver o resultado transformado mesmo quando seu visual seja igual. Callbacks, erros e warnings do caminho com outline permanecem vigentes. Exigir RED→GREEN para conservação do carrier sem outline e revalidar outlines reais.

## Prova de ausência por leitura emprestada — 2026-10-09

Medição anterior à decisão: `lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/stacks-deep.json.gz`
guarda binário, HEAD/working tree, fonte, comandos e UTC. Uma em cada 64
alocações foi amostrada; `TableElem::map_content` sob `realize_outlines`
aparece em 451 amostras de `local-set-1600` e 150 no controle sem set.


`outline-scan-red-gate.json.gz` registra 899.619 pedidos para o caso local
no release anterior, com estado e hora. A fonte `outline.rs:14-47` devolve o
Content original se nenhum Outline headings foi transformado, mas antes
materializa toda a travessia `map_content`. Inferência refutável: provar por
leitura que não há alvo evita a cópia sem alterar uma única aplicação.

Antes do mapeamento transformador, percorrer por empréstimo Source, Located,
Realized, ShowRecipe, Styled, Sequence, Table e TableCell. Located com body
imediato Outline headings é alvo; o callback no body raiz permanece suprimido
como em `map_content_inner`, e o body ainda é visitado para descendentes.
Realized visita apenas visual. Table e Cell seguem a projeção visual de
`map_content`; demais variantes usam fallback exaustivo via `map_content`.
Erro no fallback invalida a prova de ausência e encaminha o conteúdo ao
caminho transformador original, que conserva seu diagnóstico.
Retornar cedo o Content original somente quando a ausência de Outline
headings foi comprovada. Em presença de alvo, executar exatamente o caminho
transformador anterior, preservando Location, ordem, callbacks, erros e
produto. Sem assinatura, contrato público, fase, default ou compatibilidade
nova. Aceitação: teste de prova positiva/negativa em invólucros e tabela,
mesma saída bilateral e redução medida de alocações/tempo. Fluxo contínuo
ADR-0127.
