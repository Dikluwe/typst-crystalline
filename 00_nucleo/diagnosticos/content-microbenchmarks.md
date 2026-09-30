# Content microbenchmark suite

## Purpose

This suite isolates costs inside the closed `Content` model so performance
work can distinguish representation, sequence normalization, traversal,
morphology, and projections before attributing an end-to-end compiler delta to
any one of them.

The benchmark code lives in the existing `typst-benches` crate. It does not
legitimize productive behavior and therefore is diagnostic infrastructure, not
a Prompt L0 owner.

## Benchmark groups

- `content/representation`: clone, structural equality, and Rust hashing.
- `content/construction`: representative leaf, wrapper, structural, and math construction.
- `content/sequence`: normalization of empty, singleton, and populated inputs.
- `content/identity`: element-name lookup and compact Debug projection.
- `content/traversal`: no-op and transforming walks over wide and deep trees.
- `content/morphology`: canonicalization of mixed styled/text/space content.
- `content/projections`: semantic emptiness, plain text, and named fields.
- `content/page_structure`: column wrapping fast path and structural rewrite.

A shared catalog adds aggregate sweeps across representative leaf, container,
style, math, reference, break, metadata, and document-resource families. The
catalog increases behavioral breadth but is not yet a claim that every enum
variant or parameter combination has a fixture.

The untimed `benches/content/coverage.rs` gate maintains an exhaustive match independent from productive
code. Adding a `Content` variant therefore breaks benchmark compilation until
the new variant is assigned to a cost family. Harness startup also rejects a
catalog that lacks a fixture for any family. This is a classification gate;
per-variant fixture completeness remains a separate incremental target.

Fixture construction is kept outside the timed body where setup is not part of
the behavior under measurement. Operations that inherently allocate retain
their output allocation inside the measurement.

## Execution

Build without measuring:

```text
cargo bench -p typst-benches --bench content_bench --no-run
```

Run the suite:

```text
cargo bench -p typst-benches --bench content_bench
```

Any number used to make or close a performance decision must be recorded with
the exact commit or dirty working-tree state, command, timestamp, build mode,
machine, and repeated-run dispersion. A smoke run only proves executability;
it is not performance evidence.
