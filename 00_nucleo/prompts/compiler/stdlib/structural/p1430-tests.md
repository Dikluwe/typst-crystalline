# Testes independentes P1430 — convergência pública de `math.cases`

## Dono produtivo

Este L0 de teste possui exatamente um consumer:
`01_core/src/compiler/stdlib/structural/p1430_tests.rs`.

## Entradas congeladas

A suíte deriva exclusivamente do Passo 1430 confirmado e dos oito L0s
produtivos nele enumerados, sobre o baseline `dbcc6f62a1460607fad1d9f7f1fe43c528845c60`.
Ela não depende da implementação candidata. O fragmento observado é somente a
convergência pública de `math.cases`; não afirma equivalência geral do motor
matemático nem das outras dezassete rotas residuais.

## Obrigação

Exercer a função real publicada pelo módulo `math` e a rota bare, sem substituir
a fronteira por helper de teste. Cobrir:

- binding e identidade `cases`;
- zero, um e muitos children posicionais, com ordem preservada;
- cast de `Content`, string e `none`, e rejeição atribuível de inteiro;
- `delim` como string unitária, `none` e array de dois lados opcionais;
- casts tipados de `reverse` e `gap`, named desconhecido, `children:` e
  duplicados;
- bits independentes de explicitude, inclusive valores iguais aos defaults;
- `repr` canónico na ordem `delim`, `reverse`, `gap`, `children`;
- cascata sem explicitude morfológica e precedência explícito > cascata >
  default;
- convergência bare/qualificada no mesmo carrier para children equivalentes;
- igualdade, hash, clone, `plain_text`, `map_content` e `map_text`, preservando
  todos os valores e bits do carrier.

Os testes constroem argumentos e fontes pela API pública. Inspeção do carrier
usa identidade `Content::MathCases`, igualdade/hash e projeções públicas; não
replica casts, normalização, cascata nem o predicado sob teste.

## Observáveis e vereditos

Binding correto, carrier único, `repr`, conteúdo/ordem, diagnósticos e relação
de igualdade/hash são observáveis obrigatórios. Um resultado completo esperado
é `Preserved`; divergência atribuível é `Violated` e pode matar mutante.

Ausência ou multiplicidade do carrier, identidade divergente, cascata não
extraível, execução ilegível, timeout, OOM, budget esgotado ou falha de
infraestrutura são `Unknown`. `Unknown` implica `acceptance = false` e
`mutation_credit = false`; jamais é convertido implicitamente em sucesso ou
mutante morto.

## Budget e repetição

Budget inicial: duas revisões focais e uma execução completa por revisão. Duas
revisões consecutivas sem delta discriminatório exigem reabrir o desenho. Após
GREEN, a suíte deve passar em três ordens determinísticas antes do selo final.

