# Passo 1013 — Fatiamento hub/nó `eval::bindings`

**Tipo**: Aplicação do método completo (P1002) a `compiler::eval::bindings`, terceiro e
último da família `eval::*` nesta leva (depois de `rules` P1009/1011, `closures` P1012).
**Candidato confirmado**: P1008 + adendo P1010 — agregado (sem `trait`), fan-in real de
módulo (10 ficheiros distintos chamam os seus símbolos, dominados internamente por `eval`/
`stdlib`; `long_type_name` confirmado com 11 chamadas reais em 5 ficheiros, não 36).
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1012 (inclui `call_dispatch`,
`font_dict` corrigido).

---

## Lição a aplicar desde já (não redescobrir)

Do Passo 1012 (`font_dict`): a presença de `Engine`/`EvalContext` como parâmetro **não é
cosmética** — confirmar por `file:line` se cada função efectivamente propaga esse contexto
para algo com efeito lateral (via `eval_expr`, `apply_func`, etc.), não presumir pureza ou
impureza a partir do propósito/nome da função.

## Fase A — Inventário completo

```
grep -n '^pub fn \|^pub(crate) fn \|^fn ' 01_core/src/compiler/eval/bindings.rs
```
Confirmar contra a lista do Passo 1000 (`destructure_pattern`, `destructure_array`,
`destructure_dict`, `wrong_number_of_elements`, `access`, `access_dict`,
`expect_positional`, `finish_args`, `has_readonly_method`, `call_method_access`,
`is_dict_mutating_method`, `call_method_mut`, `state_at_dispatch`,
`parse_counter_display_args`, `extract_display_at_label`, `render_counter_at_label`,
`value_to_query_selector`, `element_or_type_with_name`, `content_field`,
`content_set_fields`, `content_func_not_callable`, `content_elem_func`,
`long_type_name`) — o ficheiro pode ter mudado desde então; não presumir que a lista
continua exacta.

## Fase B — Os 4 critérios, com evidência

1. **Isolamento de teste** — quais funções precisam de `Engine`/`EvalContext` reais
   (dispatch de método, counter/state) vs quais operam só sobre `Value`/`Content`/`Dict`
   (destructuring, `access_dict`, `content_field`)?
2. **Pureza vs estado** — aplicar per `file:line`, como no P1012. Hipótese a confirmar, não
   presumir: `destructure_*` e `access`/`access_dict`/`content_field` são candidatos a
   puros (`Value`/`Content` → `Value`, sem `Engine`); `state_at_dispatch`,
   `parse_counter_display_args`, `extract_display_at_label`, `render_counter_at_label`
   tocam introspecção (contadores/state), logo stateful; `call_method_access`/
   `call_method_mut`/`has_readonly_method`/`is_dict_mutating_method` são dispatch de
   método, confirmar se tocam `Engine` ou só despacham por tipo.
3. **Co-mudança histórica** — mesma técnica, descontar ruído de resselo.
4. **Correspondência vanilla** — já sabemos do P1003: `typst_eval::access` + `methods` +
   `binding` (três ficheiros pequenos, 107+104+209 linhas) fragmentam o que o cristalino
   agrega num só `bindings.rs`. Usar como candidato de fronteira (destructuring / access
   de campo / dispatch de método = três nós plausíveis), confirmar com Critério 3 antes de
   aceitar às cegas — o P1002 já mostrou que a divisão "óbvia" por inspecção pode falhar.

## Fase C — Materializar

Nós conforme a Fase B decidir. Candidato inicial a testar contra os critérios (não
comprometer sem confirmar): `destructuring.md`/`.rs` (puro), `access.md`/`.rs` (campo/
content, maioritariamente puro), `method_dispatch.md`/`.rs` (stateful), `counter_display.md`/
`.rs` (stateful, introspecção). Ajustar conforme a evidência real de co-mudança, não impor
esta divisão a priori.

Cada nó: L0 sem referência a passo, campo Técnica preenchido só se houver algo nomeável
(destructuring de padrões pode corresponder a *pattern matching* estrutural — confirmar se
vale a nota), ficheiro `.rs` próprio (V15).

## Fase D — Validar

```
crystalline-lint .
cargo test --workspace
```
Zero regressão. Confirmar contagem de testes antes/depois.

## Fase E — Avaliação do método

Registar se a fronteira vanilla (access/methods/binding) bateu com o Critério 3, ou se a
co-mudança real revelou outra divisão — mesmo padrão de honestidade já usado em P1002
(`coercion.md` descartado) e P1006 (`metrics` recusado por inteiro).

---

## Resultado esperado

`eval::bindings` fatiado com evidência dos 4 critérios, zero regressão. Com este passo,
a família `eval::*` da leva do P1008 fica completa (`rules`, `closures`, `bindings`).
Próximo: `stdlib::structural` (Passo 1014).
