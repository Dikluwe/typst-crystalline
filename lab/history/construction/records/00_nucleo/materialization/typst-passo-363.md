# Passo 363 — F-5a: de-bake dos 3 numbering (fonte única) + introspect-chain

> **O que faz.** Começa o **F-5 (de-bake)**, movido pelo **princípio da fonte única de verdade
> (atomização)** — **não** pela demanda da lente (essa questão foi decidida pelo dono ao escolher
> o princípio; este lote não a re-litiga). Fecha **3 dos 4 caminhos duplos** que a auditoria P362
> mediu: `heading`/`equation`/`figure` numbering, hoje vivos **em campo assado** (`numbering_active`)
> **E** na chain — duas representações do mesmo dado. O de-bake **remove o campo assado**; o
> consumidor lê **só a chain**. Os 3 numbering exigem que o **introspect leia a chain** (hoje o walk
> **não tem `StyleChain`** — P353), então este lote constrói o **introspect-chain** como a infra que
> a fonte única demanda (não é demanda especulativa: o princípio pede o de-bake, o de-bake pede a
> infra). O **4º ponto** (`Content::Text` `TextStyle`) fica para o **F-5b** — ele arrasta o de-bake
> do bold do heading (P353), escopo separado. **Content-preserving**: o output fica **idêntico**
> (paridade pela rede de caracterização); o que muda é haver **uma** fonte, não duas. **Não toca o
> contador** (P335 incondicional — o de-bake lê o **gate** da chain, não o **número**), o α/caso 2,
> o caso 4, o `morph_canon`/`==`, a flag P350c nem o Marco G. **Design-first**: o L0 + a Trava antes
> do código.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P363 (confirmar livre).
**Pré-condição**: P362 (auditoria de retorno ao F) fechado — os 4 caminhos duplos medidos com
`file:line`; decisão do dono: **completar a atomização (fonte única) começando pelo item (1)**.
HEAD pós-P359, suíte **2737**, lint **0/0**, lente **66/0**, árvore limpa. Lente `tekt-cargo-dsm`
disponível (registrar versão/commit). Caveat de stack: `RUST_MIN_STACK=33554432`. Se algo não
bater, parar e reportar.
**Tipo**: F-5a de-bake — **content-preserving** (output idêntico; remove a representação dupla). A
**rede de caracterização (+11, P331)** é o oráculo de paridade. **Justificativa = princípio
(fonte única / atomização)**, não demanda da lente. O introspect-chain é **infra aditiva** (o walk
ganha a chain); o de-bake é **remoção** (o campo assado sai).

---

## Leituras da Fase A (a fonte vence; `file:line`)

1. **ADR-0107, ADR-0108, P329** e a **definição de atomização / fonte única** do projeto — o
   **princípio** é o critério. A lente é instrumento.
2. **A auditoria P362 + o recon P353**: os 3 numbering (`HeadingElem`/`EquationElem`/`FigureElem`
   `numbering_active`), o campo assado e os **consumidores** — o **layout** já lê a chain
   (`self.chain`); o **introspect** lê o **payload assado** (sem chain). Confirmar cada ponto com
   `file:line`.
3. **O L0** `entities/f_fronteira_e1.md` (o F-D / canal único — **a chain é a fonte por desenho**)
   + a **rede de caracterização (+11, P331)** como spec de paridade. Sincronizar hashes antes do
   código (critério 5).
4. **O caminho mais limpo do introspect-chain** — medir, não assumir a forma: onde o walk
   (`rules/introspect.rs`) lê o `numbering_active` hoje, e o que ele decide (contador/outline).
   Desenhar como o introspect passa a ler o gate de **uma fonte só** — threading da `StyleChain` no
   walk (a infra que o P353 apontou), **ou** um caminho mais barato se a Fase A o medir. **A forma
   sai da medição.**
5. **O contador (P335 incondicional)** — confirmar que o de-bake do **gate** (`numbering_active`)
   **não toca o número** (`formatted_counter_at`): são eixos distintos (a lição medida no
   DEBT-60/P359 — o gate e o número são coisas diferentes). Se a Fase A medir que tocam, **parar e
   reportar** (seria re-escopo, não de-bake).

---

## Limites duros

- **Não tocar o contador.** O de-bake lê o **gate** (`numbering_active`) da chain; o **número**
  (`formatted_counter_at`) e a decisão P335 (contador incondicional) ficam **intactos**.
- **Não de-bakar o `TextStyle`** (ponto 4) — é o **F-5b**, arrasta o de-bake do bold do heading
  (`markup.rs:85-86`); fora deste lote.
- **Não tocar o α / caso 2, o caso 4, o `morph_canon`/`==`, a flag P350c nem o Marco G**
  (`content→elements` = 66 é fato da lente, não gate do F; este lote não o move).
- **Não re-litigar a demanda.** A justificativa é o princípio (fonte única); o dono decidiu. O
  introspect-chain é a infra mínima que o de-bake exige — não infra especulativa para uso futuro.
- **Content-preserving.** O output via chain é **idêntico** ao output via campo assado (paridade,
  rede de caracterização). Discrepância = o de-bake está errado, ou o assado mascarava um bug — nos
  dois casos, **declarar**, não esconder (lição S5b).

---

## Estágios

### Estágio L0 — desenho + Trava (PARA aqui para o dono)
Editar, sincronizar hashes, e **parar** para a aprovação do dono antes de qualquer `.rs`:
1. **O introspect-chain** — o desenho que a Fase A mediu (como o walk passa a ler o gate de uma
   fonte só). Registrar no L0 `entities/introspect.md`.
2. **O de-bake dos 3 numbering** — `numbering_active` sai do payload assado; layout e introspect
   leem a chain. Registrar no L0 `f_fronteira_e1.md` (a fonte única para os 3 numbering).
**TRAVA**: o passo termina no chat — L0 + hashes para o dono aprovar. Nenhum código antes.

### Estágio 1 — introspect-chain (após aprovação)
O walk do introspect ganha a `StyleChain` (ou a forma medida na Fase A), de modo a ler o gate de
numbering da chain. **Aditivo** — não muda o que o introspect produz ainda (a fonte do gate continua
a mesma até o Estágio 2 remover o campo assado).

### Estágio 2 — de-bake dos 3 numbering
Remover `numbering_active` do payload assado de heading/equation/figure; o consumidor (layout +
introspect) lê o gate **da chain**. Fonte única para os 3.

### Estágio Teste
- A **rede de caracterização (+11)** passa sem alteração — é a spec de paridade. Cada asserção que
  mude é onde o de-bake corrige um bug que o duplo mascarava; justificar uma a uma.
- **Novo**: para cada um dos 3 numbering, o gate vem **só da chain** (o campo assado removido), e o
  output é **idêntico** ao de antes (paridade).
- Confirmar que o **número** (contador) e a decisão P335 ficaram **intactos**; e que o α / caso 2,
  o caso 4, o `morph_canon`/`==` e a flag não foram tocados.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): 2737, paridade pela rede de caracterização — nenhuma asserção
  alterada exceto onde o de-bake corrige um duplo que mascarava (declarada, justificada).
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável; oráculo = rede de caracterização / vanilla):
  - cada um dos 3 numbering: output idêntico ao de antes (paridade) lendo o gate SÓ da chain.
  - fonte única: o campo assado numbering_active removido dos 3; só a chain carrega o gate.

INTACTOS (confirmar): o NÚMERO (formatted_counter_at) e a decisão P335; α / caso 2, caso 4,
  morph ==/morph_canon (P345), flag P350c, Marco G.

lente (instrumento): o introspect-chain pode mudar arestas internas — registrar o antes/depois
  como medição (NÃO é gate; content→elements continua fato da lente, não critério do F).
perf (critério 4): antes = baseline da mesma sessão (re-medir no início); depois reportado.
L0 (critério 5): introspect.md + f_fronteira_e1.md com hashes sincronizados ANTES do código,
  Trava cumprida (aprovação do dono registrada).
```

---

## Válvula declarada

Se a Fase A medir que **introspect-chain + os 3 numbering** não cabe num lote, fatiar:
- **P363** = o introspect-chain (a infra, aditiva, verificável sem o de-bake ainda).
- **P364** = o de-bake dos 3 numbering sobre a infra.
O ponto 4 (`TextStyle`) já é o **F-5b**, separado. Registrar a fatia e o número medido.

---

## O que NÃO fazer

- **Não tocar o contador** (P335 — só o gate, não o número).
- **Não de-bakar o `TextStyle`** (F-5b — arrasta o bold do heading).
- **Não tocar o α / caso 2, o caso 4, o `morph_canon`/`==`, a flag P350c nem o Marco G.**
- **Não re-litigar a demanda** (o dono decidiu pelo princípio da fonte única).
- **Não pular a Trava** (L0 + hash do dono antes de qualquer `.rs`).
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Relatório (`typst-passo-363-relatorio.md` + resumo no chat)

As leituras da Fase A com `file:line` (os 3 numbering, os consumidores, o caminho do introspect-chain
medido, a separação gate-vs-número); o L0 (introspect.md + f_fronteira_e1.md) com hashes e a Trava
aprovada; o introspect-chain e o de-bake com `file:line`; a paridade pela rede de caracterização
(e qualquer asserção corrigida, justificada); a prova de que o número/P335, o α/caso 2, o caso 4, o
morph `==` e a flag ficaram intactos; a medição da lente (antes/depois, como instrumento); a perf;
`git status` limpo por estágio fora de `lab/` e docs; lint 0/0; o caveat de stack.

## Fora de escopo (confirmado)

O **ponto 4** (`Content::Text` `TextStyle`) — **F-5b** (arrasta o bold do heading); o **F-6** (item
2 da auditoria); o **mapa aberto para `#set` de props de usuário** (item 3); o **Marco G** (não é F);
DEBT-59 (flag CLI); DEBT-60 contador; qualquer toque no α / `morph_canon` / `==` ou na flag.
