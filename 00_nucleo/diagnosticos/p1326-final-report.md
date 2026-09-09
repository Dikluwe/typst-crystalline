# P1326 — diagnóstico específico para fields de closures

## Problema e recorte

`{ let f() = 1; f.nope }` publicava `cannot access fields on type function`
com destaque em `f.nope` inteiro. No vanilla ratificado upstream `a51e02804`,
a mensagem é `cannot access fields on user-defined functions` e somente
`nope` é destacado. A medição fresca reproduziu isso em funções anônimas,
aliases, aplicações parciais With aninhadas, multilinha e callee benigno.

Implementado e validado no recorte: testes do owner, workspace e comparação
bilateral passaram, com revisão segregada e preservação das fronteiras.

O recorte é somente Closure, inclusive através de With, quando o acesso
alcança o lookup deste owner. Não é mudança de lookup, namespace, argumentos
ou chamada. Não generaliza a Plugin/Element, não altera nativas, Module,
Dict/Content/Float P1325, snapshots ou gates antecipados. O L0 proprietário
`compiler/eval/bindings/field_access.md` sucede explicitamente a proteção
antiga de Closure e seus testes P1311/P1324.

O requisito já pode ser expresso pela categoria existente e pelo span da
AST; não foi necessário ampliar Func, entidade, API, default ou fase.
Classificação contínua ADR-0127, com medição e L0 antes de código.

## O que não foi corrigido

`f.nope(panic("arg"))` continua expondo a ordem anterior de avaliação no
cristalino: o argumento falha antes do lookup. Vanilla acusa o field primeiro.
Essa diferença foi classificada antes do candidato como preservação do
baseline, não crédito de paridade. Corrigi-la exigiria investigar o despacho
de chamadas; este passo não o altera.

Continuam abertas as pendências anteriores de identidade para o warning de
`math.join`, lookup de `[x].text` e outras classes do inventário P1322.
Plugins e elementos de usuário não estão cobertos por esta decisão. Nenhuma
afirmação de paridade geral de funções ou da linguagem decorre deste recorte.

## Proveniência e verificação

Baseline `p1326-baseline.json`, SHA
`3198d97b15c02085af158e88e44c4025e4f76b29654e7309c2ca21fdb2a41f79`,
concluído em `2026-09-09T01:23:32.345952+00:00`. Working tree não commitada
sobre HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`; o recibo inclui a lista
exata/diff/stat dos dez arquivos tracked alterados, estado, comandos, UTC,
saídas integrais e hashes dos binários. Os demais recibos de execução
registram inventário antes/depois para não atribuir medições ao HEAD isolado.

- Baseline P1325: `/tmp/p1325-target.KQl8cD/release/typst`, SHA
  `3511b08aa89d088e908dd239d8942f1eeea1978d31140ef9510e81de06023dab`.
- Vanilla: `/usr/local/bin/typst`, SHA
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Manifesto `p1326-manifest.json`, SHA
  `800db40f2300d0fda0275d06365c5f3019ca0848577ee54a9c834975b231146f`.
- Norma congelada SHA
  `4b3789c3decc0463ba553937ce7595c3a9c0ecc5fae545726ed009ad928d7451`,
  excluindo somente a linha canônica recíproca `Hash do Código:`.
- Candidato `/tmp/p1326-target.6Vi4Km/release/typst`, SHA
  `bd86b34602323a390a60a8f41b97b180f93077affcd30b6b1e85aa5926dee811`;
  build do workspace em `p1326-final-build.json`.

A skill `tekt-materializacao-segregada` orienta ensaio A/B: root redige
L0/implementação, `/root/p1326_tests` recebe norma, contratos públicos,
binários e extrato test-only legado, sem source funcional; o revisor
`/root/p1326_review` audita sem editar os artefatos julgados. Ambiente
compartilhado, sem atestação técnica de isolamento ou selo de refinamento.
O extrato legado permite suceder somente expectativas de Closure, mantendo
as outras categorias. Não derivar expected da implementação candidata.

A revisão encontrou antes de C as duas expectativas antigas de Closure
agrupadas com Element/Plugin nos testes embutidos P1311/P1324. O autor
separou apenas essas expectativas, conservando os controles das outras
categorias. A pesquisa também examinou o owner test-only de eval e não
encontrou outro teste de field de closure que precisasse de sucessão.

O primeiro RED compilou, mas tinha um controle positivo com literal JSON
compacto onde os dois binários produziam o default formatado. Essa falha
não foi usada para justificar mudança de produto. O autor publicou R1 com
apenas esse literal corrigido pela medição bilateral, preservando R0.
O RED válido `p1326-unit-red-r1.json`, SHA
`7526f89f71f959c58bd7aa99a921f7f124517559db2c6f382a79abcaf82287ee`,
confirmou três falhas contratuais de Closure e 15 controles aprovados, com
inventário idêntico antes/depois. A correção funcional só foi aplicada após
esse resultado e o congelamento integral de casos/expectativas/comparador.

Freeze `p1326-ab-freeze.json`, SHA
`8d2484bc219b16f8a9aa106ec4efff2595bc174c80f888c0da14098385a48064`,
registrado automaticamente em `2026-09-09T01:32:47.106064+00:00` e conferido
antes de C. O baseline completo classificou 92 pares caso/perfil:
36 correções necessárias, 48 controles já coincidentes e oito residuais
para preservação integral do baseline, sem Unknown. A classificação cobre
os quatro perfis juntos; não significa 92 casos por perfil.

O delta funcional consiste em um helper privado que reconhece Closure
através de With, e duas escolhas locais: mensagem e span. O helper é
exaustivo e não usa nome ou ausência de namespace como heurística de origem.
A revisão reconstruiu o baseline removendo exatamente esse delta e
desintegrando os snippets congelados. Não foi alterado nenhum outro owner.

GREEN do owner: 18 testes aprovados, zero falhas, em
`p1326-unit-green.json`, SHA
`6a3611382c5651471b5276bc3883f6bf47b596b07cf7cac8fec98a28c3bd282d`.
Os testes independentes fazem avaliação completa pela API pública e resolvem
o span na Source; o helper candidato não gera suas expectativas.

O dry-run do linter retornou novamente `Nothing to fix` apesar da metadata
recíproca antiga. Após cálculo independente, somente `Hash do Código:` foi
corrigido para `3a6ed279`, com recibo `p1326-reciprocal-correction.json`.
Header efetivo `97a2773e`, núcleo e norma congelada intactos; o source que
passou GREEN não mudou. `p1326-final-lineage.json` verifica os dois hashes
canônicos e a integração literal dos snippets. V5/V15/V26 terminaram sem
violations; o lint completo registra zero erros, 240 warnings existentes e
1.139 infos. O info adicional corresponde ao match exaustivo da categoria;
não se declara lint livre de avisos.

## Gates do workspace

| Gate | Resultado e recibo |
|---|---|
| Build release locked workspace | exit 0; `p1326-final-build.json` |
| Testes do owner | 18 aprovados, zero falhas; `p1326-unit-green.json` |
| Suite completa release locked | 6.680 aprovados, zero falhas, três ignorados; `p1326-workspace-tests.json` |
| Formatação/diff check | exit 0; `p1326-final-fmt.json`, `p1326-final-diff-check.json` |
| Lint completo | zero erros, 240 warnings, 1.139 infos; `p1326-final-lint.json` |
| Linhagem e ownership V5/V15/V26 | zero violations; `p1326-final-lineage-lint.json` e checker `p1326-final-lineage.json` |

O recibo do workspace tem SHA
`e63fb7e1e79bf23ca49f39b145f53870f9921bebd680145123d6b26c474d5698`;
as contagens agregam os resumos de testes do cargo, sem incluir filtrados
como executados. Seu inventário produtivo foi idêntico antes/depois.
Execução de `2026-09-09T01:39:40.005046+00:00` a
`01:41:58.252250+00:00`, somando 17 resumos de grupos.
Não foi necessário alterar outro owner de testes após a implementação.

## Resultado bilateral e limites

O focal passou em 72/72 comparações. A execução completa sucessora, de
`2026-09-09T01:41:47.867506+00:00` a `01:42:12.081337+00:00`, passou em
276/276 comparações integrais: 23 casos, quatro perfis e três ordens do
candidato. São 108 registros de closures corrigidas, 144 controles e 24
registros de residuais preservados. Zero Unknown; 92 chaves caso/perfil
estáveis entre normal, repetição e ordem inversa. Não há normalização ampla
de mensagem/span/saída nem conversão de erro em sucesso.

`p1326-ab-final.json`, SHA
`73f74a2aa5eee7faaab3ad6c8d56d3fee9a30ed7ced37001bb482409b409be29`,
contém os comandos e vetores completos; `p1326-ab-verdict.json`, SHA
`67ed3eb5b52f71eadaf86b7a692e9fa46b905df847816b2de76faef8ed3bcf6b`,
confere os hashes e delimita a evidência. O corpus inteiro dos dois baselines
foi medido uma vez antes de C; apenas o subconjunto focal teve medição
adicional. A estabilidade normal/repetida/inversa observada é do candidato,
não uma alegação de três ordens completas também dos baselines ou de race.

A primeira rodada completa encontrou `E2BIG` ao persistir o recibo:
o texto do apply_patch foi passado como argumento e excedeu o limite do
sistema. As saídas não persistidas não recebem veredito, e seus horários
exatos não foram inventados. Um adaptador mudou somente o transporte do
mesmo patch para stdin; runner, casos, comparadores e expectativas
congelados ficaram byte-idênticos. A repetição completa acima é a evidência
válida. Houve uma revisão nessa causa, com custo de uma execução completa
adicional, sem mudança no poder discriminatório dos testes.

O parecer final está em `p1326-review-final.md`. O fechamento reproduzível
`p1326-closure.json` reúne hashes, estado final e preservação da evidência
anterior. Oito arquivos tracked que já estavam modificados fora do par
de field_access continuam byte-idênticos ao baseline, incluindo o owner
test-only de eval e seu L0. Não houve limpeza de temporários ou artefatos.

Temporários no target exclusivo `/tmp/p1326-target.6Vi4Km`, cópia sem
hardlinks do anterior. Preservar arquivos e evidências anteriores.
Sem staging, commit ou push.
