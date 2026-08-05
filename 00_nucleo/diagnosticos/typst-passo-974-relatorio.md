# Relatório — Passo 974 (símbolo `√` com altura fixa, não escalava com o radicando)

**Data:** 2026-08-05 · **Gate:** campo novo em `MathConstants` confirmado
pelo dono em 2026-08-05 (pergunta directa após a Fase A — ADR-0127 ponto 1).
**Proveniência**: HEAD no início = `9d43474e5` (P973). Binários: debug
reconstruído durante a Fase B; release "antes" = binário de P971 (P973 não
mudou código) copiado para `temp/p974/typst-antes`.

## Fase A — causa dupla confirmada (não uma só)

Reprodução com medição de tinta por pixel (render 600 DPI + análise de
colunas de tinta — os bboxes do `pdftotext` são nominais e enganam para
glifos esticados):

| caso | cristalino (antes) | vanilla |
|---|---|---|
| `√(a²+b²)` | 10.80pt | **13.08pt** (variante `radical.v1`) |
| `³√x` | 10.80pt | 10.92pt (glifo base) |
| `√x` | 10.80pt | 10.92pt (glifo base) |

Instrumentação do caminho real (eprintln temporário, removido): o alvo em
`sqrt(a²+b²)` era **936du** → short-fall 836du → glifo base (1001du) cobre
→ nunca esticava. A variante correcta só é seleccionável cruzando 1001du,
o que exige **as duas** correcções (verificado nas contas — cada uma
sozinha é insuficiente):

1. **Gap de Display ausente** (`radical.rs:32-36` do vanilla): Display usa
   `RadicalDisplayStyleVerticalGap` = **148du**; o cristalino usava sempre
   `RadicalVerticalGap` (50du). +98du ao alvo. (Era o residual registado
   no adendo de P970 — confirmado como metade desta causa.)
2. **Short-fall não se aplica ao radical** (`resolve.rs:1246` do vanilla:
   `StretchInfo::new(Rel::one(), Em::zero())`; assinatura em
   `item.rs:1241`; selecção em `glyph.rs:265-271`). O cristalino subtraía
   `DELIM_SHORT_FALL` (0.1em — correcto só para delimitadores de
   `lr`/matrizes, P912). +100du ao limiar efectivo.

Verificação da combinação (11pt, base = 1001du): `√(a²+b²)` → 838+48+148
= **1034du > 1001** ⇒ v1 (ambos os lados); `√x` → 649du ⇒ base (ambos).

## Fase B — implementação

- **Parte A (fluxo contínuo)**: `layout_radical_symbol` em `stretchy.rs` —
  o caminho do radical partilha a implementação com os delimitadores
  (`layout_stretchy_delimiter_impl` com flag) mas não aplica short-fall.
  RED confirmado retroativamente (implementação precedeu o teste — com o
  caminho antigo, alvo 1050du caía para 950du e ficava o glifo base;
  registado).
- **Parte B (após gate)**: campo `radical_display_style_vertical_gap` em
  `MathConstants` (fallback 148.0, valor real medido) + leitura da tabela
  MATH em `font_metrics.rs` + escolha por nível em `root.rs`
  (`math_size == Display → 148du, senão 50du`). **TDD na ordem certa**:
  teste RED primeiro (`p974_display_usa_gap_de_display` — Display → v1,
  Text → base, com o mesmo radicando).

Testes (`p974_tests`, stub com as variantes reais de NewCMMath):
- `p974_radical_sem_short_fall_selecciona_v1` (Parte A);
- `p974_display_usa_gap_de_display` (Parte B);
- `p974_radical_pequeno_mantem_glifo_base` (guarda).

Suite completa: **5745 testes, 0 falhas** (+3).

## Fase C — Revalidação

- **Os três casos** (medição de tinta por pixel, 600 DPI):
  `√(a²+b²)` = **12.96pt** (vanilla 13.08, Δ0.12 dentro da tolerância da
  medição de render; era 10.80); `³√x` = **10.92pt** (vanilla 10.92,
  exacto); `√x` = **10.80pt** (vanilla 10.92, Δ0.12). O símbolo escala
  com o radicando — fim da "altura fixa".
- **Confirmação visual** (crops 300 DPI lado a lado): os três casos
  indistinguíveis do vanilla; sem sobreposição traço/radicando; o índice
  de `³√x` continua encaixado no vinco (P970 intacto).
- **compare.py** (secções de raiz): sec 1 — max|dy| **2.017 → 0.904**,
  med|dx| 1.147 → 0.638, emparelhados 75 → 78; secs 13/14 inalteradas em
  mediana (max|dy| de 14 melhorou 3.179 → 0.904).
- **Benchmark** (`benchmark-p974-canonical.py`, 7 cenários,
  `tools/perf/results/p974-canonical/`): 01-hello 1.005 · 02-lorem 1.006 ·
  03-images 1.014 · 04-math 1.005 · 05-tables 1.010 · 06-long 1.004 ·
  07-context 1.009 — rácio médio **1.008**, zero regressão (spread ±1.4%,
  dentro do ruído habitual da máquina).
- **Linter**: resselo dos ficheiros tocados; `crystalline-lint .` →
  0 violations (só o V7 órfão pré-existente).

## Resultado

- Causa exacta confirmada como **dupla** (gap de Display + short-fall
  indevido no radical), cada uma com `file:line` do vanilla e valores da
  fonte medidos — a leitura anterior do achado (9.1) como dois bugs
  separados fica substituída pela causa única, como o passo previa.
- Altura do √ escala com o radicando (12.96/10.92/10.80 vs vanilla
  13.08/10.92/10.92), testada em três tamanhos de radicando.
- P970 (índice) não regrediu (guarda visual + suite).
- Benchmark sem regressão; linter limpo.
