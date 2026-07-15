# Diagnóstico P761 — Sonda directa sobre o resíduo deixado por P760

**Data da medição:** 2026-07-14T22:34:05Z  
**Commit base:** `8b3bf605d7fc8e5fcbf84c09ac0ba5feaf5a4dd2`  
**Working tree:** modificado (ver lista de ficheiros em "Alterações").  
**Passo:** P761  
**Objectivo:** Executar os passos de diagnóstico que P760 não executou (mapa visual, identidade de fonte, coordenadas Y de todas as linhas, largura de linha, palavra única em alta resolução), identificar a causa real do resíduo de ~97,8 % e corrigir o que for corrigível.

---

## Documento de teste

`/tmp/p760-fixo.typ`:

```typst
#set page(width: 350pt, margin: 40pt)
#set text(font: "DejaVu Sans", size: 11pt)
#[
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
]
```

Vanilla de referência: `/tmp/p760-fixo-vanilla.pdf` + `/tmp/p760-fixo-vanilla.png` (300 ppp, `pdftoppm`).  
Cristalino: `/tmp/p761-fixo-cristalino-corrigido-final2.pdf` + `/tmp/p761-fixo-cristalino-corrigido-final2.png`.

---

## Medições

### Diferença global de pixels (300 ppp)

| Estado | AE | RMSE |
|--------|-----|------|
| P760 corrigido (baseline desta sonda) | 378 997 | 0,225596 |
| Caso de palavra única (`/tmp/p761-linha-unica.typ`, 600 ppp) | 53 137 | 0,186674 |

A palavra única isola o problema de qualquer coisa relacionada com quebra de linha / layout de parágrafo. O facto de ainda haver ~53 k pixels de diferença numa só palavra indica que parte do resíduo é de baixo nível (hinting / anti-aliasing / rasterização), mas a maior parte do resíduo no parágrafo (AE ~379 k) vem de algo que se acumula linha a linha.

### Coordenadas Y das linhas (extraídas dos streams PDF com `mutool show`)

| Linha | Vanilla Y (pt) | Cristalino Y (pt) | ΔY (pt) |
|-------|---------------|-------------------|---------|
| 1 | 793,53235 | 793,533 | +0,00065 |
| 2 | 778,02490 | 780,330 | +2,305 |
| 3 | 762,51750 | 767,128 | +4,610 |
| 4 | 747,01010 | 753,926 | +6,916 |
| 5 | 731,50270 | 740,724 | +9,221 |

- A primeira linha está praticamente coincidente (erro < 0,001 pt).
- A diferença cresce linearmente: **ΔY ≈ 2,304 pt × (n − 1)**.
- O avanço vertical entre linhas consecutivas é:
  - Vanilla: **15,507 pt**
  - Cristalino: **13,203 pt**

### Cálculo das métricas envolvidas (DejaVu Sans 11 pt, UPM 2048)

- `sTypoAscender` = 1556 → 11 × 1556 / 2048 = **8,357 pt**
- `lineGap` = 410 → 11 × 410 / 2048 = **2,201 pt**
- `sTypoDescender` = −492 → 11 × 492 / 2048 = **2,642 pt**
- `line_height` cristalino = ascender + |descender| + lineGap = **13,203 pt** ✓
- `leading` default do Typst vanilla = 0,65 em = 0,65 × 11 = **7,150 pt**
- Avanço vanilla = cap_height (≈ 8,357 pt) + leading (7,150 pt) = **15,507 pt** ✓

A diferença de avanço entre linhas (15,507 − 13,203 = **2,304 pt**) explica exactamente o deslocamento crescente observado.

### Largura das linhas

As quebras de linha são idênticas; a largura total de cada linha difere na ordem dos milésimos de ponto, consistente com variações de kerning/AA, não com um bug de shaping mensurável.

### Identidade da fonte embutida

Extraídos os streams TTF dos dois PDFs (`mutool extract`) e comparados:

- Vanilla: `/tmp/p761-extract-vanilla/font-0010.ttf` (17 504 bytes)
- Cristalino: `/tmp/p761-extract-cristalino/font-0007.ttf` (34 128 bytes)

Ambos são DejaVu Sans, mas os subsets diferem em tamanho (o cristalino inclui mais glifos). A diferença visual não se deve a fontes diferentes, mas ao posicionamento vertical.

---

## Causas reais encontradas

### 1. Duplicação do offset de baseline inicial (corrigida)

Em três locais (`set_page.rs`, `cursor.rs::new_page()`, `cursor.rs::start_column()`) o cursor era inicializado em `margem + cap_height` mesmo quando `initial_baseline_pending` era `true`. Como `ensure_initial_baseline()` adiciona depois outro `cap_height`, a primeira baseline ficava em `margem + cap_height_default + cap_height_real`.

Correcção: quando `initial_baseline_pending` é `true`, `cursor_y` passa a começar apenas em `margem`; `ensure_initial_baseline()` adiciona o `cap_height` correcto uma única vez.

### 2. `cap_height` usava `hhea.ascender` como fallback (corrigida)

`FontBookMetrics::cap_height()` e `FallbackFontMetrics::cap_height()` usavam `face.ascender()` (tabela `hhea`) como fallback de `capital_height()`. Para DejaVu Sans, `hhea.ascender` = 1901 unidades (10,21 pt), enquanto o vanilla usa `typographic_ascender().unwrap_or(ascender())` → 1556 unidades (8,36 pt).

Correcção: ambas as implementações passam a preferir `face.typographic_ascender()` antes de recair em `face.ascender()`.

### 3. Modelo de avanço entre linhas diferente do vanilla (não corrigida neste passo)

O cristalino avança `line_height + leading`, onde `line_height` = ascender + descender + lineGap (métricas tipográficas).  
O Typst vanilla usa `top_edge: cap-height` / `bottom_edge: baseline` por defeito, pelo que a altura efectiva da linha é `cap_height`, e o avanço de baseline para baseline é `cap_height + par.leading` (default 0,65 em).

Esta diferença arquitetural explica o deslocamento crescente de ~2,304 pt por linha. Corrigir isto implica:
- Adoptar o conceito de `top_edge` / `bottom_edge` no cristalino, **ou**
- Alterar o L0 de layout (`00_nucleo/prompts/rules/layout.md`) para que `flush_line()` avance `cap_height + leading` em vez de `line_height + leading`.

Ambas as opções estão fora do scope tático de P761. Ficam documentadas como causa confirmada e work futuro.

---

## Correcções aplicadas

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/rules/layout/set_page.rs` | `cursor_y` inicial só adianta `cap_height` quando a baseline já foi fixada. |
| `01_core/src/rules/layout/cursor.rs` | Mesmo ajuste em `new_page()` e `start_column()`. |
| `03_infra/src/font_metrics.rs` | `FontBookMetrics::cap_height()` e `FallbackFontMetrics::cap_height()` usam `typographic_ascender()` como fallback. |
| `01_core/src/rules/layout/grid.rs` | Só subtrai `cap_height` ao `cursor_y` quando `initial_baseline_pending` é `false`, corrigindo a regressão introduzida pela mudança anterior. |

---

## Validação

- `cargo build --release` — OK.
- `cargo test --workspace` — OK: 635 passed; 0 failed; 5 ignored.
- `crystalline-lint .` — zero violations.
- Primeira linha do caso P760 alinhada com o vanilla a < 0,001 pt.
- Grid regressão corrigida (testes `grid_auto_com_multiplas_celulas_reutiliza_cache`, `grid_valign_bottom_ancora_ao_limite_inferior_da_celula`, `place_dentro_de_grid_ancora_a_celula` passam).

---

## Conclusão

- Os bugs corrigíveis identificados por P761 (duplicação de baseline inicial e fallback errado de `cap_height`) foram corrigidos.
- A primeira linha do documento P760 está agora alinhada com o vanilla.
- O resíduo de ~97,8 % restante é explicado por uma diferença arquitetural no avanço entre linhas: o cristalino usa `line_height` (ascender + descender + lineGap) enquanto o vanilla usa `cap_height + par.leading` (default 0,65 em). Esta diferença acumula ~2,304 pt por linha.
- A correção deste modelo de line_height requer alteração ao Prompt L0 `00_nucleo/prompts/rules/layout.md` e é scope-out de P761.

---

## Alterações (ficheiros modificados no working tree)

```text
01_core/src/rules/layout/cursor.rs
01_core/src/rules/layout/grid.rs
01_core/src/rules/layout/set_page.rs
03_infra/src/font_metrics.rs
```
