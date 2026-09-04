# P1301 — calibração revisão 1: contrato não selável para o candidato

## Resultado

`REOPEN_CONTRACT_HARNESS_SURFACE`

O filtro interno P1301 passou `8/8` após a primeira implementação, mas o gate
público não chegou a observar semântica: todas as `46` execuções foram
rejeitadas pelo parser de CLI cristalino (exit `2`). O contrato v1 congelou
`--diagnostic-format short` e, nos perfis HTML, `--target html`; essas opções
existem no vanilla usado pelo autor do contrato, mas não em `typst eval` do
cristalino.

Isso é `HARNESS_UNSUPPORTED_CANDIDATE_CLI`, não witness contra a correção de
`field_access`. Adaptar silenciosamente o runner depois do selo violaria a
cadeia causal. Por isso o delta P1301 de `field_access.rs` e `eval/tests.rs`
foi removido seletivamente antes da revisão 2; as alterações anteriores P1300
foram preservadas.

## Evidência e causa

- contrato v1: SHA-256 `2d8d7a141d63c13473681abdefa24ed9370e03925824a9cae74ca26305ea85f5`;
- selo v1: SHA-256 `658fe879eaf7dce5b0a6372f686e376e7cc652f621d298c817628b03e16f82fb`;
- candidato transitório: SHA-256
  `ab61ce31cc6eac3ca672608643d312989c597bc3698d3459f966f0fd11304909`;
- runner P7 v1: SHA-256
  `30989db6d6fdbf8f2cb8c4cdc8c9693d91c26d72c23ff5fb5657cef3ab63b5de`;
- resultado: `0 Preserved / 23 Violated` em cada ordem, todos por opção CLI
  rejeitada antes de avaliar a expressão;
- `02_shell/src/cli.rs:334-345` mede a superfície cristalina comum:
  `eval <expression> --format <json|yaml|raw> --features <...>`; não há
  `--diagnostic-format` nem `--target` em `EvalArgs`.

É inferência que um adaptador bilateral baseado na superfície comum
`eval <expression> --format json [--features ...]`, comparando envelopes
normalizados de diagnóstico, remove apenas o erro de harness. Refutaria essa
inferência qualquer divergência posterior de message, hints, span, exit,
stdout ou perfil depois de ambos os binários realmente avaliarem a expressão.

## Obrigação da revisão 2

1. Novo manifesto, contrato, oracle, mutantes, runner e selo; v1 permanece
   evidência de calibração e não é reinterpretado.
2. Os quatro perfis usam somente a feature correspondente; HTML não introduz
   warning ou target no `eval`.
3. O runner possui adaptadores CLI por binário apenas para chegar ao mesmo
   envelope público; nenhum adaptador pode mudar message, hints ou span.
4. `rendered_stderr` bruto deixa de ser igualdade cross-CLI quando a forma de
   apresentação não é comum; o envelope exige classe, mensagem, hints e range
   exatos, mais exit/stdout reais.
5. Antes do selo, a política de linhagem deve definir digest semântico do L0
   que neutraliza exclusivamente a linha derivada `Hash do Código`, mantendo
   hash byte-level antes/depois e delta permitido exato. Qualquer outro byte
   alterado invalida a cadeia.

Regime: executado sem atestação de isolamento técnico. Nenhum staging, commit
ou push foi executado.
