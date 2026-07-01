# Relatório de Sonda — Passo 526

| Campo | Valor |
|-------|-------|
| Passo | P526 |
| Foco | Igualdade de saída: HTML, SVG, PNG, IDE/LSP |
| Data | 2026-07-01 |
| Autor | IA (Kimi Code CLI) sob direcção do utilizador |
| Status | Concluído |

---

## Contexto

O handoff declarou HTML export, SVG export, raster render (PNG) e IDE/LSP como **Fora de escopo** permanentes (PDF-only). O utilizador estabeleceu nova condição: **inovação (Lookahead, etc.) só quando houver igualdade de saída com o vanilla 0.15.0**, incluindo estas funcionalidades.

Este passo executa a sonda A.0 exigida pela ADR-0114: medir o estado actual, classificar o código existente, estimar complexidade e propor ordem de execução. **Zero código de produção** foi escrito.

---

## Grupo 1 — Inventário de código existente

### 1.1 HTML export

```bash
grep -rn "html\|HTML" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
```

Resultado relevante:

```text
01_core/src/entities/world_types.rs:255:    pub const MAX_HTML_DEPTH: usize = 72;
01_core/src/entities/world_types.rs:297:pub fn check_html_depth(route: Tracked<'_, Route<'_>>) -> SourceResult<()> {
01_core/src/entities/world_types.rs:298:    if !route.within(Route::MAX_HTML_DEPTH) {
01_core/src/entities/world_types.rs:302:                "maximum HTML depth exceeded",
```

Classificação: **AUSENTE**. A única referência a "HTML" é uma constante de profundidade máxima de recursão e uma função de guarda (`check_html_depth`), sem qualquer módulo de export HTML, DOM, ou CSS.

### 1.2 SVG export

```bash
grep -rn "svg\|SVG" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
```

Resultado relevante:

```text
01_core/src/rules/stdlib/structural.rs:2184:        "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" => EcoString::from("image"),
```

Classificação: **AUSENTE**. A única referência é a detecção de extensão `.svg` para ficheiros de imagem de entrada. Não existe export SVG.

### 1.3 Raster render (PNG)

```bash
grep -rn "png\|PNG\|raster" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
```

Resultado relevante:

```text
03_infra/src/export/images.rs:108:pub fn process_png_for_pdf(raw_data: &[u8]) -> Result<PdfImagePayload, String>
03_infra/src/export/images.rs:354:pub(super) fn build_png_smask_xobject(...)
03_infra/src/export/images.rs:370:pub(super) fn build_png_rgb_xobject(...)
```

Classificação: **AUSENTE para export PNG**, **PARCIAL para leitura de PNG**. O cristalino lê PNGs e os embute em PDF, mas não rasteriza o documento para PNG.

### 1.4 IDE / LSP

```bash
grep -rn "tower-lsp\|lsp-types\|jsonrpc\|lsp_server\|LanguageServer\|Lsp" \
  Cargo.toml 01_core/Cargo.toml 02_shell/Cargo.toml 03_infra/Cargo.toml 04_wiring/Cargo.toml lab/Cargo.toml
```

Resultado: **nenhuma ocorrência**.

```bash
ls 02_shell/src/bin/ 04_wiring/src/bin/
```

Resultado: **nenhum bin `typst-lsp`**. O único binário é `typst` (`04_wiring/src/main.rs`).

Classificação: **AUSENTE**. Não existe servidor LSP, nem dependências LSP, nem binário separado.

### 1.5 Infraestrutura reutilizável existente

Apesar das funcionalidades de saída estarem ausentes, a base de dados/layout já existe:

| Componente | Estado | Reutilizável para |
|------------|--------|-------------------|
| `Document` / `Frame` / `FrameItem` | ✅ Existe | HTML, SVG, PNG |
| `FrameItem::TextShaped` com posições e glifos | ✅ Existe | HTML, SVG, PNG |
| Paths / Bézier (`PathItem` em `geometry.rs`) | ✅ Existe | SVG, PNG (via SVG) |
| `Image` element e carregamento | ✅ Existe | HTML, SVG, PNG |
| Source mapping (`Source`, `Span`, AST spans) | ✅ Existe | LSP |
| Eval engine / diagnostics | ✅ Existe | LSP (semantic analysis) |
| fontdb + shaping | ✅ Existe | HTML, SVG, PNG |

### 1.6 Dependências Cargo relevantes

```bash
grep -rn "tower-lsp\|lsp-types\|jsonrpc\|resvg\|tiny-skia\|png\|svg" \
  Cargo.toml 01_core/Cargo.toml 03_infra/Cargo.toml 04_wiring/Cargo.toml lab/Cargo.toml
```

Resultado:

```text
03_infra/Cargo.toml:32:image = { version = "0.24", default-features = false, features = ["png", "jpeg"] }
```

Apenas a crate `image` está presente (para leitura de PNG/JPEG). Não existem `resvg`, `tiny-skia`, `lsp-types`, `tower-lsp`, etc.

---

## Grupo 2 — Comportamento actual da CLI

### 2.1 `--help` cristalino

```text
Typst compiler (crystalline)
Usage: typst [OPTIONS] <INPUT> [OUTPUT]
```

Opções disponíveis: `-o/--output`, `--root`, `--font-path`, `--color`, `--full-error`, `--timings-json`, `-h`, `-V`.

**Opções ausentes:** `--format`, `--ppi`, `--pages`, `--input`, `--package-path`, `--jobs`, `--open`, `--deps`, `--features`, subcomandos (`compile`, `watch`, `init`, `eval`, `fonts`, `completions`, `info`).

### 2.2 Tentativas de compilação para outros formatos

```bash
./target/release/typst compile /tmp/test.typ /tmp/test.html
./target/release/typst compile --format html /tmp/test.typ /tmp/test.html
./target/release/typst compile --format svg /tmp/test.typ /tmp/test.svg
./target/release/typst compile --format png /tmp/test.typ /tmp/test.png
```

Resultado:

```text
error: unexpected argument '/tmp/test.html' found
error: unexpected argument '--format' found
```

Classificação: **AUSENTE**. A CLI cristalina não tem subcomando `compile` nem flag `--format`; só aceita `typst <input> [output-pdf]`.

---

## Grupo 3 — Análise de complexidade por funcionalidade

### 3.1 HTML export

| Componente | Esforço | Reutilizável do cristalino? |
|------------|---------|----------------------------|
| Layout tree (`Frame`) → HTML DOM | M–L | `Frame` existe; mapeamento é novo |
| CSS generation (inline ou `<style>`) | M | `TextStyle`, elementos de layout têm propriedades CSS-análogas |
| Font embedding / `@font-face` | S–M | fontdb já descobre fontes (P515) |
| Math → MathML | L–XL | Math elements existem (P510–P511); MathML é markup diferente |
| Interactive (links, outlines) | S | PDF já suporta (P517); HTML é `<a>`, `<nav>` |
| Images | S | `image` element existe |

**Estimativa HTML:** M–L para MVP sem MathML; L–XL completo.

### 3.2 SVG export

| Componente | Esforço | Reutilizável? |
|------------|---------|---------------|
| Layout tree → SVG | M | `Frame`, coordenadas, paths existem |
| Text rendering (`<text>` ou paths) | S–M | Shaping existe (P515); outlines podem ser convertidos |
| Font embedding / outlines | M | Subsetting existe (P516); converter glyphs para paths requer parse de outlines |
| Images (`<image href=...>`) | S | `image` element existe |
| Gradients/patterns | S–M | `Gradient`/`Pattern` existem em entities |

**Estimativa SVG:** M para MVP com fontes referenciadas; L com fontes embutidas como paths.

### 3.3 Raster render (PNG)

| Componente | Esforço | Reutilizável? |
|------------|---------|---------------|
| Layout tree → bitmap | M | Necessita rasterizer |
| Text rasterization | M | HarfBuzz + fontdb + rasterizer |
| Path rasterization | S–M | resvg/tiny-skia fazem isso |
| Image compositing | S | rasterizer faz isso |
| Multi-page / paged output | S | CLI flag `--pages` |
| DPI / PPI | XS | Factor de escala |

**Abordagem recomendada:** `Document` → SVG (reutilizar 3.2) → `resvg` + `tiny-skia` → PNG.  
**Estimativa PNG:** S–M se SVG existir; M–L se feito directamente.

### 3.4 IDE / LSP

| Componente | Esforço | Reutilizável? |
|------------|---------|---------------|
| LSP server (`tower-lsp`) | S | Boilerplate padrão |
| Source mapping | M | Parser/lexer preserva spans |
| Semantic analysis | L–XL | Eval engine já faz isso; precisa de API de introspecção |
| Diagnostics | S–M | Eval engine já produz diagnostics |
| Completions | L | Requer symbol table + scope analysis |
| Hover / Go to Definition | L | Requer semantic analysis + source mapping |
| Formatting | M–L | Requer pretty-printer do AST |

**Estimativa IDE/LSP:** L para MVP (diagnostics + básico); XL completo.

---

## Grupo 4 — Dependências e ordem de execução

```text
HTML ── independente
SVG ───┬── independente
       └── PNG (via SVG)
LSP ─── independente (mas requer semantic analysis do eval)
```

| Ordem | Funcionalidade | Tamanho | Razão |
|-------|---------------|---------|-------|
| 1 | **SVG export** | M–L | Reutiliza infraestrutura de layout, paths, shaping. Serve de base para PNG. |
| 2 | **PNG export** | S–M (pós-SVG) | Via `resvg`/`tiny-skia` a partir de SVG. |
| 3 | **HTML export** | M–L (MVP) | Independente, mas requer mapeamento DOM/CSS. MVP sem MathML é M. |
| 4 | **IDE / LSP** | XL | Mais complexo, menos prioritário se o foco é "igualdade de saída" de compilação. |

---

## Grupo 5 — Paridade com vanilla 0.15.0

### 5.1 CLI vanilla

```text
Typst 0.15.0 (969087ec)
Commands: compile, watch, init, eval, fonts, completions, info
compile options: --format [pdf, png, svg, html, bundle], --ppi, --pages, --input, etc.
```

### 5.2 Testes de saída vanilla

| Formato | Comando | Resultado |
|---------|---------|-----------|
| HTML | `typst compile --features html --format html /tmp/test.typ /tmp/test.html` | ✅ 173 bytes, `<!DOCTYPE html><html>...<p>Hello world.</p>...</html>` |
| SVG | `typst compile --format svg /tmp/test.typ /tmp/test.svg` | ✅ 7.4 KB, SVG com `<use>` e `<defs>` para glyphs |
| PNG | `typst compile --format png --ppi 144 /tmp/test.typ /tmp/test.png` | ✅ 14 KB, 1191×1684 RGBA |
| LSP | `typst lsp` | Não testado (binário `typst` vanilla não inclui `lsp`; existe `typst-lsp` separado) |

### 5.3 Requisitos mínimos para igualdade de saída

- **SVG:** suportar `--format svg` e gerar SVG paginado com texto e paths.
- **PNG:** suportar `--format png --ppi N` e gerar PNG por página.
- **HTML:** suportar `--format html` e gerar HTML semântico (pode requerer feature flag, como no vanilla).
- **LSP:** fornecer binário ou modo LSP com diagnostics básicos (pode ser scope-out se o utilizador aceitar).

---

## Tabela final de classificação

| Funcionalidade | Código existente | CLI actual | Esforço MVP | Esforço completo | Dependências | Prioridade proposta |
|----------------|------------------|------------|-------------|------------------|--------------|---------------------|
| HTML export | AUSENTE | AUSENTE (sem `--format`) | M–L | L–XL | — | 3 |
| SVG export | AUSENTE | AUSENTE (sem `--format`) | M | L | — | 1 |
| PNG export | PARCIAL (leitura PNG) | AUSENTE (sem `--format`) | S–M (pós-SVG) | M | SVG | 2 |
| IDE/LSP | AUSENTE | AUSENTE | L | XL | Semantic analysis | 4 |

---

## Decisão de prosseguimento

Baseado na sonda:

- Todas as funcionalidades de saída estão **ausentes no cristalino**.
- A infraestrutura de layout (`Frame`, `TextShaped`, paths, images) é reutilizável.
- A ordem recomendada é **SVG → PNG → HTML → LSP**.

**Recomendações:**

1. **P527 — Especificação de SVG export (Trilha 8).** Tamanho M–L. Definir se fontes são embutidas como `@font-face`, convertidas para `<path>`, ou referenciadas; decidir scope de elementos suportados.
2. **P528+ — Implementação de SVG export.** 3–5 passos, dependendo da fidelidade.
3. **P53x — PNG export (Trilha 9).** S–M se SVG existir, via `resvg`/`tiny-skia`.
4. **P54x — HTML export (Trilha 10).** M–L para MVP sem MathML.
5. **P55x+ — IDE/LSP (Trilha 11).** XL; considerar scope-out se o utilizador aceitar que "igualdade de saída" se refira apenas a formatos de compilação.

**Nota sobre ADR-0107:** A paridade é com a linguagem Typst, mas a igualdade de saída (HTML/SVG/PNG) é uma propriedade mecânica de produção que o utilizador declarou como pré-requisito para inovação. Esta sonda mediu o gap; a decisão de fechar esse gap é do utilizador.

---

## Reprodução

```bash
# Inventário de código
grep -rn "html\|HTML" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
grep -rn "svg\|SVG" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
grep -rn "png\|PNG\|raster" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
grep -rn "tower-lsp\|lsp-types\|jsonrpc" Cargo.toml 01_core/Cargo.toml 03_infra/Cargo.toml 04_wiring/Cargo.toml

# CLI cristalina
./target/release/typst --help
./target/release/typst compile --format html /tmp/test.typ /tmp/test.html

# CLI vanilla
./lab/typst-original/target/release/typst compile --format svg /tmp/test.typ /tmp/test-vanilla.svg
./lab/typst-original/target/release/typst compile --format png --ppi 144 /tmp/test.typ /tmp/test-vanilla.png
./lab/typst-original/target/release/typst compile --features html --format html /tmp/test.typ /tmp/test-vanilla.html
```

---

## Linhagem

- ADR-0107: paridade é com a linguagem, não com a mecânica.
- ADR-0108: medir antes de decidir.
- ADR-0114: sonda A.0 antes de spec.
- Handoff: `00_nucleo/diagnosticos/cristalino-contexto-handoff.md` (secção 5.2 — Fora de Escopo).
