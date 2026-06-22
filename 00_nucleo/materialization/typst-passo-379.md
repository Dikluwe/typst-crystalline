# Passo 379 — medir os arms restantes: tamanho de fatia + classificar o core/infra (recon read-only)

> **O que faz.** **Read-only.** Mede o que resta do `layout_content` (929 linhas após o P378) para
> o dono decidir, **com valores**, duas coisas que hoje são palpite:
> 1. **O tamanho de fatia** dos não-math diretos — família-pequena (várias rodadas) vs lote-grande
>    (uma rodada). A medição que decide: quanto **acoplamento entre arms** existe — quais arms leem
>    o **mesmo estado privado do `Layouter`** (se compartilham, movê-los em fatias separadas duplica
>    o acesso/risco; se são independentes, fatiar é barato), e o tamanho por família.
> 2. **A classificação do core/infra** (Text 82, Sequence 51, Styled 14, Dynamic 31, SetPage 25,
>    state/counter) — cada um é **atomizável como elemento** (lógica de domínio separável, forma B),
>    ou é **máquina do layouter** (controle de fluxo entrelaçado: o `Sequence` itera, o `Styled`
>    empurra estilo na chain, o `Dynamic` despacha pelo trait) que **não deve** virar arquivo flat?
>    A ADR-0109 atomiza elementos; mover máquina do layouter para `rules/layout/<nome>.rs` pode ser
>    cortar no meio de algo que deveria ficar inteiro.
>
> **Não move código, não decide, não recomenda fatia** — dá os valores. **Zero código de produto,
> zero L0.** Saída: `00_nucleo/diagnosticos/arms-restantes-medidos-passo-379.md`. **Termina no
> relatório** (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P379 (confirmar livre).
**Pré-condição**: P378 fechado (15 unidades atomizadas; `layout_content` 929 linhas; −928 acumulado).
Suíte verde, lint **0/0**. HEAD pós-P378. Caveat de stack: `RUST_MIN_STACK=33554432`. Se algo não
bater, parar e reportar.
**Tipo**: medição/recon read-only — leitura + grep (nenhuma alteração). Termina no relatório com os
valores. **Não desenha fatia nem decide** — o dono escolhe com os valores na mão.

---

## O que medir (a fonte vence; `file:line`)

### Parte 1 — os não-math diretos: acoplamento entre arms + tamanho (decide o tamanho de fatia)
1. **Inventário com linhas**, `file:line`: os não-math restantes (do P378: Quote 46, Cite 35,
   Colbreak 33, TermItem 25, SmartQuote 25, Repeat 25, Pagebreak 25, VSpace 23, Divider 19,
   Bibliography 19, Table/TableCell/TableFooter, EnumItem/ListItem/Terms, Raw, Hide, Footnote, Ref,
   Link, família Grid, HSpace — **confirmar e completar no HEAD**).
2. **O acoplamento entre arms — a medição que decide o tamanho de fatia**: para cada arm, **o que
   ele lê do estado privado do `Layouter`** (campos, métodos, tipos auxiliares como
   `floats_pending`/`DeferredFloat`/`figure_progress`). Agrupar: arms que compartilham o mesmo
   estado/helper são uma **família natural** (movê-los juntos é coeso; separá-los duplicaria o
   acesso por descendência). Arms independentes podem ir em qualquer fatia.
3. **Famílias coerentes** (medidas, não supostas): breaks (Colbreak/Pagebreak), spacing (VSpace/
   HSpace/Repeat), listas (EnumItem/ListItem/Terms/TermItem), tabelas (Table/TableCell/TableFooter),
   refs (Cite/Ref/Link/Bibliography/Footnote), Grid, e os avulsos (Quote/SmartQuote/Raw/Hide/Divider).
   Para cada família: total de linhas, acoplamento interno, e se cabe numa fatia.
4. **O trade tamanho-vs-segurança, em valores**: família-pequena = N lotes pequenos (cada um fácil
   de verificar; mais Travas); lote-grande = 1-2 lotes (mais rápido; mais superfície por lote, mais
   arms a verificar de uma vez). **Dar os números** (quantos arms/linhas por opção) para o dono
   escolher — não recomendar.

### Parte 2 — o core/infra: classificar cada um (atomizável vs máquina do layouter)
5. Para **cada** de Text (82), Sequence (51), Styled (14), Dynamic (31), SetPage (25), state/counter,
   medir com `file:line`:
   - **o que a lógica de layout dele faz** — é lógica de **domínio** (como renderizar um Text: fonte,
     glyphs — separável como um elemento), ou é **controle de fluxo do layouter** (o `Sequence`
     itera os filhos e chama o layout de cada um; o `Styled` empurra/restaura estilo na chain; o
     `Dynamic` faz downcast e despacha pelo trait)?
   - **se mover quebra a coesão**: a lógica depende de orquestrar outros arms (re-entra no
     `layout_content`)? Se sim, é máquina, não elemento — mover para arquivo flat **cortaria** a
     orquestração.
   - **veredito por item**: [atomizável-como-elemento] (forma B serve) / [máquina-do-layouter] (fica
     onde está, legitimamente — não é elemento de domínio) / [a-decidir] (ambíguo, precisa do dono).
   - marcar [medido]/[inferido].

---

## A saída — os valores para a decisão

O recon entrega:
- **Parte 1**: a tabela dos não-math (arm × linhas × estado-que-lê), as famílias naturais (por
  acoplamento medido), e o trade tamanho-vs-segurança em números (X lotes pequenos vs Y lotes
  grandes, com os arms/linhas de cada).
- **Parte 2**: a classificação do core/infra item a item (atomizável / máquina / a-decidir), com o
  porquê e `file:line`.
- **A leitura honesta**: qual tamanho de fatia os números sugerem (sem decidir), e quais do
  core/infra são atomização legítima vs quais ficariam de fora por serem máquina do layouter (não
  por preguiça — por não serem elementos de domínio).

---

## Limites duros

- **Não mover código, não editar L0** (só o documento de diagnóstico).
- **Não decidir** o tamanho de fatia nem o destino do core/infra — dar os valores; o dono escolhe.
- **Não classificar o core/infra de memória** — medir o que cada lógica faz (a Trava 1; o
  `Sequence`/`Styled`/`Dynamic` parecem elementos mas podem ser máquina — medir).
- **A lente é instrumento** — `content→elements` não entra aqui (a atomização não o move).
- **Não importar a quarentena.**

---

## Verificação (gates)

```
read-only: nenhum código de produto, nenhum L0; leitura/grep; suíte não re-rodada.
saída: arms-restantes-medidos-passo-379.md — Parte 1 (não-math: acoplamento + famílias + trade
  tamanho em valores), Parte 2 (core/infra classificado), a leitura honesta. Marcado medido/inferido.
INTACTOS: nada tocado (sem código).
```

---

## O que NÃO fazer

- **Não recomendar uma fatia** nem decidir o tamanho — dar os números.
- **Não assumir que o core/infra é atomizável** (nem que não é) — classificar por medição.
- **Não confundir "parece elemento" com "é elemento de domínio"** — o `Sequence`/`Styled`/`Dynamic`
  podem ser máquina do layouter; medir a lógica.
- **Não escrever código nem L0.**
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no relatório; a decisão é do dono.

---

## Relatório (`arms-restantes-medidos-passo-379.md` + resumo no chat)

A **Parte 1** — a tabela dos não-math (arm × linhas × estado-que-lê do `Layouter`), as famílias
naturais por acoplamento medido, e o trade tamanho-vs-segurança em valores (N lotes pequenos vs Y
grandes); a **Parte 2** — o core/infra classificado item a item ([atomizável]/[máquina]/[a-decidir])
com `file:line` e o porquê; a **leitura honesta** (o que os números sugerem, sem decidir). Marcado
[medido]/[inferido]. Read-only; árvore limpa; lint inalterado; o caveat de stack. **Termina aqui —
não decide nem move; a escolha (tamanho de fatia + destino do core/infra) é do dono, com estes
valores.**

## Fora de escopo (confirmado)

A execução das fatias (após a decisão do dono); a fatia **math** (final, path próprio); a atomização
do `introspect.rs` (após o layout); a varredura/crates (depois); o **Marco G / desacoplamento**
(descartado); DEBT-59; DEBT-60.
