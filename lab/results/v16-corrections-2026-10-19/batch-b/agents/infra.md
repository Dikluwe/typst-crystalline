# Infrastructure observer audit — V16-091

## Scope and evidence

The only selected consumer is `03_infra/src/integration_tests.rs`, owned by
`00_nucleo/prompts/infra/integration_tests.md`. The original nested `equations`
observer was extracted without changing its decision into the private
`fixture_equation_bodies` helper. It collects Equation.body once and stops at
that Equation; it recurses through Sequence, MathSequence and Styled only.

The first baseline state is recorded by
`../preservation-before/state.json.gz`: checkpoint
`059d1a0ffa9523dbe21af6860a215acaeef55d73`, uncommitted working tree, UTC
`2026-10-19T11:45:55.229047+00:00`, exact changed-file stat and input hashes.
`../preservation-before/freeze.json` records unchanged inputs. This audit does
not use the commit alone to identify the measured state.

The temporary explicit diagnostic projection was distinct from the actual
observer. Unlike Content Debug and elem_name, it exposed Source, ShowRecipe,
Located and Realized paths, including separate original/visual branches. It
stopped at equations and nominal semantic leaves. `../preservation-before/new-infra.log`
completed at UTC `2026-10-19T11:48:01.666719+00:00` and records four required
Equation roots through Sequence and two Styled bodies, at final sequence
positions 17, 19, 21 and 23. No causal carrier occurred on those traversed
equation paths. Opaque leaf names are diagnostic names, not enum identity:
`opaque:sequence` does not establish a literal Sequence payload. No assertion
about carrier absence inside opaque semantic leaves follows from this scan.

The unchanged real-font geometry integration case passed at UTC
`2026-10-19T11:48:02.426558+00:00`, in
`../preservation-before/sec27-geometry.log`. Its fixture bytes, font choices,
four-equation assertion, third-equation selection and 11.4686pt ascent with
0.001pt tolerance were preserved.

## Decision and controls

This evidence refutes the proposed missing-required-equation defect for the
current fixture. General Content traversal projecting causal carriers does not
authorize widening this fixture observer. Retain its bounded behavior and
replace the selected wildcard with nominally explicit unsupported variants.
No productive math, constructor, geometry or traversal change is authorized.

Three permanent controls exercise the same extracted helper:

- Direct roots and terminal outer Equation, without collecting a nested equation twice.
- Nested Styled and both sequence forms, preserving first-to-last body order and empty input.
- Strong, Heading, Par and all four causal carrier decoys remain opaque even when their payload contains an Equation.

The diagnostic-only test was retired after measurement because its nonempty
assertion was not an independent acceptance condition. Its exact instrumentation
remains in `../preimplementation.patch.gz` and its output remains in the first
baseline log. No golden snapshot of the fixture's internal topology was added.

ADR-0127 continuous flow applies: private test helper only, no public signature,
default, pipeline phase, compatibility, tolerance or production change. The L0
was revised with actual measurements before nominal implementation. Runtime
baseline success is not an exhaustiveness proof: the coordinator must additionally
record an attributable compiler-negative result at this helper in a disposable
real workspace, then validate the final candidate and lineage.

## Limits

The retained synthetic carrier opacity controls characterize this observer's
scope, not universal language semantics. A future fixture whose required
equation sits behind an unsupported path would reopen the boundary with source
and runtime evidence. This work certifies neither general carrier traversal nor
new current-pin vanilla geometry or overall math parity.

Read-only peer review of the four math evaluation observer test/L0 diffs found
no concrete concern: existing constructor assertions remained unchanged, while
new controls distinguish permitted wrappers, terminal target identity, ordered
search and target-containing opaque decoys. The root finder uniquely preserves
Styled and MathDelimited opacity. Pointer identity there is confined to naming
the fixture carrier returned by a private observer, not language parity.
