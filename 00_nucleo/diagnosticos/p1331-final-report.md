# P1331 — tipos rejeitados por calc.abs recebem diagnóstico localizado

## Resultado e limites

`calc.abs` continua rejeitando os mesmos tipos, mas seu fallback agora
informa a união aceita, o tipo encontrado e a origem do argumento.
Exemplos de mensagem primária:

- `calc.abs("x")` → `expected integer, float, length, angle, ratio,
  fraction, or decimal, found string`.
- `calc.abs(true)` termina com `found boolean`, não com o nome curto bool.
- `calc.abs(sym.alpha)` termina com `found symbol`.
- Path e comprimento relativo continuam rejeitados como path e relative
  length, inclusive o relativo com componentes zero.

O único trecho produtivo alterado é o corpo do fallback `[other]` em
`01_core/src/compiler/stdlib/calc.rs`. A origem vem do value_span da
primeira ocorrência posicional; ausência ou detached não são substituídos
por posição inventada. O erro nativo não recebe hints nem trace próprio.
Os nomes longos de Str/Bool foram tratados localmente; Value::type_name e
helpers compartilhados não mudaram.

Conteúdo P1328, dimensões P1329 e overflow inteiro P1330 mantêm suas
regras. Nenhum tipo novo é aceito; string numérica não vira número.
Guards de named/aridade, nomes das funções e outros consumers permanecem
intactos. O passo é `00_nucleo/materialization/typst-passo-1331.md`; a
norma proprietária é `00_nucleo/prompts/compiler/stdlib/calc.md`.

### Pendências que este passo não fecha

- Traces de origens pré-vinculadas por With/arguments ainda podem usar
  `calc.abs`, enquanto vanilla usa `abs`. A mensagem e a origem primárias
  estão corrigidas; não há equivalência integral nessas rotas.
- Named/quantidade ainda têm precedência diferente da do vanilla.
- Parsing do literal mínimo inteiro, construção dimensional NaN,
  ausência de float.nan, Float×Fraction e diferenças em outras funções
  continuam fora do recorte.
- `path()` sem argumento falha antes de chegar a abs; essa observação não
  prova o novo fallback. Path válido foi medido separadamente.
- Location já construída tem teste nativo de diagnóstico, não uma prova
  da sua produção por introspecção. `calc.abs(location)` recebe o tipo,
  não uma instância de Location.

Uma medição pré-C evitou uma classificação errada de math: string
explícita em `$std.calc.abs("-1")$` continua string. Os controles
históricos que recebem conteúdo permaneceram intactos, sem mudar fase
ou fazer coerção entre texto e número.

## Proveniência e preservação

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, **working tree não
commitado**. Baseline iniciado em `2026-09-09T13:36:38.154537+00:00`.
`p1331-baseline.json`, SHA-256
`7287bfac86e29e8ff0e86955ce96f63e140b745fb781eb4b4b7410215c794b45`,
registra lista exata de arquivos alterados, diff HEAD/stat, inventário,
argv, horários e binários. Os recibos root dos gates guardam esses dados
antes/depois. Sobre P1330, apenas calc L0/owner mudou; alterações anteriores
e evidências históricas foram preservadas.

Vanilla ratificado upstream/main **a51e02804**, executável
`/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
BASE P1330: `/tmp/p1330-target.f0lmDu/release/typst`, SHA-256
`6f1db621bc0b2a7fe4fc9d05925fb96636f33232b8527b83c040b793970fbda0`.
Não se usa a string de versão como identidade da medição.

As oito células históricas string/symbol têm expectativas sucessoras;
as outras 324 foram mantidas literalmente. Os testes históricos só
migraram aqueles dois diagnósticos (mensagem/span/trace). A condição de
sqrt no bloco compartilhado continua com suas asserções; módulos P1329 e
P1330 foram mantidos integralmente. Prova:
`p1331-historical-expectations-r2.json`, SHA-256
`3a4556c4ebb96f7a9b7c5f65d975ef550e2a1f5995b5ddb64d20987fa43c1c7d`.

Skill `tekt-materializacao-segregada`: A/B com root como autor L0/candidato,
`p1331_tests` como autor de testes sem ler runtime/patch calc e
`p1331_review` como revisor sem editar o que julga. **Sem atestação técnica
de isolamento**, sem selo de refinamento, sem alegação de paridade geral.
O autor corrigiu a classificação de math com medição antes de congelar.
As expectativas de valor/erro não foram ajustadas ao candidato; houve
reabertura explícita da fixture após a primeira tentativa, descrita abaixo.

### Reabertura da fixture Path

O primeiro GREEN compilado teve 29 testes aprovados e uma falha. A
expressão `#calc.abs(path("p1331.typ"))` parava antes de abs porque
TestWorld não implementava World::resolve_path. O método padrão da trait
retorna `cannot access file system from here` independentemente do FileId;
trocar apenas Source::detached por uma Source attached não resolveria.
A atribuição inicial à Source detached foi corrigida pela revisão da fonte.

O primeiro RED não isolou essa insuficiência: o teste denso parava em
uma família rejeitada anterior. Por isso R1/C1 e seus recibos foram
preservados e o candidato foi retirado antes da nova autoria da fixture.
R2 adicionou resolução virtual pura e uma sentinela de construção,
preservando a expressão Path e as mesmas asserções de diagnóstico. Não se
corrigiu produto/constructor nem se removeu o caso para obter aprovação.

Registros: `p1331-unit-green.json`, `p1331-review-green-r1-reopen.md` e
`p1331-r2-restore-pre-c.json`. A restauração foi comparada integralmente
com a integração pré-C. A cadeia foi refeita desde fixture/freeze: a
sentinela passou antes de C2; o novo RED compilado teve 25 testes aprovados
e seis falhas diagnósticas esperadas; GREEN R2 passou nos mesmos 31 testes.
As 504 expectativas CLI permaneceram literalmente iguais às de R1.
A revisão `p1331-review-candidate-r2.md` também confirmou que o fallback
de produção C2 é idêntico ao de C1: nenhuma mudança de produto acomodou a
fixture. A norma permaneceu inalterada.

Manifesto ativo `p1331-manifest-r2.json`, SHA-256
`6383d89ef0fccf78290182c1180fccaba290c1a11f36f4fe44ba75df53de81c9`;
norma SHA-256
`ef406128181e4cfa5b802b86ec0a9c58370cc4f7dab09365a088eef74523956a`;
owner canônico
`8d7359a9585e354c370c032b573a7133d004806b3a3c84cd881f8665bb992521`.
Depois do freeze, no L0 mudou apenas a metadata recíproca Hash do Código.
O manifesto, candidato e recibos R1 permanecem como histórico da tentativa.

## Gates e ganho observado

Todos os gates finais R2 terminaram com exit 0, sobre o mesmo inventário
produtivo. Os recibos `p1331-*-r2.json` incluem argv, horários UTC,
HEAD, diff HEAD/stat e inventário antes/depois; não se atribuem os números
a um commit limpo inexistente.

| Verificação | Resultado | Recibo em diagnosticos/ |
| --- | --- | --- |
| Sentinela Path, antes de C2 | 1 aprovado; quatro perfis internos | `p1331-fixture-path-r2.json` |
| RED R2, antes de C2 | 25 aprovados, seis falhas diagnósticas esperadas | `p1331-unit-red-r2.json` |
| GREEN R2 | 31 aprovados, zero falhas | `p1331-unit-green-r2.json` |
| Build release locked | aprovado | `p1331-final-build-r2.json` |
| Workspace release locked | 6.717 aprovados, zero falhas, três doctests ignorados | `p1331-workspace-tests-r2.json` |
| Formatação e diff check | aprovados | `p1331-final-fmt-r2.json`, `p1331-final-diff-check-r2.json` |
| Linhagem e V5/V15/V26 | aprovados, nenhuma violação selecionada | `p1331-final-lineage-r2.json`, `p1331-final-lineage-lint-r2.json` |
| Linter geral SARIF | zero erros, 240 warnings, 1.146 notes | `p1331-final-lint-r2.json` |

GREEN foi medido de `2026-09-09T14:05:14.431022+00:00` a
`2026-09-09T14:07:26.139881+00:00`; workspace de
`2026-09-09T14:09:55.275126+00:00` a
`2026-09-09T14:12:51.129469+00:00`. O total workspace é a soma dos
resumos de testes executados, sem somar novamente o GREEN focal.
SHA-256 do recibo workspace:
`10b439e3bedf72805ff078c10557baa36df7f549eb84d1b7776b9f821035ae83`.

Comparado ao SARIF P1330, sem considerar deslocamento de linhas, nenhum
warning ou achado foi removido; há uma única nota nova V16 no fallback,
`wildcard _ => delega decisão para other.type_name()`. Portanto não se
declara linter sem achados: zero erros não significa zero warnings/notes.
Essa delegação preserva o nome já definido para as demais espécies; só
Str e Bool precisam dos nomes diagnósticos longos locais.

Binário C2: `/tmp/p1331-target.rtY0la/release/typst`, SHA-256
`a8d6e2f4472fefc9e63a123783feac852445191dcb82ac269d366a6419ae1a47`.
O target dedicado ficou em /tmp: o registro `p1331-target.json` explica
que a memória disponível em /dev/shm não comportava a cópia do cache.
Os targets anteriores não foram sobrescritos nem removidos.

### Comparação CLI literal, não uma estimativa de paridade global

O corpus tem 126 casos em quatro perfis: 504 observações por execução.
Ordem normal, repetição normal e ordem reversa produziram exatamente as
expectativas congeladas: **1.512 verificações aprovadas**. Foram comparados
exit, stdout e stderr completos, sem normalização do candidato.

| Medida, em cada execução de 504 observações | BASE P1330 | C2 P1331 |
| --- | ---: | ---: |
| Equivalência integral com vanilla | 252 | 404 |
| Diferenças integrais restantes | 252 | 100 |
| Observações antes equivalentes que regrediram | — | 0 |

O candidato mudou 164 observações: 152 passaram à equivalência integral;
as outras 12 corrigiram o diagnóstico primário, mas preservaram o nome
divergente no trace de `fallback-with-bound`, `fallback-with-nested` e
`fallback-arguments-spread`. Não se contabilizam essas 12 como paridade
integral. Das 100 diferenças restantes, 48 são dívidas de nome no trace
(24 históricas de correção, 12 de overflow, 12 deste fallback) e 52 são
controles de outras dívidas preservadas. Isso descreve este corpus, não
quantifica tudo o que falta na linguagem Typst.

Recibos públicos e SHA-256, com bindings ao manifesto R2 e hashes dos
três binários; os recibos root correspondentes registram o estado completo:

- `p1331-ab-cli-normal-r2.json`, início UTC
  `2026-09-09T14:10:13.681570+00:00`,
  `02134e5dca4d4a36b102af0966eeb613ed52436c19f024eedb7bd25b6f7470f3`.
- `p1331-ab-cli-repeat-r2.json`, início UTC
  `2026-09-09T14:10:18.495878+00:00`,
  `6c3acbef28d159068c8414b1a5c800ddfbc33fbb0f21d8f8dede61f19f6e0981`.
- `p1331-ab-cli-reverse-r2.json`, início UTC
  `2026-09-09T14:10:24.610381+00:00`,
  `2bb25466d3bfa4ba31c7cc6c9f942e32c6512ab2d07b53cb8a9d188600aade5b`.

O aceite final depende também dos pareceres segregados
`p1331-ab-receipt.md` e `p1331-review-final.md`; `p1331-closure.json`
é o fechamento verificável que exige esses pareceres e todos os gates,
e pina seus hashes junto com este relatório. Sem stage, commit ou push.
