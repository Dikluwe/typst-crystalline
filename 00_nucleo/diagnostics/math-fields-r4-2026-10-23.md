# Public mathematical fields and qualified conversion — R4 — 2026-10-23/24

The session started on local 2026-10-23 and completed on 2026-10-24; receipts
retain the exact UTC for every measurement.

The reported R4 divergences are corrected against pinned vanilla. In the expanded
**210-case R4 corpus, 80 differences become
0** in both debug and release. The positive observations
compare complete JSON values; rejected calls compare exit status and the first
diagnostic line. Full stderr/spans remain recorded, without claiming universal
diagnostic-span parity or complete mathematical reflection/render parity.

## Provenance and gates

HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, **uncommitted working tree**. The
[received baseline](../../lab/results/math-fields-r4-2026-10-23/baseline.json) records exact UTC, full dirty
`git diff HEAD --stat`, status, all source/L0/configuration hashes and previous
R3 completion hash. The [source archive](../../lab/results/math-fields-r4-2026-10-23/baseline-source.json.gz)
includes untracked productive files. Only 38 inputs changed
from that state: nineteen existing owning L0 prompts and their nineteen consumers.
1853 font/reference assets remain unchanged. The
[batch-only diff](../../lab/results/math-fields-r4-2026-10-23/changes-from-received.diff) separates this work from
all preexisting A–H/R1–R3 changes, which were preserved. The complete
[preserved evidence archive](../../lab/results/construction/diagnostics/math-fields-r4-2026-10-23/README.md)
lives under construction/diagnostics; original runtime receipt paths remain available.

Oracle commit **bd5ebec285505889e8ff1d4f68804ab147a699f3**, tree
**cac836c07e4dec2a89219370428292393a3216b5**, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Its existing verified build was retained; no resync.
[Executable/source bindings](../../lab/results/math-fields-r4-2026-10-23/artifact-source-bindings.json) bind each
immutable executable to its actual source archive and build recipe. Observation
working-tree maps are not substituted for a frozen before executable's sources.
Final release SHA-256 `300c2259ffbd72924004e1af55db4eaba12fcd4d97080704ffb519b579b6aef2`; debug `081fa9f804e29cb983005324f3b3a8b6b670ce4e44d2c1460987fdb1840cfd7e`.

This follows ADR-0127 continuous parity flow. Existing entity payloads, trait
method declarations, public signatures, product defaults and phase order did not
change. L0 was updated before implementation, and V15/V26 passed before resealing.
The [first functional RED](../../lab/results/math-fields-r4-2026-10-23/red-tests/receipt.json) has three behavioral
failures; [supplemental RED](../../lab/results/math-fields-r4-2026-10-23/red-supplemental-tests/receipt.json) has five.
The [focused GREEN](../../lab/results/math-fields-r4-2026-10-23/green-tests/receipt.json) passes all five initial tests. An earlier
fixture compilation failure is preserved in `red-invalid-tests` and is excluded
from the functional RED evidence. Preflight V16 caught a test-fixture catchall;
its repair used an explicit expectation table, L0 first, without exemptions.
A further [accent carrier RED](../../lab/results/math-fields-r4-2026-10-23/red-accent-tests/receipt.json) isolates
Content::Symbol in a qualified formula accent; [expanded GREEN](../../lab/results/math-fields-r4-2026-10-23/green-accent-tests/receipt.json)
passes all six focused tests. The complete final-source suite rechecked all six.

## Corrected observations

- Vec and Cases expose ordered `children`, preserving native Symbol children.
  Their existing explicit bits govern `delim`, `align`/`reverse`, and `gap`:
  omitted options stay unset; explicit defaults and delimiter None sides survive.
- Class exposes its required `class` and `body`; numeric MathText exposes `text`.
  Getters and `fields/has/at` agree. Direct Vec/Cases absent-field reads preserve
  the distinction between unset and undeclared and use the measured element names.
- Qualified frac, sqrt/root, cancel, underline, accent, limits/scripts, class, vec
  and equation reuse the existing content cast. Accepted Symbol retains its
  native carrier, String becomes text, None becomes empty, and integer/boolean
  remain rejected with the measured type vocabulary. Root's absent index stays
  distinct from a present empty index.
- Accent also accepts the native Content::Symbol accent carrier, including Source
  wrappers; qualified formula calls no longer reject that carrier.
- Attach's six Symbol slots preserve Symbol instead of MathText. Omitted,
  explicit None and present empty slots remain distinct. Its base accepts None.
- Binom's required upper content cast stays separate from its variadic lower
  display conversion; lower None, integer and boolean now follow that display path.
  Existing String/Symbol upper and String attachment-slot morphology remains
  bounded by the prior contracts; arbitrary display-value parity is not certified.

[Before](../../lab/results/math-fields-r4-2026-10-23/before-final/receipt.json),
[final debug](../../lab/results/math-fields-r4-2026-10-23/after-debug/receipt.json) and
[final release](../../lab/results/math-fields-r4-2026-10-23/after-release/receipt.json) retain exact expressions,
argv, UTC, binary/source hashes, values and errors. The corpus has 170
successful JSON observations and 40 bilateral rejections. Matrix-symbol
fields remain a passing control, rather than being counted as repaired debt.

## Complete validation and preservation

[Full validation](../../lab/results/math-fields-r4-2026-10-23/validation-complete/validation-commands.json) passes all
12 gates: build, domain/infra and wiring tests, doctests,
all targets, Clippy with warnings as errors, formatting, strict Rustdoc, Python
checks, structure and strict/reverse lineage. Rust: **9873 passed,
7 ignored**. Final full `crystalline-lint . --fail-on warning`
and `git diff --check` pass. Builds run sequentially, offline/locked, one job,
12 GiB max, 8 GiB high and zero swap; effective active-unit resource receipts
are preserved. [Final audit](../../lab/results/math-fields-r4-2026-10-23/audit.json) checks the frozen inputs and
all replay acceptance rather than inferring success from process dispatch.

[Replays](../../lab/results/math-fields-r4-2026-10-23/replays.json) preserve A's 60 complete page/word cases,
B's 28 init cases, C/D/F's 40 PDF geometry cases, H's 98 stdout checks and
R1's ten controls. The separate prior E 210-case corpus preserves
209 raw outcomes; 1 changes
now match successful oracle JSON (numeric MathText fields). Preserved error
outcomes and known extensions are not relabeled as language parity.
R2's 24 cases and 14 boundary controls retain their qualified scope and
known justified-overhang/unsupported-context limits. All 47 R3 PDF/SVG probes
retain their prior geometry at 0.001 pt, including the nineteen qualified
originals and the three already recorded additional limits.

[Additional PDF/SVG validation](../../lab/results/math-fields-r4-2026-10-23/render-audit.json) compares fourteen
qualified Symbol/None/display forms to fourteen explicit Content counterparts.
Every converted form has identical full PDF text/line/page/bounds and ordered
SVG coordinates to its explicit counterpart; all fourteen explicit layouts are
preserved from the received binary. Nine families pass the strict bilateral
observer. Root additionally has equal physical geometry with a different SVG
paint order; Scripts has equal physical word boxes/SVG positions with different
PDF extraction line order. These mechanical ordering observations remain recorded.
Accent/Class native Symbol bodies retain a preexisting Greek-form discrepancy;
nested Equation retains a preexisting layout failure. The before Content
refuters prove these three geometry limits predate this conversion correction.
They remain open; fields/casts success does not certify their layout.

[Memory replay](../../lab/results/math-fields-r4-2026-10-23/memory/receipt.json) retains all seven original PDF
workloads under the existing repeatability and 3% before/current preservation
gate. [Normalized executable size](../../lab/results/math-fields-r4-2026-10-23/final-artifact-test.json) retains
the G size gate. These are preservation measurements, not a new performance claim.

## Remaining work

The reported R4 scope is closed. R1 note geometry and broader note layout,
R2 overhang/unsupported continuation contexts, R3 ordinary-text fallback-edge
and inline-after-pagebreak baselines, the additional Accent/Class Symbol-body
and nested Equation layout observations above, and R5 painting/R6 cost/R7 legacy-checker
policy remain open. Math families outside the observed field/constructor corpus,
arbitrary Value display and exact diagnostics across all invalid calls remain
uncertified. See the [current comparison](vanilla-comparison-after-ah-2026-10-23.md)
and [handoff](vanilla-divergence-handoff-2026-10-23.md).
