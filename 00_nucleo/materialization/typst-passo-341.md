# Passo 341 — Spike de divergência: o modelo eager-cascade do `#show` vs o fixpoint do vanilla (medição, sem mudança de comportamento)

> **Reescrita da fatia 2.** O P340 (corrida autônoma) aterrou o caso 4 / `f3s3`
> (confinamento de escopo, content-preserving, committado) e estacionou o
> multi-passe ao bater em content-preservation. Este passo **mede** se o
> multi-passe é sequer preciso para fidelidade **comportamental**, antes de
> qualquer construção. Não constrói nada.

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P341 (confirmar livre; ver nota de numeração abaixo).
**Pré-condição**: P340 (corrida da noite) committado — `03619dc93` (L0 §3a.7-bis
+ decisões), `5cccf06bd` (caso 4 / `f3s3` confinado), `f859d8c9f` (caronas).
Suíte **2719** (`typst-core --lib`) / **3238** (workspace), lint 0/0, árvore
limpa. Se o estado-base não bater, parar.
**Tipo**: **spike de medição** — content-preserving estrito: **zero código de
produção, zero teste alterado, zero ficheiro de produto tocado**. Termina na
**TRAVA** com o mapa de divergência, para decisão do dono. **Não** construir o
fixpoint; **não** evoluir nenhum teste.
**Objetivo**: medir, contra o **vanilla compilado** (`lab/`), onde o modelo
eager-cascade do `#show` no cristalino diverge — na **saída observável** — do
fixpoint de realização do vanilla. O entregável é o mapa: padrão de interação →
saída vanilla → saída cristalino → igual/diverge. Esse mapa é o que diz **se** e
**quanto** do multi-passe é preciso.
**Fontes**: `lab/typst-original/` (compilar — referência de "medido"), os 18
testes eager (`show_rule_*`, achado §D do relatório P340), `f_fronteira_e1.md`
§3a.7-bis (o desenho do vanilla que a noite mapeou: `realize→visit→
visit_show_rules` `lib.rs:43/224/335`, fixpoint externo `:380`, recipe-index
innermost-first `:472`, teto `:401-402`, `visit_styled` `:574`),
`f-spike2-show-passo-333.md` (os 5 casos), relatório P340.

---

## Por que este passo (transcrever no §0 do relatório)

O P340 assumiu que a F-realização precisa trocar o `#show` eager-cascade pelo
fixpoint do vanilla. A medição da noite mostrou que essa troca colide com 18
testes que asseveram a semântica eager. Mas content-preservation a bloquear o
fixpoint **não prova que o fixpoint é preciso** — prova que o modelo atual tem
comportamento definido e testado. O norte do projeto (P329) é **fidelidade
comportamental**: mesma `.typ`, mesma saída. Construir o mecanismo do vanilla
porque o vanilla o tem seria fidelidade **estrutural** — o erro já nomeado e
rejeitado (opção C do P332; opção 2 do F-3; o "fiel ao vanilla" do β1). A única
coisa que justifica mudar o comportamento dos nativos é uma **divergência
medida** entre o eager-cascade e o vanilla. Este passo mede essa divergência —
nada mais.

---

## Fase A — inventário dos 18 testes eager (com `file:line`)

Sem código. Para cada um dos 18 testes que o §D do relatório P340 nomeou
(`show_rule_composicao_sem_loop`, `show_rule_encadeamento_duas_regras`, os
`show_rule_*` de composição/encadeamento, etc.), registar:
- a `.typ` que ele exercita e a asserção exata (a saída que ele fixa);
- a classe da semântica que ele asservera, em três baldes:
  - **(i) auto-reaplicação prevenida por guard** — uma regra não se reaplica ao
    próprio output. **Hipótese a verificar:** o vanilla também previne isto (guard
    por `RuleId`, `world_types.rs:249` / `lib.rs:401-402`), logo provável paridade;
  - **(ii) interação multi-regra** — o output de uma regra deveria (ou não)
    disparar **outra** regra. **Esta é a pergunta real:** sob cascata o cristalino
    aplica B ao output de A? Sob fixpoint o vanilla aplica?
  - **(iii) outra** — descrever.

Saída: tabela "teste × `.typ` × asserção × balde". É o que diz quais testes
estão em risco de divergir (os de balde ii) e quais quase certamente são fiéis
(balde i).

---

## Fase B — a bateria de medição (vanilla compilado vs cristalino)

Sem código de produção. Construir uma bateria de padrões de interação `#show`,
compilar **cada um no vanilla** (`lab/`, registar o comando) e rodar **o mesmo no
cristalino**, comparando a **saída observável** (o que a fatia 1 usou: layout/
`plain_text` e introspect/`query` — não estrutura interna). Padrões mínimos a
cobrir (acrescentar os que a Fase A revelar):

1. Regra única sobre um nativo.
2. Duas regras independentes (alvos disjuntos).
3. **Regra cujo output casa o seletor de outra regra** (A→B) — o discriminador
   central entre cascata e fixpoint.
4. Recursão controlada (regra que se aplicaria ao próprio tipo de output; o guard
   deve cortar) — comparar onde o guard corta no vanilla vs no cristalino.
5. Composição em cadeia de 3+ regras.
6. Show-set (`#show … : set …`) — **apontar como fatia 3 (P342)**, medir só para
   o mapa, não para conserto aqui.

Para cada padrão, registar: `.typ`, comando de compilação do vanilla, saída do
vanilla, saída do cristalino, **igual/diverge**, e — se divergir — em que ponto
(`file:line` do vanilla que explica a diferença). Onde a leitura da fonte for
ambígua, a compilação do vanilla é a árbitra (é o que "medido" já permite).

Tabela final: **mapa de divergência** = padrão × {vanilla, cristalino, veredito,
causa medida}.

---

## TRAVA ARQUITETURAL — checkpoint com o mapa

Emitir o mapa de divergência e a recomendação. Dois desfechos possíveis, ambos
para **decisão do dono**:

- **Zero divergência observável** → o eager-cascade é **fiel**; o multi-passe
  **não é preciso** para fidelidade. Consequências a propor: a §3a.7-bis fica como
  desenho do vanilla **registrado por referência, não como obrigação**; os 18
  testes ficam (estão certos); a "fatia 2 multi-passe" **encerra** (o caso 4 foi o
  único trabalho real dela); a fila segue para a fatia 3 (show-set) e F-5.
- **Divergências existem** → listar **exatamente** quais padrões divergem e a
  causa medida. Propor o **conserto mais estreito** que fecha cada divergência
  (pode ser pontual — uma reaplicação dirigida em A→B — e **não** um fixpoint
  inteiro). O dono decide o escopo do lote de execução, que então roda com a regra
  certa: **paridade contra o vanilla compilado**, com os testes do balde (ii)
  evoluídos **um a um** com a saída do vanilla colada como justificativa (os do
  balde (i) permanecem intocados).

Parar aqui. Nenhum código de produção, nenhum teste alterado.

---

## Verificação (gates)

```
medição reproduzível: cada compilação do vanilla e cada corrida do cristalino
  com o comando registrado (RUST_MIN_STACK=33554432 onde aplicável).
content-preserving: zero código de produção, zero teste alterado, zero ficheiro
  de produto tocado. Suíte inalterada: 2719 / 3238.
lint: crystalline-lint . = 0 violations, 0 warnings.
lente: não medir delta (nada de produto mudou) — declarar inalterada.
```

---

## O que NÃO fazer

- **Não construir o fixpoint / loop de realização.** Este passo só mede.
- **Não evoluir nem alterar nenhum dos 18 testes.** A evolução, se houver, é do
  lote de execução **depois** da decisão do dono.
- **Não mudar comportamento de nenhum nativo.**
- **Não decidir o escopo do multi-passe sozinho.** O mapa vai ao dono; o escopo é
  decisão dele no checkpoint.
- **Não tocar o caso 3 / show-set** além de medi-lo para o mapa (é a fatia 3).
- **Não estimar com "~".** Cada veredito do mapa é uma saída medida.

---

## Relatório (`typst-passo-341-relatorio.md`)

- §0: por que este passo (o resumo acima — medir antes de construir).
- Fase A: a tabela dos 18 testes por balde.
- Fase B: o mapa de divergência completo, com comandos de compilação do vanilla e
  as saídas comparadas.
- Recomendação: multi-passe preciso? Se sim, os padrões divergentes e o conserto
  mais estreito; se não, a proposta de encerrar a fatia 2 e anotar a §3a.7-bis.
- Item aberto carregado: `content→elements → 0` continua **fora da fila** e **sem
  dono** (as três saídas: reconciliar o baseline / marco pós-F-6 / lacuna). Para
  decisão, não bloqueio.

---

## Nota de numeração (para o dono decidir)

A corrida da noite committou trabalho real sob o rótulo "Passo 340" (o caso 4 /
`f3s3`, correto e content-preserving). Por isso este passo está numerado **P341**,
para não duplicar o 340 no histórico (Responsabilidade Única). Se preferir
recuperar o número 340 — revertendo o commit do L0 da noite (`03619dc93`) e
mantendo o caso 4 — o corpo deste prompt serve igual, só mudando a pré-condição.
Recomendação: manter tudo o que a noite committou (é correto) e seguir como P341.
