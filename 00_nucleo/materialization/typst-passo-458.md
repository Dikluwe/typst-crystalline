# P458 — Sonda DEBT-2: Verificação de premissa + consolidação do oráculo de introspecção

> **Passo:** 458  
> **Data:** 2026-06-25  
> **Foco:** (1) Verificar a premissa do DEBT-2 no Typst vanilla; (2) Se confirmada (lazy), consolidar o oráculo de introspecção existente; (3) Distinguir oráculo de introspecção (necessário agora) de memoização incremental (performance, fase futura).  
> **Tipo:** Sonda / Investigação / Decisão arquitectural.  
> **ADR-0114:** Sonda A.0 real antes de decidir; medição antes de construir infraestrutura.

---

## Contexto

O DEBT-2 documenta uma divergência semântica: `#let x=1; #let f()=x; #let x=2; #f()` devolve `1` no cristalino (eager, snapshot) vs `2` no Typst vanilla (lazy, resolução por referência). A premissa de que o vanilla é lazy **nunca foi verificada empiricamente**. O roteiro de conclusão (Trilha 9) assume que fechar DEBT-2 exige `comemo`/`TrackedWorld` — mas isso junta duas necessidades distintas num único nome.

Este passo separa as duas necessidades, verifica a premissa, e decide o caminho com números.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| Premissa "vanilla é lazy" verificada? | Não — nunca foi medida | ❌ |
| Oráculo de introspecção existe em pedaços? | Sim — fixpoint TOC, BibStore, SealedPositions, CounterRegistry, known_page_numbers | ✅ |
| `comemo` (crate 0.4) existe no projeto? | Sim — typst-core depende dele; guards de recursão usam `Tracked<Route>` | ✅ |
| `TrackedWorld` existe? | Não — não implementado | ❌ |
| Teste de paridade vanilla vs cristalino para closures? | Não existe | ❌ |
| Bloqueadores? | Nenhum técnico — apenas decisão de premissa | ✅ |

**Reclassificação:** S (~20 min; 1 teste de paridade + análise de arquitectura + decisão).

---

## Fase A — Verificação da premissa

### A.1 Teste de paridade no Typst vanilla

Executar no Typst vanilla (versão de referência do projeto, tipada no `Cargo.toml`):

```typst
#let x = 1
#let f() = x
#let x = 2
#f()
```

**Resultado esperado (premissa):** `2` (lazy evaluation, resolução de `x` no momento da chamada).

**Resultado alternativo (premissa falsa):** `1` (eager evaluation, `x` capturado por valor no momento da definição de `f`).

### A.2 Teste de paridade no cristalino

Executar o mesmo teste no cristalino (compilador actual, branch `main`):

```typst
#let x = 1
#let f() = x
#let x = 2
#f()
```

**Resultado actual:** `1` (snapshot de `Arc<Scope>` no momento da definição de `f`).

### A.3 Decisão

| Caso | Vanilla | Cristalino | Divergência? | Acção |
|------|---------|------------|--------------|-------|
| **1** | `1` | `1` | ❌ Nenhuma | **DEBT-2 fecha como premissa corrigida.** Sem código. Atualizar `DEBT.md`: "Premissa 'vanilla é lazy' era falsa. Ambos são eager. Sem divergência." |
| **2** | `2` | `1` | ✅ Confirmada | **DEBT-2 mantém aberto.** Prosseguir para Fase B (consolidação do oráculo). |
| **3** | `1` | `2` | ✅ Invertida | **DEBT-2 mantém aberto.** Cristalino é mais lazy que vanilla; investigar porque. |

**Nota:** O Caso 1 é o mais provável? Não. O Typst vanilla é conhecido por usar lazy evaluation com `comemo`. Mas **nunca foi medido neste projeto**. A medição é o portão de honestidade da ADR-0114.

---

## Fase B — Se a premissa se confirmar (Casos 2 ou 3): consolidação do oráculo

Se o vanilla for lazy (Caso 2), o cristalino precisa de **avaliação em duas passagens com um oráculo de introspecção** — não de `TrackedWorld` completo.

### B.1 Separação de necessidades

| Necessidade | Descrição | Implementação | Fase |
|-------------|-----------|---------------|------|
| **Oráculo de introspecção** | Documento final visível durante o eval; resolução de referências cruzadas, counters, page numbers, TOC | Consolidar pedaços existentes: fixpoint TOC, BibStore, SealedPositions, CounterRegistry, known_page_numbers | **Agora** (Fase 2–4) |
| **Memoização incremental** | Recompilação parcial quando o documento muda; cache de sub-árvores | `TrackedWorld` via `comemo` | **Futuro** (Fase 5), só se benchmark P441 demonstrar necessidade |

### B.2 O oráculo já existe em pedaços

Os seguintes componentes já implementam fragmentos do oráculo:

| Componente | O que faz | O que falta |
|------------|-----------|-------------|
| **Fixpoint da TOC** (`layout()` com loop de convergência) | Layout com introspecção iterativa até estabilizar | Generalizar para outros consumidores (não só TOC) |
| **BibStore** (`P429`) | Chave determinística para entradas bibliográficas | Integrar no oráculo global de documento |
| **SealedPositions** | Posições imutáveis após layout | Expor como API de introspecção para eval |
| **CounterRegistry** | Contadores hierárquicos (heading, figure, equation) | Persistir entre passagens de eval |
| **known_page_numbers** | Page numbers conhecidos após layout | Retro-alimentar para eval de segunda passagem |

### B.3 Trabalho de consolidação (se necessário)

Se Caso 2 se confirmar, o trabalho não é "inventar `comemo`" — é **consolidar estes pedaços num oráculo selado**:

1. **Fase 2:** Definir a fronteira L1/L3 do oráculo — o que o eval (L1) pode perguntar ao layout (L3) e vice-versa.
2. **Fase 3:** Consolidar o oráculo selado — unificar SealedPositions, CounterRegistry, known_page_numbers numa única estrutura `DocumentOracle`.
3. **Fase 4a:** Ligá-lo ao eval de segunda passagem — o eval corre duas vezes: primeira sem oráculo, segunda com oráculo populado pelo layout da primeira.
4. **Fase 4b:** Ligá-lo aos consumidores — TOC, bibliography, counters, page numbers.
5. **Fase 4c:** Se e só se a premissa se confirmar, ligar ao modelo de closures — captura por referência simbólica em vez de snapshot.

### B.4 O que NÃO é necessário agora

- **`TrackedWorld`** (memoização incremental) — só se benchmark P441 demonstrar que a recompilação parcial é necessária.
- **`comemo` como oráculo** — `comemo` é memoização, não introspecção. O oráculo é construído no projeto, não importado.

---

## Fase C — Atualização do DEBT-2 e do roteiro

### C.1 Se Caso 1 (premissa falsa)

```markdown
- **DEBT-2** — Closures eager (captura por snapshot, não por referência)
  - Estado: **FECHADO**
  - Nota: Premissa "vanilla é lazy" verificada em P458 e encontrada falsa. O vanilla também é eager (captura por valor). O cristalino bate com o vanilla. Sem divergência semântica. Sem código necessário.
  - Fechado em: P458
```

### C.2 Se Caso 2 (premissa confirmada)

```markdown
- **DEBT-2** — Closures eager (captura por snapshot, não por referência)
  - Estado: **EM ABERTO**
  - Nota: Premissa confirmada em P458 — vanilla é lazy, cristalino é eager. Fecho requer oráculo de introspecção (Fases 2–4), não `comemo`/`TrackedWorld`. Trabalho estimado: M-L (consolidação de pedaços existentes + segunda passagem de eval). Não é XL.
  - Bloqueador: Oráculo de introspecção consolidado (Fases 2–4).
  - Não bloqueador: `comemo`/`TrackedWorld` (Fase 5, performance, só se benchmark exigir).
```

### C.3 Atualização do roteiro

Se Caso 2, adicionar à Trilha 9:

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| Fase 2: Fronteira L1/L3 do oráculo | — | S | Pronto |
| Fase 3: Consolidar `DocumentOracle` | Fase 2 | M | Pronto |
| Fase 4a: Eval de segunda passagem | Fase 3 | M | Pronto |
| Fase 4b: Consumidores (TOC, bib, counters) | Fase 4a | S-M | Pronto |
| Fase 4c: Closures lazy (se premissa confirmada) | Fase 4a | M | Pronto |
| Fase 5: `TrackedWorld` via `comemo` (performance) | benchmark P441 | XL | Bloqueado por métrica |

---

## Scope-out explícito

- **Não** implementa `TrackedWorld` ou memoização incremental — isso é Fase 5, performance, só se benchmark exigir.
- **Não** implementa o oráculo completo — este passo é sonda e decisão; materialização é P459+ se necessário.
- **Não** altera código de produção no Caso 1 — DEBT-2 fecha sem código.
- **Não** assume que o vanilla é lazy — medir é o ponto central deste passo.

---

## Critério de fecho

- [ ] Teste `#let x=1; #let f()=x; #let x=2; #f()` executado no Typst vanilla (versão referência).
- [ ] Teste executado no cristalino (branch `main`).
- [ ] Resultado documentado em `DEBT.md` com decisão (Caso 1, 2 ou 3).
- [ ] Se Caso 1: DEBT-2 reclassificado para **FECHADO**, sem código.
- [ ] Se Caso 2: DEBT-2 actualizado com plano de fases (2–5), bloqueador correto (oráculo, não `comemo`).
- [ ] Roteiro de conclusão actualizado (Trilha 9).
- [ ] `cargo test --workspace` verde (zero código alterado no Caso 1; no Caso 2, apenas tests de paridade).
- [ ] `crystalline-lint` zero violations.

---

## Lição central

> **Nunca construir infraestrutura grande sobre uma suposição não medida.**

A deriva recorrente deste projeto (P388, P409, P413, P416, P421, P447, P451, P452) foi escrever specs de infraestrutura sem verificar o estado real. O DEBT-2 é o caso mais grave: uma infraestrutura XL (`comemo`/`TrackedWorld`) foi assumida como necessária sem verificar se a premissa (vanilla lazy) é verdadeira. A paridade é o primeiro item da sonda. Se o vanilla for eager, o cristalino já bate — e não há débito.

---

**Aguardando sua indicação:**

1. **Executar o P458** (sonda de paridade + decisão, ~20 min)?
2. **Escrever o P459** (próximo passo fechável, independente do resultado de P458)?
3. **Ajustar o escopo** do P458 (adicionar mais testes de paridade para outras construções de closure)?
