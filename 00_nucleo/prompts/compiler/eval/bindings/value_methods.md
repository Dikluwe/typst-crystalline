# Prompt L0 — `compiler/eval/bindings/value_methods` — métodos de instância com args em AST
Hash do Código: efef75f5

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

## P1149 — argumentos contextuais completos de counter

`parse_counter_display_args` preserva `at:` como `Label`, `Location`,
`Selector` ou `Auto`, aceita `both: bool` e transporta o numbering posicional
como string, função ou `Auto` ao owner. Não formata por concatenação local.

`eval_counter_static_method_value` cobre somente `counter.at` e
`counter.display`, extraindo o receiver e preservando literal label antes da
avaliação genérica. Depois delega a `stdlib/counter`; resolução de location,
numbering, total final e callback não vivem neste nó.

## P1284 — limite fechado do glue AST

Este nó continua owner apenas da orchestration que realmente precisa dos
argumentos em AST: `state`, `counter`, `color`, `version`, `where` e os
combinadores de selector. Coleções, `arguments`, direção, alinhamento,
duração, comprimento e localização não movem sua semântica para cá.

Para selector, P1284 autoriza somente:

- `and(self, ..others)` → `Selector::And`, preservando receiver primeiro e a
  ordem dos demais selectors;
- `or(self, ..others)` → `Selector::Or`, com a mesma disciplina;
- `within(self, ancestor)` → `Selector::Within` já existente.

As formas não ligadas recebem `self` primeiro e chamam estas mesmas rotas.
Conversão `Value → Selector`, aridade, tipo, metadata e erros são únicos; não
há implementação paralela no wrapper.

`before(self,end,inclusive:true)` e `after(self,start,inclusive:true)` ficam
fora do match e são `BLOCKED_ADR0127_PUBLIC_CONTRACT`: o enum público atual
não contém as variantes, e os consumers exaustivos exigiriam revisão. Não
simular com `Within`, `And`, `Or`, `Where` ou composição aproximada.

No ramo de cor, `color.map` não passa por este nó: é constante de tipo com
kind `module`, descoberta por `field_access` e construída por `stdlib/color`.
`color.spot/tint` permanece bloqueado. Nenhuma alteração de contrato Rust
público, default ou fase é autorizada.

## P1307-R4 — transporte do carrier Args aprovado

### Medição anterior à decisão

Sobre HEAD `b303f1f15b610e09872b567027e0d806387fde8c` e working tree composto
P1306 + L0 R3 aprovado, baseline R4 SHA-256
`52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57`,
`01_core/src/compiler/eval/bindings/value_methods.rs:202` insere Color em
Args avaliado; `:256-284,524-548` constrói Args de display a partir de AST
com spans disponíveis, mas guarda somente views. O texto L0 anterior foi
congelado em `00_nucleo/diagnosticos/p1307-r4-transport-amendment.json`.
Esses writers não podem manter Some incoerente após migração de eval_args.

### Decisão de adaptação interna

Este owner consome a API de `entities/args.md` aprovada pelo dono no gate
P1307-R3. Na chamada ligada de cor, antepor uma ocorrência positional com
Value::Color do receiver e spans individuais detached (o helper não recebe
origem lexical do receiver), reconstruindo por from_occurrences e preservando
span agregado e origens dos demais argumentos. None continua síntese explícita
por from_parts. Não inferir origem por igualdade de cor nem invalidar Some.

Os dois builders AST de counter.display devem construir ocorrências de cada
argumento aceito na ordem de avaliação, com span do argumento completo e da
expressão-valor, e finalizar por from_occurrences com o agregado vigente.
O literal label em at continua preservado pelo tratamento sintático existente;
não avaliá-lo novamente. Manter critérios, aridade, casts, rejeição de spread,
mensagens e ordem de falhas atuais; não promover estes métodos à validação
nova de encoders ou ampliar suporte de counter/selector/color.

Testes preservam os caminhos ligados/estáticos de display, at/both, label,
cor e erros preexistentes, além da coerência Some/views. É inferência que
esses dados já disponíveis bastem; nova informação de domínio ou mudança
pública além do carrier aprovado exige reabertura. Fluxo contínuo ADR-0127
para esta migração interna, com L0 primeiro e verificação independente.
