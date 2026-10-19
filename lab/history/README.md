# Construction history

The pre-migration repository is preserved by Git tag
`tekt-construction-2026-10-18` at commit
`8ec827fd975bf14cd849bebb9ede245e49033337`. No history was rewritten and no old
development ref was removed.

`construction/manifest.json.gz` maps original paths to retained files with
SHA-256, byte size, and whether each file was tracked at the checkpoint. Files
were moved without rewriting their contents. The manifest covers local ignored
evidence as well as Git-tracked files. Local ignored evidence is still local;
the Git tag alone cannot restore it on another computer.

Construction records live in `construction/records/`; old diagnostic datasets
live in `../results/construction/diagnostics/`. Historical paths inside these
records describe their original snapshot and are intentionally not rewritten.
Non-owner L0 documents retired during this migration live in
`construction/retired-l0/`, with their status explained in the
[active retirement diagnostic](../../00_nucleo/diagnostics/retired-nonowners-2026-10-18.md).

To inspect an original tracked document, use `git show
tekt-construction-2026-10-18:<original-path>`. To reconstruct a retained local
artifact, read its manifest row, apply any recorded relocation overlay, verify
the SHA-256 of the retained destination, and copy it to `<restore-root>/<old>`.
Use a separate restoration directory; do not overwrite the active layout.
The [migration receipts](../results/mature-layout-2026-10-18/) include full
hash verification and a sample restoration. The root `.typ` relocation overlay,
when present there, supersedes its initial scratch destination.
