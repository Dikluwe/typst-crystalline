# P1324 — revisão instrumental e fronteira dos positivos

Revisor `/root/p1324_review`, 2026-09-09; working tree de HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`. Evidência está em
`p1324-ab-baseline-v2.json`, SHA-256
`f5005f734d917fb68e808ea527a750d2bcc70d50d955ad3092c775d9554b6e03`,
com UTC, argv, diff/stat e canais integrais. O baseline pinado precede C.

V2 corrige a opção CLI `--diagnostic-format human`, não aceita pela CLI
cristalina. O default humano com NO_COLOR conserva os observáveis vanilla;
exit de uso não conta como divergência semântica. Os artefatos V1 permanecem
e o novo freeze é anterior a C. Revisão instrumental legítima.

O baseline V2 mostra que `table-present-call` e `with-present-call` resolvem
o membro, chamam a função e então falham no segundo acesso `.body` ao
Content retornado, com `table.cell does not have field "body"`. O L0 P1324
preserva Content e limita a nova paridade a fields de nativas com namespace;
exigir corrigir esse segundo acesso ampliaria o produto além da obrigação.

Aprovo a sucessão delimitada proposta para V3: conservar os dois literais
originais como controles baseline=C, sem crédito de paridade; acrescentar
positivos que observem tipo/nome de cell e `type(cell[ok])`, incluindo With,
para provar lookup, kind, identidade pública e execução da chamada. Os novos
positivos devem ser medidos no oracle e baseline antes de execução de C, com
nenhuma outra mudança em casos, comparador ou política de Unknown. As formas
negativas Some, traces e campos ausentes permanecem exigindo vanilla.

Este parecer usa apenas L0 e resultados baseline/oracle para julgar a revisão.
O root informou que C já existia quando a insuficiência dos positivos foi
descoberta; o autor dos testes declara não ter lido fonte/diff ou observado C.
Logo V3 é posterior à existência de C e não deve ser descrita como integralmente
congelada antes de C. É revisão cega de escopo observável dentro do regime A/B
sem atestação de isolamento, sem selo de refinamento. V2 vermelho permanece
e sua causa não é apagada. Não repetir RED unitário sem mudança do teste/L0.

As revisões têm causas diferentes: adaptação de CLI e contaminação do positivo
por operação Content. O limiar de duas tentativas sem ganho na mesma causa
não foi atingido. Se novos positivos falharem por outra operação fora de
escopo, reavaliar o desenho antes de nova execução integral.
