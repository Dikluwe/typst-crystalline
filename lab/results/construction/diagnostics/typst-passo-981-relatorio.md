# Relatório — Passo 981 (`lr(` vazava como texto literal no PDF)

**Data:** 2026-08-05
**Proveniência**: HEAD no início = `344a63517` (P980). Benchmark:
`temp/p981/typst-antes` = release de P980.

## Fase A — causa confirmada

- **Reprodução** (`$ lr((a/b)) $`): o nome vazava como texto — `lr((𝑎))`
  no PDF (o vanilla: só os delimitadores esticados).
- **Causa** (`01_core/src/engine/eval/math.rs`): não havia braço para
  `lr` no dispatch de funções math — caía no fallback P302/P303 de
  identificador desconhecido (`MathSequence([MathIdent("lr"),
  MathDelimited(...)])`), que emite o nome como texto. A mesma classe de
  bug de P958 (nomes a cair em fallback literal).
- **Mecanismo do vanilla** (`typst-library/src/math/lr.rs` +
  `ir/resolve.rs:850-940`): `LrElem { body, size: Rel = 1 }` — o corpo
  inclui os delimitadores; `resolve_lr` estica o primeiro/último item de
  classe Opening/Closing/Fence ao conteúdo.
- **O redimensionamento já funcionava parcialmente**: o caminho
  `MathDelimited` estica delimitadores por omissão — daí o braço do eval
  poder ser fino (devolver o corpo, ou reescrever sequências com
  delimitadores soltos). **Residual adjacente medido e registado**: os
  delimitadores à volta de uma fracção ficam ~2pt aquém do vanilla
  (17.8pt vs 19.7pt de tinta, medido por pixel) — divergência de
  alvo/selecção de variante em `layout_delimited`, para passo próprio.

## Fase B — implementação (TDD directo; fluxo contínuo ADR-0127)

L0 primeiro (`engine/eval.md` §P981), depois testes RED (3 falharam —
vazamento confirmado), depois o braço `"lr"` no dispatch:

- 1 argumento posicional avaliado como math;
- corpo `MathSequence` com ≥2 itens, primeiro de classe Opening/Fence e
  último Closing/Fence (`entities::math_class::default_math_class` —
  paridade com a verificação do vanilla) → reescrito para
  `math_delimited(primeiro, meio, último)` — cobre
  `lr(chevron.l a/b chevron.r)` e `lr(\]a/b\[)`;
- qualquer outro corpo devolvido inalterado (grupo já delimitado —
  `lr((a/b))`, `lr([a/b])`, `lr({a/b})`, …).
- Scope-out registado no L0: named `size:`.

Testes (`tests_p981` em `eval/tests.rs`): os 3 casos (grupo, soltos,
chavetas) — RED→GREEN. Suite completa: **5768 testes, 0 falhas** (+3).

## Fase C — Revalidação

- **Documento de 30 secções**: **zero** ocorrências de `lr(` (antes:
  vazava nos 8 casos da secção 22). Secção 22 renderiza os 8
  delimitadores esticados à volta da fracção, visualmente iguais ao
  vanilla (crops lado a lado).
- **Comparação por sequência de caracteres** (o método pedido pelo
  passo): o multiset de caracteres da secção 22 é **idêntico** ao
  vanilla — `(9 (9 )9 [2 ]2 { } |2 ⌊ ⌋ ⌈ ⌉ ⟨ ⟩ 𝑎×8 𝑏×8` + dígitos dos
  números de equação, iguais nos dois lados. (A ordem de extracção do
  pdftotext difere item a item — mesma tinta, outra ordem de leitura.)
- **Benchmark** (`benchmark-p981-canonical.py`, 7 cenários,
  `tools/perf/results/p981-canonical/`): 01-hello 1.011 · 02-lorem 1.010 ·
  03-images 1.003 · 04-math 1.011 · 05-tables 1.008 · 06-long 0.994 ·
  07-context 0.992 — rácio médio **1.004**, zero regressão (um braço a
  mais num match de eval).
- **Linter**: resselo de `eval/math.rs` e `eval/tests.rs`;
  `crystalline-lint .` → 0 violations (só o V7 órfão pré-existente).

## Resultado

- O texto `lr(` deixou de vazar — nos 8 casos da secção 22 e em qualquer
  uso de `lr()`.
- Redimensionamento preservado (o que já funcionava ficou igual; a
  divergência de ~2pt na altura dos delimitadores é pré-existente e fica
  registada no L0 como residual).
- Benchmark sem regressão.
