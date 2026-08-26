# Specification: `pdf_defaults` (L3 Export Defaults)
Hash do Código: 61654d67

## P1140.20.1 — A4 normativo

Defaults A4 derivam de `entities/page_geometry::Paper::A4`, tabela em mm com
`pt = mm × 72 / 25.4`. Proibido manter `595.28`/`841.89` como constantes de
produção. Serialização pode arredondar; domínio não é calibrado por bytes.

## 1. Responsabilidade
Consolidar constantes canónicas e valores de fallback padronizados utilizados pelo motor de serialização PDF e operadores gráficos em `03_infra/src/export/` (Passo 1069), evitando duplicações e literais soltos no código.

## 2. Constantes Canónicas

### 2.1 `BEZIER_CIRCLE_KAPPA: f64 = 0.552_284_749_831`
* **Descrição**: Fator de aproximação de quarto de círculo por curva Bézier cúbica: $\kappa = \frac{4(\sqrt{2}-1)}{3} \approx 0.552284749831$.
* **Proveniência**: ISO 32000-1:2008 (PDF 1.7 Spec) §4.4 / Adobe PostScript Language Reference Manual.
* **Consumidores**: `stream.rs` na emissão de operadores de curva para elipses e retângulos arredondados.

### 2.2 `FAUX_BOLD_K: f64 = 0.04`
* **Descrição**: Proporção da espessura de traço (stroke) relativa ao tamanho da fonte para emulação de negrito sintético (faux bold).
* **Proveniência**: `lab/typst-original/crates/typst-pdf/src/text.rs` / Typst PDF renderer.
* **Consumidores**: `stream.rs` em operadores `2 Tr` e cálculo de `faux_bold_stroke_pt`.

### 2.3 `A4_DEFAULT_WIDTH: f64 = 595.28` e `A4_DEFAULT_HEIGHT: f64 = 841.89`
* **Descrição**: Dimensões padrão precisas de página A4 em pontos tipográficos ($72\text{ pt/in}$, $210 \times 297\text{ mm}$).
* **Proveniência**: ISO 216 / Adobe PostScript Paper Sizes.
* **Consumidores**: `builder.rs` na construção de páginas em branco (`blank_page`).

### 2.4 `A4_FALLBACK_WIDTH_ROUNDED: f64 = 595.0` e `A4_FALLBACK_HEIGHT_ROUNDED: f64 = 842.0`
* **Descrição**: Dimensões de página inteiras de fallback de emergência para conversão de coordenadas e anotações quando metadados de página estão ausentes.
* **Consumidores**: `builder.rs` em rotas de `unwrap_or`.
