# Preliminary V19/V20 note review

Evidence-only draft. Source and L0 remain frozen. Exact full messages, multiplicities, lines and receipt hashes: `lint-notes-review.json`.

Within the manifested 50 source/owner pairs, exact source/rule/message multisets change from 491 notes (361 V19, 130 V20) to 561 (404 V19, 157 V20): 487 unchanged, 4 removed and 74 added. Lines are ignored for matching; no diagnostic is filtered from the SARIF. These counts bind only to the frozen preliminary 2026-10-19T23:40:28–23:40:34Z snapshot and its recorded inputs/HEAD; they do not certify later sources.

## Every removal accounted for

- `content_methods.rs:239` V19 and V20: the old 98-alternative fallback becomes a 97-alternative fallback, retaining both notes at preliminary line249. Figure moved into explicit body/caption/kind/supplement field dispatch, which adds its own V19.
- `selector_matching.rs:449` V19: the old 47-alternative false group becomes 32, retaining V19 at preliminary line516. HtmlElem, HtmlFrame, Title, ListItem, EnumItem, FigureCaption, Curve, Flush, Divider, TermItem, Document, Asset, PdfAttach, PdfArtifact and FootnoteEntry now have concrete native-constructor matching. No wildcard replaced these cases.
- `layout/figure.rs:30` V20: `Some(Value::Str(s))` is now `Some(Value::Str(pattern))`, still V20 at preliminary line27. Typed numbering/prefix and synthetic compatibility branches add visible notes.

## Additions reviewed

| Source responsibility | Added occurrences | Assessment |
|---|---:|---|
| `01_core/src/compiler/eval/bindings/field_access/content_methods.rs` | 3 | Figure field dispatch extracted from the old grouped fallback; retained fallback is 97 instead of 98 alternatives. |
| `01_core/src/compiler/eval/observation/elements.rs` | 4 | Different FigureSupplement variants remain explicit Different observations; grouped mismatches are not evidence of testing every pair. |
| `01_core/src/compiler/eval/operators/equality.rs` | 5 | Nominal Func/String separation and recursive Figure-selector classification; grouped selectors retain their explicit owning recursion. |
| `01_core/src/compiler/eval/selector_matching.rs` | 1 | Fifteen Content variants moved from the false group into concrete native-constructor matching; remaining 32 alternatives still emit V19. |
| `01_core/src/compiler/introspect.rs` | 3 | Nested Option/FigureKind assertion in regression fixtures, not a new production default. |
| `01_core/src/compiler/introspect/figure_demands.rs` | 3 | Exhaustive actual-view collector groups structurally shared children and explicit opaque/terminal variants; runtime does not certify every grouped variant. |
| `01_core/src/compiler/introspect/labelled.rs` | 3 | Explicit legacy/synthetic FigureKind Name versus Element-or-absent projection; no productive typed counter string shadow. |
| `01_core/src/compiler/introspect/materialize_time.rs` | 3 | Content-only FigureSupplement mapping; Auto/None/Func/absent remain opaque and callbacks are not executed by the map. |
| `01_core/src/compiler/layout/figure.rs` | 7 | Numbering style gates, rich Content prefix absence and explicit unresolved synthetic FigureKind boundary; deep nominal patterns remain visible as notes. |
| `01_core/src/compiler/layout/references.rs` | 1 | Rich reference supplement empty/absent compose the same number-only result; no neutral missing-history fallback introduced. |
| `01_core/src/compiler/layout/tests.rs` | 2 | Nested Option/FigureKind assertion in regression fixtures, not a new production default. |
| `01_core/src/compiler/realize/figure_kind.rs` | 7 | Closed semantic search catalogue and explicit Type/Value exclusions, shared child forms and Grid header/footer ordering; compile exhaustiveness is not runtime alternative coverage. |
| `01_core/src/compiler/stdlib/figure_image.rs` | 11 | Explicit constructor Value/FigureKind/FigureSupplement validation groups, preserving actual error paths and Function/Content distinctions. |
| `01_core/src/entities/content/traversal.rs` | 3 | FigureSupplement Content is mapped structurally; Auto/None/Func/absent are not interpreted or executed. |
| `01_core/src/entities/elements/figure.rs` | 16 | Nominal FuncRepr cross-kind false cases, pointer-kind hashing, Content-only maps and typed field projections; enum identity depth is deliberate, not suppressed. |
| `02_shell/src/cli.rs` | 2 | Query serialization recognizes both Func and String Figure kind language values without collapsing their identity. |

## Runtime limits and remaining gates

V19 explicitly warns that entering a shared arm does not prove every alternative. This review does not claim full runtime coverage of every Content, Value, Type, FuncRepr or supplement mismatch alternative. Existing fixed language cases and real-enum extension probes provide complementary evidence: structural exhaustiveness probes cannot substitute for per-alternative runtime behavior. V20 identifies nested nominal Option/FigureKind/FigureSupplement patterns; they remain visible and are not suppressed or rewritten merely to lower a note count.

No hidden diagnostic loss is evidenced by the four removals. This is a reconciliation of the preliminary diagnostic stream, not final validation. 13 manifested source files differ from the preliminary input hashes at review time; rerun full lint after source stabilization and resealing. Preliminary V5 is unsealed lineage and V16=19 is the preliminary remaining global finding count; neither is waived by this note review.
