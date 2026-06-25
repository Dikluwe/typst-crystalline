# P461 — Correção de divergência arquitetural P459: `table_counter` → `CounterRegistry`

> **Passo:** 461  
> **Data:** 2026-06-25  
> **Foco:** (1) Mover contador de table do `Layouter` (campo local `table_counter: usize`) para o `CounterRegistry`/introspector com chave `"table"`, coerente com heading (P451), figure (P454) e equation (P456); (2) Corrigir prompt órfão `eval/table.md` de P459; (3) Atualizar tests e relatório de divergência.  
> **Tipo:** Correção arquitetural / Refacto mecânico.  
> **ADR-0117 Cláusula 4:** Aplica a lição de P454/P456: verificar padrão estabelecido antes de implementar; P459 violou ao colocar estado mutável no Layouter em vez de no oráculo.

---

## Contexto

P459 fechou table numbering com contador local mutável (`table_counter: usize`) no struct `Layouter`. Isso diverge do padrão da Trilha 1:

- **P451 (heading):** Número via `CounterRegistry`/`Introspector`.
- **P454 (figure):** Número via `CounterRegistry`/`Introspector` (chave `"figure"`).
- **P456 (equation):** Número via `CounterRegistry`/`Introspector` (chave `"equation"`).

O P459 criou três problemas:

1. **Incoerência arquitetural:** Contador de estado mutável no Layouter contradiz princípio de que contadores são oráculos selados.
2. **Bloqueio de Trilha 2 (label/ref):** Número da table não é locatable. `label`/`ref` e List of Tables não conseguem resolver número de table porque ele só existe em campo transitório do Layouter, não no `CounterRegistry`.
3. **Bug de re-layout:** Se houver re-layout (fixpoint TOC, multi-região, `measure()`), `table_counter` reinicia ou incrementa duplicado. `CounterRegistry` persiste entre passagens.

Este passo corrige a divergência movendo contador para o oráculo, coerente com resto da Trilha 1.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `table_counter: usize` existe no `Layouter`? | Sim — P459 (`rules/layout/mod.rs`) | ✅ |
| `CounterRegistry` com chave `"table"` existe? | Não — P459 usou campo local | ❌ |
| `CounterRegistry` com `"heading"`, `"figure"`, `"equation"`? | Sim — P451/P454/P456 | ✅ |
| `format_counter` existe? | Sim — P451 | ✅ |
| `table.numbering` na `StyleChain`? | Sim — P459 | ✅ |
| Bloqueadores? | Nenhum — refacto mecânico | ✅ |

**Reclassificação:** S (~15 min; remover campo local + adicionar chave ao oráculo + ajustar layout + atualizar tests).

---

## Toques pontuais

### 1. Remover `table_counter` do `Layouter`

**Ficheiro:** `01_core/src/rules/layout/mod.rs`

- Remover campo `table_counter: usize` do struct `Layouter`.
- Remover inicialização `table_counter: 0` em `Layouter::new`.
- Remover incremento `self.table_counter += 1` em `layout_table`.

### 2. Adicionar chave `"table"` ao `CounterRegistry`/`Introspector`

**Ficheiro:** `01_core/src/rules/introspect.rs` (ou onde `CounterRegistry` é populado)

- No walk de introspecção (ou eval que popula `CounterRegistry`), adicionar contagem de `Content::Table` com `table.numbering` na chain.
- Chave: `"table"`.
- Tipo: contador flat (1 dimensão, como `"figure"` e `"equation"`).

**Verificação:** Confirmar que `CounterRegistry` já tem mecanismo para adicionar chaves dinamicamente (heading/figure/equation foram adicionados em passos anteriores).

### 3. Layout de table lê do oráculo

**Ficheiro:** `01_core/src/rules/layout/table.rs`

- Em vez de `self.table_counter += 1`, usar:
  ```rust
  let table_number = introspector.flat_counter_at("table", current_location);
  // ou equivalente: counter_registry.get("table") + 1
  ```
- Formatar via `format_counter(&[table_number], pattern)`.
- Prefixar caption: `"Table {formatted}: " + caption_body`.

**Decisão:** Se o oráculo não expõe `flat_counter_at` diretamente para o layout, usar `CounterRegistry` via `ctx` (eval context) ou `introspector` passado ao layout. Verificar como figure/equation fazem (P454/P456) e replicar.

### 4. Tests — ajustar se necessário

- **L2 (layout):** `p459_table_caption_numbering_prefixo_acima` — deve continuar passando, mas agora o número vem do oráculo, não do campo local.
- **L3 (E2E):** `p459_table_source_sequencia_numerada` — verificar que sequência persiste corretamente via oráculo.
- **Novo teste:** `p461_table_counter_persiste_relayout` — simular dois layouts da mesma table e verificar que número não duplica (regressão do bug de re-layout).

### 5. Prompt órfão `eval/table.md`

**Problema:** Relatório P459 deixa warning: "prompt `eval/table.md` ainda não foi referenciado por arquivo dono único".

**Correção:**
- Opção A: Adicionar `@prompt-hash` em `01_core/src/rules/eval/rules.rs` (onde `table.numbering` é lido da chain) referenciando `eval/table.md`.
- Opção B: Se `eval/table.md` é redundante com `rules/eval/table.md` (ou similar), fundir ou renomear.
- Opção C: Registrar exceção no `crystalline-lint` (como feito em P437 para `square.md`).

**Recomendação:** Opção A — adicionar referência no ficheiro dono (`rules/eval/rules.rs` ou `rules/eval/mod.rs`) se `eval/table.md` documenta a leitura de `table.numbering` da chain. Se não há ficheiro dono claro, Opção C (exceção registada com justificativa: "prompt documenta gate de eval para table numbering, referenciado por múltiplos ficheiros").

### 6. Nota de divergência P459

**Ficheiro:** `00_nucleo/materialization/typst-passo-459-nota-divergencia.md` (novo)

```markdown
## Nota de divergência arquitetural — P459

**Data:** 2026-06-25  
**Autor:** Auditoria pós-P459  
**Referência:** Relatório P459, nota arquitetural; P451, P454, P456.

---

### Divergência

P459 implementou table numbering via contador local `table_counter: usize` no
`Layouter`, em vez de via `CounterRegistry`/introspector com chave `"table"`.

### Padrão estabelecido (Trilha 1)

| Passo | Elemento | Contador | Camada |
|-------|----------|----------|--------|
| P451 | Heading | `CounterRegistry` / `Introspector` | Eval / Oráculo |
| P454 | Figure | `CounterRegistry` / `Introspector` | Eval / Oráculo |
| P456 | Equation | `CounterRegistry` / `Introspector` | Eval / Oráculo |
| **P459** | **Table** | **`table_counter: usize` no `Layouter`** | **Layout (local)** |

### Consequências

1. **Incoerência arquitetural:** Estado mutável no Layouter vs. oráculo selado.
2. **Bloqueio de Trilha 2:** Número de table não é locatable; `label`/`ref` e LoT não conseguem resolver.
3. **Bug de re-layout:** `table_counter` reinicia em re-layout; oráculo persiste.

### Correção

Passo 461 move contador para `CounterRegistry` com chave `"table"`, coerente
com heading/figure/equation.

### Lição

A Cláusula 4 da ADR-0117 (verificar fronteiras/ADR vigentes antes de propor
estrutura) deveria ter impedido esta divergência. P456 aplicou a cláusula
corretamente (verificou P365 antes de propor campo em `MathElem`). P459 não
verificou o padrão de contadores estabelecido em P451/P454/P456 antes de
propor campo local no `Layouter`.
```

---

## Scope-out explícito

- **Não** adiciona funcionalidade nova — apenas corrige alinhamento arquitetural.
- **Não** altera comportamento observável (números devem ser idênticos em 1-pass).
- **Não** implementa `label`/`ref` para tables — isso é Trilha 2, desbloqueado por esta correção.
- **Não** altera `table.numbering` na `StyleChain` — já existe em P459.

---

## Critério de fecho

- [ ] `table_counter` removido do `Layouter`.
- [ ] `CounterRegistry`/`Introspector` com chave `"table"` populado.
- [ ] Layout de table lê número do oráculo, não de campo local.
- [ ] Tests L2/L3 de P459 continuam passando (paridade funcional).
- [ ] Novo teste de regressão: `p461_table_counter_persiste_relayout`.
- [ ] Prompt órfão `eval/table.md` resolvido (referência ou exceção).
- [ ] Nota de divergência P459 criada em `00_nucleo/materialization/`.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations novas.
- [ ] Trilha 1 marcada como **COMPLETA E COERENTE** no roteiro de conclusão.

---

## Próximo passo (Trilha 2 desbloqueada)

Com P461, Trilha 1 está completa e coerente. Trilha 2 (label/ref) está desbloqueada:
- **P462** — `label<x>`: Destinos nomeados (continuação de P460, se não executado)
- **P463** — `ref<x>` / `@x`: Resolução de destino + texto da referência
- **P464** — PDF `/Dests` + links internos `/GoTo`

**Aguardando sua indicação:**

1. **Executar o P461** (correção de divergência P459, ~15 min)?
2. **Escrever o P462** (Trilha 2: `label` — maior impacto estrutural)?
3. **Pivotar para outra trilha** (Trilha 3: Selector::Where, Trilha 4: visuais, Trilha 6: bibliografia Fase 2, Trilha 8: refinos)?
4. **Ajustar o escopo** do P461?
