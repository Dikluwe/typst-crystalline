# Passo 362 — Auditoria de retorno ao F: o que está feito, o que falta para completá-lo

> **O que faz.** **Read-only.** Audita o **escopo proposto do F** contra o repo, item por item,
> para descobrir o que está **feito / adiado / não-construído**, e o que falta para **completar o
> F segundo os princípios do projeto** — **atomização**, **fonte única de verdade**, e os **dois
> públicos** —, com a **lente como instrumento de medição, não como o critério do objetivo**. A
> fonte do "que o F propôs" é a **decisão P332/P333 + `f-plano-lotes-passo-333.md` + o L0
> `f_fronteira_e1.md`**, lida no repo — **não da memória** (que se provou não-confiável neste arco).
> **NÃO conclui "F fechou" nem "F não fechou" a priori** — mede e reporta com `file:line`. **NÃO
> reintroduz o Marco G como parte do F**: `content→elements→0` era expectativa **órfã da lente**,
> não compromisso do F (P346). **Termina no relatório e PARA — não emenda nem inicia o passo
> seguinte.** Saída: `00_nucleo/diagnosticos/f-auditoria-retorno-f-passo-362.md`.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P362 (confirmar livre; o "P361" foi um recon do Marco G que o executor
emendou por conta própria — read-only, agora **superseded** por esta auditoria).
**Pré-condição**: F-realização fechada (casos 1–4: F-340/352/356/358 + a linha da recursão);
DEBT-60 (b) feito (P359). HEAD pós-P359, suíte **2737**, lint **0/0**, árvore limpa. Lente
`tekt-cargo-dsm` disponível (registrar versão/commit). Caveat de stack: `RUST_MIN_STACK=33554432`.
Se algo não bater, parar e reportar.
**Tipo**: **auditoria/recon read-only** — zero código de produto, zero L0. Probes descartáveis
(compiladas e **revertidas**; árvore limpa; suíte não re-rodada). Termina num relatório.

---

## Dois limites duros de processo (explícitos)

1. **Termina no relatório.** **NÃO** emendar, propor-e-executar, nem iniciar o passo seguinte. O
   que fazer com o que a auditoria achar é **decisão do dono**, no próximo turno. (Correção: passos
   recentes foram emendados sem a decisão do dono — P360→P361.)
2. **O Marco G não é F.** `content→elements→0` foi um alvo da **lente** adotado como marco no P346
   (Saída 2) — **fora** da fila F. A auditoria **não** o conta como compromisso do F. Mede
   `content→elements` como **fato da lente**, não como critério do F.

---

## O critério: os princípios definem, a lente mede

A auditoria julga o F pelos **princípios do projeto**, não pela lente:

- **Atomização** — elementos atômicos e independentes. **Instrumento**: a lente mede
  `edges(elemento→elemento)`; o alvo do princípio é **0**.
- **Fonte única de verdade** — não há representação **dupla** do mesmo dado. O canal único das
  `Set*` (F-D) e o de-bake (F-5) existem para isto: a chain é a fonte; o campo assado **some**.
  Um caminho duplo vivo (campo assado **e** chain carregando o mesmo dado) é **atomização
  incompleta** — mede-se pelo princípio, **não** pela demanda da lente.
- **Os dois públicos** — o caminho **Rust** (implementa trait, registra) **e** o caminho **typst**
  (o que um usuário faz puramente em `.typ`, sem Rust). O F prometeu os dois.

A **lente é um instrumento** entre outros (grep, leitura, probes). Ela mede acoplamento e
atomização; **não** define o objetivo. `content→elements` é um número que a lente reporta — a
auditoria o registra, mas **não** o trata como gate do F.

---

## Leituras da Fase A (a fonte do escopo proposto; `file:line`)

1. **A decisão do F**: P332 (D agora → B como F-2; E1 escolhida) + P333 Parte 1 (a decisão da
   fronteira) + a **ADR da fronteira E1** (numerar — provável ADR-0106). O que o F **prometeu
   entregar**.
2. **O plano**: `f-plano-lotes-passo-333.md` — os 6 lotes, a ordem (P337: F-4 → F-realização →
   F-5 → F-6), e os **critérios transversais** (aditivo/content-preserving, trava ADR-0105 cl.3,
   L0 primeiro, lente por lote, perf por lote).
3. **O L0**: `entities/f_fronteira_e1.md` — o desenho completo: F-D (canal único), **F-B (a chain
   com os 10 campos nativos fechados + o mapa aberto + o `Value`)**, a fronteira E1, **os dois
   públicos**, a trava ADR-0105 cl.3, o contrato de verificação 3c (C1–C8 + S* + a rede de
   caracterização).
4. **Os princípios**: P329 (fidelidade comportamental — o norte), ADR-0107 (paridade com a
   linguagem), ADR-0105 (a trava), e a definição de atomização do projeto.

---

## O que auditar (cada compromisso do F: feito / adiado / não-construído, com `file:line`)

### A — A fila de lotes (6)
- **F-1** (fronteira E1: `Content::Dynamic` + registro + trait público) — estado?
- **F-2** (canal único das `Set*`, o F-D) — estado? Os 4 canais viraram um?
- **F-3 / F-realização** (`#show`, casos 1–4) — estado?
- **F-4** (`Styled`, colapso da dualidade de backing) — estado?
- **F-5** (de-bake dos 4 pontos assados) — estado? (esperado: **adiado**; medir o que falta e os
  caminhos duplos que ele deixaria — ver E).
- **F-6** (3 folhas, DEBT-58: `Text`/`MathText`/`MathIdent` via chain) — estado? (esperado:
  **não-feito**.)

### B — O lado estilo (F-B) — o que NUNCA foi verificado
- **O mapa aberto** (`PropKey → Value`) para propriedades de **elementos de usuário** — foi
  construído? testado? Ou existem **só** os 10 campos nativos fechados, e props de usuário não
  têm canal?
- **O `enum Value`** fechado espelhando a linguagem — completo? E o **escape `Value::Custom`** — a
  decisão era item da Trava (entra/não entra, com a razão): **entrou ou não, e com que evidência?**

### C — Os dois públicos
- **Caminho Rust** (implementa o trait, registra) — feito? O `callout` (fixture, fora dos 65)
  prova-o?
- **Caminho typst** (o que um usuário faz **puramente em `.typ`** — definir/compor/`#show` sem
  Rust) — **feito? testado?** Ou só o caminho Rust existe e o público typst ficou por construir?

### D — As travas e o contrato
- **A trava ADR-0105 cl.3** (teste-varre-tabela ou regra de lint antes de relaxar o compilador no
  caminho dinâmico) — **presente e ativa?**
- **O contrato de verificação 3c** (C1–C8 + os S* do spike-2 + a rede de caracterização) — em
  vigor? Quais S* foram satisfeitos?

### E — Os princípios, medidos
- **Atomização**: `edges(elemento→elemento)` (lente). Confirmar **= 0**.
- **Fonte única de verdade**: listar os **caminhos duplos vivos** (mesmo dado em campo assado **e**
  na chain), com `file:line` — os 4 pontos do F-5 são os candidatos. Cada um é atomização
  **incompleta**, medida pelo princípio (não pela demanda). Isto é o que o F-5/F-6 fechariam.
- **Acoplamento**: `edges(content→elements)` (lente) — registrar como **fato da lente** (66), e
  **explicitamente NÃO** como gate do F (Marco G fora de escopo). Anotar se o alvo `0` foi
  reconciliado (P346 Saída escolhida) ou continua órfão.

---

## A saída — a tabela e o veredito (medido, não a priori)

O recon entrega:
- A **tabela** do escopo proposto do F × o estado real (feito / adiado / não-construído), com
  `file:line`, para cada item de A–D.
- A lista dos **caminhos duplos vivos** (atomização incompleta) de E.
- O **que falta para o F estar completo segundo os princípios** — não segundo a lente. (Ex.: se
  F-5/F-6 estão adiados e deixam caminhos duplos, a atomização não está completa; se o mapa aberto
  ou o público typst não foram construídos, a extensibilidade prometida não está completa.)
- A **separação explícita**: o que **é** F (a fila, o F-B, os dois públicos, a trava) vs o que
  **não é** F (o Marco G / `content→elements→0`, órfão da lente).
- A **recomendação marcada** (a decisão é do dono).

---

## O que NÃO fazer

- **Não escrever código de produto, não editar L0.** Probes descartáveis para medir são
  permitidas (compiladas e revertidas; árvore limpa).
- **Não concluir "F fechou" nem "F não fechou" a priori** — medir cada item.
- **Não usar a lente como critério do objetivo.** Ela é instrumento; os princípios são o critério.
- **Não reintroduzir o Marco G como parte do F.** Ele é fora de escopo; `content→elements` é fato
  da lente, não gate do F.
- **Não emendar nem iniciar o passo seguinte.** Termina no relatório; a decisão é do dono.
- **Não importar a quarentena.** `lab/` é leitura de semântica/doc, nunca import.

---

## Relatório (`f-auditoria-retorno-f-passo-362.md` + resumo no chat)

A tabela escopo-proposto × estado-real (A–D, `file:line`); os caminhos duplos vivos (E, atomização
incompleta); o estado do F-B (mapa aberto, `Value`, escape `Custom`), dos dois públicos (Rust E
typst), e da trava ADR-0105 cl.3; os princípios medidos (atomização `elem→elem`, fonte única, o
fato da lente `content→elements`); o que **falta para completar o F segundo os princípios**; a
separação F vs não-F (Marco G fora); a recomendação marcada. A versão/commit da lente. Suíte não
re-rodada (read-only); árvore limpa; lint inalterado; caveat de stack nas probes.

## Fora de escopo (confirmado)

O **Marco G** (`content→elements→0` — não é F; órfão da lente, P346); a **execução** de qualquer
item que falte (nasce da decisão do dono, no próximo turno); qualquer toque em código de produto
ou L0; qualquer mudança no α / `morph_canon` / `==` ou na flag.
