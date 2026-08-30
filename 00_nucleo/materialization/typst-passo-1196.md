# P1196 — separar bitmap glyphs do PDF builder

**Estado:** EXECUTADO — GREEN EM 2026-08-26
**Baseline condicionado:** P1195 GREEN; V15=11.

Manter `infra/export/builder.md` para `builder.rs`; criar
`infra/export/bitmap_glyphs.md` para `bitmap_glyphs.rs`. Nuclearizar somente
obrigações compartilhadas de embedding/identidade de recursos em
`_nuclei/export/bitmap-embedding.toml`; objetos PDF e rasterização permanecem
nos owners adequados.

Exigir V15 11→10, V26=0, ausência focal V5, testes PDF/bitmap selecionados,
build e igualdade dos sources fora da linhagem. Dry-run bloqueado por 10 V15.
Fechar `typst-p1196-saneamento-bitmap-builder.md`.
