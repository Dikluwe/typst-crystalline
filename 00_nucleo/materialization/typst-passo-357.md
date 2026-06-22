# Passo 357 — Medição da lacuna (ii): múltiplos func same-kind (decide A2 vs A3)

> **O que faz.** **Mede** o que falta para fechar a lacuna (ii) do recon do P355 — **múltiplos
> `#show` func de mesmo kind sobre um elemento** (hoje só a 1ª efetiva; o vanilla acumula). Mede
> duas coisas que **decidem o rumo**, e que **nunca foram medidas** (a premissa "raro" foi
> suposta no P354, herdada na ADR-0109, e refutada para a lacuna (i) — esta é a lição do P355):
> **(1) a demanda** — a referência da linguagem promove múltiplos func same-kind como feature, ou
> é marginal? **(2) a ordem exata** que o vanilla produz nos casos não-comutativos. **Read-only**,
> zero código de produto, zero L0. Termina numa **decisão proposta ao dono** (A2 / A3 / fechar) —
> **a escolha de desenho é do dono**, agora com o custo de A3 medido na frente. **NÃO implementa.**
> Saída: `00_nucleo/diagnosticos/f-recon-lacuna-ii-passo-357.md`.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P357 (confirmar livre). *(O diagnóstico do contador / DEBT-60 desloca para
o P358.)*
**Pré-condição**: P356 fechado (lacuna (i) show-set+func feita; suíte **2736**; lint **0/0**;
lente 66; o teste `multiplos_func_same_kind_ainda_diverge_lacuna_ii` documenta a (ii) como aberta).
HEAD pós-P356, árvore limpa. Caveat de stack: `RUST_MIN_STACK=33554432`. Se algo não bater, parar
e reportar.
**Tipo**: **medição/recon** — zero código de produto, zero L0. Probes descartáveis (compiladas e
**revertidas**; árvore limpa; suíte não re-rodada). O passo **termina numa decisão proposta ao
dono** (como P337/P347d/P354/P355).

---

## O que medir (a fonte vence; `file:line`)

1. **ADR-0107 e ADR-0108** — reler e aplicar. A classificação demanda/raridade **só vale
   medida** (a regra que o P355 violou).
2. **A demanda — contra a referência da linguagem** (o gate da lição do P355):
   - `lab/typst-original/docs/reference/language/styling.md` **e o resto de `docs/reference/`**:
     múltiplos `#show` **func** (transform) de mesmo kind sobre o mesmo elemento aparecem como
     feature promovida/exemplificada? **Quantos exemplos**, e são **centrais ou marginais**?
   - **Nuance medida a registrar**: o exemplo canônico do `styling.md` (4× `#show heading`) tem
     **3 show-set + 1 func** — ele **não** mostra múltiplos func same-kind. Então a demanda da
     (ii) é **pergunta separada** da (i); a resposta **não** está dada por esse exemplo. Procurar
     no resto da referência se múltiplos func same-kind ocorrem de fato.
   - **Não concluir "raro" sem a varredura.** Reportar o que foi achado, com `file:line`.
3. **A ordem exata do vanilla nos casos não-comutativos** (vanilla 0.14.2 como oráculo, probe
   descartável):
   - 2 func same-kind não-comutativos, ex.: `#show heading: it => [A:]+it.body` ⨁ `#show heading:
     it => [B:]+it.body` sobre `= T` → **qual o output exato** do vanilla? (innermost-first:
     `next_back`, `styles.rs:835`, então a última-declarada primeiro — confirmar o resultado
     concreto, não inferir.)
   - O caso **comutativo** (ex.: dois `upper`/`lower` que não dependem de ordem) → confirmar que
     A2 e A3 **coincidem** aí (a divergência de A2 só existe nos não-comutativos).
4. **O espaço de desenho — re-confirmar** (P354 §4), marcando medido vs inferido:
   - **A1** (guard por-`RuleId`-uma-vez): regride o caso 2 (a regra guardada não re-aplica ao
     output recursivo) — **inviável**. Confirmar ainda inviável.
   - **A2** (guard por-`(RuleId, morph_canon)`): compõe + preserva o α; **diverge na ordem** dos
     não-comutativos (R1-até-fixo-depois-R2, não intercalado). Toca só `rules.rs:97-248`.
   - **A3** (guard por-instância do vanilla): paridade **exata** de ordem; **reabre o α/caso 2**.
   - **Se a medição revelar um modelo não considerado (A4), reportar** — não forçar o resultado
     no espaço A2/A3.
5. **O custo de A3, medido** (para a decisão ter o preço na frente): o que exatamente reabrir o
   α/caso 2 custa — quantos sítios em `rules.rs:97-248`, **quais testes do caso 2 entram em
   risco** (`p348_*`, o teto-64, a identidade de instância que o P347d/P348 dispensou). Sem isto,
   "A3 reabre o α" é uma etiqueta sem preço.

---

## O que o passo decide — o mapa resultado → rumo (a DECISÃO é do dono)

A medição **produz** a recomendação; o dono escolhe. Os ramos:

- **Se a referência promove múltiplos func same-kind como feature E os casos não-comutativos são
  comuns** → a **ordem exata importa** para a paridade → recomenda **A3** (paridade exata), **se**
  o custo medido de reabrir o caso 2 for aceitável; **senão**, **A2 com a divergência de ordem
  declarada e justificada** (um gatilho de reabertura para A3 quando a ordem exata for exigida).
- **Se a referência não promove / é marginal E os não-comutativos são raros** → recomenda **A2
  com a divergência de ordem declarada** — basta, e **não** se paga o custo de reabrir o α
  (ADR-0107: não reabrir o caso 2 sem demanda de ordem exata medida).
- **Se a medição mostrar que múltiplos func same-kind no mesmo elemento NÃO é feature** (o
  vanilla também não acumula, ou a referência não o usa) → **não há lacuna a fechar**: declarar e
  **fechar o caso 1** com a (i) feita e a (ii) registrada como não-aplicável. (Improvável — o
  P354 mediu que o vanilla **acumula** via guard por-instância — mas a medição confirma ou
  refuta.)
- **Em qualquer caso**: a escolha A2 / A3 / fechar é do dono; o próximo lote (implementação ou
  fecho) **nasce da decisão**, não desta probe.

---

## O que NÃO fazer

- **Não escrever código de produto.** Probes descartáveis para medir são permitidas (compiladas e
  **revertidas**, árvore limpa — padrão P352/P354/P355); produção, não.
- **Não editar L0.** A edição nasce do lote seguinte, após a decisão.
- **Não decidir o rumo.** O recon mede e recomenda; o dono decide A2/A3/fechar.
- **Não assumir "raro".** A demanda é **medida** contra a referência — é a regra que o P355
  quebrou.
- **Não reabrir o α nesta probe.** Medir o custo de A3 é leitura; pagá-lo é o lote seguinte, se o
  dono escolher A3.
- **Não importar a quarentena.** `lab/` é leitura de semântica/doc, nunca import.

---

## Saída/relatório (`f-recon-lacuna-ii-passo-357.md` + resumo no chat)

A demanda medida contra a referência (com `file:line`: quantos exemplos de múltiplos func
same-kind, centrais ou marginais); a ordem exata do vanilla nos não-comutativos (os outputs
concretos) e a confirmação de que A2≡A3 nos comutativos; o espaço de desenho re-confirmado
(A1 inviável, A2/A3, ou um A4 se aparecer), medido vs inferido com o que refutaria; **o custo de
A3 medido** (sítios + testes do caso 2 em risco); a recomendação marcada conforme o mapa acima; a
decisão proposta ao dono. Suíte não re-rodada (read-only); árvore limpa; lint inalterado; caveat
de stack se alguma probe foi compilada e revertida.

## Fora de escopo (confirmado)

A **implementação** da (ii) (A2/A3) ou o **fecho** do caso 1 — nasce da decisão do dono, no lote
seguinte; o diagnóstico do contador `1.1`≠`0.1` / DEBT-60 (P358); o de-bake do F-5 (adiado P353);
F-6; Marco G; flag na CLI (DEBT-59); qualquer toque no α / `morph_canon` / `==` ou na flag.
