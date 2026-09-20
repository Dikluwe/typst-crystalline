# Prompt L0 — testes públicos de `math.frac` P1428
Hash do Código: 5c5f7ae8

## Owner

`03_infra/tests/p1428_math_frac.rs` (teste L3; nenhum código produtivo).

## Contrato

- Extrair texto de HTML por um único passe, considerando apenas text nodes.
- Decodificar uma vez entidades HTML nomeadas e numéricas válidas; entidade
  desconhecida, markup ausente ou desequilibrado produz erro observável.
- Avaliar frações por `eval_to_module_with_sink` e percorrer o `Content`
  resultante, sem inferir estado a partir da grafia da fixture.
- Exigir exatamente um `MathFracElem` para observar os bits
  `num_deparenthesized` e `denom_deparenthesized`.
- Cobrir as três formas públicas de `math.frac`, diagnósticos de assinatura,
  constantes OpenType e a independência de `math.binom`.

## Aceitação

Os testes verificam somente comportamento público, carriers de domínio e
invariantes de layout. Não executam mutantes, runners externos, hashes de
implementação, ordens artificiais ou artefatos administrativos.
