# Prompt L0 — `compiler/eval/bindings/value_methods` — métodos de instância com args em AST
Hash do Código: b7638e8b

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/bindings/value_methods.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/bindings.md`
**ADRs**: ADR-0107 (paridade língua), ADR-0044 (`Engine<'_>`)

---

## Contexto

Este nó reúne os métodos de instância sobre tipos de valor cujos **argumentos
ainda estão em AST** no momento do despacho — `state`, `counter`, `color`,
`version` e os combinadores de `selector`. Todos partilham a mesma abertura
(`call_dispatch::eval_args`) e por isso recebem `Scopes`/`EvalContext`/`Engine`.

**No vanilla não existe um ficheiro correspondente**: estas são nativas
declaradas junto do tipo em `typst-library::{introspection, visualize,
foundations}`. A agregação é do cristalino. O que justifica o nó não é o
vanilla mas a co-mudança: P506 (state+counter), P640 (counter.display), P742
(color+state), P796 (version), P417/P423/P504 (selector) movem estes símbolos
em conjunto e quase nunca em conjunto com os outros nós.

## Restrições Estruturais

- L1 puro; nenhuma leitura de ficheiro ou relógio.
- O contexto é usado de três formas distintas, **medidas por `file:line`** e a
  respeitar em qualquer alteração (lição do Passo 1012 — a presença do
  parâmetro não é prova de uso):
  1. **Pass-through**: `eval_version_method_value`, `eval_element_where`,
     `eval_selector_or_and`, `eval_selector_within` usam `ctx`/`engine`
     **apenas** para `eval_args`; depois disso, zero usos.
  2. **`Engine` real**: `eval_color_method` lê `engine.world` e
     `engine.current_file` para as nativas de cor.
  3. **Estado/introspecção**: `eval_state_method` e `eval_counter_method_value`
     propagam `ctx` para `state_*`/`counter_*` e para o introspector.

## Instrução

### `eval_state_method(state, method, args, …)`

`update`, `get`, `display`, `at`, `final` — delegando em
`stdlib::state::{state_update, state_get, state_display, state_at_location,
state_final}`. `at` passa por `state_at_dispatch`, que resolve o argumento
(label ou localização) antes de consultar o introspector.

### `eval_counter_method_value(counter, method, args, …)`

`step`, `update`, `get`, `display`, `at`, `final` — delegando em
`stdlib::counter::*`. O caminho de `display` passa por
`parse_counter_display_args` (padrão de numeração opcional),
`extract_display_at_label` e `render_counter_at_label` (P640 — unificação de
`counter.display` e das suas mensagens de erro).

### `eval_color_method(color, method, args, …)`

`lighten`, `darken`, `saturate`, `desaturate`, `negate`, `rotate`, `mix`,
`opacify`, `transparentize`, `components`, `space`, `to-hex` — delegando em
`stdlib::color`. Único método deste nó que precisa de `World`/`current_file`.

### `eval_version_method_value(version, method, args, …)`

`at` — componente por índice (P411/P796).

### Combinadores de selector

`eval_element_where` (`elem.where(field: v)`), `eval_selector_or_and`
(`.or()`/`.and()`), `eval_selector_within` (`.within()`), suportados por
`value_to_query_selector` (conversão `Value → Selector`).

## Critérios de Verificação

```
#let s = state("k", 0); s.update(1); #s.get()      → 1
#counter("c").step(); #counter("c").get()          → (1,)
#counter(heading).display("1.1")                   → numeração conforme padrão
#counter("c").at(<lbl>)                            → valor no local da label
rgb("#ff0000").lighten(50%)                        → cor mais clara
rgb("#ff0000").to-hex()                            → "#ff0000"
version(1, 2, 3).at(1)                             → 2
heading.where(level: 1)                            → Selector de heading nível 1
heading.where(level: 1).or(strong)                 → Selector Or
figure.where(kind: image).within(heading)          → Selector Within
```

## Resultado Esperado

- `eval_state_method`, `eval_counter_method_value`, `eval_color_method`,
  `eval_version_method_value`, `eval_element_where`, `eval_selector_or_and`,
  `eval_selector_within` visíveis em `eval`; os auxiliares privados ao nó.
