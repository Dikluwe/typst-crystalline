# Prompt L0 — `math/layout/accent` — `MathAccent`
Hash do Código: d30a553b

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/accent.rs`
**Origem**: fatiado de `math/layout/mod.rs` em **P909**, completando o padrão de fatiamento
iniciado em P314 (ADR-0104) para `frac`/`root`/`stretchy`/`assembly`/`matrix`/`cases`/
`delimited`. `layout_accent` foi adicionado em **P296**, depois de P314, e nunca tinha sido
movido. Núcleo partilhado: ver `math/layout/_comum.md`.

---

`MathAccent` — posiciona o glifo `accent` centrado horizontalmente acima de `base`. Heurística
minimal per ADR-0054 graded:
- Sem `dotless` (i/j) handling — base mantém glyph original.
- Sem `size` ratio — accent é width natural (antes de esticar, ver P906 abaixo).

## P906 — accent de 1 carácter estica para cobrir a largura da base

**Achado** (`typst-passo-899-relatorio.md`, Parte C adiada): `layout_accent` centrava `accent` no
seu tamanho natural (`let dx = (w - ob.width) / 2.0`), sem esticar para cobrir a largura da base —
`hat(a+b)` produzia um circunflexo do tamanho de 1 carácter sobre uma expressão larga, sem relação
visual com ela.

**Correcção**: antes de `self.layout_node(accent, style)`, um guard partilhado com
`layout_underover` (`layout_stretchy_or_node`, definido em `mod.rs` — ver `_comum.md` §guard
partilhado): se `accent` é `Content::MathText(s)` com exactamente 1 carácter, calcula
`min_width_du = base_box.width * self.constants.upem / style.size.val().max(0.001)` e chama
`layout_stretchy_glyph_horizontal` (`stretchy.rs`, ver `stretchy.md` §P906) em vez de
`layout_node`. Para qualquer outro conteúdo (multi-carácter, sequência) — comportamento
**inalterado**. Não verifica antecipadamente se o char tem dados de esticamento —
`layout_stretchy_glyph_horizontal` já faz fallback a `layout_text_node` quando não há variantes
nem assembly.

**Alcance confirmado com o dono**: acentos largos `hat`/`tilde` sobre base multi-carácter, já
aceite como limitação cosmética menor em P899 Parte A, corrigida como efeito colateral barato do
mesmo mecanismo usado por `layout_underover` (chaves/colchetes).

**Critério de regressão**: `hat(a+b)`/`tilde(a+b)` — a largura do `MathBox` do accent é maior que
a de `hat(a)` sozinho (esticou). Testes: `p906_layout_accent_1char_stretchy_domina_largura_apos_esticar`,
`p906_layout_accent_multicaracter_permanece_texto_literal` (`tests.rs:2201`, `tests.rs:2230`).

## P906 (cont.) — offsets Y usavam convenção "topo do box"

**Achado, só visível ao confirmar visualmente `underbrace(a+b+c, "soma")`** (estrutura aninhada —
`MathAccent`/`MathUnderover` usados como base de outro): mesmo padrão já corrigido em `frac.rs`
(P905) e `root.rs` (P901), nunca antes auditado em `layout_accent` — `accent`/`base` eram
posicionados por offsets crescentes a partir de `Pt(0.0)` (convenção "topo do box"), não pela
convenção baseline-relativa confirmada em P901 (`local_y=0` é a BASELINE PRÓPRIA da `MathBox`, y
cresce para baixo). Funcionava por coincidência quando a caixa era o conteúdo de topo da equação —
quebrava assim que fosse usada como sub-caixa de outra (aninhamento, ou `hconcat_spaced` com
irmãs).

**Correcção**: `base` fica em `Pt(0.0)` (a sua própria baseline já é `local_y=0`); `accent_y =
-(base_box.ascent + accent_box.descent)` (a baseline do accent sobe o suficiente para o seu
descent parar exactamente no topo da tinta da base). `ascent`/`descent` (valores escalares) não
mudaram — só os offsets dos items.

**Critério**: `MathAccent { base: a, accent: hat }` → item do accent não sobrepõe visualmente o
item da base quando `MathAccent` é usado como base de outro `MathUnderover`/`MathAccent`
(aninhamento).

## P915 — base do accent é cramped (incondicional neste ficheiro)

Achado do vanilla (`resolve_accent`, `resolve.rs:360-379` — ver
`entities/layout_types.md` §P915, `typst-passo-915-relatorio.md` Fase A): a
base só é cramped **quando o accent fica acima** (`position ==
Position::Above`) — o vanilla também suporta accent "abaixo"
(`accent.is_bottom()`), caso em que a base **não** é cramped.

**Confirmado no cristalino**: `MathAccentElem` (`entities/elements/
math_accent.rs`) só tem campos `base`/`accent` — sem posição — e
`layout_accent` (este ficheiro) é usado exclusivamente para os acentos que
P899 Parte A introduziu (`hat`/`tilde`/`dot`/`dot.double`), todos "acima"
por definição. A condição do vanilla é, portanto, **trivialmente sempre
verdadeira** neste ficheiro — não há ramo "abaixo" para excluir. `base`
passa a usar `base_style` (`cramped: true`) em vez de `style` directo, sem
condicional.

**Achado de âmbito, não implementado**: se o cristalino vier a suportar
accents "abaixo" no futuro, a condicional do vanilla (cramped só se
`position == Above`) terá de ser reintroduzida — este passo não a codifica
porque não há hoje nenhum caso que a exercite.

**Efeito prático**: um superscrito na base do accent (`hat(a^2)`) usa
`superscript_shift_up_cramped` (`attach.md` §P915).

## P918 — `accent_y` migrado para `stack_tight_above` partilhado

**Achado** (P918 Fase A, `accent.rs:65` vs `underover.rs:73`): `accent_y = -(base_box.ascent +
accent_box.descent)` é idêntica byte-a-byte à fórmula de `over_y` em `layout_underover`. Extraída
para `stack_tight_above(base_ascent, top_descent)` em `mod.rs` (ver `_comum.md` §P918) —
`accent_y = stack_tight_above(base_box.ascent, accent_box.descent)`. Comportamento inalterado.

## P920 — `accent_base_height`: achado confirmado, implementação DESTACADA para passo dedicado

**Achado** (`typst-passo-906-relatorio.md` achado original apontava genericamente para
`underover.rs`; `typst-passo-920-relatorio.md` Fase A confirmou, por leitura de
`typst-layout/src/math/accent.rs:56-65` do vanilla, que o mecanismo real é o de **acento**, não
de `underbar`/`overbar` — `hat`/`tilde` (este ficheiro) usam a MESMA fórmula que `overbrace`
(`underover.rs`, achado partilhado, ver `underover.md` §P920)): `gap = -accent.descent() -
base.ascent().min(accent_base_height)` — um **cap**, não uma constante aditiva.

**Correcção de uma nota anterior deste L0** (versão anterior desta secção, incorrecta): "bases
altas ganham espaço extra" — **errado**. O comentário do próprio vanilla (`accent.rs:57-60`) diz o
oposto: *"Only if the base is very small, we need a larger gap so that the accent doesn't move too
low"* — são as bases PEQUENAS que ganham mais espaço (para o acento não ficar demasiado baixo); o
cap em bases altas limita o gap, não o aumenta.

**Não implementado neste passo — destacado para passo dedicado.** Investigação mais funda (Fase B,
antes de qualquer código tocado) revelou que portar esta fórmula não é uma simples substituição:
`-accent.descent()` no vanilla pode ser **negativo** (comentário explícito do vanilla:
"Descent is negative because the accent's ink bottom is above the baseline" — glifo de acento cuja
tinta fica inteiramente acima da própria baseline). O contrato de `FontMetrics::text_ink_bounds`
no cristalino garante `ascent`/`descent` sempre `>= 0` (`engine/layout/metrics.rs:59-61`) — perde
exactamente essa informação. Uma aproximação (`accent.descent() ≈ 0`) produz sobreposição (gap
negativo), não uma aproximação inofensiva — o termo é estrutural na fórmula do vanilla, não
cosmético. Requer decisão arquitectural própria (estender `FontMetrics` para extensões com sinal,
ou mecanismo equivalente) antes de poder ser implementado fielmente — fora do âmbito deste passo.
Ver `typst-passo-920-relatorio.md` para o registo completo da investigação (incluindo a tentativa
de resolver por medição directa com o binário vanilla real, que confirmou a direcção do comentário
mas não resolveu como representar o termo com sinal no modelo do cristalino).

**`stack_tight_above` fica inalterada** — `accent_y` continua `stack_tight_above(base_box.ascent,
accent_box.descent)`, sem o cap, até o passo dedicado decidir a representação correcta.
