# Passo 353 — F-5: de-bake dos 4 pontos assados (+ restauração do rig de perf)

> **O que faz.** **De-bake**: os **4 pontos assados** (heading `numbering`, equation
> `numbering`, figure `numbering`, e o `TextStyle` do `Content::Text`) **deixam de assar**; o
> consumidor passa a **ler a chain** que a F-realização garante no nó (o `Content::Styled` da
> fatia 1, P339). **Estágio 0**: restaura o **rig de perf do P330** ao repo — o critério 4 do
> plano ficou inmedível sem ele desde o P352. O ponto 4 (de-bake do `TextStyle`) **completa o
> render do show-set** — é exatamente a lacuna que o relatório do P352 declarou (show-set
> feito na morfologia, render pendente do F-5). **Content-preserving por ponto**: cada ponto
> de-baked lê a chain e produz output **idêntico** ao campo assado, com a **rede de
> caracterização (+11, P331 Fase 2)** como oráculo de paridade. **Não toca** o loop α / caso 2,
> o caso 4, o `morph_canon`/`==`, a flag P350c nem o Marco G. **Risco `is_numbering_active`
> RESOLVIDO** (a API morta foi removida no P338; são **4 pontos, não 5**).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P353 (confirmar livre; independente do P354).
**Pré-condição**: P352 fechado (show-set materializado; `Transformation::Style` + reutilização
do `Content::Styled`; suíte **2733**, lint **0/0**). HEAD pós-P352, árvore limpa. Caveat de
stack: `RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: F-5 de-bake — **content-preserving por ponto** (o consumidor lê a chain em vez do
campo assado; mesmo output). Onde o de-bake **corrigir** um bug que o assado mascarava, a
mudança de asserção é **declarada e justificada** contra a rede de caracterização — não
silenciosa (lição do S5b).

---

## Leituras da Fase A (a fonte vence; `file:line`)

1. **ADR-0107 e ADR-0108** — reler e aplicar. Paridade com a linguagem; medir antes de decidir;
   afirmar só o medido (relevante ao critério 4 / perf abaixo).
2. **L0 da realização** — `entities/f_fronteira_e1.md`: a seção dos 4 pontos assados e o
   transporte `Content::Styled` que o consumidor passa a ler. Sincronizar o hash antes do
   código (critério 5).
3. **Rede de caracterização (+11, P331 Fase 2)** — a spec de paridade do de-bake. Localizar os
   testes e confirmar que cobrem os 4 pontos.
4. **Os 4 pontos, com `file:line`**: onde cada campo é assado hoje e onde o consumidor o lê —
   heading `numbering`, equation `numbering`, figure `numbering`, `Content::Text` `TextStyle`.
   Confirmar que são **4** (não 5): a API `is_numbering_active`/`is_numbering_active_at` foi
   **removida no P338** (triagem-47, Desfecho A); o de-bake **não** religa esse consumidor.
5. **Recon** — `f-recon-passo-337.md` / `f-recon` do P338: o dimensionamento do de-bake e o
   estado "risco `is_numbering_active` RESOLVIDO".

---

## Limites duros

- **Não tocar o loop α / caso 2 (recursão), o caso 4 (escopo, P340), o `morph_canon`/`==`
  (P345), a flag P350c nem o Marco G** (`edges content→elements` = 66). O de-bake mexe nos 4
  campos e em quem os lê — não nesses caminhos.
- **Cada ponto de-baked é paridade** contra a rede de caracterização: o output via chain é
  **idêntico** ao output via campo assado. Discrepância = ou o de-bake está errado, ou o
  assado mascarava um bug — nos dois casos, **declarar**, não esconder.
- **Não deixar caminho duplo dormindo.** O de-bake **remove** o campo assado e aponta o
  consumidor para a chain — não roteia pela chain mantendo o campo (isso seria o tampão do
  F-6, com a condição C1; não é o F-5). Se por algum ponto o campo não puder sair de uma vez,
  o caminho duplo nasce com gatilho de remoção escrito + teste de paridade entre os dois
  caminhos (C1), não adormecido.
- **Não religar `is_numbering_active`** — está morto desde o P338.

---

## Estágios

### Estágio 0 — restaurar o rig de perf (commit isolado; medição, não produto)
- Restaurar ao repo o harness de perf do P330 (o que produziu `0.6518 s ± 0.0057`). Registrar
  onde mora e o comando exato.
- **Re-medir o "antes" no HEAD atual** (pós-P352): muitos passos se passaram; o baseline pode
  ter mudado. Registrar o **novo baseline medido** — não reusar o `0.6518` por memória
  (ADR-0108: afirmar só o medido).
- Isto destrava o critério 4 para o F-5 **e** para os lotes futuros (F-6, caso 1). Commit
  isolado, sem tocar produto.

### Estágios 1–4 — de-bake por ponto (um commit por ponto; agrupar só se a Fase A medir que cabem)
1. **heading `numbering`** deixa de assar; o consumidor lê a chain.
2. **equation `numbering`** idem.
3. **figure `numbering`** idem.
4. **`Content::Text` `TextStyle`** deixa de assar; o consumidor lê a chain — **completa o
   render do show-set** (o efeito de `set text(...)` sobre o texto agora alcança o render, não
   só a morfologia).

Cada ponto: paridade contra a rede de caracterização (output idêntico ao campo assado).

### Estágio Teste
- A **rede de caracterização (+11)** passa sem alteração — é a spec de paridade. Qualquer
  asserção que mude é onde o de-bake corrige um bug que o assado mascarava; justificar uma a
  uma.
- **O teste de show-set do P352** (`show_set_text_embrulha_heading_em_styled_bold`) agora
  mostra o **efeito de render** — o heading renderiza bold de verdade, fechando a lacuna que o
  relatório do P352 declarou. Adicionar/estender o teste para asserir o render, não só a
  morfologia.
- Confirmar por construção/teste que o loop α / caso 2, o caso 4, o `morph_canon`/`==` e a
  flag P350c **ficaram intactos**.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates)

```
build: limpo por estágio (release buildável por estágio — par de perf granular, agora com o
  rig restaurado no Estágio 0).
suíte (RUST_MIN_STACK=33554432): 2733 ± as asserções que o de-bake corrige (só essas;
  justificadas contra a rede de caracterização; reportar quais e por quê) + o teste de render
  do show-set.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável, contra a harness de paridade — o oráculo):
  - cada um dos 4 pontos: o output via chain é idêntico ao output via campo assado (paridade).
  - show-set: `#show heading: set text(bold: true)` agora RENDERIZA bold (não só na morfologia)
    — a lacuna do P352 fechada.

INTACTOS (confirmar): loop α / caso 2 (recursão), caso 4 (escopo P340), morph ==/morph_canon
  (P345), flag P350c, Marco G (edges content→elements = 66).

lente (critério 3): edges(content→elements::*) = 66 INALTERADO; edges(elemento→elemento) = 0;
  par --comparar antes/depois na camada que o de-bake toca.
perf (critério 4): antes = o NOVO baseline medido no Estágio 0 (não o 0.6518 de memória);
  reportar o depois (≥10 execuções) com o rig restaurado.
L0 (critério 5): f_fronteira_e1.md auditado/atualizado e hash sincronizado ANTES do código.
```

---

## Válvula declarada

Se a Fase A medir que os 4 pontos juntos passam a faixa validada, fatiar:
- **P353** = Estágio 0 (rig) + os 3 numbering (heading/equation/figure).
- **P354'** = o `TextStyle` do `Content::Text` (o que completa o render do show-set).
Registrar a fatia e o número medido. (Renumerar o diagnóstico do caso 1 se preciso.)

---

## O que NÃO fazer

- **Não tocar o loop α / caso 2, o caso 4, o `morph_canon`/`==`, a flag P350c nem o Marco G.**
- **Não religar `is_numbering_active`** (morto desde o P338).
- **Não deixar caminho duplo dormindo** (se um campo não sair de uma vez, C1: gatilho + teste
  de paridade).
- **Não fazer o caso 1 (composição)** — é o P354 (diagnóstico/desenho).
- **Não reusar o `0.6518` por memória** — re-medir o "antes" no Estágio 0.
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Relatório (`typst-passo-353-relatorio.md` + resumo no chat)

O rig restaurado (onde mora, o comando, o novo baseline medido); os 4 pontos de-baked com
`file:line`; a rede de caracterização passando (e qualquer asserção corrigida, justificada);
o teste de render do show-set fechando a lacuna do P352; a prova de que loop α / caso 2 / caso
4 / morph `==` / flag ficaram intactos; os números da lente (edges 66 inalterado, par
`--comparar`) e da perf (novo antes/depois); `git status` limpo por estágio fora de `lab/` e
docs; lint 0/0; o caveat de stack.

## Fora de escopo (confirmado)

Caso 1 (composição — P354 diagnóstico/desenho); F-6 (3 folhas, DEBT-58); Marco G; exposição da
flag na CLI (DEBT-59); qualquer mudança no loop α / `morph_canon` / `==` ou na flag de
diagnóstico.
