# P1331 — revisão do L0 antes de A/B e C

Revisor `/root/p1331_review`, sem escrita de L0, produto ou testes.
Regime A/B executado sem atestação de isolamento; sem refinamento.
Predecessor: `p1331-review-preliminary.md`. Nenhum candidato C foi recebido.

## Entradas identificadas

Prompt `00_nucleo/prompts/compiler/stdlib/calc.md`, SHA-256 integral observado
`1df3bee4e189cbe8e08a1634ee17d064fe75aa4aa1ebb1f6aef5785276b49e64`;
norma sem a linha `Hash do Código`:
`ef406128181e4cfa5b802b86ec0a9c58370cc4f7dab09365a088eef74523956a`.
O integral pode mudar no resselo de metadata; a norma acima é a entrada
avaliada. Baseline e HEAD são os identificados no parecer preliminar.
Sonda adicional `p1331-domain-probes.json`, SHA-256 verificado
`2b40d0f6d973a580122fcaee87ef42ee742d2a4ef167919091da0b85beeb7818`,
conserva estado, argv, binários e horários próprios; observações começam em
`2026-09-09T13:37:56.441085+00:00`.

## Medição e avaliação

A sonda adicional demonstra que `path("x")` e `path("")` são construídos
e alcançam a rejeição de abs, com nome path nos dois sistemas. `type(path("x"))`
confirma essa identidade observável. `0pt + 0%` continua Relative e rejeitado
como relative length. Estes exemplares resolvem a incerteza específica
assinalada no parecer preliminar; não provam paridade geral de Path/Relative.

`calc.abs(location)` recebe Type, como o L0 agora afirma. Para a instância
nativa, `lab/typst-original/crates/typst-library/src/introspection/location.rs:52-59`
declara Location e construtor; o cast ToAbs não a admite. O L0 limita o teste
nativo ao diagnóstico de um valor já construído e exclui alegações sobre sua
produção por introspecção. Não há inferência de equivalência a partir do
literal `location`.

O L0 mantém medida antes de decisão, diferencia nomes diagnósticos de
`Value::type_name`, manda montar os nomes localmente e não autoriza mudar
entidades ou helpers. A obrigação é rejeição com diagnóstico completo,
primeiro value_span posicional e preservação explícita de ausência/detached.
Guard named/aridade, outros braços de abs, dispatcher e outras funções
permanecem fora da mudança. A substituição dos controles históricos str/symbol
inclui mensagem, origem e trace; o restante do corpus permanece vigente.

ADR-0127 contínuo é aplicável à correção medida. O L0 define a fronteira
Unknown e não declara paridade geral. Considero a norma suficiente para
iniciar os testes A/B congelados e obter RED antes de qualquer C.

## Veredito limitado

Aprovado para o próximo gate A/B, condicionado ao resselo/preflight de
linhagem e à preservação da norma identificada. Esta revisão não atesta
isolamento, não sela refinamento, não aprova implementação e não fecha P1331.
