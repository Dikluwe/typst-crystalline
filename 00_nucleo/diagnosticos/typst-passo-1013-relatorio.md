# Passo 1013 — Relatório: `eval::bindings` fatiado em hub + 5 nós

**Resultado**: fatiado. 5 nós, código preservado bit-a-bit, zero regressão.
**Proveniência**: medições feitas a partir de `HEAD = b72d2e8ae` (Passo 1012), em
2026-08-12 ~18:10 −03. Antes de começar, `git status` mostrava apenas os dois ficheiros
de passo untracked (`typst-passo-1013.md`, `typst-passo-1014.md`) — árvore limpa.

---

## Fase A — Inventário: a lista do passo estava a 52% do real

O passo trazia 23 funções, herdadas do Passo 1000. O ficheiro tem **44 itens** (43
funções + `enum ContentField`).

**Causa, e é a mesma lacuna do P1000 numa forma nova**: o grep do passo era
`'^pub fn \|^pub(crate) fn \|^fn '`. O P1000 já tinha sido corrigido para incluir
`pub(crate)`, mas `bindings.rs` usa maioritariamente **`pub(super) fn`** — 18 das 43.
O padrão certo cobre qualquer visibilidade restrita:

```
grep -nE '^(pub(\([a-z:) ]+\))? )?fn ' <ficheiro>
```

| Visibilidade | Contagem |
|---|---:|
| `fn` (privada) | 24 |
| `pub(super) fn` | 18 |
| `pub(crate) fn` | 1 |
| **total** | **43** (+1 `enum`) |

Duas funções privadas também faltavam à lista do passo: `is_accessor_method` (:433) e
`missing_key` (:439).

---

## Fase B — Os 4 critérios

### Critério 2 — pureza vs estado, medido por `file:line` (lição do P1012)

Não basta ver `Engine`/`EvalContext` na assinatura. Medindo o **uso depois** de
`eval_args`, aparecem três classes distintas:

| Classe | Funções | Evidência |
|---|---|---|
| **Sem contexto nenhum** | `call_method_access`, `call_method_mut`, `eval_value_field_access`, `eval_content_method`, `field_callee_error`, `content_*`, as 4 tabelas de classificação, `long_type_name`, `wrong_number_of_elements`, `expect_positional`, `finish_args`, `value_to_query_selector`, `element_or_type_with_name` | assinatura sem `ctx`/`engine` |
| **Pass-through** — contexto só para avaliar argumentos | `eval_version_method_value`, `eval_element_where`, `eval_selector_or_and`, `eval_selector_within` | **0** usos de `ctx`/`engine` depois da linha do `eval_args` |
| **Contexto real** | `eval_color_method` (14 usos; lê `engine.world` e `engine.current_file`, :1121-1122), `eval_state_method` (4), `try_eval_mutating_method` (1, via `access`), `eval_counter_method_value`, `state_at_dispatch`, `render_counter_at_label` (lê `ctx.introspector`) | contagem de usos pós-`eval_args` |

A classe do meio é o achado: **quatro funções pareciam stateful e não são**. Sem esta
medição, `eval_element_where` e os combinadores de selector teriam sido classificados
como "tocam contexto" e agrupados com counter/state pelo motivo errado. (Acabaram no
mesmo nó, mas por co-mudança — critério 3 — não por estado.)

### Critério 3 — co-mudança histórica

`python3 tools/analysis/cochange_metrics.py 01_core/src/compiler/eval/bindings.rs '(rules|engine|compiler)/eval/bindings\.rs$'`

- 60 commits tocaram o ficheiro; **20 só linhagem** (`@prompt-hash`/`@updated`), 40 com
  mudança de corpo. O desconto de ruído de resselo — regra nova proposta no P1006 — foi
  aplicado desde o início e está agora dentro da ferramenta.
- `eval_field_access` é a função de maior churn: **20 commits**, contra 8 do segundo
  (`eval_let`, `eval_counter_method`).

Agrupamentos, com o commit que os isola:

| Cluster | Commit isolador | Funções que se movem juntas |
|---|---|---|
| binding | `7a901edd4` (P715) | `destructure_array`, `destructure_dict`, `destructure_let`, `destructure_pattern`, `eval_assign`, `eval_destruct_assignment`, `eval_let`, `wrong_number_of_elements` — **os 8, e só os 8** |
| method_dispatch | `c570d53e5` (P717) | `call_method_access`, `call_method_mut`, `is_dict_mutating_method`, `is_mutating_method`, `try_eval_mutating_method` |
| access | `2b981e7e0` (P772q+r) | `access`, `missing_key`, `unknown_variable` |
| value_methods | `969087ecf` (P506), `8fc61d456` (P640), `6b321acc1` (P742), `23ff5b5c3` (P796), `c7bd13ce7` (P423) | state+counter; counter.display+render+parse; color+state; version; selector |
| field_access | (sem commit isolador; `eval_field_access` cruza com selector e counter em P417/P493/P504) | — |

`b806f562d` (P716, "Access genérico") toca 16 funções de três clusters — é o commit
fundador do acesso genérico, ruído estrutural, não sinal de fronteira.

### Critério 4 — vanilla

O candidato do passo (`typst_eval::{access, methods, binding}`) **confirmou-se, e
função a função** para três dos cinco nós:

| Nó cristalino | Vanilla | Correspondência |
|---|---|---|
| `binding.rs` | `typst-eval/src/binding.rs` (209 l.) | `destructure`↔`destructure_let`, `destructure_impl`↔`destructure_pattern`, `destructure_array`, `destructure_dict`, `wrong_number_of_elements` — mesmos nomes |
| `access.rs` | `typst-eval/src/access.rs` (107 l.) | `access_dict` mesmo nome; o vanilla usa `trait Access` com impl por variante, nós usamos free function com `match` — divergência **mecânica** (ADR-0107) |
| `method_dispatch.rs` | `typst-eval/src/methods.rs` (104 l.) | `is_mutating_method`, `is_dict_mutating_method`, `is_accessor_method`, `call_method_mut`, `call_method_access` — **cinco nomes idênticos** |

Os outros dois nós **não têm correspondente em `typst-eval`**: `value_methods` são
nativas declaradas junto do tipo em `typst-library::{introspection, visualize,
foundations}`; `field_access` está em `foundations::value::field()` /`Content::field()`.
Os 420 lines dos três ficheiros vanilla cobrem ~940 das nossas 2192 linhas de corpo; o
resto é agregação do cristalino sem espelho directo.

### Critério 1 — isolamento de teste

Vácuo pela terceira vez consecutiva (P1002, P1006, P1013): não há testes locais em
`bindings.rs`; a cobertura vive em `eval/tests.rs`, end-to-end por código Typst. Nenhuma
partição de testes sugere fronteira.

---

## Fase C — Materialização

`bindings.rs` (2235 linhas) → directório `bindings/` com hub + 5 nós:

| Ficheiro | Linhas | Corpo movido |
|---|---:|---:|
| `bindings/mod.rs` (hub) | 28 | 0 |
| `bindings/binding.rs` | 403 | 373 |
| `bindings/access.rs` | 209 | 181 |
| `bindings/method_dispatch.rs` | 384 | 359 |
| `bindings/value_methods.rs` | 742 | 700 |
| `bindings/field_access.rs` | 609 | 579 |
| **total** | **2375** | **2192** = 2235 − 43 de cabeçalho |

### O hub não tem tabela de despacho — e isso está certo

Diferença face a `operators/mod.rs` (P1002): `operators` tem entrada única
(`eval_binary_op`) e o hub é a jump table. `bindings.rs` era um **agregado plano** — 43
funções chamadas directamente por `eval/mod.rs`, sem dispatcher. O hub é a fronteira de
re-exportação que mantém `bindings::<fn>` válido nos consumidores, sem tocar num único
call site.

Registado no L0 do hub para não ser lido mais tarde como "hub por preencher": quando a
unidade fatiada não tinha dispatcher, um hub sem lógica é o resultado correcto.

### Visibilidades

- Funções que eram `pub(super)` (visíveis em `eval`) → `pub(in crate::compiler::eval)`
  no nó, re-exportadas por `pub(super) use` no hub. Necessário: `pub(super)` dentro do
  nó significaria "visível em `bindings`", e o hub não pode re-exportar mais largo do que
  o item (E0364).
- Sete símbolos privados são chamados de outro nó — passaram a `pub(super)` (visíveis
  dentro de `bindings`, não fora): `access`, `access_dict`, `missing_key`,
  `call_method_access`, `is_accessor_method`, `expect_positional`, `finish_args`. As
  arestas entre nós são 6, todas medidas antes do corte.
- `long_type_name` continua `pub(crate)` (usado por `eval/mod.rs`, `call_dispatch.rs`,
  `stdlib/*`).

### Prova de preservação

Comparação item a item entre `HEAD:bindings.rs` e a concatenação dos 5 nós, normalizando
apenas as três reescritas conhecidas (`super::call_dispatch::`/`super::operators::` →
caminho absoluto, e as mudanças de visibilidade acima):

```
original: 44 itens | nós: 44
em falta: nenhuma  | a mais: nenhuma
corpos diferentes: NENHUM — corte e cola exacto, doc-comments incluídos
```

Os 89 `use` removidos são imports que ficaram por usar em cada nó; removidos com as
sugestões do próprio rustc (`--message-format=json`, `unused_imports`), aplicadas só a
ficheiros sob `eval/bindings/`, iteradas até estabilizar. Zero warnings novos.

---

## Fase D — Validação

| Verificação | Resultado |
|---|---|
| `cargo build --workspace` | ✅ ok, zero warnings nos ficheiros novos |
| `crystalline-lint --fix-hashes .` | `0 drift warnings remaining` |
| `crystalline-lint .` | **2 warnings**, ambos V7 órfãos pré-existentes (`auditar-spec.md`, `infra/package_version_resolution.md`) — confirmados em `HEAD` no Passo 1005 |
| `cargo test --workspace` | **5832 passed, 0 failed, 3 ignored** |
| `#[test]` em `HEAD` vs working tree | 5832 = 5832 |

---

## Fase E — Avaliação do método

**A fronteira vanilla bateu com o critério 3** — pela primeira vez desde que o método
existe. P715 isola exactamente os 8 símbolos de `binding.rs`; P717 isola exactamente os
5 de `methods.rs`. Não houve correcção a fazer, ao contrário de P1002 (`coercion.md`
descartado) e P1006 (`metrics` recusado por inteiro).

Mas o vanilla **só cobre 3 dos 5 nós**. Os outros dois saíram de co-mudança pura
(`value_methods`) e de churn concentrado (`field_access`). Se o critério 4 tivesse sido
usado sozinho, teriam ficado juntos num resto amorfo de 1250 linhas.

### Lacunas do método

1. **A lacuna de visibilidade do P1000 não estava fechada — só deslocada.** Foi corrigida
   para `pub(crate)` no P1002 e voltou como `pub(super)` aqui, escondendo 18 de 43
   funções. O padrão tem de ser genérico sobre qualquer `pub(<restrição>)`, e é isso que
   fica registado para os passos seguintes.
2. **O critério 1 (isolamento de teste) é vácuo em todo o `compiler/`** — três de três
   aplicações. Os testes deste repositório são E2E por código Typst, num ficheiro de
   testes por módulo; nunca vão particionar por função. Proposta: retirá-lo do método ou
   substituí-lo por "quem chama" (fan-in por símbolo dentro do módulo).
3. **O critério-zero proposto no P1006 ("agregado ou interface?") funcionou.**
   `bindings.rs` é agregado (sem `trait`, 43 free functions) → o método aplicou-se sem
   fricção. Confirma que o teste deve vir antes dos outros quatro.

### Achado lateral (não corrigido — fora de âmbito)

Existem **três** cópias independentes de `long_type_name` no núcleo:
`eval/bindings/access.rs:34` (`pub(crate)`), `eval/operators/join.rs:86` (privada),
`stdlib/foundations.rs:1692` (privada). Duplicação anterior a este passo — o L0
`operators/join.md` chega a documentar a sua cópia como se fosse a única. É dedup, não
atomização; assinalado para decisão do dono, não tocado aqui.

---

## Resultado

Com este passo a família `eval::*` da leva do Passo 1008 fica completa: `rules`
(P1009/P1011), `closures` (P1012), `bindings` (P1013).
