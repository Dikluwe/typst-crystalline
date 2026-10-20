# Independent structural witness review

PASS: all eight selected candidate witnesses are raw E0004 primary spans at the actual frozen match decisions. Baseline selects zero at the old inferred-kind match. No Cargo or productive/L0 edits were performed.

| Decision | Raw primary source span | Token |
|---|---|---|
| fn semantic_children( / match content { | 01_core/src/compiler/realize/figure_kind.rs:307:11 bytes 10841–10848 | `content` |
| fn inspect_value( / match value { | 01_core/src/compiler/realize/figure_kind.rs:218:11 bytes 7852–7857 | `value` |
| fn semantic_children( / match descriptor { | 01_core/src/compiler/realize/figure_kind.rs:468:27 bytes 18150–18160 | `descriptor` |
| fn validate_grid( / match descriptor { | 01_core/src/compiler/realize/figure_kind.rs:594:26 bytes 23007–23017 | `descriptor` |
| fn visit( / match content { | 01_core/src/compiler/introspect/figure_demands.rs:41:11 bytes 1033–1040 | `content` |
| pub(crate) fn figure_counter_key( / match kind { | 01_core/src/compiler/introspect/figure_counters.rs:27:23 bytes 880–884 | `kind` |
| fn eq( / match self { | 01_core/src/entities/elements/figure.rs:31:15 bytes 1023–1027 | `self` |
| fn hash< / match self { | 01_core/src/entities/elements/figure.rs:115:15 bytes 4890–4894 | `self` |

Every reported witness was matched independently against raw compiler JSON, archived source bytes, line/column positions, and the copied workspace. All original sources match the receipt input hashes. Mutations are exactly one added probe variant in each of Content, Value, FigureKind and GridChildOrder; no other staged source differs from its archive. Candidate has 223 E0004 diagnostics and no other error code; baseline has 123 E0004 diagnostics and no other error code. Only the selected eight and zero, respectively, are credited.

The receipts preserve HEAD 731f0ffdf4c7c152f590551e72ff63f7bd85ef85, exact input hashes, command and UTC windows. Full spans, source hashes and receipt/log hashes are in structural-witness-review.json. These are structural evolution witnesses, not runtime semantic coverage.
