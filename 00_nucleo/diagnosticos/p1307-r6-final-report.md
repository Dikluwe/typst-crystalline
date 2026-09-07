# P1307-R6 — implementação entregue, fechamento ainda bloqueado

## Resultado concreto

A autorização foi usada para implementar os encoders e o transporte causal
aprovados, não apenas para produzir documentos. O código está na working tree,
sem stage, commit ou push nesta execução. P1307 ainda **não está aceito**: a
integração encontrou obrigações do oráculo incompatíveis com restrições do L0
congelado, além de uma lacuna de transporte da Source no caminho público de eval.

Implementado:

- `json.encode`, `toml.encode` e `yaml.encode`, respectivos namespaces,
  aliases/With, validação dos argumentos e projeção textual dos valores.
  JSON/TOML usam árvores dos formatos autorizados; YAML usa writer privado.
  Não foi alterada a whitelist de L1. CBOR mantém a classificação anterior.
- `ArgOccurrence` e sequência causal opcional em Args: ordem mista, named
  repetidos, spans distintos do argumento e valor, spread, With, sink,
  map/filter e join Args+Args; migração dos writers/consumers afetados.
- `IntrospectedContent` com snapshot opcional autoritativo. Heading captura
  campos na introspecção; query, acesso direto, métodos, aliases estáticos,
  igualdade da linguagem, repr e serialização CLI preservam esse snapshot.
  Label é campo observável quando presente, mas não participa da igualdade.
- Ajustes de repr aprovados para State, Counter, Location, With e Args.

Entradas principais: `01_core/src/compiler/stdlib/loading.rs:359`,
`01_core/src/entities/args.rs:16`, `01_core/src/entities/value.rs:26`,
`01_core/src/compiler/introspect/heading.rs:21`,
`03_infra/src/query_helpers.rs:459` e `02_shell/src/cli.rs:664`.

## Evidência e proveniência

Todas as medições abaixo são sobre HEAD
`b303f1f15b610e09872b567027e0d806387fde8c` **mais working tree não commitado**.
Cada recibo de comando citado contém UTC inicial/final, argv, cwd, saída
integral, `git diff HEAD --stat`, diff integral antes/depois e status, permitindo
identificar exatamente os arquivos alterados em cada medição. As medições
isoladas de RED declaram separadamente a cópia de fontes realmente compilada.

Baseline R6: `p1307-r6-baseline.json`, SHA-256
`8cc0eae00457d2e7d54b420024eae49344032b34b295b4eda536faeb1ec3c4a3`.
Vanilla ratificado: upstream `a51e02804`, `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Target descartável autorizado: `/dev/shm/p1307-r4-target.8W1BEA`.

### Testes independentes e honestidade do reparo

O primeiro GREEN candidato resultou em 19 testes aprovados e 3 falhas
(`p1307-r6-green-1.json`). Duas falhas decorriam de uma Source incompatível no
MockWorld do harness; outra usava `float.inf`/`float.nan`, indisponíveis no
baseline cristalino. O autor independente reparou apenas essas pré-condições:
instalou no MockWorld a mesma Source parse_code e usou os construtores
`float("1e999")`/`float("NaN")`. Não mudou resultados esperados, mensagens,
hints, spans, traces ou cardinalidade.

O reparo voltou a ser compilado contra cópia isolada do baseline: 2 testes
aprovados e 20 falhas semânticas, exit 101, não falha de compilação
(`p1307-r6-repaired-baseline-red.json`). A integração foi liberada pelo selo
sucessor exclusivamente de fixtures `p1307-r6-successor-seal.json`.
Depois da integração, **22/22 passaram**, exit 0
(`p1307-r6-green-2.json`, SHA-256
`0f078a50a1dbbda91406216186e8bdc976900d8ab65980cca5f9ea25f6e90faf`).
Esse GREEN do harness **não resolve** a falta de Source do CLI descrita abaixo.

### Matriz pública e gates

A primeira matriz completa observou 1906 Preserved, 76 Violated e zero Unknown
em 1982 células (`p1307-r6-public-matrix-1.json`, SHA-256
`9d6dffc3a3ba5c245ae95e9cf896127310a9a81b571735e5e750c2a9909527f8`).
O verificador recalculou todas as classificações sem divergência.
Cinco casos de emissão falharam nos quatro perfis: expoente positivo em
JSON/YAML e aspas internas de TOML multilinha. O candidato foi corrigido por
passes lexicais restritos a tokens numéricos/strings basic multilinha;
strings JSON não sofrem substituição global. Os resultados posteriores
confirmaram a correção:

| Gate | Resultado | Recibo |
|---|---|---|
| Focal das emissões e controles | 72 Preserved, 0 Violated, 0 Unknown | `p1307-r6-public-focal-2.json` |
| Matriz completa após correção | **1926 Preserved, 56 Violated, 0 Unknown** | `p1307-r6-public-matrix-2.json` |
| `cargo build --workspace --release` | exit 0 | `p1307-r6-build-2.json` |
| `cargo test --workspace --release --no-fail-fast` | **exit 101**; 6599 aprovados, 3 falhas, 3 ignorados | `p1307-r6-workspace-tests.json` |

O focal foi executado em `2026-09-07T19:48:39.631654+00:00` e a matriz em
`2026-09-07T19:49:04.797215+00:00`. Ambos usaram o binário SHA-256
`16aeeec4783aa8fa66821a50a3d1a1d4fdebc279602253a1bf731bcb6c8eb7be`.
O recibo da matriz final tem SHA-256
`53737a33e636323f4e0ab50499bde3cbb0905fe50889cdc5d3d7c5d631f6e51c`.
As 20 células corrigidas são os cinco casos de emissão nos quatro perfis;
restam os 14 casos das primeiras três fronteiras abaixo, também nos quatro perfis.
O runner de replay retorna exit 0 mesmo com Violated: a decisão acima foi
extraída de `stdout.counts` e das linhas individuais, não do exit do runner.

### Falhas adicionais da suíte histórica

A suíte ampla começou em `2026-09-07T19:45:53.001471+00:00`; seu recibo tem
SHA-256 `0043e5ad03f03e02d6456ad7fed6e5a0c9552f2b9de6bb2b98aeeb95090ef25b`.
As falhas não foram ocultadas nem tiveram expectativa alterada:

- `repr_value_complex_types`, `eval/repr.rs:1300`: espera `location(...)`,
  recebe `location(..)`; o L0 atual de repr:600 exige a segunda forma.
- `p1305_args_fields_remain_integral`, `eval/tests.rs:19447`: espera Args
  longo inline, recebe todos os campos em forma multilinha. Não houve
  elisão dos valores, mas isso **não basta para legitimar o delta de forma**:
  L0 repr:532 trata de Array, não Args; repr:604–607 explicita ordem causal,
  não a adoção geral de multiline. Este caso exige auditoria da legitimidade
  do formatter, não simples atualização de expectativa.
- `p1293_d_ten_short_names_with_calls_and_flat_aliases_preserved`,
  `04_wiring/tests/p1293_contract.rs:980`: espera nomes das funções internas
  para With nativo, recebe `(..) => ..`; L0 repr:601 exige With anônimo.

Location e With têm supersessão normativa explícita; Args longo ainda tem
uma fronteira de escopo a resolver. A revisão independente refutou minha
classificação inicial de todos os três como testes obsoletos. A suíte
existente ainda está vermelha: migração independente dos dois primeiros
contratos claramente supersedidos e auditoria específica de Args continuam
pendentes. Não é lícito anunciar "todos os testes passaram" a partir dos
22 testes novos.

O linter integral final terminou com exit 0 (`p1307-r6-lint-3.json`), mas emite
warnings/info: **não** é alegação de zero violations ou ausência de débito.
`cargo fmt --all -- --check` terminou com exit 0
(`p1307-r6-fmt-final.json`). Os hashes recíprocos de código foram atualizados
somente nas linhas de metadados autorizadas; o texto normativo do L0 e o
oráculo continuam congelados.
`p1307-r6-final-integrity.json` registra verificação posterior sem nenhuma
correção de hash de código restante. Durante o GREEN dos 22 testes houve
apenas atualização de oito metadados de prompts; nenhum Rust mudou.

## O que impede fechar — decisão concreta pendente

| Fronteira | Medição e contrato | Decisão necessária |
|---|---|---|
| `Args + none` | `r4.args.join-none` exige sucesso; `compiler/eval/operators/arithmetic.md:181–186` autoriza somente Args+Args e exclui expressamente Args+None. Baseline e candidato rejeitam a soma. | Incluir esse par no escopo L0 ou retirar legitimamente a obrigação do incremento. Não há implementação que cumpra ambos simultaneamente. |
| Diagnósticos de callbacks | `r4.error.map-first-callback`, `filter-first-callback` e `filter-non-bool` exigem outras âncoras e, no último, `integer` em vez de `int`. `compiler/stdlib/collections.md:498–501` preserva mensagens/fronteiras anteriores. A ordem causal já mudou corretamente para o primeiro callback. | Autorizar correção dessas superfícies diagnósticas nos owners pertinentes ou conservar explicitamente o débito no contrato. Não ampliar todos os diagnósticos por conveniência. |
| Traces públicos de eval | Em dez casos, stderr observado é prefixo exato do esperado, faltando apenas o trecho `while calling`. `eval/mod.rs:347–363` cria Source local, mas Engine recebe World original; `call_dispatch.rs:1017–1023` precisa resolver a Source pelo World e não inventa trace quando não consegue. | Refinar o L0 do transporte causal de Source do entrypoint, preservando pureza e a regra de contenção. O reparo do MockWorld não é solução produtiva. |
| Forma de Args longo, detectada fora da matriz R6 | O teste histórico P1305 espera inline, enquanto o novo formatter causal usa multiline. A cláusula de formato de Array não autoriza automaticamente mudar Args. | Tornar explícita a decisão de formato no contrato de Args/repr e sua relação com a referência ratificada, ou corrigir a extrapolação do candidato. Não tratar a falha como expectativa obsoleta sem prova. |

O checkpoint independente `p1307-r6-verification.json` registra esses conflitos
e reconhece que o pré-selo **não os detectou**. O pré-selo não prova coerência
entre todos os L0 e o oráculo. Nenhuma falha foi reclassificada como Unknown
ou apagada para obter aceitação.

O adendo independente final `p1307-r6-verification-final.json`, SHA-256
`48c89a1513ff0f6b56f79b286f1e81557b4bc5272423cd546a28a41b4edb1062`,
confirma a matriz corrigida, a integração canônica dos testes, o GREEN focal,
as três falhas da suíte ampla e os gates de build/fmt/lint/linhagem. Seu
veredito é `BLOCKED_CONTRACT_BOUNDARIES_AND_WORKSPACE_TEST_FAILURE`.
Não emite certificado nem transforma resultados parciais em PASS geral.

## Limites desta entrega

Regime exigido pela skill `tekt-materializacao-segregada`: **executado sem
atestação de isolamento técnico**. Autor de testes, implementadores e
verificador tiveram responsabilidades separadas, mas a árvore é compartilhada.
Hashes demonstram identidade dos artefatos, não isolamento de capacidades.

Não foram executados mutantes Rust reais nesta revisão. O plano de mutação e
os controles negativos de envelopes não são mutation testing do produto.
Não há mutation score de código, certificado, nem PASS geral. Repetições e
ordem inversa finais permanecem pendentes de resolver o contrato incoerente.

O fragmento aprovado não completa presença de todos os campos raw/estilizados,
callbacks/custom supplement/numbering, máscaras de depth/offset ou região de
tradução. Também não corrige globalmente repr de dicionários. Esses limites
não podem ser inferidos como paridade geral a partir dos exemplos que passaram.

Próxima ação depende da escolha do dono sobre as fronteiras acima;
depois dela, atualizar L0 primeiro e reabrir somente as etapas afetadas antes
de implementar e certificar. Esta parada é por incompatibilidade concreta de
intenção/contrato, não por ausência de implementação.
