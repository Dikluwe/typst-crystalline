# Relatório — typst-passo-870: PNG e SVG como formatos de saída reais

**Data:** 2026-07-23T18:20:27Z  
**Executor:** Kimi Code  
**Commit base:** `8506a5dec64b14e555b76e971e667d6a13609700` (HEAD do ramo `Tekt` após P869)  
**Ramo:** `Tekt`  

---

## 1. Resumo

O P870 implementa a rasterização PNG e a exportação SVG como formatos de saída reais no compilador cristalino. O CLI já não recusa PNG/SVG nem escreve PDF disfarçado — gera ficheiros válidos para ambos os formatos, usando a infraestrutura de renderização existente em L3 e os tipos cristalinos de `Page`/`FrameItem`.

A estratégia foi a **Opção 2** decidida em `00_nucleo/materialization/typst-passo-870.md`:
- funções simples `export_png` / `export_svg` para texto sem fontes resolvidas;
- variantes `_with_fonts` com chave `((FontList, FontVariant, FontVariations), Vec<u8>)`, espelhando o padrão já usado pelo PDF.

---

## 2. Ficheiros alterados / criados

| Ficheiro | Tipo | Notas |
|---|---|---|
| `00_nucleo/prompts/infra/export/render.md` | novo | L0 do rasterizador PNG |
| `00_nucleo/prompts/infra/export/svg.md` | novo | L0 do exportador SVG |
| `00_nucleo/prompts/infra/export/mod.md` | alterado | reexporta `export_png*`, `export_svg*`, `RenderOptions`, `SvgOptions` |
| `03_infra/src/export/render.rs` | novo | 658 linhas — rasterização PNG via `tiny-skia`/`pixglyph` |
| `03_infra/src/export/svg.rs` | novo | 507 linhas — exportação SVG via `xmlwriter` |
| `03_infra/src/export/mod.rs` | alterado | declara e reexporta os novos módulos |
| `03_infra/src/pipeline.rs` | alterado | adiciona `compile_to_png_bytes*` / `compile_to_svg_string*` com resolução de fontes |
| `04_wiring/src/main.rs` | alterado | CLI invoca PNG/SVG em vez de recusar |
| `04_wiring/tests/cli.rs` | alterado | substitui testes de recusa por testes de geração válida |
| `03_infra/Cargo.toml` | alterado | dependências `tiny-skia`, `resvg`, `pixglyph`, `bytemuck`, `xmlwriter`, `base64`, `itoa`, `ryu` |
| `Cargo.lock` | alterado | lockfile actualizado pelas novas dependências |

```text
 13 files changed, 2079 insertions(+), 141 deletions(-)
```

---

## 3. Decisões de arquitetura

### 3.1 Paridade com a linguagem, não com a mecânica (ADR-0107)

A implementação baseia-se na estrutura de `typst-render` e `typst-svg` do vanilla (`lab/typst-original/crates/typst-render/`, `lab/typst-original/crates/typst-svg/`), mas foi adaptada aos tipos cristalinos:
- `FrameItem` cristalino (incluindo `TextShaped`, `Text` legado, `Shape`, `Image`, `Link`, `Tag`, `Meta`).
- `ShapeKind`, `PathItem`, `Stroke`, `Color`, `Gradient`, `ImageFormat`.
- `Page`/`PagedDocument` com `size` e `fill`.

Não se pretendeu igualdade byte-a-byte com o vanilla; pretendeu-se que um documento Typst simples produza PNG/SVG semanticamente equivalente (página A4, texto posicionado, formas visíveis).

### 3.2 Fontes: duas variantes de API

Seguindo o padrão do PDF:
- `export_png(page, opts)` / `export_svg(page, opts)` — não recebem fontes; texto é omitido (PNG) ou renderizado como tofu/vazio (SVG).
- `export_png_with_fonts(page, opts, fonts)` / `export_svg_with_fonts(page, opts, fonts)` — recebem o mesmo tipo de mapa de fontes usado por `export_pdf_multifont`, permitindo texto real.

### 3.3 Escopo intencionalmente limitado

- **Apenas a primeira página** é exportada neste MVP.
- Ligações (`Link`), metadados (`Meta`) e alguns itens de frame são ignorados em SVG/PNG quando não há representação visual directa.
- Gradients e imagens embutidas são suportadas na medida do possível com os tipos cristalinos existentes.

---

## 4. Implementação

### 4.1 L3 — `03_infra/src/export/render.rs`

- Canvas `tiny-skia` com transformação de pt → px (`pixel_per_pt` default 2.0).
- Renderização de formas: preenchimento e stroke de `ShapeKind::Path`, `ShapeKind::Rect`, `ShapeKind::Ellipse`.
- Texto via `pixglyph` quando fontes resolvidas são fornecidas; fallback para rectângulos/tofu quando não.
- Imagens: decode via `image` + `bytemuck` para pixmap `tiny-skia`.
- Saída: `sk::Pixmap::encode_png()`.

### 4.2 L3 — `03_infra/src/export/svg.rs`

- Geração via `xmlwriter`.
- ViewBox em pt, dimensões da página.
- Formas convertidas para paths SVG (`<path>`).
- Texto como `<text>` com `font-family`, `font-size`, `fill`; sem fontes resolvidas, texto é omitido.
- Imagens embutidas como `data:image/png;base64,...`.

### 4.3 L3 — `03_infra/src/pipeline.rs`

Novas funções:
- `compile_to_png_bytes(...)` / `compile_to_png_bytes_with_fonts(...)`
- `compile_to_svg_string(...)` / `compile_to_svg_string_with_fonts(...)`

Resolvem fontes do sistema usando `FontResolver` e constroem o mapa `((FontList, FontVariant, FontVariations), Vec<u8>)` esperado pelos exporters.

### 4.4 L4 — `04_wiring/src/main.rs`

- Recebe `OutputFormat` (PDF/PNG/SVG) do `RunIntent`.
- Para PNG/SVG, invoca o pipeline correspondente e escreve o resultado no ficheiro de saída.
- Mantém o comportamento `--format vence extensão` herdado do P866.

### 4.5 L0

- `00_nucleo/prompts/infra/export/render.md` — hash `f464d3b6`.
- `00_nucleo/prompts/infra/export/svg.md` — hash `2efe9af1`.
- `00_nucleo/prompts/infra/export/mod.md` — hash `bf6cec00`.

Os headers dos ficheiros Rust foram sincronizados com `crystalline-lint --fix-hashes`.

---

## 5. Dependências adicionadas

```toml
tiny-skia   = "0.12"                # canvas 2D / rasterização PNG
resvg       = { version = "0.47", default-features = false, features = ["raster-images"] }  # renderização SVG interno
pixglyph    = "0.6.1"               # rasterização de glifos (PNG)
bytemuck    = "1"                   # cast seguro de bytes para tiny-skia
xmlwriter   = "0.1.0"               # geração de SVG
base64      = "0.22"                # imagens embutidas em SVG
itoa        = "1"                   # serialização de inteiros em SVG
ryu         = "1"                   # serialização de floats em SVG
```

Todas são permitidas em L3 (I/O e rendering). Nenhuma dependência nova em L1/L2.

---

## 6. Testes

### 6.1 Integração em `04_wiring/tests/cli.rs`

Foram adicionados 4 testes P870:
- `p870_output_png_gera_png_valido`
- `p870_output_svg_gera_svg_valido`
- `p870_format_flag_png_vence_extensao_pdf`
- `p870_format_flag_svg_vence_extensao_pdf`

Foram removidos 3 testes P866 que verificavam recusa de PNG/SVG (já não aplicável):
- `p866_output_png_recusado_com_erro_claro`
- `p866_output_svg_recusado_com_erro_claro`
- `p866_format_flag_png_vence_extensao_pdf`

Os testes P866 de PDF continuam:
- `p866_output_pdf_continua_funcionar`
- `p866_format_flag_pdf_continua_funcionar`

### 6.2 Resultados da suíte completa

```bash
$ cargo test --workspace
```

```text
test result: ok. 4682 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out  (typst-core)
test result: ok. 721 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out   (typst-shell)
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out    (typst-infra)
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out     (typst-infra integration)
test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out    (typst-wiring cli)
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out     (crystalline_lint)
```

**Total: 5485 passaram, 0 falharam.**

### 6.3 Linter

```bash
$ crystalline-lint .
```

```text
warning: Prompt órfão: '00_nucleo/prompts/infra/package_version_resolution.md' não é referenciado por nenhum arquivo em L1–L4. Materializar ou remover. [V7]
```

Apenas V7 pré-existente. Nenhum V3/V4/V5/V13/V14 introduzido pelo P870.

---

## 7. Validação manual

```bash
$ echo 'Hello World' > /tmp/p870.typ
$ ./target/debug/typst /tmp/p870.typ -o /tmp/p870.png
$ ./target/debug/typst /tmp/p870.typ -o /tmp/p870.svg
$ file /tmp/p870.png /tmp/p870.svg
/tmp/p870.png: PNG image data, 1191 x 1684, 8-bit/color RGBA, non-interlaced
/tmp/p870.svg: SVG Scalable Vector Graphics image
```

```bash
$ head -2 /tmp/p870.svg
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 595.28 841.89" width="595.28pt" height="841.89pt"><rect x="0" y="0" width="595.28" height="841.89" fill="#ffffff"/><text x="70.866666667" y="78.104666667" font-size="11" font-family="libertinus serif" fill="#000000" dominant-baseline="alphabetic">Hello</text>...</svg>
```

A resolução 1191×1684 corresponde a A4 a 2 px/pt (595.28 × 841.89 × 2). O SVG contém texto posicionado correctamente com fonte Libertinus Serif (default do vanilla).

---

## 8. Limitações

1. **Primeira página apenas.** Documentos multi-página produzem apenas a página 0.
2. **Fontes.** A variante simples (`export_png`/`export_svg`) omite texto. Para texto real, é necessário usar a variante `_with_fonts` com fontes resolvidas.
3. **Texto shaped.** A rasterização PNG usa `pixglyph` sobre glifos; o SVG emite `<text>` com `font-family`, confiando no renderer do visualizador.
4. **Paridade visual.** Não se garante equivalência pixel-perfect com o vanilla; garante-se equivalência semântica/morfológica (conteúdo visível no sítio certo).

---

## 9. Conclusão

O P870 cumpre o objectivo: PNG e SVG passaram a ser formatos de saída reais no compilador cristalino. A arquitetura respeita a topologia de camadas (toda a lógica de render em L3), reutiliza os tipos cristalinos existentes, e segue o padrão de API de fontes já estabelecido pelo PDF. A suíte de testes continua verde e o linter não introduziu novas violações.

Trabalho futuro: exportação multi-página, embedding de fontes no SVG, e maior cobertura de formas/gradientes.
