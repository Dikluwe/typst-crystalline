# Passo 381 — atomização Fatia 2 (refs/avulsos) + Fatia Text (isolada)

> **Estado: COMPLETO.** Duas fatias, três commits (L0 + Fatia 2 + Text).
> Caveat de stack: `RUST_MIN_STACK=33554432`. HEAD pós-P380. **Suíte verde** (2747+472+24+21+2, 0
> falhas; rede `+11` sem asserção virada, incl. o oráculo crítico `f_caracterizacao_estilo::*` do
> Text); **lint 0/0**; build limpo.

## Mitigação do mecanismo externo
O **L0 foi commitado ANTES de mover código** (Estágio L0, `abcb07969`). A ADR-0109 voltou a
sumir/reaparecer da working tree (mecanismo externo) — confirmada presente a cada estágio.

## Fatia 2 — refs/citações + avulsos (commit `4625c7e6e`)
9 arms inline → `rules/layout/<elem>.rs` (forma B, por-elemento):

| Arm | Arquivo | Estado que lê |
|---|---|---|
| Cite | `cite.rs` | `introspector` (bib_entry/number) |
| Bibliography | `bibliography.rs` | `super::format_bib_entry` |
| Footnote | `footnote.rs` | **dedicado** `footnote_counter`/`pending_footnote_bodies` |
| Link | `link.rs` | `layout_content` |
| Quote | `quote.rs` | `chain.lang`, fluxo-texto |
| SmartQuote | `smartquote.rs` | **dedicado** `smartquote_*_open` + `layout_word` |
| Raw | `raw.rs` | `layout_word`, `style` |
| Hide | `hide.rs` | `regions`, `layout_content` |
| Divider | `divider.rs` | `regions`, `page_config`, `font_size_pt` |

**`Ref`/`Labelled` já delegavam a `references.rs`** — não tocados. Estado dedicado de Footnote/
SmartQuote acedido por **descendência de módulo** (sem extracção, como previsto).

## Fatia Text — folha de render, isolada (commit próprio)
`Text` (`@664`, ~82 linhas) → `rules/layout/text.rs`. Confirmado **folha** (`layout_word`, **não
re-entra `layout_content`**). Decodifica o `#set text`/`#set par` da chain (`custom`), merge top-wins
com `layouter.style`, dispõe palavras. Caminho quente — a rede `f_caracterizacao_estilo::*` (11
testes) passou sem asserção virada.

## Métrica de leitura (o monólito "fechado" no sentido correto)
- Fatia 2: `layout_content` 790 → 643 (−147). Text: 643 → **564** (−79). **Total P381: −226.**
- **Acumulado desde 1857: −1293 (~70%).** `layout_content` agora **564 linhas**.
- **Unidades de domínio atomizadas (P376→P381): 43.**
- **O `layout_content` agora é só máquina + math** (+ no-ops + arms já-magros): `Sequence`/`Styled`/
  `Dynamic`/`SetPage` (máquina), `Equation` + 16 variantes math, displays counter/state ([a-decidir]).
  **Os elementos de domínio do layout estão atomizados.**

## Não-metas confirmadas (medido)
`match` exaustivo (0 wildcards) · despacho estático (0 `dyn` de elemento) · `entities/` **não
tocado** → `content→elements` inalterado, sem ciclo · content-preserving (rede `+11` + style net) ·
`@prompt-hash` sincronizado · INTACTOS α/caso 2, `morph_canon`/`==`, caso 4, flag P350c, F-5b,
numbering, `#set`. **Máquina (Sequence/Styled/Dynamic/SetPage) e math NÃO tocadas.**

## Nota cosmética
Após mover os arms que usavam `Point`, o import `Point` em `mod.rs` ficou usado **só** por
`tests.rs` (via `use super::*`) → 1 warning "unused import" no build **não-test** (test-only
re-export; padrão conhecido). Não é violação (lint 0/0); mantido para não mexer no ficheiro de
testes (9k linhas) com vários `use Point` locais.

## Commits
| Estágio | Commit |
|---|---|
| L0 (pré-código) | `abcb07969` |
| Fatia 2 | `4625c7e6e` |
| Text (folha) | (este) |

**Resta:** a **fatia math** (final, `Equation` + 16 variantes → `rules/math/layout/`) e depois a
atomização do **`introspect.rs`** (43 arms). A máquina do layouter **fica** (não é elemento).
**Fora de escopo (Trava 5)** — decisão do dono, passos separados.
