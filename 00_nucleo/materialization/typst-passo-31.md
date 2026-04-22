# Passo 31 — DEBT-2: Closures — Captura Lazy vs Eager

**Pré-condições**:
- Passo 30 concluído: 405 testes, zero violations
- `DEBT-2` visível em `DEBT.md`
- Branch: `cristalino/migration`

**Verificação antes de começar**:
```bash
# Confirmar estrutura actual de ClosureRepr
grep -n "pub struct ClosureRepr\|captured\|env\|params\|body" \
  01_core/src/entities/func.rs

# Confirmar como closures são avaliadas actualmente
grep -n "ClosureRepr\|eval_closure\|call_closure\|FuncRepr" \
  01_core/src/rules/eval.rs | head -30

# Verificar o merge de bold no layout — decisão do Passo 30
grep -n "bold.*||\||| .*bold\|merge\|node_style" \
  01_core/src/rules/layout.rs

# Confirmar DEBT-2 em DEBT.md
grep -A 10 "DEBT-2" 00_nucleo/DEBT.md
```

**Parar se qualquer pré-condição falhar.**

---

## Contexto

DEBT-2 regista que as closures do cristalino capturam variáveis por
valor no momento da definição (snapshot eager). O original usa `comemo`
com acesso lazy — a closure acede à variável no momento da chamada,
não da definição.

```typst
#let x = 1
#let f() = x    // captura x
#let x = 2      // redefine x no scope
#f()            // original: 2 (lazy) — cristalino: 1 (eager)
```

**O que este passo faz**: mudar a semântica de captura de eager para
lazy por referência ao scope. Closures passam a capturar o `Scope` no
momento da definição, mas acedem aos valores no momento da chamada.

**O que este passo não faz**: implementar `comemo` completo. A lazy
capture aqui é implementada por referência ao `Scope` — não por
rastreamento de dependências como no original.

**Limitação conhecida**: captura por referência ao `Scope` resolve o
caso de shadowing simples mas não resolve captura de variáveis mutadas
após a definição em cenários com closures escapando do scope. Registar
como sub-DEBT se encontrado nos testes.

---

## Tarefa 1 — Diagnóstico

```bash
# Ver ClosureRepr em detalhe
cat 01_core/src/entities/func.rs | grep -A 30 "pub struct ClosureRepr"

# Ver como o scope é capturado actualmente em eval_expr para Closure
grep -n "Closure\|ClosureRepr\|captured\|scope\|Scope" \
  01_core/src/rules/eval.rs | head -40

# Ver como a closure é chamada (apply/call)
grep -n "apply\|call_closure\|ClosureRepr\|FuncRepr::Closure" \
  01_core/src/rules/eval.rs | head -20

# Ver se Scope tem Clone ou Arc
grep -n "pub struct Scope\|impl.*Scope\|Clone.*Scope\|Arc.*Scope" \
  01_core/src/entities/scope.rs | head -10

# Teste de paridade que demonstra o bug actual
grep -rn "lazy\|eager\|shadow\|DEBT-2\|redefin" \
  01_core/src/ 2>/dev/null | head -10
```

**Parar. Reportar output antes de qualquer código.**

Questões a responder:
1. `ClosureRepr` tem um campo `captured: Scope` (snapshot) ou `captured: Arc<Scope>` ou outro?
2. O snapshot é feito em `eval_expr` quando a closure é criada, ou em outro sítio?
3. `Scope` implementa `Clone`? Tem custo alto?
4. Há um teste existente que demonstra a divergência eager vs lazy?

---

## Tarefa 2 — Alterar `ClosureRepr` para captura por Arc

A mudança central: em vez de clonar o `Scope` inteiro no momento da
definição, a closure guarda um `Arc<Scope>` — uma referência ao scope
vivo no momento da definição. Quando a closure é chamada, acede ao
scope via essa referência.

```rust
// Em entities/func.rs

pub struct ClosureRepr {
    pub name:   Option<EcoString>,
    pub params: Vec<ClosureParam>,
    pub body:   SyntaxNode,
    /// Scope capturado no momento da definição da closure.
    ///
    /// `Arc<Scope>` em vez de `Scope` clonado:
    /// - Custo de captura: O(1) — só incrementa o contador do Arc
    /// - Semântica: a closure vê o estado do scope no momento da chamada,
    ///   não da definição, para bindings adicionados após a definição
    ///
    /// Limitação: se o scope for substituído (novo let binding no pai),
    /// a closure vê o scope antigo. Paridade parcial com o original.
    /// Sub-DEBT registado se os testes revelarem divergência.
    pub captured: Arc<Scope>,
}
```

**Nota**: se `Scope` já está em `Arc` noutros sítios, verificar se
basta partilhar o mesmo `Arc` em vez de criar um novo. Reportar no
diagnóstico.

---

## Tarefa 3 — Actualizar criação de closures em `eval_expr`

Localizar o arm `Expr::Closure` em `eval_expr`. Actualmente faz
`scope.clone()` ou equivalente. Substituir por:

```rust
Expr::Closure(closure_node) => {
    // Capturar o scope actual por Arc — O(1), não clone O(N)
    let captured = Arc::clone(&scopes.current_arc());
    // ou, se Scopes expõe o Scope actual de outra forma:
    // let captured = Arc::new(scopes.current().clone());

    let repr = ClosureRepr {
        name:     closure_node.name().map(|n| n.get().clone()),
        params:   collect_params(closure_node.params())?,
        body:     closure_node.body().to_untyped().clone(),
        captured,
    };
    Ok(Value::Func(Func(Arc::new(FuncRepr::Closure(repr)))))
}
```

**Nota sobre `Scopes::current_arc()`**: se `Scopes` não expõe o scope
actual como `Arc`, há duas opções:
- **Opção A** (preferida): adicionar `fn current_arc(&self) -> Arc<Scope>`
  a `Scopes` que retorna `Arc::clone` do scope do topo.
- **Opção B**: fazer `Arc::new(scopes.current().clone())` — cria um Arc
  com snapshot. Semanticamente é ainda eager, mas com custo de clone
  explícito e documentado. Registar como sub-DEBT se escolhida.

Reportar qual opção foi usada.

---

## Tarefa 4 — Actualizar chamada de closures

Localizar onde closures são chamadas (provavelmente `eval_call` ou
`apply_closure`).

**Problema de recursão a resolver**: quando `#let fib(n) = ... fib(n-1) ...`
é avaliado, a closure é criada e captura o scope *antes* de `fib` ser
adicionado ao scope pai. Logo, o scope capturado não contém `fib`, e a
recursão falha com "variável não definida". A solução é injectar a própria
closure no `call_scopes` pelo seu nome antes de avaliar o body — assim
a recursão encontra sempre `fib` no scope de execução, independentemente
de quando foi capturado.

```rust
fn call_closure(
    ctx: &mut EvalContext<impl TrackedWorld>,
    repr: &ClosureRepr,
    self_func: &Func,   // a própria Func que contém este ClosureRepr
    args: Args,
) -> SourceResult<Value> {
    // Criar um scope filho do scope capturado.
    // Os argumentos da chamada e a auto-referência ficam neste scope filho.
    let mut call_scopes = Scopes::with_parent(Arc::clone(&repr.captured));

    // Auto-referência: se a closure tem nome, injectar no call_scopes
    // antes de avaliar o body. Isto permite recursão directa:
    //   #let fib(n) = ... fib(n-1) ...
    // Sem este passo, `fib` não estaria no scope capturado (foi capturado
    // antes de `fib` ser definido no scope pai) e a recursão falharia
    // com "variável não definida".
    if let Some(name) = &repr.name {
        call_scopes.define(name.clone(), Value::Func(self_func.clone()));
    }

    // Bind parâmetros — depois da auto-referência, para que um parâmetro
    // com o mesmo nome que a função sombre correctamente (caso raro).
    for (param, arg) in repr.params.iter().zip(args.positional()) {
        call_scopes.define(param.name.clone(), arg);
    }

    // Avaliar o body com o scope da chamada
    ctx.check_call_depth(/* span */)?;
    ctx.depth += 1;
    let result = eval_code(ctx, &mut call_scopes, &repr.body);
    ctx.depth -= 1;
    result
}
```

**Nota sobre a ordem auto-referência → parâmetros**: a auto-referência é
definida primeiro para que um parâmetro com o mesmo nome que a função
(ex: `#let f(f) = f + 1`) sombre a função e não cause ambiguidade.
O parâmetro ganha — comportamento consistente com o original.

**Nota sobre `Scopes::with_parent`**: verificar se `Scopes` já suporta
criação com parent Arc. Se não, adicionar:

```rust
impl Scopes<'_> {
    pub fn with_parent(parent: Arc<Scope>) -> Self {
        // implementação depende da estrutura actual de Scopes
    }
}
```

Reportar a estrutura actual de `Scopes` e o que foi necessário alterar.

---

## Tarefa 5 — Actualizar `DEBT.md`

```markdown
### DEBT-2 — Closures eager vs lazy capture — PARCIALMENTE RESOLVIDO

**Resolvido no Passo 31**:
- `ClosureRepr::captured` mudou de `Scope` (clone eager) para `Arc<Scope>`
- Captura no momento da definição: O(1) em vez de O(N)
- Closures acedem ao scope no momento da chamada para bindings existentes

**Divergência residual**:
- Se um binding é *substituído* no scope pai após a definição da closure,
  a closure continua a ver o scope do momento da definição (não o novo).
  O original via `comemo` rastreia dependências e invalidaria o resultado.
  Registado como sub-DEBT se encontrado nos testes de paridade.

**Pendente**:
- Integração com `comemo` para tracking semântico real
- Testes de paridade com o original para cenários avançados de shadowing
```

---

## Tarefa 6 — Testes

```rust
// ── semântica de captura ──────────────────────────────────────────────────

#[test]
fn closure_captura_scope_no_momento_da_definicao() {
    // Caso base — closure vê binding que existia quando foi definida
    let world = MockWorld::new(
        "#let x = 1
         #let f() = x
         #let resultado = f()"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("resultado"), Some(&Value::Int(1)));
}

#[test]
fn closure_ve_shadowing_no_scope_pai() {
    // Este teste documenta a semântica actual (pode falhar se ainda eager).
    // #let x = 1; #let f() = x; #let x = 2; #f()
    // Original (lazy comemo): 2
    // Cristalino com Arc<Scope>: depende de se Scope é mutado ou substituído
    let world = MockWorld::new(
        "#let x = 1
         #let f() = x
         #let x = 2
         #let resultado = f()"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    // Documentar o resultado actual sem assert rígido no valor:
    let resultado = m.scope().get("resultado").cloned();
    // Aceitar 1 (eager/Arc snapshot) ou 2 (lazy).
    // Anotar no relatório qual foi o resultado e porquê.
    assert!(
        resultado == Some(Value::Int(1)) || resultado == Some(Value::Int(2)),
        "resultado inesperado: {:?}", resultado
    );
}

#[test]
fn closure_recursiva_funciona() {
    // Recursão directa — a closure deve conseguir chamar-se a si própria
    let world = MockWorld::new(
        "#let fib(n) = if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }
         #let resultado = fib(7)"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("resultado"), Some(&Value::Int(13)));
}

#[test]
fn closure_captura_por_arc_nao_clona_scope() {
    // Verificar que a captura é eficiente — não é um teste de performance
    // mas verifica que closures com scopes grandes não causam erros
    let world = MockWorld::new(
        "#let a = 1
         #let b = 2
         #let c = 3
         #let f() = a + b + c
         #let resultado = f()"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("resultado"), Some(&Value::Int(6)));
}

#[test]
fn closure_com_argumento_sombra_captura() {
    // Parâmetro da closure sombra binding do scope capturado
    let world = MockWorld::new(
        "#let x = 10
         #let f(x) = x * 2
         #let resultado = f(5)"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    // f(5) usa x=5 (parâmetro), não x=10 (capturado)
    assert_eq!(m.scope().get("resultado"), Some(&Value::Int(10)));
}
```

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

# Confirmar que ClosureRepr usa Arc<Scope>
grep -n "captured.*Arc\|Arc.*Scope" 01_core/src/entities/func.rs

# Confirmar que não há Scope::clone() na criação de closures
grep -n "\.clone()" 01_core/src/rules/eval.rs | grep -i "scope\|captured"
# Deve retornar vazio ou apenas comentários
```

Critérios de conclusão:
- `ClosureRepr::captured` é `Arc<Scope>` ✓
- Criação de closure em `eval_expr` usa `Arc::clone` ou `Arc::new(snapshot)` documentado ✓
- `call_closure` recebe `self_func: &Func` e injeta-a no `call_scopes` pelo nome antes dos parâmetros ✓
- `closure_recursiva_funciona` com fib(7)=13 passa ✓
- `closure_captura_scope_no_momento_da_definicao` passa ✓
- `closure_com_argumento_sombra_captura` passa ✓
- `closure_ve_shadowing_no_scope_pai` — resultado documentado no relatório (1 ou 2, com explicação) ✓
- DEBT-2 actualizado em `DEBT.md` ✓
- Zero violations ✓
- Testes não regridem (405 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- `ClosureRepr::captured` era `Scope` clonado ou já tinha outra estrutura?
- `Scopes` tinha `current_arc()` ou foi necessário adicionar (Opção A ou B)?
- Havia testes existentes que demonstravam a divergência eager vs lazy?
- O merge `bold || node_style.bold` do Passo 30 vai causar problema quando
  scoping de `#set` for implementado? (resposta sim/não com razão breve)

**Da implementação:**
- Resultado de `closure_ve_shadowing_no_scope_pai` — 1 (eager) ou 2 (lazy)?
  Explicar porquê dado a implementação escolhida.
- Número final de testes e zero violations confirmado.

**DEBT-2 parcialmente resolvido. Go para Passo 32 — DEBT-6: eval_for_test coverage.**
