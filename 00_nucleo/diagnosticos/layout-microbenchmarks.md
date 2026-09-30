# Layout microbenchmark suite

## Purpose

This suite measures two public behaviors of the atomized layout core.
`measurement` includes temporary layouter construction and sub-frame layout
with FixedMetrics. `style_scope` includes complete introspection/layout on
style fixtures; it does not isolate the private style-scope helper. Neither
group measures real font shaping. Compare identical fixtures between revisions
before attributing a pipeline change to either behavior.

The benchmark code lives in the existing `typst-benches` crate. It is
diagnostic infrastructure and does not legitimize productive behavior or
change the Prompt L0 ownership graph.

## Initial groups

- `layout/measurement`: `measure_content_real` over short and long text, a
  wide sequence, nested strong wrappers, and a vertical stack.
- `layout/style_scope`: the public `layout` entry point over plain, strong,
  emph, small-caps, one styled wrapper, and nested styled wrappers.

Fixture construction happens outside the timed body. Every fixture is checked
once before Criterion starts sampling: measurements must be finite and styled
layout must preserve its expected morphology, including the uppercase
projection currently emitted by small-caps.

`par.line` remains represented by the `pipeline/pdf/line_callback` cohort. Its
private pass state is deliberately not exposed merely to make a benchmark;
an isolated callback benchmark requires an existing public behavioral seam.

## Execution

Build without measuring:

```text
cargo bench -p typst-benches --bench layout_bench --no-run
```

Run the suite:

```text
cargo bench -p typst-benches --bench layout_bench
```

Filter one family during refinement:

```text
cargo bench -p typst-benches --bench layout_bench -- layout/measurement
cargo bench -p typst-benches --bench layout_bench -- layout/style_scope
```

A smoke run proves only that the harness executes. Any number used to make or
close a performance decision must record the exact commit or dirty working-tree
state, command, timestamp, release profile, machine, and dispersion from at
least three sequential runs.
