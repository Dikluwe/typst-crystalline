# Testes independentes P1429 — fronteira pública de `math.accent`

## Dono produtivo

Este L0 de teste possui exatamente um consumer:
`01_core/src/compiler/stdlib/structural/p1429_tests.rs`.

## Obrigação

A suíte deve partir somente do contrato P1429 congelado e exercer a função
real publicada pelo módulo `math`, sem substituir a fronteira por um helper de
teste. Ela verifica binding, dois posicionais obrigatórios, casts de base,
normalização do acento, `size`/`dotless`, presença explícita, ordem de named,
duplicados, argumentos desconhecidos, rotas bare e wrappers simbólicos.

Os observáveis são identidade `MathAccent`, `repr` canónico, conteúdo público
do carrier e diagnósticos atribuíveis. Erro genérico, panic de infraestrutura
ou mera falha de compilação não satisfazem uma rejeição semântica.

## Política de opacidade

Ausência ou multiplicidade do carrier, identidade de entrada divergente,
execução ilegível ou extração sem atribuição são `Unknown`; `Unknown` bloqueia
casos obrigatórios e nunca conta como mutante morto.

## Revisão R2

O harness executável da suíte enumera separadamente U01–U06 e exige para
cada causa o estado `Unknown`, `acceptance = false` e
`mutation_credit = false`. Controles `Preserved` e `Violated` impedem que o
harness seja uma constante que classifica toda entrada como opaca.
