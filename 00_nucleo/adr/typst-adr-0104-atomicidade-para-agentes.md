# ⚖️ ADR-0104: Atomicidade para agentes — força arquitetural de primeira classe

**Status**: `EM VIGOR`
**Data**: 2026-06-10
**Passo promotor**: P314 (materialização de processo/linhagem)
**Diagnóstico**: P313 (`diagnosticos/diagnostico-modelo-elemento-passo-313.md`, §0 requisito + §2.1 imposto medido + §6 texto do princípio)
**Categoria**: Processo / Metodologia de manutenção por IA
**Cross-ref**: ADR-0026 (Content enum fechado — o hub que motiva),
              ADR-0037 (coesão por domínio),
              ADR-0094 (meta-operacional specs),
              ADR-0105 (Modelo de elemento — a primeira aplicação),
              `diagnostico-bloqueio-processo-2026-06-09.md` (falha F4 — precedente da cláusula 3)

---

## Contexto

O dono do projeto formulou (transcrito no §0 do diagnóstico P313) um requisito
arquitetural que não estava escrito em nenhuma ADR:

> **Atomicidade para agentes.** O custo real de um passo de materialização é
> proporcional ao que a sessão de IA precisa **ler e editar** (tokens de
> contexto). O desenho atual concentra cada elemento novo num hub
> (`content.rs` + matches), fazendo esse custo crescer com a cobertura já
> entregue.

As medições do P313 quantificaram a pressão:

- `content.rs` tem **5782 linhas** e o `enum Content` **77 variantes** (a
  auditoria F2 contava 59; +18 desde então). Os três maiores matches
  por-variante somam **972 linhas** (`map_content` 416, `map_text` 316, `eq`
  228) — cada variante nova obriga arm em todos.
- O passo P311b (MathStyled) tocou **29 ficheiros `.rs`**; só **2** (420
  linhas) eram o elemento. Dos restantes, **18 ficheiros mudaram uma única
  linha** — o `@prompt-hash` — porque o passo editou os prompts L0 grossos
  `rules/stdlib.md` e `rules/math/layout.md`, e a trava V5 (PromptDrift)
  obriga toda a linhagem a re-espelhar o hash.
- O hash de `content.rs` esteve estável por 27 passos e quebrou no P311 — o
  churn do hub é custo recorrente.

Esta ADR eleva o requisito a **força arquitetural de primeira classe do Tekt**,
para que decisões futuras de desenho (modelo de elemento, organização de
prompts, travas do linter) sejam julgadas também por este eixo.

---

## Decisão

O custo de um passo de materialização é proporcional ao contexto (tokens) que
uma sessão de IA precisa **ler e editar** para o completar. Disto decorrem três
cláusulas vinculativas e um corolário de sequência. As cláusulas **não podem
ser enfraquecidas** por decisões de componente; só por ADR sucessora explícita.

### Cláusula 1 — Hubs concentradores são anti-padrão de manutenção por IA

Um ficheiro cujo tamanho **cresce com a cobertura já entregue** (ex.:
`content.rs`; um `enum` cujo número de variantes só aumenta; matches de 400+
linhas) faz o custo-por-feature crescer monotonicamente com o que já foi
entregue. **Preferir um módulo por unidade de feature** (um elemento, uma
função) ao concentrador único. Quando o hub é inevitável (ex.: o `enum` para
exaustividade), a lógica por-variante deve **morar fora dele** (delegação), de
modo que o hub seja uma tabela de despacho, não o corpo da lógica.

### Cláusula 2 — O acoplamento por linhagem deve ser fino

Prompts L0 de granularidade grossa (um prompt para toda uma área — ex.:
`stdlib.md` cobrindo todas as funções nativas) propagam o `@prompt-hash` a
**dezenas** de ficheiros não relacionados a cada edição (medido: ~18 ficheiros
no P311). **Um prompt por elemento/função** mantém o toque local: editar a
spec de uma função re-hasheia só o(s) ficheiro(s) dessa função. Regras
genuinamente partilhadas vivem num `_comum.md` da área, citado pelos finos,
não duplicado. (Primeira aplicação: P314 fatia `stdlib.md` e `math/layout.md`.)

### Cláusula 3 — A verificação mecânica não pode depender de o agente lembrar

Se um desenho troca **erro-de-compilação** (ex.: a exaustividade do `match`
sobre um `enum`) por convenção, tem de **repor a garantia** com um mecanismo
que não dependa de memória entre sessões: um **teste que varre** a tabela/
descritores × os consumidores, **ou** uma **trava nova do `crystalline-lint`**.
Sem isso, a regressão fica silenciosa entre sessões — exatamente a classe da
falha **F4** do `diagnostico-bloqueio-processo-2026-06-09.md` (deriva que
ninguém propaga porque ninguém é obrigado a lembrar).

### Corolário de sequência

Medir o **custo-por-elemento** (ficheiros tocados, dos quais hub/imposto) é a
**métrica de saúde** do modelo. Redesenhos que aumentam a atomicidade (ex.: os
candidatos D e F do P313) são **investimento contra o custo marginal
crescente**, e devem ser avaliados por quanto reduzem essa métrica — não só
por elegância.

---

## Prompts Afetados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `rules/stdlib.md` | fatiado em finos (P314, A.3); vira índice |
| `rules/math/layout.md` | fatiado em finos (P314, A.3); vira índice |
| (futuros) `rules/layout.md`, `rules/eval.md`, `rules/parse.md` | candidatos a fatiar (fan-in alto medido no P314; não fatiados aqui) |

---

## Consequências

**Positivas**: torna explícito e mensurável um eixo que já operava como
pressão implícita; dá critério objetivo para o modelo de elemento (ADR-0105) e
para a organização de prompts; reduz o imposto de hash medido.

**Negativas**: "um módulo/prompt por unidade" multiplica o número de ficheiros
(mais ficheiros pequenos vs menos ficheiros grandes); a cláusula 3 obriga a
construir travas (teste/lint) antes de relaxar a verificação do compilador —
trabalho extra no momento da transição.

**Neutras**: não revoga ADR-0026 (o `enum` fechado permanece pela
exaustividade); reorienta **onde** a lógica mora, não **se** há enum.

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Não registrar (deixar implícito) | zero trabalho | o requisito continua invisível para sessões futuras; repete o padrão F1/F4 |
| Registrar só como LESSONS | mais leve | LESSONS não tem força de trava; decisões de desenho não o citariam |
| ADR (esta) | força vinculativa; citável por ADR-0105 e pelo linter | obriga manutenção dos critérios |

---

## Notas de aplicação

Esta ADR é **processo**, não lógica: não altera nenhum `.rs` por si. A primeira
aplicação concreta é o próprio P314 (fatiamento dos dois prompts medidos) e o
ADR-0105 (modelo de elemento D→F), que a cita como justificativa.
