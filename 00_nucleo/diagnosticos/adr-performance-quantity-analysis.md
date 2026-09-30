# Quantitative Analysis: ADR Decisions and Tekt–Vanilla Compile Time

**Measured:** 2026-09-26, starting at 00:31:59 -03:00  
**Purpose:** determine whether architectural quantities can plausibly explain the observed compile-time gap.  
**Classification:** diagnostic evidence, not an architectural decision and not a parity certificate.

## Provenance

- Repository HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`
- Tekt binary: `typst 0.15.1 (6ef8b698)`
- Tekt binary SHA-256: `55142cb384c6fbfac10ad4d617a2f5eda611a005c412c001b4883ce60a963826`
- Vanilla binary: `typst 0.15.1 (9f2b6e87)`
- Vanilla binary SHA-256: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`
- Tekt was rebuilt with `cargo build --release` immediately before the final run.
- Benchmark protocol: Hyperfine, 3 warmups and 15 measured runs per command.
- Temporary raw results: `/tmp/tekt-perf-quantity-20260926/`.

The working tree was not committed. These were the exact pre-existing changes at measurement time; this report itself was added only after the measurements:

```text
 M 00_nucleo/adr/README.md
 D 00_nucleo/adr/adr-stub-vs-fallback.md
 M 00_nucleo/adr/template-adr.md
 D 00_nucleo/adr/typst-adr-0002-hierarquia-contencao.md
 D 00_nucleo/adr/typst-adr-0003-comemo-contencao.md
 D 00_nucleo/adr/typst-adr-0004-passo1-descobertas.md
 D 00_nucleo/adr/typst-adr-0007-rustc-hash.md
 D 00_nucleo/adr/typst-adr-0028-typographic-types.md
 D 00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md
 D 00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md
 D 00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md
 M 00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md
 D 00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md
 D 00_nucleo/adr/typst-adr-0081-m7-plus-pipeline-restructuring-scope.md
 D 00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md
 D 00_nucleo/adr/typst-adr-0096-pattern-debt-37-campo-layouter-consumer-pending.md
 D 00_nucleo/adr/typst-adr-0097-scope-out-reconfirmado-por-fase-a.md
 M 00_nucleo/adr/typst-adr-0130-referencias-canonicas-l0.md
 M 00_nucleo/prompts/compiler/layout/word_segment.md
 M 01_core/src/compiler/layout/word_segment.rs
?? 00_nucleo/adr/ORDER.md
?? 00_nucleo/diagnosticos/adr-archive/
?? 00_nucleo/diagnosticos/adr-cleanup-classification.md
?? 00_nucleo/diagnosticos/adr-reference-legacy-portuguese.md
?? 00_nucleo/diagnosticos/adr-template-legacy-portuguese.md
?? 00_nucleo/diagnosticos/tekt-english-reference-pilot.md
?? 00_nucleo/materialization/tekt-english-reference-migration.md
```

The only modified productive Rust file above contains the in-progress English comment migration; the final binary nevertheless corresponds to the exact uncommitted state listed here.

## End-to-end results

| Case | Vanilla mean | Tekt mean | Tekt / vanilla | Difference | Vanilla mean RSS | Tekt mean RSS |
|---|---:|---:|---:|---:|---:|---:|
| hello | 132.9 ms | 289.7 ms | 2.18× | 156.8 ms | 29.8 MiB | 39.8 MiB |
| lorem(500) | 137.1 ms | 335.4 ms | 2.45× | 198.3 ms | 32.0 MiB | 41.4 MiB |
| 100 math expressions | 139.3 ms | 371.0 ms | 2.66× | 231.7 ms | 32.8 MiB | 57.3 MiB |
| 20 tables | 141.2 ms | 302.6 ms | 2.14× | 161.5 ms | 33.2 MiB | 41.3 MiB |
| 200 context/measure calls | 176.9 ms | 424.5 ms | 2.40× | 247.6 ms | 39.0 MiB | 187.0 MiB |
| multilingual UTF-8 | 188.2 ms | 668.9 ms | 3.55× | 480.7 ms | 68.5 MiB | 324.2 MiB |
| DejaVu subsetting | 134.5 ms | 303.6 ms | 2.26× | 169.1 ms | 31.6 MiB | 44.0 MiB |

## Finding 1: most of the minimum-document gap is font startup

Both `typst fonts` commands listed exactly 473 families. With the same Hyperfine protocol:

| Command | Mean |
|---|---:|
| vanilla font inventory | 128.9 ms |
| Tekt font inventory | 283.4 ms |
| difference | 154.5 ms |

The hello compilation gap is 156.8 ms. The independently measured font-inventory gap is therefore 98.5% of that minimum-document gap. This strongly supports a fixed startup explanation.

The implementation supports the measurement: `SystemWorld::with_fonts_and_system` calls `load_system_fonts` before the timed pipeline, and Tekt's loader extracts metadata and Unicode coverage for every discovered face. The Tekt timings for hello report only 3.9 ms inside the pipeline while wall time is 289.7 ms.

This result is associated with the activated behavior described by ADR-0020, but the cost is caused by the present eager implementation, not by the L1/L2/L3 boundary itself.

## Finding 2: `Content` has a quantitatively large representation penalty

Direct `size_of` measurements against the release artifacts:

| Representation | Size | Relative to vanilla |
|---|---:|---:|
| vanilla `Content` / `RawContent` | 24 bytes | 1.00× |
| Tekt `Content` | 664 bytes | 27.67× |

The Tekt enum currently has 116 variants. Its rare inline `SetPage` payload is itself 664 bytes; because Rust enums must hold their largest variant, every `Content`, including `Empty`, `Text`, and `Space`, occupies 664 bytes. Important contributors include `PageFill` at 200 bytes, `PageBleedSpec` at 136 bytes, and `PageMarginSpec` at 72 bytes. `Styles` is also 328 bytes and keeps the `Styled` variant large even if `SetPage` is made indirect.

Consequences by quantity:

- 1,000 contiguous values require about 648.4 KiB in Tekt versus 23.4 KiB in vanilla.
- 100,000 contiguous values require about 63.3 MiB versus 2.3 MiB, before referenced allocations.
- `Arc<[Content]>` makes cloning the sequence owner O(1), as ADR-0026-R1 intended, but it does not reduce the 664-byte stride of each element while the sequence is built or traversed.

A controlled artifact-level microbenchmark built vectors of 100,000 real text `Content` values and cloned each vector ten times:

| Implementation | Mean | Peak RSS |
|---|---:|---:|
| vanilla | 12.2 ms | 8.3 MiB |
| Tekt | 414.3 ms | 131.8 MiB |

The controlled operation is 33.9× slower in Tekt. This is not a claim that the compiler clones that exact vector ten times; it demonstrates that the representation selected by ADR-0026 has enough magnitude to explain a substantial load-sensitive penalty wherever contiguous construction, cloning, or traversal occurs.

The real compilations are consistent with this risk: Tekt reaches 187.0 MiB on the context case and 324.2 MiB on the UTF-8 case, respectively 4.80× and 4.73× vanilla. Correlation is not proof of exclusive causality, but the direction and magnitude match the controlled representation result.

## Finding 3: dynamic dispatch is not the leading explanation here

ADR-0106's `Content::Dynamic(Arc<dyn DynElement>)` is a single escape variant. None of the measured inputs constructs a user-registered dynamic element. The ordinary layout dispatch remains the exhaustive static match required by ADR-0109. Therefore the measured gap should not be attributed primarily to vtables or the dynamic escape hatch.

## Load-sensitive decomposition

Using hello as the fixed-load baseline, the additional Tekt-minus-vanilla gap is:

| Case | Gap beyond hello |
|---|---:|
| lorem(500) | 41.5 ms |
| 100 math expressions | 74.9 ms |
| 20 tables | 4.7 ms |
| 200 context/measure calls | 90.8 ms |
| multilingual UTF-8 | 323.9 ms |
| DejaVu subsetting | 12.3 ms |

Tekt's internal phase telemetry locates the largest absolute costs:

- multilingual UTF-8: 322.8 ms internal, including 141.4 ms layout, 117.3 ms shaping, 51.5 ms rendering, and 9.5 ms subsetting in this run;
- context/measure: 121.1 ms internal, dominated by evaluation/context expansion and layout;
- math: 78.6 ms internal, dominated by layout.

This means there are two different performance problems: a fixed font-startup penalty and a workload-dependent pipeline penalty. The enum footprint is a strong candidate for part of the second, but shaping, fallback, layout algorithms, and output differences must be isolated separately.

## Comparability limits

The outputs are close in size for hello (5,571 vs 5,467 bytes), lorem(500) (14,763 vs 14,622), and DejaVu subsetting (22,640 vs 24,067). They differ materially for math, context, and multilingual UTF-8. Those cases are useful stress signals but are not clean proof of equal work or language parity. No decision should use their ratios alone.

Hardware cache counters were not available because the host has `perf_event_paranoid=4`; therefore cache-miss causality remains an inference. The controlled clone benchmark and RSS measurements establish a representation cost without relying on those counters.

## Conclusion and next gate

Yes: the measured quantities can explain the observed time difference, but not as one single cause.

1. The fixed difference is explained almost entirely by eager font inventory: 154.5 ms measured versus a 156.8 ms hello gap.
2. The workload-dependent difference has a credible representation cause: Tekt `Content` is 27.67× larger, and the controlled real-type operation is 33.9× slower with 15.8× peak RSS.
3. Dynamic dispatch and ADR-0109 atomization are not supported as primary causes by this corpus.

Before changing productive code, the owner L0 `00_nucleo/prompts/entities/content.md` must be updated. Its Model D already moves element-shaped payloads behind `Arc`, but it explicitly excluded `Set*` while awaiting the style decision; the now-664-byte `SetPage` is the measured exception that invalidates the earlier performance assumption. Because changing the public enum variant shape may be a public-contract change, ADR-0127 requires the L0 stop-and-confirm gate.

The next decisive experiment is an A/B release build that changes only the storage shape of `SetPage` (and, separately, `Styled`) while preserving semantics, then repeats the same corpus. Startup font loading must be measured separately so it does not hide the representation result.
