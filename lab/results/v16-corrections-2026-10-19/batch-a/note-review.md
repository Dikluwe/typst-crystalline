# Informational-note review

The full SARIF comparison retains unchanged severity and scan scope. After
normalizing only location shifts, V19 gains 17 occurrences (16 distinct
messages); V20 loses the nested `Normal(Expr::Ident)` pattern. The complete
messages and multiplicity are in `note-content-delta.json`. All messages outside
the eleven selected consumers are exactly unchanged, including locations.

The root reviewed the productive diff; a separate agent reviewed the note delta
without editing sources. Grouped alternatives retain one body for one justified
outcome. They were not split into duplicate bodies or suppressed to change counts.

| New V19 groups | Meaning and preservation controls | Coverage limits |
| --- | --- | --- |
| Five token-negative groups | Unary/binary arity and Pratt stopping; positive mappings, representative negatives, precedence, composed `not in`, trivia and parsed structures. | Negative runtime inputs are representative, not every SyntaxKind. The exact-site evolution proof is a separate gate. |
| One nonbinding Expr group | Only Normal Ident declares a binding; tests retain named/spread/nested order and distinguish uses from declarations. | Not every nonbinding Expr payload is executed. No new binding grammar is admitted. |
| One validated-font group | All 37 current nonnumeric Value categories are omitted with valid neighbors; normalization, collisions, strict errors, hints and spans are checked. | No new numeric/nonfinite admission or external font corpus claim. |
| One radial delegation group | Seven spaces use the unchanged domain interpolator. Tests cover every delegated space, hue/alpha and complete spot identity, plus precision/boundaries. | No new render or bilateral parity claim. |
| One heterogeneous-node group | Six mixed pairs remain unequal; the full ordered 3×3 matrix, spans, errors and structure are exercised. | Syntax representation equality is not Typst Content equality. |
| One PDF continuation group | Nine kinds continue into existing processing. Nested-table, boundary and artifact integration controls remain; an added isolated-root control exercises each continuation at collect_top. | Isolated table roles test an internal walker, not PDF/UA document conformance. |
| Four SVG groups | Five precise spaces, four early delegates, six no-hue alternatives and four unreachable reconstruction cases retain formulas and routing. | Only two no-hue alternatives reach interpolation; the other four are excluded earlier. Unreachable reconstruction cases are not new supported conversions. |
| One cmap callback group | Six non-native formats still select the existing typed callback and rollback path. Existing format0/6/10 membership, native, malformed and termination controls remain. | Valid fixtures for HighByteMappingThroughTable, MixedCoverage and UnicodeVariationSequences are not demonstrated. The L0 retains this explicit gap; no new variation-sequence/corpus certification. |
| Two JSON-leaf groups | Four scalar kinds are false; structured resolved-path IDs and nested object/array recursion remain tested. | These are test-local schema predicates. |

The removed V20 is explained by splitting the old nested pattern into an
exhaustive Pattern match and a direct nested Expr match. Separate sentinel
experiments attribute E0004 to each. No helper hides the decision.

V19 notes are reminders to audit alternatives, not execution counts. Keeping
them visible is intentional; passing a shared body is not proof of executing
every alternative or payload.
