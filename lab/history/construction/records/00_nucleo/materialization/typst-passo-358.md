# Passo 358 — caso 1, lacuna (ii): conserto de ordem innermost-first (fecha o caso 1)

> **O que faz.** Fecha a **lacuna (ii)** conforme a decisão do P357: inverte a iteração de
> `node_rules` para **innermost-first** (a **última-declarada** vence), casando o **`"B:T"`** do
> vanilla no subconjunto de um-func-efetivo. **A medição do P357 estabeleceu**: o vanilla **NÃO
> acumula** múltiplos func same-kind (aplica um quando o output muda de kind; **erra** no caso
> mesmo-kind); **demanda zero** na referência; a **única divergência real é de ORDEM**. Por isso
> este conserto é **completo, não parcial** — **não há acumulação a mascarar** (a objeção que
> P352/P355 levantaram contra a reordenação **cai com a premissa** que a sustentava). **A2/A3
> declinados** (alto custo, reabrem o α/P348 selado, por demanda nula e um alvo que o vanilla
> erra). **Não toca o α.** O teste `multiplos_func_same_kind_ainda_diverge_lacuna_ii` (P356)
> **vira de divergência para paridade**. **Sem ADR de divergência** (acumulação de func não é
> feature); o **residual** (func mesmo-kind-returning → recursão = caso 2, paridade de erro) é
> **declarado no L0** com gatilho de reabertura. **Fecha o caso 1 e, com ele, a F-realização.**

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P358 (confirmar livre). *(O diagnóstico do contador / DEBT-60 desloca para
o P359.)*
**Pré-condição**: P357 fechado — a medição refutou a premissa "vanilla acumula func"; demanda
zero medida na referência; decisão do dono **FECHAR + conserto de ordem innermost-first; A2/A3
declinados**. HEAD pós-P356 (`37adaf867`), suíte **2736**, lint **0/0**, árvore limpa. Caveat de
stack: `RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: caso 1, lacuna (ii) — **paridade** (casa o `"B:T"` do vanilla). **NÃO
content-preserving** para o subconjunto um-func-efetivo: a ordem flipa de **1ª-declarada** para
**última-declarada**. O **único** teste que muda asserção é o `multiplos_func_..._lacuna_ii`
(P356) — de divergência declarada **para paridade**; mudança **esperada e justificada** (move
para o vanilla). Confirmar que nenhum outro teste muda.

---

## Por que a reordenação é legítima agora (o que mudou desde P352/P355)

No P352 e no P355 eu marquei o conserto de ordem como **mascaramento** — ele consertaria a
polaridade mas deixaria o caso de **acumulação** (B1) aberto e escondido. A medição do **P357**
removeu essa base: para **func**, **não existe** caso de acumulação (o vanilla aplica um ou
erra). O B1 do spike era **show-set** (fold de estilo), já feito no **P356** (lacuna (i)). Logo o
conserto de ordem **não deixa buraco escondido** — ele é o conserto **completo** da única
divergência real (ordem). A objeção do P355 caiu junto com a premissa de acumulação.

---

## Leituras da Fase A (a fonte vence; `file:line`)

1. **ADR-0107 e ADR-0108** — reler. A divergência é de **ordem** (observável, língua); o alvo é
   o `"B:T"` do vanilla; declinar A2/A3 é a 0107 (não reabrir o caso 2 sem demanda medida — e a
   demanda é zero).
2. **O recon do P357** — `f-recon-lacuna-ii-passo-357.md`: confirmar (a) vanilla não acumula func
   (um, ou erro mesmo-kind); (b) demanda zero na referência; (c) inverter `node_rules` é **no-op
   para regra única e para cross-kind** — só flipa o same-kind multi-func.
3. **O ponto do conserto** — `rules/eval/rules.rs` (o loop de `apply_show_rules`/`apply_all`): a
   iteração de `node_rules` a inverter para innermost-first.
4. **As travas a confirmar verdes**: os 5 testes do caso 2 (`p348_show_recursao_converge_…:693`,
   `…o_inf_converge:708`, `…ciclo_erra:722`, `f3s2_show_callout_anti_recursao_termina:657`,
   `show_rule_nao_recursiva_sem_stack_overflow:3042`) — **no-op** (usam 1 regra); o show-set
   (P352) e o show-set+func (P356) — inalterados; `show_rule_encadeamento_duas_regras` (cross-kind)
   — inalterado.
5. **Confirmar que o `multiplos_func_..._lacuna_ii` (P356) é o ÚNICO teste de same-kind
   multi-func** — logo a única asserção que muda.

---

## Limites duros

- **Não tocar o loop α / caso 2.** O conserto inverte só a **ordem** de iteração de `node_rules`;
  para regra única é **no-op**; o `morph_canon`/`==` (P345) e o teto ficam intactos.
- **Não fazer A2/A3** (declinados no P357). Acumular func divergiria mais (A2) ou reabriria o
  α/P348 (A3) por demanda nula.
- **Não escrever ADR de divergência.** Acumulação de func não é feature (medido); não há
  divergência de língua a declarar. O **residual** (mesmo-kind-returning erra em ambos) é **caso
  2, paridade de erro** — declarado no L0, não numa ADR de divergência.
- **Não apresentar como acumulação.** Não há acumulação; o conserto é de ordem.
- **Não tocar o caso 4, a flag P350c, o DEBT-59 nem o Marco G** (`edges content→elements` = 66).

---

## Estágios

### Estágio L0 — a ordem correta + o residual (Trava; PARA aqui para o dono)
`entities/show.md`:
1. A regra de ordem: **innermost-first** — no subconjunto de um-func-efetivo, a **última-declarada**
   vence (casa o vanilla; cross-ref `styles.rs:835` `next_back`).
2. O **residual declarado**: 2+ func same-kind **mesmo-kind-returning** → ambos (vanilla e
   crystalline) **erram por recursão** (= caso 2, já tratado pelo α/teto) — **paridade de erro**,
   não buraco. Com o **gatilho de reabertura** para A2/A3 se surgir demanda real de ordem/acumulação
   exata (que o P357 mediu como zero).
Sincronizar o hash. **TRAVA**: para no chat para a aprovação do dono antes de qualquer `.rs`.

### Estágio 1 — o conserto (após aprovação)
Em `rules/eval/rules.rs` (`apply_show_rules`/`apply_all`): inverter a iteração de `node_rules`
para **innermost-first** (última-declarada primeiro). Sem guard, sem tocar o α.

### Estágio Teste
- **Vira para paridade**: `multiplos_func_same_kind_ainda_diverge_lacuna_ii` (P356) passa a
  asserir **`"B:T"`** (última-declarada vence) — de divergência **para paridade** com o vanilla.
- **Verde (não regredir)**: os 5 testes do caso 2 (no-op); o show-set (P352); o show-set+func
  (P356); o cross-kind (`show_rule_encadeamento_duas_regras`).
- Confirmar que o `multiplos_func_..._lacuna_ii` é a **única** asserção alterada.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): 2736, com UMA asserção alterada (multiplos_func_..._lacuna_ii:
  de "A:T" divergente para "B:T" paridade — esperada, justificada vs vanilla; reportar). Nenhuma
  outra muda.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável, nível da língua — ADR-0107/0108; oráculo = vanilla 0.14.2):
  - 2 func same-kind (output muda de kind): a última-declarada vence ("B:T"), igual ao vanilla.
  - residual mesmo-kind-returning: erro por recursão em ambos (= caso 2; paridade de erro,
    declarada no L0).

INTACTOS (confirmar): α / caso 2 (5 testes no-op, verdes), caso 4 (P340), morph ==/morph_canon
  (P345), flag P350c, Marco G (edges content→elements = 66); show-set (P352) e show-set+func (P356).

lente (critério 3): edges(content→elements::*) = 66 INALTERADO; edges(elemento→elemento) = 0.
perf (critério 4): antes = o baseline da mesma sessão (re-medir no Estágio 0 se a sessão mudou —
  o 0.7564/0.7473 do P356 era daquela sessão); reportar o depois — reordenação O(regras), ~nulo.
L0 (critério 5): show.md (ordem + residual) com hash sincronizado ANTES do código, Trava aprovada.
```

---

## O que NÃO fazer

- **Não fazer A2/A3** (declinados — custo alto, reabre o α/P348, demanda nula).
- **Não escrever ADR de divergência** (acumulação de func não é feature; o residual é caso 2).
- **Não apresentar como acumulação** (é conserto de ordem; não há acumulação).
- **Não tocar o loop α / caso 2, o caso 4, a flag P350c nem o Marco G.**
- **Não pular a Trava** (show.md + hash do dono antes de qualquer `.rs`).
- **Não importar a quarentena.** `lab/` é leitura de semântica/doc, nunca import.

---

## Relatório (`typst-passo-358-relatorio.md` + resumo no chat)

A confirmação da Fase A (vanilla não acumula func; demanda zero; inverter `node_rules` é no-op
para regra única/cross-kind); o L0 (`show.md`, ordem + residual) com hash e a Trava aprovada; o
conserto com `file:line`; o teste virando de divergência para paridade (a única asserção
alterada); a prova de que os 5 testes do caso 2, o caso 4, o show-set (P352), o show-set+func
(P356), o morph `==` e a flag ficaram intactos; os números da lente (edges 66) e da perf
(antes/depois); **a nota de que o caso 1 e a F-realização fecham com este lote**; `git status`
limpo por estágio fora de `lab/` e docs; lint 0/0; o caveat de stack.

## Estado da fila após o P358 (para o resumo do chat)

Com a lacuna (ii) fechada, **o caso 1 fecha** (i: show-set+func P356; ii: ordem P358; o residual
é caso 2). Com ele, **a F-realização fecha** — casos 1, 2 (P342–P350c), 3 (show-set P352), 4
(escopo P340) todos feitos. A fila F passa a: **F-5** de-bake (adiado no P353 como limpeza sem
demanda), **F-6** (3 folhas, DEBT-58), e o **Marco G** (pós-F-6, spec própria). Débitos nomeados:
DEBT-59 (flag na CLI), DEBT-60 (contador `1.1`≠`0.1` + supplement "Secção", P359).

## Fora de escopo (confirmado)

A2/A3 (declinados; só o gatilho de reabertura); o diagnóstico do contador / DEBT-60 (P359); o
de-bake do F-5 (adiado P353); F-6; Marco G; flag na CLI (DEBT-59); qualquer toque no α /
`morph_canon` / `==` ou na flag.
