# Candidate replay review

Result: **PASS for the declared replay scope**, with no audit findings. This is
an artifact audit of the completed candidate build and both replays, not a claim
of global language parity or completion of the remaining structural gates.

Review state: HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:27:17.120149+00:00`. The JSON companion records
full uncommitted diffstat, evidence hashes, independently extracted SVG bounds,
source snapshots and completed command entries. No Cargo, compiler binary or
PDF extractor was executed during review.

## Build and frozen-source link

The recorded workspace build completed before the grid replay; the measurement
replay followed it. `.cargo/config.toml` selects `lab/target/product`, matching
the replay binary. Both receipts identify product SHA-256
`0c60f5fdab3947b160bcb9c147bd933c8aa24279c9f38f2800e94d24dfe4a64d` and the ratified oracle SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Both reference the same candidate state receipt SHA-256
`c495d19de432d2854c2ecdab47341801327ab0a0ea6f6aec4c65cba3b9244679`. Every replay source/L0 hash agrees with that
receipt, and all 2126 recorded build inputs still match at audit time.
The build log records a successful workspace build; the separate core log records
7512 passed, zero failed and five ignored. These counts are evidence from the
archived commands, not new test execution by this reviewer.

## Measurement replay

All 27 cases / 54 compiler runs completed successfully. I recomputed global SVG
coordinates from the actual output files, including ancestor transforms, without
using the runner's pass flags as the geometric oracle. Each non-transform output
has exactly one red marker; the runner rejects missing markers even if both
products omit them. The shape transform has exactly one green rectangle with
matching transformed bounds; empty and metadata transforms have none.

The next-column origin changed from zero to 40pt for Image, Align(Image),
Styled(Image) and captionless Figure, and from zero to 23.124444444pt for the
simple inline equation. Those values match the pinned oracle. Actual body
presence remains observable: each image case retains its image, the figure
retains its green shape, and the equation retains its three glyph instances.
Fixed-width and neutral controls pass without discarding output.

Each of the four source effect fixtures contains an explicit final-state-equals-one
assertion; both compilers successfully compile each fixture. This supports the
one-update controls, not a universal assertion about all callbacks. The unit
checks for hidden callback rejection and emitter state are separate evidence.

Measurement receipt SHA-256:
`0d85514b786768f3fe7c9d97542cc691151daa12a06863555cfa618f02c6dae3`.

## Grid replay and limits

All 18 grid fixtures preserve the baseline product's exact extracted layout text,
page data and word boxes. Source fixture text, successful exits and product
diagnostics are unchanged. Compile and extraction success are checked separately;
missing observables cannot satisfy the final runner checks.

This is product conservation, not full upstream parity. HITS: 4 remains present
in both products for the show/state control. The contextual case still has
FINAL: 2 in both, while the product omits the intermediate cell values 1 and 2
that the oracle displays. The nested Table vertical-origin difference, other
recorded geometry/extraction differences, captioned figures and unsupported
math/image-axis cases remain in `../../open-obligations.json`.

Grid receipt SHA-256:
`89befe7124f5543bc7f0c4b4d2beca43d1397466b988fb671758361dbe22ab13`.

The measurement comparator validates the declared column-origin and transform
observables. It does not certify full-page raster equality, every mathematical
composite, or language behavior outside these sources. No unresolved debt was
converted to an accepted zero extent or erased from the evidence.
