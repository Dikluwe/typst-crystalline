# L0 — infra/shaper
Hash do Código: 0b78a31d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper.rs`

## Current evidence

- `03_infra/src/shaper.rs:1` — lineage and facade module declarations/reexports.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper.rs`.

## Owned contract

- Own only private module declarations and facade reexports. Preserve public shape_document/fix_line_positions and the existing crate-private FaceCache/CachedFace/shaped_width import paths.
- The paged pipeline owner places the pass: `00_nucleo/prompts/infra/pipeline/paged.md`. Text traversal and fallback are governed by `00_nucleo/prompts/infra/shaper/traversal.md`; glyph materialization is governed by `00_nucleo/prompts/infra/shaper/shaping.md`. This facade does not repeat those contracts.
- Traversal, bidi, font selection, shaping, caching and reconciliation are owned independently below. Keep static delegation and existing visibility; do not add a registry, dynamic dispatcher, public contract or L1 font-byte access.

## Child owners

- `00_nucleo/prompts/infra/shaper/bidi.md` owns only `03_infra/src/shaper/bidi.rs`.
- `00_nucleo/prompts/infra/shaper/cache.md` owns only `03_infra/src/shaper/cache.rs`.
- `00_nucleo/prompts/infra/shaper/font_selection.md` owns only `03_infra/src/shaper/font_selection.rs`.
- `00_nucleo/prompts/infra/shaper/reconciliation.md` owns only `03_infra/src/shaper/reconciliation.rs`.
- `00_nucleo/prompts/infra/shaper/shaping.md` owns only `03_infra/src/shaper/shaping.rs`.
- `00_nucleo/prompts/infra/shaper/tests.md` owns only `03_infra/src/shaper/tests.rs`.
- `00_nucleo/prompts/infra/shaper/traversal.md` owns only `03_infra/src/shaper/traversal.rs`.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## SHY autoral não selecionado — correção de identidade

### Medição anterior à decisão

`00_nucleo/diagnosticos/shy-autoral-2026-09-28/initial.json` e
`before/resultados.json` registram estado, binários, comandos e trinta sondas
bilaterais. Em largura 300pt, repre<U+00AD>sentation extrai dois SHY no Tekt
e um no vanilla pinado. O controle spaces extrai seis versus um.
`03_infra/src/shaper/shaping.rs:313-325` transporta o cluster SHY produzido
pelo rustybuzz; `03_infra/src/export/fonts.rs:318-328` associa sua substring
ao GID de espaço. `export/builder/fonts.rs:523-535` conserva uma identidade
por GID; a stream do controle anterior associa CID 0001 a 00AD e o reutiliza
no espaço posterior. São observáveis textuais, não igualdade de bytes.
A hipótese seria refutada por ausência dessa associação ou por duplicação
já existente na string que entra no shaper. Não inferir intenção histórica.

### Decisão e limite

Correção de paridade interna em fluxo contínuo ADR-0127: reutiliza contrato
TextReplacement aprovado, sem campo, assinatura pública, flag ou mudança
de fase. Não modifica seleção de quebras ou desenho. A causa explícita é
o U+00AD existente no texto autoral, não aparência, GID ou posição.
Casos estreitos já têm divergências de seleção e não fecham a política SHY.

Declarar somente o novo filho privado text_identity, cujo owner exclusivo é
`00_nucleo/prompts/infra/shaper/text_identity.md`. A fachada não decide
a política de identidade nem muda reexports.

## Correção de tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/tracking-residual-2026-09-29/initial.json`, `results.json`, `summary.json` e `causal-checks.json` registram 36 entradas/108 PDFs contra vanilla pinado, fontes, UTC, HEAD e working tree completa. A palavra isolada acumula +0.052pt com tracking 0.2pt, mas fronteiras perdem um intervalo na adjacência, dois com um espaço e quatro com três espaços literais. ±1.2pt elimina a quantização sem eliminar as fronteiras. O código shaper/shaping.rs:225-232,319-326 arredonda em i32; text.rs:49-83 divide palavras; space.rs:15-33 aplica somente avanço nominal. O estado anterior à materialização está em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29/initial.json`.

### Decisão interna e responsabilidade

Correção interna de paridade em fluxo contínuo ADR-0127, sem contrato público, flag ou fase nova. As cláusulas desta rodada supersedem somente a preservação dos limites de tracking descritos abaixo; demais políticas de render permanecem.

Declarar filho tracking com visibilidade crate-private para consumo pelo exportador, sem novo reexport público. Owner exclusivo: `00_nucleo/prompts/infra/shaper/tracking.md`. Fachada não repete fórmulas.

Aceitação: testes RED→GREEN de fronteiras/seleção e interpretação independente Tc/TJ, controles bilaterais de tracking ±/zero, repetidos, adjacência e wrap; build e lint sem erros, resselo restrito.
