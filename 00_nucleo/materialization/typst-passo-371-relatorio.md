# Relatório P371 — F-5b fatia (1): distinção de tipo `strong`/`emph`/`text` (fidelidade 0107)

> **Desfecho.** `strong` e `emph` voltaram a ser **tipos distintos** (variantes próprias
> `Content::Strong`/`Content::Emph`, modelo D) — o colapso do P101 (que os fazia
> `Content::Styled[Bold/Italic]` indistinguíveis de `#set text`) está **superado**. Dá à
> **ADR-0107** a fidelidade ao vanilla: `*bold* ≠ _italic_ ≠ #set text X ≠ X` (distinção por
> **tipo**). **Sem vtable** (variante de enum, `match` exaustivo). **GATE DO α verde**: o caso 2
> (α-fixpoint) + a rede de caracterização passam **sem alteração** — a distinção **não reabriu** o
> caso 2 (confirma a inferência do P370). **Render byte-idêntico** (paridade visual). Suíte
> **2742 → 2747** (+5: 4 dos módulos novos + 1 da distinção; **zero** asserção existente virou — a
> divergência `strong == #set text` era latente, não testada). lint **0/0**.

**HEAD**: pós-P370 (388a5a462). **Branch**: Tekt. **Não é emenda de ADR** (P370: conflito
aparente); só a nota "P101 superado" no L0.

---

## Fase A — a forma + o α (medido; `file:line`)

- **Forma (decisão do dono):** **(1) variante própria** (modelo D, 0026 `:63`/0105-D). Medido:
  ~9 sítios exaustivos × 2 = arms de hub; vs (3) discriminante em `Styles` (punhado, mas não
  variante). O dono escolheu (1) — o modelo prescrito.
- **`==` rumo ao vanilla:** hoje `strong X == #set text(bold) X` (ambos `Styled[Bold]`); o vanilla
  dá `≠` (`Packed::eq` por id de elemento). A distinção corrige; **nenhuma asserção direta** o
  codificava (latente).
- **α (gate duro):** o `morph_canon` mantém o `Styled[Bold]` não-vazio (`content.rs`); Strong/Emph
  como variantes **herdam** isso (mantidos via `_ => None` + recursão `map_content`) — a
  auto-igualdade (`Strong(x)==Strong(x)`) preserva-se → terminação inalterada. **Provado** no
  Estágio 2.
- **Fronteira fatia 2:** esta fatia é só o **tipo**; o de-bake do render `#set text` fica fora.

---

## Estágio L0 (Trava aprovada) — `§3a.12` + nota P101-superado

Desenho da variante própria (modelo D), os ~18 arms, o render-replica no layout, o `morph_canon`,
o `#show` por variante, a nota de que o colapso P101 é **superado** pelo modelo de variantes
(0026/0105). Hash sincronizado ANTES do código; Trava aprovada.

---

## Estágio 1 — a distinção (`file:line`)

- **+2 variantes** `Content::Strong(Arc<StrongElem>)`/`Emph(Arc<EmphElem>)` (`content.rs`) + **2
  módulos** `entities/elements/strong.rs`/`emph.rs` (`StrongElem{body}`/`EmphElem{body}` impl
  `Element`, delegação ao body). **Sem vtable/`dyn`/proc-macro.**
- **Construtores** `Content::strong/emph` → as variantes (eram `Styled[Bold/Italic]`).
- **Hub (arms uniformes, model D):** `plain_text`, `is_empty`, `eq`, `get_field`, `map_content`,
  `map_text` (`content.rs`) — cada um ganha `Content::Strong(e) => e.<m>()`/`Emph`. (`hash_content`
  é por Debug → automático.) **Catch-alls corrigidos:** `is_empty`/`eq`/`get_field` tinham `_ =>
  false`/`None` que dariam comportamento errado — arms explícitos adicionados (lição ADR-0105 cl.3).
- **`morph_canon`:** sem arm dedicado — `_ => None` mantém Strong/Emph (morfologia) e recursa o
  body; distintos do `Styled[Bold]` do `#set text` → `strong X ≠ #set text X`.
- **Render idêntico (layout `mod.rs`):** arms `Content::Strong`/`Emph` **replicam** o arm `Styled`
  (push `Bold(true)`/`Italic(true)` na chain, layout do body, restore) → output byte-idêntico.
- **`#show strong`/`emph` (`eval/rules.rs`):** `is_bold_styled`/`is_italic_styled` **removidos**;
  casam `Content::Strong`/`Emph` por **variante** (S1). `#set text(bold)` (`Styled[Bold]`) **não**
  casa `show strong`.
- **introspect:** `walk` (transparente, desce no body), `materialize_time` (reconstrói via ctor),
  `is_locatable` (Strong/Emph → false).

## Estágio 2 — o gate do α (`file:line`, o número)

`cargo test -p typst-core`: **2747 passed; 0 failed**. Inclui o **caso 2** (α-fixpoint, P342–P350c)
e a **rede de caracterização** (render), **verdes sem alteração** → a distinção **não reabriu** o
α nem mudou o render. (Se reabrisse, o passo reverteria; não foi preciso.)

---

## Asserções que viraram (declaradas, S5b)

- **Nova:** `f5b_strong_distinto_de_set_text_bold` (`content.rs`) — prova `strong ≠ #set text(bold)`,
  `strong ≠ emph`, `strong` é `Content::Strong`, e a auto-igualdade (o α).
- **Atualizadas (não viraram comportamento, só a forma):** `map_content_bottom_up…` casava
  `Content::Styled` (o wrapper de strong); agora casa `Content::Strong` (a variante). Comentários
  "Passo 101: strong→Styled" atualizados para "P371: strong→Strong (P101 superado)". **Nenhuma
  asserção de `==` existente virou** (a divergência `strong==#set text` não era testada).

---

## Gates (todos verdes)

```
build: workspace limpo (8 warnings lib = pré-existentes; este lote adicionou 0).
suíte (RUST_MIN_STACK=33554432): typst-core 2742 → 2747; demais 472/24/2/21, 0 falhas.
GATE DO α (duro): caso 2 (P342–P350c) + rede de caracterização — VERDES sem alteração. ✅
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (oráculo = vanilla 0.14.2):
  - distinção: strong X ≠ emph X ≠ #set text X ≠ X (por tipo, como o vanilla). ✅
  - render idêntico: strong/emph renderizam bold/italic como antes (rede de caracterização). ✅
  - sem vtable: variante de enum estática (cláusula 0026 respeitada). ✅

INTACTOS (confirmado): o de-bake do render #set text (fatia 2 — não tocado); caso 4, flag P350c,
  Marco G; os 3 numbering (P364/P365); o #set de props de usuário (P368).
lente (instrumento): não re-corrida (externa). Analiticamente +2 arestas content→elements::{strong,
  emph} (66→68 esperado) — NÃO é regressão de atomização: a 0026/0105 prescrevem variantes
  (é o modelo prescrito). Registrar o delta.
perf: 0.7163 s ± 0.0067 (n=19); P368 0.7000 — ≈neutro (dentro do ruído cross-session; +2 variantes
  + render-replica são negligíveis).
L0 (critério 5): f_fronteira_e1.md §3a.12 + nota P101-superado, hash sincronizado ANTES do código,
  Trava aprovada.
commit: árvore commitada (hash abaixo).
```

---

## Estado / próximo

**Tocados:** `f_fronteira_e1.md` (§3a.12, L0) + backings; **novos** `entities/elements/strong.rs`/
`emph.rs`; `content.rs` (variantes + ~6 arms de hub + construtores + testes), `eval/rules.rs`
(`#show` por variante), `layout/mod.rs` (render-replica), `introspect.rs` (walk/materialize_time),
`introspect/locatable.rs`, `elements/mod.rs`.

**F-5b — fatia (1) feita.** A **fatia (2)** é o **de-bake do render `#set text`** (o estilo de
render viaja transparente à morfologia pelo `custom`, como o numbering — P366; o `morph_canon`
distingue render-transparente de morfologia-mantida, agora que strong/emph são variantes próprias).
Eu a escrevo/executo quando o dono rodar esta e subir o relatório. **Termino aqui — não emendo o
seguinte (Trava 5).**

## Fora de escopo (confirmado)

A **fatia (2)** do F-5b (de-bake do render `#set text`); o **Marco G** (não-F); DEBT-59 (flag CLI);
DEBT-60 (contador); qualquer toque no caso 4, no Marco G ou na flag.
