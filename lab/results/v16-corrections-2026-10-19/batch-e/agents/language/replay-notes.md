# Candidate replay and marker audit

The frozen corpus is `baseline-manifest.json`: 173 exact cases across six original receipts. `artifact-marker-receipt.json` adds 32 independently measured cases after the first report. This manifest hashes the baseline receipts; the runner rejects altered inputs and refuses an existing output directory. No candidate replay has been run by this agent yet; wait for the parent's built-product signal.

Run from repository root after the candidate build and input receipt are available:

```sh
python3 lab/results/v16-corrections-2026-10-19/batch-e/agents/language/replay.py --output lab/results/v16-corrections-2026-10-19/batch-e/agents/language/candidate-replay --validation-state PATH_TO_PARENT_BUILD_STATE
```

The build state must have a nonempty `inputs` path-to-SHA256 map. The runner verifies every input before execution and after the last case, records the build state itself/hash, hashes both binaries before and after, checks the pinned oracle SHA, and records source bytes/hash, argv, stdout/stderr, return code, UTC, HEAD and exact diffstat per case. It replays each original argument list, substituting only executable and source-file paths. Each subprocess has a 60-second timeout and commands run serially.

Classification is deterministic:

- `PRESERVED`: candidate observation equals the old product observation.
- `REPAIRED`: changed candidate observation equals the freshly rerun pinned oracle.
- `REGRESSION`: neither comparison succeeds; the run fails by default.

Successful JSON/YAML are compared as decoded values; stdout formatting and YAML trailing newlines are not language changes. Successful stderr remains checked. For errors, stdout and stderr remain exact. The only diagnostic normalization replaces the exact query source path, including its relative spelling, with `<probe-document>` because replay writes the same source into a new evidence directory. No message, span, warning, cardinality, whitespace, or error channel is discarded. Oracle drift, timeouts, source/binary/build-receipt changes, and private marker text in stdout fail the run.

A partial repair of a previously unequal compound object may classify `REGRESSION` even if individual differences improve. This is deliberate: inspect the component-level evidence rather than silently accepting all changes. The raw runner accepts no exceptions and always fails such changes. A separate `assess.py --receipt RECEIPT --review INDEPENDENT_REVIEW --output NEW_JSON` reports scoped acceptance without changing the raw receipt or pretending full parity. It binds the exact candidate binary and raw-receipt hashes, checks all changed and remaining difference paths against the independent review, rejects introduced regressions, and separately requires native/alias Equation identity to be true plus returned constructors callable. Observation digests bind reviewed values/errors; they are not a byte-format parity requirement.

The first assessor run is preserved as `candidate-scoped-assessment.json`: raw comparator false, full parity false, scoped acceptance false. It verifies 23 partial repairs but rejects three introduced lexical-style losses and the still-false native/alias identity. These results remain historical if a later candidate repairs them.

The script was syntax-checked and its argument interface inspected. Candidate execution remains pending, with no baseline overwrite.

## Artifact omission refuter

`artifact-kind-omitted`, `artifact-kind-other`, `artifact-kind-layout`, `artifact-kind-unset-error`, and all other accepted kind names are recorded in `artifact-marker-receipt.json`.

Omitted kind in the old product has `has(kind)=true`, `at(kind)="other"`, but `fields()` omits it. Explicit `kind:"other"` has the same representation. The oracle distinguishes omitted (`has=false`, absent field, unset error without default) from explicit other (`has=true`, field present). The native constructor at `compiler/stdlib/pdf.rs:331` collapses omission to `ArtifactKind::Other`; no presence carrier is retained. Therefore a universal kind candidate fabricates an omitted field. Adding only non-Other values could improve those specific probes, but would leave explicit Other unrepresentable and is not evidence for total projection parity. This debt requires its own presence design/classification; it is not an accepted difference.

## New nucleus marker audit

Read candidate `prompts/_nuclei/math/equation-constructor-origin.toml`. Its local marker requirement can distinguish constructor transport from an ordinary style wrapper, provided recognition reads only the direct local delta and never a merged chain. The four carrier rules, ordinary Styled barrier, and semantic container barriers need independent tests. An external wrapper that merely carries an equation option key is not constructor provenance.

The baseline marker probes cover direct constructor, `text(fill:red, constructor)`, nested text wrappers and lexical `set text(fill:red)` scopes, observing `repr`, `json.encode`, direct JSON/YAML, `has("equation.constructor")`, and same-source language equality. Their sources must remain in candidate replay.

Source findings distinguish confirmed protections from possible leakage:

1. Shell `cli.rs:1408–1410` emits literal `styles(..)` for ordinary Styled, so it does not directly serialize the custom key. This does not prove the constructor is projected correctly.
2. `eval/repr/content.rs:554–620` reads named equation options only, not an arbitrary custom dump. Current public repr nonetheless erases outer ordinary Styled; candidate must not claim universal repr parity from absence of marker text.
3. Native `json.encode` is a separate projection. The baseline direct constructor encodes func styled/child/styles, while the oracle encodes equation/numbering/body. The nucleus's generic serialization claim is not fully certified by shell-only tests. Preserve this residual or implement through its own audited L0; do not weaken the old expected evidence.
4. `eval/observation/style.rs:59` compares the entire custom vector. This may be intentional for other style data, but a new internal marker can affect observational equality through callers. Counterexamples should compare alias/clone and reconstructed equal constructor values under ordinary styles; test marker-only transport versus its semantic content at the owner boundary.
5. `entities/content/identity.rs:265–266` Debug prints Styles; Styles derives Debug (`entities/style.rs:244`). The marker can appear in internal Rust Debug. That alone is mechanics, not a parity violation. Trace any public error/diagnostic path that uses this Debug before claiming the marker is unobservable.

Recommended refuters: exact raw constructor callback identity and callability; outer and nested ordinary text remain styled rather than equation; explicit option overrides preserve none and lexical presence; local set text cannot acquire a constructor marker by merging; marker key absent from fields/has/at/CLI encoders/native encoders/repr; location/snapshot precedence preserved; no function-name or geometry inference; non-Equation and Sequence receivers remain opaque. The existing source corpus covers reachable portions, while exact synthetic carrier combinations still need the independently owned Rust tests.
