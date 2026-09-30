# L0 — builder/fonts
Hash do Código: ed4d5f42

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/fonts.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/bitmap-embedding.toml sha256:016908bda7174f00ff63a909070553e9728d3d44c7ebf17b9616b4f676ad4def
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

## Consumer evidence

`03_infra/src/export/builder/fonts.rs:51` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Font preparation and construction

Own CIDFont/multifont build paths, font usage collection, subsetting and variable
instancing, descriptors, widths, ToUnicode, embedded font streams and CIDSet.
Font identity includes FontList, FontVariant and FontVariations. Merge derived and
explicit axes using the existing precedence; do not extend the single-font variable
instancing scope. Accumulate only the existing subsetting timing into subset_ms.

Collect shaped cluster text once before the per-font loop. Include codepoints and
glyphs from fallback Text, including spaces, in each applicable subset. ToUnicode
must retain shaped clusters and ligatures even when subsetting falls back to full
embedding; remap GIDs only when a subset mapping exists. Prepare the per-font reverse
map for direct mathematical glyphs; do not hardcode their resource to F1.

Compute glyph_to_nominal from the same effective face and axes as /W. Use variable
advances only when instancing succeeded; a default-face fallback uses default-face
advances consistently on both sides of the TJ boundary.

Emit Type0 Identity-H fonts with the correct descendant and stream representation:
TrueType uses CIDFontType2 and FontFile2/SFNT; CFF1 uses CIDFontType0 and
FontFile3/CIDFontType0C with the extracted CFF table; CFF2 uses CIDFontType0 and
FontFile3/OpenType with the complete container. Preserve descriptor metrics and
mapping semantics. Resolve BaseFont/FontName from PostScript name, then full name,
then family, then fallback; remove spaces and use the deterministic AAAAAA subset
prefix without renaming internal Fn resources.

Retain the 256 KiB font-stream compression cutoff and existing raw-data fallback
on compression failure. Every embedded CID font has a CIDSet: set MSB-first bits
for the effective CIDs after subset remapping, or used original CIDs for full
embedding, and allocate its stream outside reserved IDs. Bitmap-only fonts do not
emit an outline font or CIDSet; mixed fonts retain outline embedding for those glyphs
that need it. Prepare bitmap references under the pinned bitmap-embedding nucleus;
resource dictionary merging belongs to the resources child, and painting to the
text child. Unsupported bitmap formats retain the normal font path.

## Verification

Exercise TrueType, CFF1/CFF2, variable fallback, multifont identity, sparse/remapped
CIDSet, ligatures, spaces and bitmap-only/mixed fonts. Verify extracted text and
glyph morphology, not equivalence to an external object-allocation algorithm.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/builder/fonts.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Incluir o glyph de U+0020 na face correspondente a cada carrier no subset multifont, usando a resolução existente de estilo.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Geometria auto de separadores literais — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-auto-espacos-2026-09-29/before/results.json`, `provenance.json` e `initial.json` medem 30 fontes/60 PDFs, font-path, comandos, UTC, HEAD, working tree e binários contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Só espaços literais (`#text("   ")`) com tracking 0/±0.2 produzem no Tekt `/MediaBox [0 0 0 0]` (Poppler apresenta 612×792), enquanto o vanilla mede respectivamente 9.36/9.76/8.96×8.568pt. Separadores literais finais após texto perdem a sua largura em páginas auto; os iniciais seguidos de texto já coincidem. Escopo com tamanho 18pt termina em 14.44×12.852pt vanilla para +0.2pt. Texto vazio e markup sem tinta revelam outra falta de baseline/mínimo; newline final tem divergência vertical própria e não é fechado por esta fatia.

Causa fonte: `space.rs:22-35` não transporta Literal inicial sem tracking; `page_geometry.rs:104-135` usa `helpers::line_content_right` que filtra todos os TextSeparator, por necessidade correta do alinhamento RTL, e a altura de linha só com carrier não recebe topo tipográfico; `builder.rs:291` e `builder/fonts.rs:633,1167` escrevem MediaBox zero. Vanilla `crates/typst-pdf/src/convert.rs:108-118` impõe mínimo físico de 3×3 na saída PDF. Isso justifica projeção auto separada do alinhamento RTL e mínimo compartilhado entre os três caminhos exportadores. Não inferir intenção histórica do escopo antigo.

### Contrato interno da correção

Nos caminhos CIDFont único e multifont, aplicar ao MediaBox o mesmo helper privado do builder para mínimo físico de 3pt por eixo. Preservar fonte/subsetting, operadores, texto, coordenadas e demais dimensões. Não duplicar a constante nem atribuir a correção de largura lógica ao exportador.

Correção interna de paridade em fluxo contínuo ADR-0127, sem campo público, assinatura/flag/fase nova. Aceitação RED→GREEN para espaços 0/± e PDF mínimo, casos causais bilaterais, corpus anterior, build e lint sem erros, resselo estritamente no manifesto. Texto vazio explícito e newline final permanecem para rodada própria.
