# P1285 — plano adversarial de mutações v2 pré-candidata

**Estado:** `INVALIDATED_BY_P1285_CONTRACT_V3`  
**Regime:** protocolo completo de materialização segregada.  
**Papel:** adversário/mutation tester, sem autoria da obrigação, da candidata,
dos testes congelados, dos oráculos ou do veredito.  
**Instante:** `2026-08-30T14:55:22-03:00`.  
**HEAD observado sem leitura de diff:**
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.

> Invalidado pelo contrato v3
> `1ea58e14a01f6cb55fe38695133f4c279164ca2943134d2dcf5cab81c3df681e`, que
> acrescentou dois owners produtivos e M-S7…M-S10. Sucessor:
> `00_nucleo/diagnosticos/p1285-adversarial-mutation-plan-v3.md`.

Este documento é diagnóstico e plano executável em duas fases. Não é Prompt L0,
mutante aplicado, score, selo ou veredito. A fase sintática que transforma o código
só pode ser preenchida depois de o coordenador declarar a candidata congelada e
fornecer os hashes dos consumers candidatos.

## 1. Capacidades efetivas

Entradas lidas: `AGENTS.md`/`CLAUDE.md` idênticos, `01_core/CLAUDE.md`, skill
`tekt-materializacao-segregada` e suas duas referências diretas, ADRs
0107/0108/0127/0129, os seis L0 P1285 finais, contrato observável v2 e
receipt RED apresentado como v2.

Não foram lidos: `00_nucleo/materialization/`, `00_nucleo/context/`, baseline,
oráculos, runner, implementação candidata ou diffs. Nenhum ficheiro produtivo,
teste congelado, L0, contrato ou oráculo foi editado.

Hashes protegidos relevantes:

| Entrada | SHA-256 |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| skill | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` |
| `papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| `artefatos-e-gates.md` | `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |
| ADR-0107 | `e680d22bbf4486cf93f5bfb4ec85f4ae965e6db788c2a18c48f3be000029d49d` |
| ADR-0108 | `31daec5ae9e84cb5bbdcb806e9a2b6cb9160b7e90519df53e6e0bd8809076405` |
| ADR-0127 | `5e8581b5f9ebb0798d4213e59e39ee8dfcd4f41b34d4ca639b00b1f287699ad9` |
| ADR-0129 | `64756b81ce58ca62e1a166b3776303759bc7af507a1c97a4e3ad91a8dc5b906e` |
| `shell/cli.md` | `fa2d3c4471822550994dfe2e4d23e503925efd9a067aac2e0054ad2c77ae004c` |
| `compiler/eval.md` | `1fb66ec889304cd3a21accd84b009388cd1613c6fc613937c3af60015483cf56` |
| `compiler/eval/repr.md` | `a2b35c8b3e622432b5a129a4cad0bb20ab94b450d3e793fd1441c5af6bc2775c` |
| `compiler/eval/selector_matching.md` | `1de6b662299ba6dc1492d3230cd4e12c732dcc5648a288bbd0ea26b9dbfd7c4d` |
| `infra/query-helpers.md` | `42debc22810cf3eebf16185d91543cedc36be4e179221f5b6d15b8f2ff6c6705` |
| `wiring.md` | `f8db4e993fd1931d8f77b69a2ec416377f0700569e3942cc78e32daf945e0ec8` |
| contrato v2 | `94bb0f0115e5179b3c744834402058343a53792df8eae1c69102cad403595aa9` |
| receipt RED atual | `ceb14510b2e5caafe9d7c7cfce346d2a2da985111415d298f442e07c938ed7f6` |

## 2. Inconsistência causal bloqueante para atestação

O receipt RED atual declara ter sido reconstruído após o restart do sexto owner,
mas a sua tabela de entradas pina o contrato como
`9897a6f893734b66b2d1c93ee653bc1155ab0c51f705c89f22011ee73a3c77ec`, hash que o
próprio contrato v2 identifica como v1 invalidado. O contrato v2 efetivo tem hash
`94bb0f…`. Logo os testes podem ser usados como evidência RED histórica e como
targets independentes, mas não como receipt causal v2 atestado até serem repinados
e repetidos contra `94bb0f…`. A matriz pode ser preparada; score/selo/veredito não.

## 3. Ownership lógico dos testes

Cada ID abaixo tem um único teste proprietário. Um teste pode matar vários mutantes,
mas nenhum mutante fica sem owner. Os nomes `A-*` são casos obrigatórios da suite
black-box selada e devem ser ligados pelo verificador ao runner sem revelar o seu
código ao adversário. Os nomes `F-*` são suites/testes congelados cuja existência e
comandos constam do receipt RED.

| Teste owner | Observável | Comando focal/adapter do verificador |
|---|---|---|
| `A-Y-HELP-EVAL` | inventário `json,yaml,raw` | runner selado: caso help eval |
| `A-Y-TREE` | YAML composto equivale à árvore JSON | runner selado: eval composto |
| `A-Y-PRETTY` | YAML idêntico com/sem pretty | runner selado: par YAML |
| `A-Y-QUERY-RAW` | raw em query, exit 2 | runner selado: query raw |
| `F-L4-STDIN-YAML` | transporte L4 + stdin markup | `cargo test -p typst-wiring --test cli p1285_query_stdin_transporta_yaml_ate_stdout -- --nocapture` |
| `A-J-VERSION-ACCEPT` | Version serializável em JSON | runner selado: `sys.version` |
| `A-J-VERSION-EXACT` | `version(0, 15, 1)` exato | runner selado: `sys.version` |
| `A-J-STRUCTURAL` | array/dict/content preservam árvore | runner selado: composto/content |
| `A-J-NOMINAL-REPR` | gradient/selector/regex usam repr pública | runner selado: sentinelas nominais |
| `A-J-RAW-REJECT` | raw Version falha nominalmente | runner selado: raw Version |
| `A-Q-NONHEADING` | serializer aceita figure/equation/metadata/quote | runner selado: matriz não-heading |
| `F-L3-NONHEADING` | recupera metadata/figure | `cargo test -p typst-infra p1285_query_elements_ -- --nocapture` |
| `A-Q-SHAPE` | `func` + fields de figure/equation | runner selado: shapes não-heading |
| `A-Q-ORDER` | quotes First, Second | runner selado: duas quotes |
| `A-Q-FIELD-SPLIT` | lista filtra; one erra | runner selado: field ausente em dois modos |
| `A-Q-ONE-FIRST` | cardinalidade antes do field | runner selado: 0/2 + field |
| `F-L2-LABEL-SERIALIZE` | wrapper não vira `func: label` | `cargo test -p typst-shell p1285_ -- --nocapture` |
| `F-L3-LABEL` | wrapper preservado sem match extra, controle | `cargo test -p typst-infra p1285_query_elements_ -- --nocapture` |
| `A-Q-STDIN-EQUAL` | ficheiro e stdin têm mesma árvore | runner selado: execução pareada |
| `A-Q-STDIN-IDENTITY` | markup contextual e diagnóstico resolvível | runner selado: stdin contextual/erro |
| `A-S-DOT` | literal `.` vs regex `.` | runner selado: dupla discriminatória |
| `A-S-ALL` | todas as ocorrências e fatias | runner selado: literal/regex repetidos |
| `A-S-NONMATCH` | não-match não transforma | runner selado: regex `z+` |
| `A-S-EMPTY` | vazio/match vazio falha | runner selado: três casos vazios |
| `A-S-NONLOCATABLE` | query text/regex falha | runner selado: selectors não locatáveis |
| `A-S-PARTIAL` | literal parcial `xfooY` | runner selado: show parcial |

Controles congelados adicionais, executados para toda mutação no seu crate:

```sh
cargo test -p typst-shell p1285_ -- --nocapture
cargo test -p typst-core p1285_selector_ -- --nocapture
cargo test -p typst-infra p1285_query_elements_ -- --nocapture
```

## 4. Matriz fechada de mutações

Cada operador deve ser materializado como patch mínimo compilável numa cópia
efémera do snapshot candidato. A coluna owner determina o teste cujo resultado mata
o mutante; falha acidental de build não substitui essa testemunha observável.

| ID | Operador semântico a materializar após freeze | Owner | Mutante válido é morto quando |
|---|---|---|---|
| M-Y1 | remover `Yaml` apenas de eval/help | `A-Y-HELP-EVAL` | inventário diverge |
| M-Y2 | despachar YAML pelo serializer JSON | `A-Y-TREE` | stdout não é a árvore/formato YAML esperado |
| M-Y3 | fazer `pretty` selecionar forma YAML diferente | `A-Y-PRETTY` | par com/sem pretty diverge |
| M-Y4 | aceitar `raw` no parser de query | `A-Y-QUERY-RAW` | exit deixa de ser 2 |
| M-Y5 | em L4 substituir formato da query por JSON | `F-L4-STDIN-YAML` | fluxo completo não produz YAML |
| M-J1 | rejeitar `Value::Version` no estruturador | `A-J-VERSION-ACCEPT` | comando deixa de ter sucesso/string |
| M-J2 | usar Display/Debug `0.15.1` para Version | `A-J-VERSION-EXACT` | string difere de `version(0, 15, 1)` |
| M-J3 | converter array/dict/content inteiro em repr string | `A-J-STRUCTURAL` | tipo da árvore deixa de ser array/mapa |
| M-J4 | trocar fallback nominal por Debug ou erro | `A-J-NOMINAL-REPR` | qualquer sentinela nominal diverge/falha |
| M-J5 | permitir Version no raw via repr | `A-J-RAW-REJECT` | raw deixa de falhar com exit 1 |
| M-Q1 | restaurar guarda heading-only | `A-Q-NONHEADING` | testemunha não-heading falha/some |
| M-Q2 | omitir `func` ou um field público obrigatório | `A-Q-SHAPE` | shape figure/equation perde chave |
| M-Q3 | ordenar, inverter ou deduplicar resultados | `A-Q-ORDER` | array deixa de ser First, Second |
| M-Q4 | usar a mesma política de field ausente em lista e one | `A-Q-FIELD-SPLIT` | um dos dois modos diverge |
| M-Q5 | filtrar field antes de validar cardinalidade one | `A-Q-ONE-FIRST` | erro deixa de reportar cardinalidade 0/2 |
| M-Q6 | perder label, expor `func: label` ou duplicar item | `F-L2-LABEL-SERIALIZE` | wrapper/elemento/cardinalidade diverge |
| M-Q7 | enviar `-` a `SystemWorld::new` como path | `A-Q-STDIN-EQUAL` | stdin falha ou difere do ficheiro |
| M-Q8 | construir stdin em modo code ou com identidade não resolvível | `A-Q-STDIN-IDENTITY` | query markup/diagnóstico deixa de ser resolvível |
| M-S1 | compilar selector literal como regex sem escape | `A-S-DOT` | literal `.` passa a casar como regex |
| M-S2 | parar depois do primeiro match | `A-S-ALL` | segunda ocorrência desaparece |
| M-S3 | transformar também quando não há match | `A-S-NONMATCH` | aparece metadata/transformação espúria |
| M-S4 | remover rejeição de vazio/match vazio | `A-S-EMPTY` | qualquer caso vazio deixa de falhar |
| M-S5 | permitir text/regex no caminho locatável de query | `A-S-NONLOCATABLE` | query deixa de falhar como não locatável |
| M-S6 | exigir igualdade do nó inteiro para literal | `A-S-PARTIAL` | `foo` deixa de casar dentro de `xfooY` |

## 5. Procedimento executável pós-freeze

1. Receber do coordenador: commit/estado da working tree, lista fechada de consumers
   candidatos, SHA-256 de cada consumer e hash do binário candidato.
2. Recalcular os hashes da secção 1 e exigir correspondência. Exigir novo receipt
   RED que pine o contrato `94bb0f…`; qualquer divergência é `BLOCKED_INPUT_DRIFT`.
3. Executar uma vez a candidata sem mutação sob todos os owners. Caso obrigatório
   não verde é `BASE_CANDIDATE_NOT_PRESERVED`; não se inicia score.
4. Para cada ID, criar diretório temporário dedicado, copiar somente o snapshot
   congelado, aplicar exatamente um patch e registrar hashes antes/depois.
5. Validar que o patch aplicou uma vez no owner correto, compila e produz a
   divergência semântica descrita. Patch que não aplica, não compila ou introduz
   segunda mutação é `INVALID_MUTANT`, fica fora do denominador e deve ser refeito;
   não permite fechar o gate.
6. Correr primeiro o teste owner e depois os controles focais do crate. Owner falha
   pela testemunha prevista = `KILLED`; owner passa = `SURVIVED`; infraestrutura,
   timeout, parser opaco ou testemunha ambígua = `UNKNOWN`.
7. Repetir cada mutante válido duas vezes, depois repetir em ordem permutada. Exigir
   mesmo verdict por ID. Capturar comando, exit, stdout e stderr separadamente.
8. Recalcular todas as entradas protegidas. Qualquer alteração fora do diretório
   efémero invalida a execução.

O executor pós-freeze deve produzir uma linha canónica por execução:

```text
mutant_id candidate_hash patch_hash owner_test command run_order exit verdict witness
```

## 6. Política explícita de Unknown e score

- `KILLED`: mutante válido, owner executado, falha causal com testemunha esperada.
- `SURVIVED`: mutante válido e owner continua verde; impede selo.
- `UNKNOWN`: execução não determinável por falha de harness, timeout, output
  truncado/decodificado de forma lossy, parser sem suporte, identidade ambígua ou
  budget encerrado. Nunca conta como sucesso e impede selo em qualquer caso
  obrigatório.
- `INVALID_MUTANT`: patch não representa isoladamente a mutação, não aplica ou não
  compila. Não entra no denominador, mas a mutação obrigatória continua pendente.
- `BLOCKED_INPUT_DRIFT`: hash protegido ou receipt causal divergiu; toda a execução
  posterior é inválida.

Somente depois de existirem 24 mutantes obrigatórios válidos, todos `KILLED` em duas
repetições e na ordem permutada, pode-se calcular
`mutation_score = KILLED / VALID = 24 / 24 = 1.0`. A contagem deriva diretamente
da tabela fechada do contrato v2 com hash `94bb0f…`, não de medição da working tree.
Este documento deliberadamente
não calcula score nem emite veredito.
