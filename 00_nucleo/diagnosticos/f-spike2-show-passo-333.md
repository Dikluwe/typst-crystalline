# f-spike2-show — `#show` real exercitado contra a fronteira E1 (Passo 333, Parte 2)

**Spike disposable.** Vive só em `lab/spikes/f-extensao/e1/`. Zero código de produto,
zero imports de produto ou quarentena. As citações `lab/typst-original/...` são leitura
autorizada (medição); nada foi importado — os snippets necessários foram **reescritos**
no spike.

Objetivo: o spike E1 do P332 **stubava** `#show` como um `fn(&Content)->Content` fixo,
1 regra, 1 passe. Aqui implementámos um harness `#show` **real e mínimo** que reproduz a
máquina vanilla e exercita 5 casos, para descobrir o que a semântica vanilla de `#show`
**exige** da fronteira E1 (`Content::Dynamic(Arc<dyn Element>)`). Essas exigências
alimentam o L0 do F (Parte 3).

---

## 1. As 5 comportamentos vanilla (com `file:line`)

Ficheiros-chave:
`lab/typst-original/crates/typst-library/src/foundations/styles.rs`,
`lab/typst-original/crates/typst-realize/src/lib.rs`,
`lab/typst-original/crates/typst-library/src/foundations/content/mod.rs`,
`lab/typst-original/crates/typst-library/src/foundations/selector.rs`.

### B1 — Ordem multi-regra: innermost-first, **uma** regra por passe
- As recipes vivem na `StyleChain` (`Style::Recipe` — styles.rs:219). O iterador
  `StyleChain::recipes()` filtra as recipes do iterador de entries
  (styles.rs:696-698).
- `Entries::next` consome o link da cabeça **em reverso** (`inner.next_back()`,
  styles.rs:835) e só depois desce para o link seguinte (mais externo)
  (styles.rs:839-841). Resultado: **a recipe mais interna / mais recentemente
  empurrada é vista PRIMEIRO**.
- Em `verdict`, o loop `for (r, recipe) in styles.recipes().enumerate()`
  (lib.rs:449) pega a **primeira** recipe que casa, não-guardada e não-show-set, e
  fixa-a como o passo (`step = Some(ShowStep::Recipe(...))`, lib.rs:478). A partir
  daí, qualquer recipe func extra é ignorada (`if step.is_some() { continue }`,
  lib.rs:467-469). → **No máximo 1 transformação func por passe.** A próxima regra
  aplica-se no **próximo** passe (a regra interna fica guardada, ver B2).

### B2 — Recursão/revogação: guard por-nó faz terminar
- Cada nó de conteúdo carrega um bitset `meta().lifecycle` de índices de recipe já
  aplicados. `is_guarded(index)` consulta-o, `guarded(index)` insere
  (content/mod.rs:148-156).
- Em `verdict`, antes de fixar o passo: `let index = RecipeIndex(*depth - r); if
  elem.is_guarded(index) { continue; }` (lib.rs:472-474). `RecipeIndex(depth - r)` é
  o índice **estável** da recipe contada innermost-first (lib.rs:447 calcula
  `depth`).
- Ao aplicar: `recipe.apply(..., output.into_owned().guarded(guard))`
  (lib.rs:363-367) — o **input** é guardado e passado ao closure. Como a recipe
  tipicamente embrulha o input no output, a cópia guardada fica aninhada no corpo
  produzido → no passe seguinte essa cópia é saltada → **terminação**.
- (Há também `Style::Revocation(RecipeIndex)` (styles.rs:225) + `SmallBitSet revoked`
  (lib.rs:1223-1233, 1256-1258), mas é **só para recipes de regex** — show rules
  normais usam os guards por-nó, como diz o próprio comentário em styles.rs:221-224.)

### B3 — Show-set (`#show k: set ...`): acumula no map, **não** consome o passo
- A recipe tem `transform: Transformation`; o caso show-set é
  `Transformation::Style(styles)` (styles.rs:454, 459, 503-505).
- Em `verdict`: `if let Transformation::Style(transform) = recipe.transform() { if
  !prepared { map.apply(transform.clone()); } continue; }` (lib.rs:459-464). →
  empurra os styles para o `map` (que entra na chain via `styles.chain(&map)`,
  lib.rs:358) e **continua** — não vira `step`. O nó renderiza sob a chain
  aumentada.

### B4 — Scope: regra dentro de bloco não vaza para irmão
- Show rules são empacotadas num `StyledElem { child, styles }`
  (content/mod.rs:744-752); só o `child` carrega esses styles. A chain é construída
  por-subárvore (`StyleChain { head, tail }`, links em styles.rs:848-857), logo um
  irmão fora do `StyledElem` **não vê** a recipe. (Show rule sem seletor é a exceção
  — é aplicada *eagerly* ao resto do scope: styles.rs:449-451 / `styled_with_recipe`,
  mas continua confinada ao scope.)

### B5 — Identidade: a regra precisa de id de elemento estável p/ casar **e** revogar
- O seletor `Selector::Elem(element, dict)` casa por **igualdade de id de elemento**:
  `target.elem() == *element && ...` (selector.rs:133-138). `Element` é um id estável
  do tipo de elemento.
- O guard usa `RecipeIndex` (posição estável na chain), não o id do elemento — mas
  para **casar** a recipe ao nó é estritamente preciso `elem()` estável
  (selector.rs:134). Sem id estável, nem o match nem (por consequência) a aplicação
  do guard certo acontecem.

---

## 2. Os 5 casos do spike + output real

Correr: `cd lab/spikes/f-extensao/e1 && cargo run --release`.
Código: `src/core.rs` (harness `#show`: `Recipe`, `Transformation`, `ShowChain`,
`realize`/`realize_pass`/`apply_recipes`, guards via `Content::Guarded`,
show-set via `Content::Styled`), `src/main.rs::show_cases()`.

Output medido (`=== 5 #show cases ===`):

```
[1 multi-rule]   [WARN|A] body  [3 passes]
                 -> innermost (danger) fires pass 1; outer (warn) pass 2 — final tone = last-applied = WARN
[2 recursion]    ## >> # Title  [2 passes — TERMINATED via guard]
[3 show-set]     [WARN|S] body  [1 pass]
                 -> instance tone=note, show-set forces WARN at render
[4 scope]        [DANGER|scoped] in block[NOTE|sibling] outside
                 -> 1st callout DANGER (in scope); sibling stays NOTE (rule didn't leak)
[5 native+dyn]   ### [H] Native H[DANGER|Dyn] user element  [2 passes]
                 -> heading rewritten (native) AND callout tone=danger (dynamic), one chain
```

| Caso | Comportamento provado | Pass/Fail |
|------|-----------------------|-----------|
| 1 Multi-regra | innermost-first, 1 regra/passe; danger (interna) passe 1, warn (externa) passe 2; final = última-aplicada = WARN; converge em 3 passes | **PASS** |
| 2 Recursão | recipe que emite o próprio kind (`heading` que embrulha `heading`) — termina em 2 passes via guard | **PASS** |
| 3 Show-set | `#show callout: set callout(tone:"warn")` — instância tone=note é sobreposta a WARN no render, sem consumir step (1 passe) | **PASS** |
| 4 Scope | 1ª callout sob recipe scoped (DANGER); irmão realizado fora do scope fica NOTE — regra não vaza | **PASS** |
| 5 Native+Dynamic | a MESMA chain transforma um `heading` nativo **e** o `callout` dinâmico | **PASS** |

(Sanity extra no demo end-to-end: `render (#show): ...[DANGER|...]... [2 pass]` — regra
única força tone=danger nas duas callouts e estabiliza em 2 passes.)

---

## 3. Requisitos que a semântica vanilla EXIGE da fronteira E1 (S1…S7)

Consumidos pelo L0 do F (Parte 3). Crisp e acionáveis.

- **S1 — Identidade de elemento estável no trait dinâmico.**
  O match de `#show <elem>:` faz `target.elem() == *element` (selector.rs:134). Para a
  arm `Content::Dynamic`, o `trait Element` **tem** de expor um id estável e comparável
  por igualdade. O spike usa `fn kind(&self) -> &'static str` (core.rs, trait
  `Element`); o produto precisa de um `element_kind()`/`ElementId` estável e `Eq`
  (string interned ou TypeId-like), idêntico para todas as instâncias do mesmo
  elemento dinâmico. **L0 deve declarar o método de identidade no contrato.**

- **S2 — Guards por-nó (revocação) têm de viver no nó, não num wrapper.**
  Vanilla guarda no `meta().lifecycle` bitset do elemento empacotado
  (content/mod.rs:148-156), inclusive nos nós **dinâmicos**. O spike só conseguiu
  modelar isto com um wrapper `Content::Guarded` (porque o `Arc<dyn Element>` não tem
  campo meta). O produto E1 **tem** de carregar o conjunto de guards/lifecycle no
  *próprio* nó dinâmico (campo meta partilhado, fora do `dyn Element`, p.ex. no
  invólucro do `Content::Dynamic`), senão a recursão não termina de forma local.
  **L0 deve especificar onde mora o lifecycle/guard set de um nó dinâmico.**

- **S3 — Recipes na StyleChain + ordem innermost-first determinística.**
  A realização precisa de iterar as recipes da chain em ordem innermost-first e parar
  no 1.º match func não-guardado (1 step/passe — lib.rs:449-486). **L0 deve fixar:
  recipes coabitam com sets na chain, ordem = innermost-first, 1 transformação func por
  passe.**

- **S4 — Loop de realização multi-passe até fixpoint.**
  Um único passe não basta (caso 1 precisou de 2 passes para aplicar ambas as regras;
  caso 2 de 2 para terminar). A realização é um loop que re-realiza o conteúdo
  produzido até estabilizar, com os guards a garantir convergência (vanilla:
  `route.increase()/check_show_depth()` lib.rs:401-407 + introspection loop). **L0 deve
  prever múltiplos passes + um teto de profundidade.**

- **S5 — Show-set: a transformação pode ser styles, não só conteúdo.**
  `Transformation` tem de incluir um caso `Style(styles)` que empurra um `#set` scoped
  e **não** consome o show step (styles.rs:459-464, lib.rs:459-464). O contrato do nó
  dinâmico precisa de resolver props pela chain aumentada no render (o spike já o faz:
  `Callout::render` lê `chain.get("callout","tone")` antes do valor de instância).
  **L0 deve definir o trio `Transformation = Content | Func | Style`.**

- **S6 — Scope via subárvore estilizada (StyledElem).**
  Recipes confinam-se à subárvore que embrulham (content/mod.rs:744-752). A fronteira
  E1 precisa que o conteúdo dinâmico participe nessa hierarquia de chain como qualquer
  nativo — i.e. `Content::Dynamic` é um nó de 1.ª classe que pode ser filho de um
  `StyledElem` e cuja realização recebe a chain scoped. (O spike confirma: caso 4, o
  dinâmico respeita o scope.) **L0 deve afirmar que o nó dinâmico é membro pleno da
  árvore de realização (pai/filho/chain), não um leaf opaco.**

- **S7 — O closure de `#show` recebe o nó (possivelmente guardado) e devolve
  `Content` arbitrário.**
  Vanilla: `recipe.apply` chama `func.call(.., [content])` e usa `result.display()`
  (styles.rs:494-503). A fronteira E1 tem de permitir que um closure user receba um
  `Content::Dynamic` e devolva qualquer `Content` (nativo, dinâmico, sequência). O
  contrato dinâmico precisa de uma forma de **introspeção de campos** para o closure
  (`get_prop`/`children` no spike; vanilla `Content::get`/`field` em
  content/field.rs). **L0 deve garantir leitura de campos do elemento dinâmico a partir
  do closure** (id + acesso a props), senão `it => it.body` / `.with(..)` não funciona
  em elementos dinâmicos.

---

## 4. Travas / contradições com a fronteira E1

Nenhuma contradição **dura** foi encontrada: os 5 casos passam sobre
`Content::Dynamic(Arc<dyn Element>)`. Mas há **2 dependências** que o L0 da Parte 3 tem
de marcar explicitamente, sob pena de a fronteira não suportar a semântica:

- **Trava-Q1 (depende de S2): onde mora o lifecycle/guard set de um nó dinâmico?**
  Vanilla guarda no meta bitset do elemento empacotado. Em E1, `Arc<dyn Element>` é
  imutável e partilhado; o guard set **não** pode viver dentro do `dyn Element` (mudaria
  por-instância de realização e custaria um campo no trait a cada user). A resolução
  natural é o guard/lifecycle viver no **invólucro** `Content::Dynamic` (um campo
  `meta` ao lado do `Arc<dyn Element>`), exatamente como os nativos têm meta no
  empacotamento. **O L0 tem de decidir e fixar isto** — é a única peça em que E1 diverge
  estruturalmente do vanilla (que mete o meta dentro do elemento). Se o L0 puser o guard
  dentro do trait, vira fardo do user e quebra a clonagem O(1) do `Arc`.

- **Trava-Q2 (depende de S1+S7): igualdade de id + introspeção de campos no trait.**
  Não é contradição, mas é uma **exigência mínima do contrato** que o L0 deve travar: o
  `trait Element` tem de expor (a) um id estável `Eq` para o match de seletor e (b)
  leitura de campos por nome para os closures de `#show`. O spike provou ambos com
  `kind()` + `get_prop()`/`children()`. Se o L0 do F deixar o trait sem estes, `#show`
  sobre elementos dinâmicos é impossível. **Marcar como dependência explícita no L0.**

---

## 5. Limites da medição (o que foi stubado vs realmente exercitado)

**Realmente exercitado:**
- Ordem innermost-first + 1 regra/passe (caso 1, 3 passes observados).
- Terminação por guard em recipe auto-produtora (caso 2).
- Show-set a sobrepor prop no render sem consumir step (caso 3).
- Scoping (caso 4) — modelado por realizar a subárvore scoped com uma `ShowChain`
  separada do irmão.
- Uniformidade native+dynamic na mesma chain (caso 5).
- Acumulação de guards entre passes (multi-regra converge, não oscila).

**Stubado / simplificado (não invalida os requisitos, mas o L0 não deve assumir que
foram provados a fundo):**
- **Guards num wrapper `Content::Guarded`**, não no meta do nó. É a razão de S2/Trava-Q1
  — o spike *não pôde* meter o guard dentro do `Arc<dyn Element>`. A medição prova que a
  semântica é necessária; **não** prova o sítio de armazenamento do produto.
- **Scope modelado por chains separadas** (`inner_chain`/`outer_chain` no caso 4) em vez
  de um verdadeiro `StyledElem` na árvore + um único passe de realização que respeita a
  subárvore. A propriedade ("não vaza") é a mesma; a mecânica de empacotamento não foi
  exercitada.
- **Selector só por kind** — sem `.where(field: v)` (vanilla selector.rs:135-137),
  sem labels, sem regex/`Revocation` (que é só regex de qualquer modo).
- **Filhos de elementos dinâmicos não são re-realizados** (`realize_children` trata
  `Content::Dynamic` como folha já-realizada — ver comentário em core.rs). As callouts
  do demo têm corpo de texto simples; recipes que mirem conteúdo *dentro* de um
  container dinâmico não foram exercitadas. Implica que S6 precisa de um
  `with_children`/walk no trait para o caso geral — registado mas não testado.
- **Sem comemo/track, sem introspection loop real, sem locations/tags** — o loop de
  fixpoint aqui é um `while changed` puro; vanilla tem o introspection loop e o route
  depth (lib.rs:401-407). A convergência foi provada empiricamente (assert de
  `max_passes` nunca dispara), não por construção formal.

---

### Ponteiros
- Spike: `lab/spikes/f-extensao/e1/src/core.rs`, `.../src/main.rs`, `.../src/callout.rs`.
- Este doc: `00_nucleo/diagnosticos/f-spike2-show-passo-333.md`.
