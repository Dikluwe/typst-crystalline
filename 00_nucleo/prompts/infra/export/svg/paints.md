# L0 — svg/paints
Hash do Código: e496e1b6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/paints.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/svg/paints.rs:19` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Paint servers and support boundary

Own PaintDefs/SvgPaintDef/PaintReference, support/fallback predicates, local definition
deduplication and linear/radial/conic/tiling/stops writers. References resolve within
the SVG graph and preserve their fill/stroke role. Definition reuse includes the
observable paint geometry; conic uses the finite positive item box rather than
discarding aspect ratio. Keep alpha separate from color and preserve stop order,
multiplicity, coincident offsets, discontinuities and a unique terminal endpoint.

Linear/Radial support sRGB, Oklab, LinearRgb, Oklch, Hsl and Hsv individually.
Use objectBoundingBox geometry, including the independent radial focal point.
sRGB stops retain the native numeric writer. Other supported spaces consume
svg_adaptive_stops from the adaptive-gradient owner without changing its 0.001
threshold or cap 64; serialize every adaptive offset directly from f64 using
Ratio::repr's two-decimal percentage with offset * 100 followed by percentage
rounding away from zero. Do not narrow to f32 or merge arithmetic operations.
Luma/CMYK and unvalidated spaces remain explicit gradient-color-space fallback.

Conic support is limited to sRGB, Oklab, LinearRgb and Hsv. Emit a vector pattern
of angular wedges referencing local two-stop linear gradients, preserving center,
initial angle, clockwise progression and reciprocal box-aspect correction. Do not
replace the graph by raster or disguised solid. Wedge count is not a parity rule.
For promoted combinations, interior premultiplied encoded-sRGB error is max <=4/255,
p95 <=2/255 and alpha max <=2/255; the complete local mask requires premultiplied
RGBA p95 <=4/255 and at most 0.5% of pixels above 16/255. Derive the mask from the
declared graph/geometry before comparison, including seams/boundaries/stroke; no
error-based crop, erosion, fitting or realignment. Preserve the measured side at
coincident boundaries and the 0/1 seam. Local paint failures are not external layout
Unknown; integration beyond that local geometry remains outside this owner.
Other conic spaces, including CMYK, keep conic-gradient fallback.

Tiling supports only resolved size plus Color body, retaining size/spacing/relative
semantics. Opaque/unresolved tiling keeps tiling-content-or-size fallback. Unsupported
paint retains its existing first-color geometry-preserving fallback with the correct
data-crystalline-fill-fallback or stroke marker and Unknown classification. Do not
promote unsupported pairs by wildcard or add ICC/downloads/decoders here.

## Verification

Exercise each supported pair/role, negative spaces, references, focal/aspect geometry,
stop discontinuities/alpha, adaptive representation, conic local error budgets and
tiling support. Do not replace the existing validated numerical envelopes with
stop counts, IDs or XML bytes as parity criteria.

## Cálculos numéricos dos exportadores — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/svg/paints.rs` nas linhas [165, 166, 259, 260, 299, 300, 301, 302, 327, 328, 329, 330] usa métodos apontados pelo Clippy. `clippy.toml` exige alternativas sem dependência da matemática da plataforma; libm =0.2.11 já está pinado no core/Cargo.lock. Os fontes da biblioteca, consumer, owner, manifest/lock, spans, HEAD, working tree não commitado, UTC, status e diffstat estão em `00_nucleo/diagnosticos/gradientes-numericos-ci-2026-10-13/before.json.gz`.

`before-renders.json` registra 24 fontes renderizadas em PDF/SVG por Tekt e pelo vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, com hashes dos binários e comandos. `before-comparison.json` mede o canvas inteiro sem máscaras ajustadas pelo erro. Existem divergências anteriores; o corpus não é prova de paridade integral. A primeira tentativa de fixture usava hsv fora do namespace color, foi rejeitada pelo parser antes de qualquer mudança produtiva e está preservada em initial-fixture-error.log; cases.json contém as entradas válidas.

### Decisão e aceitação

Usar libm::sin, libm::cos e libm::atan2 de libm 0.2.11, todos f64, nas geometrias linear e cônica. Conservar a ordem y/x de atan2, a negação depois de rem_euclid, correção de aspecto e semântica de centro, wedges, stops e fallback. Não mudar thresholds, espaços promovidos, máscaras ou envelopes visuais.

Dependência direta interna de L3 fixada em =0.2.11 no manifest, reutilizando a versão já no lock. Não atualizar versões de pacotes nem adicionar I/O. Mudança interna em fluxo contínuo ADR-0127, sem contrato público, default, fase ou estratégia novos. L0 precede código e resselo é limitado aos owners envolvidos.

Validar as fórmulas com a suíte existente, exportação dos mesmos 24 documentos antes/depois e leitura raster/estrutural de PDF/SVG. Comparar a preservação local separadamente das divergências contra o pin vanilla; nenhuma diferença nova de render é aceita sem análise. Não alargar tolerâncias/limites, mudar fixtures ou supressões para esconder uma falha. Compilar todos os targets, executar a suíte infra, verificar build, Clippy, Rustdoc, formato e linhagem. Contagens e identidade de arquivos não certificam a paridade da linguagem.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Armazenar FramePaint e a matriz de placement em definições privadas. Deduplicar Solid/Gradient por seus dados e Tiling por identidade privada da célula mais matriz, nunca serializar endereço. Emitir tiling como pattern userSpaceOnUse com uma célula vetorial recortada a frame.size, width/height iguais ao pitch, overflow visível para spacing negativo, e patternTransform calculada pelo helper afim. Enfileirar definições aninhadas durante a emissão sem invalidar referências; usar fonte/glyph/clip IDs do documento, raiz local e modo decorativo dentro da célula. Não inferir equivalência de linguagem pela identidade usada no cache. O suporte/fallback de gradientes vigente é preservado; tiling preparado não tem fallback sólido. Testar densidade, clipping, Self/Parent, pivô, período grande, overlap e recursos aninhados.

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

## Higiene da migração nativa

Medição: verified-v3/followup-clippy-inventory.json e followup-architecture.log, com candidate-final e workspace-pre próprios, registram quatro f32::hypot proibidos, um argumento adicional, dois itens produtivos depois de módulos de teste e dois wildcards neutros novos.

Decisão interna em fluxo contínuo (ADR-0127): O comparador privado de cache enumera os seis pares de variantes distintas como false, sem wildcard que aceite silenciosamente futuras variantes. Identidade/igualdade do cache não é igualdade de linguagem. Não elevar limites nem adicionar allow. Validar suíte e renders antes/depois, além de formato, doc e linhagem.

## Composição de período e alpha sólido — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/tiling-overlap-2026-10-14/before.json.gz` preserva fontes, owners, HEAD, UTC e working tree. `tiling-nativo-2026-10-14/final-renders` e `remaining-analysis.md` registram cell8×8/pitch4×4 alpha50%: PDF vermelho opaco; SVG só uma contribuição; PNG quatro contribuições (255,15,15). `pdf_tiling.rs:184` emite BBox=size, `svg/paints.rs:148` usa overflow visible; `stream/shape.rs:155` e `stream/text.rs:275` descartam alpha. São defeitos observáveis de composição; o vanilla também corta overlap e não serve de requisito para esse defeito. Hipótese refutável: compor explicitamente um período preserva controles sem overlap e quatro contribuições coincidem com fonte-over analítica.

### Responsabilidade desta unidade

Guardar o corpo vetorial recortado uma vez num g identificado dentro de defs. Pattern com viewport pitch e overflow hidden contém use traduzido para cada contribuição do helper, preservando ordem. Não depender de overflow visible para compor vizinhos. Manter fila de patterns aninhados, glyphs/fonts/clip IDs únicos, raiz local e exclusão de links decorativos. Cláusula anterior sobre overflow visible fica substituída.

### Aceitação

Correção interna ADR-0127 dentro do T1–T3 já aprovado, sem nova API/fase/flag/default. L0 primeiro, RED→GREEN, composição analítica (incluindo ordem com cores distintas), controles simples/nested/rotated/text/image e extração/seleção de páginas. Build, suíte workspace, formato, Rustdoc e linhagem. Alpha variável de gradientes e qualidade PNG seguem incompletos; não certificar pacote de produção com esses itens abertos.


## Aridade final — materialização interna, 2026-10-16

### Medição anterior à decisão

Fontes e L0 vigentes conferidos antes do código; snapshot 00_nucleo/diagnosticos/clippy-aridade-final-2026-10-16/before.json.gz conserva HEAD, working tree não commitado, UTC, status, diffstat e hashes globais. callers.txt registra busca no workspace, incluindo fixtures; as chamadas homônimas do math::MathLayouter são outro motor e permanecem intactas. Inventário anterior tem 22 avisos, cinco destas aridades. Trata-se de estrutura privada Rust, sem nova afirmação de paridade ou intenção upstream.

### Responsabilidade exclusiva e aceitação

Migrar chamada do dispatcher de células de tiling para SvgRenderSession privado do owner 00_nucleo/prompts/infra/export/svg.md. Criar visão reemprestada dos mesmos fonts/glyph_defs/self/clip_next_id/glyph_fonts, com destinations vazio e html_frame false no escopo da célula; nenhuma cópia de caches ou contador novo. Manter identity/decorative true temporários e restaurar o par anterior após o empréstimo; manter fila de paints aninhados, ordem, clip, contribuição/período e assinatura de PaintDefs.write.

Este adendo materializa somente o escopo nominal e supersede seu estado pendente da auditoria. Fluxo contínuo ADR-0127: sem contrato externo, default, fase ou compatibilidade novos. L0 primeiro e resselo restrito. RED de too_many_arguments antes do código, GREEN sem novas supressões ou alteração de limiar; expectativas anteriores devem continuar satisfeitas. Conferir corpos e transporte de valores, preservar fixtures/esperados, sem testes que espelhem carriers. Executar controles existentes de Grid/Table/outline e SVG/tiling antes/depois, suíte workspace completa, build, Clippy all-targets, Rustdoc, formato, linhagem estrita/reversa e Tekt sem novos avisos, sob 12 GiB, swap zero e um job. Resultados não certificam paridade global ou conformidade PDF/UA.
