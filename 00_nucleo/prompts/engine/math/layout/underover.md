# Prompt L0 — `math/layout/underover` — `MathUnderover`
Hash do Código: 1953612a

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/underover.rs`
**Origem**: fatiado de `math/layout/mod.rs` em **P909**, completando o padrão de fatiamento
iniciado em P314 (ADR-0104) para `frac`/`root`/`stretchy`/`assembly`/`matrix`/`cases`/
`delimited`. `layout_underover` foi adicionado em **P297**, depois de P314, e nunca tinha sido
movido. Núcleo partilhado: ver `math/layout/_comum.md`.

---

`MathUnderover` — empilha `over` (topo), `base` (meio), `under` (fundo). Cada `Option` é skipped
se `None`. Width final = max das 3 partes; cada parte centrada horizontalmente. Agregação
cristalina per ADR-0054 graded — vanilla typst fragmenta em 12 elementos (`underbrace`/
`overbrace`/`underbracket`/etc., dispatch em `eval/math.rs`); cristalino unifica num único
variant `MathUnderover`.

## P906 — over/under de 1 carácter esticam para cobrir a largura da base

**Achado** (`typst-passo-899-relatorio.md`, Parte C, motivador original): `layout_underover`
centrava `over`/`under` no seu tamanho natural (`let dx = (w - ob.width) / 2.0`), sem esticar para
cobrir a largura da base — `underbrace(a+b+c)` produzia uma chave do tamanho de 1 carácter sobre
uma expressão larga.

**Correcção**: antes de `self.layout_node(c, style)` para `over`/`under`, um guard partilhado com
`layout_accent` (`layout_stretchy_or_node`, definido em `mod.rs` — ver `_comum.md` §guard
partilhado): se o conteúdo é `Content::MathText(s)` com exactamente 1 carácter, calcula
`min_width_du = base_box.width * self.constants.upem / style.size.val().max(0.001)` e chama
`layout_stretchy_glyph_horizontal` (`stretchy.rs`, ver `stretchy.md` §P906) em vez de
`layout_node`. Para qualquer outro conteúdo — incluindo a **anotação** de `underbrace`/`overbrace`
(texto normal, nunca deve esticar) — comportamento **inalterado**.

**Alcance confirmado com o dono**: chaves/colchetes (`⏟`/`⏞`/`⎵`/`⎴`), o caso que a fonte suporta
com dados reais.

**Critério de regressão**: `underover(a+b+c, over: Content::MathText("⏞"))` — a largura do
`MathBox` do `over` é maior que a de `underover(a, over: "⏞")` sozinho (esticou). Anotação
multi-carácter mantém-se centrada no tamanho natural (não estica). Testes:
`p906_layout_underover_over_1char_stretchy_domina_largura_apos_esticar`,
`p906_layout_underover_anotacao_multicaracter_nao_estica` (`tests.rs:2154`, `tests.rs:2181`).

## P906 (cont.) — offsets Y usavam convenção "topo do box"

**Achado, só visível ao confirmar visualmente `underbrace(a+b+c, "soma")`** (com anotação —
estrutura aninhada de 2 `MathUnderover`, `eval.md` §P906): a chave/anotação apareciam sobrepostas,
ilegíveis. `underbrace(a+b+c)` SEM anotação (1 nível, sem aninhamento) já renderizava
correctamente — isolou o problema à COMPOSIÇÃO (`MathUnderover` usado como `base` de outro
`MathUnderover`/`MathAccent`), não ao esticamento em si (secção acima).

**Causa**: mesmo padrão já corrigido em `frac.rs` (P905) e `root.rs` (P901), nunca antes auditado
em `layout_underover` — `over`/`base`/`under` eram posicionados por offsets crescentes a partir de
`Pt(0.0)` (convenção "topo do box"), não pela convenção baseline-relativa confirmada em P901
(`local_y=0` é a BASELINE PRÓPRIA da `MathBox`, y cresce para baixo). Funcionava por coincidência
quando a caixa era o conteúdo de topo da equação (`place()` no topo cancela o termo `-ascent`
independentemente da convenção interna) — quebrava assim que a caixa fosse usada como sub-caixa de
outra (aninhamento, ou `hconcat_spaced` com irmãs).

**Correcção**: `base` fica em `Pt(0.0)` (a sua própria baseline já é `local_y=0`); `over_y =
-(base_box.ascent + over_box.descent)` (a baseline do over sobe o suficiente para o seu descent
parar exactamente no topo da tinta da base); `under_y = base_box.descent + under_box.ascent`
(simétrico, para baixo). `ascent`/`descent` (valores escalares) não mudaram — só os offsets dos
items.

**Efeito colateral**: a ORDEM de push em `items` mudou (base agora primeiro, depois over/under)
— irrelevante para o render, mas quebrou 1 teste pré-existente que assumia posição por índice
(`items.first()`) em vez de por conteúdo — corrigido para procurar por conteúdo.

**Fora de âmbito, achado novo registado**: com a correcção, a estrutura aninhada renderiza
correctamente mas com um gap visualmente maior do que o vanilla entre `base` e a chave —
`layout_underover` não usa nenhuma constante de gap explícita (empilha directamente por
`height()`/ink-to-ink), ao contrário do vanilla que usa `underbar_vertical_gap`/
`overbar_vertical_gap` da tabela MATH. Candidato a passo dedicado.

**Critério**: `MathUnderover { base, under: Some(c1), over: Some(c2) }` → `over` acima e `under`
abaixo da base sem sobreposição, mesmo quando `MathUnderover` é usado como base de outro
`MathUnderover`/`MathAccent` (aninhamento).

## P918 — `over_y` migrado para `stack_tight_above` partilhado

**Achado** (P918 Fase A, `underover.rs:73` vs `accent.rs:65`): `over_y = -(base_box.ascent +
ob.descent)` é idêntica byte-a-byte à fórmula de `accent_y` em `layout_accent`. Extraída para
`stack_tight_above(base_ascent, top_descent)` em `mod.rs` (ver `_comum.md` §P918) — `over_y =
stack_tight_above(base_box.ascent, ob.descent)`. `under_y` (linha 82, espelho "abaixo") **não**
migra — sem segundo consumidor confirmado, fica inline (critério do próprio P918: não generalizar
sem duplicação real).

## P920/P922 — achado original de P906 corrigido (mecanismo real: acento, não underbar/overbar); gap real implementado via `layout_accent`

**Achado** (`typst-passo-906-relatorio.md` achado original: "`layout_underover` empilha por
`height()` sem nenhuma constante de gap da tabela MATH — o vanilla usa `underbar_vertical_gap`/
`overbar_vertical_gap` explícitos"; `typst-passo-920-relatorio.md` Fase A **corrige** a premissa:
`underbrace`/`overbrace`/`underbracket`/etc. resolvem no vanilla como `AccentItem`
(`typst-library/src/math/ir/resolve.rs:1277-1472`, `resolve_underoverspreader`) — o MESMO
mecanismo de `hat`/`tilde` (`accent.rs` §P920) — **não** como `LineItem`
(`underbar_vertical_gap`/`overbar_vertical_gap`, exclusivos de `underline()`/`overline()`, que o
cristalino nem implementa como `MathUnderover` — resolvem para decoração de texto,
`entities/layout_types.md` §P915).

Fórmula real (`typst-layout/src/math/accent.rs:56-71`, dois ramos, POSIÇÕES DIFERENTES):
- **Acima** (`overbrace`/`overbracket`/etc., `over_y` neste ficheiro): `gap = -accent.descent() -
  base.ascent().min(accent_base_height)` — cap. **Nota de correcção**: uma versão anterior desta
  secção dizia "produz gap extra só para bases altas" — errado; o comentário do próprio vanilla
  (`accent.rs:57-60`) diz o oposto ("only if the base is very small, we need a larger gap") — são
  as bases pequenas que ganham mais espaço, o cap em bases altas limita o gap.
  **P922**: a decisão arquitectural de introduzir `FontMetrics::text_ink_bounds_signed` e
  `MathConstants::accent_base_height` resolve a incompatibilidade de sinal destacada em P920.
  `layout_underover` pode reaproveitar a mesma fórmula de `layout_accent` para o `over_y`;
  neste passo o foco é `layout_accent`, deixando `underover` como seguimento natural a validar
  com medição do vanilla real.
- **Abaixo** (`underbrace`/`underbracket`/etc., `under_y` neste ficheiro): `gap = -accent.ascent()`
  — **sem** `accent_base_height`, equivale a empilhamento justo puro. `under_y` (linha 82, `base_
  box.descent + ub.ascent`) **já implementa exactamente isto** — **sem mudança nesta secção**,
  confirma que o achado original de P906 não se aplicava ao lado "abaixo". Esta parte do achado
  fica resolvida (não precisa do passo dedicado): under_y está correcto tal como está.

## P945 — nota: manter `math_size` honesto, sem mudar factores

Com a introdução de `TextStyle::math_size` (`entities/layout_types.md`
§P945), este módulo passa a **actualizar o campo** ao construir estilos de
descida (scripts/num/den/radicando/índice/limits), para o nível vanilla
correspondente (`style.rs:315-363` do vanilla), **sem alterar o factor de
tamanho actual** — o comportamento geométrico deste módulo fica inalterado
neste passo. Motivo: consumidores abaixo (ex.: uma matriz dentro de um
subscrito — `matrix.md` §P945) dependem do nível correcto para a sua própria
descida. A correcção dos factores deste módulo (ex.: fracção display a ×1.0,
P944 relatório §8.3 item 6) fica para passo dedicado — scope-out registado.
