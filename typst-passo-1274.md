# P1274 — promover somente os pares aprovados pelo dono

**Estado:** BLOQUEADO ATÉ APROVAÇÃO P1273
**Predecessor:** gate ADR-0127 P1273 confirmado
**Regime:** implementação produtiva L0-first já aprovada

## Objetivo

Aplicar a promoção exatamente autorizada, sem expandir por similaridade para
outros espaços ou geometrias.

## Execução

1. Confirmar que o L0 aprovado e todos os artefatos P1272 mantêm os hashes do
   gate.
2. Escrever primeiro testes públicos RED para o caminho produtivo de cada par:
   servidor nativo resolvido, marcador de fallback ausente e observáveis do
   envelope preservados.
3. Alterar somente o predicado produtivo proprietário da promoção e os testes
   de fallback diretamente afetados.
4. Manter todos os pares fora da aprovação com o marcador explícito vigente.
5. Atualizar `@prompt-hash`/`Hash do Código` pelo fluxo Tekt válido.
6. Executar envelope, controles, workspace e lint; nenhuma falha pode ser
   convertida em mudança de contrato nesta fase.

## Limite

Este passo implementa a decisão; não emite o veredito independente final.
