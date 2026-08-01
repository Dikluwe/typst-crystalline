# Passo 946 — Relatório

**Data**: 2026-08-01
**Estado da árvore na medição**: commit `019dfbd5c` (P945, HEAD de `Tekt`). Working tree:
untracked `typst-passo-946.md`, `test_crystalline.pdf`, `test_vanilla.pdf`.
**Artefactos**: `temp/p946/` (repros, PDFs, traces, crops de alta DPI).

---

## 1. Resumo executivo

Os dois sintomas reportados são **artefactos de rasterização a baixa DPI**, não defeitos de
render — provado a nível de byte (charstrings idênticos nos três: cristalino, vanilla,
fonte original) e por renders a 300dpi equivalentes nos dois compiladores. O único defeito
real confirmado era na **extracção de texto** (ToUnicode) das peças de assembly —
corrigido conforme decisão do dono, como **divergência deliberada** (codepoints reais das
peças, `⎧⎪⎨⎪⎩`). O render ficou **pixel-idêntico** (comprovado). Achado extra registado:
`binom` usa mecânica de matriz (P899) em vez de frac-like do vanilla — fica para o passo
de frações (colide com o scope-out de P944 §8.3.6).

## 2. Fase A — os dois sintomas, medidos

### 2.1 `binom(n, k)` "dois delimitadores separados" — REFUTADO

- Estrutura: `eval/math.rs:625` constrói `Content::math_matrix([[n],[k]], ('(', ')'))` —
  **um** par de delimitadores sobre a grelha de 2 linhas; nunca um por linha.
- O delimitador desenhado é a variante `parenleft.v6` (seleccionada para o alvo de
  2311.2du — `select_variant` sobre 8 variantes reais). O glifo embutido no PDF cristalino
  tem **charstring byte-a-byte idêntica** à do PDF vanilla e à da fonte original
  (programa, operandos e hints `vstem` iguais — verificado via pikepdf/fontTools).
- A 300dpi, cristalino e vanilla renderizam `(n/k)` com um único par de parênteses
  contínuo, equivalentes (`temp/p946/zoom-binom-hi.png`, `binom-150-cmp.png`).
- A 72dpi, **os dois** compiladores perdem o traço fino do meio da variante — os ganchos
  grossos do topo/fundo parecem "dois delimitadores" (`binom-72-cmp2.png`, lado a lado).
- Medição adicional (real, mas fora do sintoma): passo entre linhas do binom 13.16pt
  (cristalino, mecânica de matriz) vs 15.0pt (vanilla, `resolve_vertical_frac_like` —
  `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs:713-748`). Ver §5.

### 2.2 Gancho inferior da chave de `cases()` "glifo errado" — REFUTADO no render

- A assembly da chave tem 5 peças; os charstrings embutidos são **idênticos** aos da
  fonte: `uni23A7` (topo), `braceleft.ex` (extensores), `uni23A8` (meio), `uni23A9`
  (fundo) — cada peça na sua posição correcta (ordem da fonte fundo→topo, verificada com
  a inversão de eixo correcta do trace).
- A 300dpi, o fundo da chave do cristalino é **idêntico** ao do vanilla
  (`temp/p946/cases-bottom2-cmp.png`).
- A suspeita de "codepoint errado" era verdade **na extracção de texto** — ver §3.

## 3. O defeito real: ToUnicode das peças de assembly

Antes da correcção, a extracção de uma assembly saía com os caracteres de conveniência de
P906 (ganchos → char base, extensores → `|`/`_`): chave de `cases` extraía `{|{|` (+
`{`).

**Medição do comportamento do vanilla** (ToUnicode CMap de PDF real, lida via pikepdf):
o vanilla **não** emite codepoints reais — agrupa a assembly por cluster no char de
origem (a chave extrai como **um único `{`**; extensores e peça do meio não mapeados). A
premissa inicial do passo ("o vanilla emite os codepoints reais") foi **refutada** por
esta medição e reportada ao dono antes da decisão final.

**Decisão do dono**: manter os **codepoints reais** das peças quando a cmap da face os
cobre (NewCMMath cobre U+239B–U+23A0, U+23A1–U+23AA — verificado) — divergência
deliberada (ADR-0107), extracção semanticamente mais rica e acessível.

## 4. Fase C — correcção implementada (TDD directo, mapeamento pontual)

- L0: `00_nucleo/prompts/infra/font_metrics.md` §P946 (com a medição do vanilla e a
  decisão registadas).
- `03_infra/src/font_metrics.rs::build_math_glyph_reverse_map`: constrói o inverso da
  cmap da face (um passe) e prefere o codepoint real; fallback anterior
  (`base_char`/`|`/`_`) para glifos não codificados (variantes `.vN`; extensores sem
  codepoint próprio, ex.: `braceleft.ex`, glifo distinto de `uni23AA`).
- Teste `p946_reverse_map_assembly_usa_codepoints_reais_da_cmap` (red → green): chave →
  `[⎩, |, ⎨, |, ⎧]`; parêntese → `[⎝, ⎜, ⎛]`; variantes `.vN` → base char (inalterado).
- Extracção end-to-end confirmada: `pdftotext` do `repro.typ` mostra `⎧0`, `⎨`, `⎩`.
- **Render inalterado — prova pixel a pixel**: o documento de 30 secções renderizado
  antes/depois da correcção é **idêntico** (`ImageChops.difference` → bbox `None`).
- Validação: `cargo test --workspace` → **4821 + 749 + 41 + 2 + 37 + 2 = 5652 passed,
  0 failed**; `crystalline-lint .` → zero violations (resta só o V7 pré-existente alheio).

## 5. Achados registados (não tratados neste passo)

1. **`binom` com mecânica de matriz em vez de frac-like** (P899): passo entre linhas
   13.16pt vs 15.0pt do vanilla (`resolve_vertical_frac_like`, `resolve.rs:713-748` — o
   binomial é uma fracção sem barra, com `FractionItem`). Corrigir agora pioraria o
   binom (o caminho `frac.rs` cristalino ainda reduz num/den para ×0.7 — scope-out de
   P944 §8.3.6). **Candidato a incluir no passo de frações**: quando `frac.rs` ganhar a
   descida por nível, o binom pode migrar para frac-like no mesmo movimento.
2. **Lição de processo (reforça a nota do passo)**: as imagens de revisão de geometria
   têm de ser geradas a **≥200–300dpi** — a 72–150dpi os traços finos de variantes
   esticadas desaparecem **nos dois compiladores** e simulam defeitos inexistentes (foi
   a origem dos dois sintomas deste passo). Registado também o formato da verificação
   byte-level (charstring via pikepdf/fontTools) como ferramenta de attestation para
   glifos.

## 6. Fase D — revalidação e benchmark

- `review.typ` de P945 e documento de 30 secções recompilados: renders **pixel-idênticos**
  aos renders anteriores (a correcção é só ToUnicode); extracção agora com os codepoints
  reais das peças.
- Benchmark (hyperfine, warmup 1, min 10 runs; "antes" = binário do commit P945
  `019dfbd5c` compilado em worktree; "depois" = working tree P946; corpus
  `tools/perf/corpus/p922923-canonical`, JSONs em `tools/perf/results/p946-*.json`):

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 95.16 | 94.90 | 0.997 |
| 02-lorem | 113.17 | 114.18 | 1.009 |
| 03-images | 101.79 | 101.19 | 0.994 |
| 04-math | 126.03 | 128.38 | 1.019 |
| 05-tables | 98.41 | 98.99 | 1.006 |
| 06-long | 302.29 | 306.36 | 1.013 |
| 07-context | 137.23 | 138.36 | 1.008 |

Ratio médio **1.007** (desvios-padrão sobrepostos nos 7 cenários) — **zero regressão de
performance** dentro do ruído de medição.

## 7. Fecho

- `binom` e gancho da chave de `cases`: **sem defeito de render** — refutação provada a
  nível de charstring (byte-a-byte) e a 300dpi; os sintomas eram artefactos de
  rasterização a baixa DPI, partilhados com o vanilla.
- Única falha real (ToUnicode das peças de assembly) corrigida com a decisão do dono:
  codepoints reais da cmap (divergência deliberada, registada no L0 com a medição do
  comportamento do vanilla). Render **pixel-idêntico**; extracção end-to-end confirmada
  (`⎧⎨⎩` em `cases`, `⎛⎜⎝` em assemblies de parêntese).
- Suite 5652 verde, lint zero, benchmark 7 cenários sem regressão.
- `test_crystalline.pdf` (raiz) regenerado (render idêntico, extracção melhorada).
- Registos para o futuro: `binom` frac-like (junto do passo de frações, §5.1); geração de
  imagens de revisão de geometria a ≥200–300dpi (§5.2).
