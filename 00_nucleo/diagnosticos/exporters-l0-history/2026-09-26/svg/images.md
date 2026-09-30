---
Hash do Código: 9084a712
prompt: infra/export/svg/images
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130
---

# Prompt L0 — infra/export/svg/images

## Medição anterior à decisão

Fonte anterior `03_infra/src/export/svg.rs:1304-1473`, HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, capturada em `2026-09-26T09:29:07-03:00`. Proveniência exata: `00_nucleo/diagnosticos/svg-atomization.md`. O texto histórico conserva medições e gates anteriores; não é nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo `03_infra/src/export/svg/images.rs`. Possuir identificação conservadora de SVG autossuficiente, MIME, data URL, payload raster, geometria, orientação EXIF, clip local de imagem e marcadores de fallback. Preservar os bytes, caixa resolvida e transform único. Não compartilhar a lógica com o exporter PDF nem introduzir I/O, decoder, recodificação ou novo suporte de formato.

Atomização estrutural reversível em L3: fluxo contínuo ADR-0127, sem mudança de contrato público, comportamento por defeito, fase ou compatibilidade. Referências ao monólito no texto histórico abaixo indicam a fonte anterior; a implementação específica passa a pertencer somente a este consumer. Não ampliar alegações de paridade nem interpretar mecanicamente novos IDs/bytes como paridade de linguagem.

### P1248 — imagens web, SVG embutido e fallback observável

Medição anterior à decisão: `FrameItem::Image` já entrega a este owner bytes,
posição, dimensões resolvidas, `clip_rect` de cover e orientação EXIF. Em
`03_infra/src/export/svg.rs`, o braço de imagem consumia apenas posição,
bytes, largura, altura e orientação (esta última ignorada); `render_image`
embutia PNG/JPEG/GIF/WebP, fixava `preserveAspectRatio="none"` e retornava sem
nó para formato `Unknown`. A referência ratificada prepara formatos web,
incluindo `ImageKind::Svg`, e emite data URL; o layout já decide fit/box antes
do exporter. Logo a obrigação é preservar a morfologia visual e os carriers
resolvidos, não copiar `WebImage`, bytes base64 ou estrutura Rust.

Decisão interna de paridade (fluxo contínuo ADR-0127):

- PNG, JPEG, GIF e WebP continuam embutidos com MIME coerente, data URL e
  caixa física exata. Alpha permanece nos bytes originais; não recodificar.
- Bytes cujo primeiro elemento XML significativo é `<svg` são embutidos como
  `image/svg+xml`. A identificação é local a L3 e conservadora: BOM UTF-8,
  whitespace, declaração XML e comentários iniciais podem ser ignorados;
  qualquer construção ambígua, comprimida ou sem raiz SVG fica `Unknown`.
  Este owner não altera o enum público `ImageFormat` nem promete parser SVG.
- O SVG aninhado permanece um recurso de imagem; não se reescrevem IDs,
  scripts, texto, formas ou alpha internos. A preservação alegada limita-se a
  um SVG finito e autossuficiente demonstrado por landmarks visuais.
- `clip_rect` finito emite clip local no espaço da página e recorta cover sem
  assar novamente posição ou orientação. Carrier ausente/malformado é
  `Unknown`, nunca ausência silenciosamente equivalente.
- Orientação EXIF 1–8 é aplicada exatamente uma vez por transformação SVG
  sobre a caixa já resolvida pelo layout. A ortografia da matriz não é
  observável; marcadores assimétricos discriminam ignorar e aplicar duas vezes.
- Formato desconhecido em `FrameItem::Image` emite um marcador diagnóstico
  não visual `data-crystalline-image-fallback="unknown-format"`; não emite
  `<image>`, não inventa MIME e permanece `Unknown`.
- Fit/aspect ratio não é redescoberto no exporter: as dimensões e posição do
  carrier já representam stretch/contain/cover; `preserveAspectRatio="none"`
  apenas aplica essa transformação resolvida. O exporter não recalcula layout.

Testes do owner cobrem os quatro MIME raster, alpha conservado no payload,
caixa/posição, cover clip, orientação assimétrica, SVG aninhado vermelho/azul
com alpha, desconhecido marcado e repetição determinística. Aceitação é
morfologia/semântica; igualdade byte/base64 serve somente para provar
determinismo do mesmo input, nunca paridade com o vanilla.

