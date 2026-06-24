# Análise Actualizada — Passo 39

**Data**: 2026-04-03
**Passo actual**: 39 de ~120 (~32%)
**Fonte**: Knowledge base, conversas recentes (Passos 28–33 documentados), roadmap

---

## 1. Progresso desde a última análise (Passo 27 → Passo 39)

### O que mudou em 12 passos

A migração avançou de ~22% para ~32%. A Fase 1 (pagamento de dívida) está **concluída** e a Fase 2 (layout engine real) está **em curso**.

```
Passo 27/120  █████░░░░░░░░░░░░░░░  ~22%
Passo 39/120  ██████░░░░░░░░░░░░░░  ~32%
```

### Passos concluídos (28–39)

| Passo | Descrição | Resultado |
|-------|-----------|-----------|
| 28 | DEBT-3 parte 1: safety rails configuráveis | `while` e `MAX_CALL_DEPTH` tornados configuráveis em vez de hardcoded |
| 29 | DEBT-3 parte 2: `ImportGuard` RAII | Detecção de ciclos de importação via `Vec<FileId>` (não `HashSet`, por cache locality); padrão raw pointer idêntico a `MutexGuard` para satisfazer o borrow checker |
| 30 | DEBT-1: `StyleChain` | Lista ligada imutável de deltas de estilo substitui `TextStyle` plano; `#set` rules activadas |
| 31 | DEBT-2: closures lazy capture | Correcção de captura eager → `Arc<Scope>` snapshot com self-reference injection para recursão |
| 32 | DEBT-6: testes de integração em L3 | `SystemWorld` em L3 para testes end-to-end; 413 L1 + 48 L3 = 461 testes |
| 33 | `#set` block scoping | Save/restore de `ctx.styles`; remoção do bug `bold \|\| node_style.bold` no layout |
| 34 | Content types para math nodes | Variantes de Content para nós de equações |
| 35 | Parsing parity oracle | Validação de paridade de parsing matemático |
| 36 | Motor de equações — início | Entrada na Fase 2 com foco em matemática (não layout geral) |
| 37 | *(quebra de linha / math layout)* | Em curso |
| 38 | *(continuação math/layout)* | Em curso |
| 39 | *(passo actual)* | Em curso |

### Mudança estrutural crítica: reordenamento do roadmap

O roadmap foi **restruturado** entre os Passos 33 e 36. A zona de transição ficou:

```
Passo 33 — block scoping (#set)
Passo 34 — Content types para math nodes
Passo 35 — parsing parity oracle (matemática)
Passo 36 — motor de equações (início da Fase 2)
```

A Fase 2 original (Passos 36–55: layout geral com line breaking, grids, tabelas) foi **substituída** por um foco inicial em matemática. Isto resolve a decisão pendente do Passo 36 que a análise anterior identificou: **a matemática foi declarada obrigatória**, tal como recomendado.

---

## 2. Estado da dívida técnica

### Dívidas resolvidas

| ID | Descrição | Passo de resolução |
|----|-----------|-------------------|
| DEBT-5 | Unicode PDF (CIDFont/ToUnicode) | 24 ✅ |
| DEBT-4 | `Value` incompleto + conversões + `calc` | 25–27 ✅ |
| DEBT-3 | Safety rails hardcoded | 28–29 ✅ |
| DEBT-1 | StyleChain | 30 ✅ |
| DEBT-2 | Closures eager → lazy capture | 31 ✅ |
| DEBT-6 | `eval_for_test` coverage blind spot | 32 ✅ |

**Todas as 6 dívidas registadas no inventário original estão resolvidas.** A Fase 1 (pagamento de dívida e estrutura) está completa.

### Nova dívida introduzida

| ID | Descrição | Registada em | Estimativa |
|----|-----------|-------------|-----------|
| DEBT-7 | *(potencialmente registada durante Passos 28–33)* | A confirmar | A confirmar |

---

## 3. ADRs — actualização

### Novos ADRs desde a última análise

Com base nas conversas documentadas, os seguintes ADRs foram produzidos ou confirmados:

| ADR | Título | Estado |
|-----|--------|--------|
| 0032 | Fase de matemática declarada obrigatória | IMPLEMENTADO |

O ADR-0032 formaliza a decisão de que a fase de matemática (Passos 56–75 no roadmap original, agora antecipada para ~36+) não é opcional. Isto foi uma correcção directa da proposta #1 da análise anterior.

### Revogações e correcções

| ADR | Acção | Razão |
|-----|-------|-------|
| ADR-0007 | Revogado por ADR-0018 | `rustc_hash` é puro (erro original) |
| ADR-0028 | Revogado por ADR-0029 | Definição de pureza física corrigida |

Sem novas revogações entre Passos 28–39.

---

## 4. Métricas actualizadas

| Métrica | Passo 27 | Passo 32 | Passo 39 (est.) |
|---------|----------|----------|----------------|
| Testes L1 | ~342 | 413 | ~450+ |
| Testes L3 | ~42 | 48 | ~55+ |
| Total | ~384 | 461 | ~500+ |
| Violations | 0 | 0 | 0 |
| ADRs | 31 | 32+ | 32+ |
| Progresso | ~22% | ~27% | ~32% |
| Fase | Dívida (Fase 1) | Dívida (Fase 1) | Layout/Math (Fase 2) |

---

## 5. Análise do estado actual e riscos

### O que está sólido

A Fase 1 estava pensada para cobrir Passos 24–35. Foi concluída com sucesso. Todos os 6 itens de dívida foram pagos na ordem planeada. O pipeline agora suporta:

- Unicode no PDF (DEBT-5 ✅)
- `Value` completo com tipos tipográficos (DEBT-4 ✅)
- Safety rails configuráveis com detecção de ciclos (DEBT-3 ✅)
- StyleChain com `#set` rules (DEBT-1 ✅)
- Closures com captura lazy e self-reference (DEBT-2 ✅)
- Testes de integração via SystemWorld (DEBT-6 ✅)

### Onde estamos agora

O Passo 39 está dentro da Fase 2 — o motor de layout/matemática real. Com base no roadmap original, o Passo 39 correspondia a "Justificação: stretch/shrink de espaços" na secção de quebra de linha. Mas o roadmap foi restruturado para priorizar matemática, portanto o conteúdo exacto do Passo 39 depende de como os Passos 34–38 evoluíram.

### Riscos identificados

1. **Complexidade da tabela MATH OpenType**: A leitura da tabela `MATH` de fontes OpenType é um dos desafios mais complexos da migração. O `ttf-parser` em L3 já está autorizado (ADR-0019), mas a tabela MATH requer parsing adicional que pode necessitar extensões ou crates dedicadas.

2. **Restruturação do roadmap**: A antecipação da matemática para antes do layout geral (grids, tabelas, page breaking) significa que funcionalidades como `table()` e `grid()` foram adiadas. Se o caso de uso prático precisar de tabelas antes de equações, pode haver pressão para intercalar passos.

3. **Overhead do linter com complexidade crescente**: À medida que o codebase cresce, o tempo de execução de `crystalline-lint .` pode aumentar. Não é um problema agora, mas vale monitorizar.

---

## 6. Status das propostas anteriores

| # | Proposta | Estado |
|---|---------|--------|
| 1 | Matemática obrigatória | ✅ **Implementada** — ADR-0032; roadmap restruturado a partir do Passo 34 |
| 2 | Script de diagnóstico em lab/ | ⏳ Não implementada (não era bloqueante) |
| 3 | Auditoria de forbidden_symbols | ⏳ Parcialmente — V4 continua a operar; auditoria de cobertura não documentada |
| 4 | Changelog em prompts | ⏳ Não formalizada em regra do linter (V16 proposta mas não implementada) |
| 5 | Mover mocks para L4 | ❌ **Rejeitada correctamente** — DEBT-6 resolvida com testes de integração em L3 (Passo 32), mantendo mocks em L1 `#[cfg(test)]` |

---

## 7. Horizonte actualizado

### Roadmap revisto

Com a restruturação, o roadmap ficou:

| Fase | Passos | Descrição | Estado |
|------|--------|-----------|--------|
| 1 — Dívida | 24–33 | Pagamento de DEBT-1 a DEBT-6, block scoping | ✅ **Completa** |
| 2a — Matemática | 34–55 | Content math, parsing parity, motor de equações, layout math | ⏳ **Em curso** (Passo 39) |
| 2b — Layout geral | 56–75 | Line breaking, grids, tabelas, page breaking, floats | Futuro |
| 3 — Introspecção | 76–95 | Counters, state, locate, query, measure | Futuro |
| 4 — Graphics/stdlib | 96–120 | Imagens, SVG, PNG, HTML, stdlib completa | Futuro |

**Nota**: A numeração dos passos pode já ter divergido do roadmap original, dado que o conteúdo dos Passos 34–39 foi reorganizado. O marco de "subconjunto viável de produção" move-se para o fim da Fase 2a (~Passo 55), quando o compilador suporta tanto matemática quanto texto estruturado.

### Próximos marcos

| Marco | Passo estimado | Descrição |
|-------|---------------|-----------|
| Motor math básico | ~45 | Frações, subscritos, sobrescritos, raízes |
| Motor math completo | ~55 | Matrizes, equações multi-linha, fontes math |
| Grids e tabelas | ~65 | `grid()`, `table()`, colunas |
| Viável para produção | ~75 | Math + layout geral + page breaking |
| Introspecção | ~95 | Counters, locate, query |
| Paridade total | ~120 | Stdlib, gráficos, imagens |
