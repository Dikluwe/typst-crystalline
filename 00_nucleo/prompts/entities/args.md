# Prompt L0 — `entities/args` — Gestão de Argumentos de Função
Hash do Código: 58f41a7a

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/args.rs`
**Passo de origem**: Passo 17 (stdlib e chamadas de função)
**ADRs relevantes**: ADR-0016 (spread adiado — `..args` não implementado ainda)

---

## Contexto e Objetivo

Quando uma `Value::Func` é chamada durante a avaliação (`eval.rs`), os
argumentos passados pelo utilizador precisam de ser capturados, validados e
entregues à implementação nativa da função de forma segura e tipada.

A `struct Args` é o contentor de argumentos de uma chamada de função — separa
**argumentos posicionais** (por ordem) de **argumentos nomeados** (por chave).

**Separação de `func.md`**: `func.md` documenta a entidade `Func` (a função
em si — a sua assinatura e tipo). `args.md` (este) documenta `Args` — os
valores concretos passados numa chamada específica.

---

## Struct `Args`

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    /// Argumentos posicionais, em ordem de chamada.
    pub items: Vec<Value>,
    /// Argumentos nomeados (named args), preservando ordem de inserção.
    /// Usa IndexMap com FxBuildHasher para performance.
    pub named: IndexMap<EcoString, Value, FxBuildHasher>,
}
```

---

## Interface Pública

```rust
impl Args {
    /// Cria Args apenas com posicionais (named vazio).
    pub fn positional(items: Vec<Value>) -> Self

    /// Número de argumentos posicionais.
    pub fn len(&self) -> usize

    /// True se não há posicionais NEM nomeados.
    pub fn is_empty(&self) -> bool
}
```

### Acesso a Named Args

Named args são acedidos directamente via `args.named.get("key")`.
A `IndexMap` preserva a ordem de inserção — importante para mensagens de erro.

---

## Dívidas Técnicas (ADR-0016)

O método `take()` (consumo mútuo de posicionais) e `finish()` (validação de
argumentos não consumidos) foram **adiados** para Passo 17+. O estado actual
suporta apenas:
1. Criação via `positional(items)`
2. Injecção de named args via `args.named.insert(key, value)`
3. Leitura via `args.items[i]` e `args.named.get(key)`

**Passo futuro**: implementar `take<T>() -> Option<T>` (desserialização tipada
de posicionais) e `finish() -> SourceResult<()>` (erro se restam args).

---

## Integração com Stdlib

As funções nativas em `stdlib.rs` consomem `&[Value]` directamente —
**não recebem `Args`**. Esta é uma simplificação intencional (Passo 17):
as funções nativas são chamadas via `Func::native(name, fn_ptr)` e o
despachante converte `args.items` para `&[Value]` antes de chamar.

```rust
// Chamada de função nativa (eval.rs, passo 17)
Value::Func(f) => f.call(&args.items)  // passa &[Value], não Args

// Acesso a named args (ainda sem suporte completo no eval)
args.named.get("color")
```

---

## Invariantes

| Invariante | Detalhe |
|-----------|---------|
| Posicionais em ordem | `items[0]` é o primeiro argumento da chamada |
| Named preservam ordem | `IndexMap` garante ordem de inserção |
| `is_empty()` verifica ambos | Vazio só se `items` E `named` estão vazios |
| Nenhuma validação de tipo | `Args` é agnóstica de tipo — a função decide |

---

## Critérios de Verificação

```
// Construção
Args::positional([]).is_empty()   = true
Args::positional([]).len()        = 0

Args::positional([Int(1), Bool(true)]).len()       = 2
Args::positional([Int(1), Bool(true)]).is_empty()  = false
Args::positional([Int(1), Bool(true)]).items[0]    = Int(1)
Args::positional([Int(1), Bool(true)]).items[1]    = Bool(true)

// Named
a = Args::positional([])
a.named.insert("x", Int(1))
a.is_empty()             = false  // named não está vazio
a.len()                  = 0      // len() conta apenas posicionais
a.named.get("x")         = Some(&Int(1))
a.named.get("y")         = None

// Clone e PartialEq
a1 = Args::positional([Int(42)])
a2 = a1.clone()
a1 == a2                 = true

// Named preservam ordem
a.named.insert("b", Int(2))
a.named.insert("a", Int(1))
a.named.keys().collect::<Vec>() = ["b", "a"]  // ordem de inserção
```
