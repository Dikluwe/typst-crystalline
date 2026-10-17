# Pendências após espaços de eval/HTML

Proveniência: final-state.json.gz, HEAD/working tree/UTC/status/diffstat/hashes. O gate global continua bloqueado pelos 141 V16 abaixo; remaining-v16.json conserva locais e mensagens completos. A ordenação por quantidade é inventário, não autorização para mudar contratos.

| Consumer | V16 |
|---|---:|
| `01_core/src/compiler/stdlib/structural/table_grid.rs` | 5 |
| `01_core/src/compiler/eval/bindings/field_access/content_methods.rs` | 4 |
| `01_core/src/compiler/realize/table_cells.rs` | 4 |
| `01_core/src/compiler/eval/selector_matching.rs` | 3 |
| `01_core/src/compiler/layout/grid/tracks.rs` | 3 |
| `01_core/src/compiler/layout/paragraph.rs` | 3 |
| `01_core/src/compiler/stdlib/calc/power.rs` | 3 |
| `01_core/src/entities/ast/expr.rs` | 3 |
| `02_shell/src/cli.rs` | 3 |
| `03_infra/src/pipeline/paged.rs` | 3 |
| `03_infra/src/query_helpers.rs` | 3 |
| `01_core/src/compiler/eval/call_dispatch/type_fields.rs` | 2 |
| `01_core/src/compiler/layout/grid/measurement_cache.rs` | 2 |
| `01_core/src/compiler/layout/mod.rs` | 2 |
| `01_core/src/compiler/layout/text/style.rs` | 2 |
| `01_core/src/compiler/realize/bare_math.rs` | 2 |
| `01_core/src/compiler/realize/mod.rs` | 2 |
| `01_core/src/compiler/stdlib/collections.rs` | 2 |
| `01_core/src/compiler/stdlib/figure_image.rs` | 2 |
| `01_core/src/compiler/stdlib/html/attr_cast.rs` | 2 |
| `01_core/src/compiler/stdlib/layout/casting.rs` | 2 |
| `01_core/src/entities/content/fields.rs` | 2 |
| `01_core/src/entities/operators.rs` | 2 |
| `03_infra/src/pipeline/context_stabilization.rs` | 2 |
| `03_infra/src/pipeline.rs` | 2 |
| `01_core/src/compiler/eval/bindings/field_access/fields.rs` | 1 |
| `01_core/src/compiler/eval/bindings/value_methods/element_where.rs` | 1 |
| `01_core/src/compiler/eval/bindings/value_methods/selector.rs` | 1 |
| `01_core/src/compiler/eval/call_dispatch/call_spans.rs` | 1 |
| `01_core/src/compiler/eval/call_dispatch/content_methods.rs` | 1 |
| `01_core/src/compiler/eval/call_dispatch/instance_methods.rs` | 1 |
| `01_core/src/compiler/eval/expression_dispatch.rs` | 1 |
| `01_core/src/compiler/eval/markup_stream.rs` | 1 |
| `01_core/src/compiler/eval/modules.rs` | 1 |
| `01_core/src/compiler/eval/operators/arithmetic.rs` | 1 |
| `01_core/src/compiler/eval/operators/equality.rs` | 1 |
| `01_core/src/compiler/eval/p1426_tests.rs` | 1 |
| `01_core/src/compiler/eval/p1427_tests.rs` | 1 |
| `01_core/src/compiler/eval/p1430_tests.rs` | 1 |
| `01_core/src/compiler/eval/p1431_tests.rs` | 1 |
| `01_core/src/compiler/eval/rules/intercept.rs` | 1 |
| `01_core/src/compiler/eval/rules/realize.rs` | 1 |
| `01_core/src/compiler/eval/rules/set/page.rs` | 1 |
| `01_core/src/compiler/eval/rules/set/text.rs` | 1 |
| `01_core/src/compiler/eval/rules/set.rs` | 1 |
| `01_core/src/compiler/eval/rules.rs` | 1 |
| `01_core/src/compiler/introspect/bibliography.rs` | 1 |
| `01_core/src/compiler/introspect/labelled.rs` | 1 |
| `01_core/src/compiler/introspect/populate.rs` | 1 |
| `01_core/src/compiler/layout/convergence.rs` | 1 |
| `01_core/src/compiler/layout/grid/borders.rs` | 1 |
| `01_core/src/compiler/layout/grid/cell_body.rs` | 1 |
| `01_core/src/compiler/layout/grid/row_groups.rs` | 1 |
| `01_core/src/compiler/layout/grid_placement.rs` | 1 |
| `01_core/src/compiler/layout/helpers.rs` | 1 |
| `01_core/src/compiler/layout/image.rs` | 1 |
| `01_core/src/compiler/layout/line_commit.rs` | 1 |
| `01_core/src/compiler/layout/measurement.rs` | 1 |
| `01_core/src/compiler/layout/par_line_callbacks.rs` | 1 |
| `01_core/src/compiler/layout/place.rs` | 1 |
| `01_core/src/compiler/layout/references.rs` | 1 |
| `01_core/src/compiler/parse/math.rs` | 1 |
| `01_core/src/compiler/stdlib/calc/abs.rs` | 1 |
| `01_core/src/compiler/stdlib/collections/validation.rs` | 1 |
| `01_core/src/compiler/stdlib/counter/display.rs` | 1 |
| `01_core/src/compiler/stdlib/foundations/selector.rs` | 1 |
| `01_core/src/compiler/stdlib/layout/grid.rs` | 1 |
| `01_core/src/compiler/stdlib/layout/place.rs` | 1 |
| `01_core/src/compiler/stdlib/layout/stroke.rs` | 1 |
| `01_core/src/compiler/stdlib/layout.rs` | 1 |
| `01_core/src/compiler/stdlib/loading/encoding.rs` | 1 |
| `01_core/src/compiler/stdlib/loading/yaml_encode.rs` | 1 |
| `01_core/src/compiler/stdlib/shapes/ellipse.rs` | 1 |
| `01_core/src/compiler/stdlib/shapes/line.rs` | 1 |
| `01_core/src/compiler/stdlib/structural/bibliography.rs` | 1 |
| `01_core/src/compiler/stdlib/structural/outline.rs` | 1 |
| `01_core/src/compiler/stdlib/structural/p1429_tests.rs` | 1 |
| `01_core/src/compiler/stdlib/structural/p1430_tests.rs` | 1 |
| `01_core/src/entities/content/emptiness.rs` | 1 |
| `01_core/src/entities/content/equality.rs` | 1 |
| `01_core/src/entities/content/morphology.rs` | 1 |
| `01_core/src/entities/content/rust_hash.rs` | 1 |
| `01_core/src/entities/content/source.rs` | 1 |
| `01_core/src/entities/content/tests/traversal.rs` | 1 |
| `01_core/src/entities/content/traversal.rs` | 1 |
| `01_core/src/entities/elements/table_cell.rs` | 1 |
| `01_core/src/entities/font_variations.rs` | 1 |
| `01_core/src/entities/gradient/radial.rs` | 1 |
| `01_core/src/entities/syntax_node/representation.rs` | 1 |
| `03_infra/src/export/builder/metadata.rs` | 1 |
| `03_infra/src/export/gradients/adaptive.rs` | 1 |
| `03_infra/src/export/html/document.rs` | 1 |
| `03_infra/src/export/html/encoding.rs` | 1 |
| `03_infra/src/font_metrics/math.rs` | 1 |
| `03_infra/src/fonts/coverage.rs` | 1 |
| `03_infra/src/integration_tests.rs` | 1 |
| `03_infra/src/pipeline/bundle.rs` | 1 |
| `03_infra/src/pipeline/html.rs` | 1 |
| `03_infra/src/shaper/shaping.rs` | 1 |
| `03_infra/tests/p1295_watch_contract.rs` | 1 |
| `03_infra/tests/p1297_watch_capability_contract.rs` | 1 |

## Obrigações que permanecem abertas

- **math-stretch-composite**: completar anexos/corpos compostos e eixo nativo do contrato vigente; o lote anterior só restaurou Source.
- **matching-local-cjk** e **matching-identical-rule-precedence**: revalidar as obrigações ainda declaradas em `00_nucleo/prompts/compiler/eval/rules/textual/matching.md` (normalização/show local CJK e override de regras idênticas). Este lote não mede seu fechamento; o inventário não as declara bugs novamente comprovados.
- Auditar os próximos consumers de V16, medir, atualizar L0 antes de código e preservar grupos V19 de decisão compartilhada legítima. Notas V19/V20 requerem classificação própria; não silenciar globalmente.

Manter 12 GiB, swap zero e um job Cargo. Alterações públicas exigem o gate pertinente. Sem commit/push/merge/release do Typst nesta etapa.
