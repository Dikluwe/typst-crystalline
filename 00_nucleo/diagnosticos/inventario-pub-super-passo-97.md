# Inventário `pub(super)` — Passo 97.A

Data de captura: 2026-04-23.
Total: **269** ocorrências em `01_core/src/rules/`.

Ficheiros `tests.rs` (cfg(test) gated) fora do escopo.

## Resumo por módulo

| Módulo | Total | fn | field | outros |
|--------|------:|---:|------:|-------:|
| `parse` | 135 | 102 | 23 | 10 |
| `layout` | 46 | 30 | 14 | 2 |
| `math` | 29 | 18 | 7 | 4 |
| `eval` | 24 | 24 | 0 | 0 |
| `lexer` | 24 | 16 | 4 | 4 |
| `stdlib` | 11 | 11 | 0 | 0 |

## Detalhe por ficheiro

### `src/rules/parse/parser.rs` (77)

| Linha | Tipo | Nome |
|------:|------|------|
| 24 | const | `MAX_DEPTH` |
| 60 | struct | `Parser` |
| 62 | field | `text` |
| 66 | field | `lexer` |
| 68 | field | `nl_mode` |
| 73 | field | `token` |
| 76 | field | `balanced` |
| 79 | field | `nodes` |
| 83 | field | `memo` |
| 85 | field | `depth` |
| 91 | struct | `Token` |
| 93 | field | `kind` |
| 96 | field | `node` |
| 98 | field | `n_trivia` |
| 100 | field | `newline` |
| 103 | field | `start` |
| 105 | field | `prev_end` |
| 110 | struct | `Newline` |
| 112 | field | `column` |
| 114 | field | `parbreak` |
| 119 | enum | `AtNewline` |
| 136 | fn | `stop_at` |
| 161 | struct | `Marker` |
| 181 | fn | `new` |
| 200 | fn | `finish` |
| 205 | fn | `finish_into` |
| 212 | fn | `current` |
| 217 | fn | `at` |
| 222 | fn | `at_set` |
| 229 | fn | `end` |
| 234 | fn | `directly_at` |
| 239 | fn | `had_trivia` |
| 244 | fn | `had_newline` |
| 250 | fn | `current_column` |
| 258 | fn | `current_text` |
| 263 | fn | `current_start` |
| 268 | fn | `current_end` |
| 274 | fn | `prev_end` |
| 283 | fn | `marker` |
| 289 | fn | `before_trivia` |
| 295 | fn | `eat_and_get` |
| 305 | fn | `eat_if` |
| 317 | fn | `assert` |
| 323 | fn | `convert_and_eat` |
| 331 | fn | `eat` |
| 338 | fn | `flush_trivia` |
| 347 | fn | `wrap` |
| 358 | fn | `wrap_error` |
| 375 | fn | `enter_modes` |
| 396 | fn | `with_nl_mode` |
| 419 | fn | `lex` |
| 459 | struct | `MemoArena` |
| 462 | field | `arena` |
| 466 | field | `memo_map` |
| 475 | struct | `Checkpoint` |
| 476 | field | `node_len` |
| 477 | field | `state` |
| 483 | struct | `PartialState` |
| 484 | field | `cursor` |
| 485 | field | `lex_mode` |
| 486 | field | `token` |
| 493 | fn | `memoize_parsed_nodes` |
| 503 | fn | `restore_memo_or_checkpoint` |
| 520 | fn | `restore` |
| 526 | fn | `restore_partial` |
| 533 | fn | `checkpoint` |
| 548 | fn | `expect` |
| 565 | fn | `expect_closing_delimiter` |
| 572 | fn | `expected` |
| 579 | fn | `after_error` |
| 586 | fn | `expected_at` |
| 593 | fn | `hint` |
| 602 | fn | `unexpected` |
| 609 | fn | `trim_errors` |
| 629 | fn | `check_depth_until` |
| 641 | fn | `increase_depth` |
| 652 | fn | `depth_check_error` |

### `src/rules/layout/mod.rs` (22)

| Linha | Tipo | Nome |
|------:|------|------|
| 56 | other | (tuple/use) |
| 60 | other | (tuple/use) |
| 64 | field | `metrics` |
| 66 | field | `font_size_pt` |
| 67 | field | `style` |
| 70 | field | `pages` |
| 72 | field | `current_items` |
| 73 | field | `cursor_x` |
| 74 | field | `cursor_y` |
| 80 | field | `line_start_x` |
| 81 | field | `current_line` |
| 93 | field | `is_height_unconstrained` |
| 101 | field | `cell_available_h` |
| 108 | field | `cell_origin_x` |
| 109 | field | `cell_origin_y` |
| 110 | field | `cell_origin_w` |
| 141 | fn | `available_width` |
| 147 | fn | `available_height` |
| 155 | fn | `page_bottom_limit` |
| 166 | fn | `resolve_alignment` |
| 575 | fn | `measure_content_constrained` |
| 629 | fn | `layout_sub_frame_with_width` |

### `src/rules/math/layout/mod.rs` (21)

| Linha | Tipo | Nome |
|------:|------|------|
| 31 | other | (tuple/use) |
| 35 | other | (tuple/use) |
| 37 | struct | `MathBox` |
| 38 | field | `width` |
| 39 | field | `ascent` |
| 40 | field | `descent` |
| 42 | field | `items` |
| 85 | fn | `offset_item` |
| 137 | fn | `needs_grid_layout` |
| 149 | fn | `partition_grid` |
| 193 | enum | `GridAlign` |
| 203 | field | `metrics` |
| 204 | field | `constants` |
| 207 | field | `block` |
| 222 | fn | `apply_axis_offset` |
| 246 | fn | `layout_node` |
| 311 | fn | `layout_text_node` |
| 331 | fn | `layout_sequence` |
| 348 | fn | `layout_grid_rows` |
| 437 | fn | `layout_grid` |
| 449 | fn | `hconcat` |

### `src/rules/lexer/mod.rs` (16)

| Linha | Tipo | Nome |
|------:|------|------|
| 24 | other | (tuple/use) |
| 29 | other | (tuple/use) |
| 30 | struct | `Lexer` |
| 32 | field | `s` |
| 35 | field | `mode` |
| 37 | field | `newline` |
| 39 | field | `error` |
| 90 | fn | `error` |
| 96 | fn | `hint` |
| 200 | fn | `keyword` |
| 228 | trait | `ScannerExt` |
| 249 | fn | `is_space` |
| 329 | fn | `count_newlines` |
| 372 | fn | `is_math_id_start` |
| 378 | fn | `is_math_id_continue` |
| 384 | fn | `is_valid_in_label_literal` |

### `src/rules/parse/patterns.rs` (13)

| Linha | Tipo | Nome |
|------:|------|------|
| 22 | fn | `expr_with_paren` |
| 90 | fn | `parenthesized_or_array_or_dict` |
| 148 | struct | `GroupState` |
| 161 | fn | `array_or_dict_item` |
| 209 | fn | `node_key` |
| 217 | fn | `args` |
| 257 | fn | `arg` |
| 289 | fn | `params` |
| 316 | fn | `param` |
| 348 | fn | `pattern` |
| 364 | fn | `destructuring_or_parenthesized` |
| 402 | fn | `destructuring_item` |
| 449 | fn | `pattern_leaf` |

### `src/rules/parse/rules.rs` (13)

| Linha | Tipo | Nome |
|------:|------|------|
| 21 | fn | `let_binding` |
| 52 | fn | `set_rule` |
| 71 | fn | `show_rule` |
| 90 | fn | `contextual` |
| 98 | fn | `conditional` |
| 114 | fn | `while_loop` |
| 123 | fn | `for_loop` |
| 146 | fn | `module_import` |
| 176 | fn | `import_items` |
| 206 | fn | `module_include` |
| 214 | fn | `break_stmt` |
| 221 | fn | `continue_stmt` |
| 228 | fn | `return_stmt` |

### `src/rules/parse/markup.rs` (12)

| Linha | Tipo | Nome |
|------:|------|------|
| 24 | fn | `markup` |
| 34 | fn | `markup_exprs` |
| 49 | fn | `reparse_markup` |
| 72 | fn | `markup_expr` |
| 122 | fn | `strong` |
| 133 | fn | `emph` |
| 144 | fn | `heading` |
| 154 | fn | `list_item` |
| 164 | fn | `enum_item` |
| 174 | fn | `term_item` |
| 188 | fn | `reference` |
| 198 | fn | `equation` |

### `src/rules/parse/code.rs` (10)

| Linha | Tipo | Nome |
|------:|------|------|
| 31 | fn | `code` |
| 38 | fn | `code_exprs` |
| 61 | fn | `embedded_code_expr` |
| 88 | fn | `code_expr` |
| 93 | fn | `code_expr_prec` |
| 165 | fn | `code_primary` |
| 226 | fn | `reparse_block` |
| 235 | fn | `block` |
| 244 | fn | `code_block` |
| 255 | fn | `content_block` |

### `src/rules/parse/math.rs` (10)

| Linha | Tipo | Nome |
|------:|------|------|
| 24 | fn | `math` |
| 32 | fn | `math_exprs` |
| 50 | fn | `math_expr` |
| 56 | fn | `math_expr_prec` |
| 191 | fn | `math_op` |
| 208 | fn | `is_math_alphabetic` |
| 222 | fn | `math_delimited` |
| 247 | fn | `math_unparen` |
| 264 | fn | `math_args` |
| 313 | fn | `math_arg` |

### `src/rules/eval/markup.rs` (7)

| Linha | Tipo | Nome |
|------:|------|------|
| 29 | fn | `eval_strong` |
| 46 | fn | `eval_emph` |
| 62 | fn | `eval_heading` |
| 80 | fn | `eval_raw` |
| 93 | fn | `eval_link` |
| 99 | fn | `eval_list_item` |
| 112 | fn | `eval_enum_item` |

### `src/rules/layout/helpers.rs` (7)

| Linha | Tipo | Nome |
|------:|------|------|
| 18 | fn | `item_pos` |
| 30 | fn | `translate_frame_item` |
| 54 | fn | `heading_scale` |
| 62 | fn | `resolve_pt` |
| 78 | fn | `measure_content` |
| 106 | fn | `collect_sub_items` |
| 112 | fn | `collect_items_at` |

### `src/rules/layout/cursor.rs` (6)

| Linha | Tipo | Nome |
|------:|------|------|
| 19 | fn | `word_width` |
| 23 | fn | `space_width` |
| 27 | fn | `layout_word` |
| 41 | fn | `flush_line` |
| 63 | fn | `new_page` |
| 81 | fn | `current_page_number` |

### `src/rules/lexer/markup.rs` (5)

| Linha | Tipo | Nome |
|------:|------|------|
| 22 | fn | `markup` |
| 59 | fn | `backslash` |
| 88 | fn | `raw` |
| 325 | fn | `label` |
| 397 | fn | `space_or_end` |

### `src/rules/eval/bindings.rs` (4)

| Linha | Tipo | Nome |
|------:|------|------|
| 27 | fn | `eval_let` |
| 68 | fn | `extract_counter_key` |
| 90 | fn | `eval_counter_method` |
| 137 | fn | `eval_field_access` |

### `src/rules/eval/closures.rs` (4)

| Linha | Tipo | Nome |
|------:|------|------|
| 36 | fn | `eval_args` |
| 90 | fn | `apply_closure` |
| 149 | fn | `eval_closure_expr` |
| 191 | fn | `eval_func_call` |

### `src/rules/eval/control_flow.rs` (3)

| Linha | Tipo | Nome |
|------:|------|------|
| 25 | fn | `eval_conditional` |
| 48 | fn | `eval_while` |
| 76 | fn | `eval_for` |

### `src/rules/layout/counters.rs` (3)

| Linha | Tipo | Nome |
|------:|------|------|
| 11 | fn | `layout_set_heading_numbering` |
| 17 | fn | `layout_counter_update` |
| 38 | fn | `format_counter_display` |

### `src/rules/stdlib/layout.rs` (3)

| Linha | Tipo | Nome |
|------:|------|------|
| 112 | fn | `extract_alignment` |
| 124 | fn | `parse_track_sizing` |
| 135 | fn | `extract_tracks` |

### `src/rules/eval/math.rs` (2)

| Linha | Tipo | Nome |
|------:|------|------|
| 25 | fn | `eval_math_content` |
| 45 | fn | `eval_math_expr` |

### `src/rules/eval/modules.rs` (2)

| Linha | Tipo | Nome |
|------:|------|------|
| 27 | fn | `eval_module_import` |
| 37 | fn | `eval_module_include` |

### `src/rules/eval/rules.rs` (2)

| Linha | Tipo | Nome |
|------:|------|------|
| 165 | fn | `eval_set_rule` |
| 278 | fn | `eval_show_rule` |

### `src/rules/layout/placement.rs` (2)

| Linha | Tipo | Nome |
|------:|------|------|
| 21 | fn | `layout_align` |
| 109 | fn | `layout_place` |

### `src/rules/layout/references.rs` (2)

| Linha | Tipo | Nome |
|------:|------|------|
| 16 | fn | `layout_labelled` |
| 33 | fn | `layout_ref` |

### `src/rules/lexer/code.rs` (2)

| Linha | Tipo | Nome |
|------:|------|------|
| 19 | fn | `code` |
| 224 | fn | `string` |

### `src/rules/stdlib/calc.rs` (2)

| Linha | Tipo | Nome |
|------:|------|------|
| 176 | fn | `coerce_to_f64` |
| 187 | fn | `guard_float` |

### `src/rules/stdlib/foundations.rs` (2)

| Linha | Tipo | Nome |
|------:|------|------|
| 135 | fn | `format_float` |
| 141 | fn | `format_length` |

### `src/rules/stdlib/mod.rs` (2)

| Linha | Tipo | Nome |
|------:|------|------|
| 55 | fn | `err` |
| 64 | fn | `expect_no_named` |

### `src/rules/stdlib/shapes.rs` (2)

| Linha | Tipo | Nome |
|------:|------|------|
| 26 | fn | `parse_color` |
| 207 | fn | `extract_coordinate` |

### `src/rules/layout/equation.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 21 | fn | `layout_equation` |

### `src/rules/layout/figure.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 17 | fn | `layout_figure` |

### `src/rules/layout/grid.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 22 | fn | `layout_grid` |

### `src/rules/layout/outline.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 22 | fn | `layout_outline` |

### `src/rules/lexer/math.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 23 | fn | `math` |

### `src/rules/math/layout/assembly.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 17 | fn | `layout_assembly` |

### `src/rules/math/layout/attach.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 21 | fn | `layout_attach` |

### `src/rules/math/layout/cases.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 20 | fn | `layout_cases` |

### `src/rules/math/layout/delimited.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 19 | fn | `layout_delimited` |

### `src/rules/math/layout/frac.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 20 | fn | `layout_frac` |

### `src/rules/math/layout/matrix.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 20 | fn | `layout_matrix` |

### `src/rules/math/layout/root.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 19 | fn | `layout_root` |

### `src/rules/math/layout/stretchy.rs` (1)

| Linha | Tipo | Nome |
|------:|------|------|
| 16 | fn | `layout_stretchy_delimiter` |

