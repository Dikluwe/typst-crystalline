# P1325 — erro aponta o campo, não o acesso inteiro

## Resultado e limite

A correção foi implementada para acesso direto a campos de dicionário,
conteúdo raw e float. Por exemplo, em `(x: 1).nope`, o erro agora destaca
somente `nope`, como o vanilla ratificado. A mensagem e o lookup não mudam.
Aliases, nomes Unicode e mudanças de linha seguem a mesma regra.

O único delta funcional é a seleção do span em
`01_core/src/compiler/eval/bindings/field_access.rs`; o proprietário
`compiler/eval/bindings/field_access.md` foi atualizado antes do código,
substituindo explicitamente as antigas proteções de span total neste recorte.
LocatedContent, erros de método/callee, outros tipos e gates antecipados não
entraram na mudança. Os controles anteriores de Module e funções nativas
continuam passando. Não houve alteração de API, default ou fase: ADR-0127
contínua para paridade diagnóstica.

Implementado e validado no recorte: corpus bilateral e workspace passaram,
após sucessão explícita das três expectativas test-only antigas.

## O que continua faltando

`math.join` continua sem o warning de depreciação visto no vanilla. A
hipótese de resolver isso pelo nome do módulo e por conteúdo ausente foi
refutada: `compiler/eval/modules.rs:105–114` também produz `Module::new`
sem conteúdo para arquivos importados, inclusive `math.typ`. Emitir o aviso
por essa heurística poderia criar falsos positivos. O recibo independente
`p1325-review-math-identity-preflight.md` reabre a suficiência de owner e a
identidade nativa; não afirma que qualquer solução local seja impossível.

`[x].text` ainda falha no cristalino, enquanto vanilla retorna `"x"`.
Neste passo mudou somente a âncora do erro cristalino. Essa ausência de
campo permanece débito de lookup, sem crédito de paridade. Erros de integer,
string e closure também permanecem como baseline onde estão fora do recorte.
Não se declarou paridade geral do compilador ou do inventário P1322.

## Evidência reproduzível

Working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`. Cada recibo de comando inclui
UTC, argv, cwd, saída integral, exit e diff/stat/inventário antes/depois.
A lista exata dos oito arquivos tracked já alterados está no baseline;
seis arquivos externos ao par deste passo permanecem byte-idênticos.

| Entrada | Identidade |
|---|---|
| Baseline fresco | `p1325-baseline.json`, SHA `e2ed7ae9f8290ee9557b38d151ffffebae760b79fa1abb64979d6c1eb9d7998b`, concluído `2026-09-09T00:49:40.661695+00:00` |
| Binário baseline P1324 | SHA `c5c9aa39c8b7053a19f53bf9f37c8d3731a4081683a51cf8ca5e9e9c0197d2f7` |
| Vanilla upstream `a51e02804` | `/usr/local/bin/typst`, SHA `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| Binário candidato | `/tmp/p1325-target.KQl8cD/release/typst`, SHA `3511b08aa89d088e908dd239d8942f1eeea1978d31140ef9510e81de06023dab` |
| Manifesto | `p1325-manifest.json`, SHA `b4057fe02a65105654a2379be04b826179ce4ee2573e69a4d1f347c089347dce` |
| Norma congelada | SHA `2648ff575db76ac3a98832a211bb24d65c1bdfac6d4510a96bc85e6bf36b1b35`, excluindo somente a linha recíproca `Hash do Código:` |

## Testes e revisão

A skill `tekt-materializacao-segregada` orientou o ensaio A/B: root escreveu
L0/implementação, `/root/p1325_tests` derivou testes sem ler o owner/candidato,
e `/root/p1325_review` julgou artefatos sem editá-los. Ambiente compartilhado,
sem atestação técnica de isolamento e sem selo de refinamento.

O primeiro teste não compilou por import incorreto de Route; seu exit 101
em `p1325-unit-red.json` é falha de instrumentação, não RED. O autor corrigiu
o import e acrescentou a fronteira LocatedContent em R1, preservando R0.
R1 foi formatado/congelado e integrado literalmente, sem adaptação à solução.
O RED real `p1325-unit-red-r1.json` executou quatro testes: três falhas de
âncora, uma por categoria, e um positivo aprovado. O controle AST separado
de LocatedContent passou no baseline, incluindo snapshot Some/None.
GREEN do owner em `p1325-unit-green.json`: 15 aprovados, nenhuma falha,
de `01:02:07.224216` a `01:03:33.340364` UTC em 2026-09-09.

O controle CLI de LocatedContent encontrou uma opção não suportada; isso
foi tratado como instrumentação e substituído pelo controle AST, não como
preservação implícita. A classificação inteira do baseline CLI precedeu C,
separando âncoras, positivos e residuais. Unknown não vale sucesso.

A matriz final `p1325-ab-candidate-cli.json`, SHA
`bd3f0070d2832de246bf00ed7f71d0d93f1f864f9e7fb87390862adae03ea793`,
iniciada em `2026-09-09T01:06:58.709152+00:00`, preserva 1.008 execuções:
28 casos, quatro perfis, três produtos e ordens normal/repetida/inversa.
O veredito `p1325-ab-verdict.json` e a auditoria independente conferem
1.008/1.008 vetores completos com as expectativas congeladas, zero
diferenças inesperadas e nenhum protegido alterado. A classificação por
caso/perfil é: 44 âncoras a corrigir, quatro resíduos `[x].text` com apenas
âncora nova, 52 controles de paridade e 12 resíduos fora do recorte cuja
saída cristalina é preservada. Esses números abrangem os quatro perfis,
não são contagens por perfil. Preservação de residual não significa
coincidência com vanilla. A repetição também inclui os dois baselines.

### Três expectativas antigas encontradas no workspace

A primeira suíte completa, `p1325-workspace-tests.json`, terminou com
5.570 testes core aprovados e três falhas. Todas exigiam span total de Dict
em `01_core/src/compiler/eval/tests.rs` (P1301, P1303, P1306). O L0 runtime
já sucedia essa obrigação, mas o L0 test-only ainda a repetia. Portanto,
a identificação inicial do conjunto de owners estava incompleta quanto
aos testes; não seria correto declarar workspace GREEN nem afrouxar o gate.

O adendo L0 `compiler/eval/tests.md` e o manifesto separado
`p1325-test-succession-manifest.json`, SHA
`6c9e00228545f6101ccff3af7057d7983fc4f376a6ddedf8184055949e3ab685`,
autorizam estritamente a sucessão test-only. O manifesto original e o
congelamento A/B continuam intactos; a exceção explícita de inventário
abrange somente `01_core/src/compiler/eval/tests.rs` e seu L0
`compiler/eval/tests.md`, congelando byte a byte o par runtime.
Não é novo contrato público/default/fase nem correção funcional adicional.

O autor independente derivou ranges das expressões e confirmou o vanilla
nos quatro perfis, sem usar saída candidata como expected. O patch
`p1325-ab-legacy-delta.patch` migra somente três ranges e renomeia o teste
P1301 para descrever field-only. Nenhum teste foi apagado, nem mensagem,
hint, helper, perfil ou controle relaxado. O revisor aprovou antes da
integração literal. Isso não é apresentado como novo RED pré-candidato do
produto; a falha original e o achado de cobertura permanecem registrados.

### Gates finais

| Gate | Resultado e recibo |
|---|---|
| Build release locked workspace | exit 0; `p1325-final-build-r1.json`; binário final byte-idêntico ao validado no corpus |
| Grupos históricos P1300–P1308 | 100 aprovados, zero falhas; `p1325-test-succession-focal.json` |
| Workspace release locked | 6.677 aprovados, zero falhas, três ignorados; `p1325-workspace-tests-r1.json` |
| Formatação e diff check | exit 0; `p1325-final-fmt-r1.json`, `p1325-final-diff-check-r1.json` |
| Lint completo | zero erros, 240 warnings e 1.138 infos; `p1325-final-lint-r1.json` |
| V5/V15/V26 com fail-on warning | zero violations; `p1325-final-lineage-lint-r1.json` |
| Hashes, normas e testes congelados | PASS; `p1325-final-lineage-r1.json`, `p1325-test-succession-lineage.json` |

A execução completa sucessora ocorreu de
`2026-09-09T01:16:16.590287+00:00` a `01:18:51.110444+00:00`, com inventário
produtivo idêntico antes/depois, conforme recibo SHA
`8ee75cc42b5cc6ba88a911cf5fb2bb5f829f92e7c40a7f4dc509e16f336c2326`.
As contagens agregam os 17 resumos de grupos do cargo, sem contar testes
filtrados como executados. O lint não está livre de avisos: conserva as
mesmas contagens anteriores. O par test-only terminou em header `c15d1c0e`
e recíproco `0b22e216`, com pin de núcleo preservado. As revisões finais
independentes constam em `p1325-review-final.md`; o fechamento reúne hashes,
proveniência e preservação da evidência anterior em `p1325-closure.json`.

O autor corrigiu o horário digitado do envelope de freeze. O root viu o
envelope provisório antes de C e recebeu o hash final durante/depois da
chamada de implementação. Os hashes normativos, testes, expectativas,
comparador e baseline eram idênticos e já estavam congelados; não se alega
leitura antecipada do envelope final. As duas notas de correção temporal e
a revisão preservam essa limitação, sem mudar expectativas depois de C.

O linter voltou a dizer `Nothing to fix` com hash recíproco antigo.
Após cálculo independente, somente a metadata foi corrigida para
`Hash do Código: 48f648ca`; header efetivo `6f7aad92`, núcleo inalterado.
`p1325-reciprocal-correction.json` e `p1325-final-lineage.json` comprovam a
correção e a identidade da norma/testes. Fonte e binário não mudaram por ela.

Todos os temporários ficaram no target dedicado em `/tmp`, sem hardlinks
com o cache anterior. Nenhum artefato histórico anterior a P1325 foi apagado
ou substituído; a correção do envelope de freeze deste passo é a exceção
documentada acima, sem mudança das entradas protegidas.
Sem staging, commit ou push.
