# L0 — stdlib/foundations/path
Hash do Código: 6d935eb3

**Vanilla ratificado:** `a51e02804`

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/path.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/path.rs:15`, `01_core/src/compiler/stdlib/foundations/path.rs:27`, `01_core/src/compiler/stdlib/foundations/path.rs:37` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own native_path and the resolve_path_value/read_path_value helpers. resolve_path_value preserves Path identity and resolves Str through World; read_path_value materializes bytes only after obtaining RootedPath. native_path rejects named arguments, requires one positional, preserves Path and resolves Str relative to current_file; other types fail `expected path or string`. Preserve normalization, rooted identity and retention across callers/files. No direct filesystem I/O or physical-path resolution in L1.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
