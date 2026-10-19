# Passo 347 — recursão de `#show`: revisitação (semântica) + terminação (teto + flag de erro completo)

> **O que fecha.** O resíduo do `m1` (P341b/P345): o cristalino trunca a recursão de
> `#show` no nível 1 (guard por-regra); o vanilla deixa o output ser revisitado e
> recursa até parar ou até o teto. A diferença é **semântica** (muda a saída: `m1`="b"
> vs "c") → paridade real (ADR-0107). Este lote dá ao cristalino a **semântica de
> revisitação** e uma **terminação** que reproduz o comportamento do vanilla por
> padrão (infinito → erro com a mensagem do vanilla) e, **sob flag**, classifica o
> erro (cíclico / divergente / converge-fundo) num canal separado. **NÃO**
> content-preserving — muda a semântica de `#show` de propósito.

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P347 (confirmar livre).
**Pré-condição**: P346 fechado (marco G registrado; item `content→elements` redirecionado).
HEAD pós-P346; P345 commitado (`3ebb397fb`). Suíte **2723** (`typst-core --lib`) /
**3242** (workspace), lint 0/0, árvore limpa. Se não bater, parar.
**Tipo**: recursão de `#show` — **NÃO content-preserving**: muda a semântica de
terminação/revisitação de propósito. Regra do P340 + **ADR-0107**: paridade e
aceitação **morfológicas** (a morfologia da árvore final = a do vanilla, para
composição e recursão limitada; recursão infinita → mesmo erro observável),
**nunca** "o booleano bate" nem diff de bytes. O vanilla é oráculo da **semântica**.
**Limites duros**: **não** tocar o `==` morfológico (P345, fechado); **não** mexer no
`PartialEq` do Rust; o de-bake (F-5) **só** entra se a Fase A provar que não se cruza
com a recursão (senão é lote seguinte).
**Objetivo**: (1) a realização de `#show` deixar o **output de uma regra ser
revisitado** pelas regras (reproduz a semântica de composição/recursão do vanilla);
(2) **terminação**: por padrão, recursão que não para vira **erro com a mensagem base
idêntica à do vanilla**; um **backstop** (teto, número = mecânica) garante que não
trava; (3) uma **flag de erro completo** que, ligada numa re-rodada, **classifica**
(cíclico / divergente / converge-fundo) e emite o detalhe **num canal separado**
(hint), sem alterar a mensagem base.
**Fontes**: relatórios P340 (o eager bate em content-preservation; o multi-passe
estacionado; o desenho do vanilla mapeado no L0 §3a.7-bis: `realize→visit→
visit_show_rules` `lib.rs:43/224/335`, fixpoint externo `:380`, recipe-index
innermost-first `:472`, teto `:401-402`, `visit_styled` `:574`), P341b (o guard
por-regra vs por-conteúdo; o teto-64 `world_types.rs:249`; o `m1`), P345 (o `==`
morfológico — usado para detectar ponto fixo/ciclo), P337 (a interseção
F-5↔F-realização, o critério "não des-assar duas vezes / não migrar consumo para um
modelo que a recursão vai substituir"), **ADR-0107** (paridade morfológica),
`lab/typst-original/` (vanilla — oráculo da semântica de terminação e da **mensagem
base** exata; `file:line`; compilar onde ambíguo).
**Commits** (isoláveis): "Passo 347 — caronas" · "Passo 347 — Fase A (4 medições)" ·
"Passo 347 — L0 da realização (se nascer/mudar superfície)" · "Passo 347 —
revisitação" · "Passo 347 — terminação + backstop" · "Passo 347 — flag de erro
completo (classificação)" · "Passo 347 — evolução de testes vs semântica".

---

## Caronas (commit próprio)
- **C0 — base exata**: confirmar 2723 / 3242 no HEAD pós-P346; P345 commitado.
- **C1 — o `m1` como âncora**: registrar o `m1` atual (`= a` → "b") como o teste que
  este lote move para "c" (paridade com o vanilla). É a prova morfológica do lote.

---

## Fase A — as quatro medições que decidem o lote (`file:line`; sem código de produção)

A Fase A mede quatro coisas; a trava decide o escopo sobre elas. Probes descartáveis
removidas ao fim (tree limpo).

### M1 — a semântica de terminação do vanilla (o que reproduzir)
Da fonte (`lab/`, `typst-realize/src/lib.rs`) + compilação:
- **Revisitação**: como o output de uma regra é reprocessado — o output nasce fresco
  (sem o guard) e é revisitado; a ordem (innermost-first, `:472`); o guard por-conteúdo
  (recipe-index no output, `:366/:471-473`) que impede só a re-aplicação da **mesma**
  regra ao **próprio** output. Registrar a semântica, não o mecanismo.
- **Terminação**: o teto-64 (`MAX_SHOW_RULE_DEPTH`, `:401-402`) — o **comportamento**
  (estoura → erro) e a **mensagem base exata** (texto literal, `file:line`; e se o
  vanilla usa um canal de **hint** separado da mensagem principal — é o que decide
  onde a flag escreve). Casos-oráculo: `m1` (a→b→c, converge), uma regra que cicla
  (a→b→a), uma que diverge (cresce sem repetir), uma recursão legítima profunda.

### M2 — a forma de detecção (teto puro vs ponto-fixo+ciclo)
Medir o custo de cada uma, para a trava decidir:
- **teto puro** (incrementa contador, erra em N): barato, caminho quente intacto; é o
  do vanilla; não distingue ciclo de divergência sem a análise.
- **ponto-fixo + ciclo** (usa o `==` morfológico do P345: para quando um passe não muda
  a morfologia; detecta ciclo guardando as morfologias do caminho): mais bonito (parada
  semântica, sem número mágico) mas paga `==` por passe e guarda histórico.
- **híbrido (a ideia do dono)**: teto barato no caminho quente como **backstop**; a
  análise (ponto-fixo/ciclo, classificação) corre **só quando o teto dispara** (caso já
  falho) e **só sob a flag**. Recomendado a priori; a M2 confirma o custo.

### M3 — cabe no modelo eager ou exige a camada multi-passe?
A medição que o P340 deixou pendente. A revisitação cabe num **loop local até ponto
fixo na criação** (eager estendido), ou exige a **camada de realização multi-passe**
do vanilla (o lote grande que o P340 estacionou)? Medir da fonte o que a revisitação
toca. **Se eager**: lote menor, executa aqui. **Se multi-passe**: dimensionar (pontos
de toque, testes afetados) e **a trava decide** se executa agora ou vira lote próprio
— um multi-passe é mudança de mecanismo grande, decisão do dono.

### M4 — interseção com o de-bake (F-5)
O critério do P337, com a ordem atual (recursão antes do de-bake): a revisitação muda
*quando* os elementos são realizados; o de-bake muda *onde se lê*. Mapear, da fonte, se
os pontos que a recursão toca/reorganiza são os mesmos que o de-bake religaria. **Se
disjuntos**: o de-bake fica liberado para o lote seguinte sem risco (registrar). **Se
cruzam**: a recursão vai primeiro (este lote), o de-bake depois, e os pontos da
interseção ficam anotados para o de-bake não religar para um modelo que esta recursão
muda.

**Saída da Fase A**: M1 (a semântica + a mensagem base + o canal de hint), M2 (a forma
de detecção escolhida com o custo), M3 (eager vs multi-passe, com dimensão se
multi-passe), M4 (a interseção com o de-bake). Sem código antes disto.

---

## TRAVA ARQUITETURAL — checkpoint com as quatro medições

Emitir M1–M4 e **parar** se:
- **M3 = multi-passe** → o lote vira grande (mudança de mecanismo); o dono decide
  executar agora ou fatiar. (Fronteira de decisão — multi-passe é caro e foi estacionado
  uma vez.)
- a mensagem base do vanilla (M1) for ambígua, ou o canal de hint não existir no vanilla
  (a flag precisaria de um canal novo — superfície sem L0 → fronteira do L0).
- M4 mostrar interseção que mude a ordem.
**Seguir sem parar** se M3 = eager, a mensagem base é clara, e M4 disjunto — o lote
executa a revisitação eager + terminação + flag.

---

## Fase B — execução (após a trava; só o que ela liberar)

### Estágio L0 — a realização (se M3/M1 criaram superfície)
Materializar/atualizar o L0 da realização (`f_fronteira_e1.md §3a.7`): a semântica de
revisitação, a terminação (comportamento, não o número), a flag de erro completo e o
canal de hint. `--fix-hashes`, e **parar** para o dono selar se for superfície nova
(fronteira do P339).

### Estágio R — revisitação (testes primeiro, morfológicos)
1. Teste morfológico: composição/recursão limitada produz a **mesma morfologia** que o
   vanilla (o `m1` → "c"). Confirmar que **falha** antes.
2. Código: o output de uma regra passa a ser revisitado (loop até ponto fixo, eager —
   ou a forma que M3 liberou), com o guard que impede só a re-aplicação imediata da
   mesma regra ao próprio output (anti-infinito local).

### Estágio T — terminação + backstop (mensagem base = vanilla)
1. Teste: recursão infinita → **erro com a mensagem base idêntica à do vanilla** (M1);
   recursão profunda legítima → não erra (abaixo do teto). Confirmar que falham antes.
2. Código: o backstop (teto; o **número** espelha o 64 do vanilla — declarado como
   mecânica que pode divergir, não paridade). Por padrão, a mensagem é a do vanilla,
   **sem** acréscimo.

### Estágio E-flag — a flag de erro completo (canal separado)
1. Teste: com a flag ligada, o erro de recursão ganha um **hint separado** (canal de
   diagnóstico, **não** a mensagem base) com a classificação correta — **cíclico**
   (morfologia repete no caminho, via `==` do P345), **divergente** (cresce sem
   repetir), **converge-fundo** (recursão legítima que passou do teto). Sem a flag, a
   mensagem base é **byte-idêntica** à do vanilla. Confirmar que falham antes.
2. Código: a flag; a análise que só corre quando o teto dispara **e** a flag está ligada
   (caminho quente intacto); a classificação usando o histórico de morfologias.

### Estágio Ev — evolução de testes pela semântica
Os testes que asseveram o comportamento eager-trunca-no-nível-1: classificar pela
**semântica** (o vanilla revisita) e evoluir **um a um** com a justificativa morfológica
(saída do vanilla colada). O `m1` vai de "b" → "c". Os testes de anti-recursão-infinita
(`f3s2_show_callout_anti_recursao_termina`, `show_rule_nao_recursiva_sem_stack_overflow`)
revalidados contra a nova terminação.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates) — morfológica, não booleana/byte

```
build: limpo a cada estágio (release buildável por estágio — perf C2).
suíte (RUST_MIN_STACK=33554432): C0 (2723) ± N. Asserções alteradas só as que a
  semântica de revisitação muda (cada uma justificada morfologicamente + vanilla).
  Reportar evoluídas e novas.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (morfológica — ADR-0107):
  - composição/recursão limitada: a morfologia da árvore final == a do vanilla
    (o m1 → "c"; casos do spike-2 1/2). NÃO "os bytes batem".
  - terminação por padrão: recursão infinita → erro com mensagem base IDÊNTICA ao
    vanilla (byte-a-byte na mensagem base — aqui o byte importa porque a MENSAGEM é
    comportamento observável, ADR-0033); recursão profunda legítima → não erra.
  - flag de erro completo: ligada, classifica certo (cíclico/divergente/converge-fundo)
    num hint SEPARADO; desligada, a mensagem base não muda. A flag NÃO altera o
    comportamento padrão — só adiciona detalhe sob demanda.

paridade da mensagem: a mensagem base do erro de recursão é a do vanilla (M1), sem
  o acréscimo da flag — a dica da flag e a classificação vivem no canal de hint
  separado. Registrar a decisão (mensagem base idêntica; detalhe em canal próprio).

o número do teto é mecânica: declarar que o valor (64, espelhando o vanilla) é
  implementação e pode divergir; a paridade é o COMPORTAMENTO (infinito→erro), não o
  número. Nenhum documento legítimo deve bater no teto antes do vanilla.

lente (--comparar antes/depois): delta de aresta no megaciclo (a realização toca o
  anel show/introspect — pode mexer); content→elements 66; elem→elem 0. Se a contagem
  de ciclos mudar, é ACHADO a explicar, não esconder. Registrar.

perf (C2, par back-to-back): o caminho quente — o teto é incremento O(1); a análise da
  flag só corre no erro+flag (não no caminho normal); a revisitação adiciona passes
  (medir o custo da composição). Reportar o par; regressão é achado, não se esconde.
```

---

## O que NÃO fazer
- **Não tocar o `==` morfológico (P345)** nem o `PartialEq` do Rust.
- **Não pôr a dica da flag na mensagem base** — ela vai no canal de hint separado; a
  mensagem base é idêntica ao vanilla (decisão do dono).
- **Não rodar a análise de classificação no caminho quente** — só quando o teto dispara
  E a flag está ligada.
- **Não executar o de-bake (F-5)** aqui — se M4 disjunto, fica liberado para o seguinte;
  se cruza, é lote depois. Esta recursão **não** des-assa nada.
- **Não tratar o número do teto como paridade** — é mecânica; a paridade é o
  comportamento de terminação.
- **Não medir aceitação pelo booleano ou pelos bytes da saída** (exceto a mensagem base,
  que É comportamento observável) — a métrica é morfológica.
- **Não improvisar a camada multi-passe** se M3 a exigir — isso é decisão do dono na
  trava (foi estacionada uma vez por ser grande).
- **Não estimar com "~"**.

---

## Relatório (`typst-passo-347-relatorio.md`)
- Fase A: M1 (semântica + mensagem base exata + canal de hint), M2 (forma de detecção +
  custo), M3 (eager vs multi-passe + dimensão se multi-passe), M4 (interseção com o
  de-bake).
- Fase B: o diff por estágio; a semântica de revisitação; a terminação (mensagem base =
  vanilla); a flag e a classificação no canal separado.
- Evolução de testes: cada um, pela semântica (vanilla colado); o `m1` → "c".
- Aceitação morfológica: a árvore final == vanilla (composição/recursão); a mensagem
  base idêntica; a flag classificando certo sem mudar o padrão.
- Verificação: suíte, lint, paridade-da-mensagem, número-do-teto-é-mecânica, lente, perf.
- **Decisões registradas**: a mensagem base idêntica ao vanilla + flag em canal separado
  (decisão do dono, P347); o número do teto como mecânica; o resultado de M3 (eager ou
  multi-passe) e de M4 (de-bake liberado ou ordenado).
- **Mapa de filtro (campo):** o lugar lógico — "a semântica de recursão de `#show` mora
  na camada de realização, e a terminação separa comportamento (paridade: infinito→erro)
  de mecanismo (o teto, e a flag de classificação como melhoria sobre a paridade)" — com
  o rastro (eager trunca desde sempre; P340 estacionou o multi-passe ao bater em
  content-preservation; P345 destravou medindo que a igualdade mascarava; P347 fecha a
  recursão com revisitação + terminação + erro melhor que o vanilla sob flag).
- Item: o `content→elements` agora aponta para o **Marco G** (P346), não volta como órfão.
```
