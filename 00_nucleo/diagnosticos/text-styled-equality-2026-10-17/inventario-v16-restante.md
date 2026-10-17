# V16 após TS1 e correção de igualdade

124 erros em 95 consumers. Proveniência: final-state.json.gz e SARIF integral.

| Consumer | Casos | Linhas |
|---|---:|---|
| `./01_core/src/compiler/eval/bindings/field_access/content_methods.rs` | 4 | 258, 276, 339, 441 |
| `./01_core/src/compiler/eval/bindings/field_access/fields.rs` | 1 | 749 |
| `./01_core/src/compiler/eval/bindings/value_methods/element_where.rs` | 1 | 533 |
| `./01_core/src/compiler/eval/bindings/value_methods/selector.rs` | 1 | 38 |
| `./01_core/src/compiler/eval/call_dispatch/call_spans.rs` | 1 | 321 |
| `./01_core/src/compiler/eval/call_dispatch/content_methods.rs` | 1 | 66 |
| `./01_core/src/compiler/eval/call_dispatch/instance_methods.rs` | 1 | 237 |
| `./01_core/src/compiler/eval/call_dispatch/type_fields.rs` | 2 | 42, 122 |
| `./01_core/src/compiler/eval/expression_dispatch.rs` | 1 | 760 |
| `./01_core/src/compiler/eval/markup_stream.rs` | 1 | 319 |
| `./01_core/src/compiler/eval/modules.rs` | 1 | 264 |
| `./01_core/src/compiler/eval/operators/arithmetic.rs` | 1 | 49 |
| `./01_core/src/compiler/eval/operators/equality.rs` | 1 | 241 |
| `./01_core/src/compiler/eval/p1426_tests.rs` | 1 | 71 |
| `./01_core/src/compiler/eval/p1427_tests.rs` | 1 | 72 |
| `./01_core/src/compiler/eval/p1430_tests.rs` | 1 | 74 |
| `./01_core/src/compiler/eval/p1431_tests.rs` | 1 | 69 |
| `./01_core/src/compiler/eval/rules.rs` | 1 | 336 |
| `./01_core/src/compiler/eval/rules/intercept.rs` | 1 | 86 |
| `./01_core/src/compiler/eval/rules/realize.rs` | 1 | 100 |
| `./01_core/src/compiler/eval/rules/set.rs` | 1 | 41 |
| `./01_core/src/compiler/eval/rules/set/page.rs` | 1 | 439 |
| `./01_core/src/compiler/eval/rules/set/text.rs` | 1 | 333 |
| `./01_core/src/compiler/eval/selector_matching.rs` | 3 | 41, 336, 354 |
| `./01_core/src/compiler/introspect/bibliography.rs` | 1 | 72 |
| `./01_core/src/compiler/introspect/labelled.rs` | 1 | 80 |
| `./01_core/src/compiler/introspect/populate.rs` | 1 | 44 |
| `./01_core/src/compiler/layout/convergence.rs` | 1 | 244 |
| `./01_core/src/compiler/layout/grid/borders.rs` | 1 | 49 |
| `./01_core/src/compiler/layout/grid/cell_body.rs` | 1 | 103 |
| `./01_core/src/compiler/layout/grid/measurement_cache.rs` | 2 | 59, 100 |
| `./01_core/src/compiler/layout/grid/tracks.rs` | 3 | 87, 298, 469 |
| `./01_core/src/compiler/layout/grid_placement.rs` | 1 | 106 |
| `./01_core/src/compiler/layout/helpers.rs` | 1 | 481 |
| `./01_core/src/compiler/layout/image.rs` | 1 | 133 |
| `./01_core/src/compiler/layout/line_commit.rs` | 1 | 700 |
| `./01_core/src/compiler/layout/measurement.rs` | 1 | 426 |
| `./01_core/src/compiler/layout/mod.rs` | 2 | 1245, 1261 |
| `./01_core/src/compiler/layout/par_line_callbacks.rs` | 1 | 138 |
| `./01_core/src/compiler/layout/place.rs` | 1 | 309 |
| `./01_core/src/compiler/layout/references.rs` | 1 | 369 |
| `./01_core/src/compiler/layout/text/style.rs` | 2 | 89, 131 |
| `./01_core/src/compiler/parse/math.rs` | 1 | 207 |
| `./01_core/src/compiler/realize/bare_math.rs` | 2 | 74, 131 |
| `./01_core/src/compiler/realize/mod.rs` | 2 | 81, 121 |
| `./01_core/src/compiler/stdlib/collections.rs` | 2 | 232, 373 |
| `./01_core/src/compiler/stdlib/collections/validation.rs` | 1 | 100 |
| `./01_core/src/compiler/stdlib/counter/display.rs` | 1 | 132 |
| `./01_core/src/compiler/stdlib/figure_image.rs` | 2 | 44, 104 |
| `./01_core/src/compiler/stdlib/foundations/selector.rs` | 1 | 155 |
| `./01_core/src/compiler/stdlib/html/attr_cast.rs` | 2 | 108, 124 |
| `./01_core/src/compiler/stdlib/layout.rs` | 1 | 124 |
| `./01_core/src/compiler/stdlib/layout/casting.rs` | 2 | 37, 52 |
| `./01_core/src/compiler/stdlib/layout/grid.rs` | 1 | 18 |
| `./01_core/src/compiler/stdlib/layout/place.rs` | 1 | 38 |
| `./01_core/src/compiler/stdlib/layout/stroke.rs` | 1 | 151 |
| `./01_core/src/compiler/stdlib/loading/encoding.rs` | 1 | 109 |
| `./01_core/src/compiler/stdlib/loading/yaml_encode.rs` | 1 | 54 |
| `./01_core/src/compiler/stdlib/shapes/ellipse.rs` | 1 | 89 |
| `./01_core/src/compiler/stdlib/shapes/line.rs` | 1 | 35 |
| `./01_core/src/compiler/stdlib/structural/bibliography.rs` | 1 | 334 |
| `./01_core/src/compiler/stdlib/structural/outline.rs` | 1 | 434 |
| `./01_core/src/compiler/stdlib/structural/p1429_tests.rs` | 1 | 343 |
| `./01_core/src/compiler/stdlib/structural/p1430_tests.rs` | 1 | 327 |
| `./01_core/src/entities/ast/expr.rs` | 3 | 602, 692, 909 |
| `./01_core/src/entities/content/emptiness.rs` | 1 | 125 |
| `./01_core/src/entities/content/equality.rs` | 1 | 209 |
| `./01_core/src/entities/content/fields.rs` | 2 | 13, 120 |
| `./01_core/src/entities/content/morphology.rs` | 1 | 127 |
| `./01_core/src/entities/content/rust_hash.rs` | 1 | 33 |
| `./01_core/src/entities/content/source.rs` | 1 | 23 |
| `./01_core/src/entities/content/tests/traversal.rs` | 1 | 89 |
| `./01_core/src/entities/content/traversal.rs` | 1 | 1852 |
| `./01_core/src/entities/elements/table_cell.rs` | 1 | 184 |
| `./01_core/src/entities/font_variations.rs` | 1 | 98 |
| `./01_core/src/entities/gradient/radial.rs` | 1 | 167 |
| `./01_core/src/entities/operators.rs` | 2 | 27, 95 |
| `./01_core/src/entities/syntax_node/representation.rs` | 1 | 196 |
| `./02_shell/src/cli.rs` | 3 | 1161, 1405, 1437 |
| `./03_infra/src/export/builder/metadata.rs` | 1 | 789 |
| `./03_infra/src/export/gradients/adaptive.rs` | 1 | 275 |
| `./03_infra/src/export/html/document.rs` | 1 | 33 |
| `./03_infra/src/export/html/encoding.rs` | 1 | 19 |
| `./03_infra/src/font_metrics/math.rs` | 1 | 337 |
| `./03_infra/src/fonts/coverage.rs` | 1 | 90 |
| `./03_infra/src/integration_tests.rs` | 1 | 4884 |
| `./03_infra/src/pipeline.rs` | 2 | 487, 528 |
| `./03_infra/src/pipeline/bundle.rs` | 1 | 379 |
| `./03_infra/src/pipeline/context_stabilization.rs` | 2 | 92, 177 |
| `./03_infra/src/pipeline/html.rs` | 1 | 176 |
| `./03_infra/src/pipeline/paged.rs` | 3 | 20, 123, 216 |
| `./03_infra/src/query_helpers.rs` | 3 | 242, 256, 270 |
| `./03_infra/src/shaper/shaping.rs` | 1 | 28 |
| `./03_infra/tests/p1295_watch_contract.rs` | 1 | 189 |
| `./03_infra/tests/p1297_watch_capability_contract.rs` | 1 | 255 |
