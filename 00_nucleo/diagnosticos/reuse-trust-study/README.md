# Reuse and trust study

Diagnostic-only behavioral controls, not performance measurements. Production, L0s, ADRs and prior evidence remain unchanged. Reproduce in a fresh sibling directory with no existing outputs using `PYTHONDONTWRITEBYTECODE=1 python3 run.py`; the runner refuses to overwrite results.

The probe uses current public SystemWorld, FontSlot and shape_document APIs. Four pinned fonts are in inputs.json. Temporary files stay under runtime/ in this directory; main.typ is deliberately changed and restored to its initial bytes, and a transient font is created then removed. These fixture mutations are interventions, not input drift in production. Consumer/L0/vanilla-pin fingerprints remain frozen.

Controls: retained World versus fresh source after an edit; cached missing font versus fresh slot after file creation; same versus separate World TTC buffer ownership while all handles remain alive; replacing a catalog after populating coverage; mixed normal/math shaping in both orders versus isolated calls; four-decimal f32 formatting collision. The formatting probe is a diagnostic reproduction of an encoding operation, not an invocation of private cache APIs or proof of variable-font rendering error.

Glyph signatures compare raw glyphs/advances/clusters for the same public shaping entry point and fixture. They are bounded cache consistency checks, not proof of all language parity, PDF validity or performance. Actual mixed cases/results are in results.json; unsuccessful controls must remain visible.

run.py records exact dirty status/diff-stat/patch, source hashes before/after, Cargo.lock, commands/profile, executable hash/toolchain and UTC. No concurrency isolation or stable timing claim is made. The final executable is archived in bin/ and the dataset sealed by MANIFEST.sha256. Read ../reuse-trust-study.md for lifecycle ownership, invalidation requirements, findings and unresolved experiments.
