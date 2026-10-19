# Passo 365 — F-5a (cont.): de-bake da figura — `figure.numbering` fonte única na chain

> **O que faz.** Fecha o de-bake dos numbering começando pela **figura** — o 3º dos 4 caminhos
> duplos (heading e equation feitos no P364). O campo assado **`FigureElem.numbering`** sai; o
> consumidor lê o padrão de numeração **só da chain** (`chain.custom("figure.numbering")`, sobre o
> introspect-chain do P363) → **fonte única** (atomização). Justificativa = **princípio (fonte
> única)**, decidido pelo dono (P362), não demanda da lente. A figura tem **mais superfície** que
> heading/equation, medida no P364: o gate é **`Option<String>`** (o **padrão**, não um bool) e há
> a **assinatura partilhada `figure_numbering`** em ~10 funções de `stdlib/structural.rs`. Por isso
> a Fase A **começa varrendo** essa assinatura (produção vs fixture; some ou muda de fonte) **antes
> de remover campo** (Trava 1/6; S5b). **Content-preserving**: output idêntico (paridade pela rede
> de caracterização). **Não toca o número/contador** (só o gate/padrão), o α/caso 2, o caso 4, o
> `morph_canon`/`==`, a flag P350c nem o Marco G. **Design-first**: L0 + Trava antes de remover
> campo. **Commita ao terminar** e **não emenda o passo seguinte** (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P365 (confirmar livre).
**Pré-condição**: P364 fechado (heading + equation de-bakados, fonte única; o bloco P362–P364
**commitado** conforme o dono mandou). Suíte **2738**, lint **0/0**, lente **66/0**, árvore limpa.
HEAD pós-P364. Lente `tekt-cargo-dsm` disponível (registrar versão/commit). Caveat de stack:
`RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: F-5a de-bake (figura) — **content-preserving** (output idêntico; remove a representação
dupla). A **rede de caracterização (+11, P331)** é o oráculo. **Justificativa = princípio (fonte
única / atomização)**, não demanda da lente (não re-litigar). Onde uma **fixture** lia o padrão fora
do transporte, a asserção **vira** para refletir a leitura da chain — **declarada e justificada**
(S5b), nunca silenciosa.

---

## Fase A — a varredura vem PRIMEIRO (a fonte vence; `file:line`)

**Antes de remover qualquer campo:**

1. **Reler** ADR-0107, ADR-0108, as **Travas anti-deriva**, e o princípio da fonte única.
2. **A assinatura partilhada `figure_numbering`** (a superfície extra, medida no P364): varrer as
   ~10 funções de `stdlib/structural.rs` (e onde mais aparecer) que threadam `figure_numbering`,
   com `file:line`, e classificar cada uma:
   - **[produção, transporte]** — recebe o padrão via o `Content::Styled` da fatia-1 (o
     `#set figure(numbering:)` → custom na chain). O P364 mediu o produtor de figura
     (`eval/closures.rs:79-83`) como **[produção, transporte]** — confirmar que isso vale para toda
     a cadeia da assinatura.
   - **[produção, direto]** — passa o padrão sem o transporte → o de-bake o roteia pela chain (mais
     superfície, declarada).
   - **[fixture, direto]** — a asserção vira (declarada, justificada).
   - **Decidir, medido**: a assinatura `figure_numbering` **some** (o padrão vem da chain em todos
     os pontos) ou **muda de fonte** (alguns pontos ainda a threadam)? A resposta decide o tamanho.
3. **O gate como padrão (`Option<String>`), não bool** — confirmar `file:line`: o que a chain custom
   de figura carrega (o padrão `Some("1")` vs `None`), e que o consumidor o lê dali. Diferente do
   bool de heading/equation — registrar a forma.
4. **A infra do P363** (introspect-chain) + o L0 `f_fronteira_e1.md §3a.9` (a fatia F-5a) e o
   `introspect.md`. Sincronizar hashes antes do código.
5. **A separação gate(padrão)-vs-número** (P335/DEBT-60): confirmar que o de-bake troca **só a
   leitura do padrão** (o gate); o **número** (`formatted_counter_at`) e o contador **não** são
   tocados. Se a varredura mostrar que tocam, **parar e reportar** (re-escopo).

---

## A forma do de-bake — decidida pela varredura (não antes dela)

- **Ponto [produção, transporte]** → o consumidor lê `chain.custom("figure.numbering")`; o campo
  assado sai; zero roteamento.
- **Ponto [produção, direto]** → roteado pela chain (declarado).
- **Ponto [fixture, direto]** → asserção virada (declarada, justificada vs vanilla/rede).
- A **assinatura `figure_numbering`** some onde a chain a substitui; onde permanecer por outra
  razão (não-gate), registrar por quê.
- Caso que não cabe em nenhum → **parar e reportar** (re-escopo).

---

## Limites duros

- **Não remover campo antes da varredura** (Trava 1/6; S5b).
- **Não tocar o número/contador.** O de-bake lê o **padrão** (gate); o número e o contador ficam
  intactos.
- **Não de-bakar o `TextStyle`** (ponto 4) — é o **F-5b**.
- **Não tocar o α / caso 2, o caso 4, o `morph_canon`/`==`, a flag P350c nem o Marco G.**
- **Não re-litigar a demanda** (o dono decidiu pelo princípio).
- **Content-preserving.** Output idêntico via chain (paridade, rede). As únicas asserções que mudam
  são as de fixtures que liam o padrão fora do transporte — declaradas.

---

## Estágios

### Estágio L0 — desenho (com a varredura na mão) + Trava (PARA aqui para o dono)
Com a classificação dos pontos da assinatura partilhada, desenhar o de-bake (quais roteados, quais
asserções viram, se `figure_numbering` some). Registrar no L0 `f_fronteira_e1.md §3a.9` (a figura na
fatia F-5a). Sincronizar hashes. **TRAVA**: para no chat — varredura + desenho + hashes para o dono
aprovar. Nenhum campo removido antes.

### Estágio 1 — rotear os pontos [produção, direto] (se houver; após aprovação)
Os pontos de produção que passam o padrão sem o transporte passam a dar o padrão à chain. Aditivo.

### Estágio 2 — remover `FigureElem.numbering` + colapsar a assinatura
Remover o campo assado `numbering` de `FigureElem`; o consumidor lê o padrão da chain; a assinatura
`figure_numbering` some onde a chain a substitui (per a varredura). **Fonte única.**

### Estágio Teste
- A **rede de caracterização (+11)** passa — spec de paridade. As asserções que mudam são **só** as
  de fixtures que liam o padrão fora do transporte; cada uma justificada.
- **Novo / virado**: a figura lê o padrão **só da chain** (campo removido); output **idêntico**
  (paridade).
- Confirmar que o número (contador) e o P335 ficaram **intactos**; e que o α / caso 2, o caso 4, o
  `morph_canon`/`==` e a flag não foram tocados.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão de todo passo daqui em diante)
Com os gates verdes e o trabalho completo, **commitar** o resultado: `git add -A && git commit -m
"Passo 365 — de-bake da figura (fonte única)"`. A árvore fica **limpa e commitada** — nenhum
trabalho acumulado não-commitado entre passos.
- Passo que **para na Trava** (sem código): commita só o L0/recon (se houver) e para.
- Passo que **reverte por contradição** medida (S5b/ADR-0108): **não** commita código — commita só
  o relatório do achado.
- **Read-only/recon**: commita o documento de saída.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): 2738, paridade pela rede. Asserções alteradas = SÓ as de fixtures
  que liam o padrão fora do transporte (declaradas, justificadas, listadas). Nenhuma outra muda.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável; oráculo = rede de caracterização / vanilla):
  - figura: output idêntico ao de antes (paridade), lendo o padrão SÓ da chain.
  - fonte única: o campo assado FigureElem.numbering REMOVIDO; só a chain carrega o padrão.

INTACTOS (confirmar): o NÚMERO (formatted_counter_at) e o contador P335; α / caso 2, caso 4,
  morph ==/morph_canon (P345), flag P350c, Marco G; heading + equation (P364) — não regridem.

lente (instrumento): content→elements = 66 (registrar; não é gate do F).
perf (critério 4): antes = baseline da mesma sessão (re-medir no início); depois reportado.
L0 (critério 5): f_fronteira_e1.md §3a.9 + hash sincronizado ANTES do código, Trava aprovada.
commit: árvore limpa e commitada ao fim (estágio de fecho).
```

---

## Válvula declarada

Se a varredura achar **muitos** pontos [produção, direto] a rotear, ou a assinatura partilhada não
puder sumir num lote, fatiar (ex.: roteamento P365 / remoção P366). Registrar a fatia e o número
medido. O ponto 4 (`TextStyle`) já é o F-5b.

---

## O que NÃO fazer

- **Não remover campo antes da varredura** (Trava 1/6; S5b).
- **Não tocar o número/contador/P335** (só o padrão/gate).
- **Não de-bakar o `TextStyle`** (F-5b).
- **Não tocar o α / caso 2, o caso 4, o `morph_canon`/`==`, a flag P350c nem o Marco G.**
- **Não re-litigar a demanda** (o dono decidiu pelo princípio).
- **Não pular a Trava** (varredura + L0 + hash do dono antes de remover campo).
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no commit; a decisão é do dono.
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Relatório (`typst-passo-365-relatorio.md` + resumo no chat)

A **varredura da assinatura `figure_numbering`** com `file:line` e a classificação (transporte /
direto × produção / fixture, marcada medido vs inferido); a forma do gate (padrão `Option<String>`);
o L0 (`f_fronteira_e1.md §3a.9`) com hash e a Trava aprovada; o roteamento (se houve) e a remoção do
campo + o colapso da assinatura, com `file:line`; a paridade pela rede e a **lista** das asserções
de fixture que viraram (com justificativa); a prova de que o número/P335, o α/caso 2, o caso 4, o
morph `==`, a flag, e heading/equation (P364) ficaram intactos; a medição da lente; a perf; **o
commit de fecho** (hash do commit); `git status` limpo; lint 0/0; o caveat de stack. **Termina aqui
— não emenda o passo seguinte.**

## Fora de escopo (confirmado)

O **ponto 4** (`Content::Text` `TextStyle`) — **F-5b** (arrasta o bold do heading); o **F-6** (item
2 da auditoria); o **mapa aberto para `#set` de props de usuário** (item 3); o **Marco G** (não é
F); DEBT-59 (flag CLI); DEBT-60 contador; qualquer toque no α / `morph_canon` / `==` ou na flag.
