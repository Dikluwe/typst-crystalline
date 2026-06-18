# Passo 364 — F-5a (cont.): de-bake dos 3 numbering — campo assado removido, fonte única na chain

> **O que faz.** Completa o **F-5a** sobre a infra do P363 (introspect-chain): os 3 gates de
> numbering (`heading`/`equation`/`figure` `numbering_active`) **deixam de viver em campo assado**;
> o consumidor (layout + introspect) lê o gate **só da chain** → **fonte única** (atomização).
> Justificativa = **princípio (fonte única)**, decidido pelo dono (P362) — não demanda da lente. A
> Fase A **começa medindo** o risco que o P363 levantou: os **produtores de numbering que NÃO
> passam pelo transporte `Content::Styled`** (ex.: `Content::heading_numbered` direto, que não tem
> o custom na chain) — varrer e classificar (**produção vs fixture**) **antes de remover qualquer
> campo** (Trava 1/6; S5b: não descobrir removendo e vendo quebrar). **A forma do de-bake sai dessa
> medição**: ou estende o transporte aos produtores de produção, ou vira as asserções das fixtures
> (declaradas, justificadas). **Content-preserving**: output idêntico (paridade pela rede de
> caracterização). **Não toca o número/contador/P335** (só o gate), o α/caso 2, o caso 4, o
> `morph_canon`/`==`, a flag P350c nem o Marco G. **Design-first**: L0 + Trava antes de remover
> campo. **Termina no relatório; não emenda o passo seguinte** (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P364 (confirmar livre).
**Pré-condição**: P363 fechado — o introspect-chain construído (o walk threada a `StyleChain`,
empurrada em `Content::Styled`); o gate **disponível** na chain, **ainda não consumido** (os reads
seguem no campo assado). Suíte **2738**, lint **0/0**, lente **66/0**, árvore limpa. HEAD pós-P363.
Lente `tekt-cargo-dsm` disponível (registrar versão/commit). Caveat de stack:
`RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: F-5a de-bake — **content-preserving** (output idêntico; remove a representação dupla).
A **rede de caracterização (+11, P331)** é o oráculo de paridade. **Justificativa = princípio
(fonte única / atomização)**, não demanda da lente (a questão foi decidida pelo dono; não
re-litigar). Onde uma **fixture** não usava o transporte, a asserção **vira** para refletir a
leitura da chain — **declarada e justificada** (S5b), nunca silenciosa.

---

## Fase A — a medição vem PRIMEIRO (a fonte vence; `file:line`)

**Antes de remover qualquer campo:**

1. **Reler** ADR-0107, ADR-0108, as **Travas anti-deriva**, e o princípio da fonte única. O
   critério é o princípio; a lente é instrumento.
2. **A varredura dos produtores fora do transporte** — o risco que o P363 mediu, agora confirmado
   **antes** de remover. Listar **todos** os pontos que setam o gate de numbering dos 3 elementos
   (`heading`/`equation`/`figure`), com `file:line`, e classificar cada um:
   - passa pelo `Content::Styled` (o `#set …(numbering:)` via F-D → custom na chain) → o de-bake o
     lê da chain sem mais nada;
   - constrói numerado **direto** (ex.: `Content::heading_numbered`) **sem** o custom na chain → o
     gate sumiria ao remover o campo;
   - e para cada um do segundo grupo: **[produção]** ou **[fixture]** (marca medido vs inferido,
     Trava 8). **Esta varredura decide a forma do de-bake.** Não há de-bake antes dela.
3. **A infra do P363** (`introspect.md`, o introspect-chain — `chain.custom("X.numbering")`) + o L0
   `f_fronteira_e1.md` (a chain é a fonte por desenho). Sincronizar hashes antes do código.
4. **A rede de caracterização (+11)** como spec de paridade.
5. **A separação gate-vs-número** (P335/DEBT-60): confirmar que o de-bake lê o **gate**
   (`numbering_active`), **não** o número (`formatted_counter_at`) nem o contador incondicional
   (P335). Se a varredura mostrar que tocam, **parar e reportar** (re-escopo, não de-bake).

---

## A forma do de-bake — decidida pela varredura (não antes dela)

- **Produtor [produção] fora do transporte** → o de-bake o **roteia** pela chain (via
  `Content::Styled` ou a forma que a Fase A medir como mais limpa), para a chain ser a fonte. Mais
  superfície — **declarada** no L0.
- **Produtor [fixture] fora do transporte** → a asserção **vira** para refletir a leitura da chain
  (o gate vem da chain agora), **declarada e justificada** contra o vanilla/rede de caracterização.
  Não silenciosa (S5b).
- Se a varredura achar um caso que **não** cabe em nenhum dos dois (ex.: um produtor de produção
  que não pode passar pelo transporte sem quebrar outra coisa), **parar e reportar** — é re-escopo.

---

## Limites duros

- **Não remover campo antes da varredura.** A forma do de-bake sai da medição (Trava 1/6; S5b).
- **Não tocar o número/contador/P335.** O de-bake lê o **gate**; o número (`formatted_counter_at`)
  e o contador incondicional (P335) ficam intactos.
- **Não de-bakar o `TextStyle`** (ponto 4) — é o **F-5b** (arrasta o bold do heading).
- **Não tocar o α / caso 2, o caso 4, o `morph_canon`/`==`, a flag P350c nem o Marco G.**
- **Não re-litigar a demanda** (o dono decidiu pelo princípio).
- **Content-preserving.** Output idêntico via chain (paridade, rede de caracterização). As únicas
  asserções que mudam são as de fixtures que liam o gate fora do transporte — declaradas.

---

## Estágios

### Estágio L0 — desenho (com a varredura na mão) + Trava (PARA aqui para o dono)
Com a classificação dos produtores, desenhar o de-bake: quais produtores de produção são roteados
pela chain, e quais asserções de fixture viram (com a justificativa). Registrar no L0
`f_fronteira_e1.md` (a fonte única para os 3 numbering). Sincronizar hashes. **TRAVA**: para no
chat — a varredura + o desenho + os hashes para o dono aprovar. Nenhum campo removido antes.

### Estágio 1 — rotear os produtores de produção (após aprovação)
Os produtores de produção que não passavam pelo transporte passam a dar o gate à chain (per o
desenho). **Aditivo** — ainda não remove o campo; só garante que o gate chega à chain por todos os
caminhos de produção.

### Estágio 2 — remover os 3 campos assados
Remover `numbering_active` do payload/construtor de heading/equation/figure; o consumidor (layout +
introspect) lê o gate **da chain**. Cascata em construtores / `to_payload` / fixtures conforme a
varredura. **Fonte única** para os 3.

### Estágio Teste
- A **rede de caracterização (+11)** passa — é a spec de paridade. As asserções que mudam são **só**
  as de fixtures que liam o gate fora do transporte; cada uma justificada (a chain é a fonte agora).
- **Novo / virado**: para cada um dos 3 numbering, o gate vem **só da chain** (o campo removido), e
  o output é **idêntico** (paridade).
- Confirmar que o número (contador) e o P335 ficaram **intactos**; e que o α / caso 2, o caso 4, o
  `morph_canon`/`==` e a flag não foram tocados.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): 2738, paridade pela rede de caracterização. Asserções alteradas =
  SÓ as de fixtures que liam o gate fora do transporte (declaradas, justificadas, listadas). Nenhuma
  outra muda.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável; oráculo = rede de caracterização / vanilla):
  - cada um dos 3 numbering: output idêntico ao de antes (paridade), lendo o gate SÓ da chain.
  - fonte única: o campo assado numbering_active removido dos 3; só a chain carrega o gate.

INTACTOS (confirmar): o NÚMERO (formatted_counter_at) e o contador P335; α / caso 2, caso 4,
  morph ==/morph_canon (P345), flag P350c, Marco G.

lente (instrumento): content→elements = 66 (registrar; não é gate do F). Registrar qualquer delta
  de aresta do roteamento como medição.
perf (critério 4): antes = baseline da mesma sessão (re-medir no início); depois reportado.
L0 (critério 5): f_fronteira_e1.md (fonte única dos 3) + hash sincronizado ANTES do código, Trava
  cumprida (aprovação do dono registrada).
```

---

## Válvula declarada

Se a varredura achar **muitos** produtores de produção a rotear (de-bake + roteamento não cabem num
lote), fatiar — ex.: roteamento dos produtores (P364) / remoção dos campos (P365), ou por elemento.
Registrar a fatia e o número medido. O ponto 4 (`TextStyle`) já é o F-5b.

---

## O que NÃO fazer

- **Não remover campo antes da varredura** (Trava 1/6; S5b).
- **Não tocar o número/contador/P335** (só o gate).
- **Não de-bakar o `TextStyle`** (F-5b).
- **Não tocar o α / caso 2, o caso 4, o `morph_canon`/`==`, a flag P350c nem o Marco G.**
- **Não re-litigar a demanda** (o dono decidiu pelo princípio da fonte única).
- **Não pular a Trava** (varredura + L0 + hash do dono antes de remover campo).
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no relatório; a decisão é do dono.
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Relatório (`typst-passo-364-relatorio.md` + resumo no chat)

A **varredura dos produtores** com `file:line` e a classificação (transporte / fora-do-transporte ×
produção / fixture, marcada medido vs inferido); o L0 (`f_fronteira_e1.md`) com hash e a Trava
aprovada; o roteamento dos produtores de produção e a remoção dos 3 campos, com `file:line`; a
paridade pela rede de caracterização e a **lista** das asserções de fixture que viraram (com
justificativa); a prova de que o número/P335, o α/caso 2, o caso 4, o morph `==` e a flag ficaram
intactos; a medição da lente (instrumento); a perf; `git status` limpo por estágio fora de `lab/` e
docs; lint 0/0; o caveat de stack. **Termina aqui — não emenda o passo seguinte.**

## Fora de escopo (confirmado)

O **ponto 4** (`Content::Text` `TextStyle`) — **F-5b**; o **F-6** (item 2 da auditoria); o **mapa
aberto para `#set` de props de usuário** (item 3); o **Marco G** (não é F); DEBT-59 (flag CLI);
DEBT-60 contador; qualquer toque no α / `morph_canon` / `==` ou na flag.
