# L0 — builder
Hash do Código: c1a5f9e0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

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

`lab/results/construction/diagnostics/correcao-auto-espacos-2026-09-29/before/results.json`, `provenance.json` e `initial.json` medem 30 fontes/60 PDFs, font-path, comandos, UTC, HEAD, working tree e binários contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Só espaços literais (`#text("   ")`) com tracking 0/±0.2 produzem no Tekt `/MediaBox [0 0 0 0]` (Poppler apresenta 612×792), enquanto o vanilla mede respectivamente 9.36/9.76/8.96×8.568pt. Separadores literais finais após texto perdem a sua largura em páginas auto; os iniciais seguidos de texto já coincidem. Escopo com tamanho 18pt termina em 14.44×12.852pt vanilla para +0.2pt. Texto vazio e markup sem tinta revelam outra falta de baseline/mínimo; newline final tem divergência vertical própria e não é fechado por esta fatia.

Causa fonte: `space.rs:22-35` não transporta Literal inicial sem tracking; `page_geometry.rs:104-135` usa `helpers::line_content_right` que filtra todos os TextSeparator, por necessidade correta do alinhamento RTL, e a altura de linha só com carrier não recebe topo tipográfico; `builder.rs:291` e `builder/fonts.rs:633,1167` escrevem MediaBox zero. Vanilla `crates/typst-pdf/src/convert.rs:108-118` impõe mínimo físico de 3×3 na saída PDF. Isso justifica projeção auto separada do alinhamento RTL e mínimo compartilhado entre os três caminhos exportadores. Não inferir intenção histórica do escopo antigo.

### Contrato interno da correção

Centralizar em helper privado de builder o mínimo físico de página PDF 3pt por eixo, como medido no vanilla pinado em convert.rs:108-118. Aplicar depois de somar bleed ao canvas nos caminhos Helvetica; sem mudar Page lógica, TrimBox, extensão acima do mínimo, seleção de páginas, versões/modes. Os caminhos de fontes usam o mesmo helper, sem copiar a constante. Este 3pt vem da regra do oráculo PDF pinado, não de compensação tipográfica ad hoc.

Correção interna de paridade em fluxo contínuo ADR-0127, sem campo público, assinatura/flag/fase nova. Aceitação RED→GREEN para espaços 0/± e PDF mínimo, casos causais bilaterais, corpus anterior, build e lint sem erros, resselo estritamente no manifesto. Texto vazio explícito e newline final permanecem para rodada própria.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.


## Fast path de face única certificada — 2026-10-05

Medição anterior: first-benchmark.json registra regressão em casos de face única. Quando os bindings certificados contêm uma só fonte e não requerem instanciação VF, usar build_cidfont com essa fonte, preservando o mesmo domínio de GIDs. Pedidos VF de eixos continuam no multifont; sem carrier certificado manter dispatch anterior. Isto reutiliza um caminho existente na mesma fase/API, sem nome artificial nem seleção por estilo. Validar regressões de fonte única e mistura real.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Integrar registro e emissão privada de tiling PatternType 1 no caminho Type1, antes da emissão final dos streams; reservar IDs após gradientes e antes dos demais recursos. Compartilhar fontes/imagens/spot e referências de patterns, mantendo seleção de páginas e tagging intactos.

### Aceitação e continuidade

Aplicar o núcleo pinado. A ratificação supersede cláusulas antigas que
aguardavam aprovação ou exigiam Paint/Stroke declarativo dentro de Shape,
somente no âmbito do contrato aprovado. Preservar os demais comportamentos
vigentes desta unidade. L0 primeiro e testes RED→GREEN; migração explícita de
fixtures e consumidores, sem conversão total From<Paint>, sem PartialEq nos
carriers e sem fallback sólido para obter build verde. Não elevar o limite
antigo nem transferir layout para L3. Validar transporte fill/stroke, recursos,
clipping, Parent afim, alvos nativos, controles Solid/Gradient, semântica de
página e toda a suíte/build/doc/fmt/linhagem. T1c, T2 e T3 só estão concluídos
quando integrados e verificados; estado intermediário quebrado ou parcial
não é certificação de produção. T4 e publicação continuam fora deste pacote.
