# Auditoria de paridade código vs vanilla — 2026-05-19

**Data**: 2026-05-19
**Método**: empírico — grep/contagem direta no código, sem leitura
de relatórios/documentação. Cristalino `01_core/`–`04_wiring/` vs
vanilla `lab/typst-original/crates/`.
**Estado do branch**: `Tekt`, post-P305.

---

## 1. LOC totais

| | Cristalino | Vanilla | Ratio |
|---|---:|---:|---:|
| **Total LOC `.rs`** | 91.806 | 135.685 | **67,7%** |
| `typst-syntax` (parser+lexer+ast+source) | ~4.408 | 10.556 | 42% |
| `typst-eval` | 5.616 (3.065 são testes) | 2.954 | 87% prod |
| `typst-layout` | ~16.759 (L1 layout + L3 layout) | 20.915 | 80% |
| `typst-pdf` | 9.856 (`export.rs` único) | 7.559 | 130% |
| `typst-library` (entities+stdlib) | dispersa em L1 | 66.182 | — |

**Crates inteiros vanilla sem equivalente no cristalino: ~13k LOC**

- `typst-html` (4.981 LOC) — **0%**
- `typst-svg` (2.033 LOC) — **0%**
- `typst-ide` (4.532 LOC) — **0%**
- `typst-realize` (1.509 LOC pipeline rewrite) — dispersa em L1 walks
- `typst-render` (1.127 LOC raster) — **0%**
- `typst-timing` (317 LOC) — **0%**

---

## 2. Elementos (`#[elem]` vanilla vs `Content` cristalino)

Contagem direta no código:

- **Vanilla**: 132 `pub struct …Elem` distintas em
  `lab/typst-original/crates/typst-library/src/`
- **Cristalino**: 76 variants em `pub enum Content`
  (`01_core/src/entities/content.rs:42`)

Match por nome direto: **47**.
Cobertos via variant agregado: **37** (e.g. `BoxElem→Boxed`,
`Circle/Ellipse/Rect/Line/Polygon→Shape{ShapeKind}`,
`Rotate/Scale/Move/Skew→Transform`, `Strong/Emph→Styled(Bold/Italic)`,
`MatElem/VecElem→MathMatrix`, `Under*Elem/Over*Elem (8)→MathUnderover`,
`PageElem→SetPage`).

**Cobertura efetiva: ~84/132 = 64%.**

### 2.1 — 46 elementos vanilla sem equivalente cristalino

| Cluster | Elementos faltantes | N |
|---|---|---:|
| **Curve granular** | `CurveElem`, `CurveMove`, `CurveLine`, `CurveCubic`, `CurveQuad`, `CurveClose` | 6 |
| **Paragraph** | `ParElem`, `ParbreakElem`, `ParLine`, `ParLineMarker`, `FlushElem` | 5 |
| **Footnote machinery** | `FootnoteContainer`, `FootnoteEntry`, `FootnoteMarker` (apenas `Footnote` básico existe) | 3 |
| **PDF tagging** | `PdfMarkerTag`, `TagElem`, `ArtifactElem`, `AssetElem`, `DirectLinkElem`, `LinkMarker` | 6 |
| **Math granular** | `BinomElem`, `ClassElem`, `LimitsElem`, `MidElem`, `PrimesElem`, `ScriptsElem`, `StretchElem` | 7 |
| **Text granular** | `SmallcapsElem`, `SubElem`, `SuperElem`, `HighlightElem`, `SymbolElem`, `RawLine` | 6 |
| **Table/Grid lines** | `TableHLine`, `TableVLine`, `GridHLine`, `GridVLine` | 4 |
| **Bibliography CSL** | `CslIndentElem`, `CslLightElem` | 2 |
| **Outline entries** | `OutlineEntry` (apenas `Outline` básico) | 1 |
| **Outros** | `ContextElem`, `DocumentElem`, `FigureCaption`, `InlineElem`, `LayoutElem`, `TargetElem`, `TitleElem` | 7 |

---

## 3. Stdlib (`#[func]` vanilla vs `native_*` cristalino)

### 3.1 — Calc (`foundations/calc.rs`)

- Vanilla: **41** funções
- Cristalino: **25** funções → **61%**

**Faltam 16**: `binom`, `div_euclid`, `erf`, `even`, `fact`, `fract`,
`gcd`, `lcm`, `norm`, `odd`, `perm`, `quo`, `rem`, `rem_euclid`,
`root`, `trunc`.

### 3.2 — Funções não-calc vanilla ausentes (39 funções)

Total: ~80 funções vanilla `#[func]` fora de `calc.rs`. Cristalino
implementa ~41 via `native_*`. Ausentes:

| Categoria | Funções faltantes | N |
|---|---|---:|
| **Math style** | `bb`, `bold`, `cal`, `frak`, `italic`, `mono`, `sans`, `scr`, `script`, `serif`, `sscript`, `upright` | 12 |
| **Data parsing** | `cbor`, `csv`, `json`, `toml`, `xml`, `yaml`, `read` | 7 |
| **Sistema** | `plugin`, `panic`, `eval`, `repr` | 4 |
| **Layout shorthand** | `layout`, `lorem`, `numbering`, `display`, `inline` | 5 |
| **Table cells** | `data_cell`, `header_cell`, `table_summary` | 3 |
| **Outros** | `target` | 1 |

---

## 4. Parser/Lexer — paridade quase total

- `SyntaxKind`: **134 variants vanilla = 134 variants cristalino**
  (paridade 100% na enum)
- Lexer: vanilla 1168 LOC ⟷ cristalino 1977 LOC (mais verboso,
  fragmentado em `code.rs` / `markup.rs` / `math.rs` / `scanner.rs`)
- Parser: vanilla 2137 LOC ⟷ cristalino 2431 LOC (similar)

---

## 5. L0 prompts vs L1 código

- 73 prompts em `00_nucleo/prompts/entities/`
- 69 ficheiros `.rs` em `01_core/src/entities/`
- Órfãos código→prompt: **1** (`world_types.rs` sem L0 — violação
  ténue da Regra de Ouro CLAUDE.md)
- Órfãos prompt→código: **0**

---

## 6. Paridade observacional empírica (`lab/parity/`)

- Corpus: 36 ficheiros `.typ`
- Compilam em cristalino: **24/25** (96%)
- Comparação contra vanilla: **N/A em todas as colunas**
  (`text_content` / `structural` / `geometric`) — bloqueada por World
  adapter cristalino↔vanilla
- Testes parity: 5 ficheiros (parse, eval, layout, structural,
  vanilla_cli_smoke), 2.273 LOC infra

---

## 7. Veredicto cru

| Dimensão | Paridade |
|---|---:|
| Parser/Lexer (SyntaxKind) | **100%** |
| Eval LOC | ~87% (incl 3k LOC testes) |
| Elementos `#[elem]` vanilla cobertos | **64%** (47 match + 37 agregados) |
| Layout LOC | ~80% |
| PDF export | 130% LOC (monolito; falta tagging/PDF-A/transparency) |
| Stdlib calc | **61%** (25/41) |
| Stdlib geral (não-calc) | ~50% (39 funções vanilla faltantes) |
| Crates auxiliares (html, svg, ide, render) | **0%** |
| LOC total código | **68%** |

### 7.1 — Maiores buracos no código (não na documentação)

1. **HTML/SVG/Render/IDE** — 12.673 LOC vanilla, zero no cristalino.
2. **PDF tagging/acessibilidade** — cluster de 6 elementos + lógica
   em `typst-pdf` ausente.
3. **Curve granular** — só `Path(Vec<PathItem>)` existe; falta a
   sintaxe `curve.move/line/cubic/quad/close`.
4. **Paragraph como elemento** — texto flui mas não há wrapper `Par`
   para introspection/layout dedicado.
5. **Math style** (`bb`, `cal`, `frak`, etc.) — 12 funções de variant
   tipográfica ausentes.
6. **Data parsing** (`json`, `csv`, `yaml`, `toml`, `xml`, `cbor`,
   `read`) — sem entrada/saída de dados externos.
7. **Footnote machinery** — só elemento básico; falta
   Container/Entry/Marker + 2-pass layout.
8. **Table/Grid lines** — sem `HLine`/`VLine` elementos.
