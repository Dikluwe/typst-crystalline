# Passo 378 — atomização fatia visuais/decorações (rumo a fechar o monólito)

> **Estado: COMPLETO.** Fatia Place + Image + Figure + Decorações materializada (forma B).
> Caveat de stack: `RUST_MIN_STACK=33554432`. HEAD pós-P377. **Suíte verde** (2747+472+24+21+2, 0
> falhas; rede `+11` sem asserção virada); **lint 0/0**; build limpo.

## Mitigação do mecanismo externo
O **L0 foi commitado ANTES de mover código** (Estágio L0, commit `284cff19a`): regista a fatia (§8)
+ o inventário do que falta. A ADR e o L0 nunca ficaram não-commitados entre estágios.

## Fatia movida (forma B)

| Elemento | Arm antes (`@linha`) | Destino | Tipo |
|---|---|---|---|
| `Place` | `:1166` (58) | `rules/layout/place.rs` | **novo** |
| `Underline`/`Strike`/`Overline` | `:1502` (75, agrupado) | `rules/layout/decorations.rs` | **novo** (1.º arm agrupado) |
| `Image` | `:1115` (~36) | `rules/layout/image.rs::layout` | **completa** (junto do helper `calculate_dimensions`) |
| `Figure` | `:909` (~31) | `rules/layout/figure.rs::layout` | **completa** (dobra o prefixo + `layout_figure`) |

Arm magro: `Content::Place(e) => place::layout(self, e)`; decorações:
`Content::Underline(_) | Content::Strike(_) | Content::Overline(_) => decorations::layout(self, content)`
(free function recebe `&Content` e re-match interno — 1.º arm agrupado atomizado). Estado privado do
`Layouter` (`floats_pending`/`decoration_lines_collector`/`figure_progress`/`sizer`/…), tipos
`DeferredFloat`/`DecoSegment` e métodos (`layout_place`/`layout_sub_frame_with_width`) acedidos por
**descendência de módulo**; helpers via `super::`. Import morto `use std::sync::Arc` removido de
`mod.rs`. `image.rs`/`figure.rs` mantêm a sua própria linhagem (`layout-image.md`/`layout_figure.md`).

## Métrica de leitura (progresso rumo a fechar)
- `layout_content` **1126 → 929 linhas** (−197 nesta fatia). **Acumulado desde 1857: −928** (≈50%).
- `mod.rs` total 1955.
- **Unidades atomizadas até aqui (P376+P377+P378): 15** (Block/Boxed/Stack/Pad, Heading/Transform/
  Shape/Columns, Place/Image/Figure/Decorações×3).

## Inventário — o que falta para fechar (medido)
- **Não-math restantes** (lotes futuros, forma B): Quote 46, Cite 35, Colbreak 33, TermItem 25,
  SmartQuote 25, Repeat 25, Pagebreak 25, VSpace 23, Divider 19, Bibliography 19, Table/TableCell/
  TableFooter, EnumItem/ListItem/Terms, Raw, Hide, Footnote, Ref, Link, família Grid, HSpace. Plus o
  **core/infra** a decidir (Text 82, Sequence 51, Styled 14, Dynamic 31, SetPage 25, state/counter).
- **Math** (fatia FINAL própria): `Equation` + arm agrupado de 16 variantes (`:865`) = **17** →
  descem a `rules/math/layout/`.

## Não-metas confirmadas (medido)
`match` exaustivo (0 wildcards) · despacho estático (0 `dyn` de elemento) · `entities/` **não
tocado** → `content→elements` inalterado, sem ciclo · content-preserving (rede `+11`
`f_caracterizacao_estilo::*` sem asserção virada) · `@prompt-hash` sincronizado · INTACTOS α/caso 2,
`morph_canon`/`==`, caso 4, flag P350c, F-5b, numbering, `#set`.

## Commits
| Estágio | Commit |
|---|---|
| L0 (pré-código) | `284cff19a` |
| fatia (fecho) | (este) |

**Fora de escopo (Trava 5):** lotes não-math seguintes (Quote/Cite/breaks/tables/lists/…); a fatia
**math** final; a atomização do `introspect.rs`; varredura/crates — decisão do dono, passos
separados.
