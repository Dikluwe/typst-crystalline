# L0 — compiler/eval/selector_matching
Hash do Código: 676840d2

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/selector_matching.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/selector_matching.rs:29` — `fn query_selector_to_show_selector`.
- `01_core/src/compiler/eval/selector_matching.rs:112` — `fn selector_matches`.
- `01_core/src/compiler/eval/selector_matching.rs:203` — `fn native_element_base`.
- `01_core/src/compiler/eval/selector_matching.rs:212` — `fn is_par_rule`.
- `01_core/src/compiler/eval/selector_matching.rs:226` — `fn native_element_matches`.
- `01_core/src/compiler/eval/selector_matching.rs:306` — `fn native_element_field`.
- `01_core/src/compiler/eval/selector_matching.rs:324` — `fn element_selector_matches`.
- `01_core/src/compiler/eval/selector_matching.rs:342` — `fn values_eq_semantic`.
- `01_core/src/compiler/eval/selector_matching.rs:354` — `fn is_node_rule`.
- `01_core/src/compiler/eval/selector_matching.rs:370` — `fn is_text_rule`.
- `01_core/src/compiler/eval/selector_matching.rs:376` — `fn textual_selector_matches`.
- `01_core/src/compiler/eval/selector_matching.rs:389` — `fn textual_match_ranges`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/selector.rs:134-140`.
- `foundations/selector.rs:149-153`.
- `typst-realize/src/lib.rs:1261-1325`.
- `typst-realize/src/lib.rs:1320-1371`.

## Contract

Own conversion from query selectors to show selectors, typed node matching,
node-rule classification and literal/regex splicing. Operate on received
values/content without scope mutation, World reads or layout. Splicing invokes
only its supplied replacement closure for each matched slice; this helper does
not independently resolve or apply a user Func.

Literal/regex splicing passes only each nonempty matched text slice to the recipe,
keeps unmatched prefix/suffix, leftmost order and causal content morphology.
This helper does not concatenate descendants or form realized text groups;
00_nucleo/prompts/compiler/eval/rules.md owns group construction/normalization.
Text/Regex are not node rules, Label has its separate path, and empty And/Or do
not match. NodeKind/DynKind/Where/native Element and existing compositions retain
the closed dispatch and their current semantics.

NativeElement matching recognizes actual typed native element identity without
executing it or consulting a public spelling. Native paragraph detection is a
typed helper for rules, not a generic expansion to all compositions. Legacy
strong/emph matching uses semantic origin, not a baked render style; list sequence
recognition preserves the current uniformity requirement.

Legacy Where preserves values_eq_semantic; native Element filter groups use
recursive language equality, names and ordered values under
00_nucleo/prompts/compiler/eval/operators/equality.md. Use the existing explicit
field projection (including text/raw/lang/block/body), without reverse imports
from entities. Queried-content fields delegate to
00_nucleo/prompts/compiler/eval/bindings/field_access.md and its pinned snapshot
contract: Some is authoritative, None uses its legacy path. Label absence and
explicit None remain distinct; Location is not a linguistic filter field.

OutlineEntry matches its own entry identity, not outline or heading. raw.line
uses its real typed built-in element and current dynamic identity; arbitrary
user-defined elements with a similar name do not inherit it. before/after remain
query-only and do not become show predicates. Preserve match/miss behavior for
all pre-existing selector variants; this helper does not fabricate query data.
