# Relatório — Passo 967 (espaço ausente na transição matemática → palavra, secção 8)

**Data:** 2026-08-05
**Proveniência das medições** (regra de proveniência): HEAD no início do passo =
`9c8218b00` (P966). Todas as alterações de P967 estavam **no working tree não
commitado** durante as medições — ficheiros alterados (`git diff HEAD --stat`):
`00_nucleo/prompts/engine/layout/equation.md`,
`00_nucleo/prompts/engine/math/layout/_comum.md`,
`01_core/src/engine/layout/equation.rs`,
`01_core/src/engine/layout/tests.rs`,
`01_core/src/engine/math/layout/{mod,matrix,spacing,tests}.rs`. Binários:
debug construído ~11:44, release reconstruído ~11:53 (depois de copiar o
release anterior — binário de P966, construído 00:29 — para
`temp/p967/typst-antes`).

A auditoria externa reportava um sintoma; a Fase A encontrou **dois bugs
distintos** com o mesmo sintoma visível ("espaço some entre matemática e a
palavra seguinte"):

- **Bug 1** — transição equação **inline** → texto no fluxo de parágrafo
  (o candidato apontado no passo). Reproduzido no mínimo `$ 3x + y = 9 $ dado`.
- **Bug 2** — limite `&` da **grelha multiline** antes de anotação entre
  aspas (`&& "dado"`), o caso real da secção 8. Reproduzido em
  `temp/p967/sec8.typ` (fragmento real do documento).

Ambos corrigidos neste passo.

---

## Bug 1 — cursor após equação inline não incluía o espaçamento de classe

**Medição** (Fase A, `pdftotext -bbox`, caso mínimo): "dado" era colocada
**sobre** o "9" — início em x=121.19 com o "9" a terminar em 129.44; no
vanilla "dado" inicia em 132.50.

**Causa** (`01_core/src/engine/layout/equation.rs`, braço inline): o cursor
de fluxo acumulava `advance()` por item, mas as posições `pos.x` dos items
já incluem o espaçamento de classe embutido por `compute_gaps` (P772y) —
o THICK à volta do `=` no fim da equação ficava de fora do cursor, e o
texto seguinte era colocado cedo demais.

**Correcção**: nos braços `FrameItem::Text` e `FrameItem::Glyph`, o cursor
passa a `max(cursor, abs_pos.x + advance)` (if/else — `Pt` só tem
`PartialOrd`). L0: `prompts/engine/layout/equation.md` §P967.

**Teste** (RED confirmado antes do fix):
`p967_equacao_inline::p967_texto_apos_equacao_com_relacao_nao_sobrepoe`
em `01_core/src/engine/layout/tests.rs` — falhava com "dado (x=130.267) <
fim do 9 (134.667)".

**End-to-end** (caso mínimo): "dado" agora em x=132.19, gap 2.75pt —
igual ao vanilla.

## Bug 2 — limite `&` da grelha sem o espaço de texto (item espaçado)

**Medição** (Fase A, fragmento real da secção 8): `9 && "dado"` → "9dado"
colado (0pt); vanilla: gap de 3.652pt = largura do espaço da fonte
(NewCMMath-Book). Mecanismo vanilla: strings em math são "items espaçados"
(`process.rs::spacing()`, ramo `is_spaced() => return space`) e o espaço
atravessa o align point (a grelha vanilla é um único run).

**Causa dupla**:

1. `align_boundary_spacing` (P825, `matrix.rs`) usava `spacing_between`
   (catch-all = 0.0) — sem o fallback de item espaçado de P903
   (`compute_gaps`: catch-all E um dos lados `Content::Text`/`Fence` ⇒
   `advance(" ")`).
2. `layout_grid` (multiline `&`/`\\`) passava `&[]` a `layout_grid_boxes` —
   nunca incorporava espaçamento de limite. Como `partition_grid` só parte
   em `MathAlignPoint`, todos os limites multiline são `&` por construção.

**Correcção** (L0: `prompts/engine/math/layout/_comum.md` §P967b):

- `align_boundary_spacing` passa a método `pub(super)` de `MathLayouter`
  (precisa de `self.metrics`) com a semântica de P903: `spacing_between_class`
  e, só no catch-all com item espaçado, `metrics.advance(" ", …)`. Regras
  explícitas de 0.0 (pontuação/abertura/fecho) continuam a ganhar.
  `spacing_between_class` passou a `pub(super)` em `spacing.rs`.
- `layout_grid` mede as células, incorpora o gap na largura da célula
  esquerda (modelo P825 das matrizes) e passa `align_boundaries` todos
  `true`. O caminho de matrizes sem `&` (`layout_grid_rows`) inalterado.

**Testes** (`p967b_tests` em `math/layout/tests.rs`, RED confirmado no 1º):
- `p967b_grid_limite_com_string_leva_espaco_de_texto` — `9 && "dado"`:
  gap = `advance(" ")` = 7.2pt com FixedMetrics a 12pt (antes: 0pt).
- `p967b_limite_com_relacao_mantem_espaco_de_classe` — guarda P825: `x &= y`
  continua THICK (a regra explícita ganha ao fallback).
- `p967b_limite_antes_de_fecho_sem_espaco` — guarda: `Text` antes de `)`
  continua 0.0 (regra explícita).

## Fase C — Revalidação

**As 4 ocorrências da secção 8** (documento completo, gap tinta-a-tinta
antes da anotação; vanilla entre parênteses):

| anotação | antes (P966) | depois | vanilla |
|---|---|---|---|
| `"dado"` | 0.00 | **3.652** | 3.652 |
| `"multiplique por 7"` | 0.00 | **3.652** | 3.652 |
| `"subtraia y"` | 0.00 | **3.652** | 3.960 |
| `"divida por 3"` | 0.00 | **3.652** | 4.850 |

Os dois desvios residuais têm domínio próprio já catalogado, não deste bug:
"subtraia" (0.31pt) é advance itálico do `y`; "divida" (~1.2pt) é a largura
da barra da fracção `y/3` (domínio de P952) — o espaço em si é 3.652pt a
partir da borda da caixa da fracção, como no vanilla.

**compare.py secção 8** (`temp/p967/doc-c.pdf` vs `temp/p957/doc-vanilla.pdf`):
med|dx| **8.252 → 4.231**, glifos >0.5pt **111 → 101** (de 123 emparelhados).
Os outliers restantes são dominados por artefactos de emparelhamento do
compare.py nesta secção (o vanilla funde palavras como `𝑛(𝑛` e `\=` que o
cristalino emite separadas; pares com bx=0 exacto são default de extracção) —
lição de P949; a medição directa acima é a evidência fiável.

**Alcance (Fase C.2)**: o documento não tem outras ocorrências de
matemática inline em prosa (todas as equações são isoladas) — o bug 1 não
tem mais casos neste documento. O padrão `& "..."` aparece também em
`cases` (secções 9 e 26): medido **inalterado** por este passo (gaps
10.142/5.500/… idênticos antes e depois) porque `cases` não parte em align
points — o espaço de "se" já vinha de `compute_gaps` (P903) dentro da
sequência da linha, e já estava próximo do vanilla (10.14 vs 9.44, 5.50 vs
4.27). Sem regressão aí.

**Guarda de não-regressão**: espaçamento palavra-palavra comum
("multiplique por") medido igual nos dois lados (14.9pt/3.65pt entre
palavras, inalterado). Suíte completa: **5720 testes, 0 falhas**
(4876 core + 762 infra + 41 + 37 + 2 + 2), +3 novos p967b.

**Benchmark** (`tools/perf/benchmark-p967-canonical.py`, hyperfine,
7 cenários, resultados em `tools/perf/results/p967-canonical/`):
01-hello 1.005 · 02-lorem 1.014 · 03-images 0.992 · 04-math 0.998 ·
05-tables 1.002 · 06-long 1.003 · 07-context 0.995 — **rácio médio 1.001,
zero regressão** (a alteração toca só layout de grelha math e cursor de
equação inline; 1.014 no lorem está dentro do ruído habitual da máquina).

**Linter**: `crystalline-lint --fix-hashes .` (resselo de `equation.rs`,
`math/layout/mod.rs`, `math/layout/tests.rs`) seguido de
`crystalline-lint .` → **0 violations** (permanece só o V7 órfão
pré-existente de `package_version_resolution.md`, alheio a este passo).

## Resultado

- Causa exacta confirmada — e era **dupla**: cursor de fluxo sem o
  espaçamento de classe embutido (bug 1) e grelha multiline sem o fallback
  de item espaçado no limite `&` (bug 2, o caso real da secção 8).
- Espaço restaurado nas 4 ocorrências (3.652pt = largura do espaço da fonte,
  igual ao vanilla em "dado" e "multiplique"; desvios residuais com domínio
  próprio já catalogado).
- Benchmark sem regressão; suíte verde; linter limpo.
