# Relatório P356 — caso 1, lacuna (i): show-set + func (o exemplo canônico do doc)

> **Desfecho.** A lacuna (i) do recon do P355 está **fechada**: `#show heading: set text(…)` **+**
> `#show heading: it => …` no mesmo elemento — o crystalline deixou de **perder** o show-set; agora
> a show-set casa o **elemento** e o seu `Styles` embrulha o **output da func**, espelhando o vanilla
> (`map.apply` + func sob `chained`). Honra a composição como **língua** (lição do P355). **NÃO**
> conserta a lacuna (ii) (múltiplos func same-kind — A2/A3, fatia seguinte), **declarada** num teste.
> Conserto **mínimo** (1 identificador: `&work` → `node`), **não reabre o α**. Suíte **2733 → 2736**;
> lint **0/0**; lente **66**; perf sem regressão.

**HEAD**: pós-P353 (1eb216303). **Branch**: Tekt. Caveat `RUST_MIN_STACK=33554432`.

---

## 1 — Fase A (a fonte vence; `file:line`)

- **Doc de referência (gate, lição do P355)** — `lab/typst-original/docs/reference/language/styling.md`,
  *Show rules*: o exemplo canônico com **4× `#show heading`** (3 show-set + 1 transform), *"keeping
  styling composable"*, *"good practice"*. Define o comportamento-alvo e classifica o combo como
  **língua**.
- **Mecanismo vanilla** — `typst-realize/src/lib.rs:458-464` (`Transformation::Style(t) => {
  map.apply(t); continue }` — dobra na chain, não consome o passe); `:357` (`chained =
  styles.chain(&map)`); `:341` (a func vira `step` e aplica **sob** `chained`). → a show-set fica
  **ativa quando a func realiza**.
- **Causa no crystalline (medida)** — `rules/eval/rules.rs`, loop de show-set em `apply_all`: casava
  `selector_matches(&work, …)` com `work` = **output pós-func**. O output da func não casa o seletor
  do elemento → **show-set perdida**. (Confirmado por probe P355: `set text(bold)` ⨁ `it=>[X:]+body`
  → `"X:"` não-bold.)
- **Travas pré-confirmadas verdes**: múltiplos show-set já compõem (P352, `collapse`); `p348`/caso 2
  intactos; **0 testes** no combo show-set+func.

## 2 — Estágio L0 (Trava, aprovada)

`entities/show.md`: o bullet `Transformation::Style` ganhou a secção **"Ordem show-set-vs-func"** — a
show-set dobra com base no **elemento** (não no output da func); quando show-set **e** func casam,
o output da func é embrulhado no `Styles` da show-set (espelha `chained`); cross-ref `lib.rs:341,357,
458-464` + `styling.md`. O §Fora de escopo foi estreitado para a **lacuna (ii)** (múltiplos func,
A2/A3). Hash sincronizado (`show.rs` → d8447ab0); lint 0/0. **Trava aprovada pelo dono** antes do `.rs`.

## 3 — Estágio 1 — o conserto (`file:line`)

`rules/eval/rules.rs`, loop de show-set em `apply_all` (a linha que era `selector_matches(&work, …)`):
passa a **`selector_matches(node, …)`** — casa a show-set contra o **nó original** (o elemento que
entra na realização); o `work` (output da func) continua a ser embrulhado no `Styles` coletado
(`collapse`). É a única mudança de lógica (1 identificador + comentário). Invariantes preservados por
construção: **sem func, `work == node`** → idêntico ao P352 (show-set-só e múltiplos show-set por
`collapse`). O loop α (func/recursão) **não é tocado** — a show-set é `Transformation::Style`, fora
do guard/morph.

## 4 — Estágio Teste (3 novos; `eval/tests.rs`)

- `show_set_mais_func_estilo_alcanca_output` — **o exemplo canônico**: `#show heading: set text(bold)`
  ⨁ `#show heading: it => [X:]+it.body` → `plain_text` tem `"X:T"` (func aplicou) **e** existe um
  `Content::Styled` com `bold` embrulhando o output (show-set **não** perdida). Antes: sem Styled.
- `multiplos_show_set_continuam_compondo` — **regressão**: `set text(bold)` ⨁ `set text(weight:700)`
  → um `Styled{bold, weight}` (P352 `collapse` intacto).
- `multiplos_func_same_kind_ainda_diverge_lacuna_ii` — **divergência ABERTA E DECLARADA**: dois `func`
  same-kind → só a 1ª efetiva (`"A:T"`, sem `"B:"`); o teste asserta o comportamento **atual
  declarado**, **não** finge paridade; cita o recon (A2/A3, fatia seguinte). (Lição S5b — o conserto
  parcial não mascara o buraco maior.)

## 5 — Gates (todos verdes)

```
build: workspace limpo (só warnings pré-existentes).
suíte (RUST_MIN_STACK=33554432): 2733 → 2736 (+3 novos; ZERO asserção existente alterada — o combo
  show-set+func tinha 0 testes). Demais crates: 472/24/2/21, 0 falhas.
lint: crystalline-lint . = 0/0.
ACEITAÇÃO (oráculo = vanilla + styling.md): show-set + func → show-set aplica sob o func (paridade);
  múltiplos show-set compõem (P352 verde); múltiplos func same-kind = DIVERGÊNCIA DECLARADA (lacuna ii).
INTACTOS: α/caso 2 (p348 verde), caso 4 (P340), morph ==/morph_canon (P345), flag P350c, Marco G.
lente (critério 3): content→elements = 66 INALTERADO; elemento→elemento = 0.
perf (critério 4, mesma sessão — o 1.1991s do P353 era cross-session, drift confirmado):
  antes (binário P353, nesta sessão) = 0.7564 s ± 0.0274; depois (P356) = 0.7473 s ± 0.0226 →
  Δ dentro do σ, sem regressão (conserto de 1 linha em caminho frio).
L0 (critério 5): show.md (ordem show-set-vs-func) + hash sincronizado ANTES do código; Trava aprovada.
```

## 6 — Estado e próximo

Tocados: `show.md` (+ hash em `show.rs`), `rules.rs` (1 identificador + comentário), `eval/tests.rs`
(+3). Nenhuma outra camada (mudança interna a `apply_all`, sem API). Árvore limpa fora de docs/`tools`.

**Fora de escopo / próximo**: lacuna (ii) — múltiplos func same-kind (A2/A3, fatia seguinte, com a
demanda medida e a decisão do dono); DEBT-60 (contador `1.1`≠`0.1` + supplement "Secção", P357); F-5
de-bake (adiado P353); F-6; Marco G; flag CLI (DEBT-59).
