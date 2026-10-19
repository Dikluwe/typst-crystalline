# Passo 337 — Recon dos lotes restantes (F-4 / F-5 / F-6 / F-realização) + fecho do P336

**Pré-condição**: P336 fechado (`677a89d11`) — F-3 FECHADO, suíte **2717**,
lint 0/0, árvore limpa.
**Tipo**: **caronas de fecho** (código mínimo: 1 teste + 2 frases de doc) +
**recon dimensionador** — sem código de produção além das caronas. O
entregável é um **relatório com números** e uma **proposta de ordem**, fechado
por checkpoint do dono.
**Objetivo**: transformar "falta muito para o F?" em quatro dimensões medidas
e uma ordem decidida com evidência — em particular a ordem entre **F-5
(de-bake)** e **F-realização**, que se tocam: os dois mexem em como
estilo/transformação chega ao consumo, e a ordem errada pode significar
des-assar duas vezes.

---

## Caronas (commit próprio, antes do recon)

- **C1 — o contrato negativo da `FuncRepr::Element`** (pendência apontada na
  revisão do P336): teste de que a variante é **veículo do construtor de
  elemento, não chamável genérico** — um `Func` comum de utilizador
  (closure) não passa pelo braço `Element` do `apply_func`, e um
  `Func::element` não é construível pela superfície de linguagem comum
  (só via registry no escopo). Se um teste existente já cobre isso sob outro
  nome, **apontar e não duplicar**. **Δ testes: +1 ou 0 (apontado).**
- **C2 — a emenda do L0 §3b.6** (duas frases que faltaram no S4 do P336):
  1. **Redefinição do termo**: para elemento custom, "vanilla medido"
     significa *semântica extraída da fonte do `lab/` com `file:line`* —
     execução é inviável (sem binário pronto; sem como definir elemento
     custom em vanilla `#[elem]`).
  2. **Gatilho de segundo nível (escalada)**: se um teste de paridade
     baseado em leitura-da-fonte conflitar com comportamento observado, ou a
     leitura for ambígua num caso concreto, **então** compila-se o
     typst-cli do `lab/` e mede-se com **nativos** o mecanismo compartilhado
     em disputa. **Δ testes: 0.**

## Fase A — recon (com `file:line`; paralelizável por lote, precedente P335)

Para **cada** um dos quatro lotes, o mapa responde as mesmas cinco perguntas:
**(i)** onde mora a maquinaria (arquivos/funções/linhas); **(ii)** quantos
pontos de toque (produtores, consumidores, leitores — número, não "~");
**(iii)** o que o vanilla faz no equivalente (`lab/`, `file:line`);
**(iv)** quais DEBTs/decisões registradas o lote fecha ou esbarra;
**(v)** riscos nomeados (o que pode mascarar, o que pode pendurar, o que
exige decisão do dono).

### A1 — F-4 `Styled` (a 2ª StyleChain do Layouter colapsa na chain única)

- Onde vive a 2ª StyleChain do Layouter; quem a constrói; quem a lê.
- `Content::Styled`: produtores e consumidores hoje; relação com o canal
  `StyleDelta.custom` do F-2 (colapsam no mesmo mecanismo ou convivem?).
- Pergunta de dependência: o F-5 (de-bake) **pressupõe** a chain única do
  F-4? (Se des-assar significa "resolver na chain ao consumir", qual chain?)

### A2 — F-5 `de-bake`

- Inventário **exato** dos pontos assados: `HeadingElem.numbering_active`,
  `ElementPayload::Equation.numbering_active`, figure via
  `styles.custom("figure.numbering")` ao criar, e o que mais o padrão
  "assa na criação" tocou (grep dirigido + leitura).
- Para cada ponto: o que des-assar exige no consumidor (a chain disponível
  no ponto de layout? — a pergunta que conecta ao F-4) e o que quebra de
  testes (número).
- O vanilla como referência: onde o vanilla resolve essas propriedades
  (criação vs consumo), `file:line`.
- **A pergunta de ordem com o F-realização**: a realização multi-passe
  muda *quando* os elementos são transformados; o de-bake muda *quando* as
  propriedades são lidas. Mapear se há pontos onde as duas mudanças tocam
  as mesmas linhas — esse conjunto-interseção é o argumento da ordem.

### A3 — F-6 `folhas`

- O que "folhas" cobre exatamente (a definição do plano P333) e o
  inventário do que falta: quais elementos-folha, qual superfície cada um.
- É paralelizável/independente dos outros três? (Se sim, é o lote-tampão —
  o que se faz enquanto uma decisão grande descansa.)

### A4 — `F-realização` (caso 4 `#show` léxico + caso 1 composição + caso 3 show-set)

- O modelo eager hoje: onde `engine.show_rules` é mutado da declaração em
  diante (a causa da divergência do caso 4, `file:line`); o ciclo de vida
  de uma regra.
- O multi-passe do vanilla: o `realize` do `lab/` (`typst-realize`) —
  estrutura, fases, o que `Transformation::Style` (caso 3) exige.
- Dimensão do toque nos **nativos** (o lote afeta o modelo de todos, não só
  do dyn): quantos pontos do pipeline atual assumem o eager.
- O teste `f3s3_..._divergencia_registrada` como âncora: ele grita quando o
  caso 4 mudar — listar os demais testes que asseriam comportamento eager e
  precisarão de atualização consciente (número).

## Fase B — o relatório e a ordem (checkpoint do dono)

O entregável, num doc único (`f-recon-passo-337.md`):

1. **Tabela de dimensão**: lote × pontos de toque × testes afetados ×
   riscos nomeados × DEBTs fechados.
2. **Grafo de dependência entre os lotes** (com a evidência de A1/A2/A4):
   em particular F-4 → F-5 (a chain única é pré-requisito do de-bake?) e
   F-5 ↔ F-realização (o conjunto-interseção; quem vai primeiro e por quê —
   o critério é **não des-assar duas vezes** e **não migrar consumo para um
   modelo que o F-realização vai substituir**).
3. **Proposta de ordem com justificativa** — e a alternativa rejeitada, com
   a razão (o dono decide sobre opções, não sobre uma conclusão).
4. **Checkpoint do dono**: a ordem escolhida entra na fila
   (`f-plano-lotes-passo-333.md` atualizado).

## O que NÃO fazer

- **Nenhum código de produção** além das caronas — recon que começa a
  "consertar de passagem" vira lote sem prompt.
- **Nenhum "~" nas contagens** do relatório — pontos de toque e testes
  afetados são números ou a nota de por que não dá para contar ainda.
- **Não decidir a ordem sem o conjunto-interseção mapeado** — a ordem
  F-5/F-realização é a decisão cara; o recon existe para ela não ser chute.
- **Não tocar o teste-âncora da divergência** (`f3s3`) — ele só muda quando
  o F-realização executar.

## Critérios de Verificação

```
Dado as caronas
Então o contrato negativo da FuncRepr::Element testado (+1) ou apontado (0),
e o L0 §3b.6 com a redefinição do termo e o gatilho de escalada — suíte
2717 (+0 ou +1, exato), lint 0/0

Dado o recon de cada lote
Então as cinco perguntas respondidas com file:line e contagens exatas

Dado o relatório
Então a tabela de dimensão, o grafo de dependência com evidência, a proposta
de ordem com a alternativa rejeitada, e o checkpoint do dono registrado na
fila atualizada

Dado a árvore
Então limpa; commits: caronas + relatório; nenhum código de produção
```

---

## Histórico

| Data | Motivo |
|---|---|
| 2026-06-11 | P337: fecho do P336 (C1 teste-contrato negativo da `FuncRepr::Element` — veículo de construtor, não chamável genérico; C2 emenda do L0 §3b.6 — "vanilla medido" redefinido como semântica-da-fonte com file:line + gatilho de escalada para compilar o vanilla com nativos em caso de conflito/ambiguidade) + recon dimensionador dos quatro lotes restantes (F-4 Styled, F-5 de-bake, F-6 folhas, F-realização — o lote novo do gatilho do P336) com as cinco perguntas por lote, o conjunto-interseção F-5↔F-realização como argumento da ordem, e checkpoint do dono sobre opções com a alternativa rejeitada registrada. Sem código de produção; "falta muito para o F?" vira quatro números e uma ordem. |
