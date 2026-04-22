# Passo 92 — Integração estrutural de `Route<'a>` no `EvalContext`, pagar DEBT-44

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0036-*.md` — princípio de atomização
  progressiva. Este passo é a primeira aplicação concreta.
- `00_nucleo/adr/typst-adr-0033-*.md` — paridade funcional com
  vanilla. Comportamento observável não muda.
- `00_nucleo/DEBT.md` — entrada DEBT-44 (será movida para
  Secção 2). Entrada DEBT-45 fica em aberto (dependência do
  DEBT-44 resolver-se primeiro, Passo 93).
- `00_nucleo/diagnosticos/diagnostico-route-vanilla-passo-85.md`
  — forma do `Route<'a>` no vanilla e como é propagado.
- `01_core/src/entities/world_types.rs` — `Route<'a>` já
  materializado (Passo 90).
- `01_core/src/rules/eval.rs` — estado actual do `EvalContext<'w>`
  com campo `route: Vec<FileId>` e API `with_route_id`.
- `lab/typst-original/crates/typst-library/src/engine.rs` —
  como o vanilla propaga `Route<'a>` via `Engine<'a>`.

Pré-condição: `cargo test` — 747 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 91.5 concluído (ADR-0036 em vigor).

---

## Natureza deste passo

Passo único de construção. Paga DEBT-44 integrando `Route<'a>`
estruturalmente no `EvalContext`, eliminando o campo
`route: Vec<FileId>` e a API `with_route_id` introduzidos como
solução pragmática no Passo 90.

É o primeiro pagamento concreto da ADR-0036. Remove um campo de
estado partilhado (`Vec<FileId>`) e substitui-o por propagação
estrutural (`Route<'a>` que vive no contexto com lifetime
próprio ou é passado como parâmetro em funções de recursão).

Escopo restrito ao DEBT-44. **Não toca** nos `check_*_depth`
(DEBT-45, fica para Passo 93).

---

## Decisões formalizadas neste passo

- ADR-0036 (atomização progressiva) — aplicação concreta. O
  `Route<'a>` passa de "existe mas não é usado estruturalmente"
  para "declarado nas assinaturas das funções que o requerem".
- ADR-0033 (paridade funcional) — comportamento observável
  preservado. Teste E2E `import_cycle_detectado_retorna_err_sem_panic`
  continua a passar sem alteração.
- ADR-0034 (diagnóstico obrigatório) — satisfeito pelo diagnóstico
  do Passo 85 e pela materialização do Passo 90.

---

## Estratégia de integração

O Passo 90 classificou o `EvalContext` como **Classe A**
(`EvalContext<'w>` já tem lifetime). Duas sub-opções:

**Opção A.1 — Route como campo do EvalContext**

```rust
pub struct EvalContext<'w> {
    // ... outros campos ...
    pub route: Route<'w>,
}
```

- Vantagem: propagação automática (funções acedem via
  `ctx.route`).
- Desvantagem: `Route<'a>` é linked list imutável — mutar via
  `extend` cria novo `Route`, não muta o existente. Incompatível
  com armazenamento como campo mutável.

**Opção A.2 — Route como parâmetro explícito**

```rust
fn eval_expr(&mut self, route: &Route<'_>, expr: &Expr) -> Value
```

- Vantagem: semântica de linked list preservada. Cada frame cria
  seu próprio `Route` via `parent_route.extend(id)`.
- Vantagem: alinha com ADR-0036 — assinatura declara dependência.
- Desvantagem: requer alterar assinatura de ~12 funções `eval_*`.

**Escolha recomendada: Opção A.2**. Justificação: é a opção que
o vanilla usa e que ADR-0036 prescreve. A Opção A.1 é sintaxe
conveniente mas destrói a semântica de linked list (`Route<'a>`
como campo mutável é contradição estrutural).

Se durante a execução surgirem obstáculos não previstos
(lifetimes impossíveis de reconciliar, recursão mútua que
propaga o `Route` de forma problemática), **parar** e reportar
antes de prosseguir — pode exigir ajustes ao enunciado.

---

## Sequência de tarefas

### Tarefa 1 — Mapear pontos de mudança

#### 1.1 — Identificar funções `eval_*` que precisam de `Route`

```bash
# Todas as funções eval_*:
grep -n "fn eval_\|pub fn eval\b" 01_core/src/rules/eval.rs

# Funções que actualmente usam route_contains ou with_route_id:
grep -n "route_contains\|with_route_id\|\.route\b" \
    01_core/src/rules/eval.rs

# Ponto de entrada (eval público):
grep -B 2 -A 10 "pub fn eval\b" 01_core/src/rules/eval.rs | head -20
```

Esperado: entre 8 e 15 funções que directa ou indirectamente
participam na recursão de eval.

**Classificação necessária**:

- **Tier 1 — Funções que precisam de `route`**: acessam o
  mecanismo actual (Expr::ModuleInclude, Expr::ModuleImport).
  Precisam de `route: &Route<'_>` na assinatura.
- **Tier 2 — Funções intermédias**: chamam funções Tier 1.
  Precisam propagar `route` mesmo que não o usem directamente.
- **Tier 3 — Funções folha**: não chamam nada que precise de
  `route`. Não precisam alterar assinatura.

Reportar contagem por tier antes de prosseguir.

#### 1.2 — Identificar ponto de entrada

O ponto de entrada público (`pub fn eval(...)`) cria o `Route`
inicial via `Route::root().with_id(main_file_id)`. Confirmar
onde isto acontece actualmente:

```bash
grep -B 2 -A 15 "pub fn eval\b\|EvalContext::new" \
    01_core/src/rules/eval.rs 01_core/src/rules/eval/*.rs \
    2>/dev/null
```

### Tarefa 2 — Refactor das assinaturas

#### 2.1 — Adicionar `route: &Route<'_>` nas Tier 1 e Tier 2

Para cada função dos tiers 1 e 2, adicionar o parâmetro **como
penúltimo** (antes do parâmetro principal, ex: `expr`, `markup`):

```rust
// Antes:
fn eval_expr(&mut self, expr: &Expr) -> SourceResult<Value>

// Depois:
fn eval_expr(&mut self, route: &Route<'_>, expr: &Expr) -> SourceResult<Value>
```

**Razão da posição**: convenção vanilla; também legível —
`ctx.eval_expr(&route, &expr)` reflecte a ordem natural.

#### 2.2 — Actualizar chamadores

Cada chamada a função Tier 1 ou Tier 2 passa a receber `route`
explicitamente. Em funções Tier 2, propagar o `route` recebido.
Em funções Tier 1, criar novo `Route` via `extend` antes da
recursão:

```rust
// Em eval_module_include (Tier 1):
let new_route = route.extend(src_id);
eval_markup(ctx, &new_route, &body)?
```

#### 2.3 — Verificação intermédia

```bash
cargo check --package typst-core 2>&1 | tail -30
```

Esperado: compila. Se não compilar, depurar antes da Tarefa 3.

Problemas comuns:
- **Lifetime errors**: o `Route<'a>` propagado tem lifetime da
  função que o criou. Se for armazenado em estrutura com
  lifetime diferente, há incompatibilidade. Solução: garantir
  que não é armazenado — só propagado.
- **Recursão mútua**: se `eval_expr` chama `eval_markup` que
  chama `eval_expr`, ambos precisam do parâmetro. Propagar
  mecanicamente.

### Tarefa 3 — Remover mecanismo antigo

Depois do refactor compilar, remover:

- Campo `route: Vec<FileId>` de `EvalContext`.
- Método `route_contains` de `EvalContext`.
- Método `with_route_id` de `EvalContext`.
- Pré-preenchimento `current_file` em `EvalContext::new`
  (se só existia para manter `route` sincronizado).

Manter:

- `EvalContext::new` criar e armazenar qualquer outro estado
  legítimo.
- `check_call_depth` antigo em `EvalContext` (parte do DEBT-45,
  não tocado neste passo).

### Tarefa 4 — Actualizar ponto de entrada

Em `pub fn eval(...)`:

```rust
pub fn eval(world: Tracked<dyn World>, source: &Source) -> SourceResult<Module> {
    let route = Route::root().with_id(source.id());
    let mut ctx = EvalContext::new(world, source);
    eval_markup(&mut ctx, &route, source.root())?
    // ...
}
```

Adapta-se ao ponto de entrada real do cristalino. O princípio é:
`Route::root().with_id(main_id)` no entry point, propagado para
todas as chamadas recursivas.

### Tarefa 5 — Actualizar testes

Testes que usam `with_route_id` ou acedem `ctx.route` directamente:

```bash
grep -n "with_route_id\|\.route\b\|route_contains" \
    01_core/src/rules/eval.rs 01_core/src/rules/eval/*.rs \
    2>/dev/null
```

Reescrever para usar `Route<'_>` directamente:

```rust
// Antes:
ctx.with_route_id(id, span, |ctx| eval_markup(ctx, &body))

// Depois:
let route = Route::root().with_id(main_id);
let new_route = route.extend(id);
eval_markup(&mut ctx, &new_route, &body)
```

Os testes `with_route_id_*` introduzidos no Passo 90 precisam
de ser reescritos ou removidos — eram específicos ao mecanismo
intermédio que agora desaparece. O teste E2E
`import_cycle_detectado_retorna_err_sem_panic` **não é alterado**
— valida comportamento observável, independente da mecânica
interna.

### Tarefa 6 — Verificação final

```bash
# Contagem de testes (esperado: 747 L1 + 174 L3 + 6 ignorados
# mantidos, menos testes específicos de with_route_id removidos
# se forem substituídos):
cargo test --workspace 2>&1 | tail -10

# Linter:
cargo run --package crystalline-lint 2>&1 | tail -5

# Campo route removido de EvalContext:
grep -n "pub route:\|self\.route\b\|ctx\.route\b" \
    01_core/src/rules/eval.rs 01_core/src/rules/eval/*.rs \
    2>/dev/null

# Zero unsafe em eval:
grep -n "unsafe" 01_core/src/rules/eval.rs
```

Esperado:
- Contagem de testes: 747 L1 ± diferença de testes
  removidos/adicionados (reportar se divergir).
- Zero violations no linter.
- `grep` do campo `route` no `EvalContext`: zero resultados.
- `grep` de `unsafe` em `eval.rs`: zero.

### Tarefa 7 — Fechar DEBT-44

Mover a entrada DEBT-44 da Secção 1 (em aberto) para a Secção 2
(encerrados). Inserir após DEBT-40 (ou DEBT-41 conforme ordem
actual da Secção 2). Preservar o texto original e acrescentar
linha final:

```markdown
**Resolvido no Passo 92.** `EvalContext.route: Vec<FileId>` e
API `with_route_id` eliminados. `Route<'a>` agora é propagado
como parâmetro `route: &Route<'_>` nas funções `eval_*` que
participam na recursão, criando novo frame via `route.extend(id)`
antes de cada chamada recursiva. Primeira aplicação concreta da
ADR-0036 (atomização progressiva). DEBT-45 (check_*_depth não
chamados) continua em aberto para Passo 93.
```

---

## Critérios de conclusão

- [ ] Campo `route: Vec<FileId>` removido de `EvalContext`.
- [ ] Métodos `route_contains` e `with_route_id` removidos de
      `EvalContext`.
- [ ] Funções `eval_*` em tier 1 e 2 têm parâmetro
      `route: &Route<'_>` na assinatura.
- [ ] Ponto de entrada `eval()` cria `Route::root().with_id(id)`
      e propaga.
- [ ] Recursão em módulos (`ModuleInclude`, `ModuleImport`) cria
      novo `Route` via `route.extend(id)`.
- [ ] Teste E2E `import_cycle_detectado_retorna_err_sem_panic`
      passa sem alteração.
- [ ] Nenhum novo `unsafe` introduzido em L1.
- [ ] DEBT-44 movido para Secção 2 do `DEBT.md` com linha de
      resolução que cita ADR-0036.
- [ ] DEBT-45 permanece em aberto (não é tocado).
- [ ] `cargo run --package crystalline-lint` reporta zero
      violations.
- [ ] Nenhum ADR alterado (ADR-0036 continua `EM VIGOR`, não é
      revisto).

---

## Ao terminar, reportar

Tarefa 1 (mapeamento):
- Número de funções por tier (1, 2, 3).
- Ponto de entrada identificado.

Tarefa 2 (refactor assinaturas):
- Número de funções alteradas.
- Linhas alteradas em `eval.rs` e submódulos.

Tarefa 3 (remover mecanismo antigo):
- Confirmação de campo e métodos removidos.

Tarefa 4 (ponto de entrada):
- Forma final da criação do `Route` inicial.

Tarefa 5 (testes):
- Testes removidos (ex: `with_route_id_*`).
- Testes adicionados (se algum for necessário para cobertura
  perdida).
- Teste E2E `import_cycle_detectado_retorna_err_sem_panic`
  confirmado a passar sem modificação.

Tarefa 6 (verificação):
- Contagem final de testes.
- Zero violations.

Tarefa 7 (DEBT-44):
- Posição final do DEBT-44 na Secção 2.

Go/No-Go para Passo 93:
- **Go** se todas as tarefas concluídas e testes verdes. Passo
  93 = pagar DEBT-45 (ligar `check_show_depth`,
  `check_layout_depth`, `check_html_depth` nos pontos correctos
  do eval). Agora que `Route<'a>` é propagado, chamar
  `route.check_X_depth()?` é trivial.
- **No-Go parcial** se a Tarefa 2 revelar que o número real de
  funções a alterar é significativamente maior que o previsto
  (ex: 30+ em vez de ~12). Nesse caso, avaliar se:
  - Continuar com o esforço adicional (pode ser aceitável se o
    refactor for mecânico).
  - Dividir em sub-passos por submódulo de eval (cada submódulo
    refactorado independentemente).
  - Reverter e reconsiderar estratégia (Opção A.1 com
    compromisso semântico, ou camada intermédia).

Reportar a contagem real na Tarefa 1 antes de avançar para a
Tarefa 2 permite decisão antes de refactor grande.
