# Passo 1000 — Mapeamento: estrutura de `eval` (cristalino vs vanilla) e `main` vanilla

**Tipo**: Reconhecimento/mapeamento — sem decisão, sem correção, sem código alterado.  
**Estado de medição**: commit `00dc949665317dedabc5ec01f0af5f4f4dc6cc80`, working tree limpo (`git diff HEAD --stat` vazio).  
**Data da varredura**: 2026-08-12.

---

## Parte 1 — Estrutura do `eval` no cristalino

Diretório: `01_core/src/engine/eval/`

| Ficheiro | Linhas | Âmbito de alto nível |
|---|---|---|
| `mod.rs` | 1 995 | Entry points `eval` / `eval_with_full_error`; `EvalContext`; despacho central; construção da stdlib (`make_stdlib`); helpers de despacho (`eval_markup_body`, `remaining_are_dict_spreads`, `content_has_state_or_counter`). |
| `bindings.rs` | 2 235 | Despacho de acesso/call em valores e conteúdos: destructuring, field access, method calls, argument handling, contadores/state/query selectors, campos de content/element. |
| `rules.rs` | 2 720 | Show/set rules: matching de seletores, realização de flow/node, captura de estilos, parse de fontes, variantes, regras de bibliografia/citação. |
| `closures.rs` | 1 120 | Aplicação de funções (`apply_func`), tracing, merge de args, chamadas a plugin, métodos de location. |
| `operators.rs` | 1 148 | Operadores binários/unários, comparação e igualdade de valores, conversões de comprimento/relativo, formatação de mismatch. |
| `math.rs` | 1 288 | Avaliação de expressões matemáticas: lookup de operadores, parsing de delimitadores (`lr`), avaliação de callee/arg/value. |
| `repr.rs` | 1 292 | Representação textual de valores (`repr`), cores, strokes, alinhamentos, duração, data/hora, listas/array. |
| `bibtex.rs` | 489 | Parser de ficheiros BibTeX. |
| `bibliography.rs` | 486 | Carregamento de entradas bib, resolução de estilos CSL, conversão hayagriva → `BibEntry`. |
| `modules.rs` | 298 | Avaliação de ficheiros importados (`eval_imported_file`). |
| `control_flow.rs` | 185 | Loops (`run_for_loop`). |
| `markup.rs` | 154 | Corpo de markup com gestão de delta de estilos (`eval_body_with_delta`). |
| `cast.rs` | 57 | Conversão de `Value` → `Length` (`cast_length`). |
| `flow.rs` | 75 | Eventos de controlo de fluxo (reexportado como `FlowEvent`). |
| `tests.rs` | 15 452 | Testes unitários do módulo. |

**Funções de topo por ficheiro** (retirado de `grep -n '^pub fn \|^fn ' 01_core/src/engine/eval/*.rs`):

- `bibliography.rs`: `load_bib_entries_from_path`, `parse_bibliography`, `resolve_style`, `load_csl_style_from_path`, `hay_entry_to_bib_entry`, `person_to_string`.
- `bibtex.rs`: `parse_bibtex` + helpers de parse de entradas/chaves/campos/valores/autores.
- `bindings.rs`: `destructure_pattern`, `destructure_array`, `destructure_dict`, `wrong_number_of_elements`, `access`, `access_dict`, `expect_positional`, `finish_args`, `has_readonly_method`, `call_method_access`, `is_dict_mutating_method`, `call_method_mut`, `state_at_dispatch`, `parse_counter_display_args`, `extract_display_at_label`, `render_counter_at_label`, `value_to_query_selector`, `element_or_type_with_name`, `content_field`, `content_set_fields`, `content_func_not_callable`, `content_elem_func`.
- `cast.rs`: `cast_length`.
- `closures.rs`: `apply_func`, `trace_call`, `merge_with_args`, `call_plugin`, `eval_location_method`.
- `control_flow.rs`: `run_for_loop`.
- `markup.rs`: `eval_body_with_delta`.
- `math.rs`: `lookup_math_op`, `unknown_variable_math`, `eval_math_callee`, `eval_math_arg_value`, `eval_math_expr`, `rewrite_lr_body`, `lr_delim_char`, `parse_delim_val`, `parse_delim_char`, `parse_delim_str`.
- `mod.rs`: `eval`, `eval_with_full_error`, `content_has_state_or_counter`, `remaining_are_dict_spreads`, `eval_markup_body`, `make_stdlib`.
- `modules.rs`: `eval_imported_file`.
- `operators.rs`: `binary_mismatch`, `long_type_name`, `value_eq`, `values_eq`, `value_cmp`, `cmp_arrays`, `length_partial_cmp`, `rel_partial_cmp`, `sanitize_length_nan`.
- `repr.rs`: `repr_value`, `repr_duration`, `repr_relative`, `round_with_precision`, `format_float_with_unit`, `format_float_component`, `format_float_rounded`, `repr_length`, `repr_ratio`, `repr_angle`, `repr_color`, `repr_stroke`, `repr_paint`, `repr_align`, `repr_content`, `repr_selector`, `is_closure`, `repr_float`, `repr_datetime`, `pretty_comma_list`, `pretty_array_like`.
- `rules.rs`: `query_selector_to_show_selector`, `unsupported_property_warn`, `unsupported_target_warn`, `is_styled_origin`, `selector_matches`, `values_eq_semantic`, `is_node_rule`, `splice_text_rule_matches`, `realize_flow`, `realize_node`, `capture_set_styles`, `parse_font_dict_named_fields`, `parse_font_dict_legacy`, `variants_from_value`.

**Divisão do cristalino**: a organização é mista — por **domínio da linguagem** (`math.rs`, `markup.rs`, `modules.rs`), por **concerns transversais do eval** (`bindings.rs`, `closures.rs`, `operators.rs`, `rules.rs`, `repr.rs`, `flow.rs`, `control_flow.rs`) e por **formatos de dados** (`bibtex.rs`, `bibliography.rs`). O `mod.rs` mantém o despacho central e os entry points.

---

## Parte 2 — Estrutura do `eval` no vanilla (`lab/typst-original/crates/typst-eval`)

Diretório: `lab/typst-original/crates/typst-eval/src/`

| Ficheiro | Linhas | Âmbito de alto nível |
|---|---|---|
| `lib.rs` | 184 | Entry points `eval` / `eval_string`; trait `Eval`; re-exports (`Vm`, `FlowEvent`, `eval_closure`, `import`, `CapturesVisitor`). |
| `call.rs` | 1 048 | Avaliação de chamadas: calls gerais, math calls, field callee, closures, resolução de mutação, `element_or_type_with_name`. |
| `code.rs` | 525 | Avaliação de nós de código: blocos, expressões, literais, loops, condicionais, return, break, continue, erro de inteiros, `find_bad_digits`. |
| `markup.rs` | 249 | Avaliação de nós de markup: texto, strong, emph, heading, link, ref, enum/list/item, raw, parbreak, etc. |
| `math.rs` | 183 | Implementações `Eval` para nós matemáticos (math, attach, frac, lr, root, etc.). |
| `ops.rs` | 152 | Operadores binários e unários, assignments, overflow de inteiros. |
| `binding.rs` | 209 | Destructuring de patterns (array/dict). |
| `access.rs` | 107 | Acesso a campos/métodos em valores. |
| `methods.rs` | 104 | Resolução e erro de métodos em tipos. |
| `flow.rs` | 245 | Controlo de fluxo: loops, condicionais, return/break/continue, `FlowEvent`, `is_invariant`, `can_diverge`. |
| `import.rs` | 295 | Importações e packages (`import`, `import_file`, `import_package`, `resolve_package`). |
| `rules.rs` | 95 | Regras de show/set: validações de page rule e par set block. |
| `vm.rs` | 113 | Máquina virtual `Vm` + `hint_if_shadowed_std`. |

**Funções de topo por ficheiro** (retirado de `grep -n '^pub fn \|^fn ' lab/typst-original/crates/typst-eval/src/*.rs`):

- `lib.rs`: `eval`, `eval_string`.
- `call.rs`: `eval_math_call`, `call_func`, `maybe_resolve_mutating`, `eval_field_callee`, `disallowed_field_call_error`, `element_or_type_with_name`, `unparse_math_args`, `eval_closure`.
- `code.rs`: `eval_code`, `warn_for_discarded_content`, `int_literal_error`, `find_bad_digits`.
- `markup.rs`: `eval_markup`.
- `methods.rs`: `missing_method`.
- `ops.rs`: `apply_binary`, `apply_binary_with`, `apply_assignment`, `overflowing_int_negation_error`.
- `rules.rs`: `check_show_page_rule`, `check_show_par_set_block`.
- `vm.rs`: `hint_if_shadowed_std`.
- `binding.rs`: `destructure_impl`, `destructure_array`, `destructure_dict`, `wrong_number_of_elements`.
- `flow.rs`: `is_invariant`, `can_diverge`.
- `import.rs`: `import`, `import_file`, `import_package`, `resolve_package`.

**Divisão do vanilla**: predominantemente por **tipo de nó da AST / domínio da linguagem** — cada ficheiro implementa o trait `Eval` para um subconjunto de nós (`code.rs`, `markup.rs`, `math.rs`, `call.rs`, `ops.rs`, `binding.rs`, `access.rs`, `methods.rs`, `flow.rs`, `import.rs`, `rules.rs`). A lógica de baixo nível (VM, engine) vive fora, em `typst-library`.

---

## Parte 3 — Estrutura do `main` vanilla

Ficheiro: `lab/typst-original/crates/typst-cli/src/main.rs`

```text
main()
  ├── sigpipe::reset()
  ├── dispatch()
  │     ├── Command::Compile  → crate::compile::compile(command)
  │     ├── Command::Watch    → crate::watch::watch(command)
  │     ├── Command::Init     → crate::init::init(command)
  │     ├── Command::Query    → crate::query::query(command)
  │     ├── Command::Eval     → crate::eval::eval(command)
  │     ├── Command::Fonts    → crate::fonts::fonts(command)
  │     ├── Command::Update   → crate::update::update(command)
  │     ├── Command::Completions → crate::completions::completions(command)
  │     └── Command::Info     → crate::info::info(command)
  └── EXIT com tratamento de erro/hint
```

Subcomandos definidos em `args.rs::Command`:

- `compile` (alias `c`) — compilar ficheiro de entrada.
- `watch` (alias `w`) — recompilar em alterações.
- `init` — inicializar projeto a partir de template.
- `query` — extrair metadata (deprecated, escondido).
- `eval` — avaliar código Typst.
- `fonts` — listar fontes descobertas.
- `update` — self-update (condicional a feature).
- `completions` — gerar scripts de shell completion.
- `info` — informações de debugging.

Módulos auxiliares do CLI: `args`, `compile`, `completions`, `deps`, `download`, `eval`, `fonts`, `greet`, `info`, `init`, `packages`, `query`, `terminal`, `update`, `watch`, `world`.

---

## Parte 4 — Comparação directa

| Responsabilidade | Cristalino (`01_core/src/engine/eval/`) | Vanilla (`crates/typst-eval/src/`) |
|---|---|---|
| Entry point de avaliação de ficheiro | `mod.rs::eval` | `lib.rs::eval` |
| Entry point de avaliação de string | `mod.rs::eval_with_full_error` (com modo) | `lib.rs::eval_string` |
| Trait / dispatcher de nós | `mod.rs` (funções `eval_*` por tipo) | `lib.rs::Eval` + impls dispersas |
| Avaliação de markup | `markup.rs` (`eval_body_with_delta`) | `markup.rs` (`eval_markup` + impls `Eval`) |
| Avaliação de math | `math.rs` | `math.rs` |
| Avaliação de código (code block, exprs) | `mod.rs` / `control_flow.rs` | `code.rs` |
| Chamadas / closures | `closures.rs` (`apply_func`) | `call.rs` (`call_func`, `eval_closure`) |
| Destructuring / bindings | `bindings.rs` | `binding.rs` |
| Operadores | `operators.rs` | `ops.rs` |
| Acesso a campos/métodos | `bindings.rs` | `access.rs` + `methods.rs` |
| Controlo de fluxo | `flow.rs` + `control_flow.rs` | `flow.rs` |
| Importações / packages | `modules.rs` | `import.rs` |
| Show/set rules | `rules.rs` | `rules.rs` |
| Repr / debug de valores | `repr.rs` | (em `typst-library`, foundations) |
| Bibliografia / BibTeX | `bibliography.rs` + `bibtex.rs` | (fora de `typst-eval`, em `typst-library`) |
| VM / contexto de execução | `EvalContext` em `mod.rs` | `Vm` em `vm.rs` |
| Main CLI / subcomandos | `02_shell/src/main.rs` (não abrangido neste passo) | `crates/typst-cli/src/main.rs` |

**Observação**: a granularidade do cristalino é maior em ficheiros como `bindings.rs`, `rules.rs` e `repr.rs`, onde várias responsabilidades (acesso, chamadas, regras, representação) foram agrupadas em módulos monolíticos. O vanilla, pelo contrário, separa mais nitidamente por tipo de nó AST (`code.rs`, `markup.rs`, `math.rs`, `call.rs`, `ops.rs`) e por mecanismo (`access.rs`, `methods.rs`, `binding.rs`).

---

## O que este passo NÃO faz

- Não decide como fatiar `eval.md`.
- Não avalia se a estrutura do vanilla é "melhor" — só descreve.
- Não escreve nenhum prompt novo.
- Não altera nenhum ficheiro de código.

## Resultado entregue

Documento de mapeamento preenchido (`00_nucleo/diagnosticos/mapeamento-eval-vanilla-cristalino-passo-1000.md`) com as 4 partes solicitadas, pronto a alimentar o passo seguinte (fatiamento de `eval.md`).
