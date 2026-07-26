# Prompt L0 — `math/layout/accent` — `MathAccent`
Hash do Código: a699b600

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
