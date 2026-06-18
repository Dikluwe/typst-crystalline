# Relatório P358 — caso 1, lacuna (ii): conserto de ordem innermost-first (fecha o caso 1 e a F-realização)

> **Desfecho.** A lacuna (ii) está **fechada por conserto de ordem** (decisão do dono, P357): a
> travessia de regras **func** passa a **innermost-first** (última-declarada vence), casando o
> **`"B:T"`** do vanilla 0.14.2. **Não é acumulação** — o P357 mediu que o vanilla não acumula func
> same-kind (aplica um, ou erra no caso mesmo-kind); a única divergência real era de **ordem**.
> **A2/A3 declinados.** Conserto **mínimo** (uma linha: `node_rules.iter().rev()` no loop func),
> **não toca o α**. O teste `multiplos_func_…_lacuna_ii` (P356) **virou de divergência para
> paridade**. **Com isto o caso 1 fecha e, com ele, a F-realização** (casos 1–4 feitos).
> Suíte **2736** (1 asserção alterada, justificada); lint **0/0**; lente **66**; perf sem regressão.

**HEAD**: pós-P356 (37adaf867). **Branch**: Tekt. Caveat `RUST_MIN_STACK=33554432`.

---

## 1 — Por que a reordenação é legítima (o que mudou desde P352/P355)

No P352/P355 marquei o conserto de ordem como **mascaramento** — consertaria a polaridade mas
deixaria o caso de **acumulação (B1)** aberto/escondido. A medição do **P357** removeu a base: para
**func**, **não existe** caso de acumulação no vanilla (aplica um quando o output muda de kind;
**erra** no caso mesmo-kind). O B1 do spike era **show-set** (fold de estilo) = **lacuna (i)**, feita
no **P356**. Logo o conserto de ordem **não deixa buraco** — é o conserto **completo** da única
divergência real. A objeção do P355 caiu **junto com a premissa de acumulação**.

## 2 — Fase A (confirmada; `file:line`)

- Recon P357 (`f-recon-lacuna-ii-passo-357.md`): vanilla não acumula func; demanda zero na
  referência; inverter a iteração de regras é **no-op para regra única e cross-kind**.
- **O conserto é mais estreito do que "inverter `node_rules`"**: há **dois** loops sobre
  `node_rules` — o **func** (`rules/eval/rules.rs:152`) e o **show-set** (`:278`). Inverte-se **só o
  func**. O show-set **fica** em ordem de declaração + `collapse` (a **última-declarada sobrepõe** —
  como a referência promove, `styling.md`); invertê-lo quebraria essa polaridade.
- Travas pré-confirmadas: 5 testes do caso 2 (regra única → no-op); show-set (P352); show-set+func
  (P356); cross-kind (`show_rule_encadeamento_duas_regras`) — todos inalterados.

## 3 — Estágio L0 (Trava, aprovada)

`entities/show.md`:
- Invariante **"Ordem das `func` (innermost-first)"** — última-declarada vence no subconjunto de
  um-func-efetivo (paridade `styles.rs:835` `next_back`); é reordenação, não acumulação; o fold de
  show-set **não** é invertido.
- §**"Caso 1, lacuna (ii) — FECHADA (P358)"** — a premissa de acumulação cai (P357); A2/A3
  declinados; o **residual** (func mesmo-kind-returning → recursão = caso 2, **paridade de erro**)
  declarado com gatilho de reabertura. Hash sincronizado (`show.rs` → b05a5f7a). **Trava aprovada.**

## 4 — Estágio 1 — o conserto (`file:line`)

`rules/eval/rules.rs:152`: o loop func `for rule in &node_rules` → **`for rule in
node_rules.iter().rev()`** (innermost-first; última-declarada primeiro). Uma linha + comentário.
**Não toca o α** (regra única = no-op; o `morph_canon`/teto/`active_guards` intactos). O loop de
show-set (`:278`) **inalterado** (ordem de declaração + `collapse`).

## 5 — Estágio Teste

- `multiplos_func_same_kind_ultima_declarada_vence` (renomeado de `…_ainda_diverge_lacuna_ii`, P356):
  `#show heading: it=>[A:]+it.body` ⨁ `…[B:]…`, `= T` → agora assere **`"B:T"`** (última-declarada
  vence) e **`!contains("A:")`** — **paridade com o vanilla 0.14.2** (medido P357). Era "A:T"
  (divergência). **Única asserção alterada.**
- Verdes (não regrediram): os 5 testes do caso 2 (no-op); show-set (P352); show-set+func (P356);
  cross-kind. Confirmado: é a única asserção que muda.

## 6 — Gates (todos verdes)

```
build: workspace limpo. suíte (RUST_MIN_STACK=33554432): 2736; UMA asserção alterada
  (multiplos_func_…: "A:T" divergente → "B:T" paridade — esperada/justificada vs vanilla). Demais
  crates: 472/24/2/21, 0 falhas.
lint: crystalline-lint . = 0/0.
ACEITAÇÃO (oráculo vanilla 0.14.2): 2 func same-kind (output muda de kind) → última-declarada vence
  ("B:T"), igual ao vanilla; residual mesmo-kind-returning → erro por recursão em ambos (caso 2,
  paridade de erro, declarada no L0).
INTACTOS: α/caso 2 (5 testes no-op verdes), caso 4 (P340), morph ==/morph_canon (P345), flag P350c,
  Marco G; show-set (P352) e show-set+func (P356).
lente (critério 3): content→elements = 66 INALTERADO; elemento→elemento = 0.
perf (critério 4, mesma sessão): antes (binário P356) = 0.8182 s ± 0.0248; depois (P358) =
  0.7800 s ± 0.0316 → Δ dentro do σ, sem regressão (reordenação O(regras)).
L0 (critério 5): show.md (ordem + residual) + hash sincronizado ANTES do código; Trava aprovada.
```

## 7 — Caso 1 e F-realização FECHAM

Com a lacuna (ii) fechada, **o caso 1 fecha**: (i) show-set+func (P356), (ii) ordem (P358); o
residual é caso 2. Com ele, **a F-realização fecha** — caso 1, caso 2 (recursão, P342–P350c/α),
caso 3 (show-set, P352), caso 4 (escopo, P340) todos feitos.

**Fila F restante**: F-5 de-bake (adiado no P353 — limpeza sem demanda), F-6 (3 folhas, DEBT-58),
Marco G (pós-F-6, spec própria). **Débitos nomeados**: DEBT-59 (flag na CLI), DEBT-60 (contador
`1.1`≠`0.1` + supplement "Secção", P359).

## Estado

Tocados: `show.md` (+ hash em `show.rs`), `rules.rs` (1 linha + comentário), `eval/tests.rs` (1 teste
renomeado/virado para paridade). Nenhuma outra camada (mudança interna a `apply_all`). Árvore limpa
fora de docs/`tools`.

## Fora de escopo (confirmado)

A2/A3 (declinados; só o gatilho de reabertura); DEBT-60 / contador (P359); de-bake F-5 (adiado P353);
F-6; Marco G; flag CLI (DEBT-59); qualquer toque no α / `morph_canon` / `==` ou na flag.
