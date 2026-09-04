# P1301 — recibo do oracle vanilla segregado

## Resultado

**Estado:** `P1301_ORACLE_COMPLETE_WITH_INFORMATIVE_VANILLA_DIVERGENCE`

O corpus canónico foi executado no vanilla ratificado nas ordens normal e
invertida. Em cada ordem, `22/23` casos obrigatórios foram `Preserved`, nenhum
foi `Unknown` e um foi `Violated`. Os mapas de observáveis e de vereditos por
`case_id` foram idênticos entre as duas ordens.

A divergência é o sentinela
`S-NONMODULE-dictionary-whole-span`. O contrato congelado exige range
`10..21` e stderr na coluna `10`, mas o vanilla ratificado publicou coluna
`17`, correspondente ao identificador público `nope` em `17..21`:

```text
<input-expression>:1:17: error: dictionary does not contain key "nope"
```

Esta expectativa **não foi adaptada** ao resultado. Os casos ficam
explicitamente particionados: `22` são fontes de oracle vanilla e todos foram
`Preserved`; `S-NONMODULE-dictionary-whole-span` é uma invariável de
preservação cristalina cuja fonte é
`oracle_source=contract_pre_candidate_invariant`. A sua execução vanilla é
divergência informativa, continua `Violated` neste recibo e não substitui a
expectativa do gate candidato posterior, que permanece `10..21`.

## Regime, papel e isolamento

- Regime: protocolo completo da materialização segregada Tekt.
- Papel: autor de oráculos P2, executor `/root/p1301_oracles`.
- Linguagem proporcional obrigatória: **executado sem atestação de isolamento técnico**.
- Entradas permitidas efetivamente lidas: `AGENTS.md`, manifesto P1301, os dois
  Prompts L0 P1301, contrato P1301 e o binário vanilla ratificado. Também foram
  lidas as instruções operacionais da skill obrigatória
  `tekt-materializacao-segregada`; elas não deram acesso a artefatos do produto.
- Saídas gravadas, ambas exclusivamente via `apply_patch`:
  `00_nucleo/diagnosticos/p1301-oracle-suite.json` e este recibo.
- Não foram lidos código ou testes candidatos, Rust pré-candidato, artefatos
  P1300, saídas privadas do adversário, implementação cristalina nem testes
  cristalinos. `repr(std)` não foi executado.
- Limitação: os agentes partilham filesystem e contexto conversacional; não
  houve sandbox/allowlist de sistema operacional independente que provasse a
  ausência de acesso. Os comandos e a disciplina de leitura dão
  rastreabilidade, mas não constituem atestação técnica de isolamento.

## Entradas congeladas e rehash

Os hashes foram revalidados antes da execução normal, antes da invertida e ao
final. Permaneceram iguais nos três gates:

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1301-manifest.json` | `074fabeda17f741f064fe1d561ad6a78ad81e45654861825586664a9646f05a8` |
| `00_nucleo/diagnosticos/p1301-contract.json` | `2d8d7a141d63c13473681abdefa24ed9370e03925824a9cae74ca26305ea85f5` |
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `38d6f5cdee302491ec059577174d4f6dc6d3c28e3d2da2748f61c05853cc0c16` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `a4ade7eda600891465c62c320d80b7c2182c30dbb5f456de210c4a823b3e609a` |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

O baseline identifica o upstream ratificado `a51e02804`. A proveniência do
repositório vem do manifesto congelado: HEAD
`1f082370e59939de7b57992e137a9f74bfb6758f`, working tree não commitida,
`git status --short` pré-manifesto de SHA-256
`3ea93237c50446334d7008832632dc262cceb3bf8e3cc25da1c1d91e3e6b8d4f`
e `git diff HEAD --stat` pré-manifesto de SHA-256
`df861a696777f2714969a76128a6c5deafd7383537223c9dbb1efc2f1f1e12b6`.
O estado atual da árvore não foi inspecionado, para não atravessar a fronteira
que proíbe leitura de candidato.

## Comandos, perfis e processos

Comandos de rehash executados:

```text
sha256sum AGENTS.md 00_nucleo/diagnosticos/p1301-manifest.json 00_nucleo/diagnosticos/p1301-contract.json /usr/local/bin/typst
sha256sum 00_nucleo/prompts/compiler/eval/bindings/field_access.md 00_nucleo/prompts/compiler/eval/tests.md
```

Cada caso foi executado num processo vanilla novo com a forma exata:

```text
/usr/local/bin/typst --color never eval <profile-argv...> --diagnostic-format short <expression>
```

Ambiente fixado para os filhos: `LC_ALL=C.UTF-8`, `LANG=C.UTF-8`, `TZ=UTC`.
Perfis:

| Perfil | argv | Processos por ordem |
|---|---|---:|
| `default` | `[]` | 14 |
| `html` | `["--target","html","--features","html"]` | 3 |
| `a11y` | `["--features","a11y-extras"]` | 3 |
| `html+a11y` | `["--target","html","--features","html,a11y-extras"]` | 3 |

O runner foi o PID `3`. A janela total foi
`2026-09-04T00:10:29.396435Z`–`2026-09-04T00:10:29.755237Z`. Foram criados
`46` processos vanilla: `23` na ordem normal e `23` na invertida. PIDs normais:
`4,21,38,55,56,57,58,75,92,109,110,111,112,129,146,163,180,197,214,231,248,265,282`.
PIDs invertidos:
`299,316,333,350,367,384,401,418,435,452,469,486,487,488,489,506,523,540,541,542,543,560,577`.

O JSON da suite registra, para cada processo, `case_id`, PID, início, fim e
duração monotónica em nanossegundos, além do comando exato por caso, perfil,
expressão, stdout, stderr, exit code e observáveis diagnósticos.

O formato `short` publica diretamente apenas o início do span. O fim
half-open foi mapeado a partir do identificador final na expressão pública. No
sentinela, a coluna `17` aponta para o token `nope`, que ocupa `17..21`; a
expectativa cristalina `10..21` não foi usada para deslocar esse início.

## Vereditos por ordem

| Ordem | Preserved | Violated | Unknown obrigatório | Execution Unknown |
|---|---:|---:|---:|---:|
| normal | 22 | 1 | 0 | 0 |
| invertida | 22 | 1 | 0 | 0 |

- Gate de invariância por ordem: observáveis e vereditos idênticos por
  `case_id` — `PASS`.
- Partição oracle vanilla: `22/22 Preserved` — `PASS`.
- Partição de invariável pré-candidata: o sentinela tem
  `oracle_source=contract_pre_candidate_invariant`; sua execução vanilla é
  `Violated` informativo, e o gate candidato posterior continua a exigir
  `10..21`.
- Divergência determinística nas duas ordens:
  `S-NONMODULE-dictionary-whole-span`, primeiro observável distinto
  `span_bytes`, esperado `10..21`, vanilla `17..21`; stderr esperado na coluna
  `10`, vanilla na coluna `17`.
- Os `22` casos restantes coincidiram byte a byte nos observáveis exigidos,
  inclusive warnings dos perfis HTML.
- Mutation gate: não executado por P2; pertence ao posterior autor do gate
  discriminatório. Nenhuma conclusão de mutation score é alegada aqui.

## Opacos materializados

As fixtures foram copiadas literalmente do contrato, sem execução nem
normalização:

- `O-AMBIGUOUS-PRIMARY` → `Unknown`, reason code exato
  `AMBIGUOUS_PRIMARY_DIAGNOSTIC`.
- `O-UNMAPPABLE-SPAN` → `Unknown`, reason code exato
  `SPAN_CANNOT_BE_MAPPED_WITH_PUBLIC_INPUT`.

O gate de opacidade é `PASS`: os dois e somente os dois controles declarados
opacos resultam em `Unknown`; nenhum caso obrigatório foi convertido em
`Unknown`.

## Artefato

`00_nucleo/diagnosticos/p1301-oracle-suite.json` tem SHA-256
`4182575a0c11e39aed01f05c827d3cb59522f69ab1795f017523586f4c19d012`.
O JSON foi validado após a escrita com `23` casos, `23 + 23` processos e `2`
opacos `Unknown`.

Este recibo atesta somente o fragmento observável e as entradas acima. Não
alega equivalência funcional geral, não seleciona implementação e não sela o
contrato divergente.
