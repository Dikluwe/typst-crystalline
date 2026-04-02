# Passo 28 — DEBT-3 (parte 1): Safety rails — while limit e call depth

**Pré-condições**:
- Passo 27 concluído: 384 testes, zero violations
- `DEBT-3` visível em `DEBT.md` com os dois sub-problemas documentados
- Branch: `cristalino/migration`

---

## Contexto

DEBT-3 registou dois limites hardcoded em `rules/eval.rs`:

1. **`while` com 10.000 iterações máximo** — falha em documentos legítimos
   com loops longos (ex: processamento de listas grandes).
2. **`MAX_CALL_DEPTH: 200`** — arbitrário e sem documentação. O original
   suporta 1.000 via `comemo` com rastreamento de stack separado. Sem esse
   mecanismo, subir directamente para 1.000 causaria `SIGSEGV` em modo debug
   — cada frame Rust em debug é pesado e a stack de 2–8 MB esgota antes dos
   500 frames. O valor correcto sem `comemo` é 250: defensivo contra overflow,
   alto o suficiente para recursão legítima moderada.

O original usa `comemo` para detecção semântica de ciclos (memoização
com rastreamento de dependências). Não vamos implementar `comemo` neste
passo — isso é trabalho para quando `TrackedWorld` real estiver em uso.
O que fazemos aqui é remover os limites hardcoded arbitrários e
substituir por limites documentados e configuráveis.

Este passo **não** introduz detecção semântica de ciclos. Isso fica para
o Passo 29 (ou mais tarde, quando `comemo` for integrado com
`TrackedWorld`).

**Divisão DEBT-3**:
- **Passo 28** (este): `while` limit → configurável via `EvalContext`;
  `MAX_CALL_DEPTH` → aumentado para valor defensivo documentado.
- **Passo 29**: detecção de ciclos de importação (quando `import` for implementado).

---

## Tarefa 1 — Diagnóstico

```bash
# Encontrar os safety rails actuais
grep -n "10_000\|10000\|MAX_CALL_DEPTH\|CALL_DEPTH\|max_depth\|iter_limit\|loop_limit" \
  01_core/src/rules/eval.rs

# Ver a estrutura actual de EvalContext
grep -n "EvalContext\|pub struct EvalContext\|depth" \
  01_core/src/rules/eval_context.rs

# Ver como depth é usado em eval
grep -n "ctx\.depth\|context\.depth\|\.depth\s*[+>]" \
  01_core/src/rules/eval.rs | head -20

# Confirmar que não há outros limites hardcoded noutros ficheiros
grep -rn "10_000\|MAX_ITER\|MAX_LOOP\|MAX_CALL" \
  01_core/src/ 03_infra/src/ 2>/dev/null
```

**Parar. Reportar o output antes de qualquer código.**

Questões a responder com o diagnóstico:
1. O limite `while` está em `eval.rs` como constante ou inline?
2. `MAX_CALL_DEPTH` está em `eval.rs` ou `eval_context.rs`?
3. `EvalContext` já tem um campo `depth: usize` ou a profundidade é
   rastreada de outra forma?
4. Há mais algum limite hardcoded não registado em DEBT-3?

---

## Tarefa 2 — Actualizar `EvalContext`

O objectivo é mover os limites para `EvalContext` de forma a serem
visíveis e documentados, sem serem configuráveis pelo utilizador final
(não há API pública para isso — é apenas organização interna).

```rust
// Em 01_core/src/rules/eval_context.rs

/// Contexto de avaliação passado por toda a árvore de eval.
///
/// Limites de segurança:
/// - `max_loop_iterations`: impede loops infinitos. O original usa
///   detecção semântica via comemo; aqui usamos um limite alto mas finito.
///   Valor: 1_000_000 (suficiente para documentos reais; comemo virá depois).
/// - `max_call_depth`: impede recursão infinita. Rust faz stack overflow
///   antes de ~500 frames em modo debug; 250 é defensivo sem ser arbitrário.
///   O original suporta 1_000 via comemo com stack separada.
pub struct EvalContext<'world, W: TrackedWorld> {
    pub world: &'world W,
    pub depth: usize,
    pub max_call_depth: usize,
    pub loop_iterations: usize,   // contador acumulado no eval actual
    pub max_loop_iterations: usize,
}

impl<'world, W: TrackedWorld> EvalContext<'world, W> {
    pub fn new(world: &'world W) -> Self {
        Self {
            world,
            depth: 0,
            max_call_depth: 250,
            loop_iterations: 0,
            max_loop_iterations: 1_000_000,
        }
    }

    /// Retorna Err se a profundidade máxima foi atingida.
    pub fn check_call_depth(&self, span: Span) -> SourceResult<()> {
        if self.depth >= self.max_call_depth {
            Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "profundidade máxima de chamadas atingida ({}) — \
                     possível recursão infinita",
                    self.max_call_depth
                ),
            )])
        } else {
            Ok(())
        }
    }

    /// Incrementa o contador de iterações e retorna Err se o limite foi atingido.
    pub fn tick_loop(&mut self, span: Span) -> SourceResult<()> {
        self.loop_iterations += 1;
        if self.loop_iterations > self.max_loop_iterations {
            Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "limite de iterações de loop atingido ({}) — \
                     possível loop infinito",
                    self.max_loop_iterations
                ),
            )])
        } else {
            Ok(())
        }
    }
}
```

**Decisão de design — contador global, não por loop**: `loop_iterations`
acumula ao longo de toda a execução de `eval`, não reinicia por loop.

Razão: um contador local por loop permite "loop-bombing" — um autor pode
escrever milhares de loops pequenos que individualmente respeitam qualquer
limite razoável, mas colectivamente travam o motor por minutos. O contador
global impede isso: 1.000.000 iterações no total falha em segundos
independentemente de como estão distribuídas.

O limite de 1.000.000 é suficiente para documentos legítimos (um loop que
processe 100.000 elementos usa 10% do orçamento total) e pequeno o suficiente
para que um loop infinito falhe antes de 10 segundos em hardware moderno.

---

## Tarefa 3 — Actualizar `eval.rs`

Substituir os limites inline por chamadas a `EvalContext`.

### 3a — Loop `while`

Localizar o bloco de avaliação do `while`. Deve ter algo como:

```rust
// ANTES (hardcoded)
let mut iterations = 0usize;
while eval_expr(ctx, scopes, cond)? == Value::Bool(true) {
    iterations += 1;
    if iterations > 10_000 {
        return Err(/* mensagem hardcoded */);
    }
    eval_code_block(ctx, scopes, body)?;
}

// DEPOIS (via EvalContext)
while eval_expr(ctx, scopes, cond)? == Value::Bool(true) {
    ctx.tick_loop(while_node.span())?;
    eval_code_block(ctx, scopes, body)?;
}
```

### 3b — Loop `for`

Verificar se o `for` também tem limite hardcoded. Se sim, aplicar o
mesmo padrão com `ctx.tick_loop(span)`.

### 3c — Call depth em chamadas de função/closure

Localizar onde `depth` é incrementado ao chamar uma closure. Deve ser:

```rust
// ANTES
if ctx.depth >= MAX_CALL_DEPTH {
    return Err(/* mensagem hardcoded */);
}
ctx.depth += 1;
// ... avaliar corpo da closure ...
ctx.depth -= 1;

// DEPOIS
ctx.check_call_depth(call_span)?;
ctx.depth += 1;
// ... avaliar corpo da closure ...
ctx.depth -= 1;
```

**Parar. Mostrar as secções `while`, `for`, e call depth encontradas
antes de escrever qualquer código final.**

---

## Tarefa 4 — Actualizar `DEBT.md`

Marcar DEBT-3 como parcialmente resolvido:

```markdown
### DEBT-3 — Safety rails hardcoded — PARCIALMENTE RESOLVIDO

**Resolvido no Passo 28**:
- `while` limit: 10.000 → 1.000.000, via `EvalContext::tick_loop()`
- `MAX_CALL_DEPTH`: 200 → 250, via `EvalContext::check_call_depth()`
- Limites documentados em `EvalContext` (não mais magia inline)

**Ainda pendente (Passo 29+)**:
- Detecção semântica de ciclos de importação (aguarda implementação de `import`)
- Integração com `comemo` para recursão real O(1) (aguarda TrackedWorld real)

**Ficheiros alterados**: `rules/eval_context.rs`, `rules/eval.rs`
```

---

## Tarefa 5 — Testes

Adicionar em `rules/eval.rs` dentro de `#[cfg(test)]`:

```rust
// ── while limit ──────────────────────────────────────────────────────────

#[test]
fn while_com_muitas_iteracoes_passa() {
    // 100.000 iterações — deve passar (limite é 1.000.000)
    let world = MockWorld::new(
        "#let x = 0
         #while x < 100000 { x = x + 1 }
         #let resultado = x"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("resultado"), Some(&Value::Int(100_000)));
}

#[test]
fn while_infinito_retorna_err() {
    // Usa limite reduzido para o teste terminar em milissegundos.
    // eval_for_test_with_limits é uma função #[cfg(test)]-only que aceita
    // max_loop_iterations — não existe em produção.
    let world = MockWorld::new("#let x = 0\n#while true { x = x + 1 }");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test_with_limits(&world, &src, 1_000, 250);
    assert!(result.is_err(), "while infinito deve retornar Err");
}

// ── call depth ────────────────────────────────────────────────────────────

#[test]
fn recursao_profunda_retorna_err() {
    // Recursão que ultrapassa max_call_depth
    let world = MockWorld::new(
        "#let f(x) = f(x + 1)
         #let _ = f(0)"
    );
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err(), "recursão infinita deve retornar Err");
}

#[test]
fn recursao_moderada_passa() {
    // Recursão de 10 níveis — deve passar
    let world = MockWorld::new(
        "#let countdown(n) = if n == 0 { 0 } else { countdown(n - 1) }
         #let resultado = countdown(10)"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("resultado"), Some(&Value::Int(0)));
}

#[test]
fn recursao_mutua_retorna_err() {
    // A chama B, B chama A — recursão mútua infinita
    let world = MockWorld::new(
        "#let a(x) = b(x + 1)
         #let b(x) = a(x + 1)
         #let _ = a(0)"
    );
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err(), "recursão mútua infinita deve retornar Err");
}
```

**Implementação obrigatória para teste de `while` infinito**: a Opção A
é a escolha correcta — adicionar `eval_for_test_with_limits` em
`#[cfg(test)]` que aceita `max_loop_iterations: usize`. Isso é um padrão
de teste de sistemas robusto: os limites de produção existem para documentos
reais, não para testes unitários que precisam de falhar rapidamente.

A Opção B (não testar) é inaceitável — ficaria um caminho de erro crítico
sem cobertura.

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

# Confirmar que os números hardcoded sumiram
grep -n "10_000\|10000\|200.*depth\|depth.*200" 01_core/src/rules/eval.rs
# Deve retornar vazio ou apenas comentários históricos

# Confirmar que os limites estão documentados no EvalContext
grep -n "max_loop\|max_call\|1_000_000\|250" 01_core/src/rules/eval_context.rs
```

Critérios de conclusão:
- `EvalContext` tem `max_call_depth: usize` e `max_loop_iterations: usize` com valores documentados ✓
- `check_call_depth()` e `tick_loop()` implementados em `EvalContext` ✓
- `eval.rs` usa `ctx.check_call_depth()` e `ctx.tick_loop()` — sem números inline ✓
- `recursao_profunda_retorna_err` passa ✓
- `recursao_moderada_passa` passa ✓
- `recursao_mutua_retorna_err` passa ✓
- `eval_for_test_with_limits` adicionado em `#[cfg(test)]` com `max_loop_iterations` e `max_call_depth` ✓
- `while_infinito_retorna_err` usa `eval_for_test_with_limits` com limite 1.000 ✓
- `while_com_muitas_iteracoes_passa` passa ✓
- DEBT-3 marcado como parcialmente resolvido em `DEBT.md` ✓
- Zero violations ✓
- Testes não regridem (384 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- Onde estavam os limites (constantes nomeadas ou inline)?
- `EvalContext` já tinha `depth` ou foi adicionado?
- `for` tinha limite separado ou partilhava com `while`?

**Da implementação:**
- Número final de testes e zero violations confirmado.

**Estado de DEBT-3 após este passo:**
- Resolvido: while limit, call depth
- Pendente: detecção de ciclos de importação (Passo 29+)

**Go para Passo 29 — DEBT-3 (parte 2): detecção de ciclos de importação.**
