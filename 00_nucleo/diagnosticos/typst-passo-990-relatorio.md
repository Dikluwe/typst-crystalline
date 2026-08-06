# Relatório — Passo 990: fracção Display + FRAC_PADDING + itálico de MathCancel/Strike

**Estado do código das medições**: HEAD `59d2a2f3d` (P988-B) + alterações
deste passo. Commit final no fim.
**Gate ADR-0127**: aprovado pelo dono em 2026-08-06 para os campos novos de
`MathConstants` (Parte A, ver `entities/math_constants.md` §P990) e para
`FRAC_PADDING`/braços novos de `apply_math_default`/`layout_node` (correcções
de fórmula interna e de paridade — fluxo contínuo, sem novo contrato público
além dos campos já aprovados).

## Fase A — três achados independentes da auditoria (§8.5/§8.6/§8.1)

- **Parte A** — gaps de fracção Display com sobreposição real (`ρ/ε₀`,
  secção 11): `layout_frac` usava sempre as constantes de TEXTO
  (`fraction_numerator_shift_up` etc.), mesmo quando `style.math_size ==
  Display`. Vanilla (`fraction.rs:30-53`) selecciona por `MathSize`. As 4
  constantes Display (`entities/math_constants.md` §P990) foram verificadas
  contra a tabela MATH real de `NewCMMath-Book.otf` **e** `NewCMMath-Regular.otf`
  (fontTools): 677/686/120/120du em ambas — mesmos valores do fallback.
- **Parte B** — espaço ausente após `−` em expoente (`𝑒⁻ᵗ²`): não é
  espaçamento de classe — é `FRAC_PADDING = 0.1em` (`math/frac.rs:9` do
  vanilla), que a fracção seguinte ao `−` nunca recebia. `layout_frac`
  desenhava a barra e centrava o conteúdo em `width = max(num,den)` nua, sem
  os `2×padding` que o vanilla soma à largura total (`fraction.rs:56`).
- **Parte C** — `cancel(a+b)`/`std.strike(a+b)` (secção 10, linhas 164-165 de
  `typst-math-comprehensive-test.typ`) renderizavam o corpo em glifo RETO:
  `apply_math_default` (P961/P966) não tinha braço para `Content::MathCancel`
  nem `Content::Strike` — caíam no catch-all `other => other.clone()`, o corpo
  nunca recebia o itálico por defeito. `Content::Strike` caía também no
  catch-all de `layout_node` (`plain_text()`), que além do itálico perdia a
  própria linha (decoração nunca desenhada em contexto math).

## Fase B — TDD

L0 primeiro (já em `frac.md` §P990-A/§P990-B, `_comum.md` §P990-C,
`entities/math_constants.md` §P990 — redigidos antes desta implementação).

Implementação:
- `frac.rs` — `is_display = style.math_size == MathSize::Display`; shift_up/
  shift_down/gap-floors seleccionados por `is_display` (Parte A). `line_width
  = max(num,den)`; `padding = 0.1×style.size`; `width = line_width +
  2×padding`; barra desenha-se de `(width−line_width)/2` a `+line_width`, não
  margem-a-margem (Parte B).
- `mod.rs` — `apply_math_default` ganha braços `MathCancel`/`Strike`
  (recursão no corpo, espelho do braço `MathAccent` de P961). `layout_node`
  ganha braço `Content::Strike` → `layout_strike` (novo método): linha
  horizontal com a mesma geometria do lado de texto
  (`engine/layout/decorations.rs`: offset por omissão `-0.25em`, thickness
  `max(0.05em,0.4pt)`, extent simétrico), baseline-relativa (ADR-0123).

GREEN: 6 testes novos (`p990a_*`, `p990b_*`, `p990c_*`) + 2 testes
pré-existentes actualizados (formula changed, não regressão):
`p952_frac_display_geometria_cresce_para_tamanho_cheio` (constantes Display
em vez das de texto, valores recalculados: ascent 15.396→16.524pt, descent
4.14→8.232pt, a 12pt sintético) e
`export::tests::p296_math_cancel_emite_body_e_linha_diagonal_no_pdf` (base
multi-carácter `"xy"` em vez de `"x"`, mesmo padrão do teste irmão de P961 —
com 1 letra, o itálico por defeito agora aplicado dentro de `MathCancel`
produz 𝑥, que o caminho Type1 sem fontes escapa para `?`, não pesquisável).
4928+787 testes verdes (workspace completo). `crystalline-lint .`: 0
violations (só V7 órfão pré-existente, não relacionado).

## Fase C — Revalidação

**Achado metodológico**: `typst-math-comprehensive-test-oracle.pdf` (cache
no repo) revelou-se não-reprodutível neste ambiente — uma regeneração directa
com `lab/typst-original/target/release/typst` (vanilla real) sobre o mesmo
`.typ` produz contagens de glifos DIFERENTES por secção (ex.: secção 30:
282 vs 128 glifos), indicando resolução de fontes diferente da usada para
gerar o cache (falta de `--font-path` explícito para o conjunto completo de
símbolos). Diff whole-document contra este cache não é sinal fiável para
validar Parte A/B nesta sessão — descartado como método.

**Verificação usada em substituição**: reprodução isolada, mínima, dos três
achados (`.typ` de 2-4 linhas, mesmas `#set page`/`#set text` do documento
canónico), comparando `target/release/typst` (cristalino, `04_wiring` —
**não** `02_shell`, que não produz o binário `typst`) contra
`lab/typst-original/target/release/typst` (vanilla real, `Typst 0.15.0`)
directamente, via `tools/geometry/compare.py`:

- `$ a / b = c $` isolado: gap vanilla = **14.993pt**, gap cristalino =
  **14.993pt** (idêntico). Largura da página: vanilla 84.41902pt, cristalino
  84.42pt (Δ0.001pt).
- `$ nabla dot E = rho/epsilon_0 $` + `$ ... e^(-t^2/2) ... $` (repro directa
  da secção 11): gap `ρ`/`ε₀` = 14.993pt em ambos; advance `−`→próximo glifo
  no expoente = 6.760/6.761pt (vanilla/cristalino, Δ0.001pt). Resto da linha
  com deriva pequena (max|dx|=4.5pt ao longo de ~15 glifos com √/∫/scripts —
  classe de resíduo já catalogada, não nova).
- P990-C: coberto por TDD directo (`layout_equation` produzindo 𝑎 itálico +
  `FrameItem::Line`), mesmo padrão de `p962_tests`/`p966_tests`.

**Achado novo, fora de escopo, catalogado**: comparação isolada com e sem
fracção (`$ a+b=c $` vs `$ a/b=c $`, sem alterações deste passo) mostra o
mesmo excesso de ~20pt de altura de página (`page(height:auto)`) em
cristalino face ao vanilla, presente também no binário pré-P990 (`30f7389f8`)
— bug pré-existente no espaçamento vertical de equação-bloco, **não
introduzido nem corrigido por este passo**. Candidato a passo próprio; não
bloqueia P990 (as fórmulas de fracção/itálico, isoladas do efeito de
`page(height:auto)`, batem com o vanilla real ao milésimo de ponto).

Benchmark canónico (`benchmark-p990-canonical.py`, antes = release P989
`30f7389f8`): 7 cenários, ratio médio **1.006** (01-hello 1.013, 02-lorem
1.009, 03-images 1.017, 04-math 0.987, 05-tables 1.000, 06-long 1.001,
07-context 1.017). **Sem regressão sistemática.**
