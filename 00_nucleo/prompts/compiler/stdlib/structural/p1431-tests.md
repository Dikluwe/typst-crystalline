# Testes independentes P1431 — fronteira pública de `math.mat`

## Dono test-only

Este L0 possui exatamente um consumer test-only:
`01_core/src/compiler/stdlib/structural/p1431_tests.rs`.

## Entradas congeladas e limite

A suíte deriva do Passo 1431 confirmado, dos onze L0s owners, do manifesto
`p1431-manifest.json` e do contrato `p1431-contract.json`, sobre o baseline
`f5aa8d72f4d897cd1cd7ffa01e7822c082967173`. Não lê nem se adapta à
implementação candidata. O fragmento é somente `math.mat`.

## Obrigação

Exercer a função publicada no módulo fechado `math`. Cobrir binding e nome;
zero/scalar/arrays; retangularização; casts e cardinalidade de delimitadores;
os cinco alinhamentos horizontais; shorthand e precedência dos gaps relativos;
augment inteiro, dicionário e `none`; bits causais e `repr`; chaves
desconhecidas dentro do dicionário de augment aceites e ignoradas sem entrar
no carrier/`repr`; unknown named da função, `rows:`, duplicados e todos os
tipos inválidos dos campos conhecidos; validação equivalente em
`#set math.mat`. Nenhum argumento da assinatura pode ser descartado
silenciosamente.

P03 exige três conteúdos escalares ordenados. P12 exerce separadamente os
shorthands inteiros positivo e negativo. P16 observa herança e override
independentes dos cinco campos causais. P18 inclui auto sobre custom,
custom sobre custom e `none` explícito.

N03 exerce arrays de delimitador com uma e três entradas. N06 exerce inteiro
em `gap`, `row-gap` e `column-gap`. N08 exerce string, float e array com item
inválido em ambos `hline` e `vline`. U02 observa lookup real sem identidade
atribuível e U06 passa saída real de `repr` por um observador truncado; ambos
classificam `Unknown` com aceitação e crédito falsos.

Resultados completos esperados são `Preserved`; divergências atribuíveis são
`Violated`. Identidade, binding, carrier, cascade ou execução não atribuível é
`Unknown`, com aceitação e crédito de mutação falsos. O controle U01/U09/M27
verifica a identidade SHA-256 do runner público e o executa diretamente em
`--mode policy-self-check`; a resposta `Unknown` deve sair sem aceitação nem
crédito. Uma política mutada que aceite `Unknown` precisa falhar causalmente.
