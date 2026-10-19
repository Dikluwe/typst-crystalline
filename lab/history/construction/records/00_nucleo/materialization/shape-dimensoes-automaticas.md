# Passo de execução — shape-dimensoes-automaticas

Este documento é plano de continuidade, não Prompt L0 nem autorização para código.

Completar o subconjunto explicitamente pendente em
00_nucleo/prompts/compiler/layout/shape_block_behaviour.md: dimensões omitidas/Auto,
identidade de rect/ellipse/circle/square e validação dos argumentos.

A medição anterior está em
00_nucleo/diagnosticos/transformacoes-2026-10-14/shape-dim-before-complete/results.json.gz,
com HEAD/árvore/UTC/fontes/binários. Defaults de rect, ellipse e circle divergem.
shape.rs usa fallback de largura disponível/altura zero; circle e ellipse
compartilham ShapeKind::Ellipse e None/None, perdendo distinção necessária
para dimensões automáticas. Não substituir globalmente esse fallback sem
resolver a identidade: dar45×30 a todo Ellipse produziria círculo oval.

Antes de implementar: medir fields/func/set/show/casts, raio em em/percentual,
size e body, limites regionais, dimensões omitidas versus Auto; auditar owners
de entities/shape, stdlib/shapes/rect e ellipse, observação e layout. Redigir
os respectivos L0s. Se a decisão exigir contrato público ou mover resolução
entre eval e layout, preparar proposta concreta e aplicar ADR-0127 antes do
código. Correções internas que já preservem esses contratos seguem em fluxo.

Aceitação: semântica, morfologia e geometria, testes RED→GREEN, corpus bilateral,
linhagem, suíte e CI. Publicação não faz parte deste passo.
