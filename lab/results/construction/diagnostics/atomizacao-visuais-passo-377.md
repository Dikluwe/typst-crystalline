# Passo 377 — correção da ADR de atomização + fatia elementos visuais

> **Estado: COMPLETO.** Estágio 0 (correção da ADR) + fatia visuais materializada (forma B).
> Caveat de stack: `RUST_MIN_STACK=33554432`. HEAD pós-P376. **Suíte verde** (2747+472+24+21+2, 0
> falhas; rede de caracterização `+11` sem asserção virada); **lint 0/0**; build limpo.

## Estágio 0 — correção da ADR de atomização (commit `a529a0ea2`)

**A — número (verificado, não assumido):** varredura de `00_nucleo/adr/` → maior em uso = **0108**;
**0109 é o primeiro livre** (o P376 inventou `0110` sem checar). Renumerada `0110 → 0109`: ficheiro
renomeado (`typst-adr-0109-atomizacao.md`), refs atualizadas em `claude.md`, no L0, no relatório
P376 e nos headers dos 5 `layout/*.rs`.

**B — forma canónica (Opção A → B):** a ADR fora gravada com a **Opção A** (`impl XElem { fn layout }`
no arquivo do struct). A medição do P376 provou que a A **cria o acoplamento `entities →
rules::layout`** (dado→render: ciclo + `pub(crate)` + genéricos — custo §3). Substituída pela **Opção
B** (free function `pub(super) fn layout<M,S>(layouter, e)` em `engine/layout/<elem>.rs`; acede ao
`Layouter` por descendência de módulo). A **Opção A fica registada como REJEITADA** (paralelo ao
registro do erro do P346). Definição e não-metas inalteradas (estavam certas); só a forma muda.

## Fatia — elementos visuais (Trava aprovada pelo dono; commit de fecho)

| Elemento | Arm antes (`@linha`) | Arquivo novo | Linhas |
|---|---|---|---|
| `Heading` | `:765` (44) | `engine/layout/heading.rs` | 62 |
| `Transform` | `:1013` (49) | `engine/layout/transform.rs` | 68 |
| `Shape` | `:980` (33) | `engine/layout/shape.rs` | 54 |
| `Columns` | `:1544` (44) | `engine/layout/columns.rs` | 58 |

Arm magro: `Content::Heading(h) => heading::layout(self, h)` (idem os 3). As free functions acedem ao
estado privado do `Layouter` por **descendência de módulo**; helpers livres (`heading_scale`/
`resolve_pt`/`measure_content`/`collect_sub_items`) e a const `COLUMNS_DEFAULT_GUTTER_RATIO` via
`super::`. `Heading` é o caso que lê estado diferente dos containers (`chain`/`introspector`/
`current_location`) — prova a forma B em quem consulta o Introspector.

**Métrica de leitura:** `layout_content` **1276 → 1126 linhas** (−150 nesta fatia; **−731 acumulado**
desde P376, de 1857). `mod.rs` 2293 → 2149. Imports mortos em `mod.rs` removidos
(`TransformMatrix`/`collect_sub_items`/`heading_scale`).

**Não-metas confirmadas (medido):** `match` exaustivo (0 wildcards); despacho estático (0 `dyn` de
elemento); `entities/` **não tocado** → `content→elements` inalterado, sem ciclo. **Content-
preserving:** rede `+11` (`f_caracterizacao_estilo::*`) sem asserção virada. **Linhagem:** 4
arquivos com `@prompt rules/atomizacao_elementos.md` + `@prompt-hash` sincronizado. **Perf:** free
function inlinável (suíte 0.37s estável). **INTACTOS:** α/caso 2, `morph_canon`/`==`, caso 4, flag
P350c, F-5b, numbering, `#set` de props.

## Nota de execução
- O ficheiro da ADR **desapareceu da working tree** repetidamente (mecanismo externo de
  sync/editor); cada vez restaurado de `HEAD`/recriado. Conteúdo sempre íntegro em git.

**Fora de escopo (Trava 5):** fatias futuras (decorações Underline/Strike/Overline; Place/Image/
Figure; o resto); os arms math (descem a `rules/math/layout/`); a varredura do projeto; a decisão de
crates — decisão do dono, passos separados.
