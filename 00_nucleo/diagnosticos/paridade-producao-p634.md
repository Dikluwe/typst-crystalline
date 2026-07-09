# Relatório de Paridade — P634

**Data:** 2026-07-09  
**Passo:** 634  
**Foco:** Catch-all de `eval_expr` faz `#break`/`#continue`/`#return` desaparecer sem erro.  
**Hash do commit com as alterações:** `a4e0ddfae`  

## Sumário

O catch-all `_ => Ok(Value::None)` de `eval_expr` (`01_core/src/rules/eval/mod.rs:819`) foi removido. O `match` passou a ser exaustivo: `#break`, `#continue` e `#return` usados fora do contexto respectivo agora produzem erros claros, byte-idênticos ao vanilla. Testes de P633 que confirmavam a falha silenciosa foram invertidos para confirmar o erro. Não houve regressão na suite.

## Sonda

### Variantes que caíam no catch-all

Medição em `01_core/src/rules/eval/mod.rs:585-820` (`eval_expr`) comparada com a definição de `Expr` em `01_core/src/entities/ast/expr.rs:35-95`:

- Tratadas explicitamente antes da alteração: `Int`, `Float`, `Str`, `Bool`, `None`, `Auto`, `Ident`, `LetBinding`, `CodeBlock`, `Binary`, `Unary`, `Conditional`, `WhileLoop`, `ForLoop`, `Closure`, `FuncCall`, `Strong`, `Emph`, `Heading`, `Raw`, `Link`, `ListItem`, `EnumItem`, `FieldAccess`, `SetRule`, `ContentBlock`, `Equation`, `Math`, `ModuleImport`, `ModuleInclude`, `Ref`, `Label`, `ShowRule`, `Array`, `Dict`, `Parenthesized`, `Numeric`, `Contextual`, `Escape`, `Shorthand`, `Linebreak`.
- No catch-all (não migradas ou estruturais): `Text`, `Space`, `Parbreak`, `SmartQuote`, `TermItem`, `MathText`, `MathIdent`, `MathShorthand`, `MathAlignPoint`, `MathDelimited`, `MathAttach`, `MathPrimes`, `MathFrac`, `MathRoot`, `DestructAssignment`, `LoopBreak`, `LoopContinue`, `FuncReturn`.

### Comportamento do vanilla

`lab/typst-original/crates/typst-eval/src/flow.rs:28-36`:

```rust
Self::Break(span) => error!(span, "cannot break outside of loop")
Self::Continue(span) => error!(span, "cannot continue outside of loop")
Self::Return(span, _, _) => error!(span, "cannot return outside of function")
```

O vanilla usa um mecanismo `FlowEvent` propagado na VM. O cristalino ainda não o implementa; nesta fase, qualquer ocorrência destas variantes no dispatcher topo produz o erro acima.

### `#let x = 0xZZ`

Investigação mostrou que `0xZZ` não chega ao catch-all de `eval_expr`. O lexer cria um nó de erro (`SyntaxKind::Error`), o parser insere-o na AST, e o eval (via `LetBinding::init()`/`Expr::from_untyped`) acaba por tratar a ausência de expressão como `Value::None`. Uma tentativa de propagar todos os erros de parser no entrypoint (`eval_with_full_error`) expôs 10 regressões em testes existentes (smart quotes, `#set` dentro de blocos, escapes unicode inválidos em markup), indicando que a correção deste caso específico pertence a um passo dedicado ao parser/lexer, não ao catch-all de `eval_expr`. Mantido como débito documentado.

## Implementação

### Prompt L0

`00_nucleo/prompts/rules/eval.md` atualizado para refletir:

- Ficheiro alvo: `01_core/src/rules/eval/mod.rs`.
- Nova secção §P634 com as mensagens de erro e a justificação da ausência de `FlowEvent`.
- Fronteira deliberada actualizada: `match` exaustivo, sem catch-all silencioso; variantes estritamente estruturais (`Text`, `Space`, `Math*`, etc.) continuam a devolver `Value::None`; `DestructAssignment` produz erro.

### Código

`01_core/src/rules/eval/mod.rs`:

- Adicionados braços para `Expr::LoopBreak`, `Expr::LoopContinue` e `Expr::FuncReturn` com as mensagens do vanilla.
- Adicionado braço para `Expr::DestructAssignment` com erro de "não implementado".
- Agrupadas variantes estruturais de markup/math num braço que devolve `Ok(Value::None)`.
- Removido o catch-all `_ => Ok(Value::None)`.

`01_core/src/rules/eval/tests.rs`:

- `p633_break_top_level_silently_none` → `p634_break_top_level_errors`.
- `p633_continue_top_level_silently_none` → `p634_continue_top_level_errors`.
- `p633_return_top_level_silently_none` → `p634_return_top_level_errors`.
- `p633_parse_error_expr_silent_none` mantido com comentário que identifica a causa real no parser.

## Validação

```bash
cargo test --workspace
```

Resultado: `605 passed; 0 failed; 5 ignored` (typst-core: `3604 passed; 0 failed`).

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### Testes específicos de P634

```bash
cargo test -p typst-core p634_ -- --nocapture
```

- `p634_break_top_level_errors` — ok
- `p634_continue_top_level_errors` — ok
- `p634_return_top_level_errors` — ok

## Uso legítimo dentro de ciclo/função

O cristalino ainda não implementa o mecanismo `FlowEvent`, pelo que `#break`/`#continue` dentro de ciclos e `#return` dentro de funções também caíam no antigo catch-all e devolviam `Value::None`. A alteração não regrediu comportamento funcional — apenas tornou o caso de uso inválido no topo do documento explicitamente erro, conforme o vanilla.

## Débito identificado

- **Parser/lexer — erros sintáticos não propagados:** `#let x = 0xZZ` e escapes unicode inválidos em markup produzem nós `SyntaxKind::Error` que o eval ignora. Corrigir isto exige um passo dedicado ao pipeline de parsing, fora do scope de P634.
- **FlowEvent:** uso semântico de `break`/`continue`/`return` dentro dos seus contextos ainda não funciona; é trabalho futuro em `control_flow.rs`/`closures.rs`.

## Próximos passos

1. Regras `#set` que ignoram tipo inválido (itens 8–16 de P633).
2. `counter.display` com argumentos inválidos (itens 18–23 de P633).
3. Bibliografia e estado (itens 4–6 de P633).
4. Parser/lexer — propagar erros sintáticos de forma selectiva sem regressões.
