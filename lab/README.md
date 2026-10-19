# Laboratory

The laboratory contains validation infrastructure and quarantined material. Its
source is never imported by productive L1–L4 code. Unit tests remain beside their
productive units; benchmark harnesses, fixtures, comparison tools, and generated
test/build outputs live here.

| Directory | Purpose |
|---|---|
| `benches/` | Workspace benchmark crate |
| `tools/` | CI, release, audit, benchmark, and operational tooling |
| `tools/workflows/` | Authoring/audit procedures and L0 template |
| `results/` | Measurement evidence and validation receipts |
| `target/` | Generated product, benchmark, and tool build output |
| `scratch/` | Disposable working copies and temporary investigations |
| `history/` | Preserved construction records and restoration metadata |
| `typst-original/` | Quarantined upstream source and pinned oracle build |

Other experiments and fixture suites remain inside the laboratory. Generated
outputs and scratch files are ignored by Git. Selected evidence is versioned
explicitly; the construction migration preserves previously tracked evidence
and also retains local ignored evidence without pretending Git stores it.

See [CI and release](tools/ci/README.md), [history restoration](history/README.md),
and the [migration diagnostic](../00_nucleo/diagnostics/mature-layout-2026-10-18.md).
