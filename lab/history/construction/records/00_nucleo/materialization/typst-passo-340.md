# Passo 340 — Lote F-realização fatia 2: a realização multi-passe dos nativos (execução autônoma)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P340 (confirmar livre).
**Pré-condição** (única parada legítima): P339 fechado (`fc7bfbf78` — F-realização
fatia 1, transporte aditivo β1): suíte **2719** (`typst-core --lib`) / **3238**
(workspace), lint 0/0, árvore limpa. Se o estado-base não bater, **parar e
relatar** — base errada é a única coisa que justifica não começar.
**Tipo**: **Lote F-realização fatia 2** — **execução autônoma, sem paradas de
checkpoint** (modo aprovado pelo dono para esta corrida). Onde o molde manda
"TRAVA / parar para o dono", este passo **substitui** por: resolver por medição →
decidir → registrar. Content-preserving estrito e **inviolável**.
**Objetivo**: materializar a camada de realização multi-passe do `#show` sobre os
nativos, em três frentes:
- **Caso 4 / `f3s3`**: confinar o escopo do `#show` no `ContentBlock`
  (`eval/mod.rs:454-461` vaza; `CodeBlock` `:398-399` confina) — `f3s3` **vira**
  (de divergente a confinado), como **decisão registrada**.
- **Casos 1/2**: o **loop até fixpoint** (composição multi-regra) + o **guard
  por-nó com terminação limitada** (recursão de regra sobre o próprio output) —
  o invólucro transparente da **Trava-Q1 nasce aqui**.
- Completa o aspecto multi-passe do **caso 5** (nativo+dyn numa chain; a
  uniformidade da chain veio na fatia 1, os "2 passes" são deste lote).
**Fora**: caso 3 (show-set, precisa `Transformation::Style`) → **fatia 3 (P341)**.
De-bake dos consumidores → **F-5**. Os 3 consumidores continuam lendo o campo
assado (`layout:714`, `:812`, `introspect:817`).
**Fontes**: L0 `entities/f_fronteira_e1.md` (§3a.7 a materializar; §3a.8 da fatia
1), `f-spike2-show-passo-333.md` (os 5 casos, S1–S7), relatório P339,
`f-plano-lotes-passo-333.md`, `lab/typst-original/` (**leitura E compilação
autorizadas** para "medido"/gatilho de 2º nível; **nunca importar de `lab/`**),
baseline lente (`tekt-cargo-dsm@98d8f9e`) + estado P339 (`fc7bfbf78`).
**Commits** (isoláveis por estágio, para reversão limpa): "Passo 340 — caronas" ·
"Passo 340 — L0 §3a.7 realização" · "Passo 340 — guard + terminação" · "Passo 340
— caso 4 / f3s3 confinado" · "Passo 340 — fixpoint / composição" · "Passo 340 —
paridade + linhagem".

---

## MODO AUTÔNOMO — as regras que substituem as paradas

Este passo roda sem supervisão. As regras abaixo são o que protege o trabalho no
lugar do checkpoint humano. Seguir à risca.

1. **Sem paradas de checkpoint.** Onde o molde diria "emitir checkpoint e esperar
   o dono", fazer: resolver por medição, decidir, registrar no relatório. Não
   esperar resposta.
2. **Ambiguidade mecânica** → decidir e registrar (regra de sempre).
3. **Ambiguidade de desenho** → resolver por **medição**, nesta ordem:
   a. Ler o vanilla (`lab/typst-original/`) com `file:line` — "medido" = semântica
      da fonte.
   b. Se a leitura for **ambígua** ou **conflitar** com o comportamento do
      cristalino, **compilar o vanilla** e observar a saída (gatilho de 2º nível —
      agora um passo mecânico autônomo, não um pedido ao dono). Registrar o comando
      e a observação.
   c. Se a medição **decide**: seguir pelo caminho medido; registrar decisão +
      evidência (`file:line` / saída do vanilla).
   d. Se a medição **não decide** (sem discriminador medível): escolher a opção
      **mais conservadora e reversível**, isolar em commit próprio, marcar no
      relatório como **DECISÃO AUTÔNOMA PROVISÓRIA — revisar**, e seguir.
4. **Content-preserving é inviolável.** Nenhuma asserção existente é alterada. A
   **única exceção** é o `f3s3`, que o plano sempre marcou para virar quando a
   F-realização executar — atualizá-lo conta como **decisão registrada** (com a
   razão escrita), não edição silenciosa. Qualquer outra asserção que uma mudança
   exigiria alterar é **achado**: NÃO alterar, **estacionar** a sub-parte que a
   tocaria, registrar, seguir o resto.
5. **Terminação obrigatória.** O loop de fixpoint **nunca** existe sem teto de
   profundidade (eco do depth-64 do vanilla — confirmar o valor na fonte). O
   **teste de terminação** (uma regra que recursaria infinito bate no teto e
   para/erra com diagnóstico, não trava) é o **primeiro** teste, escrito **antes**
   do loop. Sem ele, o loop não entra.
6. **Degradação segura em vez de parada.** Se uma sub-parte não fechar com
   segurança (ex.: composição multi-regra com paridade que não bate), **aterrar o
   que é seguro** (ex.: caso 4 + guard + terminação), **estacionar** o resto em
   commit não-feito, e relatar — **não** forçar, **não** parar tudo.
7. **L0 autorizado.** Redigir o L0 da realização (§3a.7 materializado + spec do
   invólucro de guard), rodar `crystalline-lint --fix-hashes`, e seguir. Registrar
   o desenho do L0 no relatório para revisão de manhã. (Delegação do dono: redigir
   + selar o hash + seguir, sem parar antes do código.)
8. **Disciplina herdada (mantida):** recon com `file:line` (lição P331); contagens
   exatas, sem "~"; nada de caminho morto-alimentado (lição S5b); cada caminho
   duplo nasce com gatilho de remoção (F-5) + teste de paridade enquanto coexistir.

---

## Caronas (commit próprio)

- **C0 — base exata**: confirmar a suíte herdada do P339 = **2719** core / **3238**
  workspace em `fc7bfbf78`. Fixar antes de qualquer delta.
- **C1 — gatilho do `PartialEq=false` da fatia 1**: confirmar que a §3a.8 registrou
  o gatilho de reabertura do wrapper β1 ("se igualdade-de-`Content` virar requisito,
  reavaliar o wrapper"). Se não está lá, adicionar a frase (Δ código 0). Não deixar
  o achado da fatia 1 sem o gatilho escrito.

---

## Fase A — recon dimensionador (autônomo; `file:line`; sem "~")

Mapear da fonte canônica antes do código:

1. **A realização eager hoje**: `apply_show_rules` (`rules.rs:69`),
   `intercept_content` (`:193`), `eval_show_rule` (`:507`), `show_rules=Arc::from`
   (`:588`); **como o guard atual dos nativos chaveia** (por endereço de função
   hoje — o que existe, onde, e o cap se houver).
2. **Caso 4 / `f3s3`**: a assimetria `eval/mod.rs:398-399` (CodeBlock confina) vs
   `:454-461` (ContentBlock vaza). O que confinar o ContentBlock toca; **os testes
   que dependem do vazamento** (o `f3s3` e quaisquer outros) — listar por
   `file:line`.
3. **Casos 1/2 no vanilla** (`lab/`): como o vanilla faz o **loop multi-passe**
   (ordem innermost-first?), o **guard por-nó** (chave do guard, o que impede uma
   regra de recursar no próprio output), e a **terminação** (o cap — confirmar o
   valor). `file:line`. Onde a leitura for ambígua, **compilar o vanilla** e
   registrar a saída.
4. **Trava-Q1 decidida** (agora com os requisitos do multi-passe na mão):
   `Content::Guarded` transparente **vs** campo meta no `Dynamic` — decidir **pela
   fonte/medição** (qual representação o multi-passe exige; qual mantém o trait do
   usuário limpo e o invólucro transparente a `plain_text`/`is_empty`/`map_*`/
   closures). Registrar a escolha + a razão. (Esta decisão foi deferida da fatia 1
   exatamente para este ponto.)
5. **Os testes de `#show`**: os 20 existentes (15 `show_rule_*` + 4 `f3s2_*` + 1
   `f3s3`) e quais a mudança toca; listar.
6. **L0 §3a.7**: o que está deferido lá (camada de realização, guards, multi-passe)
   e redigir a materialização concreta (o mecanismo do loop, o guard, a
   terminação, a transparência do invólucro).

**Não emitir checkpoint.** Seguir direto para o L0 e a Fase B.

---

## Fase B — estágios (testes primeiro em cada; commits isoláveis)

### Estágio L0 — redigir e selar (commit próprio)
Materializar `f_fronteira_e1.md` §3a.7: o mecanismo do loop até fixpoint, o guard
por-nó (a representação decidida na Fase A §4), a terminação limitada, e a
transparência do invólucro. `crystalline-lint --fix-hashes`. Registrar o desenho.

### Estágio G — guard + terminação (testes primeiro)
1. **Teste de terminação** (o primeiro de tudo): uma regra `#show` que recursaria
   sobre o próprio output bate no teto de profundidade e **para com diagnóstico**
   (não trava, não estoura a stack). Confirmar que **falha** antes do código.
2. **Teste de transparência (Trava-Q1)**: `plain_text` / `is_empty` / `map_*` e
   closures de usuário **não veem** o invólucro de guard — nativo e dinâmico
   iguais. Confirmar que **falha** antes do código.
3. Código: o invólucro de guard (a representação da Fase A §4) + o cap de
   profundidade. Os dois testes passam.

### Estágio 4 — caso 4 / `f3s3` confinado (testes primeiro)
1. **Paridade do escopo** (medida do vanilla): o `#show` dentro de um
   `ContentBlock` **não vaza** para fora do escopo — afirmar contra a semântica
   medida (`file:line` do vanilla; compilar se ambíguo). Confirmar que **falha**
   antes do código.
2. Código: confinar o `ContentBlock` (`eval/mod.rs:454-461`), espelhando o
   `CodeBlock` (`:398-399`).
3. **`f3s3` vira**: atualizar o teste-âncora da divergência com a **decisão
   registrada** (de "vaza, 2×" para "confina, 1×"; a razão escrita no teste e no
   relatório). Esta é a exceção autorizada à regra content-preserving.

### Estágio F — fixpoint / composição multi-regra (testes primeiro)
1. **Paridade casos 1 e 2** (medida): composição de múltiplas regras `#show`
   converge ao mesmo resultado do vanilla; recursão controlada termina como o
   vanilla. Afirmar contra a semântica medida. Confirmar que **falham** antes.
2. Código: o loop até fixpoint, usando o guard + a terminação do Estágio G.
3. **Se a paridade não bater** e a medição não resolver: **estacionar** a
   composição multi-regra (degradação segura — regra 6), aterrar Estágios G + 4
   (que são independentes e fecham sozinhos), e relatar o caso 1/2 como sub-parte
   não-feita com a evidência do conflito.

### Estágio P — paridade + linhagem (commit próprio)
Reavaliar os 5 casos do spike-2: 1/2/4 com o resultado (paridade ou estacionado);
5 (multi-passe) confirmado; **3 confirmado como fatia 3 (P341)** com o ponteiro.
Linhagem: ficheiros novos declaram `@prompt`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates)

```
build: limpo a cada estágio.
suíte (RUST_MIN_STACK=33554432): C0 (2719/3238) + N — N exato. Zero asserções
  existentes alteradas, EXCETO f3s3 (decisão registrada). Zero regressão.
lint: crystalline-lint . = 0 violations, 0 warnings.

terminação: o teste de teto verde — nenhuma corrida trava nem estoura a stack.

lente (tekt-cargo-dsm@98d8f9e, comandos do baseline; --comparar --antes
  fc7bfbf78 --depois <fecho>):
  Expectativa registrada: a realização toca show/introspecção (o anel de
  numeração introspect↔content). Esperar delta de ARESTA no megaciclo;
  content→elements permanece 66; elem→elem 0. Se a contagem de CICLOS mudar,
  isso é ACHADO a explicar no relatório (a realização pode mexer no anel
  introspect↔content) — registrar, não esconder. Registrar os números.

perf (C2 — par back-to-back, mesma sessão): o multi-passe ADICIONA passes; medir
  o caminho quente com #show presente. Reportar o par. Regressão NÃO é bloqueio
  overnight nem se "conserta" cortando caminho — é ACHADO a relatar com destaque
  (o salto eager→multi-passe tem custo esperado; o que importa é se é limitado).
  Docs SEM #show devem ter caminho idêntico (custo zero) — confirmar.

commits isoláveis: caronas / L0 / G / 4 / F / P (reversíveis um a um).
```

---

## O que NÃO fazer

- **Caso 3 / show-set** (`Transformation::Style`) → fatia 3 (P341). Não tocar.
- **De-bake** dos consumidores (`layout:714`/`:812`/`introspect:817`) → F-5. Não
  religar; o baking permanece autoritativo.
- **Alterar asserção existente** que não seja o `f3s3` → achado, estacionar a
  sub-parte, relatar. Nunca silenciar.
- **Rodar o fixpoint sem teto** → proibido; o teste de terminação é pré-requisito.
- **Deixar caminho morto-alimentado** (lição S5b) → cada caminho duplo nasce com
  gatilho de remoção (F-5) + paridade.
- **Otimizar perf preventivamente** → medir e relatar; não cortar caminho para
  esconder custo do multi-passe.
- **Estimar com "~"** → números exatos no relatório.

---

## Relatório final (`typst-passo-340-relatorio.md`) — o entregável do modo autônomo

Estruturar para a revisão de manhã, com estas seções explícitas:

- **A. Decisões autônomas com medição** — cada decisão de desenho tomada, com a
  evidência (`file:line` do vanilla / saída de compilação / número). Para
  ciência.
- **B. DECISÕES PROVISÓRIAS — revisar** — onde a medição não decidiu e foi tomada
  a opção conservadora reversível. Para o dono confirmar ou reverter (o commit
  isolável de cada uma apontado).
- **C. Sub-partes estacionadas** — o que não foi feito, por quê, e o que falta para
  fazer (especialmente se a composição multi-regra foi estacionada).
- **D. Achados** — regressão de perf (com os números), qualquer asserção que teria
  de mudar e foi parada, mudança de contagem de ciclos na lente se ocorreu.
- **E. Trava-Q1** — a representação do guard escolhida, a razão, e a prova de
  transparência (o teste e o resultado).
- **F. Terminação** — o valor do teto, o teste, e que nenhuma corrida travou.
- **G. Estado da fila** — fatia 2 fechada (ou parcial, com C); resta P341 (caso 3),
  F-5 (de-bake), F-6 (folhas).
- **H. Item aberto carregado** — `content→elements → 0` continua **fora da fila** e
  **sem dono**: o baseline mede `target=0` que nenhum lote entrega. Repetir as três
  saídas (reconciliar o baseline / nomear marco pós-F-6 / registrar lacuna) — para
  decisão, não bloqueio.
- **I. Verificação** — suíte (C0 + N), lint, lente antes/depois (números), perf
  (par), terminação. `git log --oneline` (commits isoláveis). Caveat de stack.
