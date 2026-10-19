# V16 case index

All105 findings remain open. Every row links to its full evidence and proposed repair/test/gate record; search that report for the ID. Source lines and hashes refer to the clean baseline recorded in baseline-state.json. The JSON ledger retains exact owner paths and digests.

| ID | Batch | Function | Source | Classification / report |
|---|---|---|---|---|
| V16-001 | E | `content_set_fields` | `01_core/src/compiler/eval/bindings/field_access/content_methods.rs:307` | [mixed structural obligation and reflection coverage investigation](eval-realize-analysis.md) |
| V16-002 | E | `equation_descendant` | `01_core/src/compiler/eval/bindings/field_access/content_methods.rs:325` | [candidate semantic carrier-transparency gap with scoped-opacity ambiguity; structural obligation confirmed](eval-realize-analysis.md) |
| V16-003 | E | `content_elem_func` | `01_core/src/compiler/eval/bindings/field_access/content_methods.rs:505` | [mixed identity coverage debt and structural obligation](eval-realize-analysis.md) |
| V16-004 | E | `content_method_with_fields` | `01_core/src/compiler/eval/bindings/field_access/content_methods.rs:607` | [structural domain obligation](eval-realize-analysis.md) |
| V16-005 | C | `is_matrix_or_cell_content` | `01_core/src/compiler/eval/bindings/field_access/fields.rs:767` | [structural domain obligation](eval-realize-analysis.md) |
| V16-006 | C | `validate_element_locatability` | `01_core/src/compiler/eval/bindings/value_methods/element_where.rs:640` | [structural domain obligation](eval-realize-analysis.md) |
| V16-007 | C | `value_to_query_selector` | `01_core/src/compiler/eval/bindings/value_methods/selector.rs:38` | [mixed semantic admission investigation and structural obligation](eval-realize-analysis.md) |
| V16-008 | C | `P1293HtmlCallSpans::anchor` | `01_core/src/compiler/eval/call_dispatch/call_spans.rs:321` | [structural domain obligation](eval-realize-analysis.md) |
| V16-009 | C | `try_content_methods` | `01_core/src/compiler/eval/call_dispatch/content_methods.rs:85` | [structural domain obligation](eval-realize-analysis.md) |
| V16-010 | C | `try_instance_method` | `01_core/src/compiler/eval/call_dispatch/instance_methods.rs:238` | [structural domain obligation](eval-realize-analysis.md) |
| V16-011 | C | `content_has_state_or_counter` | `01_core/src/compiler/eval/expression_dispatch.rs:793` | [mixed warning-transparency investigation and structural obligation](eval-realize-analysis.md) |
| V16-012 | C | `value_to_display_content` | `01_core/src/compiler/eval/markup_stream.rs:556` | [mixed presentation debt and structural obligation](eval-realize-analysis.md) |
| V16-013 | B | `find_cases` | `01_core/src/compiler/eval/math_cases_evaluation_tests.rs:74` | [test-only closed fixture traversal](eval-realize-analysis.md) |
| V16-014 | B | `find_frac` | `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:72` | [test-only closed fixture traversal](eval-realize-analysis.md) |
| V16-015 | B | `find_mat` | `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:69` | [test-only closed fixture traversal](eval-realize-analysis.md) |
| V16-016 | B | `find_root` | `01_core/src/compiler/eval/math_root_evaluation_tests.rs:71` | [test-only closed fixture traversal](eval-realize-analysis.md) |
| V16-017 | C | `eval_module_include` | `01_core/src/compiler/eval/modules.rs:296` | [structural domain obligation](eval-realize-analysis.md) |
| V16-018 | C | `apply_binary` | `01_core/src/compiler/eval/operators/arithmetic.rs:50` | [structural domain obligation](eval-realize-analysis.md) |
| V16-019 | C | `selectors_eq` | `01_core/src/compiler/eval/operators/equality.rs:241` | [closed pair relation with native-ABI coverage investigation](eval-realize-analysis.md) |
| V16-020 | C | `intercept_content` | `01_core/src/compiler/eval/rules/intercept.rs:86` | [structural domain obligation](eval-realize-analysis.md) |
| V16-021 | C | `is_paragraph_node` | `01_core/src/compiler/eval/rules/realize.rs:234` | [stage-specific classifier with carrier-scope uncertainty](eval-realize-analysis.md) |
| V16-022 | C | `eval_set_page (bleed property)` | `01_core/src/compiler/eval/rules/set/page.rs:439` | [structural domain obligation](eval-realize-analysis.md) |
| V16-023 | C | `eval_set_text (font warning extraction)` | `01_core/src/compiler/eval/rules/set/text.rs:333` | [structural domain obligation](eval-realize-analysis.md) |
| V16-024 | C | `eval_set_rule::pure_path` | `01_core/src/compiler/eval/rules/set.rs:41` | [structural domain obligation](eval-realize-analysis.md) |
| V16-025 | B | `tests::collect_custom_into` | `01_core/src/compiler/eval/rules.rs:336` | [test-only closed fixture traversal](eval-realize-analysis.md) |
| V16-026 | C | `query_selector_to_show::kind_to_node` | `01_core/src/compiler/eval/selector_matching.rs:41` | [closed legacy admission with semantic scope investigation](eval-realize-analysis.md) |
| V16-027 | C | `native_element_matches` | `01_core/src/compiler/eval/selector_matching.rs:362` | [mixed native selection coverage debt and structural obligation](eval-realize-analysis.md) |
| V16-028 | C | `native_element_field` | `01_core/src/compiler/eval/selector_matching.rs:380` | [structural domain obligation](eval-realize-analysis.md) |
| V16-029 | D | `cell_effective_stroke` | `01_core/src/compiler/layout/grid/borders.rs:49` | [negative_projection_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-030 | D | `Layouter::emit_grid_cell_body` | `01_core/src/compiler/layout/grid/cell_body.rs:103` | [negative_projection_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-031 | D | `simple_placement` | `01_core/src/compiler/layout/grid/measurement_cache.rs:59` | [positive_admission_predicate_needs_nominal_boundary](layout-stdlib-analysis.md) |
| V16-032 | D | `token` | `01_core/src/compiler/layout/grid/measurement_cache.rs:100` | [safety_refusal_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-033 | D | `Layouter::resolve_grid_columns` | `01_core/src/compiler/layout/grid/tracks.rs:87` | [negative_projection_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-034 | D | `Layouter::resolve_grid_rows` | `01_core/src/compiler/layout/grid/tracks.rs:298` | [negative_projection_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-035 | D | `measurement_visual` | `01_core/src/compiler/layout/grid/tracks.rs:469` | [traversal_continue_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-036 | D | `place / fields closure` | `01_core/src/compiler/layout/grid_placement.rs:106` | [policy_guarded_negative_projection](layout-stdlib-analysis.md) |
| V16-037 | D | `measure_content` | `01_core/src/compiler/layout/helpers.rs:481` | [documented_approximation_requires_bounded_nominal_policy](layout-stdlib-analysis.md) |
| V16-038 | D | `extract_pt` | `01_core/src/compiler/layout/image.rs:133` | [documented_dimension_projection_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-039 | D | `flush_line_with_outer_singleton` | `01_core/src/compiler/layout/line_commit.rs:720` | [guarded_optional_dispatch_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-040 | D | `Layouter::measure_content_constrained` | `01_core/src/compiler/layout/measurement.rs:426` | [documented_approximation_requires_bounded_nominal_policy](layout-stdlib-analysis.md) |
| V16-041 | D | `Layouter::layout_content_once` | `01_core/src/compiler/layout/mod.rs:1256` | [forwarding_partition_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-042 | D | `Layouter::layout_content_body` | `01_core/src/compiler/layout/mod.rs:1273` | [side_observer_negative_partition](layout-stdlib-analysis.md) |
| V16-043 | D | `Layouter::apply_pending_par_line_fixups` | `01_core/src/compiler/layout/par_line_callbacks.rs:138` | [finite_product_negative_partition](layout-stdlib-analysis.md) |
| V16-044 | D | `Layouter::translate_deferred_float` | `01_core/src/compiler/layout/place.rs:309` | [finite_optional_negative_partition](layout-stdlib-analysis.md) |
| V16-045 | D | `default_supplement_for_key` | `01_core/src/compiler/layout/references.rs:369` | [guarded_semantic_default_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-046 | D | `effective_style` | `01_core/src/compiler/layout/text/style.rs:89` | [schema_negative_projection_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-047 | D | `effective_style` | `01_core/src/compiler/layout/text/style.rs:131` | [schema_negative_projection_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-048 | A | `Parser::math_op` | `01_core/src/compiler/parse/math.rs:207` | [structural domain obligation](eval-realize-analysis.md) |
| V16-049 | G | `plain_without_bare_math` | `01_core/src/compiler/realize/bare_math.rs:74` | [structural domain obligation](eval-realize-analysis.md) |
| V16-050 | G | `prepare (map_content_pre callback)` | `01_core/src/compiler/realize/bare_math.rs:131` | [structural domain obligation](eval-realize-analysis.md) |
| V16-051 | G | `normalize_insertions (children any predicate)` | `01_core/src/compiler/realize/mod.rs:93` | [structural domain obligation](eval-realize-analysis.md) |
| V16-052 | G | `observe_occurrences (map_content callback)` | `01_core/src/compiler/realize/mod.rs:136` | [structural domain obligation](eval-realize-analysis.md) |
| V16-053 | F | `type_as_callable` | `01_core/src/compiler/stdlib/collections/validation.rs:100` | [legacy_constructor_subset_needs_nominal_audit](layout-stdlib-analysis.md) |
| V16-054 | F | `counter::display::render` | `01_core/src/compiler/stdlib/counter/display.rs:132` | [nested_key_policy_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-055 | F | `infer_kind_from_body` | `01_core/src/compiler/stdlib/figure_image.rs:69` | [explicit_open_semantic_scope_not_lint_authority](layout-stdlib-analysis.md) |
| V16-056 | F | `parse_selector_arg` | `01_core/src/compiler/stdlib/foundations/selector.rs:162` | [slice_arity_negative_partition](layout-stdlib-analysis.md) |
| V16-057 | F | `cast_attr` | `01_core/src/compiler/stdlib/html/attr_cast.rs:108` | [delegated_validator_union_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-058 | F | `cast_attr` | `01_core/src/compiler/stdlib/html/attr_cast.rs:124` | [delegated_validator_union_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-059 | F | `parse_track_sizing` | `01_core/src/compiler/stdlib/layout/grid.rs:18` | [type_projection_with_probe_dependent_caller_loss](layout-stdlib-analysis.md) |
| V16-060 | F | `native_place::extract_pt` | `01_core/src/compiler/stdlib/layout/place.rs:38` | [possible_parity_debt_unvalidated_zero_coercion](layout-stdlib-analysis.md) |
| V16-061 | F | `native_stroke::dash_item` | `01_core/src/compiler/stdlib/layout/stroke.rs:151` | [guarded_validation_refusal_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-062 | F | `extract_alignment` | `01_core/src/compiler/stdlib/layout.rs:124` | [search_projection_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-063 | F | `textual_content_fields` | `01_core/src/compiler/stdlib/loading/encoding.rs:110` | [delegated_projection_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-064 | F | `write_yaml_value` | `01_core/src/compiler/stdlib/loading/yaml_encode.rs:54` | [two_stage_serialization_partition_missing_nominal_coverage](layout-stdlib-analysis.md) |
| V16-065 | F | `native_circle::extract_pt` | `01_core/src/compiler/stdlib/shapes/ellipse.rs:89` | [possible_parity_debt_unvalidated_zero_coercion](layout-stdlib-analysis.md) |
| V16-066 | F | `native_line::extract_pt` | `01_core/src/compiler/stdlib/shapes/line.rs:35` | [possible_parity_debt_unvalidated_zero_coercion](layout-stdlib-analysis.md) |
| V16-067 | F | `native_bibliography / locale closure` | `01_core/src/compiler/stdlib/structural/bibliography.rs:334` | [legacy_optional_cast_probe_dependent](layout-stdlib-analysis.md) |
| V16-068 | F | `find_accent` | `01_core/src/compiler/stdlib/structural/math_accent_tests.rs:343` | [test_finder_negative_partition_needs_scope_proof](layout-stdlib-analysis.md) |
| V16-069 | F | `find_cases` | `01_core/src/compiler/stdlib/structural/math_cases_tests.rs:327` | [test_finder_negative_partition_needs_scope_proof](layout-stdlib-analysis.md) |
| V16-070 | F | `native_outline_entry_indented` | `01_core/src/compiler/stdlib/structural/outline.rs:434` | [private_schema_negative_projection](layout-stdlib-analysis.md) |
| V16-071 | A | `ast::UnOp::from_kind` | `01_core/src/entities/ast/expr.rs:602` | [valid negative classification; missing evolution obligation](entities-boundaries-analysis.md) |
| V16-072 | A | `ast::BinOp::from_kind` | `01_core/src/entities/ast/expr.rs:692` | [valid negative classification; missing evolution obligation](entities-boundaries-analysis.md) |
| V16-073 | A | `Pattern::bindings` | `01_core/src/entities/ast/expr.rs:909` | [valid binding projection with nested enum coverage hole](entities-boundaries-analysis.md) |
| V16-074 | G | `Content::eq (Rust PartialEq)` | `01_core/src/entities/content/equality.rs:209` | [mixed: valid heterogeneous inequality plus unresolved same-variant omissions](entities-boundaries-analysis.md) |
| V16-075 | B | `map_content_bottom_up_pai_ve_filhos_transformados callback` | `01_core/src/entities/content/tests/traversal.rs:89` | [test-only valid no-replacement classification; coverage must stay meaningful](entities-boundaries-analysis.md) |
| V16-076 | B | `container_reuse_tests::same_storage` | `01_core/src/entities/content/traversal.rs:2286` | [test-only finite storage-qualification predicate, not universal identity](entities-boundaries-analysis.md) |
| V16-077 | D | `TableCellProperties::style_inset` | `01_core/src/entities/elements/table_cell.rs:184` | [valid bounded cast rejection; no rich-inset expansion](entities-boundaries-analysis.md) |
| V16-078 | A | `FontVariations::from_validated_dict` | `01_core/src/entities/font_variations.rs:98` | [explicitly specified lenient omission, missing nominal coverage](entities-boundaries-analysis.md) |
| V16-079 | A | `Radial::sample_precise` | `01_core/src/entities/gradient/radial.rs:167` | [valid delegation for supported non-specialized spaces](entities-boundaries-analysis.md) |
| V16-080 | A | `operators::UnOp::from_kind` | `01_core/src/entities/operators.rs:27` | [valid negative classification; missing evolution obligation](entities-boundaries-analysis.md) |
| V16-081 | A | `operators::BinOp::from_kind` | `01_core/src/entities/operators.rs:95` | [valid negative classification; missing evolution obligation](entities-boundaries-analysis.md) |
| V16-082 | A | `SyntaxNode::spanless_eq` | `01_core/src/entities/syntax_node/representation.rs:196` | [valid unlike-representation inequality](entities-boundaries-analysis.md) |
| V16-083 | E | `content_to_semantic initial carrier dispatch` | `02_shell/src/cli.rs:1161` | [valid wrapper dispatch versus continue-to-projection complement](entities-boundaries-analysis.md) |
| V16-084 | E | `content_to_semantic legacy field fallback` | `02_shell/src/cli.rs:1412` | [valid bounded legacy projection; incomplete fields explicitly scoped](entities-boundaries-analysis.md) |
| V16-085 | E | `content_semantic_func_name` | `02_shell/src/cli.rs:1445` | [valid canonical-name delegation after explicit overrides](entities-boundaries-analysis.md) |
| V16-086 | A | `collect_top inner UA1 SemanticKind dispatch` | `03_infra/src/export/builder/metadata.rs:789` | [valid boundary interception; complement continues tagging](entities-boundaries-analysis.md) |
| V16-087 | A | `sample_gradient_precise_for_svg hue-index selection` | `03_infra/src/export/gradients/adaptive.rs:275` | [valid no-hue decision with dominating five-space guard](entities-boundaries-analysis.md) |
| V16-088 | G | `export_html_with_frame_resolver visual map callback` | `03_infra/src/export/html/document.rs:37` | [valid unchanged complement plus carrier-root semantic candidate](entities-boundaries-analysis.md) |
| V16-089 | G | `visual_content` | `03_infra/src/export/html/encoding.rs:19` | [valid terminal return after local carrier peeling](entities-boundaries-analysis.md) |
| V16-090 | A | `collect cmap native-format selection` | `03_infra/src/fonts/coverage.rs:90` | [valid native-unsupported versus callback-supported dispatch](entities-boundaries-analysis.md) |
| V16-091 | B | `sec_27 math integration test::equations observer` | `03_infra/src/integration_tests.rs:4888` | [test-only observer completeness candidate, not productive math defect](entities-boundaries-analysis.md) |
| V16-092 | G | `bundle_html_semantic_content` | `03_infra/src/pipeline/bundle.rs:379` | [valid identity for nonconverted elements; container completeness requires focused controls](entities-boundaries-analysis.md) |
| V16-093 | G | `ordered_blocks` | `03_infra/src/pipeline/context_stabilization.rs:97` | [valid producer-recognition complement; apparent Located duplicate refuted](entities-boundaries-analysis.md) |
| V16-094 | G | `Session::new legacy fallback callback` | `03_infra/src/pipeline/context_stabilization.rs:182` | [valid producer-recognition complement; apparent Located duplicate refuted](entities-boundaries-analysis.md) |
| V16-095 | G | `collect_html_frames map callback` | `03_infra/src/pipeline/html.rs:186` | [nonframe no-op valid; Located frame observer candidate](entities-boundaries-analysis.md) |
| V16-096 | G | `page_independent_style text.font value classification` | `03_infra/src/pipeline/paged.rs:20` | [valid fail-closed optimization proof](entities-boundaries-analysis.md) |
| V16-097 | G | `initial_layout_independent` | `03_infra/src/pipeline/paged.rs:123` | [valid conservative negative dependency classification](entities-boundaries-analysis.md) |
| V16-098 | G | `realize_outlines_for_pipeline same_carrier` | `03_infra/src/pipeline/paged.rs:219` | [valid limited immutable-root continuity proof](entities-boundaries-analysis.md) |
| V16-099 | G | `collect_context_blocks_into L1 fallback visitor` | `03_infra/src/pipeline.rs:488` | [valid no-action callback complement after exhaustive delegated traversal](entities-boundaries-analysis.md) |
| V16-100 | G | `substitute_context_blocks map callback` | `03_infra/src/pipeline.rs:529` | [valid unchanged complement; marker preservation critical](entities-boundaries-analysis.md) |
| V16-101 | G | `count_element_in_content` | `03_infra/src/query_helpers.rs:242` | [valid public zero projection for non-document kinds, guarded in summary](entities-boundaries-analysis.md) |
| V16-102 | G | `count_list_groups` | `03_infra/src/query_helpers.rs:256` | [documented approximate projection with reproducible empty/singleton/wrapper limitations](entities-boundaries-analysis.md) |
| V16-103 | G | `count_enum_groups` | `03_infra/src/query_helpers.rs:270` | [documented approximate projection with reproducible empty/singleton/wrapper limitations](entities-boundaries-analysis.md) |
| V16-104 | A | `armed watch signature_mentions_id` | `03_infra/tests/armed_watch_contract.rs:255` | [test-only valid JSON leaf negative classification](entities-boundaries-analysis.md) |
| V16-105 | A | `snapshot signature_mentions_id` | `03_infra/tests/watch_snapshot_contract.rs:189` | [test-only valid JSON leaf negative classification](entities-boundaries-analysis.md) |
