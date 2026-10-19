# Prompt — Lote F-3, incremento 2: `#show` sobre o elemento dinâmico

**Pré-condição**: F-3 inc-1 commitado (`69d4929d2`) — suíte **2710**, lint 0/0,
árvore limpa; DEBT C2 fechado (Content::Dynamic renderiza); F-2 fechado (P335).
**Tipo**: **Lote F-3, inc-2** — fecha o F-3. A peça é a integração
registry↔eval que o F-1 deferiu + a maquinaria de seleção para o elemento
dinâmico. Sessão **fresca** (o inc-1 fechou em sessão longa por decisão
registrada).
**Objetivo**: o elemento de usuário passa a existir **na linguagem**:
`#callout(...)` constrói via registry, e `#show callout: ...` intercepta —
pelo **mesmo** caminho eager (guards + depth-64) dos nativos, sem caminho
paralelo.

---

## Estágio 0 — A decisão do gatilho (antes de qualquer código)

O registro (c) do inc-1 armou: "quando `#show` entrar na cobertura da
linguagem, os 5 casos do spike-2 viram testes de paridade contra o vanilla
medido". **Este incremento expõe `#show <dyn>:` na linguagem — a decisão de
se isso dispara o gatilho não pode ficar implícita.**

Posição recomendada ao dono (decidir e registrar no L0 §3b, qualquer que
seja a escolha):

- **Dispara parcialmente, agora**: dos 5 casos do spike-2, os que forem
  **expressáveis com a superfície coberta após este inc** entram como testes
  de paridade contra o vanilla **medido** (rodar o vanilla do `lab/`, anotar
  a saída, afirmar igualdade) — critério de aceitação deste lote, não item
  futuro.
- Os casos que exigirem superfície ainda não coberta: **registrar caso a
  caso o que falta** (qual construção, qual lote a trará). O gatilho continua
  armado para esses, com o ponteiro.
- Se o eager **falhar** um caso expressável: conforme o (c), a realização
  multi-pass vira **lote** — não consertar inline; registrar e parar o
  estágio.

## Fase A — reconhecimento (com `file:line`; a lição do P331 vale aqui)

O inventário já errou uma vez sobre exatamente esta maquinaria ("`#show` não
existe", P331 — falso). Antes de código, mapear da fonte canônica:

1. **Resolução de chamáveis no eval**: onde um identificador de chamada
   (`#callout(...)`) resolve hoje; onde o `ElementRegistry` vive; o que
   exatamente o F-1 deferiu no threading registry→escopo (citar o registro
   do F-1).
2. **`apply_show_rules`/`intercept_content`**: a forma do match atual
   (nativos por endereço de função), onde o braço `Content::Dynamic`
   entra, e **como o guard anti-recursão chaveia** — endereço de função não
   existe para o dinâmico; a chave do dyn (nome/kind-id) precisa entrar no
   `active_guards` com a mesma semântica (e o cap depth-64 igual).
3. **`Selector`**: variantes atuais e os pontos de match — onde
   `Selector::DynKind` (ou nome equivalente ao padrão existente) se encaixa;
   como o parser do alvo do `#show` distingue `callout` (dyn) de um nativo e
   de um target pontuado (`math.equation`, precedente do S2/B1).
4. **Os 5 casos do spike-2**: reler cada um e classificar
   expressável-agora vs falta-superfície (insumo do Estágio 0).
5. **Vanilla**: localizar no `lab/` o comportamento de `#show` sobre
   elemento custom equivalente (se existir equivalente; se não existir,
   registrar — a paridade então é contra a semântica de `#show` em nativos,
   com a diferença declarada).

Checkpoint com o dono após a Fase A se o mapa contradisser qualquer premissa
deste prompt (precedente: o checkpoint do F-2).

## Fase B — estágios (commit por estágio, isoláveis)

| Estágio | O quê |
|---|---|
| **S1 — registry no escopo** | O threading deferido pelo F-1: `#callout(args)` resolve no registry e constrói `Content::Dynamic` via o construct/dispatch do F-1. Testes eval: construção com args; elemento desconhecido = erro do catálogo de erros existente (não panic); **escopo léxico** — registro/uso respeita o padrão `local_styles` (eco do F-2, não inventar outro). |
| **S2 — `Selector::DynKind` + braço no `apply_show_rules`** | `#show callout: <transform>` intercepta `Content::Dynamic` do kind correspondente, eager, com guard por chave dyn no `active_guards` + depth-64. Testes: transform aplica; **anti-recursão provada** — uma regra `#show callout:` que emite outro `callout` termina pelo guard (o teste-contrato deste lote; sem ele, o eager dyn pode pendurar); regra para kind A não pega kind B; dyn e nativo coexistindo no mesmo doc. |
| **S3 — o gatilho executado** | Conforme o Estágio 0: os casos expressáveis do spike-2 como testes de paridade contra o vanilla **medido**; os demais registrados caso a caso com o que falta. Se um expressável falhar: parar, registrar, multi-pass vira lote. |
| **S4 — registro e fecho** | L0 §3b atualizado (a decisão do Estágio 0 + os ponteiros); o doc retomável do F-3 fechado **sem seções fósseis** (a lição do f2-progresso: a seção "S5b adiado" convivendo com "S5b ✅" — marcar superseded ou apagar ao fechar); F-3 marcado fechado na fila (F-4 Styled é o próximo). |

## Caronas (commit próprio, antes dos estágios)

- **C1** — o fóssil do `f2-progresso-passo-335.md`: a seção "S5b — adiado"
  contradiz a tabela (S5b ✅ `bfc7a5e7c`). Marcar superseded ou remover.
- **C2** (opcional, se couber sem risco) — `engine.figure_numbering`
  threaded-mas-não-lido (resíduo declarado do F-2): remover. Se não couber,
  fica registrado como está.

## O que NÃO fazer

- **Não criar caminho de interceptação paralelo** para o dyn — é o mesmo
  `apply_show_rules`, mesma ordem, mesmos guards; um segundo caminho é a
  semente do próximo DEBT 99.E.
- **Não consertar o eager inline** se um caso de paridade falhar — o (c) é
  explícito: multi-pass é lote, decidido com o dado.
- **Não estimar contagens com "~"** — números exatos de testes
  adicionados/apagados/migrados por estágio, ou a nota de por que não
  (terceira recorrência do padrão; disciplina pós-S5b).
- **Não deixar canal velho morto-mas-alimentado** — se o S1/S2 tornarem
  algum caminho do inc-1 obsoleto, remover no próprio estágio ou registrar
  o risco de mascaramento (a lição do auto-TOC do S5b: morto-alimentado
  mascara migração incompleta).

## Critérios de Verificação

```
Dado #callout(args) num doc
Então constrói via registry → Content::Dynamic → renderiza (o pipeline do
inc-1), com erro de catálogo para elemento desconhecido

Dado #show callout: <transform>
Então intercepta eager pelo apply_show_rules comum, com transform aplicado

Dado uma regra #show callout: que emite callout
Então termina pelo guard (anti-recursão provada — teste-contrato)

Dado regra para o kind A e instância do kind B
Então não intercepta; dyn e nativos coexistem no mesmo doc

Dado o Estágio 0
Então a decisão do gatilho registrada no L0; os casos expressáveis do
spike-2 como paridade contra vanilla medido; os demais com ponteiro do que
falta; falha de expressável = parada registrada, multi-pass vira lote

Dado o fecho
Então suíte verde com deltas EXATOS por estágio; lint 0/0; árvore limpa;
commit por estágio; F-3 fechado na fila; doc retomável sem fóssil
```

---

## Histórico

| Data | Motivo |
|---|---|
| 2026-06-11 | F-3 inc-2: o elemento dinâmico entra na linguagem (registry→eval deferido do F-1) e no `#show` (Selector::DynKind + braço Dynamic no apply_show_rules, guards por chave dyn + depth-64 — caminho único com os nativos). Estágio 0 obriga a decisão explícita do gatilho do spike-2 (o (c) do inc-1 armou; este inc é quem decide se puxa): expressáveis viram paridade contra vanilla medido AGORA; faltantes registrados com ponteiro; falha → multi-pass vira lote, sem conserto inline. Teste-contrato: anti-recursão do dyn. Caronas: fóssil do f2-progresso; figure_numbering opcional. Disciplinas herdadas: recon file:line (P331), sem "~" nas contagens (S5b), sem morto-alimentado (auto-TOC). |
