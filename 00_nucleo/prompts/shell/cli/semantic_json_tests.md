# L0 — Regression tests for borrowed semantic JSON emission
Hash do Código: 4c401b7d

**Layer:** L2
**Exclusive consumer:** `02_shell/src/cli/semantic_json_tests.rs`

## Evidence and scope

The parent serializer at `02_shell/src/cli.rs:2051-2096` currently serializes
through an insertion-ordered JSON tree. Its public JSON, YAML, Raw, field and
snapshot behavior is observable. C28 independent lab tests cover precision,
duplicates, writer failure and callers. The private borrowed serializer is
specified by the current owner `00_nucleo/prompts/shell/cli.md`.

## Required regressions

Own a cfg(test) child of cli using private parent APIs, without new public
bridges, feature flags, runtime probes, external heap meters or product changes.
Use useful independent lab tests as input, retaining independent goldens plus
complete equality to the old cfg(test) converter. Verify compact/pretty and LF,
null/bool/i64 extremes and integers beyond 2^53, finite f64/subnormals/-0.0,
nonfinite null, UTF8/control escaping, arrays/objects empty/nested/deep, and wide
objects in forward/reverse/permuted order with nonadjacent/nested duplicates.
First key position and last value must survive. Check authoritative snapshots
None versus Some(empty)/Some(fields), func/label, field selection, --one errors,
YAML and Raw outputs/errors. Preserve existing tests.

Exercise the borrowed SemanticJson directly with a failing writer at each
encoded prefix, checking original error, exact prefix and no retry after error.
Cover both compact and pretty. The output must remain independently owned after
input destruction. Tests must reject silently wrong order, duplicates, precision,
nonfinite handling or swallowed child errors. No wall-clock assertions, fault
injection cfg, mutants or benchmark tooling belongs in this productive test file.
