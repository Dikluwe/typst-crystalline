# P1300 — recibo independente de medição vanilla (P2)

## Veredito do papel

`ORACLE_SUITE_COMPLETE` — protocolo Tekt completo executado **sem atestação de
isolamento técnico**. A medição observou exclusivamente a superfície pública do
binário vanilla ratificado; não leu implementação candidata, testes permanentes,
mutantes nem saídas P3/P4–P8. Não há `Unknown` semântico, timeout ou identidade
ambígua. Os quatro casos opacos permanecem explicitamente `Unknown` e não foram
executados como semântica do produto.

Este recibo e a suíte atestam somente o fragmento observável P1300. Não alegam
equivalência funcional geral da stdlib ou de `color`.

## Entradas congeladas e identidade

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1300-manifest.json` | `c3e946e1330ccab108dba2f9a438fd6a8d4c3a2af7fca3307842dbd7403feed8` |
| `00_nucleo/diagnosticos/p1300-contract.json` | `c5eace73e5ff07ab49057ec1741283543408accf132ca2f4431a1ad3827a3e37` |
| `00_nucleo/prompts/compiler/stdlib/color.md` | `8115021062602c8a3663797b3fcfe0f17eb423f54a8ae7b437b260804ad58462` |
| `00_nucleo/prompts/compiler/eval.md` | `3bf911a0882e35f713cf8742f70ea921086a9a95ede6b02b8b938ab92becfc96` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `d4d6382d351e084869445fe58e1cb484e762d5b5dfc843b7974cd21149289104` |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

Identidade normativa do oracle: revisão vanilla `a51e02804`, binário acima.
`/usr/local/bin/typst --version` devolveu `typst 0.15.1 (e0e8ca4d)`; essa string
é apenas informativa e não substitui o SHA-256 pinado.

## Capacidades e allowlists efetivas

- Papel: `P2_oracle_author`.
- Leituras de repositório: apenas manifesto, contrato e os três L0 listados
  acima. As instruções operacionais da skill Tekt foram lidas fora do
  repositório por exigência do ambiente.
- Execução de produto: somente `/usr/local/bin/typst`, depois da confirmação do
  SHA-256 pinado.
- Escritas de repositório: somente
  `00_nucleo/diagnosticos/p1300-oracle-suite.json` e este recibo.
- Temporários: `/tmp/p1300_oracle_runner.py`, saída intermediária sob `/tmp` e
  diretório temporário fresco por invocação.
- Contexto: tarefa P2 fresca após o hash do contrato ter sido congelado; o
  filesystem é compartilhado, logo isolamento técnico não é atestado.
- Proibições cumpridas: nenhum `01_core/**/*.rs`, patch candidato, teste
  permanente, mutante ou conteúdo produzido por P3/P4–P8 foi lido; nenhum
  oracle foi adaptado ao candidato.

## Ambiente e protocolo de execução

Janela da execução congelada: `2026-09-03T22:26:36.210463+00:00` a
`2026-09-03T22:26:36.785419+00:00`. Plataforma registrada na suíte:
`Linux-6.17.0-42-generic-x86_64-with-glibc2.39`; Python `3.12.3` foi usado
somente como harness binding-free.

Ambiente explícito de cada processo:

```text
PATH=/usr/local/bin:/usr/bin:/bin
LANG=C.UTF-8
LC_ALL=C.UTF-8
TZ=UTC
```

Template semântico:

```text
/usr/local/bin/typst eval <expression> --format json [--features <profile>]
```

Perfis exatos:

| Perfil | Features | Sufixo argv |
|---|---|---|
| `default` | `[]` | `[]` |
| `html` | `["html"]` | `["--features", "html"]` |
| `a11y` | `["a11y-extras"]` | `["--features", "a11y-extras"]` |
| `html+a11y` | `["html", "a11y-extras"]` | `["--features", "html,a11y-extras"]` |

Timeout: `10000 ms`. Encoding: UTF-8. Comparação: bytes exatos, sem
normalização. Cada uma das 84 invocações registra argv resolvido, features,
exit, stdout, stderr, completude, duração e SHA-256 do binário. O conjunto
canônico de argv tem SHA-256
`765babad598c257ba960bcdd2d9703bdd440dc37abe1e13b5c59c01cabc6db53`.

Diagnósticos negativos foram executados individualmente para preservar span e
hints. Probes positivas foram agrupadas por família e perfil, mantendo cada
expressão e resultado lógico identificados no catálogo. Os bytes exatos ficam
na tabela `blobs`, endereçados por SHA-256; cada invocação aponta para os blobs
de stdout/stderr.

Comandos de calibração pública anteriores à execução congelada, todos contra o
mesmo binário e ambiente explícito:

```text
/usr/local/bin/typst eval 'repr(type(color.map))' --format json
/usr/local/bin/typst eval 'repr(color.map)' --format json
/usr/local/bin/typst eval 'repr(color.map.fields())' --format json
/usr/local/bin/typst eval 'color.map.viridis' --format json
/usr/local/bin/typst eval 'repr((type(color.map.viridis), color.map.viridis.len(), color.map.viridis.first(), color.map.viridis.last()))' --format json
/usr/local/bin/typst eval 'color.map.viridis.map(c => c.to-hex())' --format json
/usr/local/bin/typst eval 'repr((type(color.map.viridis), color.map.viridis.len(), color.map.viridis.first().to-hex(), color.map.viridis.last().to-hex()))' --format json
```

`color.map.fields()` foi rejeitado pelo vanilla; módulos não oferecem uma
operação pública de enumeração de fields. Portanto a ordem fechada dos 15 nomes
vem do L0 confirmado, enquanto o oracle exerce, nessa ordem, lookup público,
kind, sequência, contagem, extremos e digest de cada filho. Isso não foi
convertido em `Unknown`: `closed_inventory` tem autoridade `confirmed_l0`, e
todos os observáveis `oracle_suite` declarados para os filhos estão completos.

## Cobertura e resultados

Foram executados 84 processos, correspondendo a 21 grupos em cada um dos quatro
perfis, e 932 probes lógicas expandidas por perfil (`233 × 4`):

| Família | Probes lógicas expandidas |
|---|---:|
| 8 negativos (6 controles pinados + 2 spellings alternativos) | 32 |
| 3 rotas qualificadas, type/repr/call, mais 3 `color.space` | 48 |
| 5 globals × bare/std × type/repr/call | 120 |
| 18 cores × 6 observáveis | 432 |
| `cyan`, `magenta`, `none` conforme contrato | 32 |
| 12 operadores × type/repr/call | 144 |
| `color.map`: root + 15 × kind/valores | 124 |
| **Total** | **932** |

Resultados de processo: `28` exits `0`, `56` exits `1` esperados, `0`
timeouts e `0` invocações semanticamente incompletas. As 56 rejeições são os
32 negativos e as 24 sondas vanilla de `cyan`/`magenta`. Duração acumulada:
`560482622 ns`; mínimo `5721175 ns`; máximo `12813196 ns`.

Os 21 grupos produziram exit/stdout/stderr/diagnóstico idênticos nos quatro
perfis. O vetor canônico dessas observações tem SHA-256
`c5f64661ab1ef84729cbed45c4687a7b145ca14e74ea9a580d702e9236d74cfb`.
Há 23 blobs únicos, totalizando 32372 bytes: 8 stdout distintos e 15 stderr
distintos.

Constatações materiais:

- Os seis negativos já pinados coincidem exatamente com exit, stdout, classe,
  mensagem, hints, span renderizado e stderr do contrato em todos os perfis.
- `std.linear-rgb` é field ausente com mensagem
  `module global does not contain linear-rgb`; bare `linear-rgb` é variável
  desconhecida e inclui o hint ordenado sobre `linear - rgb`. Os bytes exatos
  estão congelados na suíte.
- As três rotas qualificadas preservam type/repr/call; os cinco constructors
  globais preservam bare/std type/repr/call; os três `color.space` devolvem as
  identidades esperadas.
- Os 60 pares mapa×perfil coincidem em root/child kind, count, first, last e
  SHA-256 canônico com o L0 para todos os 15 mapas.
- As 18 cores e os 12 operadores têm suas representações, componentes e calls
  públicas congeladas para os quatro perfis.
- `cyan` e `magenta` são rejeitados pelo vanilla, mas a expectativa candidata é
  `baseline_preservation_expectation_from_L0`; essa divergência intencional não
  é falha. `none` é aceito pelo vanilla e continua sob a mesma autoridade L0.
- Casos opacos congelados, não executados: `opaque-timeout → Unknown/timeout`,
  `opaque-missing-product → Unknown/missing_product`,
  `opaque-ambiguous-identity → Unknown/ambiguous_product_identity` e
  `opaque-unsupported-parser → Unknown/unsupported_parser_construction`.

## Artefactos e hashes de saída

- `00_nucleo/diagnosticos/p1300-oracle-suite.json`: 462836 bytes, SHA-256
  `a9a519a8230e39c55fa07223e306a7156d6c2b07bd71ef2926cc9e43f1ce8278`.
- O hash deste recibo é calculado externamente após a escrita, porque um ficheiro
  não pode conter de forma não paradoxal o seu próprio digest integral.

## Proveniência da working tree

Snapshot final capturado em `2026-09-03T19:29:22.476329642-03:00`:

- HEAD: `1f082370e59939de7b57992e137a9f74bfb6758f`.
- Branch: `Tekt`.
- Working tree: não commitada.
- `git status --short` SHA-256:
  `ce621c74345d6d11796fdc869e1cf2df3c4d2a195306c81841d4dcc90aa18f1b`.
- `git diff HEAD --stat` SHA-256:
  `afad297b69637c019ad049bf6fee6ef41d9fc54dae400f993eea99546b28bcd3`.
- Index sem alterações; `git diff --cached --stat` vazio, SHA-256
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.

Conteúdo exato de `git diff HEAD --stat`:

```text
 00_nucleo/prompts/compiler/eval.md         | 37 +++++++++++++++++++++++++
 00_nucleo/prompts/compiler/eval/tests.md   | 41 ++++++++++++++++++++++++++++
 00_nucleo/prompts/compiler/stdlib/color.md | 44 ++++++++++++++++++++++++++++--
 3 files changed, 120 insertions(+), 2 deletions(-)
```

Conteúdo exato de `git status --short`:

```text
 M 00_nucleo/prompts/compiler/eval.md
 M 00_nucleo/prompts/compiler/eval/tests.md
 M 00_nucleo/prompts/compiler/stdlib/color.md
?? 00_nucleo/diagnosticos/p1299-baseline-status.txt
?? 00_nucleo/diagnosticos/p1299-certificate.json
?? 00_nucleo/diagnosticos/p1299-crystalline-default.json
?? 00_nucleo/diagnosticos/p1299-crystalline-html.json
?? 00_nucleo/diagnosticos/p1299-decision-report.md
?? 00_nucleo/diagnosticos/p1299-feature-matrix.json
?? 00_nucleo/diagnosticos/p1299-inventory-default.json
?? 00_nucleo/diagnosticos/p1299-inventory-html.json
?? 00_nucleo/diagnosticos/p1299-manifest.json
?? 00_nucleo/diagnosticos/p1299-owner-ledger.tsv
?? 00_nucleo/diagnosticos/p1299-probe-catalog.json
?? 00_nucleo/diagnosticos/p1299-run-matrix.py
?? 00_nucleo/diagnosticos/p1300-adversarial-plan.md
?? 00_nucleo/diagnosticos/p1300-contract-author-receipt.md
?? 00_nucleo/diagnosticos/p1300-contract.json
?? 00_nucleo/diagnosticos/p1300-l0-gate-receipt.md
?? 00_nucleo/diagnosticos/p1300-manifest.json
?? 00_nucleo/diagnosticos/p1300-mutants.json
?? 00_nucleo/diagnosticos/p1300-oracle-suite.json
?? 00_nucleo/diagnosticos/p1300-pre-gate-measurement.json
?? 00_nucleo/diagnosticos/p1300-vanilla-measurement-receipt.md
?? 00_nucleo/diagnosticos/test_p1299_run_matrix.py
?? 00_nucleo/materialization/typst-passo-1299.md
?? 00_nucleo/materialization/typst-passo-1300.md
```

Esse snapshot foi usado somente como proveniência; nenhum conteúdo fora da
allowlist P2 foi aberto. Não houve staging, commit ou push.

## Gates do papel

- Hashes do manifesto, contrato, L0 e oracle: `PASS`.
- Identidade do oracle: `PASS`, não ambígua.
- Execução em quatro perfis: `PASS`.
- Completude dos observáveis públicos: `PASS`.
- Invariância entre perfis: `PASS`.
- Controles negativos com spans/hints: `PASS`.
- `color.map` count/extremos/digests: `PASS`.
- Política crystalline-only separada do vanilla: `PASS`.
- Opacos não executados e `Unknown` preservado: `PASS`.
- Limitação de atestação: filesystem compartilhado; protocolo executado sem
  atestação de isolamento técnico.
