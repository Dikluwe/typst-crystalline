# Relatório P368 — item (3): `#set <elemento-de-usuário>(prop:)` pelo mapa aberto (extensibilidade)

> **Desfecho.** A extensibilidade prometida do F-B está **entregue**: `#set <userelem>(prop:)`
> passa a pôr a prop no **mapa aberto** (`custom`) da chain, e o elemento de usuário a **lê pela
> chain** no layout (precedência **construído explícito > chain > default**, casando o vanilla).
> **Adição de capacidade** (não de-bake): suíte **2738 → 2742** (+4 fixtures provam set + read +
> precedência + sem-set; rede +11 e `#set` nativos **inalterados** — não-regressão). lint **0/0**,
> perf **0.7000s** (sem regressão). **Item (3) fechado** — e com ele o que faltava do F pelos
> princípios, **exceto** o F-5b (lote arquitetural dedicado, DEBT-61).

**HEAD**: pós-P367 (7653605d2). **Branch**: Tekt. Justificativa = a **extensibilidade do F-B**;
oráculo = vanilla 0.14.2.

---

## Fase A — caminho inteiro + oráculo vanilla (medido; `file:line`)

- **Oráculo vanilla:** `#set Elem(field:)` → `target.set(engine, args)` põe um `Style` na chain
  (`typst-eval/rules.rs:23`); o campo resolve-se da chain. **Precedência: explícito > set >
  default** (= o "instância → chain → default" do §3b.1).
- **Set:** `eval_set_rule` caía em `unsupported_target_warn` para alvo não-nativo (`rules.rs:501`).
  O registry não está no `Engine`, mas os user-elements ficam em `scopes` como `Func::element`
  (`eval/mod.rs:265`; `Func::element_name`, `func.rs:123`).
- **Read:** o layout do `Content::Dynamic` lia **só campos construídos** (`dyn_get_field`,
  `layout/mod.rs:538`), nunca a chain.
- **Transporte:** a fatia-1 (`eval/mod.rs`) só embrulhava a cauda em mudança de `NUM_KEYS` →
  um custom de usuário **não** chegava à chain do layout. (Lacuna que o desenho aprovado previu.)
- **Demo:** o `tone` do `callout` é obrigatório → usar o `badge` (`label`, sem body, renderiza
  via `plain_text`) com um campo **opcional** novo.

---

## Estágio L0 (Trava aprovada) — `§3a.11`

Realiza o §3b.1 (canal aberto + resolução `instância → chain → default`): set (`eval_set_rule` →
custom), read (elemento resolve da chain), fixture (campo opcional), público typst = `#set` em
`.typ` sobre elemento registrado (definir elementos em `.typ` = fronteira P369). Hash sincronizado
ANTES do código; Trava aprovada.

---

## Estágio 1 — set side (`file:line`)

- **`eval_set_rule`** (`rules.rs:501`+): para um `target` que resolve em `scopes` a um
  `Func::element`, em vez do warn, empurra `("<kind>.<prop>", Value)` no `custom` da chain por
  arg nomeado. Aditivo — os `#set` nativos não mudam.
- **Transporte generalizado** (`eval/mod.rs` `eval_markup`): o snapshot só-de-`NUM_KEYS` virou
  snapshot do **canal custom inteiro** (`engine.styles.collapse().custom`); o `wrap_start`
  dispara quando **qualquer** custom muda; o `Content::Styled` carrega **todos** os customs
  mudados. **Morph-safe** (o custom é transparente, `is_semantically_empty` ignora-o, P366) e
  **backward-compatible** (numbering puro → só os customs de antes).

## Estágio 2 — read side (`file:line`)

- **`Element::resolve_settable`** (`elements/mod.rs`, default = self) + bridge
  `DynElement::dyn_resolve_settable` (`dynamic.rs`): o elemento resolve campos **opcionais** de um
  `get(prop)` (a chain custom, já namespaced pelo kind), precedência **construído > chain**.
- **`BadgeElem`** (`test_callout.rs`): campo opcional `note: Option<EcoString>` (afeta o
  `plain_text`); `resolve_settable` preenche-o de `get("note")` quando `None`.
- **Layout `Content::Dynamic`** (`layout/mod.rs:538`): resolve o elemento via
  `dyn_resolve_settable(&|prop| self.chain.custom("<kind>.<prop>"))` antes de renderizar.

---

## Fixtures novos (a capacidade; declarados)

- `f_item3_set_badge_prop_entra_no_mapa_aberto` (eval): `#set badge(note: "x")\n#badge("L")` →
  `badge.note="x"` no `custom` da árvore (set + transporte).
- `f_item3_set_badge_note_resolve_da_chain` (layout): badge sem `note` sob `Styled{badge.note=…}`
  → output `"L (…)"` (read da chain).
- `f_item3_construido_explicito_vence_a_chain`: `note` construído ignora o `#set` (precedência).
- `f_item3_sem_set_badge_sem_note`: sem transporte → só o `label`.

---

## Gates (todos verdes)

```
build: workspace limpo (warnings = 8 pré-existentes, em ficheiros não tocados; este lote
  adicionou 0).
suíte (RUST_MIN_STACK=33554432): typst-core 2738 → 2742 (+4 capacidade; rede +11 e #set nativos
  INALTERADOS — não-regressão); demais 472/24/2/21, 0 falhas.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (oráculo = vanilla):
  - #set <userelem>(prop:) põe a prop no custom (mapa aberto); o elemento a lê da chain no
    layout; o output reflete a prop. Precedência explícito > set > default (vanilla).
  - público typst: `#set badge(note:)` em .typ sobre elemento registrado funciona; DEFINIR
    elementos com props setáveis puramente em .typ = fronteira (P369, P362-C: definição = Rust).
  - não-regressão: #set nativos (heading/equation/figure/page/par/text) + rede +11 inalterados.

INTACTOS (confirmado): TextStyle/F-5b (não tocado, DEBT-61); α/caso 2, morph ==/morph_canon
  (o custom é transparente à morfologia, P366), caso 4, flag P350c, Marco G; os 3 numbering.
lente (instrumento): não re-corrida (externa); analiticamente sem aresta content→elements nova
  (o set lê scopes/Func; o read reusa dyn_* + chain). content→elements = 66 esperado.
perf: 0.7000 s ± 0.0086 (n=19); P367 n/a, P365 0.6911 — sem regressão (o collapse()-por-child do
  transporte é negligível à escala do corpus).
L0 (critério 5): f_fronteira_e1.md §3a.11 + hash sincronizado ANTES do código, Trava aprovada.
commit: árvore commitada (hash abaixo).
```

---

## Estado / próximo

**Tocados:** `f_fronteira_e1.md` (§3a.11, L0) + backings; `eval/rules.rs` (set), `eval/mod.rs`
(transporte), `elements/mod.rs` + `elements/dynamic.rs` (resolve_settable), `elements/test_callout.rs`
(badge.note), `layout/mod.rs` (read), `eval/tests.rs` + `layout/tests.rs` (fixtures).

**Os 3 itens da auditoria P362:** (1) fonte única — 3/4 (numbering fechado; `TextStyle`/F-5b
adiado, DEBT-61); (2) F-6 — coberto (P367); **(3) extensibilidade — fechado (este lote).** **O F
está completo pelos princípios, exceto o F-5b** (lote arquitetural dedicado: modelo
strong/emph/styled + α-fixpoint, DEBT-61).

**Fronteira declarada (público typst):** definir elementos com props setáveis **puramente em
`.typ`** (sem Rust) é o **P369** (a outra metade do público typst); este lote entrega o `#set`
sobre elementos **registrados**. **Termino aqui — não emendo o seguinte (Trava 5).**
