# Relatório — typst-passo-849: `measure()` com métricas reais — decisão: investigar Opção 2 (resolução pós-eval)

**Data:** 2026-07-22  
**Executor:** Kimi Code (agente principal; prompt lido de `00_nucleo/materialization/typst-passo-849.md`).  
**Proveniência das medições:** commit HEAD `92daa9c66` (P847). Medições de P842 reutilizadas conforme o passo.  
**Decisão do dono:** investigar a **Opção 2** (resolver `measure()` numa fase pós-eval / durante o layout), sem implementação imediata. A dívida foi formalizada como **DEBT-69** em `00_nucleo/diagnosticos/debt/DEBT.md`.

---

## 1. Problema (reconfirmado)

`measure([hello])` devolve dimensões diferentes do vanilla:

- **Vanilla:** `(22.19pt, 7.24pt)` — shaping real via `rustybuzz`.
- **Cristalino:** `(33pt, 14.85pt)` — heurística monoespaçada (`0.6 × size` por codepoint; `1.35 × size` de altura) de `FixedMetrics`.

A causa está documentada em `01_core/src/engine/layout/mod.rs:1784-1789` (P712): `measure_content_real` constrói o `Layouter` de medição com `FixedMetrics` porque `eval_func_call` (onde `measure()` é interceptado) corre em L1, que por desenho não acede a métricas de fonte reais (`FallbackFontMetrics` é L3).

`measure()` só é válido dentro de `#context` (`ctx.in_context == true`), o que significa que a medição corre durante a expansão de `ContextBlock` em `03_infra/src/pipeline.rs:403-415` — já **depois** do `eval` inicial e **antes** do `layout_with_introspector_and_metrics` (onde as métricas reais estão disponíveis).

---

## 2. Decisão: investigar Opção 2

O dono optou por não implementar a **Opção 1** (injeccão de métricas no `Engine` durante a expansão de contexto) e por aprofundar a **Opção 2** (resolução de `measure()` numa fase pós-eval), com o objectivo de encontrar uma forma de resolver o achado sem quebrar a pureza de L1.

A dívida foi formalizada como **DEBT-69**. Este relatório desdobra a Opção 2 para apoiar o trabalho de design posterior.

---

## 3. Como o vanilla resolve `measure()`

`lab/typst-original/crates/typst-library/src/layout/measure.rs:47-105` mostra que `measure()` é uma função **contextual** que corre durante a fase de *realization*/layout:

- Recebe `engine: &mut Engine` e `context: Tracked<Context>`.
- Obtém `styles` a partir do `context`.
- Cria uma região de medição (`Region::new`) com largura/altura infinita (ou os overrides `width`/`height`).
- Coloca o `Locator` em modo de medição (`LocatorLink::measure`).
- Chama `engine.library.routines.layout_frame(engine, &content, locator, styles, pod)`.
- O `engine` já tem acesso a métricas de fonte reais durante este layout-frame isolado.

O ponto crítico: no vanilla, o conteúdo dentro de `#context { ... }` **não é totalmente avaliado durante o eval inicial**. A closure do contexto é executada durante a realização (fase intermédia entre eval e layout), quando o engine de layout — com métricas reais — está disponível.

No cristalino, a separação de fases é mais rígida:

```text
eval (L1, FixedMetrics) → introspect (L3/L1) → expand_context_blocks (L3, FixedMetrics) → layout (L3, FallbackFontMetrics)
```

`measure()` corre em `expand_context_blocks`, antes do layout. É essa deslocação temporal que produz o achado #34.

---

## 4. Abordagens possíveis para a Opção 2

### 4.1. Mover a expansão de `ContextBlock` para dentro do layout

**Ideia:** em vez de expandir todos os `ContextBlock` antes do layout, deixar que o layout os encontre e os expanda com acesso a `FallbackFontMetrics`.

**Onde encaixa:**
- O `Layouter` já percorre o `Content`. Quando encontrar um `Content::ContextBlock`, poderia:
  1. Construir um `Engine` temporário com as métricas reais.
  2. Avaliar a closure do contexto.
  3. Substituir o bloco pelo conteúdo resultante.
  4. Continuar o layout.

**Blockers:**
- **Introspecção:** o `TagIntrospector` é construído **antes** do layout a partir do conteúdo original. Se os `ContextBlock` só forem expandidos durante o layout, os elementos locatable gerados dentro deles (headings, labels, counters) não estarão no introspector. Isso quebra `query()`, `counter(heading).at(here())`, bookmarks, etc.
- **Vanilla não faz isto literalmente:** no vanilla, a realização é uma fase separada que constroi o introspector **e** executa contextos, antes do layout paginado. Mover só os contextos para o layout cristalino seria uma aproximação perigosa.

**Veredicto:** requer reestruturar o pipeline eval→introspect→layout para algo mais próximo de eval→realize→layout. Esforço alto.

---

### 4.2. Executar `measure()` numa fase de realização separada

**Ideia:** introduzir uma fase intermédia "realização" (realization) entre introspecção e layout, similar ao vanilla:

```text
eval → introspect → realize (expande ContextBlocks + resolve measure com métricas reais) → layout
```

**Onde encaixa:**
- `realize` receberia o conteúdo, o introspector e `FallbackFontMetrics`.
- Expandia `ContextBlock` exactamente como `expand_context_blocks` faz hoje, mas com métricas reais no `Engine`.
- O resultado seria o conteúdo final, sobre o qual se faria introspecção novamente (ou se manteria o introspector actualizado).

**Blockers:**
- **Re-introspecção:** após a realização, o conteúdo muda (os `ContextBlock` são substituídos pelo seu resultado). É necessário re-construir o introspector, como já acontece em `expand_context_blocks_and_reintrospect` (P844). O mecanismo existe, mas teria de ser estendido para toda a fase de realização.
- **Performance:** a realização executa closures de contexto, que podem ser pesadas. Adicionar uma fase extra não é trivial.
- **Cascatas de contexto:** um `ContextBlock` pode gerar outro `ContextBlock`? No vanilla, contextos são resolvidos iterativamente até fixpoint. O cristalino já tem `expand_context_blocks_and_reintrospect`; a lógica de fixpoint teria de ser robustecida.

**Veredicto:** alinha-se com o vanilla e resolve o achado de forma limpa. Esforço médio-alto; toca no pipeline global.

---

### 4.3. Valor lazy / `Value::Dict` atrasado

**Ideia:** `measure()` não devolve um `Value::Dict` concreto; devolve um valor lazy que só é resolvido quando as métricas reais estiverem disponíveis.

**Onde encaixa:**
- Durante `expand_context_blocks`, `measure()` produziria `Value::Dict` com `Value::Length` lazy.
- Estes comprimentos seriam propagados pelo eval e usados para construir `Content`.
- Durante o layout, os comprimentos lazy seriam resolvidos para valores reais.

**Blockers:**
- **Computações em eval:** se o utilizador fizer `#let w = measure([x]).width; #if w > 30pt [grande] else [pequeno]`, a condição `w > 30pt` tem de ser avaliada durante eval. Com valores lazy, isso exige que a comparação também seja lazy ou que a resolução aconteça antes de qualquer branch.
- **Estruturas de dados:** `Value::Length` teria de suportar um modo lazy, ou introduzir um novo `Value::LazyLength`. Todos os consumidores de `Value::Length` teriam de ser revistos.
- **Loops e acumuladores:** qualquer código que some/compara tamanhos ficaria afectado.

**Veredicto:** solução muito invasiva no modelo de valores. Risco alto.

---

### 4.4. Medição em duas fases (heurística + correção)

**Ideia:** usar `FixedMetrics` durante a expansão de contexto para produzir um documento provisório; depois do layout com métricas reais, detectar os pontos onde `measure()` foi usado e re-executar a expansão com as métricas correctas.

**Onde encaixa:**
- Primeiro passo: eval + expand_context_blocks com `FixedMetrics` (igual ao actual).
- Layout provisório com `FallbackFontMetrics` para descobrir as métricas reais.
- Segundo passo: re-executar `expand_context_blocks` com as métricas reais agora conhecidas.
- Layout final.

**Blockers:**
- **Não-convergência:** os resultados de `measure()` podem afectar o layout (ex.: `#let w = measure([x]).width; #block(width: w)`), que por sua vez pode afectar quebras de linha e, logo, as métricas. Pode não haver fixpoint único.
- **Custo:** duplicar eval+layout é caro.
- **Introspecção:** o introspector usado no segundo passo teria de ser reconstruído.

**Veredicto:** mais uma heurística do que uma solução. Não garante paridade.

---

## 5. Análise comparativa

| Abordagem | Pureza de L1 | Impacto arquitectural | Esforço estimado | Risco | Paridade garantida |
|-----------|--------------|----------------------|------------------|-------|-------------------|
| 4.1 Mover expansão para layout | Alta | Muito alto | 2-3 passos | Alto | Não sem reestruturar introspector |
| 4.2 Fase de realização separada | Alta | Alto | 1-2 passos | Médio-alto | Sim, se igualar ao vanilla |
| 4.3 Valor lazy | Alta | Muito alto | 3+ passos | Muito alto | Teoricamente sim |
| 4.4 Duas fases | Média | Médio | 1-2 passos | Alto | Não |
| **Opção 1 (injeccão no Engine)** | Média | Baixo | ~0.5 passo | Baixo | Sim |

A Opção 2 só é vantajosa se a prioridade for manter L1 estritamente puro. O custo é significativamente maior do que a Opção 1.

---

## 6. Recomendação para o trabalho de design

Se o dono quiser manter a Opção 2, a abordagem **4.2 (fase de realização separada)** é a mais promissora porque:

1. Alinha-se com o modelo do vanilla (`eval → realize → layout`).
2. Reutiliza mecanismos existentes (`expand_context_blocks_and_reintrospect`, P844).
3. Não exige alterar o modelo de valores.
4. Permite que `measure()` receba um `Engine` com métricas reais sem contaminar o eval inicial.

A próxima sonda deveria ser:
- Prototipar a fase de realização numa cópia do pipeline.
- Verificar se a re-introspecção pós-realização resolve os locatable gerados por contexto.
- Medir o impacto em documentos com múltiplos `#context` aninhados.

---

## 7. DEBT-69 formalizada

A dívida foi registada em `00_nucleo/diagnosticos/debt/DEBT.md` com:

- **Origem:** achado #34 de P810/P842.
- **Estado:** ABERTO.
- **Decisão:** investigar Opção 2 (resolução pós-eval / fase de realização); não implementar Opção 1 sem reconsideração explícita.
- **Critério de reabertura/encerramento:** quando existir um design concreto para a fase de realização (abordagem 4.2) ou decisão explícita do dono de adoptar a Opção 1.

---

## 8. Validação

Nenhum código foi alterado neste passo. A suíte mantém-se verde:

- `cargo test --workspace`: **5417 passed; 0 failed** (estado de P847).
- `crystalline-lint .`: exit 0 (aviso pré-existente V7 não relacionado).
