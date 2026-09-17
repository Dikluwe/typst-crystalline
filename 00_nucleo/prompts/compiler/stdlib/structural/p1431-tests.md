# Testes independentes P1431 — fronteira pública de `math.mat`

## Dono test-only

Este L0 possui exatamente um consumer test-only:
`01_core/src/compiler/stdlib/structural/p1431_tests.rs`.

## Entradas congeladas e limite

A suíte deriva do Passo 1431 confirmado, dos oito L0s owners, do manifesto
`p1431-manifest.json` e do contrato `p1431-contract.json`, sobre o baseline
`f5aa8d72f4d897cd1cd7ffa01e7822c082967173`. Não lê nem se adapta à
implementação candidata. O fragmento é somente `math.mat`.

## Obrigação

Exercer a função publicada no módulo fechado `math`. Cobrir binding e nome;
zero/scalar/arrays; retangularização; casts e cardinalidade de delimitadores;
os cinco alinhamentos horizontais; shorthand e precedência dos gaps relativos;
augment inteiro, dicionário e `none`; bits causais e `repr`; unknown named,
`rows:`, duplicados e todos os tipos inválidos; validação equivalente em
`#set math.mat`. Nenhum argumento pode ser descartado silenciosamente.

Resultados completos esperados são `Preserved`; divergências atribuíveis são
`Violated`. Identidade, binding, carrier, cascade ou execução não atribuível é
`Unknown`, com aceitação e crédito de mutação falsos.

