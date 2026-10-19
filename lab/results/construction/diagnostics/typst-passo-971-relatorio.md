# Relatório — Passo 971 (limites de integral não seguiam a inclinação do símbolo)

**Data:** 2026-08-05 · **Gate:** Fase B confirmada pelo dono em 2026-08-05
("Continue" após `typst-passo-971-faseA.md`) — método novo no trait
`FontMetrics` (ADR-0127 ponto 1).
**Proveniência**: HEAD no início da Fase B = `225b96a42` (P969); a árvore
também continha P970-parte-2 não commitada (mesma confirmação de gate).
Benchmark: `temp/p971/typst-antes` = release com P970-parte-2.

## Fase A (recap — relatório `typst-passo-971-faseA.md`)

O vanilla subtrai `base.italics_correction()` do kern do subscrito
pós-fixado (`scripts.rs:220-228`); IC de `integral.v1` = 450du = 4.95pt a
11pt = exactamente o delta medido (sup a 10.99 vs sub a 6.04 da aresta
esquerda do símbolo). Proxy via ink bounds refutado (−56du). Kerns MATH
dos glifos de integral: nenhum.

## Fase B — o que foi implementado

- **Trait** (`01_core/src/engine/layout/metrics.rs`):
  `fn italics_correction(&self, glyph_id: u16, size: Pt, style:
  &TextStyle) -> Pt` com default `Pt(0.0)` + delegação explícita no
  `impl FontMetrics for &dyn FontMetrics` (lição de P906: sem ela, o
  default do trait aplicar-se-ia silenciosamente). Por glyph id, não char:
  a IC da variante (450du) difere da do glifo base (180du).
- **Backends** (`03_infra/src/font_metrics.rs`, `infra/font_metrics.md`
  §P971): leitura de `MathItalicsCorrectionInfo` por glyph id em
  `FontBookMetrics` (face única) e `FallbackFontMetrics` (face MATH activa
  da cadeia, P893). Sem entrada na coverage ⇒ 0.
- **Termo** (`attach.rs`, braço de scripts): `base_ic` extraída do glyph
  id real da `base_box` (`FrameItem::Glyph`, P906) antes de ela ser
  consumida; `kern_sub − base_ic`. Sobrescrito inalterado; a largura da
  caixa (`post_width`) reflecte o kern reduzido, como no vanilla.
- **Escopo honesto** (registado no L0): só bases com `FrameItem::Glyph`
  (bases esticadas — o caso do achado). Bases `Text` de um carácter com
  IC>0 (∫ inline, IC=180du; letras itálicas, ICs pequenas) ficam como
  residual — precisam de um resolvedor char→gid no trait (contrato
  adicional, a avaliar com medição de impacto).
- **Oráculo** (P969): `post_subscript_kern(kern, ic) = kern − ic`
  (`scripts.rs:220-228`).

**Testes** (`p971_sub_de_integral_display_recua_italics_correction` +
guarda `p971_base_sem_italics_correction_inalterada`, no mod
`p952op_tests`; stub ganhou `with_int_ic`). Nota de processo: escrevi a
implementação antes dos testes neste passo — o RED foi confirmado
**retroativamente** (revertendo o termo: sub obtido 11.988 = mesma coluna
do sup, exactamente o bug; com o termo: 6.588). Guarda verde nas duas
direcções. Suite completa: **5742 testes, 0 falhas** (+2).

## Fase C — Revalidação

- **Caso mínimo** (`$ integral_a^b f(x) dif x $`, temp/p971/min.typ): sub
  "a" a **6.04pt** da aresta esquerda do ∫ (vanilla: 6.04); sup "b" a
  **10.99pt** (vanilla: 10.99). Verticais inalteradas (93.90/70.00 antes
  e depois).
- **Documento de 30 secções** (secção 4, primeiro integral): sub a
  **6.039**, sup a **10.989** — vanilla: **6.039/10.989**. Igual ao
  milésimo.
- **compare.py sec 4/25**: 173→182 acima do limiar (med|dx| 0.638→0.662
  / 1.32→2.422) — **artefacto de emparelhamento**: as subscrições mudaram
  de posição absoluta (porque agora estão certas) e o compare.py emparelha
  por texto com distâncias absolutas de página, penalizadas pelas
  diferenças pré-existentes de centragem de bloco. A medição directa
  (6.039 = 6.039) é a evidência fiável (lição P949, já aplicada em
  P967/P970).
- **Benchmark** (`benchmark-p971-canonical.py`, 7 cenários,
  `tools/perf/results/p971-canonical/`): 01-hello 0.977 · 02-lorem 0.998 ·
  03-images 1.019 · 04-math 0.982 · 05-tables 1.002 · 06-long 1.047 ·
  07-context 0.997 — rácio médio **1.003**. O 1.047 em 06-long ficou fora
  da banda habitual e foi **re-medido isoladamente** (hyperfine, 30 runs):
  1.01 ± 0.03 — era ruído de carga concorrente (builds a correr durante a
  primeira medição). Sem regressão.
- **Linter**: resselo dos ficheiros tocados; `crystalline-lint .` →
  0 violations (só o V7 órfão pré-existente).

## Resultado

- Subscritos de `∫`/`∮` em display acompanham a inclinação do símbolo,
  exactamente como o vanilla (6.039pt medido nos dois) — achado 9.2
  fechado para bases esticadas.
- Distâncias verticais (P959/P914/P963) e bases sem IC intactas (guardas
  verdes).
- Residual registado: bases `Text` com IC>0 (∫ inline, letras itálicas).
- Benchmark sem regressão; suíte verde; linter limpo.
