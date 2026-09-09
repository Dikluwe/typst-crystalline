# P1326 — parecer independente final

**Veredito: PASS_SCOPED.** Correção Closure/With no lookup de field aceita,
sem achados acionáveis no candidato examinado. Regime A/B executado sem
atestação técnica de isolamento, sem selo de refinamento e sem paridade geral.
Revisor `/root/p1326_review` escreveu somente `p1326-review-*`; não alterou
produto, L0, testes ou oráculos julgados.

## Proveniência e cadeia

Manifesto SHA-256
`800db40f2300d0fda0275d06365c5f3019ca0848577ee54a9c834975b231146f`;
freeze `8d2484bc219b16f8a9aa106ec4efff2595bc174c80f888c0da14098385a48064`.
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado:
baseline, recibos unit/build/workspace e final AB conservam estado/diff/stat
e horários. Não atribuir resultados ao HEAD isolado. Antecessores desta
auditoria: preflight, L0, RED r0, freeze-red e candidate `p1326-review-*`.

A revisão anterior a C localizou e legitimou as duas sucessões Closure
P1311/P1324, mantendo Element/Plugin. Não foi necessário alterar outro owner.
O primeiro RED tinha um controle JSON incorreto, rejeitado como prova e
corrigido pelo autor independente contra os dois binários antes de C. R1
preservou r0 e modificou somente esse literal. O RED R1 validado antecedeu
a escrita funcional e a revisão confirmou ausência de C no freeze.

O candidato altera somente helper privado de categoria, seleção de span e
mensagem Closure. Reconstrução independente retirando esses deltas e os
snippets literais restaura o baseline. Sua categoria vem de FuncRepr, não
nome, scope, ponteiro ou execução. Não há API, entidade, default ou fase nova;
ADR-0127 contínuo continua aplicável.

## Gates efetivamente auditados

- RED R1: 15 controles aprovados e três falhas contratuais, após compilação
  válida; SHA `7526f89f71f959c58bd7aa99a921f7f124517559db2c6f382a79abcaf82287ee`.
- GREEN do mesmo owner: 18 aprovados, nenhuma falha; recibo SHA
  `6a3611382c5651471b5276bc3883f6bf47b596b07cf7cac8fec98a28c3bd282d`.
- Build release locked workspace, formatação e diff check: exit 0.
- Workspace release locked: 6.680 aprovados, nenhuma falha, três ignorados,
  recontados independentemente em 17 resumos do cargo. Recibo SHA
  `e63fb7e1e79bf23ca49f39b145f53870f9921bebd680145123d6b26c474d5698`,
  `2026-09-09T01:39:40.005046+00:00`–`01:41:58.252250+00:00`.
- Inventários produtivos before/after dos gates acima idênticos durante cada
  execução. Comparação atual contra inventário baseline detectou alteração
  somente no par L0/consumer field_access entre as entradas produtivas
  auditadas; os demais owners preservam hashes.
- Recíprocos `97a2773e`/`3a6ed279`, norma normalizada
  `4b3789c3decc0463ba553937ce7595c3a9c0ecc5fae545726ed009ad928d7451`,
  núcleo e snippets literais conferem. V5/V15/V26: exit 0, sem violations.
- Lint completo: exit 0, zero erros, **240 warnings e 1.139 infos**,
  recontados da saída. Isto não satisfaz uma alegação de lint global sem
  avisos; o parecer não a faz.

Binário candidato auditado
`bd86b34602323a390a60a8f41b97b180f93077affcd30b6b1e85aa5926dee811`.
Baseline `3511b08aa89d088e908dd239d8942f1eeea1978d31140ef9510e81de06023dab`
e vanilla ratificado upstream `a51e02804`, binário
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
também reconferidos após execução. Todos os onze hashes do freeze permanecem
iguais.

## Comparação independente e limites

Final AB `p1326-ab-final.json`, SHA
`73f74a2aa5eee7faaab3ad6c8d56d3fee9a30ed7ced37001bb482409b409be29`,
de `2026-09-09T01:41:47.867506+00:00` a `01:42:12.081337+00:00`:
recalculei as 276 comparações de exit/stdout/stderr contra os envelopes
congelados e confirmei zero divergências, 276 chaves únicas, normal/repeat/
reverse e nenhum Unknown. São 108 observações Closure corrigidas, 144
controles de paridade e 24 observações de residuais preservados. O focal
anterior passou 72 comparações. Não contar os residuais como paridade.

O adaptador `p1326-ab-cli-transport-r1.py`, SHA
`036a6de08b19f196511ea90131e3da143b657c4f842dca09bc4a0237a5524aac`,
somente troca transporte argv por stdin da mesma persistência apply_patch;
corpus, runner e predicados congelados permanecem intactos. O run anterior
sem recibo por E2BIG não recebe veredito. A repetição persistida é a evidência.

Baseline completo foi medido uma vez antes de C; estabilidade em três ordens
é evidência do candidato, não do corpus baseline completo. Não houve race,
mutation gate ou selo de refinamento neste regime A/B. Hashes e contextos
fresh não substituem isolamento técnico de capacidades no filesystem comum.

O parecer limita-se a Closure e With quando o lookup é alcançado. Ordem de
argumentos de `f.nope(panic(...))`, campo `[x].text`, identidade do warning
math, Plugin/Element e demais classes de P1322 continuam fora ou residuais
nomeados. Gates públicos/default/fase permanecem obrigatórios caso outro
recorte os exija. Relatório final lido: descreve corretamente essas limitações
e as evidências auditadas. Fechamento agregado deve referenciar este parecer
e conservar as provas anteriores, sem transformar PASS_SCOPED em paridade geral.
