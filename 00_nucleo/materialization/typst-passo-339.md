# Passo 339 — Lote F-realização, fatia 1: a fundação do transporte `StyledElem`-scoped (recon → checkpoint → código liderado por testes)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P339 (confirmar livre).
**Pré-condição**: P338 fechado — F-4 FECHADO (`6139e3517` — "Passo 338 — F-4
S4: registro e fecho"): backing único `StyleDelta`↔`Styled.Styles`, trava S3
contra o renascimento da dualidade, B2 fechado (`is_empty` arm `Styled`,
`content.rs:1566`). Suíte **declarar o número exato herdado** (ver C0), lint
0/0, árvore limpa. Se não, parar.
**Tipo**: caronas (commit próprio, código mínimo) + **Fase A — recon
dimensionador** (sem código de produção) + **TRAVA** + **F-realização fatia 1**
(execução, liderada por testes). Content-preserving: **nenhuma asserção
existente alterada para passar** — asserção que dependa de bug é achado a
registrar com decisão, não edição silenciosa.
**Objetivo**: construir a **fundação** do transporte `StyledElem`-scoped — o
mecanismo pelo qual `#set`/`#show` viajam confinados a um escopo e a
`StyleChain` fica disponível no nó durante a realização. Esta fatia constrói o
transporte e migra **só** os sítios da fatia 1 (definida na Fase A); **não**
des-assa os consumidores em massa (isso é F-5) e **não** toca folhas (F-6).
F-realização é a fundação que F-5 lê.
**Fontes**: o L0 da realização (`entities/f_fronteira_e1.md` §3 — guard/
lifecycle na camada `rules/` em invólucro transparente, conforme Trava-Q1 do
P334; se houver L0 dedicado da realização, ele), `f-plano-lotes-passo-333.md`,
o relatório do recon **P337** (a dimensão/ordem dos lotes restantes e a aresta
cara F-5↔F-realização), `f-spike2-show-passo-333.md` (os 5 casos),
`baseline-estrutural-lente-passo-338.md` (o "antes" da lente),
`lab/typst-original/` (leitura autorizada para semântica; **nunca importar**).
**Commits**: "Passo 339 — caronas (registro)" (1º, tree limpo) · "Passo 339 —
F-realização fatia 1: testes de caracterização" · "Passo 339 — F-realização
fatia 1: transporte" · "Passo 339 — F-realização fatia 1: consumidores da
fatia" (commits isoláveis; ver §Verificação).

---

## Caronas (commit próprio; antes do recon)

- **C0 — contabilidade da suíte herdada**: declarar no relatório o número
  exato da suíte ao fim do P338 (não "~"). **A pré-condição desta fatia é esse
  número** — fixá-lo antes de qualquer delta.
- **C1 — prova-de-mordida da dobra do F-4 (se ainda não coberta)**: a condição
  2 da decisão do F-4 pedia caracterização de `from_iter` com estilos da
  **mesma variante duplicados** afirmando last-write-wins (ex.: `[Bold(true),
  Bold(false)]` → o delta termina `false`). Se um dos testes do builder do F-4
  já cobre, **apontar** (`file:line`, Δ testes = 0). Se não, é um teste de
  poucas linhas — adicionar aqui (Δ exato declarado). Não atravessa a
  F-realização sem desfecho.
- **C2 — protocolo de perf canônico (P335 C1)**: a prova de não-regressão é o
  **par antes/depois back-to-back na mesma sessão**, mesmo corpus/método;
  absolutos entre sessões não se comparam. Esta fatia mantém o **release
  buildável por estágio** para o par medir o colapso isolado, não estágios
  somados (a armadilha registrada no P338: build quebrado no meio mistura os
  estágios na medição).

---

## Estágio 0 — A decisão do gatilho do spike-2 (antes de código)

O registro (c) do F-3 inc-1 armou: quando `#show` (e, por extensão, a
realização) entrar na cobertura da linguagem, os 5 casos do spike-2 viram
testes de **paridade**. A F-realização mexe no modelo eager dos **nativos** —
isto pode disparar o gatilho. A decisão não pode ficar implícita.

Definição de "medido" para este lote (a redefinição registrada na escalada do
F-3 inc-2, repetida aqui para não se perder): para a semântica da realização,
**"medido" = semântica extraída da fonte (`lab/typst-original/`) com
`file:line`**, porque rodar o vanilla isolado para cada caso é inviável.
**Gatilho de segundo nível**: se a leitura-da-fonte **conflitar** com o
comportamento observado no cristalino, ou for **ambígua** num caso concreto,
aí — e só aí — compila-se o vanilla para dirimir. Registrar a posição no L0.

Posição recomendada ao dono (decidir e registrar): dos 5 casos, os
**expressáveis com a superfície que a fatia 1 cobre** entram como testes de
paridade contra a semântica medida — critério de aceitação desta fatia. Os que
exigirem superfície de fatia posterior: registrar caso a caso (qual construção,
qual fatia a trará), gatilho continua armado com o ponteiro. Se o modelo eager
**falhar** um caso expressável: conforme o (c), a realização multi-passe
completa vira fatia própria — não consertar inline; registrar e parar.

---

## Fase A — recon dimensionador (com `file:line`; a lição do P331 vale aqui)

Sem código de produção. O entregável é o plano de toque com **números exatos
(zero "~")** e a confirmação do fatiamento. Mapear da fonte canônica:

1. **Onde a realização eager dos nativos vive hoje**: o `apply_show_rules` /
   `intercept_content` / o loop de realização, com os sítios que a fatia 1
   tocaria. A estimativa preliminar do recon P337 foi **~15 sítios + ~20
   testes em revisão consciente + fixpoint externo** — esta Fase A converte o
   "~" em número exato e diz **quantos** caem na fatia 1.
2. **O transporte `StyledElem`-scoped no vanilla** como referência de desenho:
   `content/mod.rs:744-752` (leitura autorizada). Como o vanilla confina
   `#set`/`#show` ao escopo do `StyledElem` e disponibiliza a chain no nó.
3. **O caminho de baking atual**: hoje `#set` **não** emite `Content::Styled`
   (`rules.rs:242/265/297`) — o valor chega ao elemento por baking; o consumo
   atual em `layout/mod.rs:1248`. Confirmar os endereços e mapear o que a
   fundação adiciona (chain disponível no nó) **sem** ainda religar os
   consumidores (F-5).
4. **O fixpoint externo**: localizar o laço de realização até fixpoint
   (terminação, profundidade), classificar se a fatia 1 o toca ou se ele fica
   para fatia posterior.
5. **Os ~20 testes em revisão consciente**: listar **um a um** (`file:line`)
   os testes existentes que asseveram comportamento de realização que o
   transporte da fatia 1 toca — para a varredura content-preserving da Fase B.
6. **Os 5 casos do spike-2**: reler cada um, classificar
   expressável-na-fatia-1 vs falta-superfície (insumo do Estágio 0).
7. **`f3s3` (teste-âncora da divergência)**: o que ele asservera exatamente, e
   **se a fatia 1 toca o comportamento que ele fixa**. Até aqui o guarda-corpo
   era "intocado até a F-realização executar" — esta é a execução, então o
   `f3s3` pode ser alcançado: se a fatia resolve a divergência que ele asservera,
   é **decisão registrada** (teste atualizado com a razão), não edição
   silenciosa; se não toca, fica intacto.
8. **O invólucro transparente (Trava-Q1)**: confirmar que o invólucro de
   guard/lifecycle nasce nesta fatia (ou apontar onde já nasceu). A condição do
   dono é que a transparência seja **provada por teste quando o invólucro
   nascer** — esse teste é da Fase B.
9. **L0 primeiro**: auditar o L0 da realização contra o código (desvio desde o
   P338? registrar); sincronizar o hash (`--fix-hashes`) **APÓS** qualquer
   ajuste, **antes** do código.

### Confirmação do fatiamento (saída da Fase A)

A proposta preliminar do recon foi fatiar por caso do spike-2: **fatia 1 = a
fundação do transporte** (`StyledElem`-scoped), seguida por **+1** e **+3**
(a leitura "4 → 1 → 3" a confirmar). A Fase A confirma ou revisa isso **sobre
os números reais**, e define o conteúdo exato da fatia 1. As fatias 2 e 3
**não** são executadas neste passo — viram P340/P341 quando os números da
fatia 1 forem conhecidos.

---

## TRAVA ARQUITETURAL — checkpoint após a Fase A

Emitir no chat: o plano de toque (arquivos × mudança), a **contagem exata** de
sítios da fatia 1, o fatiamento confirmado/revisado com a razão, a
classificação dos 5 casos do spike-2, o destino do `f3s3`, e qualquer
ambiguidade do L0. **Parar e esperar a decisão do dono se o mapa contradisser
qualquer premissa deste prompt** (precedente: o checkpoint do F-2). Ambiguidade
de desenho = parar; ambiguidade mecânica = decidir e registrar.

---

## Fase B — a fatia 1, liderada por testes (após o checkpoint; por estágio)

Ordem dos estágios (cada um commit isolável, release buildável ao fim de cada):

### Estágio T — testes primeiro (commit próprio)

1. **Caracterização**: para cada sítio de realização da fatia 1 (lista da
   Fase A §5), um teste que captura o comportamento **atual** (a saída de hoje)
   antes de o transporte mudar. Estes testes ficam verdes durante toda a fatia
   se a mudança for content-preserving.
2. **Transparência do invólucro (condição Trava-Q1)**: teste de que
   `plain_text` / `is_empty` / `map_*` e closures de usuário **não veem** o
   invólucro de guard/lifecycle — nativo e dinâmico tratados igual.
3. **Paridade spike-2** (os casos expressáveis na fatia 1, do Estágio 0):
   afirmar igualdade contra a semântica medida (`file:line` do vanilla; ou
   compilação do vanilla se o gatilho de 2º nível disparar).

### Estágio C — o transporte (commit próprio)

4. Construir a fundação `StyledElem`-scoped: `#set`/`#show` viajam confinados
   ao escopo; a `StyleChain` fica **disponível no nó** durante a realização.
   Aditivo — a chain fica disponível; o baking atual **não** é removido aqui
   (remoção = F-5). Conforme o desenho do vanilla (§A.2) e o L0.

### Estágio M — os consumidores da fatia (commit próprio)

5. Migrar **só** os sítios da fatia 1 a lerem do transporte confinado, na
   medida em que a fatia os define. Os demais consumidores ficam no baking até
   o F-5. Os testes de caracterização (Estágio T) **permanecem verdes** —
   strong/emph e o resto content-preserving.

### Estágio F — `f3s3` e linhagem (commit próprio, se aplicável)

6. `f3s3`: conforme o destino decidido no checkpoint — resolvido com decisão
   registrada, ou intacto.
7. Linhagem: arquivos novos declaram `@prompt`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates da fatia)

```
cargo build limpo (a cada estágio — release buildável por estágio, C2)
suíte com RUST_MIN_STACK=33554432: (C0) + N novos — N exato, zero asserções
  existentes alteradas
crystalline-lint . = 0 violations, 0 warnings

Lente (tekt-cargo-dsm@98d8f9e, comandos do baseline P338):
  --comparar --antes <P338/6139e3517> --depois <esta fatia>
  Expectativa REGISTRADA (não surpresa): a F-realização opera dentro do
  megaciclo de 90 — a assinatura NÃO é queda de contagem de ciclos (isso é o
  corte content→elemento, outro marco, fora deste lote). O efeito esperado é
  delta de ARESTA dentro do megaciclo, ou ~nulo declarado. Verificar também:
  content→elements::* permanece 66 (esta fatia não o toca); 0 arestas
  elemento→elemento mantido. Registrar os números.

Perf: par back-to-back mesma sessão (C2), mesmo corpus/método; nativos sem
  regressão no caminho quente de realização. Reportar o par.

L0 auditado e hash sincronizado ANTES do código (Fase A §9).
Commits isoláveis: caronas / Estágio T / Estágio C / Estágio M / Estágio F.
```

---

## O que NÃO fazer

- **Não des-assar os consumidores em massa** — isso é F-5. Esta fatia constrói
  o transporte e migra só os sítios da fatia 1; o baking dos demais permanece.
- **Não tocar folhas** — F-6.
- **Não remover os imports `use elements::*Elem` de `content.rs`** — esse é o
  corte estrutural (content→elements → 0) que o baseline mede; ele **não está
  nesta fila de lotes** e fica fora de escopo aqui. (Achado a registrar/decidir
  em separado: a fila F-1…F-6 + F-realização não inclui esse corte; o baseline
  o espera. Não resolver dentro deste lote.)
- **Não executar as fatias 2 e 3 da F-realização** — P340/P341, com os números
  da fatia 1 em mãos.
- **Não alterar asserção existente para passar** — content-preserving;
  dependência de bug é achado com decisão.
- **Não deixar representação morta "por segurança"** — morto-alimentado mascara
  (lição S5b); se a fundação criar caminho duplo com o baking, o caminho duplo
  nasce com gatilho de remoção escrito (o F-5 remove) e teste de paridade entre
  os dois enquanto coexistirem (lição C1 do P338).
- **Não estimar com "~" no relatório final** — os "~15"/"~20" do recon viram
  número na Fase A.

---

## Relatório (`typst-passo-339-relatorio.md` + resumo no chat)

- Caronas: C0 (número da suíte), C1 (prova-de-mordida apontada ou adicionada),
  C2 (protocolo de perf aplicado).
- Estágio 0: a decisão do gatilho gravada (quais casos do spike-2 entram agora,
  quais ficam com ponteiro); a redefinição de "medido" e o gatilho de 2º nível
  no L0.
- Fase A: o plano de toque real vs previsto (os ~15/~20 viram exatos); o
  fatiamento confirmado/revisado com a razão; a classificação dos 5 casos; o
  destino do `f3s3`.
- Fase B: o diff por estágio (T/C/M/F); o teste de transparência (Trava-Q1)
  descrito; os testes de caracterização e paridade; o estado do `f3s3`.
- Verificação: suíte (C0 + N), lint 0/0, lente antes/depois (números, com a
  expectativa de aresta-não-ciclo confirmada ou a divergência explicada), perf
  par back-to-back.
- **Contabilidade do F**: F-realização fatia 1 fechada; restam as fatias 2 e 3
  da F-realização (P340/P341), depois F-5 (de-bake) e F-6 (folhas). O corte
  content→elemento registrado como **fora da fila atual** (decisão pendente do
  dono sobre se é marco pós-F-6 ou lacuna do plano).
- `git log --oneline` (commits isoláveis); `git status` limpo fora de docs.
- Caveat do stack (`RUST_MIN_STACK=33554432`).
