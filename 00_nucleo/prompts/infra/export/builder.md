# L0 — builder
Hash do Código: 5dba4d6b

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder.rs:45` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Façade contract

Own PdfBuilder, its private shared state, constructors and internal option setters,
the build entrypoint, and the Helvetica fallback orchestration. Preserve existing
signatures, visibility and defaults. Return PDF bytes and accumulated subsetting time.
Parse supplied font data for the CIDFont path; otherwise use Helvetica. Dispatch
CIDFont and multifont construction to the fonts child without moving their algorithms
into this façade. StreamMode, PdfSerializationMode, PdfTags and resolved PdfStandards
remain independent inputs; this module does not redefine their owning contracts.

Normalize an empty input document to one blank A4 page before dispatch, preserving
document metadata and attachments. Page filtering is supplied by the navigation
child. Keep catalog/pages allocation, shared object-ID coordination and assembly of
page dictionaries here; descendant emitters use that state without collisions.
Page geometry uses the physical canvas including bleed, with a nonzero TrimBox
describing the logical page; omit the optional TrimBox for zero bleed.

Compose fonts, resource dictionaries, page operators, physical stream envelopes,
navigation, metadata and final serialization through their owners below. The façade
must not duplicate their normative algorithms. Private descendants remain in L3;
do not introduce dynamic dispatch, public APIs or a pipeline phase change.

## Delegated owners

- `00_nucleo/prompts/infra/export/builder/fonts.md` owns `03_infra/src/export/builder/fonts.rs`.
- `00_nucleo/prompts/infra/export/builder/metadata.md` owns `03_infra/src/export/builder/metadata.rs`.
- `00_nucleo/prompts/infra/export/builder/navigation.md` owns `03_infra/src/export/builder/navigation.rs`.
- `00_nucleo/prompts/infra/export/builder/resources.md` owns `03_infra/src/export/builder/resources.rs`.
- `00_nucleo/prompts/infra/export/builder/serialization.md` owns `03_infra/src/export/builder/serialization.rs`.
- `00_nucleo/prompts/infra/export/builder/streams.md` owns `03_infra/src/export/builder/streams.rs`.

## Geometria auto de separadores literais — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-auto-espacos-2026-09-29/before/results.json`, `provenance.json` e `initial.json` medem 30 fontes/60 PDFs, font-path, comandos, UTC, HEAD, working tree e binários contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Só espaços literais (`#text("   ")`) com tracking 0/±0.2 produzem no Tekt `/MediaBox [0 0 0 0]` (Poppler apresenta 612×792), enquanto o vanilla mede respectivamente 9.36/9.76/8.96×8.568pt. Separadores literais finais após texto perdem a sua largura em páginas auto; os iniciais seguidos de texto já coincidem. Escopo com tamanho 18pt termina em 14.44×12.852pt vanilla para +0.2pt. Texto vazio e markup sem tinta revelam outra falta de baseline/mínimo; newline final tem divergência vertical própria e não é fechado por esta fatia.

Causa fonte: `space.rs:22-35` não transporta Literal inicial sem tracking; `page_geometry.rs:104-135` usa `helpers::line_content_right` que filtra todos os TextSeparator, por necessidade correta do alinhamento RTL, e a altura de linha só com carrier não recebe topo tipográfico; `builder.rs:291` e `builder/fonts.rs:633,1167` escrevem MediaBox zero. Vanilla `crates/typst-pdf/src/convert.rs:108-118` impõe mínimo físico de 3×3 na saída PDF. Isso justifica projeção auto separada do alinhamento RTL e mínimo compartilhado entre os três caminhos exportadores. Não inferir intenção histórica do escopo antigo.

### Contrato interno da correção

Centralizar em helper privado de builder o mínimo físico de página PDF 3pt por eixo, como medido no vanilla pinado em convert.rs:108-118. Aplicar depois de somar bleed ao canvas nos caminhos Helvetica; sem mudar Page lógica, TrimBox, extensão acima do mínimo, seleção de páginas, versões/modes. Os caminhos de fontes usam o mesmo helper, sem copiar a constante. Este 3pt vem da regra do oráculo PDF pinado, não de compensação tipográfica ad hoc.

Correção interna de paridade em fluxo contínuo ADR-0127, sem campo público, assinatura/flag/fase nova. Aceitação RED→GREEN para espaços 0/± e PDF mínimo, casos causais bilaterais, corpus anterior, build e lint sem erros, resselo estritamente no manifesto. Texto vazio explícito e newline final permanecem para rodada própria.
