# Progresso F-2 (P335) — retomável

Lote F-2: canal único das `Set*` na chain léxica (fecha DEBT 99.E). Decisão do
dono: **tudo (as 4 Set*), faseado** (migração comportamental em estágios
validados, commit por estágio).

## Estado

| Estágio | Estado | Commit |
|---------|--------|--------|
| Caronas C1–C3 | ✅ | `249497558` |
| Fundação: canal aberto + B3 | ✅ verde 2719 | `8a6aadeef` |
| **S1 — heading numbering → chain** | ⬜ próximo | — |
| S2 — equation numbering → chain + B1 | ⬜ | — |
| S3 — figure numbering → chain | ⬜ | — |
| S4 — SetPage → chain (page_config, 34 leitores; remover native_page legacy) | ⬜ | — |
| S5 — trava (teste-varre-tabela do canal) + remover código morto dos canais antigos | ⬜ | — |

## Mecanismo (estabelecido na Fase A→B)

- **Scoping léxico já existe**: `engine.styles` é escopado via `local_styles`
  (`eval/markup.rs:37`; `eval/mod.rs:83-84` nota que o save/restore antigo foi
  substituído por isto). `#set text/par` já são lexicais. As 4 `Set*` são as
  outliers que devolvem marcadores flat.
- **Padrão de migração** (precedente: figure baking via `engine.figure_numbering`):
  o `#set X` empurra para `engine.styles.push_custom("X.prop", Value)`; o elemento
  **assa** o valor na criação (lendo `engine.styles.custom("X.prop")`); o consumer
  de layout lê o campo assado; o contador continua via Introspector.
- **Canal** (já em `8a6aadeef`): `StyleDelta.custom: Vec<(EcoString, Value)>` +
  `StyleChain::push_custom`/`custom` (fallback léxico top-wins).

## S1 — heading (o template) — passos

1. `HeadingElem` (`entities/elements/heading.rs`) ganha `numbering_active: bool`
   (paridade: hoje só "ativo", padrão fixo — o pattern é descartado em
   `eval/rules.rs:216-227`; manter active-only para content-preserving).
   Construtor `new` mantém compat (default false) + `new_numbered`.
2. `eval/rules.rs` `#set heading(numbering:)`: em vez de
   `return Content::SetHeadingNumbering{active}`, fazer
   `*engine.styles = engine.styles.push_custom("heading.numbering", Value::Bool(active)); Ok(Value::None)`.
3. `eval/markup.rs:89` criação do heading (`= title`): assar
   `numbering_active = engine.styles.custom("heading.numbering")` na `HeadingElem`.
4. Consumer `layout/mod.rs:695-714`: ler `e.numbering_active` (do `HeadingElem`)
   em vez de `is_numbering_active_at("numbering_active:heading", loc)`. Contador
   (`formatted_counter_at("heading", loc)`) **fica** via Introspector.
5. Retirar `Content::SetHeadingNumbering`: variante (`content.rs`) + arms
   (`extract_payload`, `from_tags`/`introspect.rs` walk + populate, `locatable`,
   o arm de layout). **Atenção**: `extract_payload`/`from_tags` partilham o arm
   genérico `StateUpdate` com equation — não remover esse arm até S2 (equation
   ainda usa a chave `numbering_active:equation`). Remover só a produção da chave
   `numbering_active:heading`.
6. Migrar os **~29 setups de teste** em `layout/tests.rs` que constroem
   `Content::SetHeadingNumbering{active:true}` → construir o heading já com
   `numbering_active:true` (helper `Content::heading_numbered` ou campo). As
   **asserções** (heading numerado "1. ") **não mudam** — só o setup adapta-se à
   API nova (a variante saiu).
7. Build + suíte verde + lint 0; commit "Passo 335 — lote F-2 S1 (heading)".

## S2–S4 — espelham S1 (equation/figure) + S4 page (page_config←chain, o difícil)

- **S2 equation**: + **B1** (produtor eval em falta — adicionar arm
  `target == "math.equation"`/`"math"` em `eval/rules.rs` após heading; conferir
  parsing de target pontuado). Vanilla: `lab/.../math/equation.rs:55-67`.
- **S3 figure**: já assa via `engine.figure_numbering` (campo do Engine) — mover
  para `engine.styles.custom("figure.numbering")` para uniformizar; retirar
  `SetFigureNumbering`.
- **S4 page**: `SetPage` muta `page_config` direto (`layout/mod.rs:1113`),
  **34 leitores**. Rotear via chain (resolver `chain.custom("page.*")` nos
  leitores OU manter `page_config` mas alimentado pela chain). Remover o produtor
  **legacy `native_page`** (`stdlib/layout.rs:294-319`, D4). Caso difícil — fatiar
  se preciso.

## Referências
- L0 `entities/f_fronteira_e1.md` §3b. ADR-0106. `f-plano-lotes-passo-333.md`.
- Recon P335 (3 agentes): os 4 fluxos com `file:line` (ver histórico).
