# Relatório — Passo 991: `layout_grid` sem `&` centra em vez de alternar

**Estado do código das medições**: HEAD `e0e8ca4dc` (pós-P990, sync
`lab/typst-original`) + alterações deste passo. Commit final no fim.
**Gate ADR-0127**: fluxo contínuo — correcção de fórmula interna/paridade
com o vanilla, sem mudança de contrato público, comportamento por defeito
ou fase de pipeline. L0 editado primeiro (`_comum.md` §P991), hash
resselado (`8fbe7336`), sem paragem.

## Fase A — causa confirmada por leitura (código + vanilla)

Achado externo (2026-08-08, secção 7 de `typst-math-comprehensive-test.typ`,
`(n \ k) = n!/(k!(n-k)!)`): folga horizontal assimétrica dentro do
delimitador ("n": 0.00pt/0.00pt; "k": 0.70pt/0.17pt na medição original),
em vez de centrada como `binom(n, k)` (referência, 1.10pt/1.10pt e
1.45pt/1.62pt).

- `(n \ k)` avalia para `Content::MathDelimited { body: MathSequence([n,
  Linebreak, k]), .. }` (`eval/math.rs:319-327`, `Expr::Linebreak(_) =>
  Content::linebreak()`, `eval/math.rs:970`).
- `layout_delimited` → `layout_node(body)` → `layout_sequence` →
  `needs_grid_layout` (tem `Linebreak`) e `self.block` → `layout_grid`
  (`mod.rs`).
- `layout_grid` particiona com `partition_grid` (linhas por `Linebreak`,
  colunas por `MathAlignPoint`) e chamava **sempre**
  `GridAlign::Alternating`, mesmo sem nenhum `MathAlignPoint` nos nós de
  origem — 1 coluna, e `Alternating` (coluna par → direita) empurra o
  conteúdo mais estreito para a direita em vez de o centrar.
- Confirmado no vanilla: `math/equation.rs:200`
  (`AlignElem::alignment = Alignment::CENTER`, default de uma equação);
  `typst-layout/src/math/run.rs:113-140` (`stack_rows`):
  `has_alignment = !points.is_empty()` — com 1 coluna só (nunca houve
  `&`), `points` fica vazio, e a linha é posicionada por
  `pos.x = align.position(total_width - sub.width())` — o alinhamento
  **resolvido** (CENTER por omissão), não a alternância por paridade de
  coluna. `FixedAlignment::Center::position(extent) = extent/2.0`
  (`layout/align.rs:715-721`) — folga simétrica. A alternância só entra em
  jogo quando `has_alignment` é `true` (pelo menos um `&` real).
- `binom()`/`mat`/`cases` não são afectados: caminho separado
  (`matrix.rs`/`layout_grid_rows`), já `GridAlign::Center` desde a origem.

## Fase B — TDD

L0 primeiro (`_comum.md` §P991, hash resselado antes da implementação).

RED: 2 testes novos em `p991_tests` (`layout/tests.rs`), com
`FixedMetrics` (largura determinística por carácter, 0.6×size):
- `p991_grid_sem_alignpoint_centra_linha_mais_estreita` — `(n \ k)`
  sintético ("n" 1 char, "kk" 2 chars, só `Linebreak`, sem `&`):
  confirmado o bug tal como previsto (esq=7.2, dir=0.0 — "n" flush-right).
- `p991_grid_com_alignpoint_mantem_alternancia` — guarda de não-regressão,
  2 linhas × 2 colunas com `&` (largura de coluna variável por linha, para
  discriminar `Alternating` de `Center` — uma única linha não discrimina,
  a largura da coluna colapsa na da célula única). Já verde antes do fix
  (comportamento existente correcto, preservado).

Implementação (`mod.rs::layout_grid`): `n_cols = grid.iter().map(|row|
row.len()).max()` (mesmo valor já usado para `align_boundaries`);
`GridAlign::Center` quando `n_cols <= 1`, `GridAlign::Alternating` quando
`n_cols > 1` (inalterado).

GREEN: os 2 testes novos passam. Suíte completa, discriminada por crate:
**5797 testes verdes** (4930 `typst-core` + 787 `typst-infra` + 41
`typst-shell` + 37+2 `typst-wiring`), 0 falhas. `binom()` explicitamente
confirmado inalterado (3 testes: `calc_binom_combinacoes`,
`p899_binom_produz_mathmatrix_2_linhas_parenteses`,
`p899_binom_variadico_junta_lower_por_virgula_numa_celula`).
`crystalline-lint .`: 0 violations (só V7 órfão pré-existente,
`infra/package_version_resolution.md`, não relacionado).

## Fase C — Revalidação

**Medição com fonte real** (`.typ/typst-math-comprehensive-test.typ`,
secção 7, `binom(n,k)` e `(n \ k)` lado a lado): binário "antes" (release
de `e0e8ca4dc`, pós-P990, via `git worktree` isolado — sem tocar a working
tree) vs "depois" (release actual, com o fix). Extracção de posições reais
via `pdftotext -bbox` sobre os dois PDFs renderizados com o mesmo `.typ`.

- Metodologia validada contra a própria referência do achado: `binom(n,k)`
  medido nos dois PDFs bate exactamente com a tabela original — "n"
  1.10pt/1.10pt, "k" 1.45pt/1.62pt (idêntico em antes/depois, caminho não
  tocado).
- `(n \ k)`, **antes**: "n" 0.00pt/0.00pt (é a linha mais larga aqui —
  neste conteúdo real, "n" é mais largo que "k", ao contrário do
  sintético de teste; fica sempre flush, correcto para a linha mais
  larga). "k": `pdftotext` funde a palavra "k" com o glifo do delimitador
  de fecho `⎠` num único token (`237.9015`–`253.4225`) — sinal directo de
  folga ≈0 no lado direito, com quase toda a folga (≈0.87pt) empurrada
  para a esquerda. Reproduz exactamente o padrão do achado original.
- `(n \ k)`, **depois**: "n" 0.00pt/0.00pt (inalterado — continua a linha
  mais larga, corretamente flush em ambos os lados sob `Center` também).
  "k": 0.352pt/0.517pt — folga em ambos os lados, já não fundida com o
  delimitador. Assimetria residual (0.352 vs 0.517, Δ0.165) do mesmo
  sinal e ordem de grandeza da assimetria já presente em `binom()` "k"
  (1.45 vs 1.62, Δ0.17) — mesma causa (bbox de tinta de "k" itálico não
  centrado no seu próprio advance box), não um resíduo do bug de
  alinhamento.
- Confirmação visual: crop a 1200dpi da região do delimitador
  (`pdftoppm` + `convert`) — "k" visivelmente mais próximo do parêntese
  direito no "antes", visivelmente mais centrado no "depois".

Benchmark canónico (`benchmark-p991-canonical.py`, antes = release
`e0e8ca4dc`): 7 cenários, ratio médio **1.014** (01-hello 1.042, 02-lorem
1.010, 03-images 1.027, 04-math 1.008, 05-tables 1.010, 06-long 1.007,
07-context 0.996). Sem regressão sistemática (dispersão consistente com
ruído de medição — mudança é uma comparação O(1) adicional).
