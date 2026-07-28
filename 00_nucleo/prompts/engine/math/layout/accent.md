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

## P922 — gap real de acento com `accent_base_height` e `text_ink_bounds_signed`

**Contexto** (`typst-passo-920-relatorio.md` Fase A, `typst-passo-922.md`): a fórmula real do
vanilla (`typst-layout/src/math/accent.rs:56-65`) é `gap = -accent.descent() -
base.ascent().min(accent_base_height)` — um **cap**, não uma constante aditiva. O comentário do
vanilla (`accent.rs:57-60`) deixa claro que só bases muito pequenas precisam de gap maior; o cap
limita o gap para bases altas.

**Decisão arquitectural P922**: em vez de estender o contrato geral de `text_ink_bounds` (que
afectaria toda a geometria de `MathBox::ascent`/`descent`), introduz-se `FontMetrics::
text_ink_bounds_signed(text, size, style) -> (Pt, Pt)` — limites de tinta com sinal. Para um
combining mark acima da baseline (ex. `hat`, `tilde`), `bottom` é **negativo**, reproduzindo o
`accent.descent()` negativo do vanilla. Ver `engine/layout.md` §P922 e `infra/font_metrics.md`
§P922.

**Implementação em `layout_accent`**:
1. Medir o acento base (char original) antes de esticar, via `text_ink_bounds_signed`, para obter
   `(_, accent_bottom_signed)`.
2. Converter `accent_base_height` para pontos: `accent_base_height_pt = self.constants.to_pt(
   self.constants.accent_base_height, style.size).val()`.
3. Calcular o gap real: `gap = -accent_bottom_signed - base_box.ascent.min(accent_base_height_pt)`.
4. O offset vertical do acento mantém a convenção baseline-relativa do cristalino:
   `accent_y = -base_box.ascent + base_box.ascent.min(accent_base_height_pt)`.
   (O `accent_bottom_signed` cancela-se no posicionamento final, mas entra no `gap` e portanto
   no `new_ascent` da `MathBox` resultante.)
5. O `new_ascent` da `MathBox` passa a ser `base_box.ascent + accent_box.height() + gap` (em vez
   de `base_box.ascent + accent_box.height()` como em P906/P918).

**Inferência marcada (refutável)**: para acentos esticados (`hat(a+b)`), o gap usa o `bottom`
com sinal do **caractere base original**, não do resultado esticado. O que a refutaria: medição
com o vanilla real a mostrar que `hat(a+b)` tem gap visivelmente diferente do previsto por esta
aproximação. Nesse caso, estender-se-ia `layout_stretchy_glyph_horizontal`/
`layout_assembly_horizontal` para reportar também o descent com sinal do resultado esticado.

**Critérios de verificação**: `hat(a)`/`tilde(a)` sobre base pequena têm gap maior do que sobre
base alta (cap); `hat(a+b)` estica horizontalmente sem sobreposição; comparação `mutool trace`
com o vanilla real para bases pequena e grande.
