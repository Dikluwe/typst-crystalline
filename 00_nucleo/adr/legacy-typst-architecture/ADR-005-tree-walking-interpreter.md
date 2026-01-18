# ADR-005: Tree-Walking Interpreter

## Status
**Accepted**

## Context

Typst needs to execute user-defined code (functions, loops, conditionals). There are two main approaches:

1. **Bytecode Interpreter**: Compile AST to bytecode, then execute
2. **Tree-Walking Interpreter**: Traverse AST directly during execution

A choice was needed that balanced implementation simplicity, development speed, and performance.

## Decision

Implement a **tree-walking interpreter** that recursively traverses the AST during execution.

### Mechanism

```rust
trait Eval {
    fn eval(&self, vm: &mut Vm) -> Result<Value>;
}

impl Eval for Binary {
    fn eval(&self, vm: &mut Vm) -> Result<Value> {
        let lhs = self.lhs().eval(vm)?;
        let rhs = self.rhs().eval(vm)?;
        ops::binary(self.op(), lhs, rhs)
    }
}
```

### Closure Handling

Closures capture lexical scope variables **by value** (not by reference):

1. When encountering a closure definition, the interpreter identifies free variables
2. Values are cloned and stored alongside the definition
3. On call, a new `Vm` is created with captured variables

## Consequences

### Positive
- **Simplicity**: Straightforward and easy-to-understand implementation
- **Flexibility**: Easy to add new language constructs
- **Debugging**: Clearer stack traces
- **Incrementality**: Works well with closure memoization

### Negative
- **Performance**: Slower than bytecode for frequently executed code
- **Re-traversal**: Each function call re-traverses the AST
- **Overhead**: AST structure loaded in memory

### Mitigation

Memoization via comemo compensates for re-traversal penalty for identical closures.

## Alternatives Considered

**Bytecode Interpreter**: Rejected for adding complexity without clear benefit for the use case (documents, not long-running programs).

## References
- [typst-eval/](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/01_core/typst-eval/)
