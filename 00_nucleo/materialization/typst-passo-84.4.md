# Passo 84.4 — Partilha de listas quase-imutáveis em `EvalContext` (DEBT-22)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/eval.rs` — `EvalContext`, `apply_show_rules`,
  `intercept_content`, processamento de `Expr::ShowRule`,
  entrada/saída de `Expr::CodeBlock` e `Expr::ContentBlock`.
- `01_core/src/entities/show.rs` — `ShowRule`, `RuleId`.
- `00_nucleo/adr/typst-adr-0026-revisao-content-arc.md` — padrão
  `Arc<[T]>` já estabelecido para `Content::Sequence`; consistência
  arquitectural com este passo.
- `00_nucleo/DEBT.md` — DEBT-22 actual em Secção 1.

Pré-condição: `cargo test` — 904 testes (734 L1 + 170 L3, 6 ignorados
pré-existentes), zero violations. Passo 84.3 concluído. DEBT-21
encerrado.

---

## Restrições arquiteturais

1. **ADR-0029 — Pureza física**: `Arc<T>` em campos de struct é
   permitido e é o padrão canónico em L1 (ver `Module(Arc<ModuleInner>)`,
   `SyntaxNode(Arc<NodeData>)`, `Func(Arc<FuncRepr>)`).

2. **ADR-0030 — Performance de RAM é domínio**: reduzir clone O(n)
   no hot path para clone O(1) é **comportamento correcto**, não
   optimização especulativa. A justificativa textual da ADR-0030
   aplica-se literalmente ao DEBT-22: uma operação por nó AST, num
   percurso que visita milhares de nós, dobra o custo total de eval
   sem benefício.

3. **ADR-0026 revisão — Consistência com `Content::Sequence`**:
   `Arc<[T]>` foi estabelecido como padrão para sequências imutáveis
   após construção em L1. Novos tipos com o mesmo perfil devem seguir
   o mesmo padrão para facilitar manutenção por LLMs — tipos com
   papéis arquitectónicos idênticos não devem ter representações
   divergentes sem razão explícita.

4. **Sem `unsafe`** (convenção cristalina).

---

## Natureza deste passo

Este passo tem **duas fases separadas**, como o 84.3:

**Fase de diagnóstico (bloqueante)**: executar a Tarefa 1 e reportar
o output antes de escrever código. A decisão sobre `Arc<[T]>` vs
alternativas depende do padrão real de uso no projecto.

**Fase de implementação (dependente)**: escolher entre três variantes
com base no diagnóstico.

O diagnóstico cobre dois tipos no `EvalContext` que podem ter o mesmo
problema:
- `show_rules` — candidato principal documentado em DEBT-22.
- `active_guards` — candidato secundário, não coberto por DEBT mas
  potencialmente com o mesmo perfil.

---

## Tarefa 1 — Diagnóstico obrigatório

### 1.1 — Estrutura actual do `EvalContext`

```bash
# Campos de EvalContext
grep -B 2 -A 30 "pub struct EvalContext" 01_core/src/rules/eval.rs
```

Reportar o output completo. Os campos relevantes são `show_rules` e
`active_guards` — quaisquer outros com `Vec<T>` clonado por nó também
são candidatos.

### 1.2 — Padrão save/restore de `show_rules`

```bash
# Ocorrências de clone de show_rules
grep -n "show_rules.clone\|show_rules =" 01_core/src/rules/eval.rs

# Padrão de save/restore — `let saved = ctx.show_rules.clone()` seguido
# de `ctx.show_rules = saved` no final de um bloco?
grep -B 3 -A 15 "show_rules.clone()" 01_core/src/rules/eval.rs
```

Interpretar:
- Se há saves/restores explícitos em `Expr::CodeBlock` ou
  `Expr::ContentBlock`: **padrão A** — refcount ≥ 2 durante
  modificação, `make_mut` de `Arc<Vec<T>>` clonaria sempre.
- Se `show_rules` só cresce (push) durante eval, nunca volta atrás:
  **padrão B** — acumulação monotónica, `make_mut` pode dar O(1).
- Se há clone por nó em `intercept_content` mas o vector base não é
  restaurado: **padrão C** — clone defensivo, o original não é tocado.

### 1.3 — Padrão de push em `show_rules`

```bash
# Onde show_rules é modificado
grep -n "show_rules.push\|show_rules.extend" 01_core/src/rules/eval.rs

# Contar call sites aproximados de push vs clone
grep -c "show_rules.push" 01_core/src/rules/eval.rs
grep -c "show_rules.clone" 01_core/src/rules/eval.rs
```

O ratio push/clone valida o pressuposto "push raro, clone frequente".
Se push aparece mais vezes que clone nas contagens, é sinal de que o
pressuposto está errado — reportar.

### 1.4 — `active_guards` — mesmo diagnóstico

```bash
# Estrutura actual
grep -n "active_guards" 01_core/src/rules/eval.rs

# Padrão de push/pop — entrada/saída de show rules
grep -B 3 -A 5 "active_guards.push\|active_guards.pop" 01_core/src/rules/eval.rs

# Clone — existe ou não?
grep -n "active_guards.clone" 01_core/src/rules/eval.rs
```

Interpretar:
- Se há **push/pop frequente** (uma por entrada/saída de show rule) e
  **não há clone**: `active_guards` **não é um caso de DEBT-22**. Não
  se migra. Reportar sem alterar.
- Se há **push/pop frequente e clone**: problema duplo. Documentar
  separadamente, provavelmente abrir DEBT-39, não resolver neste
  passo.
- Se é puramente append-only como `show_rules`: entra no mesmo
  tratamento.

### 1.5 — Outros call sites de `show_rules`

```bash
# Todos os usos de show_rules em L1
grep -rn "show_rules" 01_core/src/

# API consumida: .iter(), .len(), .is_empty(), indexação?
grep -n "show_rules\." 01_core/src/rules/eval.rs | sort -u
```

Objectivo: inventariar exactamente os métodos chamados. `Arc<[T]>`
implementa `Deref<Target=[T]>`, logo `.iter()`, `.len()`, `.is_empty()`,
indexação directa `rules[i]` e `rules.first()`/`last()` funcionam sem
alteração. Se o diagnóstico revelar usos que exigem `Vec` especificamente
(ex: `.retain()`, `.drain()`), reportar — são impedimentos.

---

## Tarefa 1.5 — Classificação do cenário e decisão

Com base no diagnóstico, classificar em:

### Cenário A — `show_rules` segue save/restore, `active_guards` com push/pop

**Esperado** (pelo contexto do projecto conforme Passo 70 / DEBT-20).

Acção:
- `show_rules` → `Arc<[ShowRule]>`. Push reconstrói (O(n), raro).
- `active_guards` **fica fora do escopo**. Abrir DEBT-39.

### Cenário B — `show_rules` append-only, `active_guards` com push/pop

Acção:
- `show_rules` → `Arc<[ShowRule]>` (mesma conclusão — a diferença vs
  cenário A afecta apenas os call sites de restore, não a escolha do
  tipo).
- `active_guards` fica fora do escopo, mesma razão do cenário A.

### Cenário C — show_rules com api incompatível com `Arc<[T]>`

Se o diagnóstico 1.5 revelar métodos incompatíveis (ex: `retain`,
`drain`), a escolha muda para `Arc<Vec<ShowRule>>` com `make_mut`.
**Reportar ao utilizador antes de avançar** — este cenário contradiz
a expectativa e pode indicar um padrão arquitectural que merece
ADR separado.

### Cenário D — Diagnóstico revela que DEBT-22 não existe

Se `show_rules` no código actual já é `Arc<[ShowRule]>` ou estrutura
equivalente (talvez resolvido num passo anterior sem actualizar
DEBT.md), reportar e mover DEBT-22 directamente para Secção 2 com
nota "Resolvido implicitamente em Passo X; confirmado em 84.4".
Não alterar código.

---

## Tarefa 2A — Implementação (cenários A ou B)

### 2A.1 — Alterar o campo em `EvalContext`

Em `01_core/src/rules/eval.rs`:

**Antes**:

```rust
pub struct EvalContext {
    // ... outros campos ...
    pub show_rules: Vec<ShowRule>,
    // ... outros campos ...
}
```

**Depois**:

```rust
use std::sync::Arc;

pub struct EvalContext {
    // ... outros campos ...
    /// Show rules activas. Partilhada via Arc porque o clone acontece
    /// em cada nó AST durante intercept_content — clone O(1) é
    /// comportamento correcto (ADR-0030).
    /// Lista imutável após construção — push reconstrói (ADR-0026 revisão).
    pub show_rules: Arc<[ShowRule]>,
    // ... outros campos ...
}
```

Inicialização em `EvalContext::new()` (ou equivalente):

```rust
// Antes:
show_rules: Vec::new(),

// Depois:
show_rules: Arc::from(Vec::<ShowRule>::new().into_boxed_slice()),
// ou, equivalente e mais curto:
show_rules: Arc::from([]),
```

### 2A.2 — Método helper para push

```rust
impl EvalContext {
    /// Adiciona uma show rule reconstruindo o slice.
    ///
    /// Custo: O(n) onde n = número de rules actuais. Aceitável porque
    /// push é raro (uma vez por #show do utilizador) vs clone que é
    /// frequente (uma vez por nó AST).
    pub fn push_show_rule(&mut self, rule: ShowRule) {
        let mut rules = self.show_rules.to_vec();
        rules.push(rule);
        self.show_rules = Arc::from(rules);
    }
}
```

### 2A.3 — Substituir call sites

Baseado no diagnóstico 1.2 e 1.3:

**Clones**:

```rust
// Antes:
let rules = ctx.show_rules.clone();

// Depois:
let rules = Arc::clone(&ctx.show_rules);
```

**Pushs**:

```rust
// Antes:
ctx.show_rules.push(rule);

// Depois:
ctx.push_show_rule(rule);
```

**Save/restore (se cenário A)**:

```rust
// Antes:
let saved = ctx.show_rules.clone();
// ... body ...
ctx.show_rules = saved;

// Depois:
let saved = Arc::clone(&ctx.show_rules);
// ... body ...
ctx.show_rules = saved;
```

O save/restore continua a funcionar. A diferença é que `saved` é agora
um `Arc<[ShowRule]>` com contagem de referências incrementada — clone
O(1) em vez de clone O(n). No fim do bloco, a atribuição
`ctx.show_rules = saved` decrementa o refcount do `Arc` criado durante
o bloco; se havia push nesse bloco, o novo slice é descartado aqui.

### 2A.4 — Iteração sem alteração de API

`Arc<[T]>` deriva `Deref<Target=[T]>`, portanto todas estas chamadas
continuam a funcionar sem alteração:

```rust
ctx.show_rules.iter()       // OK
ctx.show_rules.len()        // OK
ctx.show_rules.is_empty()   // OK
&ctx.show_rules[i]          // OK
ctx.show_rules.first()      // OK
for rule in ctx.show_rules.iter() { ... }  // OK
```

Nenhum call site de leitura precisa de alteração.

### 2A.5 — Abrir DEBT-39 (se `active_guards` ficar fora)

Em `00_nucleo/DEBT.md`, Secção 1, adicionar:

```markdown
## DEBT-39 — `active_guards` com push/pop e potencial clone — EM ABERTO (Passo 84.4)

Diagnóstico do Passo 84.4 revelou que `EvalContext.active_guards`
tem padrão push/pop frequente (entrada/saída de cada show rule),
diferente de `show_rules` que é append-only. `Arc<[RuleId]>` com
reconstrução no push seria O(n) por entrada e O(n) por saída —
pior que o clone O(n) actual.

Padrão actual: [preencher com o que o diagnóstico revelou —
`clone()` existe? push/pop via Vec direct? mais informação sobre
o custo real]

Soluções candidatas:
- Manter `Vec<RuleId>` — aceitar o custo de clone se push/pop O(1)
  for mais valioso.
- Substituir clone por referência (&mut self explícito em
  intercept_content) — elimina clone sem mudar Vec.
- Estrutura persistente (im::Vector, rpds::List) — push/pop O(1)
  e clone O(1), mas introduz dependência externa.

Resolução: diagnóstico específico + decisão arquitectural em passo
dedicado (não 84.x deste bloco).
```

---

## Tarefa 2B — Implementação (cenário C)

Se o diagnóstico 1.5 revelou métodos incompatíveis com `Arc<[T]>`,
usar `Arc<Vec<ShowRule>>` com `make_mut`:

```rust
pub show_rules: Arc<Vec<ShowRule>>,

// Push:
Arc::make_mut(&mut ctx.show_rules).push(rule);

// Clone para eval:
let rules = Arc::clone(&ctx.show_rules);
```

**Mas antes de implementar este cenário, reportar ao utilizador**:
- Quais métodos obrigaram a esta escolha.
- Que isto diverge de ADR-0026 revisão (padrão estabelecido `Arc<[T]>`).
- Propor se faz sentido abrir ADR-0032 documentando a excepção, ou
  refactorar os call sites para eliminar os métodos incompatíveis
  (ex: substituir `drain` por reconstrução explícita).

---

## Tarefa 3 — Encerrar DEBT-22

Mover DEBT-22 da Secção 1 para Secção 2:

```markdown
## DEBT-22 — Clone de show_rules por nó — **ENCERRADO (Passo 84.4)** ✓

**Registado no Passo 68. Resolvido no Passo 84.4.**

`ctx.show_rules` migrada de `Vec<ShowRule>` para `Arc<[ShowRule]>`.
Clone reduzido de O(n) para O(1) por nó AST visitado em
`intercept_content`. Push reconstrói o slice — custo O(n) aceitável
porque push é raro (uma vez por #show do utilizador).

Padrão consistente com ADR-0026 revisão (Arc<[Content]> para
Content::Sequence). Novos tipos com papel arquitectónico semelhante
devem seguir o mesmo padrão.

Método helper `EvalContext::push_show_rule(&mut self, ShowRule)`
encapsula a reconstrução do slice. Leituras usam Deref<Target=[T]>
— API inalterada para call sites de iter/len/indexação.
```

Se `active_guards` ficou fora do escopo, verificar que DEBT-39 foi
aberto na Secção 1.

---

## Tarefa 4 — Verificação

```bash
# Testes
cargo test

# Linter
crystalline-lint .

# Confirmar que não há mais Vec<ShowRule> em EvalContext
grep -n "Vec<ShowRule>" 01_core/src/

# Confirmar que show_rules.clone() foi substituído por Arc::clone
grep -n "show_rules.clone()" 01_core/src/
# Esperado: zero ocorrências em código de produção (testes podem
# legitimamente usar clone() se o intent for "quero um Arc<[T]>
# independente" — mas no contexto deste projecto, Arc::clone é
# preferido por clareza).

# Confirmar que push directo não existe
grep -n "show_rules.push" 01_core/src/
# Esperado: zero ocorrências — todas via push_show_rule.
```

---

## Critérios de conclusão

- [ ] Tarefa 1 (diagnóstico) executada e reportada ao utilizador
  antes de qualquer edição de código.
- [ ] Cenário classificado (A, B, C, ou D) e reportado.
- [ ] Se cenário C, decisão do utilizador registada antes de codificar.
- [ ] Se cenário D, nenhum código alterado; DEBT-22 movido com nota.
- [ ] Se cenário A ou B:
  - [ ] `show_rules` migrada para `Arc<[ShowRule]>`.
  - [ ] Método `push_show_rule` adicionado.
  - [ ] Todos os `show_rules.clone()` substituídos por `Arc::clone(&...)`.
  - [ ] Todos os `show_rules.push()` substituídos por `push_show_rule()`.
  - [ ] API de leitura (iter, len, indexação) não precisou de alteração.
  - [ ] Save/restore continua a funcionar com `Arc<[T]>`.
  - [ ] DEBT-39 aberto se `active_guards` ficou fora.
- [ ] Nenhum uso de `unsafe`.
- [ ] `cargo test` — mesmo número de testes (734 L1 + 170 L3) ou +1 se
  um teste de regressão for adicionado.
- [ ] `crystalline-lint .` zero violations.
- [ ] DEBT-22 movido para Secção 2 com nota de resolução.

---

## Ao terminar, reportar

Bloco 1 — Diagnóstico:
- Output dos comandos da Tarefa 1 (resumido).
- Padrão identificado em `show_rules`: save/restore? append-only?
  clone defensivo?
- Padrão identificado em `active_guards`.
- Cenário classificado (A, B, C, ou D).

Bloco 2 — Implementação:
- Tarefa executada (2A ou 2B, ou nada em cenário D).
- Ficheiros alterados (esperado: `eval.rs` e `DEBT.md`).
- Número de call sites de clone e push substituídos.

Bloco 3 — Verificação:
- Número total de testes.
- Resultado do `crystalline-lint .`.
- Grep final de `Vec<ShowRule>` (esperado: zero ocorrências em L1).
- Se DEBT-39 foi aberto.

**Go/No-Go para o Passo 84.5** (DEBT-36 — operadores simbólicos de
alinhamento):

- **GO — `Arc<[ShowRule]>` funciona transparentemente**: testes passam,
  comportamento preservado, API de leitura inalterada.
- **NO-GO — regressão de performance visível**: se testes ficam
  significativamente mais lentos, verificar se `Arc::from(Vec::new())`
  está a ser chamado repetidamente em vez de `Arc::from([])` estático.
- **NO-GO — cenário C escolhido sem ADR**: se Claude Code avançou
  com `Arc<Vec<T>>` sem confirmação do utilizador ou ADR, reverter.
- **NO-GO — cenário D ignorado**: se o diagnóstico revelou que
  `show_rules` já era `Arc<[T]>` e Claude Code alterou o código
  mesmo assim, reverter.
