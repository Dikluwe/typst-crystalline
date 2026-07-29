# Relatório P922 — gap real de acento com `accent_base_height` e `text_ink_bounds_signed`

**Precede este passo:** `typst-passo-920-relatorio.md`, Fase A — a fórmula real do vanilla para
acentos (`gap = -accent.descent() - base.ascent().min(accent_base_height)`) não podia ser portada
fielmente porque o contrato `FontMetrics::text_ink_bounds` do cristalino força `ascent`/`descent >= 0`,
enquanto o vanilla precisa de `accent.descent()` negativo (combining mark acima da baseline).

**Commit base (antes deste passo):** `c9df6fde` (P921 integrado).
**Commit produzido por este passo:** não houve commit isolado; P922 foi integrado no commit
`da18ea9f3` juntamente com P923 e P924.
**Baseline de testes (antes, herdada de P921):** typst-core: 4800 · typst-infra: 743 ·
typst-shell: 41 — todos `0 failed`.
**Baseline de testes (fim deste passo):** typst-core: 4803 (+3 testes `p922_*`) · typst-infra: 743 ·
typst-shell: 41 — `0 failed`.
**Data:** 2026-07-28.

---

## Fase A — decisão arquitectural

### A.1 — mapeamento do problema

- A fórmula do vanilla (`typst-layout/src/math/accent.rs:56-65`) é um **cap**, não uma
  constante aditiva: `gap = -accent.descent() - base.ascent().min(accent_base_height)`.
- O comentário do vanilla (`accent.rs:57-60`) indica que só bases muito pequenas precisam de
  gap maior; o cap limita o gap para bases altas.
- O cristalino usava `text_ink_bounds` com contrato `>= 0` (`engine/layout/metrics.rs:59-61`),
  perdendo a informação de sinal necessária para `-accent.descent()` negativo.
- Consumidores actuais de `text_ink_bounds` incluem `layout_equation_measured` (extent da
  equação) e vários pontos de `MathBox`; alterar o contrato geral teria risco de cascata.

### A.2 — opções consideradas

- **(a) Estender o contrato geral de `text_ink_bounds`** para permitir valores negativos.
  Rejeitada: afectaria todos os consumidores de `MathBox::ascent`/`descent` e o default
  conservador `(cap_height, 0)` já estabelecido.
- **(b) Adicionar método novo `FontMetrics::text_ink_bounds_signed`** que devolve `(top, bottom)`
  com sinal, sem alterar `text_ink_bounds`. Escolhida: mais contida, localiza a mudança ao
  consumidor real (`layout_accent`), mantém a semântica de `MathBox` inalterada.

### A.3 — decisão

Introduzir:
- `FontMetrics::text_ink_bounds_signed(text, size, style) -> (Pt, Pt)` em L1.
- Campos `accent_base_height` e `flattened_accent_base_height` em `MathConstants` (L1).
- Leitura desses campos da tabela OpenType MATH em `math_constants_from_face` (L3).
- Implementação da fórmula real em `layout_accent` (L1).

`flattened_accent_base_height` é adicionada por paridade de dados; a variante "flattened" do
acento ainda não está implementada.

---

## Fase A.1 — L0s actualizados

Prompts afectados (hashes sincronizados via `crystalline-lint --fix-hashes .`):

- `00_nucleo/prompts/engine/layout.md` — secção P922 (`text_ink_bounds_signed`).
- `00_nucleo/prompts/engine/math/layout/accent.md` — secção P922 (fórmula do gap e
  `accent_base_height`).
- `00_nucleo/prompts/infra/font_metrics.md` — secção P922 (`text_ink_bounds_signed` e leitura
  de `accent_base_height`/`flattened_accent_base_height`).
- `00_nucleo/prompts/entities/math_constants.md` — secção P922 (campos novos).

---

## Fase B — Implementação

### B.1 — `MathConstants` (`01_core/src/entities/math_constants.rs`)

Adicionados:
```rust
pub accent_base_height: f64,
pub flattened_accent_base_height: f64,
```

Valores fallback baseados em `MathConstants::fallback()` (STIX Two Math, upem=1000).

### B.2 — `FontMetrics` trait (`01_core/src/engine/layout/metrics.rs`)

Adicionado:
```rust
fn text_ink_bounds_signed(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt);
```

Default conservador: `(cap_height(size, style), Pt(0.0))`, consistente com o default de
`text_ink_bounds` mas interpretado com sinal (`top >= 0`, `bottom <= 0`).

### B.3 — `FallbackFontMetrics` / `from_face` (`03_infra/src/font_metrics.rs`)

- `text_ink_bounds_signed` implementado com `glyph_index` + `glyph_bounding_box`, sem forçar
  `max(0.0, ...)`: `top = size * (y_max / upem)`, `bottom = size * (y_min / upem)`.
- `math_constants_from_face` lê `accent_base_height` e `flattened_accent_base_height` de
  `ttf_parser::math::Constants` (métodos já existentes em `ttf-parser 0.25`).

### B.4 — `layout_accent` (`01_core/src/engine/math/layout/accent.rs`)

Implementação da fórmula literal do vanilla:

1. Medir o acento base (char original, antes de esticar) via `text_ink_bounds_signed` para
   obter `accent_bottom_signed`.
2. Converter `accent_base_height` para pontos.
3. `gap = -accent_bottom_signed - base_box.ascent.min(accent_base_height_pt)`.
4. `accent_y = -base_box.ascent + base_box.ascent.min(accent_base_height_pt)`.
5. `new_ascent = base_box.ascent + accent_box.height() + gap`.

A inferência de que o gap usa o descent com sinal do **caractere base original** (não do
resultado esticado) está marcada como refutável nos L0s.

### B.5 — Testes (`01_core/src/engine/math/layout/tests.rs`)

Adicionados 3 testes sintéticos com um test double `SignedMetrics` que injecta bounding boxes
controladas:

- `p922_signed_descent_aumenta_new_ascent` — descent negativo aumenta o `ascent` final vs
  descent zero.
- `p922_accent_y_respeita_cap_altura_base` — `accent_y` usa o ascent da base para bases
  pequenas e o cap para bases grandes.
- `p922_gap_positivo_quando_acento_muito_acima` — fórmula literal com valores que forçam
  `gap > 0`.

### B.6 — Resultado dos testes

`cargo test --workspace`: typst-core 4803 (+3) · typst-infra 743 · typst-shell 41 — `0 failed`.

`crystalline-lint .`: hashes sincronizados; zero violations novos atribuíveis a P922.

---

## Fase C — Confirmação geométrica e benchmark

### C.1 — Confirmação contra vanilla real

Foram gerados PDFs de comparação para 7 cenários de acento (`tools/perf/corpus/p922/`):
`accent_double`, `accent_hat_frac`, `accent_hat_seq`, `accent_hat_single`, `accent_mixed`,
`accent_tilde_seq`, `accent_underover`.

Os resultados (`tools/perf/results/p922/`) incluem os PDFs cristalino/vanilla e os ficheiros
JSON de métricas. A validação visual/geométrica foi feita contra o binário vanilla real
(`lab/typst-original/target/release/typst`, `typst 0.15.0`).

### C.2 — Benchmark

Script: `tools/perf/benchmark-p922.py`. Resultados: `tools/perf/results/p922/attestation.json`.

| Cenário | Razão (cristalino / vanilla) |
|---|---|
| accent_double | 0.45× |
| accent_hat_frac | 0.44× |
| accent_hat_seq | 0.45× |
| accent_hat_single | 0.44× |
| accent_mixed | 0.44× |
| accent_tilde_seq | 0.44× |
| accent_underover | 0.45× |

**Nenhuma regressão atribuível a P922** — todos os cenários na faixa 0.44–0.45×, coerente com
medições anteriores para cenários pequenos de math.

---

## Resultado final

| Item | Estado |
|---|---|
| Decisão arquitectural registada (`text_ink_bounds_signed` vs alterar contrato geral) | ✅ Opção (b) escolhida |
| L0s actualizados e hashes sincronizados | ✅ 4 prompts |
| `MathConstants` com `accent_base_height` + `flattened_accent_base_height` | ✅ |
| `FontMetrics::text_ink_bounds_signed` em L1 com default conservador | ✅ |
| Implementação L3 da leitura OpenType MATH | ✅ |
| `layout_accent` com fórmula real do vanilla | ✅ |
| 3 testes sintéticos novos | ✅ |
| Suíte completa verde | ✅ core 4803 · infra 743 · shell 41 |
| `crystalline-lint .` | ✅ zero violations novos |
| Confirmação geométrica / benchmark vs vanilla | ✅ 7 cenários |
| Benchmark canônico depois/antes (follow-up) | ✅ 7 cenários, rácios 0.97–1.04× |

### C.3 — Benchmark canônico de regressão (follow-up)

A pedido da revisão de P922/P923/P924, repetiu-se o benchmark nos 7 cenários canônicos da
frente (P872–P921), comparando o binário cristalino do commit antes de P922 (`c9df6fde`)
com o do commit depois (`da18ea9f3`). Metodologia: `hyperfine --warmup 5 --min-runs 20`,
output para `/dev/null`; script `tools/perf/benchmark-p922923-canonical.py`; attestation em
`tools/perf/results/p922923-canonical/attestation.json`.

| Cenário | before (ms) | after (ms) | razão after/before |
|---|---:|---:|---:|
| 01-hello | 98.44 | 100.48 | 1.02× |
| 02-lorem | 130.67 | 126.16 | 0.97× |
| 03-images | 103.19 | 106.96 | 1.04× |
| 04-math | 159.68 | 166.43 | 1.04× |
| 05-tables | 101.68 | 104.89 | 1.03× |
| 06-long | 325.42 | 320.54 | 0.99× |
| 07-context | 143.24 | 140.50 | 0.98× |

**Nenhuma regressão atribuível a P922/P923** — todos os rácios dentro da banda de ruído
(0.97–1.04×).

### Achados em aberto desta frente

- Variante "flattened" do acento quando a base é muito alta (`flattened_accent_base_height` já
  está nos dados, mas não há consumidor ainda).
- Inferência refutável: o gap de acentos esticados (`hat(a+b)`) usa o descent com sinal do
  caractere base original, não do resultado esticado. Se medições futuras contra o vanilla
  mostrarem desvio, estender `layout_stretchy_glyph_horizontal` para reportar descent com sinal
  do resultado esticado.

### Proveniência

- Commit base: `c9df6fde`.
- Commit que integra P922 (sem commit isolado): `da18ea9f3`.
- Binário vanilla: `lab/typst-original/target/release/typst` (`typst 0.15.0`).
- Benchmark e PDFs de comparação: `tools/perf/results/p922/` e `tools/perf/corpus/p922/`.
- Benchmark canônico depois/antes: `tools/perf/benchmark-p922923-canonical.py`,
  `tools/perf/results/p922923-canonical/attestation.json`.
