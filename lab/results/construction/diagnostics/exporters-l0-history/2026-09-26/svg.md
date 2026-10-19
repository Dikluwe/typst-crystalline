---
Hash do Código: 1fc03204
prompt: infra/export/svg
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130
---

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c

# Prompt L0 — infra/export/svg

## Medição anterior à decisão

Fonte anterior `03_infra/src/export/svg.rs:42-50, 90-98, 833-1147`, HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, capturada em `2026-09-26T09:29:07-03:00`. Proveniência exata: `00_nucleo/diagnosticos/svg-atomization.md`. O texto histórico conserva medições e gates anteriores; não é nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo `03_infra/src/export/svg.rs`. Fachada com SvgOptions, defaults compact/sem bleed, aliases FontKey/GlyphFontIdentity, reexports compatíveis dos contextos e todos os entrypoints existentes. Possuir o writer raiz, canvas/viewBox, tradução de bleed, ordem background/body/foreground, emissão dos defs e dispatcher nominal exaustivo FrameItem. Delegar desenho a descendentes privados em L3; Semantic continua transparente. HTML frame continua a usar os mesmos writers, sem reescrita de paints. Não alterar assinaturas, flags, opções ou público do módulo.

Atomização estrutural reversível em L3: fluxo contínuo ADR-0127, sem mudança de contrato público, comportamento por defeito, fase ou compatibilidade. Referências ao monólito no texto histórico abaixo indicam a fonte anterior; a implementação específica passa a pertencer somente a este consumer. Não ampliar alegações de paridade nem interpretar mecanicamente novos IDs/bytes como paridade de linguagem.

## Owners descendentes

- `03_infra/src/export/svg/paints.rs`: `00_nucleo/prompts/infra/export/svg/paints.md`.
- `03_infra/src/export/svg/glyphs.rs`: `00_nucleo/prompts/infra/export/svg/glyphs.md`.
- `03_infra/src/export/svg/images.rs`: `00_nucleo/prompts/infra/export/svg/images.md`.
- `03_infra/src/export/svg/geometry.rs`: `00_nucleo/prompts/infra/export/svg/geometry.md`.
- `03_infra/src/export/svg/groups.rs`: `00_nucleo/prompts/infra/export/svg/groups.md`.
- `03_infra/src/export/svg/navigation.rs`: `00_nucleo/prompts/infra/export/svg/navigation.md`.
- `03_infra/src/export/svg/serialization.rs`: `00_nucleo/prompts/infra/export/svg/serialization.md`.
- `03_infra/src/export/svg/tests.rs`: `00_nucleo/prompts/infra/export/svg/tests.md`.
- `03_infra/src/export/svg/p1360_ab_tests.rs`: `00_nucleo/prompts/infra/export/svg/p1360_ab_tests.md`.

## P1140.20.2 — canvas SVG

render_bleed inclui bleed e translada TrimBox; sem ele recorta. Fill auto é
branco, none transparente, Paint pelo consumer. Ordem: background/body/foreground.

**Camada**: L3  
**Ficheiro alvo**: `03_infra/src/export/svg.rs`  
**Origem**: P870  
**ADRs**: ADR-0033 (paridade observable)

---


## Contexto

Exportador de páginas Typst para SVG. Recebe uma `Page` cristalina e devolve uma string SVG. Baseia-se na estrutura do `typst-svg` do vanilla (`lab/typst-original/crates/typst-svg/`), mas adaptado aos tipos cristalinos.


## Interface pública

```rust
pub struct SvgOptions {
    /// Se true, formata o SVG com indentação.
    pub pretty: bool,
    /// Se true, inclui o canvas físico; false recorta ao TrimBox.
    pub render_bleed: bool,
}

impl Default for SvgOptions { ... }

/// Exporta uma página para SVG (texto sem fontes resolvidas → scope-out/tofu).
pub fn export_svg(page: &Page, opts: &SvgOptions) -> String;

/// Exporta uma página para SVG com fontes resolvidas.
pub fn export_svg_with_fonts(
    page: &Page,
    opts: &SvgOptions,
    fonts: &[((FontList, FontVariant, FontVariations), Vec<u8>)],
) -> String;
```


## Dependências

- `xmlwriter` — geração de XML.
- `base64` — embutir imagens no SVG.
- `itoa` / `ryu` — serialização de inteiros/floats.
- `ttf-parser` — extração de outlines de glifos para texto como path (P871).
- `flate2` — já em `03_infra/Cargo.toml`; compressão de imagens embutidas se necessário.


## Estratégia de porte

- Copiar a estrutura de `typst-svg/src/lib.rs` (`SVGRenderer`, `State`, render recursivo).
- Mapear `FrameItem` cristalino para elementos SVG:
  - `TextShaped` → glifos como paths (`<symbol>` + `<use>`), replicando o vanilla.
  - `Shape` → `<path>`, `<rect>`, `<circle>`, etc.
  - `Image` → `<image>` com `href="data:..."`.
  - `Group` → `<g transform="...">` com recursão.
  - `Line` → `<line>`.
  - `Link` → `<a>` envolvendo filhos.
- Texto (P871): para cada `ShapedGlyph`, extrair o outline via
  `ttf_parser::Face::outline_glyph`, construir um path SVG relativo e
  emitir `<defs><symbol id="..."><path d="..."/></symbol></defs>`;
  cada ocorrência do glifo referencia o símbolo com `<use xlink:href="#...">`.
  O grupo de texto aplica `matrix(1 0 0 -1 x y)` para inverter o eixo Y
  (fontes usam Y-up; SVG usa Y-down).


## Restrições

- L3 — sem I/O directo; recebe `Page` e devolve `String`.
- Usar tipos cristalinos.
- SVG standalone válido e variante intrínseca `html.frame` aprovada em P1424.
  Routing de bundle/cross-page permanece fora do owner.

## P1360 — ativação produtiva de `SvgOptions.pretty`

O campo público `SvgOptions.pretty` já controla o `XmlWriter`: false usa
`Indent::None`; true usa dois espaços. P1360 não altera este exporter nem sua
API; torna obrigatório que a pipeline produtiva deixe de fixar
`SvgOptions::default()` e encaminhe a escolha recebida.

Pretty altera somente whitespace XML. Grafo, IDs, atributos, escapes,
geometria, ordem de pintura, fontes, destinos e fallback permanecem iguais.
Aceitação compara árvore/morfologia e presença de indentação, não bytes.


## P1140.5-A — SVG visualmente transparente

SVG recursa nos filhos de `FrameItem::Semantic` sem desenhar `alt`. Expor
acessibilidade SVG requer medição própria; nesta fase o contrato é preservar o
render e não transformar a descrição de PDF em `<text>` ou tooltip inventado.


## P1424 — variante SVG embutida em HTML (APROVADO NO GATE ADR-0127 EM 2026-09-16)

### Medição anterior à decisão

O renderer standalone acrescenta canvas/fundo de página. O vanilla
`svg_in_html` recebe um frame intrínseco e emite raiz SVG transparente com
`viewBox`, dimensões pt e estilo proporcional ao tamanho de texto; usar o
standalone como string e recortá-lo no exporter HTML perderia ownership e
misturaria sintaxe com geometria.

### Contrato proposto

Adicionar entry point L3 reutilizando o renderer e contextos de fontes já
existentes para produzir SVG inline de uma página/frame intrínseco: sem fundo
de página, com `overflow: visible`, largura/altura físicas, `viewBox` coerente
e `display: block` no recorte P1424. O fator em `em` deriva do tamanho de texto
efetivo fornecido pela pipeline. A geometria, paint e texto usam os mesmos
writers do SVG standalone; não duplicar desenho no exporter HTML.

IDs internos, ordem mecânica de atributos e bytes não são paridade. São
observáveis a presença de SVG inline, transparência, dimensões, viewBox,
display e morfologia visual suportada. A nova entry point e integração foram
confirmadas pelo dono em 2026-09-16 e ficam limitadas pelo escopo de P1424.

### P1454 — paint atravessa `html.frame` sem reescrita

#### Medição anterior à decisão

No baseline `3267d9083afef6a5b2a493221e08714e402ac7b8`, a paleta pública
completa de 18 cores produziu os mesmos valores `to-hex()` no vanilla
ratificado e no cristalino. Em particular, `red.to-hex()` resultou em
`"#ff4136"` nos dois produtos. Depois do layout, porém,
`export_svg_html_frame_with_fonts` aplicava uma substituição textual global de
`#ff4136` por `#ff0000` apenas ao SVG embutido. Portanto, a diferença observada
em P1453 não pertence à tabela de cores nem ao frame: era corrupção de paint
na fronteira de serialização L3.

#### Decisão

O SVG embutido preserva exatamente a cor transportada pelos mesmos writers do
SVG standalone. Não reinterpreta nomes, não traduz paletas e não substitui
literais hexadecimais no resultado serializado. Assim, tanto a constante
`red` quanto um `rgb("#ff4136")` autoral permanecem `#ff4136`; uma cor
explicitamente `rgb("#ff0000")` permanece `#ff0000`.

A correção remove somente a pós-transformação textual. Não altera assinatura,
tipo público, default, fase de pipeline, conjunto de cores nem formato geral
do SVG. É correção interna de paridade em fluxo contínuo conforme ADR-0127.


## Compatibilidade pública atual

Além da interface histórica acima, manter render_bleed, FontKey, GlyphFontIdentity, GlyphFontRequest, SvgGlyphFontContext, SvgDestinationContext e variantes export_svg_with_context, export_svg_with_fonts_and_context, export_svg_with_fonts_and_contexts, export_svg_html_frame_with_fonts e SvgOptions::export_html_frame_with_fonts, com assinaturas exatas anteriores. Os tipos reexportados não expõem campos antes privados.
