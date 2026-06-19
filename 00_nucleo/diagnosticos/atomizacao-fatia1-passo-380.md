# Passo 380 — atomização Fatia 1/2: fluxo de bloco e estrutura (forma B, por-elemento)

> **Estado: COMPLETO.** Fatia 1 (listas/grid+tabelas/breaks/spacing) materializada.
> Caveat de stack: `RUST_MIN_STACK=33554432`. HEAD pós-P379. **Suíte verde** (2747+472+24+21+2, 0
> falhas; rede `+11` sem asserção virada); **lint 0/0**; build limpo.

## Mitigação do mecanismo externo
O **L0 foi commitado ANTES de mover código** (Estágio L0, commit `39853860e`): §9 (Fatia 1 +
granularidade por-elemento) + inventário. Durante o passo a ADR-0109 voltou a sumir/reaparecer da
working tree (mecanismo externo); confirmada presente, tracked e limpa.

## Fatia movida (forma B, granularidade por-elemento — escolha do dono)

| Família | Arms → arquivo | Nota |
|---|---|---|
| **Listas** | `list_item.rs`, `enum_item.rs`, `terms.rs`, `term_item.rs` | TermItem push de `Bold` na chain |
| **Grid/Table** (cluster) | `Grid` → **grid.rs** (junto de `layout_grid`); `Table` → `table.rs`; `grid_header/footer/cell`, `table_cell/header/footer` → arquivos próprios | Grid/Table chamam `layout_grid` (pub(super), por descendência); cells/headers/footers são `layout_content(&e.body)` |
| **Breaks** | `pagebreak.rs`, `colbreak.rs` | `new_page`/`flush_line` |
| **Spacing** | `h_space.rs`, `v_space.rs`, `repeat.rs` | |

**18 arms → 16 arquivos novos + grid.rs (fold).** Arm magro: `Content::EnumItem(e) =>
enum_item::layout(self, e)`. Estado privado do `Layouter` + método `layout_grid` por **descendência
de módulo**; helpers via `super::`. Import morto `use ecow::EcoString` removido de `mod.rs`.

## Métrica de leitura (progresso rumo a fechar)
- `layout_content` **929 → 790 linhas** (−139 nesta fatia). **Acumulado desde 1857: −1067 (~57%).**
- `mod.rs` total 1834.
- **Unidades de domínio atomizadas (P376→P380): 33** (15 + 18 desta fatia).

## Fora desta fatia (não tocados — confirmado)
- **Text** (`:635`) — fatia própria.
- **Máquina do layouter** — Sequence/Styled/Dynamic/SetPage (intactos).
- **Math** — Equation + arm agrupado 16 variantes → fatia final.
- **Displays counter/state** ([a-decidir]).

## O que resta (medido)
**Fatia 2** (refs/citações + avulsos: Quote, SmartQuote, Raw, Hide, Divider, Cite, Ref, Link,
Bibliography, Footnote) + **Text** + (máquina fica) + **math** final.

## Não-metas confirmadas (medido)
`match` exaustivo (0 wildcards) · despacho estático (0 `dyn`) · `entities/` **não tocado** →
`content→elements` inalterado, sem ciclo · content-preserving (rede `+11` sem asserção virada) ·
`@prompt-hash` sincronizado · INTACTOS α/caso 2, `morph_canon`/`==`, caso 4, flag P350c, F-5b,
numbering, `#set`.

## Commits
| Estágio | Commit |
|---|---|
| L0 (pré-código) | `39853860e` |
| Fatia 1 (fecho) | (este) |

**Fora de escopo (Trava 5):** Fatia 2; Text; máquina (fica); math final; introspect.rs; varredura/
crates — decisão do dono, passos separados.
