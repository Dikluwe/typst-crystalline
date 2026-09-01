# P1285 — plano adversarial de mutações v3 pré-candidata v2

**Estado:** `INVALIDATED_BY_P1285_CONTRACT_V4`  
**Regime:** protocolo completo de materialização segregada.  
**Papel:** adversário/mutation tester, sem autoria da obrigação, candidata,
testes congelados, oráculos ou veredito.  
**HEAD hospedeiro declarado pelas entradas:**
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.

> Invalidado pelo contrato v4
> `8f6c94e6046574f39f8c913076c5b455914fbce3b6e74a5af05157e82b95f588`, que
> acrescentou o owner `compiler/eval/math.md`, C-MATH e M-M1/M-M2. Sucessor:
> `00_nucleo/diagnosticos/p1285-adversarial-mutation-plan-v4.md`.

Este documento substitui integralmente o plano v2. O contrato v3 invalidou o
contrato v2, a candidata v1 e a matriz adversarial derivada deles. Nenhum resultado,
score ou conclusão do plano v2 transita para este plano.

## 1. Capacidades e entradas congeladas

Foram lidos integralmente o contrato v3, o receipt RED v3, os L0 novos/repinados
`compiler/eval/selector_matching.md`, `compiler/eval/rules.md` e
`compiler/stdlib/foundations/selector.md`. Os outros cinco L0 permanecem com os bytes
já lidos no preflight anterior e foram repinados pelo contrato v3.

Não foram lidos `materialization/`, `context/`, candidata v1/v2, diffs, baseline,
oráculos ou runner. Nenhum mutante foi materializado ou executado. Nenhum ficheiro
produtivo, teste congelado, L0, contrato ou oráculo foi editado.

| Entrada protegida | SHA-256 |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| skill | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` |
| `papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| `artefatos-e-gates.md` | `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |
| `shell/cli.md` | `fa2d3c4471822550994dfe2e4d23e503925efd9a067aac2e0054ad2c77ae004c` |
| `compiler/eval.md` | `1fb66ec889304cd3a21accd84b009388cd1613c6fc613937c3af60015483cf56` |
| `compiler/eval/repr.md` | `a2b35c8b3e622432b5a129a4cad0bb20ab94b450d3e793fd1441c5af6bc2775c` |
| `compiler/eval/selector_matching.md` | `fd7a0b50308e119932d9fee90f9aa972d4a6ebab55ad31b15aa9decead59cae6` |
| `compiler/eval/rules.md` | `c97c2f6fc0275e37667b1d183ada5437e523ab3248b2549536da7b038bba875f` |
| `compiler/stdlib/foundations/selector.md` | `6849e281548fd239321dab6fdf31a9a6473ac8bd7a9bc6d7fd8a78c4be63d7d8` |
| `infra/query-helpers.md` | `42debc22810cf3eebf16185d91543cedc36be4e179221f5b6d15b8f2ff6c6705` |
| `wiring.md` | `f8db4e993fd1931d8f77b69a2ec416377f0700569e3942cc78e32daf945e0ec8` |
| `compiler/eval/tests.md` test-only | `92ac9be1bcaeb65d0cda76603ba7260ffaccda2f6ff7049bf57bff24d6d83871` |
| contrato v3 | `1ea58e14a01f6cb55fe38695133f4c279164ca2943134d2dcf5cab81c3df681e` |
| receipt RED v3 | `34fcd0ba964b5e5c475a22a0ce67a7f87aef6514376615c12e1c4a112c0f0666` |

O receipt RED v3 pina corretamente o contrato v3 e os oito owners produtivos. A
inconsistência causal identificada no v2 está resolvida. O receipt declara 16 testes:
os 10 históricos mais 6 derivados no segundo restart.

## 2. Owners dos testes

`F-*` identifica teste congelado descrito no receipt RED v3. `A-*` identifica caso
black-box obrigatório do runner selado; o verificador faz o adapter sem revelar o
runner ao adversário. Cada mutante tem exatamente um owner primário; controles podem
falhar adicionalmente sem mudar o owner.

| Owner | Observável | Comando focal/adapter |
|---|---|---|
| `A-Y-HELP-EVAL` | inventário `json,yaml,raw` | runner selado: help eval |
| `A-Y-TREE` | YAML composto equivale à árvore JSON | runner selado: eval composto |
| `A-Y-PRETTY` | YAML igual com/sem pretty | runner selado: par YAML |
| `A-Y-QUERY-RAW` | raw em query, exit 2 | runner selado: query raw |
| `F-L4-STDIN-YAML` | formato L4 + source stdin markup | `cargo test -p typst-wiring --test cli p1285_query_stdin_transporta_yaml_ate_stdout -- --nocapture` |
| `A-J-VERSION-ACCEPT` | Version serializável | runner selado: `sys.version` |
| `A-J-VERSION-EXACT` | repr exata da Version | runner selado: `sys.version` |
| `A-J-STRUCTURAL` | array/dict/content como árvore | runner selado: composto/content |
| `A-J-NOMINAL-REPR` | fallback público, sem Debug | runner selado: sentinelas nominais |
| `A-J-RAW-REJECT` | raw Version falha | runner selado: raw Version |
| `A-Q-NONHEADING` | figure/equation/metadata/quote | runner selado: matriz não-heading |
| `A-Q-SHAPE` | `func` + fields públicos | runner selado: shapes figure/equation |
| `A-Q-ORDER` | quotes First, Second | runner selado: duas quotes |
| `A-Q-FIELD-SPLIT` | lista filtra; one erra | runner selado: field ausente |
| `A-Q-ONE-FIRST` | cardinalidade antes do field | runner selado: 0/2 + field |
| `F-L2-LABEL-SERIALIZE` | wrapper não vira `func: label` | `cargo test -p typst-shell p1285_ -- --nocapture` |
| `F-L3-LABEL` | transporte de label sem match extra | `cargo test -p typst-infra p1285_query_elements_ -- --nocapture` |
| `A-Q-STDIN-EQUAL` | ficheiro e stdin com mesma árvore | runner selado: execução pareada |
| `A-Q-STDIN-IDENTITY` | markup contextual/diagnóstico resolvível | runner selado: stdin contextual/erro |
| `A-S-DOT` | literal `.` vs regex `.` | runner selado: dupla discriminatória |
| `A-S-ALL` | todas as ocorrências/fatias | runner selado: matches repetidos |
| `A-S-NONMATCH` | não-match não transforma | runner selado: regex `z+` |
| `F-S-EMPTY-REJECT` | vazio/match vazio falha | `cargo test -p typst-core p1285_native_selector_ -- --nocapture` |
| `A-S-NONLOCATABLE` | query text/regex falha | runner selado: não locatáveis |
| `A-S-PARTIAL` | literal parcial `xfooY` | runner selado: show parcial |
| `F-S-SELECTOR-IDENTITY` | `Value::Selector` preservado | `cargo test -p typst-core p1285_native_selector_ -- --nocapture` |
| `F-S-REGEX-CONSTRUCT` | regex válida vira selector | mesmo comando `p1285_native_selector_` |
| `F-S-EMPTY-DISTINCT` | três diagnósticos distintos | mesmo comando `p1285_native_selector_` |
| `F-S-SHOW-OCCURRENCE` | recipe por ocorrência | `cargo test -p typst-core p1285_show_literal_e_regex_entregam_cada_ocorrencia_a_recipe -- --nocapture` |
| `F-S-SPLICE-REGEX` | splice puro, ordem/no-match/empty-only | `RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_splice_regex_ -- --nocapture` |

Controles focais de regressão, selecionados por owner do mutante:

```sh
cargo test -p typst-shell p1285_ -- --nocapture
cargo test -p typst-core p1285_selector_ -- --nocapture
cargo test -p typst-core p1285_native_selector_ -- --nocapture
RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_splice_regex_ -- --nocapture
cargo test -p typst-core p1285_show_literal_e_regex_entregam_cada_ocorrencia_a_recipe -- --nocapture
cargo test -p typst-infra p1285_query_elements_ -- --nocapture
```

## 3. Matriz fechada de 28 mutações

Cada operador será convertido, somente após o freeze, em patch mínimo compilável
numa cópia efémera do snapshot. Falha acidental de build não substitui testemunha
observável.

| ID | Operador semântico pós-freeze | Owner primário | Kill exigido |
|---|---|---|---|
| M-Y1 | remover YAML de eval/help | `A-Y-HELP-EVAL` | inventário diverge |
| M-Y2 | despachar YAML pelo serializer JSON | `A-Y-TREE` | formato/árvore YAML diverge |
| M-Y3 | fazer pretty mudar YAML | `A-Y-PRETTY` | par diverge |
| M-Y4 | aceitar raw em query | `A-Y-QUERY-RAW` | exit deixa de ser 2 |
| M-Y5 | L4 forçar `QueryFormat::Json` | `F-L4-STDIN-YAML` | fluxo completo deixa de produzir YAML |
| M-J1 | rejeitar Version no estruturador | `A-J-VERSION-ACCEPT` | comando falha/não devolve string |
| M-J2 | usar Display/Debug `0.15.1` | `A-J-VERSION-EXACT` | repr difere de `version(0, 15, 1)` |
| M-J3 | serializar array/dict/content por repr | `A-J-STRUCTURAL` | tipo deixa de ser árvore |
| M-J4 | fallback nominal por Debug/erro | `A-J-NOMINAL-REPR` | sentinela diverge/falha |
| M-J5 | permitir Version em raw | `A-J-RAW-REJECT` | raw deixa de falhar com exit 1 |
| M-Q1 | restaurar serializer heading-only | `A-Q-NONHEADING` | testemunha não-heading falha |
| M-Q2 | omitir `func` ou field obrigatório | `A-Q-SHAPE` | shape perde chave |
| M-Q3 | ordenar/inverter/deduplicar resultados | `A-Q-ORDER` | ordem First, Second diverge |
| M-Q4 | unificar política de field ausente | `A-Q-FIELD-SPLIT` | lista ou one diverge |
| M-Q5 | filtrar field antes da cardinalidade one | `A-Q-ONE-FIRST` | erro 0/2 deixa de prevalecer |
| M-Q6 | perder label, expor `func: label` ou duplicar | `F-L2-LABEL-SERIALIZE` | wrapper/elemento/cardinalidade diverge |
| M-Q7 | tratar `-` como path físico | `A-Q-STDIN-EQUAL` | stdin falha/difere do ficheiro |
| M-Q8 | criar stdin code/identidade não resolvível | `A-Q-STDIN-IDENTITY` | markup/diagnóstico diverge |
| M-S1 | compilar literal como regex sem escape | `A-S-DOT` | literal `.` casa como regex |
| M-S2 | parar depois do primeiro match | `A-S-ALL` | segunda ocorrência desaparece |
| M-S3 | transformar no-match | `A-S-NONMATCH` | transformação espúria aparece |
| M-S4 | aceitar vazio ou match vazio | `F-S-EMPTY-REJECT` | qualquer vazio deixa de falhar |
| M-S5 | tornar text/regex locatável | `A-S-NONLOCATABLE` | query deixa de falhar |
| M-S6 | exigir igualdade do nó inteiro no literal | `A-S-PARTIAL` | `foo` deixa de casar em `xfooY` |
| M-S7 | rejeitar/reconstruir `Value::Selector` | `F-S-SELECTOR-IDENTITY` | selector composto perde identidade/repr |
| M-S8 | rejeitar regex válida ou devolver Regex cru | `F-S-REGEX-CONSTRUCT` | resultado deixa de ser Selector(Regex) |
| M-S9 | conflar diagnósticos dos três vazios | `F-S-EMPTY-DISTINCT` | qualquer mensagem deixa de ser a exata da classe |
| M-S10 | chamar recipe regex uma vez com o nó inteiro | `F-S-SHOW-OCCURRENCE` | `foo fxo` deixa de produzir duas chamadas |

Para M-S10, `F-S-SPLICE-REGEX` é controle estrutural obrigatório adicional: impede
que o mutante sobreviva escondido por um splice que já perdeu ordem, fatias ou a
política no-match/empty-only.

## 4. Execução autorizada somente após freeze

1. Receber do coordenador o identificador da candidata v2 congelada, commit/estado da
   working tree, lista fechada dos oito consumers e SHA-256 de cada consumer, testes e
   binário.
2. Recalcular contrato, receipt RED, oito L0 e testes protegidos. Divergência é
   `BLOCKED_INPUT_DRIFT`; nenhuma mutação começa.
3. Executar a candidata sem mutação sob todos os owners. Caso obrigatório não verde
   é `BASE_CANDIDATE_NOT_PRESERVED`; não se calcula score.
4. Para cada ID, criar diretório temporário isolado, copiar o snapshot congelado,
   aplicar exatamente um patch e registrar hashes antes/depois.
5. Exigir patch no owner produtivo correto, compilação e apenas a divergência
   semântica descrita. Patch inaplicável, não compilável ou composto é
   `INVALID_MUTANT`, fica fora do denominador e deve ser refeito.
6. Correr o owner primário, controles focais do crate e runner selado aplicável.
   Capturar comando, exit, stdout e stderr separadamente.
7. Repetir cada mutante válido duas vezes e repetir a matriz em ordem permutada.
   Verdict por ID deve ser estável.
8. Recalcular entradas protegidas; write fora da cópia efémera invalida o gate.

Linha canónica do receipt futuro:

```text
mutant_id candidate_hash patch_hash productive_owner test_owner command run_order exit verdict witness
```

## 5. Unknown, validade e score

- `KILLED`: mutante válido, owner executado e falha causal com testemunha prevista.
- `SURVIVED`: mutante válido e owner permanece verde; impede selo.
- `UNKNOWN`: timeout, falha de harness/infra, output truncado ou lossy, parser sem
  suporte, identidade ambígua ou budget encerrado. Nunca conta como sucesso e impede
  selo em caso obrigatório.
- `INVALID_MUTANT`: patch não representa isoladamente a mutação, não aplica ou não
  compila. Não entra no denominador, mas deixa a obrigação pendente.
- `BLOCKED_INPUT_DRIFT`: qualquer entrada protegida mudou; execução posterior nula.

Somente com 28 mutantes obrigatórios válidos, todos `KILLED` nas repetições e na
ordem permutada, pode-se calcular `mutation_score = 28 / 28 = 1.0`. A contagem deriva
da tabela fechada do contrato v3 com hash `1ea58e14…`, não de inspeção da candidata.
Este plano não executa mutantes, calcula score ou emite veredito.
