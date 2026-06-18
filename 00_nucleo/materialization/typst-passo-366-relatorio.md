# Relatório P366 — F-5b: parar e reportar (obstáculo medido no transporte do `TextStyle`)

> **Desfecho.** **Parar e reportar / re-escopo** (sancionado pelo próprio passo: *"se a
> varredura achar um produtor que não pode migrar para a chain sem quebrar outra coisa, parar
> e reportar"*). Duas medições mudam o rumo: **(A)** o arrasto **não é o bold do heading**
> (enquadramento do passo) — é o **`#set text` tipado inteiro**; **(B)** o transporte tipado
> aprovado na Trava **viola o limite duro "não tocar a morfologia"**. **Nenhum `.rs`
> escrito.** Só o L0 (`§3a.10`, com o obstáculo registrado) + este relatório.

**HEAD**: pós-P365 (e1f09cc24). **Branch**: Tekt. Justificativa = princípio (fonte única).

---

## Fase A — a varredura (medida; `file:line`)

O `TextStyle` é assado em `Content::Text(EcoString, TextStyle)` via `TextStyle::from(&*engine.
styles)` (`eval/mod.rs:344/354/401`, `markup.rs:112`). O layout já lê a chain no merge
`mod.rs:609-626` (`node_style.X || self.style.X`).

**(A) Correção de enquadramento (ADR-0108).** O passo supõe que o arrasto é "o bold do
heading". **Medido, não é:**
- **heading bold** — `layout/mod.rs:704` seta `self.style = TextStyle{bold:true, size:…}` no
  arm Heading antes de descer no corpo → **já na chain**. Não é arrasto.
- **strong/emph** — `Content::strong/emph` = `Content::Styled([Bold/Italic])` → chain. Não é
  arrasto.
- **`#set text`** (size/fill/font/weight/tracking/leading/lang/bold/italic) — empurrado a
  `engine.styles` (`rules.rs:497/638`) e **assado** em `Content::Text` (`eval/mod.rs:344`),
  **sem** wrapper `Content::Styled` (a fatia-1 `eval/mod.rs:413-419` só embrulha em mudança de
  `NUM_KEYS` de numbering). **Este é o arrasto, e é o `#set text` tipado inteiro.** [medido]

Decisão do dono na Trava (com (A) na mão): **tudo no P366** (transporte tipado + remoção).

---

## Estágio 1 — o obstáculo medido (parar)

Ao desenhar o transporte (a cauda do `#set text` embrulhada num `Content::Styled` carregando o
`StyleDelta` **tipado**, per a Trava), a fonte mostra que **isso muda a morfologia**:

- `morph_canon` (`content.rs`) **mantém** um `Content::Styled` exceto se
  `styles.is_semantically_empty()`.
- `is_semantically_empty` (`style.rs:207-219`) conta **todos os campos tipados**
  (`bold/italic/size/fill/heading_level/weight/tracking/leading/lang/font`), ignorando **só**
  o `custom`.
- ∴ um `Styled` com o `#set text` **tipado** (ex. `size`) é **semanticamente não-vazio** →
  `morph_canon` **mantém-no** → o `#set text` torna-se **morfologicamente significativo**.
- Hoje o `#set text` é assado no `Text` e `morph_canon` o **descarta**
  (`Content::Text(s,_) => Text(s, default)`) — render, transparente (ADR-0107). O transporte
  tipado **inverteria** a igualdade morfológica: `[#set text(size:20) foo] == [foo]` passaria
  de **true** a **false**.
- `#set text` usa os **mesmos campos tipados** (`rules.rs:511+`: `delta.bold/italic/size/…`)
  que `*bold*`/`_italic_` (strong/emph), que **são** morfologia (ADR-0038, mantidos por
  `morph_canon`). São **estruturalmente indistinguíveis** — `morph_canon` não separa render de
  morfologia se ambos vivem nos campos tipados de um `Styled`.

**Limite duro violado:** *"Não tocar a morfologia / `morph_canon` / `==` / caso 4. O de-bake
é estilo de render."* O transporte tipado **toca** a morfologia. ∴ **parar** (S5b: não
descobrir quebrando; ADR-0108: o dono audita a substância).

---

## A direção corrigida (morph-safe — re-escopo)

O estilo de **render** do `#set text` tem de viajar pelo **canal `custom`** (que
`is_semantically_empty` ignora → `morph_canon` **desce**, transparente), tal como o numbering
já faz — **não** pelos campos tipados de um `Styled`. Consequência: o **layout** passa a ler o
estilo de render do **`custom`** (não de `self.style` tipado). É um mecanismo **diferente e
maior** do que o aprovado na Trava (transporte tipado). **Não cabe em "ajustar o que foi
aprovado" — é um novo desenho.**

**Opções para o dono:**
1. **Redesenhar o P366** com o transporte por `custom` (render-style no canal aberto; layout
   lê do `custom`). Eu redijo o L0 e o passo corrigidos; o dono reaprova antes de código.
2. **Fatiar** o redesenho (ex.: P366 = render-style no `custom` + layout lê do `custom`,
   aditivo; P367 = remover o `TextStyle` assado).
3. **Adiar** o F-5b (o item (1) fica com 3/4 caminhos fechados; o `TextStyle` permanece
   assado) e seguir para o **item (2) F-6** ou o **item (3) `#set` de user-props** — a fonte
   única dos 3 numbering já está estabelecida.

---

## Estado / gates

```
build/suíte/lint: INALTERADOS — nenhum .rs tocado. Suíte 2738, lint 0/0 (árvore = só L0 + doc).
lente: n/a (sem código). perf: n/a.
L0: f_fronteira_e1.md §3a.10 escrito com o desenho E o ⚠ OBSTÁCULO MEDIDO + direção corrigida;
  hash sincronizado.
commit: só L0 + relatório (padrão "reverte por contradição → commita só o relatório /
  para na Trava → commita só o L0").
```

**Tocados:** `f_fronteira_e1.md` (§3a.10 + obstáculo) + 9 backings (hash sync); este relatório.
Nenhum `.rs`.

**Próximo:** decisão do dono entre as 3 opções. **Termino aqui — não emendo o passo seguinte
(Trava 5).**
