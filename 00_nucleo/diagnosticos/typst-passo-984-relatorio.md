# Relatório — Passo 984: largura do acento (`̂`/`̃`) ~30% maior que o vanilla

**Estado do código das medições**: HEAD `b193b9fd8` (P983) + alterações deste
passo (working tree). Commit final deste passo no fim deste ficheiro.
**Gate ADR-0127**: não aplicável — correcção de paridade interna (assinatura
`pub(super)`, sem contrato público, sem mudança de fase/comportamento por
defeito fora da paridade). Fluxo contínuo: L0 editado primeiro + resselo.

## Fase A — causa confirmada (medição + leitura do vanilla)

Medição da série (`temp/p984/series-v.pdf` vanilla 0.15.1 vs `series-c.pdf`
cristalino, fonte NewCMMath-Book, 11pt; larguras de TINTA do acento por pixels
a 600dpi, e de caixa por `pdftotext`/hmtx):

| base | vanilla | cristalino (antes) |
|------|---------|--------------------|
| `x` | 5.50pt (glifo **base**, hmtx 500du) | 7.08pt (variante `.h1`, 647du) |
| `a b` | 5.50pt (base) | 5.50pt |
| `a b c` | 12.10pt (`.h4`, 1103du) | 12.10pt |
| `a+b` | 20.86pt (**maior variante** `.h7`, 1897du) | 5.50pt (**fallback ao base**) |
| `a b c d e` | 20.86pt (maior) | 20.86pt |

Dados da fonte (ttx NewCMMath-Book): variantes horizontais de `circumflexcmb`
= 307/647/771/922/1103/1323/1584/1897du (`AdvanceMeasurement`; índice 0 = o
próprio base), **sem assembly**; **hmtx do base = 500du ≠ AdvanceMeasurement
307du**. `tildecomb`: 338/653/…/1916du, mesma estrutura.

Leitura do vanilla (`fragment/glyph.rs:265-300` `stretch()`,
`math/accent.rs:18`, `ir/resolve.rs:389`/`:1430`):

1. `short_target = target − short_fall`, com short_fall **por chamador**:
   acentos = `ACCENT_SHORT_FALL = 0.5em` (`math/accent.rs:18`); spreaders
   (`underbrace`/…) = `Em::zero()` (`resolve.rs:1430`). O cristalino aplicava
   `DELIM_SHORT_FALL = 0.1em` fixo a todo o eixo X (P912) — errado para ambos.
2. Se `short_target ≤ advance hmtx do glifo base` → **mantém o base**
   (`glyph.rs:267-271`). O cristalino comparava contra o `AdvanceMeasurement`
   da tabela MATH (307du) em vez do hmtx (500du) → sobre-esticava bases
   estreitas (`hat(x)` → `.h1` = 7.08pt, o valor exacto medido).
3. Se nenhuma variante chega e **não há assembly** → fica com a **maior
   variante** (`glyph.rs:278-298`, o loop fica sempre com a última). O
   cristalino caía no fallback ao glifo base → sub-esticava bases largas
   (`hat(a+b)` → 5.50pt em vez de 20.86pt).

Nota de processo: duas derivações intermédias minhas nesta Fase A estiveram
erradas (short_fall assumido 0/0.1em) — a constante real (0.5em) só fechou
todos os pontos medidos depois de lida em `math/accent.rs:18`. A regra final
explica os 5 pontos da série sem excepção.

## Fase B — TDD

L0 primeiro: `stretchy.md` §P984 (mecanismo), `accent.md` §P984 (0.5em),
`underover.md` §P984 (0.0). Resselo com `--fix-hashes`.

RED confirmado (4/4 a falhar com os valores errados esperados — variante
647du para `hat(x)`, fallback 7.2pt para alvo grande, selecção com 0.1em para
spreader): `p984_acento_base_estreita_mantem_glifo_base`,
`p984_acento_base_media_estica_para_primeira_suficiente`,
`p984_sem_variante_suficiente_sem_assembly_mantem_maior_variante`,
`p984_spreader_short_fall_zero_seleciona_contra_alvo_inteiro`
(`01_core/src/engine/math/layout/tests.rs`, módulo `p906_tests`).

Implementação (`stretchy.rs`): `layout_stretchy_glyph_horizontal` ganha
`short_fall_em: f64`; keep-base via `metrics.advance()` do char (hmtx shaped)
convertido a du; keep-largest via `variants.variants.last()` quando
`select_variant` falha e o assembly está vazio; corpo de emissão de variante
extraído para `emit_horizontal_variant` (os dois ramos eram byte-a-byte
idênticos). Chamadores: `accent.rs` passa 0.5, `underover.rs` 0.0 (via
`layout_stretchy_or_node`, `mod.rs`). O `eprintln` de debug P984DBG (que
estava na árvore) foi removido nesta reescrita — **nota**: com ele em L1
(`std::env`) havia risco V4; fica limpo.

GREEN: 4/4 novos + suíte completa **5774 testes, 0 falhas**
(4907+785+41+2+37+2 por crate). Os 4 testes P906 pré-existentes que chamam a
função directamente foram actualizados para a nova assinatura (spreaders,
`short_fall = 0.0`) e continuam a passar.

`crystalline-lint .`: **0 violations** (só o V7 órfão pré-existente de
`package_version_resolution.md`, intocado).

## Fase C — Revalidação

Série de 5 larguras, tinta medida por pixels a 600dpi — **paridade exacta nos
5 casos** (cristalino = vanilla: 3.36 / 3.36 / 12.12 / 20.88 / 20.88pt).
Documento canónico de 30 secções (secção 10): `hat(x)` 3.36pt = vanilla
3.36pt (era 7.08pt); `tilde(x)` 3.60pt = 3.60pt (era 7.17pt). Confirmação
visual a 600dpi (`temp/p984/acc-c-1.png` vs `acc-v-1.png`): crops da secção
10 essencialmente idênticos, incluindo `bar`/`vec` (que o próprio documento
redefine — renderizam igual nos dois) e `dot(dot(x))`.

Benchmark canónico (`tools/perf/benchmark-p984-canonical.py`,
`temp/p984/typst-antes` = release P983, copiado antes do rebuild;
`tools/perf/results/p984-canonical/`): 7 cenários, ratios 0.985–1.016,
**média 1.002 — sem regressão**.

## Achados registados para passos seguintes

- **P985**: os gaps de `underbrace` invertidos são o passo seguinte; a leitura
  deste passo já confirmou que o spreader é um `AccentItem` com anotação em
  estilo de script (`resolve.rs:1416-1472`) e que o gap conteúdo↔chave do
  vanilla vem do ramo "bottom accent" de `layout_accent`
  (`gap = -accent.ascent()`), não de uma constante.
- Deslocamento vertical da 1ª equação após o título da secção 10 (~5pt vs
  vanilla) — deriva acumulada de espaçamento entre equações, fora do âmbito.
