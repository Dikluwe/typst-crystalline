# Passo 1292 — relatório final

**Estado:** `CLOSED / APPROVED`
**HEAD verificado:** `0eb39f8ecb48930515f2cadb6a378450855b5a72`
**Alvo:** vanilla ratificado `a51e02804`.

## Resultado

Os quatro resíduos acionáveis da superfície padrão foram fechados:

1. `math.cancel` expõe o construtor canônico e preserva payload, presença
   morfológica, callback, span e geometria P1291;
2. `math.underline` é entidade matemática própria e deriva geometria das
   constantes MATH;
3. `math.vec` converge sintaxe e função, aceita cardinalidade zero e resolve
   `gap` contra a região efetiva, neutralizando apenas o percentual em altura
   infinita;
4. `place.flush` é membro persistente do namespace de `place`, inclusive por
   `.with`, e realiza uma transação de floats anteriores sem antecipar os
   posteriores.

A correção D preserva a engenharia vigente: `Content` fechado, dispatcher
estático e exaustivo, atomização forma B, owners Block/Place/Cursor separados,
zero I/O em L1 e ownership L0↔consumer 1:1. O default vanilla de clearance
continua explicitamente `1.5em`: é um valor semântico fixo no código vanilla,
mas relativo ao `em` efetivo, não uma distância física fixa em pt.

## Evidência

- contrato v14 canônico `07158762...`, seal `fb7e5271...`, 27/27 L0s;
- oráculo protegido inalterado `fa8f8770...`, 11/11;
- testes independentes P1292 17/17;
- campanha adversarial 23/23, score 1.0, sobreviventes 0, `Unknown=0`;
- superfície default 111 total, 99 MATCH, 12 justificadas, zero perdas dos 95;
- superfície HTML 115 total, 89 MATCH, 26 justificadas no inventário próprio,
  `Unknown=0`;
- build release, workspace, formato, lint, V5/V7/V15/V26 e diff-check verdes;
- regressões P1288–P1291 preservadas nos limites de seus contratos.

## Histórico causal relevante

A primeira verificação global reabriu P1030 porque um helper test-only só
reconhecia `MathMatrix`; o autor independente corrigiu o observador para
reconhecer `MathVec`, sem tocar produto ou expectativa. Em seguida, a auditoria
de preservação detectou que `--fix-hashes` havia alterado a linha
`Hash do Código` de dois L0s após o seal v12. O v12 foi invalidado; o autor
contratual provou por reconstrução que nenhum byte normativo mudou e emitiu o
v13. Depois, o pre-commit removeu uma linha vazia adicional do EOF de
`entities/elements/flush.md`; a reconstrução `current + LF` reproduziu o pin
v13 e os outros 26 L0s permaneceram idênticos. O v14 ressellou exclusivamente
esse whitespace e sincronizou o header do consumer. Só depois os gates focais
foram novamente executados.

## Limites

O protocolo foi executado com segregação de papéis, capacidades, ordem e
artefatos, mas sem isolamento técnico do filesystem compartilhado. O
certificado cobre somente os fragmentos A–D e não declara paridade global.
