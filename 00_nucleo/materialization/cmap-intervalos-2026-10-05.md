# Passo — cobertura cmap por intervalos

Este documento coordena a execução; não é Prompt L0.

## Concluído

Experimento independente em `00_nucleo/diagnosticos/cmap-intervalos-2026-10-05/probe`, duas versões medidas com recibos integrais, equivalência nas fontes instaladas e sete controles sintéticos em debug/release. Relatório: `00_nucleo/diagnosticos/cmap-intervalos-2026-10-05.md`. Código produtivo e binário da CLI preservados.

## Gate pendente

L0 proprietário `00_nucleo/prompts/entities/font_book.md`, SHA-256 `1ef2d9719a79f848d473e074ecee3fea8f79c2bcd56e87c957f70bf8abcd5e9d`, preparado para acrescentar `Coverage::from_ranges`. Ratificação do dono obrigatória pela ADR-0127 (contrato público). Não materializar ou ressellar o consumer antes da resposta.

## Após ratificação

1. Teste produtivo com oráculo independente; demonstrar RED por comportamento, depois GREEN.
2. Materializar união pura no owner `01_core/src/entities/font_book.rs`.
3. Redigir L0 exclusivo `00_nucleo/prompts/infra/fonts/coverage.md` para eventual adapter privado `03_infra/src/fonts/coverage.rs`, seguindo ADR-0129. Atualizar o L0 do pai antes da delegação. Preservar fallback legado de face quando um limite exclusivo não cabe em u32.
4. Implementar/testar adapter; não colocar parser externo ou I/O em L1.
5. Validar build e lint; registrar limites preexistentes separadamente; medir CLI pareada e controles de seleção de fonte, texto, geometria e PDF.
6. Atualizar relatório com proveniência integral e ganho real, inclusive resultado negativo se a hipótese não se confirmar.
