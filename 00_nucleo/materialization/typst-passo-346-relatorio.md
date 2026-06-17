# Passo 346 — relatório: destino do `content → elements → 0` (decisão do dono)

> **Decisão do dono: Saída 2 — marco pós-F-6.** O corte `content → elements::* → 0`
> (item carregado sem dono do P339 ao P345) ganha dono: o **Marco G — desacoplamento
> dos nativos**, registrado em `f-plano-lotes-passo-333.md`, **fora da fila F**, pós-F-6,
> com dependência na fronteira E1 (F-1) e o `target = 0` como métrica **dele** — não a do
> F. O item **deixa de ser órfão** e **não volta** como "item aberto carregado". Doc puro:
> zero `.rs`/`.toml`, lint **0/0**.

## Pré-condição (desvio registrado e resolvido)
Ao iniciar, o P345 estava **completo com gates verdes** (lint 0/0, suíte 2723/3242) **mas
não commitado** — HEAD em `6b61d27be` (P344), 19 modificações rastreadas. A pré-condição
literal do P346 ("HEAD pós-P345; árvore limpa") **não batia**. Surfacei o desvio; o dono
optou por **commitar o P345 primeiro**. Feito (`3ebb397fb — Passo 345 — == de conteúdo
morfológico`). HEAD passou a pós-P345, árvore de produto limpa → pré-condição satisfeita
antes de materializar o P346.

## O fato (fixado, para a decisão não ser sobre suposição)
`edges(content → elements::*) = 66`, `target = 0` (baseline lente P338). **Nenhum lote da
fila** F-1…F-6 + F-realização entrega o corte — confirmado lote a lote (F-1 aditivo
declarou `content→elements` inalterado; F-2…F-6 não removem os `use elements::*Elem` do
núcleo). A fila torna os elementos **extensíveis** e unifica estilo/realização; **não**
desacopla `content` dos 65 tipos concretos. O `target=0` era **expectativa órfã**, não
regressão.

## As três saídas apresentadas (o dono escolheu UMA)
1. **Reconciliar o baseline** (aceitar `≠ 0` por desenho) — fecharia a porta do
   desacoplamento.
2. **Marco pós-F-6** (desacoplamento como objetivo futuro) — **ESCOLHIDA**.
3. **Registrar lacuna** (adiar 1-vs-2) — só nomearia.

Recomendação do agente (marcada como tal) era a **Saída 2**; o dono confirmou.

## Registro materializado (o diff do doc)
**`00_nucleo/diagnosticos/f-plano-lotes-passo-333.md`** — nova secção, entre a fila F e os
"Critérios transversais":

> **## Marco G — desacoplamento dos nativos (pós-F-6, FORA da fila F)**
> Dono do corte `content → elements::* → 0` (decidido no P346, Saída 2). A fila F habilita
> (fronteira E1, F-1) mas **não entrega** o corte — `=66` não baixa na fila, por desenho
> do modelo D (enum fechado importa cada `*Elem`). O `= 0` é o marco G, fora da fila:
> converter os 65 nativos pela fronteira E1.
> - **Dependência**: fronteira E1 (F-1) — já habilitada.
> - **Métrica do marco G**: `edges(content → elements::*) → 0` (lente R3/R4) — é **aqui**
>   que o `target = 0` do baseline P338 mora (deixa de ser órfão).
> - **Métrica-gate do F** (distinta): atomização preservada + `elemento→elemento = 0` (já
>   satisfeito). O F **não** é medido por `content→elements`.
> - **Escopo**: migração grande (65 elementos), spec própria quando chegar — **não** é
>   trabalho desta branch; o P346 nomeia o marco, não o executa.

**Escopo respeitado**: materializada **só** a Saída 2 (uma edição de doc no plano).
**Não** editei o `baseline-…-338.md` — alterar o `target` lá seria a Saída 1; na Saída 2
o `target = 0` **permanece** no baseline, agora **possuído** pelo marco G via a referência
no plano.

## Fecho do item
O item `content → elements → 0` está **redirecionado**, não mais órfão: a partir do P346
ele **não** volta como "item aberto carregado" nos relatórios — é referência ao **Marco G**
(`f-plano-lotes-passo-333.md`). Relatórios futuros que precisem mencioná-lo apontam para o
marco, não para uma falha sem dono.

## Mapa de filtro (campo)
**Lugar lógico:** o destino do corte `content→elemento` é uma **decisão de fundação da
fila F** — o que a fila **entrega** (extensibilidade, unificação de estilo) vs o que ela
**habilita mas não entrega** (o desacoplamento). Na execução real ficou **órfã do P339 ao
P346** porque o baseline media um alvo (`target=0`) que a fila **nunca prometeu** —
medir o que a fila habilita como se fosse o que ela entrega.
**Rastro:** baseline P338 mede `target=0`; carregado sem dono P339–P345 (cada relatório
repetiu "item aberto carregado, três saídas"); **decidido aqui (P346, Saída 2)** — o alvo
ganha dono (marco G) e o gate do F é separado do gate do marco.

## Verificação (gates)
```
content-preserving: zero .rs/.toml. Suíte 2723 / 3242 intacta (não re-rodada — nada de
  código). Árvore de produto não tocada (só o doc do plano).
lint: crystalline-lint . = 0 violations, 0 warnings.
fronteira: as três saídas apresentadas; nada editado antes da escolha do dono.
escopo: materializada SÓ a Saída 2 (edição do plano) — baseline não tocado (seria Saída 1).
```

Nenhuma das três foi materializada além da escolhida. P345 commitado primeiro (aval do
dono); P346 (este) é o segundo commit, doc-only.
