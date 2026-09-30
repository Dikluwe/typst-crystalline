# L0 — stream/text/runs
Hash do Código: 0f87b73c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/pdf-shaped-advances.toml sha256:d6da63fc3b0282aaeb2573e92e850f807a2691c04d3578c54e61c1f4e01a51f3
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/text/runs.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

Before extraction, `03_infra/src/export/stream/text.rs:621,655,696,767`
contained envelope compatibility, run-boundary selection, run emission and
rounded inter-item TJ cursor compensation. The source snapshot and tests are
preserved in `00_nucleo/diagnosticos/atomizacao-separadores-2026-09-28/`.
The bilateral separator probes in
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` reproduce a text
extraction divergence against vanilla pin
`9f2b6e8715237cb086899a42873660fe744622e8`. They do not establish total PDF
byte equality, general reader behavior or the earliest historical failing build.

## Ownership and contract

Own compatibility, boundaries and emission of coalesced shaped prose runs.
Receive FrameItems and the prepared FontScenario/PageContext from the ancestor.
Keep the existing static interface within export::stream; allocate no font
resources and perform no shaping, I/O, instancing or language evaluation.

Coalesce only consecutive shaped prose with equal baseline and complete
emission envelope: font identity and variations, size, fill, weight,
bold/italic, tracking, direction, math flags/size and units_per_em. Empty
glyphs, incompatible envelopes, nontext, group/link scopes, math and bitmap
leaves terminate or prevent runs. UA1 call-site policy belongs to the page
owner; this unit does not independently decide tagging policy.

Emit balanced verbose envelopes through the text owner's prefix/render-mode/
tracking helpers. Select the prepared single-font or multifont mappings by
complete style. Preserve the supplied nominal/actual advances, offsets and
old-to-new GIDs. Every inter-item TJ boundary compensates from the effectively
rounded cursor to the next absolute x position; include per-glyph rounding
in that cursor. Do not replace absolute positioning with nominal advances.

## Acceptance and limit

This extraction preserves current text, geometry, scope boundaries, diagnostics
and rendering operators. Existing P979 and text-operator regressions verify
compatible grouping, envelope boundaries and rounded cursor positioning.
The missing separator provenance is unresolved by this structural extraction;
this L0 does not authorize inventing spaces from positive geometric gaps.

## Tracking shaped — soma única e estado PDF

### Medição anterior à decisão

`00_nucleo/diagnosticos/shy-autoral-2026-09-28/tracking-controls.json`
mede plain-verbose com desvio máximo 2.652pt contra vanilla pinado,
enquanto plain-compact tem 0.348pt. Estado, binários e comandos estão no
recibo daquela frente. O novo estado e a matriz estão em
`00_nucleo/diagnosticos/tracking-pdf-2026-09-28/initial.json` e `before/`.
`03_infra/src/shaper/shaping.rs:319-326` soma tracking aos avanços;
`03_infra/src/export/stream/text.rs:639,668` e
`03_infra/src/export/stream/text/runs.rs:107` emitem Tc novamente.
O cursor de runs considera os avanços e ajustes TJ, sem essa soma Tc.
O vanilla pinado aplica tracking nos glyphs em
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:1269-1292`.
É comportamento de espaçamento medido, não requisito de operador ou bytes.
A hipótese é refutável por avanços sem tracking ou ausência do Tc adicional.

### Decisão de paridade interna

TextShaped CIDFont/Multifont carrega os avanços autoritativos já ajustados:
tracking deve vir desses avanços somente. Tanto emissão single quanto
coalescida estabelece explicitamente Tc=0 dentro do bloco BT antes de TJ,
inclusive tracking ausente/zero, para eliminar estado herdado. O zero é
o estado normativo de espaçamento adicional, não uma exceção de fonte, GID,
fixture ou largura. Não subtrair tracking dos avanços nem inferir causa
pelo texto. Text e fallback Type1, que emitem Tj sem avanços shaped,
continuam usando seu tracking existente. Bitmap continua desenho de imagem.
Sem contrato público, flag, modo padrão ou fase nova: fluxo ADR-0127.
Arredondamento dos avanços, medição de layout e política de tracking em
ligaduras são frentes independentes; esta correção não os certifica.

### Responsabilidade exclusiva

Na emissão coalescida, usar push_shaped_spacing do owner ancestral em vez
de push_tc_if_tracking. O modelo de cursor continua sendo somente nominal,
avanço shaped e TJ arredondado. Preservar compatibilidade, boundaries e
compensação absoluta. Testes causais pela interface emit_verbose_text_run
ficam no consumer ancestral text, sem ownership compartilhado do código.

## Correção de tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/tracking-residual-2026-09-29/initial.json`, `results.json`, `summary.json` e `causal-checks.json` registram 36 entradas/108 PDFs contra vanilla pinado, fontes, UTC, HEAD e working tree completa. A palavra isolada acumula +0.052pt com tracking 0.2pt, mas fronteiras perdem um intervalo na adjacência, dois com um espaço e quatro com três espaços literais. ±1.2pt elimina a quantização sem eliminar as fronteiras. O código shaper/shaping.rs:225-232,319-326 arredonda em i32; text.rs:49-83 divide palavras; space.rs:15-33 aplica somente avanço nominal. O estado anterior à materialização está em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29/initial.json`.

### Decisão interna e responsabilidade

Correção interna de paridade em fluxo contínuo ADR-0127, sem contrato público, flag ou fase nova. As cláusulas desta rodada supersedem somente a preservação dos limites de tracking descritos abaixo; demais políticas de render permanecem.

No cursor coalescido, usar a mesma correção fracionária que a folha single. Fronteira TJ de run com tracking não zero usa cinco casas, compensando o cursor efetivamente serializado; outros runs preservam arredondamento inteiro. Conservar posições absolutas, nominal/GID/offset, compatibilidade e envelopes.

Aceitação: testes RED→GREEN de fronteiras/seleção e interpretação independente Tc/TJ, controles bilaterais de tracking ±/zero, repetidos, adjacência e wrap; build e lint sem erros, resselo restrito.
