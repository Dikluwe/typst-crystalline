# P1505 — `Content` Representation A/B/C

**Measured:** 2026-09-26, 00:31–01:13 -03:00  
**Classification:** diagnostic evidence; not a parity certificate.  
**Decision owner:** `00_nucleo/prompts/entities/content.md`.

## Provenance and protocol

- Repository HEAD for all stages: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`.
- The working tree was uncommitted. `git diff HEAD --stat` at each stage is
  retained in the task transcript; the final diff includes the pre-existing
  ADR cleanup/English migration and the P1505 implementation.
- A (baseline) release SHA-256:
  `55142cb384c6fbfac10ad4d617a2f5eda611a005c412c001b4883ce60a963826`.
- B (`SetPage` indirect only) release SHA-256:
  `1159184d1c06da2e0fb3aaca15017370ad287be14dc7a21c3dd9b1e4521a4017`.
- C (`Styled` indirect over B) release SHA-256:
  `76d905e0ea8872bf43a941a9c5f3cf2288074cfd4c0dcbafd7d84e0b8c943170`.
- Vanilla remains the pinned `9f2b6e8715237cb086899a42873660fe744622e8`
  binary with SHA-256
  `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
- End-to-end protocol: Hyperfine, 3 warmups and 15 measured runs. Clone
  protocol: 100,000 real text `Content` values cloned ten times, 3 warmups and
  10 measured runs.
- Raw JSON, timing telemetry, PDFs, probes, and `/usr/bin/time -v` output are
  under `/tmp/tekt-perf-quantity-20260926/`.

## Representation result

| Stage | Storage change | `size_of::<Content>()` | Change from A |
|---|---|---:|---:|
| A | inline `SetPage`, inline `Styles` in `Styled` | 664 B | — |
| B | `SetPage(Arc<SetPageDelta>)` | 336 B | −49.4% |
| C | B + `Styled(Arc<StyledContent>)` | 120 B | −81.9% |
| Vanilla reference | `RawContent` | 24 B | −96.4% |

Stage B proves that the 664-byte `SetPage` payload determined the original
enum size. Stage C proves that the 328-byte `Styles` value then determined the
next floor. After both are indirect, the next-largest unrelated variant sets a
120-byte floor; P1505 does not alter that variant.

## Controlled clone result

| Stage | Mean | Peak RSS | Relative to vanilla mean |
|---|---:|---:|---:|
| Vanilla | 11–12 ms | 8.3 MiB | 1.00× |
| A | 414.3 ms | 131.8 MiB | 33.9× |
| B | 239.3 ms | 66.1 MiB | 19.2× |
| C | 35.7 ms | 24.7 MiB | 3.17× |

Relative to A, C is 91.4% faster and uses 81.3% less peak RSS in the
controlled operation. This directly confirms that the inline representation
was a major cost, while the remaining 5× size difference versus vanilla can
still account for part of the residual 3.17× clone gap.

## End-to-end wall time and RSS

| Case | A mean / RSS | B mean / RSS | C mean / RSS | C vs A |
|---|---:|---:|---:|---:|
| hello | 289.7 ms / 39.8 MiB | 292.3 ms / 39.0 MiB | 277.0 ms / 39.0 MiB | −4.4% |
| lorem(500) | 335.4 ms / 41.4 MiB | 331.0 ms / 40.0 MiB | 315.8 ms / 40.0 MiB | −5.8% |
| 100 math expressions | 371.0 ms / 57.3 MiB | 333.8 ms / 50.4 MiB | 337.5 ms / 45.9 MiB | −9.0% |
| 20 tables | 302.6 ms / 41.3 MiB | 300.5 ms / 40.3 MiB | 284.2 ms / 40.5 MiB | −6.1% |
| 200 context/measure calls | 424.5 ms / 187.0 MiB | 391.1 ms / 126.9 MiB | 339.9 ms / 96.3 MiB | −19.9% |
| multilingual UTF-8 | 668.9 ms / 324.2 MiB | 612.3 ms / 324.6 MiB | 617.7 ms / 324.2 MiB | −7.7% |
| DejaVu subsetting | 303.6 ms / 44.0 MiB | 302.6 ms / 44.1 MiB | 281.1 ms / 44.0 MiB | −7.4% |

The B math wall-time value comes from the clean rerun (333.8 ms); its RSS
comes from the first run (50.4 MiB). The rerun reported an impossible constant
324.6 MiB for all 15 samples while another benchmark had just run, so that RSS
sample is retained raw but rejected as measurement contamination. The first
math wall-time run was also rejected for timing because it contained large
outliers (389.9 ± 48.3 ms).

The largest clean benefit is the context workload: 19.9% lower wall time and
48.5% lower mean RSS. Its internal phase total fell from 121.1 ms to 61.8 ms;
evaluation fell 43.9 → 23.2 ms, context expansion 41.0 → 14.8 ms, and layout
25.1 → 13.9 ms. This is the workload with the heaviest repeated tree
construction/traversal, so it is consistent with the representation mechanism.

The UTF-8 workload improves 7.7% in wall time but not RSS. Its dominant shaping
and rendering allocations are outside `Content`, which limits the memory
effect expected from P1505.

## Output and semantic validation

- All seven PDFs preserve their baseline byte size at B.
- At C, six preserve the exact size. UTF-8 is 395,767 bytes instead of
  395,768; document IDs are intentionally non-deterministic, and PDF byte
  identity is not the language-level gate under ADR-0107.
- `cargo test -p typst-core --lib`: 6,211 passed, 0 failed.
- The existing language-level equality, morphology, style precedence,
  introspection, layout, and export tests all remain green.

## Decision

Retain both B and C. Neither stage regressed the comparable corpus beyond
run-to-run noise; both materially reduce the controlled representation cost,
and C produces clear end-to-end gains on all seven cases relative to A.

The experiment explains a significant part of the workload-dependent gap,
especially context expansion, but not the fixed startup gap or the remaining
UTF-8 cost. The next independent performance target remains eager font
inventory/startup; shaping and fallback remain a separate UTF-8 target.
