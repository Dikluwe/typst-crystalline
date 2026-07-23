# Relatório — typst-passo-854: reavaliação da abordagem 4.2 para `measure()`

**Data:** 2026-07-23  
**Executor:** Kimi Code (agente principal).  
**Proveniência das medições:** working tree não commitado (sobre HEAD `dfe3c2282`); leituras em `lab/typst-original/` e `00_nucleo/adr/`.  
**Estado:** **análise concluída; sem código implementado**.

---

## 1. Tarefa

Reavaliar criticamente a recomendação de P849 (abordagem 4.2 — introduzir uma fase de "realização" separada para resolver `measure()` com métricas reais) antes de prototipar. Verificar se a comparação de P849 foi honesta, se existem variantes não consideradas, e se a abordagem 4.2 se sustenta.

---

## 2. Reavaliação da tabela comparativa de P849

A tabela de P849 (§5) pontuou cinco abordagens em cinco critérios. Os principais pontos de atenção:

### 2.1. Viés possível

As notas foram atribuídas pelo mesmo executor que recomendava a 4.2. Isso não invalida a tabela, mas exige verificação externa. Após releitura:

- **4.1 (mover expansão para layout)**: a classificação de "impacto arquitectural muito alto" e "risco alto" é justificada. O vanilla não expande contextos durante o layout paginado; fazê-lo no cristalino quebraria o introspector, que é construído antes do layout.
- **4.2 (fase de realização separada)**: a classificação de "1-2 passos" e "risco médio-alto" parece **otimista**. A "realização" no vanilla (`typst-realize`) faz muito mais do que expandir contextos: aplica show rules, grouping de parágrafos/listas, espaçamento, etc. Uma fase equivalente no cristalino não é apenas mover `expand_context_blocks`; é reestruturar a fronteira entre eval, show rules e layout.
- **4.3 (valor lazy)**: a classificação de "3+ passos" e "risco muito alto" é conservadora e justificada.
- **4.4 (duas fases)**: a classificação de "alto risco" por não-convergência é justificada.
- **Opção 1 (injeção no Engine)**: continua sendo a de menor esforço e risco, mas com custo de pureza de L1.

### 2.2. Variantes não consideradas em P849

Foram identificadas duas variantes que P849 não explorou:

1. **4.1-limitada: resolver só `measure()` dentro do layout, sem mover toda a expansão de `ContextBlock`.**
   - Problema: `measure()` depende do contexto (`styles`, `location`), que no cristalino só existe durante `expand_context_blocks`. Resolver `measure()` no layout exigiria que o layout soubesse re-executar closures de contexto ou que transportasse o contexto até ao layout. Isso reintroduz complexidade equivalente à 4.2 sem a paridade estrutural.
   - **Veredicto:** não é claramente mais barata que 4.2.

2. **Híbrido 4.2/Opção 1: usar injecção de métricas só quando `measure()` é detectado.**
   - Problema: detectar estáticamente se um `ContextBlock` contém `measure()` é difícil (closures arbitrárias); detectar dinamicamente exige instrumentação do eval.
   - **Veredicto:** possível, mas adiciona complexidade de bifurcação do pipeline. Não parece vencedora face à 4.2 pura.

---

## 3. Estudo da fase de realização do vanilla

Leitura de `lab/typst-original/crates/typst-realize/src/lib.rs` e `lab/typst-original/crates/typst-library/src/foundations/context.rs`:

- A realização no vanilla é uma fase separada e nomeada: `typst-realize`.
- Ela aplica show rules e agrupa elementos (parágrafos, listas, etc.) para produzir itens bem conhecidos.
- `ContextElem` é um elemento `Locatable` cuja show rule (`CONTEXT_RULE`) chama a closure do contexto passando o `engine` e o contexto (`location` + `styles`).
- `measure()` é uma função `#[func(contextual)]` que, quando chamada dentro dessa show rule, recebe o `engine` com métricas reais.

**Implicação para o cristalino:**

A ADR-0118 (Runtime State via context, proposta em P506) já prevê uma fase de expansão pós-introspecção para `ContextBlock`:

```text
eval → introspect → expand → layout/query
```

Esta fase é análoga à parte do vanilla que resolve contextos. No entanto, a ADR-0118 não prevê o acesso a métricas de fonte reais durante essa expansão — mantém L1 puro. A abordagem 4.2 exigiria estender essa fase para ter acesso a `FallbackFontMetrics` (L3), o que é uma quebra da pureza de L1 na prática, embora numa fase isolada.

---

## 4. Busca por precedente externo

Foram consultados:

- ADRs vigentes em `00_nucleo/adr/`.
- ADR-0118 (Runtime State via context): reforça a fase de expansão pós-introspecção, mas rejeita injetar `Engine`/estado no layout por pureza.
- ADR-0106 (fronteira de extensão E1): menos relevante; trata de extensibilidade de elementos, não de fases de compilação.

Não foram encontradas outras reimplementações de Typst ou compiladores com fases análogas documentadas no repositório. Uma busca web rápida não foi feita porque o escopo deste passo é reavaliar a análise interna, não pesquisar projetos externos exaustivamente.

---

## 5. Veredicto

A recomendação de P849 (abordagem 4.2) **mantém-se**, mas com três ressalvas importantes:

1. **O esforço é provavelmente maior do que "1-2 passos".** Introduzir uma fase de realização equivalente ao vanilla toca na fronteira entre eval, show rules e layout. O cristalino já tem uma fase de expansão pós-introspecção (ADR-0118), mas estendê-la para métricas reais exige decisões arquitecturais novas.

2. **A Opção 1 (injeção de métricas no Engine durante a expansão de contexto) continua sendo tecnicamente mais barata e menos arriscada.** A rejeição dela em P849 foi por motivo de pureza de L1, não por inviabilidade. Se o dono reconsiderar a rigidez da fronteira L1/L3 neste ponto específico, a Opção 1 deve ser reavaliada.

3. **Não foram encontradas abordagens claramente superiores às já listadas.** As variantes híbridas ou limitadas adicionam complexidade sem reduzir o risco global.

---

## 6. Recomendação para DEBT-69

Atualizar DEBT-69 para refletir:

- A abordagem 4.2 é a recomendada, mas o esforço deve ser planeado como **2-3 passos**, não 1-2.
- O design deve reutilizar a fase de expansão pós-introspecção da ADR-0118, estendendo-a com acesso a métricas reais.
- A Opção 1 permanece como alternativa viável se a prioridade passar de "pureza absoluta de L1" para "custo/risco mínimo".
- Antes de prototipar, é recomendado um passo de design detalhado que defina: (a) onde o `Engine` de realização é construído, (b) como as métricas reais são injecadas, (c) como a re-introspecção pós-realização é feita, e (d) o impacto em documentos sem `measure()` (deve ser zero).

---

## 7. Validação

Nenhum código foi alterado neste passo. A suíte mantém-se verde:

- `cargo test --workspace`: **4649 passed; 0 failed** (estado após P851–P853).
- `crystalline-lint .`: exit 0 (aviso pré-existente V7 não relacionado).
