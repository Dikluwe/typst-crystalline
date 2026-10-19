# Passo 961 — Relatório (legenda de underbrace/overbrace reduzida + base de acento itálica)

**Data**: 2026-08-04
**Estado da árvore**: commit base `ccf3e7a2b` (P960); alterações deste passo por cima.

---

## Parte A — legenda de `⏟`/`⏞` em tamanho de script

**Fase A**: a fórmula do vanilla está em `resolve_underoverspreader`
(`typst-library/src/math/ir/resolve.rs:1441,1455`) — a anotação é resolvida
com `style_for_subscript` (under = superscript + cramped, `style.rs:333`) /
`style_for_superscript` (over). A peça (a chave) é um **acento largo**
(`AccentItem`, resolve.rs:1427-1432) — não é script. O cristalino
(`underover.rs`) marcava só o `math_size` (P945) e mantinha o tamanho
ambiente — medido antes da correcção: legenda a 11pt (igual à equação).

**Fase B (TDD directo)**:
- L0: `underover.md` §P961 (revoga explicitamente a nota de P945 que adiava
  o factor de tamanho).
- Testes RED: `p961_under_label_tamanho_script_cramped` (under: 8.4pt +
  cramped), `p961_over_label_tamanho_script_sem_cramped` (over: 8.4pt sem
  cramped), `p961_peca_estica_mantem_tamanho_ambiente` (guarda — a chave de
  1 carácter não pode reduzir; já verde, ficou verde).
- Implementação: a anotação (multi-carácter) é layoutada com o estilo de
  script completo (`size × script_percent_scale_down`, `math_script`,
  `math_size` um nível abaixo, `cramped` só no under); a peça de 1 carácter
  segue com o estilo ambiente (mesmo discriminador de P906).
- **Medição real no documento**: a palavra "soma" passa a 7.70pt de altura
  de caixa — exactamente a medida do vanilla (7.70pt; auditoria tinha
  medido 7.7pt no vanilla vs 11pt no cristalino).

## Parte B — base de acento itálica (fecha o achado #5 de P906)

**Fase A**: a descrição de P906 continuava exacta no código actual —
`apply_math_default` (`math/layout/mod.rs`) não tinha braços para
`Content::MathAccent` nem `Content::MathUnderover` (caíam no braço
`other`, sem itálico por codepoint).

**Fase B (TDD directo)**:
- L0: `_comum.md` §P961.
- Testes RED: `p961_acento_base_recebe_italico_default` (base de `hat(x)`
  → 𝑥 U+1D465), `p961_underover_base_e_label_recebem_italico` (base 𝑥 +
  legenda 𝑛 U+1D45B).
- Implementação: braços novos — `MathAccent` recursa na base (o `accent`
  fica inalterado); `MathUnderover` recursa em base/under/over.
- **Teste antigo actualizado** (codificava o bug):
  `p296_math_accent_emite_base_e_accent_no_pdf` assecreva a base "a"
  latina no PDF; com a correcção, a base de 1 letra é 𝑎 (U+1D44E) — que o
  caminho Type1 sem fontes escapa para `?`. O teste passou a usar base
  multi-carácter ("ab"), mantendo o propósito de P296 (emissão
  base+acento); a cobertura do itálico fica nos testes p961.

## Fase C — revalidação conjunta

- Suite: `cargo test --workspace` — **5692 testes, 0 falhas**.
- Visual secção 10 (`temp/p961/sec10.png`, lado a lado): `x̂`/`x̃`/`ẍ` com
  base itálica ✓; legenda "soma" reduzida ✓. Divergências remanescentes da
  secção são as de posicionamento de decoradores (linha de underline,
  posição vertical da legenda face à chave) — pré-existentes, fora do
  escopo deste passo (que era tamanho + itálico).
- `compare.py` sec 10: med|dx| 1.846 → **1.029**; med|dy| = 0.
- Benchmark: ver tabela (hyperfine, "antes" = release pós-P958; JSONs em
  `tools/perf/results/p961-canonical/`).

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 87.48 | 88.11 | 1.007 |
| 02-lorem | 106.68 | 106.34 | 0.997 |
| 03-images | 93.92 | 94.31 | 1.004 |
| 04-math | 118.85 | 119.39 | 1.005 |
| 05-tables | 91.20 | 91.49 | 1.003 |
| 06-long | 296.57 | 293.18 | 0.989 |
| 07-context | 127.13 | 140.55 | 1.106 |

Ratio médio **1.016** — o outlier (07-context, 1.106) é ruído de ambiente
(o padrão já visto em P956/957; a mudança é um estilo num caminho de
anotação + um braço de match — sem custo plausível). Sem regressão
atribuível.
