# P784 — Verificação visual real: fallback de fontes matemáticas com glifo efetivamente ausente

> **Passo:** 784
> **Data:** 2026-07-17/18
> **Commit-base:** `a4bbc70d1ea0317536f662ef6901b80c277c9730` (`P783 — relatório em
> diagnosticos/paridade-producao-p783.md`) — working tree limpo no início deste
> passo.
> **Dependências:** P783 (implementação a verificar, commit `0c5ea946e`).

---

## Resultado em uma linha

**A verificação falhou.** P783 estava "correta mecanicamente" mas **nunca
funcionava** no caso comum — o glifo matemático ausente rendia com um glifo
**errado**, não tofu, de uma fonte de sistema aleatória. Dois bugs reais
encontrados e corrigidos por instrumentação directa, não suposição, conforme
mandatado pelo Passo 3 deste passo.

---

## Passo 0 — Glifo matemático real ausente em `Libertinus Serif`

`fontTools`, leitura directa de `lab/krilla-reference/assets/fonts/
LibertinusSerif-Regular.otf` e `NewCMMath-Regular.otf`:

| Candidato | Libertinus Serif | New Computer Modern Math |
|---|---|---|
| `ℕ` U+2115 | presente | — |
| `ℝ` U+211D | presente | — |
| `ℤ` U+2124 | presente | — |
| `𝔸` U+1D538 | **AUSENTE** | presente |
| `𝓐` U+1D4D0 | **AUSENTE** | presente |
| `⨿` U+2A3F | **AUSENTE** | presente |
| `⨅` U+2A05 | **AUSENTE** | presente |

Escolhido `⨿` (U+2A3F, N-ARY COPRODUCT) — não requer wrapping `bb(...)`/
`cal(...)` (evita conflar o teste com o pipeline de styling), testável como
Unicode literal directo (`$⨿$`).

---

## Passo 1 — Documento de teste real

```typ
$ ⨿ $
```

Vanilla: exit 0, `mutool trace` mostra `font="JJZKPV+NewCMMath-Book"`,
`unicode="⨿" glyph="1" adv=".75"`.
Cristalino (P783, antes de qualquer correcção deste passo): exit 0 também —
**mas** `font="AAAAAA+CrystallineFont"`, `unicode="⨿" glyph="uni27F8"
adv="1.609"`.

---

## Passo 2 — A cadeia de fallback é de facto percorrida?

**Não.** `glyph="uni27F8"` é o nome do glifo para U+27F8 (⟸, LONG LEFTWARDS
DOUBLE ARROW) — **um carácter completamente diferente** do pedido (U+2A3F).
Confirmado por extracção do subset embutido (`mutool extract`) e inspecção
com `fontTools`: os bytes crus do font program continham a string ASCII
`"MathJax_Main-Regular"` — uma fonte de **sistema** (não uma das fontes
esperadas: Libertinus Serif, NewCMMath, DejaVu). Confirmado visualmente
(`mutool draw` + crop): o glifo desenhado é de facto uma seta dupla, não o
símbolo de coproduto (comparar com o render vanilla, que mostra "Ⅱ"-like,
correcto).

**Isto é pior que tofu** — um glifo plausível mas errado, silenciosamente.

---

## Passo 3 — Investigação por instrumentação (mandatada, não suposição)

### Bug 1 — condição de gate nunca disparava no caso comum

`shaper.rs::try_shape`/`shaped_width` só injectavam a cadeia math quando
`primary_has_math` (a fonte **já resolvida**, tipicamente `Libertinus
Serif`, tem tabela MATH OpenType própria). Verificado directamente:

```python
TTFont('LibertinusSerif-Regular.otf'); 'MATH' in f  # → False
```

`Libertinus Serif` **não tem** tabela MATH — a condição nunca era verdadeira
para o caso comum (fonte de corpo por omissão). P783's próprio caso de teste
(`frac(a,b)`, `x^2_1`) usava só ASCII, então o `mutool trace` nunca mudou —
"correcto mecanicamente" não detectava isto.

**Correcção**: novo campo `TextStyle.math: bool` (`entities/layout_types.rs`),
definido `true` **uma única vez** em `layout/equation.rs::layout_equation`
(ponto de entrada do motor de layout matemático), herdado por
`..style.clone()` em toda a árvore. `shaper.rs` passa a disparar a cadeia
math com `style.math || primary_has_math` (OR — mantém o gatilho original de
P783 como caso adicional). Dois sites de construção não-spread de
`TextStyle` precisaram de valor explícito (`style_chain.rs::From<&
StyleChain>` → `false`; `layout/text.rs` → herda de `layouter.style.math`).

Confirmado por instrumentação temporária (`eprintln!`, removida após
confirmação): com a correcção, `style.math=true` chega correctamente a
`shaper.rs`.

### Bug 2 — nome de família errado (independente, descoberto pela mesma instrumentação)

Mesmo com `style.math=true` a disparar o bloco, a instrumentação mostrou
`"New Computer Modern Math"` a resolver **zero candidatos**. Medido por
leitura directa da tabela `name` (nameID 1 `FAMILY`) do ficheiro exacto
pinned em `Cargo.lock` (`typst-assets` rev `c0ae970`):

```python
TTFont('NewCMMath-Regular.otf')['name']  # nameID 1 → "NewComputerModernMath"
TTFont('LibertinusSerif-Regular.otf')['name']  # nameID 1 → "Libertinus Serif"
TTFont('NewCM10-Regular.otf')['name']  # nameID 1 → "NewComputerModern10"
```

O nome real das fontes "New Computer Modern *" é **sem espaços** — P783 (e a
sua "correcção" da heurística de `NewCMMath` → `NewCMMath` errado → "New
Computer Modern Math" também errado) nunca verificou por leitura directa,
só assumiu o nome "bonito" habitual. Confirmado via teste temporário que
imprimia os nomes de família efectivamente registados em
`load_embedded_fonts()`.

**Consequência lateral, mesma causa-raiz**: `NewCM10` (nome real
`"NewComputerModern10"`) também nunca correspondia a `starts_with("newcm10")`
— caía por acidente no `else` "math_code" de `embedded_font_group`, apesar
de dever estar em `"text"` (per a própria docstring de `EmbeddedFontSets`).
O teste `p754_newcm_math_is_not_in_text_group` passava sempre, mas
**vacuamente** — nunca encontrava a string em lado nenhum, não porque a
classificação estivesse correcta.

**Correcção**: `fallback_fonts.rs::DEFAULT_FALLBACK_FONTS_MATH[0]` e
`embedded_fonts.rs::embedded_font_group()` actualizados para os nomes reais
(`"NewComputerModernMath"`, `"NewComputerModern10"`, sem espaços). Teste
`p754_newcm_math_is_not_in_text_group` reforçado com asserção positiva
(confirma presença em `math_code`, não só ausência em `text`). Novo teste
`p784_newcm10_is_in_text_group`.

---

## Validação final

```
$ ⨿ $
  antes:  glyph="uni27F8" (seta, errado), font de MathJax_Main-Regular (sistema)
  depois: glyph="uni2A3F" (correcto), adv=".75" — bate exactamente com o
          adv=".75" do vanilla (mesma fonte NewCMMath, mesmo glifo)
```

Confirmado visualmente (`mutool draw -r 300` + crop, inspecção directa da
imagem): glifo "⨿" correcto, forma reconhecível, idêntica à do vanilla —
não tofu, não glifo errado. `compare -metric AE` entre PNGs vanilla/
cristalino: 844 pixéis diferentes numa página 2481×3508 (0.0097%) —
consistente com diferença de posicionamento/anti-aliasing entre
rasterizadores, não com erro de forma do glifo.

```
cargo build --workspace --release   → 0 erros
cargo test --workspace --release    → 4259+650+33+2+29+2 = 4975 passed, 0 failed
                                       (+2 testes P784: p784_newcm10_is_in_text_group,
                                       reforço de p754_newcm_math_is_not_in_text_group)
crystalline-lint . --fix-hashes     → 20 ficheiros re-hashed (7 L0 actualizados:
                                       layout_types.md, equation.md, style_chain.md,
                                       atomizacao_elementos.md, shaper.md,
                                       embedded_fonts.md — 2 delas, shaper.md e
                                       embedded_fonts.md, não tinham secção P783
                                       nenhuma: débito de documentação do próprio
                                       P783, fechado retroactivamente aqui)
crystalline-lint .                  → 0 drift (só V7 pré-existente, não relacionado)
```

Instrumentação temporária (`eprintln!` em `shaper.rs`, teste de debug em
`embedded_fonts.rs`) foi **removida por completo** após confirmar a causa —
não permanece no código final.

---

## Critério de fecho do passo

- [x] Glifo matemático real ausente em `Libertinus Serif` mas presente em
      `New Computer Modern Math` identificado (`⨿` U+2A3F).
- [x] Documento de teste real compilado nos dois compiladores.
- [x] Fonte efectivamente usada pelo cristalino confirmada — **inicialmente
      errada** (fonte de sistema `MathJax_Main-Regular`), corrigida para
      `NewComputerModernMath` (a fonte correcta da cadeia).
- [x] Glifo renderiza correctamente, confirmado visualmente — inicialmente
      **falhou** (glifo errado, não tofu), corrigido e reconfirmado.
- [x] Falhou: causa investigada por instrumentação directa (`eprintln!`
      temporário em `shaper.rs`, teste de debug em `embedded_fonts.rs`), não
      suposição — dois bugs reais encontrados e corrigidos.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p784.md` (este
      ficheiro), com o glifo escolhido e evidência textual (`mutool trace`)
      e visual (`mutool draw` + inspecção de imagem) documentadas.

---

## Próximo passo

Com este fechado — e realmente verificado, não só implementado às cegas —
não resta nenhum débito conhecido da série P765a-P784. Momento de fazer o
resumo final da série.
