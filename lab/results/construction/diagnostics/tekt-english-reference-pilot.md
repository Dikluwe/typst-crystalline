# English reference migration — pilot review

## Provenance

Checkpoint: `2026-09-25T23:15:18-03:00`.
HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5` (P1503/P1504 baseline).
The local `tekt-historico` branch preserves this baseline. The active branch
is `Tekt`, with an uncommitted translation working tree. Remote refs were
not changed and no push was performed.

Files present in the translation working tree at validation:

- `00_nucleo/adr/typst-adr-0130-referencias-canonicas-l0.md`;
- `00_nucleo/prompts/compiler/layout/word_segment.md`;
- `01_core/src/compiler/layout/word_segment.rs`;
- `00_nucleo/materialization/tekt-english-reference-migration.md` (new).

`git diff HEAD --stat` at that checkpoint, excluding the untracked process
document above:

```text
 .../adr/typst-adr-0130-referencias-canonicas-l0.md | 42 ++++++++---------
 00_nucleo/prompts/compiler/layout/word_segment.md  | 55 +++++++++++-----------
 01_core/src/compiler/layout/word_segment.rs        | 44 ++++++++---------
 3 files changed, 71 insertions(+), 70 deletions(-)
```

This diagnostic was added after those checks; it does not change the
validated L0, ADR, or source. The full corpus is not yet translated.

## Accepted changes

The word-segmentation L0, the ADR-0130 prose, and the corresponding Rust
comments are now in English. Decision IDs, historical measurements,
approval, paths, literals, nucleus pin, and executable code were preserved.
No prompt or ADR was renamed or moved in this batch.

The linter's literal metadata labels `Hash do Código:` and `Núcleos Tekt:`
remain in Portuguese for parser compatibility. The linter was not changed.

After resealing:

- `word_segment.rs` has `@prompt-hash ed99c335`;
- its L0 has `Hash do Código: e0221ae7`;
- its `coordinates.toml` pin is unchanged.

## Local translation review

The local LM Studio model `hunyuan-mt-7b-8k-latest` generated drafts from
the Portuguese baseline. Reviewed contract paragraphs and complete-document
drafts were compared against the original, not treated as specifications.

Full-document drafting was not safe without review. The word-segmentation
draft changed machine metadata, expanded ADR incorrectly, replaced shaping
with formatting, omitted acceptance observables, and reversed a refutation
condition into an obligation. The ADR draft corrupted annotation prefixes
and paths, translated `_comum.md`, and omitted an inventory entry. None of
those deviations was accepted.

Raw complete-document drafts:

- `/tmp/tekt-english-pilot.2KYGRa/draft-0.md`, SHA-256
  `e227ad34f590fe0137bd966030afe55d9b2579982695da5fcd486da832929c61`;
- `/tmp/tekt-english-pilot.2KYGRa/draft-1.md`, SHA-256
  `1a4d7d915f1e6594b107ae9609af263edb6dde110fa8122fe7d43ba022d59ef8`.

These temporary drafts are evidence of the observed failure modes, not
canonical documents. Future batches should isolate prose from technical
literals before local drafting and still require clause-by-clause review.

## Validation

Logs are in `/tmp/tekt-english-pilot.2KYGRa/`.

- V15/V26 preflight: exit 0 before hash repair.
- V5/V6/V15/V26 with warnings treated as failures: exit 0.
- Hash dry-run: exit 0, `Nothing to fix`.
- Full `crystalline-lint .`: exit 0; existing advisory warnings remain.
- `cargo build --offline`: exit 0.
- Focused `typst-core` library test
  `p1504_aspas_cjk_nao_ficam_na_borda_errada`: 1 passed, 0 failed.
- `rustfmt --edition 2021 --check` for the changed Rust file: exit 0.
- `git diff --check`: exit 0.
- Source comparison with `tekt-historico`, removing only complete line
  comments: no differences. The source has no changed executable lines.
- Inline-code literals in both translated documents: unchanged sequence
  relative to the Portuguese baseline.

An initial test command used `--exact` with an incomplete test name and
selected no tests; it is not counted as test evidence. The corrected
library command above ran the actual test.

## Remaining work

The remaining prompt corpus, nucleus prose, source comments, ADR translation,
and ADR reference reorganization are pending. A corpus-wide path migration
must preserve IDs and audit incoming references before any move. The next
batch should be small, protect technical syntax outside the translator,
and keep executable code frozen.
