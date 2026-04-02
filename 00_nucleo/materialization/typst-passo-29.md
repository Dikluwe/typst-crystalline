# Passo 29 — DEBT-3 (parte 2): Detecção de ciclos de importação

**Pré-condições**:
- Passo 28 concluído: 389 testes, zero violations
- `max_call_depth` em `EvalContext` está em 250 (não 200 — verificar)
- `DEBT-3` marcado como parcialmente resolvido em `DEBT.md`
- Branch: `cristalino/migration`

**Verificação antes de começar**:
```bash
grep "max_call_depth" 01_core/src/rules/eval_context.rs
# Deve mostrar: max_call_depth: 250
# Se mostrar 200, corrigir antes de continuar.
```

---

## Contexto

DEBT-3 tem dois sub-problemas. O Passo 28 resolveu os limites de iteração
e profundidade de chamada. Este passo resolve a detecção de ciclos de
importação.

O cristalino ainda não tem `import` implementado — `Expr::ModuleImport`
e `Expr::ModuleInclude` existem no AST (herdados do parser) mas não são
avaliados em `eval_expr`. Quando `import` for implementado, precisará de
detectar ciclos: A importa B que importa A.

**O que este passo faz**: adicionar a estrutura de detecção de ciclos em
`EvalContext` agora, antes de implementar `import`. Assim, quando `import`
for implementado (Passo 33+), a infraestrutura já existe e é só usá-la.

Isto respeita o princípio da Arquitectura Cristalina: não criar lógica
sem entidades presentes, mas registar dívida e preparar a estrutura quando
o custo é baixo e o risco de esquecimento é alto.

**O que este passo não faz**: implementar `import` completo. `ModuleImport`
continua a retornar `Err("import não implementado")` no fim deste passo.

---

## Tarefa 1 — Diagnóstico

```bash
# Verificar se ModuleImport/ModuleInclude já têm tratamento em eval_expr
grep -n "ModuleImport\|ModuleInclude\|Import\|import" \
  01_core/src/rules/eval.rs | head -20

# Ver se há alguma estrutura de rastreamento de fontes em EvalContext
grep -n "visited\|cycle\|import_stack\|source_stack" \
  01_core/src/rules/eval_context.rs

# Ver o tipo de FileId — é Copy? Eq? Hash?
grep -n "FileId\|NonZeroU16" \
  01_core/src/entities/file_id.rs

# Ver se World::source já é chamado algures em eval
grep -n "world\.source\|ctx\.world\.source" \
  01_core/src/rules/eval.rs
```

**Parar. Reportar antes de qualquer código.**

---

## Tarefa 2 — Adicionar rastreamento de importações em `EvalContext`

```rust
// Em 01_core/src/rules/eval_context.rs

use crate::entities::file_id::FileId;

pub struct EvalContext<'world, W: TrackedWorld> {
    pub world: &'world W,
    pub depth: usize,
    pub max_call_depth: usize,
    pub loop_iterations: usize,
    pub max_loop_iterations: usize,
    /// Pilha de FileIds actualmente em avaliação por import.
    /// Usado para detectar ciclos: A → B → A.
    /// Vazio enquanto `import` não estiver implementado.
    ///
    /// Implementado como Vec (não HashSet): a pilha de importação tem
    /// normalmente < 20 elementos. Vec com pesquisa linear é mais rápido
    /// neste regime porque os dados ficam contíguos em memória (cache
    /// friendly). HashSet exige hashing em cada operação — custo injustificado
    /// para colecções tão pequenas.
    pub import_stack: Vec<FileId>,
}

impl<'world, W: TrackedWorld> EvalContext<'world, W> {
    pub fn new(world: &'world W) -> Self {
        Self {
            world,
            depth: 0,
            max_call_depth: 250,
            loop_iterations: 0,
            max_loop_iterations: 1_000_000,
            import_stack: Vec::new(),
        }
    }

    // ... check_call_depth e tick_loop do Passo 28 mantêm-se ...

    /// Tenta entrar na avaliação de `id`.
    /// Retorna Err se `id` já está na pilha (ciclo detectado).
    /// Retorna um guard que remove `id` da pilha quando largado.
    pub fn enter_import(
        &mut self,
        id: FileId,
        span: Span,
    ) -> SourceResult<ImportGuard<'_, 'world, W>> {
        if self.import_stack.contains(&id) {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "ciclo de importação detectado: ficheiro {:?} já está \
                     na pilha de importação activa",
                    id
                ),
            )]);
        }
        self.import_stack.push(id);
        Ok(ImportGuard { ctx: self, id })
    }
}

/// Guard RAII que remove o FileId da pilha quando largado.
/// Garante que a pilha fica limpa mesmo em caso de Err.
pub struct ImportGuard<'ctx, 'world, W: TrackedWorld> {
    ctx: &'ctx mut EvalContext<'world, W>,
    id: FileId,
}

impl<'ctx, 'world, W: TrackedWorld> Drop for ImportGuard<'ctx, 'world, W> {
    fn drop(&mut self) {
        // Vec::retain remove a última ocorrência. Como cada FileId aparece
        // no máximo uma vez na pilha (enter_import verifica), isto é correcto.
        self.ctx.import_stack.retain(|id| id != &self.id);
    }
}
```

**Nota**: `HashSet` de `std` é suficiente aqui — `import_stack` tem no
máximo tantos elementos quantos os ficheiros na cadeia de importação
(normalmente < 20). Não justifica `FxHashSet`.

**Verificação obrigatória sobre `FileId`**: antes de implementar, confirmar
no diagnóstico (Tarefa 1) que `FileId` deriva `PartialEq` e `Eq`. Sem isso
`Vec::contains` não compila. Se não estiverem derivados, adicionar em
`entities/file_id.rs` antes de qualquer código neste passo.

---

## Tarefa 3 — Tratar `ModuleImport` em `eval_expr`

Localizar onde `Expr::ModuleImport` é tratado (ou não) em `eval_expr`.
Se estiver ausente ou a retornar panic/todo, substituir por:

```rust
Expr::ModuleImport(_import) => {
    // import não implementado — Passo 33+
    // A estrutura de detecção de ciclos (EvalContext::enter_import)
    // está pronta para uso quando import for implementado.
    Err(vec![SourceDiagnostic::error(
        _import.span(),
        "import não implementado nesta versão do cristalino",
    )])
}

Expr::ModuleInclude(_include) => {
    Err(vec![SourceDiagnostic::error(
        _include.span(),
        "include não implementado nesta versão do cristalino",
    )])
}
```

O objectivo é substituir qualquer `todo!()`, `unimplemented!()`, ou
ausência de arm por um erro de diagnóstico limpo — sem panic.

---

## Tarefa 4 — Actualizar `DEBT.md`

Marcar DEBT-3 como totalmente resolvido na parte estrutural:

```markdown
### DEBT-3 — Safety rails hardcoded — RESOLVIDO (estrutura)

**Resolvido no Passo 28**:
- `while` limit: 10.000 → 1.000.000, via `EvalContext::tick_loop()`
- `MAX_CALL_DEPTH`: 200 → 250, via `EvalContext::check_call_depth()`

**Resolvido no Passo 29**:
- Detecção de ciclos de importação: `EvalContext::enter_import()` + `ImportGuard`
- `ModuleImport` e `ModuleInclude` retornam Err limpo (não panic)

**Pendente (não é DEBT — é feature futura)**:
- Implementação de `import` completo (Passo 33+)
- Integração com `comemo` para tracking semântico real (aguarda TrackedWorld real)

**Ficheiros alterados**: `rules/eval_context.rs`, `rules/eval.rs`
```

---

## Tarefa 5 — Testes

```rust
// ── import_stack — testes directos de EvalContext ──────────────────────

#[test]
fn enter_import_sem_ciclo_passa() {
    let world = MockWorld::new("");
    let mut ctx = EvalContext::new(&world);
    let id_a = FileId::from_raw(1);  // ajustar conforme API de FileId
    let span = Span::detached();

    let guard = ctx.enter_import(id_a, span).unwrap();
    assert!(ctx.import_stack.contains(&id_a));
    drop(guard);
    assert!(!ctx.import_stack.contains(&id_a));
}

#[test]
fn enter_import_ciclo_retorna_err() {
    let world = MockWorld::new("");
    let mut ctx = EvalContext::new(&world);
    let id_a = FileId::from_raw(1);
    let span = Span::detached();

    let _guard = ctx.enter_import(id_a, span).unwrap();
    // Tentar entrar no mesmo id — deve falhar
    let result = ctx.enter_import(id_a, span);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err[0].message.contains("ciclo"),
        "mensagem deve mencionar 'ciclo', foi: {}", err[0].message);
}

#[test]
fn guard_remove_id_mesmo_em_err() {
    let world = MockWorld::new("");
    let mut ctx = EvalContext::new(&world);
    let id_a = FileId::from_raw(1);
    let span = Span::detached();

    {
        let _guard = ctx.enter_import(id_a, span).unwrap();
        // guard largado aqui
    }
    // Após drop, deve ser possível entrar de novo (sem ciclo)
    let result = ctx.enter_import(id_a, span);
    assert!(result.is_ok());
}

// ── ModuleImport retorna Err limpo (não panic) ────────────────────────────

#[test]
fn eval_import_retorna_err_sem_panic() {
    let world = MockWorld::new("#import \"foo.typ\": bar");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    // Deve retornar Err (import não implementado), não panic
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err[0].message.contains("import") || err[0].message.contains("não implementado"));
}

#[test]
fn eval_include_retorna_err_sem_panic() {
    let world = MockWorld::new("#include \"foo.typ\"");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err());
}
```

**Nota sobre `FileId::from_raw`**: verificar a API actual de `FileId`
(é um `NonZeroU16` newtype). Se não há construtor público para testes,
usar `MockWorld::new("")` e obter um `FileId` via `world.main()`.
Ajustar os testes conforme necessário e reportar.

---

## Verificação final

```bash
cargo test -p typst-core 2>&1 | tail -5
cargo test -p typst-infra 2>&1 | tail -5
cargo build 2>&1 | tail -5
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found

# Confirmar import_stack como Vec em EvalContext
grep -n "import_stack\|enter_import\|ImportGuard" \
  01_core/src/rules/eval_context.rs

# Confirmar que import/include não fazem panic
grep -n "todo!\|unimplemented!\|panic!" \
  01_core/src/rules/eval.rs
# Deve retornar vazio
```

Critérios de conclusão:
- `EvalContext` tem `import_stack: Vec<FileId>` (não HashSet — justificado no código) ✓
- `enter_import()` detecta ciclos via `Vec::contains` e retorna Err com mensagem que inclui o FileId ✓
- `ImportGuard` remove o FileId via `Drop` com `Vec::retain` ✓
- `ModuleImport` e `ModuleInclude` em `eval_expr` retornam Err limpo — sem panic, sem todo! ✓
- Testes `enter_import_sem_ciclo_passa`, `enter_import_ciclo_retorna_err`,
  `guard_remove_id_mesmo_em_err` passam ✓
- Testes `eval_import_retorna_err_sem_panic`, `eval_include_retorna_err_sem_panic` passam ✓
- DEBT-3 marcado como resolvido (estrutura) em `DEBT.md` ✓
- Zero violations ✓
- Testes não regridem (389 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- `ModuleImport`/`ModuleInclude` tinham `todo!()`, `panic!()`, arm ausente, ou já retornavam Err?
- `FileId` já implementava `Hash` + `Eq`, ou foi necessário derivar?
- Havia algum uso de `world.source()` em `eval.rs` antes deste passo?

**Da implementação:**
- API usada nos testes para construir `FileId` (from_raw, world.main(), ou outra)
- Número final de testes e zero violations confirmado.

**DEBT-3 encerrado. Go para Passo 30 — DEBT-1: StyleChain.**
