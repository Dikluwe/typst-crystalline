# P1285 — plano adversarial de mutações v4 pré-candidata v2 final

**Estado:** `FINAL_PRE_CANDIDATE_V2_READY_NO_EXECUTION`  
**Regime:** protocolo completo de materialização segregada.  
**Papel:** adversário/mutation tester, sem autoria da obrigação, candidata,
testes congelados, oráculos ou veredito.  
**Instante do plano:** `2026-08-30T15:37:17-03:00`.  
**HEAD declarado nas entradas:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.

Este documento substitui integralmente o plano v3. O contrato v4 invalidou o
contrato v3, as candidatas anteriores e a matriz adversarial derivada deles. Nenhum
resultado, score ou conclusão v3 transita para este plano.

## 1. Capacidades e cadeia congelada

Foram lidos integralmente o contrato v4, o receipt RED v4 e o nono L0
`compiler/eval/math.md`. Os oito owners anteriores já tinham sido lidos; o contrato v4
auditou o corpo normativo e separou pins sem selo de hashes integrais de proveniência.

Não foram lidos `materialization/`, `context/`, candidata v1/v2, diffs, baseline,
oráculos ou runner. Nenhum mutante foi criado ou executado. Nenhum ficheiro produtivo,
teste congelado, L0, contrato ou oráculo foi editado.

Entradas de protocolo:

| Entrada | SHA-256 |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| skill | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` |
| `papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| `artefatos-e-gates.md` | `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |
| contrato v4 | `8f6c94e6046574f39f8c913076c5b455914fbce3b6e74a5af05157e82b95f588` |
| receipt RED v4 | `761f2b4aad46b6fee6a431e5649afe80a7d905beba07acb1fca7f793ff1e7b32` |
| `compiler/eval/tests.md` test-only | `92ac9be1bcaeb65d0cda76603ba7260ffaccda2f6ff7049bf57bff24d6d83871` |
| `compiler/eval/tests.rs` no RED v4 | `e0e59d6e7f7af88f2b6a65752716207d043334969f165e77739cfeb478d40b15` |

Nove owners produtivos. O pin normativo exclui exatamente a linha
`Hash do Código:`; o full hash identifica os bytes observados:

| Owner L0 | Pin normativo sem selo | Full SHA-256 observado |
|---|---|---|
| `shell/cli.md` | `8fe484152e1371d7ef4a79659d6388bf5e63ff63ff84dc5698db7147a33543a7` | `fa2d3c4471822550994dfe2e4d23e503925efd9a067aac2e0054ad2c77ae004c` |
| `compiler/eval.md` | `7b2dc94f28457d84022f59a80b858c12428f671e67e3a5e90e6c0865d773e6c3` | `1fb66ec889304cd3a21accd84b009388cd1613c6fc613937c3af60015483cf56` |
| `compiler/eval/repr.md` | `3f1bcd114c5f880e10fedd3f5fbaee868d456b9c31b244244cb7a8a15d23860b` | `a2b35c8b3e622432b5a129a4cad0bb20ab94b450d3e793fd1441c5af6bc2775c` |
| `compiler/eval/selector_matching.md` | `557abd0a6fb52f8b198606e928ba23fd65234e3b75c2a14703b07a9975ba1b11` | `031ba52f7a6d5870479f0fe699a572950cce594db7688d59fc484175c10505a3` |
| `compiler/eval/rules.md` | `3655f2922835fd0bfcf0a1857a6d12d70ac9c30eae23f9f7b9eea1f69d9fc7c0` | `740919d6a874f136a6f176451977a51352266c08d347329f91207319a38018aa` |
| `compiler/stdlib/foundations/selector.md` | `9bea0284d242754ca1103baf45c53b7614817cfff1e5358fd708980f971f7c22` | `1a603e05d436e5f2359f77e3e71c0e233478ecf13209f71801fb45ae40214799` |
| `infra/query-helpers.md` | `352604527883503c72e349f9171cdb92cf730af3772cde28820275f11005713d` | `42debc22810cf3eebf16185d91543cedc36be4e179221f5b6d15b8f2ff6c6705` |
| `wiring.md` | `2142e48948482f1f86a8390b4dbd40cdc84f67f1783d5c9f3f370e9ed1401a44` | `f8db4e993fd1931d8f77b69a2ec416377f0700569e3942cc78e32daf945e0ec8` |
| `compiler/eval/math.md` | `c2ba2c5f028cd7424d15f92486e1d281a6efca1b74696f75249c6b9f1a018a9c` | `7cdf5a1c0f93d1f58cda7f93eaae09eb0cbdcead3ca78864919569755c3321b2` |

No freeze futuro, mudança apenas do full hash causada exclusivamente pela linha de
selo é permitida e registrada. Mudança em qualquer pin normativo é
`BLOCKED_INPUT_DRIFT` e reinicia a cadeia.

## 2. Owners dos testes

`F-*` é teste congelado descrito no receipt RED v4. `A-*` é caso black-box
obrigatório do runner selado, ligado pelo verificador sem revelar o runner ao
adversário. Cada mutante tem um owner primário; controles adicionais não transferem
ownership.

| Owner | Observável | Comando focal/adapter |
|---|---|---|
| `A-Y-HELP-EVAL` | formatos eval | runner selado: help eval |
| `A-Y-TREE` | YAML composto | runner selado: eval composto |
| `A-Y-PRETTY` | YAML neutro a pretty | runner selado: par YAML |
| `A-Y-QUERY-RAW` | raw em query rejeitado | runner selado: query raw |
| `F-L4-STDIN-YAML` | formato L4 + stdin markup | `cargo test -p typst-wiring --test cli p1285_query_stdin_transporta_yaml_ate_stdout -- --nocapture` |
| `A-J-VERSION-ACCEPT` | Version serializável | runner selado: `sys.version` |
| `A-J-VERSION-EXACT` | repr exata da Version | runner selado: `sys.version` |
| `A-J-STRUCTURAL` | array/dict/content estruturais | runner selado: composto/content |
| `A-J-NOMINAL-REPR` | fallback repr pública | runner selado: sentinelas nominais |
| `A-J-RAW-REJECT` | raw Version falha | runner selado: raw Version |
| `A-Q-NONHEADING` | matriz não-heading | runner selado: figure/equation/metadata/quote |
| `A-Q-SHAPE` | `func` + fields | runner selado: shapes figure/equation |
| `A-Q-ORDER` | ordem documental | runner selado: duas quotes |
| `A-Q-FIELD-SPLIT` | lista filtra; one erra | runner selado: field ausente |
| `A-Q-ONE-FIRST` | cardinalidade antes do field | runner selado: 0/2 + field |
| `F-L2-LABEL-SERIALIZE` | wrapper não vira `func: label` | `cargo test -p typst-shell p1285_ -- --nocapture` |
| `F-L3-LABEL` | transporte sem match extra, controle | `cargo test -p typst-infra p1285_query_elements_ -- --nocapture` |
| `A-Q-STDIN-EQUAL` | ficheiro/stdin equivalentes | runner selado: execução pareada |
| `A-Q-STDIN-IDENTITY` | markup/diagnóstico resolvível | runner selado: stdin contextual/erro |
| `A-S-DOT` | literal `.` vs regex `.` | runner selado: dupla discriminatória |
| `A-S-ALL` | todas as ocorrências | runner selado: matches repetidos |
| `A-S-NONMATCH` | não-match não transforma | runner selado: regex `z+` |
| `F-S-EMPTY-REJECT` | vazios falham | `cargo test -p typst-core p1285_native_selector_ -- --nocapture` |
| `A-S-NONLOCATABLE` | text/regex não locatável | runner selado: query selectors |
| `A-S-PARTIAL` | literal parcial | runner selado: `xfooY` |
| `F-S-SELECTOR-IDENTITY` | selector preservado | comando `p1285_native_selector_` |
| `F-S-REGEX-CONSTRUCT` | regex vira selector | comando `p1285_native_selector_` |
| `F-S-EMPTY-DISTINCT` | três diagnósticos distintos | comando `p1285_native_selector_` |
| `F-S-SHOW-OCCURRENCE` | recipe por ocorrência | `cargo test -p typst-core p1285_show_literal_e_regex_entregam_cada_ocorrencia_a_recipe -- --nocapture` |
| `F-S-SPLICE-REGEX` | splice puro, ordem/no-match/empty-only | `RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_splice_regex_ -- --nocapture` |
| `F-M-MORPHOLOGY` | Grapheme→MathIdent; Number→MathText | `cargo test -p typst-core p1285_math_grapheme_e_numero_preservam_variantes_distintas -- --nocapture` |

## 3. Matriz fechada de 30 mutações

Cada operador será materializado somente após o freeze, como patch mínimo
compilável numa cópia efémera. Falha acidental de build não substitui testemunha
semântica.

| ID | Operador semântico pós-freeze | Owner primário | Kill exigido |
|---|---|---|---|
| M-Y1 | remover YAML de eval/help | `A-Y-HELP-EVAL` | inventário diverge |
| M-Y2 | despachar YAML pelo serializer JSON | `A-Y-TREE` | formato/árvore diverge |
| M-Y3 | fazer pretty mudar YAML | `A-Y-PRETTY` | par diverge |
| M-Y4 | aceitar raw em query | `A-Y-QUERY-RAW` | exit deixa de ser 2 |
| M-Y5 | L4 forçar `QueryFormat::Json` | `F-L4-STDIN-YAML` | fluxo deixa de produzir YAML |
| M-J1 | rejeitar Version | `A-J-VERSION-ACCEPT` | comando falha/não devolve string |
| M-J2 | usar Display/Debug `0.15.1` | `A-J-VERSION-EXACT` | repr exata diverge |
| M-J3 | array/dict/content por repr | `A-J-STRUCTURAL` | tipo deixa de ser árvore |
| M-J4 | fallback nominal por Debug/erro | `A-J-NOMINAL-REPR` | sentinela diverge/falha |
| M-J5 | permitir Version em raw | `A-J-RAW-REJECT` | raw deixa de falhar |
| M-Q1 | restaurar serializer heading-only | `A-Q-NONHEADING` | testemunha não-heading falha |
| M-Q2 | omitir `func` ou field obrigatório | `A-Q-SHAPE` | shape perde chave |
| M-Q3 | ordenar/inverter/deduplicar resultados | `A-Q-ORDER` | ordem First, Second diverge |
| M-Q4 | unificar field ausente em lista/one | `A-Q-FIELD-SPLIT` | um modo diverge |
| M-Q5 | filtrar field antes de validar one | `A-Q-ONE-FIRST` | erro 0/2 deixa de prevalecer |
| M-Q6 | perder label, expor `func: label` ou duplicar | `F-L2-LABEL-SERIALIZE` | wrapper/cardinalidade diverge |
| M-Q7 | tratar `-` como path físico | `A-Q-STDIN-EQUAL` | stdin falha/difere |
| M-Q8 | criar stdin code/identidade irresolvível | `A-Q-STDIN-IDENTITY` | markup/diagnóstico diverge |
| M-S1 | literal como regex sem escape | `A-S-DOT` | literal `.` casa como regex |
| M-S2 | parar no primeiro match | `A-S-ALL` | segunda ocorrência some |
| M-S3 | transformar no-match | `A-S-NONMATCH` | transformação espúria |
| M-S4 | aceitar vazio/match vazio | `F-S-EMPTY-REJECT` | qualquer vazio deixa de falhar |
| M-S5 | tornar text/regex locatável | `A-S-NONLOCATABLE` | query deixa de falhar |
| M-S6 | exigir nó inteiro no literal | `A-S-PARTIAL` | `foo` não casa em `xfooY` |
| M-S7 | rejeitar/reconstruir `Value::Selector` | `F-S-SELECTOR-IDENTITY` | identidade/repr perde-se |
| M-S8 | rejeitar regex válida/devolver Regex cru | `F-S-REGEX-CONSTRUCT` | resultado não é Selector(Regex) |
| M-S9 | conflar diagnósticos vazios | `F-S-EMPTY-DISTINCT` | mensagem de uma classe diverge |
| M-S10 | recipe regex uma vez com nó inteiro | `F-S-SHOW-OCCURRENCE` | duas ocorrências viram uma |
| M-M1 | mapear Grapheme e Number para `MathText` | `F-M-MORPHOLOGY` | `$x$`/base de `$x^2$` deixa de ser ident/symbol |
| M-M2 | mapear Number para `MathIdent` | `F-M-MORPHOLOGY` | `$2$`/expoente de `$x^2$` deixa de ser text |

`F-S-SPLICE-REGEX` é controle adicional obrigatório de M-S10. Para M-M1/M-M2,
o teste owner possui controles complementares no mesmo corpo; no gate de mutações
cada mutante deve falhar na asserção correspondente, não por efeito colateral.

## 4. Procedimento executável após freeze da candidata v2 final

1. Receber identificador da candidata, estado exato, lista dos nove consumers e
   SHA-256 de consumers, testes e binário.
2. Recalcular contrato/RED e os nove pins normativos com
   `sed '/^Hash do Código:/d' "$f" | sha256sum`. Pin divergente é
   `BLOCKED_INPUT_DRIFT`. Full hash diferente é aceito apenas se diff semântico zero
   e a única linha alterada for o selo; registrar o novo full hash.
3. Verificar integridade dos 17 testes congelados. Executar a candidata sem mutação
   sob todos os owners. Caso obrigatório não verde é
   `BASE_CANDIDATE_NOT_PRESERVED`; score não inicia.
4. Para cada ID, criar diretório temporário isolado, copiar o snapshot, aplicar um
   único patch no consumer owner e registrar hashes antes/depois.
5. Exigir patch compilável, isolado e semanticamente equivalente à mutação da
   tabela. Patch inaplicável, composto ou não compilável é `INVALID_MUTANT`, fora do
   denominador e obrigatoriamente refeito.
6. Correr owner primário, controles focais do crate e runner selado aplicável,
   capturando comando, exit, stdout e stderr separadamente.
7. Repetir cada mutante válido duas vezes e repetir a matriz em ordem permutada;
   exigir verdict estável por ID.
8. Recalcular todas as entradas. Write fora da cópia efémera invalida o gate.

Receipt futuro, uma linha por execução:

```text
mutant_id candidate_hash patch_hash productive_owner test_owner command run_order exit verdict witness
```

## 5. Unknown, validade e score

- `KILLED`: mutante válido, owner executado e falha causal prevista.
- `SURVIVED`: mutante válido e owner permanece verde; impede selo.
- `UNKNOWN`: timeout, falha de harness/infra, output truncado/lossy, parser sem
  suporte, identidade ambígua ou budget encerrado. Nunca é sucesso e impede selo em
  caso obrigatório.
- `INVALID_MUTANT`: patch não isolado, inaplicável ou não compilável. Fora do
  denominador, mas a obrigação continua pendente.
- `BLOCKED_INPUT_DRIFT`: pin normativo/contrato/RED/teste protegido mudou.

Somente com 30 mutantes obrigatórios válidos, todos `KILLED` nas repetições e ordem
permutada, pode-se calcular `mutation_score = 30 / 30 = 1.0`. A contagem deriva da
tabela fechada do contrato v4 `8f6c94e6…`, não da candidata. Este plano não executa
mutantes, calcula score ou emite veredito.
