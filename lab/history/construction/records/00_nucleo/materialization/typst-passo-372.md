# Passo 372 — F-5b fatia (2): de-bake do render `#set text` — fecha o F-5b

> **O que faz.** Última fatia do F-5b: o estilo de **render** do `#set text` (size/fill/font/weight/
> tracking/leading/lang/bold/italic) deixa de ser **assado** (no `TextStyle` do `Content::Text` e no
> `Styled` **tipado**) e passa a viajar pelo canal **`custom`** da chain — **transparente à
> morfologia**, como o numbering já faz (P366). O elemento lê o estilo de render do `custom` no
> layout → **fonte única** para o render do `#set text`. **O obstáculo que travou o P366 original
> desaparece**: lá, embrulhar o `#set text` num `Styled` **tipado** o tornava morfologicamente
> significativo (`is_semantically_empty` conta os campos tipados) → mudaria o `==`/α. Agora o caminho
> é o **`custom`** (que `is_semantically_empty` **ignora** → `morph_canon` desce, transparente), e a
> **fatia (1)** já tirou `strong`/`emph` para variantes próprias — então o `#set text` é a **única**
> coisa de render em `Styled`, e movê-lo para o `custom` não colide com morfologia. **O gate do α é
> duro de novo**: mover o render para o `custom` **não pode** mudar o que o α-fixpoint vê
> (`rules.rs:229`) — provar (caso 2 + rede +11). **Com esta fatia o F-5b fecha** → o item (1) da
> auditoria vai a **4/4** → o **F fica completo pelos princípios sem nenhuma divergência registrada**.
> **Não toca** o caso 4, a flag P350c, o Marco G, nem a distinção de tipo da fatia (1). **Design-
> first**: L0 + Trava antes de remover campo. **Commita ao terminar** e **não emenda o seguinte**
> (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P372 (confirmar livre).
**Pré-condição**: P371 fechado (fatia 1: `strong`/`emph` variantes próprias; o α provado intacto;
suíte **2747**; o `#set text` é a única coisa de render em `Styled[Bold]`). Lint **0/0**, lente
**68/0** (66 + as 2 variantes da fatia 1, delta esperado). Árvore limpa. HEAD pós-P371. Lente
`tekt-cargo-dsm` disponível (registrar versão/commit). Caveat de stack: `RUST_MIN_STACK=33554432`.
Se algo não bater, parar e reportar.
**Tipo**: F-5b fatia (2) de-bake — **fonte única do render `#set text`**. O `==` rumo ao vanilla: o
P370/P366 mediu que o vanilla dá `#set text X ≠ X` (o wrapper `StyledElem` é significativo por
**presença**, mas os **valores** são ignorados — `StyledElem::eq` compara só o child). O de-bake fiel
realiza isso: o render no `custom` (presença do wrapper distinguível, valores transparentes ao
`morph_canon`). A **rede de caracterização (+11)** é o oráculo de render; as asserções que mudam são
as que codificam o `#set text X == X` atual → viram para `≠`, **declaradas e justificadas** (S5b).

---

## Fase A — a varredura + o caminho render→`custom` + o α (a fonte vence; `file:line`)

**Antes de remover qualquer campo:**

1. **Reler** ADR-0107 (a distinção render vs morfologia, `:40-46`), ADR-0026/0105 (sem vtable), as
   **Travas anti-deriva**, o recon P370 (§4: a fatia 2 é separável; o render pelo `custom`) e o
   relatório do P366 (o obstáculo do tipado; a medição vanilla `StyledElem::eq` só-child).
2. **A varredura dos produtores do `#set text`** — onde o estilo de render é assado, com `file:line`:
   - o `TextStyle::from(&*engine.styles)` assado em `Content::Text` (`eval/mod.rs:344/354/401`,
     `markup.rs:112`);
   - o `Styled` **tipado** que o `#set text` produz hoje (os campos `delta.size/fill/font/weight/…`);
   - classificar cada produtor: **[produção, chain-custom]** (já vai pelo custom) / **[produção,
     tipado/assado]** (precisa migrar para o custom) / **[fixture]**.
3. **O caminho render→`custom`** — desenhar como o estilo de render do `#set text` entra no `custom`
   (a `PropKey` por campo? um agregado `text.render`?), e como o **layout** o lê do `custom` em vez
   do `TextStyle` tipado/assado. Confirmar que o merge do layout (`mod.rs:609-626`) passa a ler o
   render do `custom`. **A forma sai da medição** (Trava 1).
4. **A separação render-`custom` vs morfologia (o gate do α)** — confirmar, com `file:line`, que o
   render no `custom` é ignorado por `is_semantically_empty` (`style.rs:207-219`) → `morph_canon`
   desce (transparente) → o α-fixpoint **não vê** o render. E que a distinção de tipo da fatia (1)
   (`strong`/`emph` variantes) **continua** intacta (morfologia mantida). Declarar o que será rodado
   para **provar** (caso 2 + rede +11).
5. **A fidelidade vanilla do `==`** — o vanilla dá `#set text X ≠ X` por **presença** do wrapper, com
   **valores ignorados**. Confirmar que o de-bake realiza isso (o wrapper `Styled` com `custom` de
   render é distinguível de `X` por presença; o `morph_canon`/`==` ignora os valores do `custom` —
   espelho do `StyledElem::eq` só-child). Listar as asserções de `#set text X == X` que viram.

---

## Limites duros

- **Mover o render para o `custom` não pode tocar o que o α vê.** O gate do α (caso 2 + rede +11
  verdes) é aceitação — se reabrir, **parar e reverter**.
- **Não tocar a distinção de tipo da fatia (1)** (`strong`/`emph` variantes — morfologia, intacta).
- **Não usar campo tipado de `Styled` para o render do `#set text`** (era o obstáculo do P366 — torna
  significativo). O render vai pelo `custom`, transparente.
- **Não tocar o caso 4, a flag P350c, o Marco G.**
- **Render idêntico** (paridade visual, rede de caracterização) — mover a fonte do render não muda
  como renderiza. As asserções que mudam são **só** as do `==` (`#set text X == X` → `≠`), declaradas.

---

## Estágios

### Estágio L0 — desenho (com a varredura na mão) + Trava (PARA aqui para o dono)
Com os produtores classificados e o caminho render→`custom` desenhado, registrar: a forma da
`PropKey` de render, a migração dos produtores tipados/assados para o `custom`, a leitura pelo
layout, a transparência à morfologia (o gate do α), e as asserções de `==` que viram. Registrar no
L0 `f_fronteira_e1.md`. Sincronizar hashes. **TRAVA**: para no chat — a varredura + o desenho + os
hashes para o dono aprovar. Nenhum campo removido antes.

### Estágio 1 — render do `#set text` para o `custom` (após aprovação)
O `#set text` passa a pôr o estilo de render no `custom` da chain (não no `TextStyle` tipado/assado).
O layout lê o render do `custom`. Aditivo onde possível; o `TextStyle` assado do `Content::Text`
removido quando o `custom` o substitui em todos os caminhos de produção.

### Estágio 2 — o gate do α + a fonte única
Rodar o caso 2 (α-fixpoint) + a rede de caracterização (+11). **Verdes sem alteração** — o render no
`custom` é transparente à morfologia; o α não muda. Confirmar a **fonte única**: o `TextStyle` assado
removido; só o `custom` carrega o render do `#set text`.

### Estágio Teste
- **Render**: a rede de caracterização passa — `#set text(...)` renderiza idêntico (paridade visual).
- **`==` (virado)**: `#set text X ≠ X` (a presença do wrapper, como o vanilla); os valores ignorados
  (`#set text(a) X == #set text(b) X`); as asserções de `== X` viram, **declaradas e justificadas**
  (S5b, listadas).
- Confirmar: o caso 2 (α) e a rede verdes; a fatia (1) (strong/emph) intacta; o caso 4, a flag e o
  Marco G intactos; os 3 numbering não regridem.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
Com os gates verdes (inclusive o α), **commitar**: `git add -A && git commit -m "Passo 372 — F-5b
fatia 2: de-bake do render #set text; fecha o F-5b e o item (1)"`. Árvore limpa e commitada. (Para na
Trava → commita só o L0; reverte por α/contradição → commita só o relatório.)

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): 2747, com as asserções de #set text X == X viradas para ≠
  (declaradas, justificadas vs vanilla, listadas). A rede (render) e o caso 2 (α) passam SEM alteração.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável; oráculo = vanilla 0.14.2):
  - #set text X ≠ X (presença do wrapper); #set text(a) X == #set text(b) X (valores ignorados).
  - render idêntico (paridade visual, rede de caracterização).
  - fonte única: o TextStyle assado do Content::Text REMOVIDO; só o custom carrega o render do #set text.

GATE DO α (duro): o caso 2 (α-fixpoint) + a rede +11 passam sem alteração. Se reabrir, REVERTER.

INTACTOS (confirmar): a fatia (1) (strong/emph variantes — morfologia); caso 4, flag P350c, Marco G;
  os 3 numbering (P364/P365); o #set de props de usuário (P368).

lente (instrumento): content→elements = 68 (registrar; não é gate; o de-bake pode reduzir se remover
  o caminho assado — registrar o delta).
perf (critério 4): antes = baseline da mesma sessão; depois reportado.
L0 (critério 5): f_fronteira_e1.md + hash sincronizado ANTES do código, Trava aprovada.
commit: árvore limpa e commitada ao fim.
```

---

## Válvula declarada

Se a varredura achar muitos produtores tipados/assados a migrar para o `custom` (não cabe num lote),
fatiar (ex.: o render no `custom` + leitura P372; a remoção do `TextStyle` assado P373). Registrar a
fatia e o número medido.

---

## O que NÃO fazer

- **Não deixar o render reabrir o caso 2** (gate duro do α — reverter se reabrir).
- **Não usar campo tipado de `Styled` para o render** (o obstáculo do P366 — o render vai pelo
  `custom`, transparente).
- **Não tocar a fatia (1)** (strong/emph variantes), o caso 4, a flag P350c, o Marco G.
- **Não remover campo antes da varredura** (Trava 1/6; S5b).
- **Não pular a Trava** (varredura + L0 + hash do dono antes de remover campo).
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no commit; a decisão é do dono.
- **Não importar a quarentena.**

---

## Relatório (`typst-passo-372-relatorio.md` + resumo no chat)

A **varredura dos produtores do `#set text`** com `file:line` (tipado/assado vs custom vs fixture,
medido vs inferido); o **caminho render→`custom`** desenhado; a confirmação da **transparência à
morfologia** (o render no `custom` ignorado por `is_semantically_empty`); o L0 com hash e a Trava
aprovada; a migração + a remoção do `TextStyle` assado com `file:line`; **a prova do α** (o caso 2 +
a rede +11 verdes — o número); a paridade visual de render; a **lista das asserções de `==` que
viraram** (`#set text X == X` → `≠`, justificadas vs vanilla); a confirmação de que a fatia (1), o
caso 4, a flag, o Marco G e os 3 numbering ficaram intactos; a medição da lente (delta); a perf; **o
commit de fecho** (hash); `git status` limpo; lint 0/0; o caveat de stack. **A nota de que o F-5b
FECHA com esta fatia, o item (1) da auditoria vai a 4/4, e o F fica completo pelos princípios sem
nenhuma divergência registrada** (o DEBT-61 fecha). Termina aqui — não emenda o seguinte.

## Estado do F após o P372 (a registrar)

Com o P372, o **F está completo pelos princípios, sem divergência registrada**: extensibilidade (os
dois públicos, P368/P369), atomização (os 4 caminhos duplos fechados — 3 numbering P364/P365 + o
`TextStyle` render agora), a distinção semântica strong/emph/text fiel ao vanilla (P371/P372). O
**DEBT-61 fecha**. Os débitos **não-F** permanecem: DEBT-59 (flag CLI), DEBT-60 (contador), e o
**Marco G** (decisão de modelo α/β, fora do F).

## Fora de escopo (confirmado)

A distinção de tipo da fatia (1) (feita, P371); o **Marco G** (não-F); DEBT-59 (flag CLI); DEBT-60
(contador); qualquer toque no caso 4, no Marco G ou na flag.
