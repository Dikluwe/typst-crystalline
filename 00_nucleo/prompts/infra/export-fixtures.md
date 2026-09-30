# L0 — Controlled PDF snapshot corpus
Hash do Código: 7683e2da

**Layer:** L3 (tests and fixtures)
**Exclusive consumer:** `03_infra/src/p307b_snapshot_tests.rs`

## Current evidence

- `03_infra/src/p307b_snapshot_tests.rs` — `compile_fixture` and `assert_bytes_eq`.
- `03_infra/fixtures/p307b/sources/`, `reference/` and `MANIFEST.md` — current input/reference corpus.
- `03_infra/fixtures/fonts/NotoSans-Regular.ttf` and `LICENSE-NotoSans.md` — the CIDFont input and license.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own only the binary snapshot harness and its controlled corpus, not PDF production algorithms. This is crystalline-to-crystalline regression evidence, distinct from language parity against upstream. Byte identity is the selected snapshot observable; it does not prove visual correctness, upstream PDF byte equality or performance.

| Input stem | Primary observable |
| --- | --- |
| 01-markup-plain | Basic text and string escaping |
| 02-markup-heading | Heading/text emission |
| 03-text-styling | Styled bold/italic text |
| 04-shapes | Rect/circle and solid paint |
| 05-gradient-linear | Linear gradient fill |
| 06-gradient-conic | Conic gradient fill |
| 07-multi-feature | Mixed structure/gradient and multiple pages |
| 08-image-jpeg | JPEG resource, with `sources/tiny.jpg` |
| 09-cidfont | Noto Sans subset, CIDFont and text mapping |

Resolve inputs/reference PDFs through `CARGO_MANIFEST_DIR`, not a developer's absolute checkout path. The harness creates a SystemWorld for each source and supplies the repository font directory only for the CIDFont case. Preserve Compact streams, Disabled tags and `CRYSTALLINE_PDF_FIXED_EPOCH=0` as explicit snapshot controls; these do not change the production defaults. Keep source/reference pairing and the licensed font identity recorded in MANIFEST.

Add a fixture for a newly exercised export path or an uncovered regression; remove it only when the behavior is removed or equivalent coverage replaces it. Regenerate references only for an intentional observable change with recorded justification. The current harness reads reference files at runtime and has an explicit `UPDATE_P307B_SNAPSHOTS` maintenance path. Use that harness rather than the removed include_bytes template or obsolete CLI invocation.

Regeneration must preserve the fixed epoch/stream/tag controls, select affected cases, repeat generation to verify deterministic bytes in the same environment, check PDF validity and refresh the operational MANIFEST. Do not bless unrelated reference changes. Do not promise identical output across toolchains, uncontrolled fonts or production identifier defaults. Exporter clocks, identifiers and deterministic serialization belong to their excluded owners, not this fixture prompt.

## Verification boundary

The existing p307b tests compare actual/reference bytes. Maintenance runs may use `cargo test -p typst-infra --lib p307b_ -- --test-threads=1` with the explicit update environment only when regeneration is authorized. No fixture/reference regeneration is part of documentary sanitation.

## Referência CIDFont — manutenção de tracking PDF em 2026-09-29

Medição anterior: tests-before-reference-update.log em
`00_nucleo/diagnosticos/tracking-pdf-2026-09-28/` registra somente
p307b_09_cidfont falhando após o estado Tc=0 explícito no caminho shaped
compacto; 6768 bytes atuais versus 6759 anteriores. A fonte e entrada
continuam iguais, sem tracking configurado; a mudança de operadores é
intencional para impedir herança de espaçamento, não uma prova de regressão
visual ou linguagem. Regerar somente 09-cidfont pelo harness, duas vezes,
com controles existentes. Validar PDF e conservar pixels/texto anteriores;
registrar bytes/hash, diferenças de stream e atualizar MANIFEST. Não
regerar outros casos nem desativar comparação byte a byte. O owner conserva
seu único harness produtivo e corpus, sem algoritmo de exportação novo.
