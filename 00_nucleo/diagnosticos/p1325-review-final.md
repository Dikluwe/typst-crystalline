# P1325 — veredito final independente

Veredito `PASS_SCOPED_IMPLEMENTATION_AND_TEST_SUCCESSION`. Execução A/B sem
atestação técnica de isolamento, sem selo de refinamento. Revisor
`/root/p1325_review`: não escreveu produto, L0, testes ou expectativas
julgados; escreveu somente diagnósticos de revisão. Nenhum achado bloqueante
permanece para o recorte especificado.

## Resultado julgado

A mudança funcional é somente a escolha de field.span() na delegação de
acesso direto para Dict, Content raw e Float. Module e nativas preservam
seus guards; LocatedContent, callee, pré-despacho, lookup, mensagens e outros
variants permanecem intactos. A comparação literal contra baseline,
excluindo header e snippet congelado, confirmou exclusivamente esse delta.

O L0 runtime sucedeu expressamente as proteções antigas antes de C. O RED
R1 compilou e falhou em assertions de âncora das três categorias; seu
controle positivo passou. A tentativa R0 que não compilou permanece
instrumentação, sem crédito de RED. GREEN do owner executou 15 testes com
sucesso, incluindo fronteira LocatedContent Some/None.

A suíte workspace inicial revelou três expectativas históricas de Dict total.
O L0 test-only e manifesto sucessor foram auditados antes de o autor
independente migrar exclusivamente três ranges e um nome descritivo de
teste. Nenhum teste, helper ou controle foi removido. A ampliação restrita
do conjunto de owners foi registrada explicitamente; runtime e A/B ficaram
congelados. Esta migração não é apresentada como novo RED de produto.

## Gates completos e proveniência

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada.
Os recibos abaixo preservam UTC, comandos, estado before/after e diff/stat.
Todas as execuções de gate auditadas mantiveram o inventário estável durante
o comando. Não há staged diff.

| Gate | Resultado | Recibo SHA-256 |
|---|---|---|
| Build release locked workspace r1 | exit 0, binário A/B idêntico | `1fa7b1deb06eb9c62532c7652c9780cdf8496abb8fca8e4c7d7b67b9177b69c3` |
| Focal grupos legados | 100 passaram, zero falhas | `d1d9d25a00730729a42dc3ade7fff0acb68c1bb489124a1fb70320ede395cc91` |
| Workspace r1 | 6.677 passaram, zero falhas, três ignorados | `8ee75cc42b5cc6ba88a911cf5fb2bb5f829f92e7c40a7f4dc509e16f336c2326` |
| fmt r1 | exit 0 | `a953c9651419e70c2687259420398f1db33e71bf6201cfa14d415e70f805493e` |
| diff-check r1 | exit 0 | `f71bbe601aee900bd0347bb27196bc6fb70439ec6c5bf34c0945df022458622d` |
| crystalline-lint r1 | exit 0, zero erros; 240 warnings e 1.138 infos | `5b34990306f3ad6dc6ec56e608745392032e79658475f1f899a2ff2bc832ba6b` |
| V5/V15/V26 fail-on-warning r1 | zero violations, exit 0 | `b191ca1dd08db3cf26e5a2e0bf15cdd64f4f5554303e9dddb383c3524f315968` |

Workspace r1 terminou em `2026-09-09T01:18:51.110444+00:00`. A contagem
6.677 soma os resultados completos do workspace, incluindo doctests;
core isolado teve 5.573 aprovados. Warnings/infos do lint não foram
convertidos em inexistentes: a declaração de zero violations é limitada
aos checks V5/V15/V26.

## A/B e congelamento

A auditoria independente `p1325-review-ab-final-audit.json`, SHA-256
`ee96bcc57adc8052d99852f8b81345d06819b7ff3a5ec003090ed8f0e770d64b`,
recomparou integralmente os 1.008 registros da CLI: exit, stdout e stderr,
sem truncamento ou filtragem. Não faltam registros nem há duplicatas. São
28 casos, quatro perfis, três produtos e ordens normal, repetida e inversa.
Todas as expectativas pré-C coincidem com as saídas exigidas por produto.

As classificações por caso/perfil continuam individualizadas: 44 âncoras
corrigidas; quatro residuais de lookup apenas reancorados; 52 controles
coincidentes; 12 residuais externos preservados. Não se somam esses
residuais como paridade com vanilla. Nenhum Unknown obrigatório foi aceito.
Fonte/baseline da contagem: `p1325-ab-candidate-cli.json`, SHA-256
`bd3f0070d2832de246bf00ed7f71d0d93f1f864f9e7fb87390862adae03ea793`.

Os hashes dos binários foram recomputados: vanilla ratificado `a51e02804`
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
baseline `c5c9aa39c8b7053a19f53bf9f37c8d3731a4081683a51cf8ca5e9e9c0197d2f7`;
candidato `3511b08aa89d088e908dd239d8942f1eeea1978d31140ef9510e81de06023dab`.
O candidato é o mesmo binário depois da sucessão test-only; o runtime não
foi alterado, portanto a comparação A/B não foi invalidada.

## Linhagem, preservação e limites

`p1325-review-succession-final-audit.json`, SHA-256
`237fb3f041b1ef325c8bbe6223dcc1b9292a52e98876cfa4ac829dba26e35cf0`,
recalculou ambos os pares e pins:

- runtime: `6f7aad92` ↔ `48f648ca`;
- testes: `c15d1c0e` ↔ `0b22e216`.

As duas normas congeladas conferem; delta test-only coincide com o hash
independente; todos os artefatos A/B/legacy protegidos permanecem idênticos.
Nenhum arquivo produtivo fora dos quatro paths autorizados difere do
inventário baseline. O revisor conferiu separadamente a insuficiência do
reparador que não atualizou hash recíproco, e as correções foram somente
metadata, sem alteração silenciosa de normas.

O envelope freeze teve correção de horário digitado. O root viu o envelope
provisório antes de C e recebeu o hash final durante/depois de C; não se
afirma leitura antecipada do envelope final. Contratos e hashes protegidos
eram idênticos e já estavam congelados. A limitação temporal e sua
retificação estão preservadas em `p1325-review-candidate.md` e nas notas dos
autores. Isso impede qualquer linguagem de selo/atestação, não altera o
resultado restrito das comparações executadas.

`math.join` permanece com origem nativa pendente e sem warning corrigido;
nome+content-None foi refutado por imports. `[x].text` ainda falha como lookup
no cristalino; apenas a âncora mudou. Integer/string/closure mantêm dívidas
anteriores. Não se declara paridade geral do compilador nem quitação do
inventário P1322. Nenhum commit, staging ou push foi feito por esta revisão.
