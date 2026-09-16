# P1420 — relatório de execução

## Veredito

`INCONCLUSIVE`, executado sem atestação de isolamento.

O preseal, o gate discriminatório e o runner passaram. O focal math classificou
`math.abs` como `PARTIAL`, com forma qualificada `ABSENT` e alternativa
`DIFFERENCE`. O focal HTML não passou o controle obrigatório de target: com a
fixture literal `#context assert.eq(target(), "html")`, o vanilla terminou com
exit 0 e o cristalino com exit 1, diagnóstico `HTML export does not yet support
this content`, em `direct`, `direct-repeat` e `reverse`.

## Parada obrigatória

A primeira revisão continha texto adicional não autorizado no controle. A
revisão R2 removeu esse texto e rederivou o expected somente no vanilla. O
vetor continuou vermelho pela mesma causa dominante. A regra de rendimento
decrescente e o budget do passo proíbem uma terceira tentativa local e proíbem
executar o corpus integral antes de o focal ficar verde. Por isso nenhuma das
64 rotas restantes recebeu classificação inventada ou crédito de produto.

## Evidência reproduzível

- Estado: HEAD `95491eec8987176616a784554d11938e1b1c1e49`, tree
  `ba40fcfa888408ca0e09395beac18e08b59a4fd0`, working tree não commitado.
- Inventário: SHA-256
  `580a0307dc3758af395d658d9d3a7de5ac7ead3a84b535c5f15a19ebe090df15`.
- Focal R2: `/tmp/p1420-focal-r2-results.json`, SHA-256
  `fe637afddb8b3f8ac4d095cb19014a3662652c1a60342bc0ae4f18eb07ed3196`.
- Vetor das três ordens:
  `dbd36664b5bf88e75a6dd5adc8baf69dd7f6aa4704ee1e27406520df22261950`.
- Recursos: processos seriais, 512 MiB, timeout 15 s, streams 64 KiB, swap 0.

## Limite da conclusão

O P1420 não fecha as 66 rotas e não prova paridade funcional de HTML, math ou
Typst. O próximo passo deve redesenhar explicitamente o controle de target HTML
ou materializar o suporte observado, preservando a política fail-closed; só
depois uma campanha integral P1420 pode ser retomada.
