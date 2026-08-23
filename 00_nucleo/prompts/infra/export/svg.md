# Prompt L0 — `infra/export/svg` — Exportação SVG
Hash do Código: f578e730

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
- Foco em SVG standalone válido; `svg_in_bundle`/`svg_in_html` ficam fora do escopo.
- P1133: raios de `ShapeKind<Pt>::RoundedRect` chegam como `Corners<Pt>` já resolvidos.
  O SVG serializa esses pontos diretamente, sem `resolve_pt(0.0)` e sem
  projeção de parcela absoluta de `Length`.

## Critérios de verificação

- Documento "Hello" produz SVG parseável com texto renderizado como paths de glifo.
- Documento com formas produz elementos SVG correspondentes.
- Documento com imagem produz `<image>` base64.
- Estrutura SVG comparável à do vanilla (não byte-exact).
- SVG renderiza correctamente mesmo sem a fonte instalada no sistema.
- `radius: 1em` preserva o raio absoluto resolvido pelo layout.
