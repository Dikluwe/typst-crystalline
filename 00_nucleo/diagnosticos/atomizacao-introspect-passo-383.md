# Passo 383 — atomização do introspect.rs: "elementos primeiro" completo

> **Estado: COMPLETO. A atomização "elementos primeiro" fecha nas DUAS camadas.** Caveat de stack:
> `RUST_MIN_STACK=33554432`. HEAD pós-P382. **Suíte verde** (2747+472+24+21+2, 0 falhas; rede `+11`
> + testes de introspeção/fixpoint sem asserção virada); **lint 0/0**; build limpo.

## Fase A — a convenção do walk (medida, não herdada do layout/math)
- `rules/introspect.rs` é o **tronco do walk**; o subsistema `rules/introspect/` já tem submódulos
  por-concern (`extract_payload.rs`, `from_tags.rs`, `fixpoint.rs`, `convergence.rs`, `locatable.rs`),
  cada um declarado `pub mod` no tronco, com `pub(super) fn`.
- **A lógica de introspeção por-elemento já foi extraída nas migrações M5/M6** (P178-P200):
  `extract_payload` (payload por elemento), `from_tags::apply_state_displays`/`apply_counter_displays`
  (os **displays counter/state** — o **[a-decidir] do P379 resolve-se aqui**: já estão atomizados em
  `from_tags.rs`), e os helpers `compute_*` (compute por-elemento, ainda soltos no tronco).

### Classificação dos arms do walk (medido)
- **[máquina]** ~31 arms de recursão/descida (`walk(&e.body…)`), + `Heading`/`Labelled`
  (orquestração de emissão de tags Start/End/HeadingForToc), + `Sequence`/`Styled`/`materialize_time`
  (rebuild recursivo). **O walk é máquina.**
- **[no-op / puro]** `Equation`/`Figure` (só descem — lógica já em `extract_payload`/`populate_intr`),
  `CounterUpdate` (no-op), `Empty`/`Text`/`Dynamic`/`Outline`.
- **[adaptador]** os displays counter/state — **já em `from_tags.rs`**.

**Conclusão:** o introspect estava **quase todo já atomizado**; o único resíduo por-elemento solto no
tronco eram os helpers `compute_*`.

## Movimento (completar a convenção por-elemento — escolha do dono)
Movidos do tronco para o submódulo (convenção `pub mod` + `pub(super) fn`):
- `compute_heading_auto_toc` + `compute_heading_for_toc` → **`rules/introspect/heading.rs`**.
- `compute_labelled` → **`rules/introspect/labelled.rs`**.

O walk chama via `heading::…`/`labelled::…` (pub(super), descendência de módulo). Os ficheiros novos
declaram `@prompt rules/atomizacao_elementos.md`. **Sem** `dyn`, **sem** wildcard no walk, `entities/`
intacto. A **máquina do walk fica** (recursão/tags/rebuild) — não é elemento.

## Métrica de leitura
- `introspect.rs` (tronco) **3446 → 3307 linhas** (−139). `heading.rs` 54 + `labelled.rs` 63.
- A lógica de introspeção de Heading/Labelled agora vive no arquivo do elemento (convenção do
  submódulo, junto de `extract_payload.rs`/`from_tags.rs`).

## "Elementos primeiro" COMPLETO nas duas camadas
- **Layout** (P376-382): 44 elementos de domínio atomizados; `layout_content` = só máquina + math.
- **Introspect** (M5/M6 + P383): lógica por-elemento extraída (`extract_payload`/`from_tags`/
  `compute_*`); o walk = máquina.
**Os elementos de domínio estão atomizados nas duas camadas.**

## Não-metas confirmadas (medido)
`match` do walk exaustivo (0 wildcards — o `_ =>` em `compute_labelled` é o match interno de target,
não o walk) · despacho estático (0 `dyn`) · `entities/` **não tocado** → `content→elements`
inalterado · content-preserving (rede `+11` + testes de introspeção/fixpoint) · introspect-chain
(P363) inalterado · INTACTOS α/caso 2 (o fixpoint morfológico — não tocado), `morph_canon`/`==`,
caso 4, flag P350c, F-5b, numbering, `#set`.

## Commits
| Estágio | Commit |
|---|---|
| L0 (pré-código) | `30610c9bc` |
| introspect (fecho) | (este) |

## A frente seguinte
Com "elementos primeiro" completo, a frente seguinte é a **varredura do projeto inteiro** (todo
monólito, não só os de elemento) — **decisão do dono**. A **máquina** (do walk e do layouter) fica.
**Fora de escopo (Trava 5).**
