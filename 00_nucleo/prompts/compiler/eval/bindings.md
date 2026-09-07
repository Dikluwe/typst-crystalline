# Prompt L0 — `compiler/eval/bindings` — hub de bindings, acesso e métodos
Hash do Código: 01d35778

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/bindings/mod.rs`
**Prompt pai**: `00_nucleo/prompts/compiler/eval.md` (dono de `compiler/eval/mod.rs`)
**ADRs relevantes**: ADR-0107 (paridade língua), ADR-0108 (medir antes de decidir), ADR-0109 (atomização), ADR-0044 (`Engine<'_>`)

---

## Contexto

`bindings` reúne a parte do eval que trata **nomes e lugares**: ligar nomes a
valores (`#let`), resolver o lugar mutável por trás de uma expressão de acesso,
despachar métodos sobre valores, e resolver acesso a campo.

Foi extraído de `eval.rs` no Passo 96.1 (ADR-0037) e fatiado em hub + 5 nós
conforme ADR-0109.

**Este hub não tem tabela de despacho.** É a diferença face a
`compiler/eval/operators.md`: `operators` tem um ponto de entrada único
(`eval_binary_op`) e o hub é a jump table; `bindings` era um **agregado plano**
— 43 funções sem entrada única, chamadas directamente por `eval/mod.rs`. O hub
é, portanto, a fronteira de re-exportação que mantém `bindings::<fn>` válido nos
consumidores. Registar isto é deliberado: um hub sem lógica é o resultado
correcto quando a unidade fatiada não tinha dispatcher, e não deve ser
confundido com um hub por preencher.

## Restrições Estruturais

- L1 puro: zero I/O, zero estado global. `Engine<'_>` e `EvalContext` são
  parâmetros, nunca estado do módulo.
- O hub **não contém lógica** — só `mod` e re-exportações.
- Nenhum nó importa `eval/mod.rs` (sem import reverso). O acesso a `eval_expr`
  e `EvalContext` é por caminho absoluto `crate::compiler::eval::…`.

## Instrução

### Nós

| Nó | Domínio | Vanilla correspondente |
|---|---|---|
| `bindings/binding.md` | `#let`, desestruturação, atribuição | `typst-eval/src/binding.rs` |
| `bindings/access.md` | lugares mutáveis, erros de nome/chave | `typst-eval/src/access.rs` |
| `bindings/method_dispatch.md` | métodos mutantes e de acesso sobre `Value` | `typst-eval/src/methods.rs` |
| `bindings/value_methods.md` | métodos de instância com args em AST (state, counter, color, version, selector) | disperso em `typst-library` |
| `bindings/field_access.md` | `a.b` sobre valores e sobre `Content` | `foundations::value::field` / `Content::field` |

### Superfície re-exportada

```rust
pub(super) use binding::{eval_let, destructure_let, eval_destruct_assignment, eval_assign};
pub(super) use access::unknown_variable;
pub(super) use method_dispatch::{is_mutating_method, try_eval_mutating_method};
pub(super) use value_methods::{eval_state_method, eval_color_method,
    eval_counter_method_value, eval_version_method_value, eval_element_where,
    eval_selector_or_and, eval_selector_within};
pub(super) use field_access::{eval_field_access, eval_value_field_access,
    eval_content_method, field_callee_error};
pub(crate) use access::long_type_name;
```

Fronteiras internas entre nós (visíveis dentro de `bindings`, não fora):
`access::{access, access_dict, missing_key}` e
`method_dispatch::{call_method_access, is_accessor_method, expect_positional, finish_args}`.

## Critérios de Verificação

```
Dado o hub sem lógica
Quando se compila `eval/mod.rs`
Então todas as chamadas `bindings::<fn>` continuam a resolver sem alteração

Dado o conjunto dos 5 nós
Quando se soma o corpo de cada um
Então iguala o corpo do ficheiro anterior (corte e cola, sem reescrita)
```

## Resultado Esperado

### P1307-R6 — re-exportação interna do método de snapshot aprovado

Medição anterior à decisão: no baseline R6
`00_nucleo/diagnosticos/p1307-r6-baseline.json`, SHA-256
`8cc0eae00457d2e7d54b420024eae49344032b34b295b4eda536faeb1ec3c4a3`,
`01_core/src/compiler/eval/bindings/mod.rs:27` já reexporta os helpers
crate-internos de Content. O owner
`00_nucleo/prompts/compiler/eval/bindings/field_access.md` R5 aprovado pelo
dono acrescenta helper interno que aceita IntrospectedContent sem descartar
o snapshot; call_dispatch acessa os helpers pela mesma fronteira deste hub.

Decisão: reexportar também `eval_introspected_content_method_at` com a mesma
visibilidade `pub(crate)`. Não alterar os helpers públicos externos, adicionar
lógica ao hub, realizar campos ou transformar o receiver. O nome interno
concretiza a delegação já aprovada e não acrescenta uma superfície Typst.
Fluxo contínuo ADR-0127; a propriedade semântica pertence a field_access.
Teste de compilação e equivalência estática/instância verificam o transporte.

- `bindings/mod.rs` — só `mod` + re-exportações.
- Cinco nós, cada um com L0 próprio e `@prompt` próprio (V15).
- Zero mudança de comportamento; suite do eval inalterada.
