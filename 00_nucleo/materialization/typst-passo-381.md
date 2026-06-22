# Passo 381 — atomização do layout: Fatia 2 (refs/avulsos) + Fatia Text (isolada) — mesmo passo

> **O que faz.** Fecha os **elementos de domínio** restantes do `layout_content` (790 linhas,
> pós-P380) em **duas fatias escritas no mesmo passo, em estágios e commits separados** (decisão do
> dono):
> - **Fatia 2 — refs/citações + avulsos**: Cite, Ref, Link, Bibliography, Footnote, Quote,
>   SmartQuote, Raw, Hide, Divider. Footnote e SmartQuote **mantêm o seu estado dedicado** (clusters
>   medidos no P379) **dentro da própria fatia**.
> - **Fatia Text — isolada**: o `Text` (`@635`, ~82 linhas) sozinho, **por ser grande, chain-pesado
>   e caminho quente** (P379) — merece verificação própria, com **commit próprio**.
> Forma **B** provada 33× (free function em `rules/layout/<elem>.rs`, módulo descendente). **Deixa
> FORA, por medição (P379):** a **máquina do layouter** (Sequence/Styled/Dynamic/SetPage — não é
> elemento, orquestra/reconfigura) e a **math** (fatia final, path próprio). **Após este passo, o
> `layout_content` fica só máquina + math** — o "monólito fechado" no sentido correto (não 0 arms).
> **Content-preserving**: a lógica move, não muda. `match` exaustivo, despacho estático, imports
> inalterados. **L0 commitado ANTES de mover** (mecanismo externo). **Cada fatia: Trava + commit
> próprio.** **Não emenda o seguinte** (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P381 (confirmar livre).
**Pré-condição**: P380 fechado (Fatia 1; `layout_content` 790; −1067 acumulado; 33 unidades). Suíte
verde, lint **0/0**, `content→elements` 68 (não-gate). HEAD pós-P380. Caveat de stack:
`RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: atomização (content-preserving, forma B). A **rede de caracterização (+11, P331)** é o
oráculo. A métrica é de **leitura**, não a lente. **Duas fatias, dois commits, um passo.**

---

## Fatia 2 — escopo (medido P379)

| Família | Arms | Nota |
|---|---|---|
| **Refs/citações** | Cite `@1028`, Ref `@905`, Link `@852`, Bibliography `@1001`, Footnote `@1020` | Cite lê `introspector`; **Footnote** tem estado dedicado (`footnote_counter`/`pending_footnote_bodies`) — fica na fatia |
| **Avulsos** | Quote `@1426`, SmartQuote `@1401`, Raw `@805`, Hide `@1199`, Divider `@1146` | **SmartQuote** estado dedicado (`smartquote_*_open`); Raw/SmartQuote leem `layout_word` |

~10 arms. Footnote/SmartQuote: o estado dedicado é acedido por **descendência de módulo** (como o
estado de fluxo), então a free function lê-o igual — **sem** precisar extrair o estado.

## Fatia Text — escopo (isolada)

`Text` `@635` (~82 linhas) → `rules/layout/text.rs::layout`. Renderizador folha (P379): decodifica o
render do `#set text` da chain (canais `custom`), merge top-wins com `self.style`, itera
`split_whitespace` → `layout_word`. **Não re-entra `layout_content`** (não orquestra). Caminho
quente — verificação e **commit próprios**.

---

## Fase A — confirmar e desenhar (a fonte vence; `file:line`)

1. **Reler** a ADR-0109 (forma B, não-metas), as **Travas anti-deriva**, e o P379 (os escopos, os
   clusters de estado dedicado, o Text folha).
2. **Confirmar, por arm** (ambas as fatias), `file:line`: a lógica e o que lê do `Layouter`. Para
   Footnote/SmartQuote, confirmar que o estado dedicado é acedido por descendência (não precisa
   extrair). Para o Text, confirmar que é folha (não re-entra `layout_content`) — se re-entrar, é
   máquina, **parar e reportar**.
3. **Confirmar as não-metas** (ADR-0109): `match` exaustivo; despacho estático; `entities/` não
   tocado. Se um arm exigir `dyn` ou tocar `entities/`, **parar e reportar**.
4. **Confirmar o que fica FORA**: máquina (Sequence/Styled/Dynamic/SetPage) e math — **não** tocar.

---

## Limites duros (ADR-0109)

- **`match` exaustivo MANTIDO**; **despacho ESTÁTICO** (sem `dyn`); **`entities/` não tocado**.
- **Forma B** — free function, módulo descendente. **Não a Opção A.**
- **Não tocar a máquina (Sequence/Styled/Dynamic/SetPage) nem a math** — fora deste passo.
- **Text é folha** — se a medição mostrar que re-entra `layout_content` (orquestra), é máquina,
  **parar e reportar** (não forçar).
- **Content-preserving** — a rede passa **sem alteração**; se virar, investigar, não mascarar.
- **Não tocar** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag, o F-5b.

---

## Estágios

### Estágio L0 — desenho + Trava + **commit do L0** (PARA aqui para o dono)
Registrar no L0 (`rules/atomizacao_elementos.md`) ambas as fatias (2 + Text), a forma B, as
não-metas, e o inventário do que resta após o passo (só máquina + math). Sincronizar hashes.
**Commitar o L0 agora** (mecanismo externo). **TRAVA**: para no chat — a medição + o desenho + os
hashes para o dono aprovar. Nenhum arm movido antes.

### Estágio 1 — Fatia 2 (após aprovação) → **commit próprio**
Mover os ~10 arms de refs/avulsos para `rules/layout/<elem>.rs` (forma B); arms magros; imports
mortos removidos. Footnote/SmartQuote com o estado dedicado por descendência. Rodar os gates
(suíte/rede/lint). **Commit:** `git commit -m "Passo 381 — atomização Fatia 2: refs/citações +
avulsos"`.

### Estágio 2 — Fatia Text (após a Fatia 2 verde) → **commit próprio**
Mover o `Text` para `rules/layout/text.rs` (forma B, folha). Rodar os gates — **com atenção ao
caminho quente** (a rede de caracterização de estilo `f_caracterizacao_estilo::*` é o oráculo
crítico aqui, pois o Text decodifica o render da chain). **Commit:** `git commit -m "Passo 381 —
atomização Text (folha de render, isolada)"`.

### Estágio Teste (cada fatia)
- A **rede de caracterização (+11)** e a suíte passam **sem alteração** (content-preserving).
- **Leitura**: o `layout_content` encolhe (registrar por fatia e o acumulado; e que após o passo
  resta **só máquina + math**).
- Confirmar: `match` exaustivo, despacho estático, `entities/` intacto; a máquina/math não tocadas;
  o α/caso 2/caso 4/flag/F-5b intactos.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): inalterada em número (content-preserving) — rede +11 sem asserção
  virada, em AMBAS as fatias. Atenção redobrada no Text (caminho quente, chain-pesado).
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (leitura — ADR-0109):
  - Fatia 2: os ~10 refs/avulsos legíveis nos seus arquivos; layout_content −X.
  - Text: text.rs legível sozinho; layout_content −~82.
  - acumulado desde 1857; após o passo resta SÓ máquina (Sequence/Styled/Dynamic/SetPage) + math.
  - content-preserving: comportamento idêntico (rede de caracterização).

NÃO-METAS: match exaustivo (0 wildcards); despacho estático (0 dyn); entities/ intacto
  (content→elements = 68 não-gate); forma B.

FORA (não tocados): Sequence/Styled/Dynamic/SetPage (máquina), math.
INTACTOS: α/caso 2, morph ==/morph_canon, caso 4, flag P350c, F-5b; os 3 numbering; o #set de props.
lente (instrumento): content→elements = 68 inalterado.
perf: free function inlinável; medir (em especial o Text, caminho quente).
L0: atomizacao_elementos.md + hash COMMITADO ANTES de mover, Trava aprovada.
commit: L0 (Estágio L0) + Fatia 2 (Estágio 1) + Text (Estágio 2) — três commits; árvore limpa.
```

---

## Válvula declarada

Se a Fatia 2 (~10 arms) não couber, reduzir (refs num, avulsos noutro) — cada arm independente,
exaustividade intacta. O **Text é sempre commit próprio** (isolamento pedido). Se a medição do Text
mostrar que ele re-entra `layout_content` (é máquina, não folha), **deixá-lo fora** e reportar (vira
parte da máquina, não atomizado).

---

## O que NÃO fazer

- **Não usar a forma A** nem `dyn`/wildcard (ADR-0109).
- **Não tocar `entities/`** nem reduzir `content→elements`.
- **Não tocar a máquina (Sequence/Styled/Dynamic/SetPage) nem a math.**
- **Não forçar o Text se ele for máquina** (re-entrar `layout_content`) — reportar.
- **Não mudar comportamento ao mover** — content-preserving; se a rede virar, investigar.
- **Não tocar** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag, o F-5b.
- **Não deixar L0/doc não-commitado** entre estágios (mecanismo externo).
- **Não pular a Trava**; **não emendar o passo seguinte** (Trava 5).
- **Não importar a quarentena.**

---

## Relatório (`typst-passo-381-relatorio.md` + resumo no chat)

A **confirmação por arm** das duas fatias (`file:line`, o que lê; Footnote/SmartQuote estado
dedicado; o Text folha confirmada); o L0 commitado (hash) e a Trava aprovada; a **Fatia 2** movida
(`file:line` + arms magros + commit) e a **Fatia Text** movida (`file:line` + commit próprio); a
paridade (rede +11 sem alteração, com atenção ao Text); a **métrica de leitura** (o `layout_content`
−X por fatia; o acumulado desde 1857; a confirmação de que resta **só máquina + math**); as
não-metas; a prova de que a máquina/math ficaram fora e o α/caso 2/caso 4/flag/F-5b intactos; a
lente (= 68, não-gate); a perf (em especial o Text); **os três commits** (L0 + Fatia 2 + Text);
`git status` limpo; lint 0/0; o caveat de stack. **A nota de que os elementos de domínio do layout
estão atomizados; resta a fatia math (final) e depois o `introspect.rs`.** Termina aqui — não emenda
o seguinte.

## Fora de escopo (confirmado)

A **máquina do layouter** (Sequence/Styled/Dynamic/SetPage — fica, não é elemento); a fatia **math**
(final, path próprio); os **displays counter/state** ([a-decidir], fronteira); a atomização do
**`introspect.rs`** (após o layout fechar); varredura/crates (depois); Marco G/desacoplamento
(descartado); DEBT-59; DEBT-60.
