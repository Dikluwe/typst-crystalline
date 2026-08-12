# Prompt L0 — `math/layout/accent` — `MathAccent`
Hash do Código: 1547e609

**Camada**: L1 · **Alvo**: `01_core/src/compiler/math/layout/accent.rs`
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
`accent.descent()` negativo do vanilla. Ver `compiler/layout.md` §P922 e `infra/font_metrics.md`
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

## P984 — acento estica com `ACCENT_SHORT_FALL = 0.5em` (não 0.1em)

**Medição** (achado §7.1 da auditoria 2026-08-06; mecanismo completo e dados da
fonte em `stretchy.md` §P984): o acento sobre base estreita (`hat(x)`) ficava
~30% mais largo que o vanilla (7.08pt vs 5.50pt) e sobre base muito larga
(`hat(a+b)`) caía no glifo base em vez da maior variante (5.50pt vs 20.86pt).

**Correcção neste ficheiro**: `layout_accent` passa `short_fall_em = 0.5`
(`ACCENT_SHORT_FALL`, vanilla `math/accent.rs:18` + `ir/resolve.rs:389`) para
`layout_stretchy_or_node`/`layout_stretchy_glyph_horizontal` — o mesmo valor
para todos os acentos de 1 carácter. As regras keep-base (base estreita mantém
o glifo base, comparando com o advance hmtx e não com o `AdvanceMeasurement`
da tabela MATH) e keep-largest (alvo acima de todas as variantes sem assembly
→ maior variante) vivem em `stretchy.rs` (ver `stretchy.md` §P984). Spreaders
(`underover.rs`) passam 0.0 (`ir/resolve.rs:1430`).

**Critério**: `hat(x)` com a fonte de produção devolve o glifo base (~5.50pt a
11pt); `hat(a+b)` devolve a maior variante (~20.86pt); larguras intermédias
escolhem a primeira variante ≥ `largura_base − 0.5em`.

## P988-B — centragem horizontal do acento usa `TopAccentAttachment`

**GATE ADR-0127 APROVADO pelo dono (2026-08-06)** — novo método no trait
`FontMetrics` autorizado (a tabela é indispensável: o fallback `(w+IC)/2`
não reproduz o valor — x itálico tem IC ausente mas TopAccent=287du).

**Medição** (achado §8.4 da auditoria 2026-08-06): o vanilla desloca o acento
~0.5pt para a direita do centro da caixa do "x" (compensação de itálico); o
cristalino centra exactamente (`dx = (base_w − accent_w)/2`, `accent.rs:66`).

**Leitura do vanilla** (`typst-layout/src/math/accent.rs:37-52`): para acento
de topo não-`exact_frame_width` (o caso `hat`/`tilde`/`dot`): `base_x = 0`,
`accent_x = base_attach − accent_attach`, onde
`base_attach = base.accent_attach().0` e `accent_attach =
accent.accent_attach().0`. Cada `accent_attach().0` vem da tabela MATH
`TopAccentAttachment` do glifo, com fallback `(width + italics_correction)/2`
(`fragment/glyph.rs:222-224`). Valores reais (NewCMMath-Book, fontTools):
x itálico = **287du** (hmtx 529, IC ausente), hat = **250du**, tilde =
**266du** → `accent_x = 287 − 250 = 37du ≈ 0.41pt` à direita do centro
(cristalino: 14.5du = meio-centro). O fallback `(w+IC)/2` NÃO reproduz o
valor (x tem IC ausente → centro exacto) — a tabela é indispensável.

**Correcção proposta**: novo método `FontMetrics::top_accent_attach(
glyph_id: u16, size: Pt, style: &TextStyle) -> Option<Pt>` — default `None`
(sem tabela → fórmula fallback actual). L3 lê `MathTopAccentAttachment` da
face (mesmo padrão de `italics_correction`, P971). `layout_accent` passa a
`accent_x = base_attach − accent_attach`: para a base, attach do glifo da
base (via `text`/glyph id — a investigar na implementação: a base é
`Content`, o attach vem do seu fragmento; para bases multi-carácter o
vanilla usa `accent_attach` do fragmento composto = metade da largura para
não-glifos, `fragment/mod.rs:149`); para o acento, attach do glifo do acento
(antes de esticar — o `accent_attach` do vanilla é medido no glifo base do
acento, `accent.rs:34-35` antes de `layout_into_fragment`? — confirmar na
implementação se é antes ou depois do stretch; `update_glyph` corre depois
do stretch, logo o attach é o da variante escolhida).

**Critério**: `hat(x)`/`tilde(x)` no doc canónico com o acento deslocado
~0.4pt à direita do centro da caixa, como o vanilla; teste unitário com stub
de `top_accent_attach` (base 287du, acento 250du → dx = 37du×scale).

## P989 — acento aninhado: `new_ascent` por `max(base.ascent, −accent_y + accent.ascent)`

**Medição** (achado §8.2 da auditoria 2026-08-06; repro `temp/p987/dots.typ`,
bandas a 1200dpi): `dot(dot(x))` — vanilla: dois pontos distintos, centros a
**2.52pt** um do outro (dot1 bottom 1.5pt acima do topo do x; dot2 1.44pt
acima de dot1); cristalino: **os dois pontos na mesma posição exacta**
(só um visível) e a equação 2.52pt mais curta. `dot(x)` simples estava
correcto (idêntico ao vanilla) — o bug só aparece quando a caixa de acento
vira BASE de outro acento.

**Causa (instrumentação P989DBG + leitura)** — dois bugs compostos:

1. **L3 com sinal invertido** (`03_infra/src/font_metrics.rs`,
   `text_ink_bounds_signed`, ambas as implementações): devolvia
   `bottom = +y_min·s` (y_min cru) em vez de `−y_min·s` ("positivo para
   baixo", a convenção documentada no trait e usada pelo vanilla
   `accent.descent()`). Para tinta que flutua acima da baseline (uni0307:
   bbox y 571..677du) devolvia +6.28pt em vez de −6.28pt — invertendo o
   `gap` de P922 (obtido −11.14pt; correcto +1.42pt). Invisível até aqui
   porque o P922 só afectava `new_ascent` (a posição do acento cancela o
   termo) e nenhuma caixa de acento tinha virado sub-caixa.
2. **`accent_h` sem sinal** (P922): `accent_h = accent_box.height()` usa a
   caixa do caminho de texto (`descent` clampado a 0 para tinta flutuante),
   não a altura de tinta com sinal do fragmento vanilla
   (`height = y_max − y_min`). Os dois erros cancelavam-se parcialmente
   (new_ascent 1.17pt vs correcto 7.45pt para dot(x) a 11pt).

**Derivação (a forma correcta fecha com o vanilla algebricamente)**: o
`new_ascent` correcto é a posição do topo da tinta acima da baseline:
`max(base.ascent, −accent_y + accent_box.ascent)`. Expandindo
`accent_y = −base.ascent + min(base.ascent, abh)` mostra-se que é
algebricamente IDÊNTICO à fórmula aditiva do vanilla
(`base.ascent + accent.height + gap`) **quando `accent.height` é a altura
de tinta com sinal** — os termos com sinal cancelam. A forma `max` obtém o
mesmo valor SEM precisar de `signed_descent` — que passa a não ter
consumidor em produção. Verificação com números reais (11pt, abh=450du):
dot(x): max(4.86, 0+7.45) = 7.45 = vanilla ✓; dot(dot(x)): accent_y externo
= −7.45+4.95 = −2.50 → pontos a 2.50pt ≈ medido 2.52 ✓✓; base alta
(ascent 10 > abh): max(10, 3.52+7.45=10.97) = 10.97 = baseline-do-topo do
frame vanilla ✓.

**Correcção**:
1. `accent.rs`: `new_ascent = base_box.ascent.max(-accent_y + accent_box.ascent)`
   (accent_y calculado antes); o helper `accent_signed_descent` e as
   variáveis `gap`/`accent_h` saem (mortos). **A fórmula aditiva de P922
   fica substituída** — os testes P922 com valores assados são actualizados
   (p922-1: ambos os casos dão 8.4 — acento engolido pela base alta; o caso
   "gap positivo" mantém 14.32 com valores auto-consistentes top=12.4).
2. L3 (`infra/font_metrics.md` §P989): corrigir o sinal de `bottom` em
   `text_ink_bounds_signed` (`−y_min·s`), ambas as implementações — a
   convenção do trait sempre foi esta; fica sem consumidor de produção mas
   o contrato documentado passa a ser respeitado.

**Critério**: `dot(dot(x))` com os dois pontos em y distintos (distância pela
fórmula ≈ 2.5pt a 11pt); `new_ascent` de acento simples = topo da tinta do
acento (7.45pt para dot(x)); suite verde com os valores P922 actualizados.
