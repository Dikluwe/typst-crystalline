# L0 — infra/shaper/shaping
Hash do Código: 5a40f19e

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/shaping.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c
```

## Current evidence

- `03_infra/src/shaper/shaping.rs:21` — `ssty_level_for_subrun`.
- `03_infra/src/shaper/shaping.rs:38` — `shaped_width`.
- `03_infra/src/shaper/shaping.rs:106` — `try_shape`.
- `03_infra/src/shaper/shaping.rs:366` — `byte_idx_to_char`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/shaping.rs`.

`00_nucleo/diagnosticos/infra-performance-followup/language-control.json` shows
that explicit ja/zh/ko settings do not affect Tekt glyph selection while a same-face
rustybuzz control does. Both local buffer sites omit language configuration.
Pinned vanilla configures language at
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:980`.
Language selection is a style semantic input; glyph IDs are discriminating render
controls, not proof of full language parity. The cache structure is mechanics.
The repository Noto Sans fixture supplies independent ru/sr local substitutions
with different advances; controls must cover measurement and emission separately.

## Owned contract

- Own shaped_width, try_shape, UTF-8 cluster mapping, rustybuzz setup, variations, tracking, ssty and TextShaped materialization. Use sibling bidi/selection/cache helpers rather than duplicate their policies.
- Clone the cached base face before applying axes. Derived variant axes are merged with explicit style variations, explicit values winning by tag; preserve nonvariable/unknown-axis no-op behavior.
- Configure each width-measurement and emission rustybuzz buffer with the explicit TextStyle.lang when present, using its normalized language code. Leave language unset when absent, preserving the existing inferred/default behavior. Pass the same optional explicit language identity to the run-cache owner. Set direction per bidi run; preserve language, offsets, glyph ids/advances and clamped positive units_per_em. Each emitted TextShaped.text is the sub-run text and glyph clusters are UTF-8 byte offsets relative to that text, with replacement character fallback for an invalid mapping. BidiRun.byte_start is a separate original-input coordinate owned by the bidi child. Whitespace-only input takes the unchanged-Text path; traversal owns the rule against reshaping TextShaped.
- Measure multiline width as the maximum paragraph width. Emission resets x for each paragraph and advances y by the first primary's resolved top/bottom edges plus style leading (default 0.65em). This does not repair L1 vertical flow/pagination for an embedded newline.
- Preserve the existing distinct measurement/emission feature and tracking paths rather than harmonize them speculatively. In emission calculate the rounded i32 tracking increment once per candidate from resolved tracking points, positive units_per_em and style.size, using the existing conversion formula. Pass that same increment to the cache key and add it only before a following different cluster, never inside a cluster or after the last glyph of a sub-run; negative tracking remains valid.
- Math mode is decided by style.math. Consume the canonical ssty_eligible_text from font_metrics; request levels 1/2 only for eligible Script/ScriptScript outside math_text_item. TextItem keeps math family/target features and every non-ssty property but no explicit ssty feature (`None`), also represented as absence in its cache key.
- Preserve forced math script behavior independently of requesting ssty and existing default OpenType features. No family/codepoint/fixture exception, speculative feature change, vertical shaping or nonzero face-index support is introduced.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## SHY autoral — controle de glifos invisíveis

### Medição anterior à decisão

`00_nucleo/diagnosticos/shy-autoral-2026-09-28/first-round/after/`
refuta a primeira materialização: trailing e ligature conservam os codepoints
mas ampliam a caixa de ActualText em 3.12pt e perdem o espaço na extração comum.
O caso tracking também inclui um intervalo derivado do placeholder invisível.
`first-round/final-receipt.json` conserva o estado e as falhas.
Rustybuzz 0.20.1, src/lib.rs:32-35, documenta que o tratamento padrão troca
Default_Ignorable por glifo de espaço com avanço zero; REMOVE_DEFAULT_IGNORABLES
remove esse placeholder depois do shaping. Um cluster compartilhado de f+SHY+fi
também pode dar hex vazio ao GID de espaço, bloqueando seu mapeamento posterior.
A decisão é refutável por alteração de glyphs visíveis/ligaduras no controle
com a flag documentada ou por contaminação de runs sem SHY.

### Responsabilidade

Configurar REMOVE_DEFAULT_IGNORABLES somente nos sub-runs cujo texto autoral
contém U+00AD, em medição e emissão. Delegar a configuração a text_identity,
antes de shape; não remover SHY do texto de entrada, não filtrar por GID
ou avanço zero e não alterar shaping de runs sem SHY. Os caracteres lógicos
completos continuam na ocorrência. A flag remove placeholders, não glyphs
visíveis de seleção de quebra. Tracking incide nos clusters que restam e
não acrescenta intervalo por um placeholder que não existe mais.
Cache permanece por texto completo; a política é determinística pelo texto.
Não mudar assinatura pública, fase, fallback, features OpenType ou fontes.
Texto exclusivamente invisível e seleção de quebras continuam fora do escopo.

## Correção de tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/tracking-residual-2026-09-29/initial.json`, `results.json`, `summary.json` e `causal-checks.json` registram 36 entradas/108 PDFs contra vanilla pinado, fontes, UTC, HEAD e working tree completa. A palavra isolada acumula +0.052pt com tracking 0.2pt, mas fronteiras perdem um intervalo na adjacência, dois com um espaço e quatro com três espaços literais. ±1.2pt elimina a quantização sem eliminar as fronteiras. O código shaper/shaping.rs:225-232,319-326 arredonda em i32; text.rs:49-83 divide palavras; space.rs:15-33 aplica somente avanço nominal. O estado anterior à materialização está em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29/initial.json`.

### Decisão interna e responsabilidade

Correção interna de paridade em fluxo contínuo ADR-0127, sem contrato público, flag ou fase nova. As cláusulas desta rodada supersedem somente a preservação dos limites de tracking descritos abaixo; demais políticas de render permanecem.

Conservar representação/cache do tracking i32. Ao posicionar o próximo sub-run, usar width derivada pelo owner tracking, que acrescenta somente o resíduo fracionário nos pares de clusters elegíveis. Não alterar glyph IDs, texto, features ou seleção de fontes. O measured shaped_width sem tracking mantém seu contrato.

Aceitação: testes RED→GREEN de fronteiras/seleção e interpretação independente Tc/TJ, controles bilaterais de tracking ±/zero, repetidos, adjacência e wrap; build e lint sem erros, resselo restrito.
