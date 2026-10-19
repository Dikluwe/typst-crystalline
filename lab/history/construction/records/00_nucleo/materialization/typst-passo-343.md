# Passo 343 — Inventário do bake (medição; o de-bake decide-se sobre os números)

> **Propósito.** Mapear, da fonte com `file:line`, **todo** campo de estilo/valor
> assado nos nós de `Content` — não só o `TextStyle` do Achado 2. O entregável é
> a tabela completa (campo × onde assa × consumidor × lê-da-chain-ou-do-nó × testes
> que asseveram × interseção com o multi-passe) + o veredito de vazamento de cada
> um. O fatiamento do de-bake decide-se **sobre esses números**, na trava — não
> agora. Não conserta nada.

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P343 (confirmar livre).
**Pré-condição**: HEAD = `3a02223f9` (último do P341b); produto content-idêntico
ao P340 (P341/P341b/P342 só adicionaram `.md`). Suíte **2719** (`typst-core --lib`)
/ **3238** (workspace), lint 0/0, árvore limpa. Se não bater, parar.
**Tipo**: **inventário / medição** — content-preserving estrito: **zero código de
produção, zero teste alterado, zero ficheiro de produto tocado**. Termina na
**TRAVA** com a tabela, para o dono decidir o fatiamento. **Não** des-assar nada,
**não** consertar a igualdade, **não** fixar o fatiamento sozinho.
**Objetivo**: o inventário completo do bake — saber **quantos** campos assados
existem, **quais** vazam para a linguagem, **quais** o multi-passe (F-realização
restante) ainda vai tocar, e **qual** a ordem segura de des-assar. É o gêmeo do
recon do P339, agora para o F-5.
**Fontes**: o relatório P342 (a raiz `mod.rs:310-311` + `markup.rs:85` + o consumo
redundante `layout/mod.rs:1293`; o segundo vazamento β1 via `==` em `content.rs:1790`),
o relatório P337 (a interseção F-5↔F-realização e o critério "não des-assar duas
vezes"), o relatório P339 (os 3 consumidores de numbering deixados no baking:
`layout:714`, `:812`, `introspect:817`), `f_fronteira_e1.md` §3b (a chain), os L0
do modelo de conteúdo, `lab/typst-original/` (vanilla: onde cada propriedade vive
— criação vs chain — `file:line`; compilar se ambíguo).

---

## Fase A — o inventário do bake (com `file:line`; sem código)

Mapear da fonte canônica. O entregável é uma tabela; cada linha é um campo assado.

1. **Enumerar todo campo de estilo/valor assado num nó de `Content`.** Começar
   pelos conhecidos e varrer para achar o resto:
   - `Content::Text` carrega `TextStyle` (a raiz do Achado 2, `mod.rs:310-311`);
   - os 3 de numbering deixados no baking pela fatia 1 (`heading.numbering_active`
     `layout:714`, `equation` `:812`, `heading` auto-TOC `introspect:817`);
   - `Content::Styled` do β1 (numbering local, `content.rs:1790`);
   - qualquer outro campo de elemento que seja estilo/valor resolvido na criação
     em vez de lido da chain (varrer os 65 módulos de elemento + `eval/markup.rs`
     pelos pontos que assam estado ativo de estilo).
2. **Para cada campo:** onde é assado (`file:line` da escrita), quem o consome
   (`file:line` da leitura), e se o consumidor **lê do nó** (assado) ou **da
   chain**. (O `TextStyle` é o caso onde o layout re-deriva da chain de qualquer
   forma — `layout:1293` — logo o assado é transporte redundante; checar se há
   outros assim.)
3. **Vazamento (cruzar com a separação do P342):** para cada campo, ele é
   observável pela linguagem (`==`, `it.body`/acesso a campo, `query`, matching de
   `#show`)? Vazamento `render→linguagem` = sim. O `TextStyle` vaza (medido); o β1
   vaza estreito via `==`; medir os demais.
4. **Testes que asseveram cada campo:** listar `file:line` os testes que fixam o
   comportamento de cada campo assado — os que o de-bake desse campo tornaria
   vermelhos (a base da evolução-contra-vanilla quando o de-bake correr).
5. **Interseção com o multi-passe (o critério do P337):** para cada consumidor que
   o de-bake religaria à chain, ele é um ponto que a recursão/F-realização restante
   ainda vai tocar? (Reconfirmar da fonte; o relatório P337 mapeou isso, mas a
   ordem agora está invertida — de-bake antes do resto do F-realização.) Os pontos
   da interseção **esperam**; os fora dela são des-assáveis já.
6. **O vanilla como referência:** para cada campo, onde o vanilla guarda a
   propriedade (criação vs chain/realize) — `file:line` — para saber a forma fiel
   do de-bake.

---

## TRAVA ARQUITETURAL — checkpoint com a tabela

Emitir a tabela completa do inventário do bake + estes vereditos, para decisão do
dono:
- **Quantos** campos assados existem (número exato, sem "~").
- **Quais vazam** para a linguagem (a lista, com a face do vazamento).
- **Quais estão na interseção** com o multi-passe (esperam) **vs** quais são
  des-assáveis com segurança já.
- **Proposta de fatiamento do de-bake** (a forma de opções, o dono decide):
  - candidato a fatia 1: o `TextStyle` (a raiz do Achado 2, o caminho crítico da
    recursão, e — pelo P342 — transporte redundante porque o layout re-deriva);
  - os demais campos agrupados por afinidade/risco, cada grupo uma fatia;
  - para cada fatia: os testes que o vanilla vai contradizer (a evolução um a um) e
    os que ficam intactos.
- **Caráter do de-bake declarado:** não é content-preserving — muda comportamento
  de propósito (ex.: `it.body == [a]` passa a casar). Roda com a regra do P340:
  paridade contra o vanilla compilado, testes contraditos evoluídos um a um com a
  saída do vanilla colada como justificativa; os que o vanilla confirma, intactos.

Parar aqui. Nenhum código de produção, nenhum teste alterado.

---

## Verificação (gates)

```
content-preserving: zero código de produção, zero teste alterado, zero ficheiro
  de produto tocado. Suíte inalterada: 2719 / 3238.
lint: crystalline-lint . = 0 violations, 0 warnings.
medição reproduzível: greps e leituras registrados; compilações do vanilla com
  comando (binários do P341); zero "~" na tabela final.
lente: read-only se usada; inalterada (nada de produto muda).
```

---

## O que NÃO fazer

- **Não des-assar nada** — só medir.
- **Não consertar a igualdade**, não tocar o guard, não construir o multi-passe.
- **Não fixar o fatiamento sozinho** — a tabela vai ao dono; o fatiamento é decisão
  dele na trava (precedente: o checkpoint do P339).
- **Não confiar em relatório de continuidade** sobre o que está assado — **ler a
  fonte**. (Esta deriva — confiar na narrativa em vez do código — é a que perdeu o
  rastro do bake do Passo 30 por ~70 passos; o inventário existe para não a repetir.)
- **Não estimar com "~"** — cada número é medido ou é uma nota de por que não dá
  para contar ainda.

---

## Relatório (`typst-passo-343-relatorio.md`)

- A tabela do inventário do bake (campo × assa × consome × nó/chain × vaza × testes
  × interseção × vanilla).
- Os vereditos: total de campos, quais vazam, quais esperam pela interseção, quais
  são des-assáveis já.
- A proposta de fatiamento do de-bake (opções, para o dono).
- **Mapa de filtro (campo novo, daqui em diante):** o lugar lógico deste passo na
  versão destilada — "o inventário do bake vem junto com a introdução do `#set`,
  porque é onde a forma assado-vs-chain se decide" — com a nota de onde o
  conhecimento entrou de verdade (a raiz marcada no Passo 22, o bake construído no
  Passo 30, o rastro perdido até o 102, a face de igualdade só no P342).
- Item aberto carregado: `content→elements → 0` (fora da fila, sem dono — as três
  saídas), para decisão, não bloqueio.
