# Passo 13 — Subset de Value e início da travessia AST em eval()

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0024-ecow-value.md` — **ler antes de qualquer código**
- `00_nucleo/adr/typst-adr-0015-ecow.md` — contexto da decisão original
- `00_nucleo/adr/typst-adr-0016-adiamento-eval-typst-library.md`
- `lab/typst-original/crates/typst-library/src/foundations/value.rs`
- `01_core/src/rules/eval.rs` — esqueleto do Passo 12

Pré-condição: `cargo test` — 216 testes (194 L1 + 22 L3), zero violations.

Duas partes com dependência sequencial:
1. **Value subset** — substituir `Value(())` por enum real com 5 variantes
2. **Travessia AST parcial** — expandir `eval()` para avaliar literais

Não inverter a ordem. A travessia AST pressupõe Value com variantes reais.

---

## Tarefa 0 — Executar ADR-0024 (ecow em l1_allowed_external)

Decisão já tomada — sem diagnóstico adicional.

### crystalline.toml

```toml
[l1_allowed_external]
rust = [
    "thiserror",
    "comemo",
    "unicode_ident",
    "unicode_math_class",
    "unicode_script",
    "unicode_segmentation",
    "rustc_hash",
    "time",
    "indexmap",
    "ecow",  # ADR-0024 — EcoString em Value::Str; clone O(1) no hot path de eval()
]
```

### Workspace Cargo.toml

```toml
[workspace.dependencies]
# verificar versão usada pelo original:
# grep "^ecow" lab/typst-original/Cargo.toml
ecow = "0.2"  # ajustar para a versão do original
```

### 01_core/Cargo.toml

```toml
[dependencies]
ecow = { workspace = true }
```

### Verificação intermédia

```bash
cargo build
crystalline-lint .
# ✓ ecow visível em L1, zero violations
```

---

## Tarefa 1 — Diagnóstico de value.rs

**Parar aqui. Reportar output antes de continuar.**

```bash
# Variantes completas do enum Value
grep -A 80 "^pub enum Value" \
  lab/typst-original/crates/typst-library/src/foundations/value.rs \
  | head -85

# Confirmar que Value::Str usa EcoString
grep -n "Str\|EcoString" \
  lab/typst-original/crates/typst-library/src/foundations/value.rs \
  | head -10

# Dependências externas de value.rs
grep "^use\|^extern" \
  lab/typst-original/crates/typst-library/src/foundations/value.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Como literais são avaliados no original
grep -rn "Value::Int\|Value::Float\|Value::Str\|Value::Bool\|Value::None" \
  lab/typst-original/crates/typst-eval/src/ | head -20

# API de ast para literais — confirmar nomes reais dos nós
grep -n "Int\|Float\|Str\|Bool\|None" \
  01_core/src/entities/ast/expr.rs | head -30

# Como LetBinding expõe o nome e o init
grep -n "pub fn\|kind\|name\|init\|pattern" \
  01_core/src/entities/ast/code.rs | head -30
```

O diagnóstico responde a duas questões práticas:
1. Confirmar que `Value::Str` usa `EcoString` no original (esperado)
2. Confirmar os nomes reais das variantes de `ast::Expr` e da API de
   `ast::LetBinding` — necessário para a Tarefa 3 compilar sem ajustes

---

## Tarefa 2 — Value subset em L1

### Princípio: enum com fronteira explícita

O enum começa com 5 variantes reais. As restantes (~35) são listadas
como comentários — isto não é código incompleto, é uma fronteira
deliberada. Os comentários impedem que o agente tente preencher
variantes que dependem de `Content`, `Func`, `Args`, `Symbol`, etc.

**Ficheiro**: `01_core/src/entities/value.rs`

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/value.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-27

use ecow::EcoString;  // ADR-0024 — clone O(1) no hot path de eval()

/// Valor em tempo de avaliação do Typst.
///
/// Subset inicial: 5 variantes de literais primitivos.
/// As restantes (~35) são adicionadas quando os tipos dependentes
/// migrarem para L1. Não adicionar variantes sem ADR e tipo migrado.
/// Ver ADR-0016.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // ── Subset inicial (Passo 13) ────────────────────────────────────────
    /// O valor `none` do Typst.
    None,
    /// Valor booleano (`true` / `false`).
    Bool(bool),
    /// Inteiro de 64 bits com semântica de número inteiro Typst.
    Int(i64),
    /// Número de vírgula flutuante IEEE 754.
    Float(f64),
    /// String de texto. EcoString — clone O(1) (ADR-0024).
    Str(EcoString),

    // ── Variantes futuras — NÃO implementar sem ADR e tipo migrado ───────
    // Auto(AutoValue),          // o valor `auto`
    // Length(Length),           // comprimento tipográfico (pt, em, etc.)
    // Angle(Angle),             // ângulo (deg, rad)
    // Ratio(Ratio),             // rácio (percentagem)
    // Relative(Relative),       // comprimento relativo
    // Fraction(Fraction),       // fracção de espaço disponível
    // Color(Color),             // cor (rgb, cmyk, etc.)
    // Gradient(Gradient),       // gradiente
    // Tiling(Tiling),           // padrão de azulejos
    // Symbol(Symbol),           // símbolo Unicode
    // Version(PackageVersion),  // versão semântica
    // Bytes(Bytes),             // bytes binários
    // Datetime(Datetime),       // data/hora — já em L1 como tipo separado
    // Duration(Duration),       // duração
    // Content(Content),         // conteúdo tipográfico — bloqueia layout()
    // Styles(Styles),           // estilos encadeados — bloqueia show/set
    // Array(Array),             // lista de Values
    // Dict(Dict),               // mapa string → Value
    // Func(Func),               // função Typst — bloqueia chamadas
    // Args(Args),               // argumentos de função
    // Type(Type),               // tipo como valor (int, str, etc.)
    // Module(Module),           // módulo importado — já em L1
    // Plugin(Plugin),           // plugin WASM
    // Dyn(Dynamic),             // valor dinâmico opaco
}

impl Value {
    /// Retorna o nome do tipo Typst deste valor.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::None    => "none",
            Self::Bool(_) => "bool",
            Self::Int(_)  => "int",
            Self::Float(_)=> "float",
            Self::Str(_)  => "str",
        }
    }

    /// Retorna true se o valor é `none`.
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Converte para bool, se for Bool.
    pub fn cast_bool(&self) -> Option<bool> {
        match self { Self::Bool(b) => Some(*b), _ => None }
    }

    /// Converte para i64, se for Int.
    pub fn cast_int(&self) -> Option<i64> {
        match self { Self::Int(i) => Some(*i), _ => None }
    }

    /// Converte para f64 (aceita Int e Float — coerção implícita do Typst).
    pub fn cast_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            Self::Int(i)   => Some(*i as f64),
            _ => None,
        }
    }

    /// Converte para &str, se for Str.
    pub fn cast_str(&self) -> Option<&str> {
        match self { Self::Str(s) => Some(s.as_str()), _ => None }
    }
}

// Conversões From para ergonomia em eval() e testes
impl From<bool>      for Value { fn from(v: bool)      -> Self { Self::Bool(v) } }
impl From<i64>       for Value { fn from(v: i64)       -> Self { Self::Int(v) } }
impl From<i32>       for Value { fn from(v: i32)       -> Self { Self::Int(v as i64) } }
impl From<f64>       for Value { fn from(v: f64)       -> Self { Self::Float(v) } }
impl From<EcoString> for Value { fn from(v: EcoString) -> Self { Self::Str(v) } }
impl From<&str>      for Value { fn from(v: &str)      -> Self { Self::Str(v.into()) } }
impl From<String>    for Value { fn from(v: String)    -> Self { Self::Str(v.into()) } }
```

### Actualizar Binding em scope.rs

`Binding::new(value: Value)` aceita o enum real sem alteração de
interface. Verificar que `cargo build` compila sem erros após a
mudança de `Value(())` para o enum.

Se `Scope::define` usa `impl Into<Value>` — funciona automaticamente
com as conversões `From` acima.

### Prompt L0

**Actualizar**: `00_nucleo/prompts/entities/value.md`

Substituir stub doc por documentação do subset real:
- As 5 variantes e razão de cada
- Variantes futuras listadas como comentários — razão da fronteira
- Decisão sobre EcoString (ADR-0024) e contraste com ADR-0015
- Critérios de verificação

### Testes de Value

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ecow::EcoString;

    #[test]
    fn type_names() {
        assert_eq!(Value::None.type_name(), "none");
        assert_eq!(Value::Bool(true).type_name(), "bool");
        assert_eq!(Value::Int(42).type_name(), "int");
        assert_eq!(Value::Float(3.14).type_name(), "float");
        assert_eq!(Value::Str(EcoString::from("hi")).type_name(), "str");
    }

    #[test]
    fn cast_float_aceita_int() {
        assert_eq!(Value::Int(3).cast_float(), Some(3.0));
        assert_eq!(Value::Float(3.14).cast_float(), Some(3.14));
        assert_eq!(Value::Bool(true).cast_float(), None);
    }

    #[test]
    fn from_primitivos() {
        assert_eq!(Value::from(true), Value::Bool(true));
        assert_eq!(Value::from(42i64), Value::Int(42));
        assert_eq!(Value::from(3.14f64), Value::Float(3.14));
        assert_eq!(Value::from("hello"), Value::Str("hello".into()));
    }

    #[test]
    fn ecostring_clone_e_eq() {
        // Verificar que EcoString funciona correctamente em Value
        let v1 = Value::Str(EcoString::from("test"));
        let v2 = v1.clone();  // clone O(1)
        assert_eq!(v1, v2);
        assert_ne!(Value::Str("a".into()), Value::Str("b".into()));
    }

    #[test]
    fn scope_com_value_real() {
        use crate::entities::scope::Scope;
        let mut scope = Scope::new();
        scope.define("x", Value::Int(42));
        scope.define("s", Value::Str("hello".into()));
        assert_eq!(scope.get("x"), Some(&Value::Int(42)));
        assert_eq!(scope.get("s"), Some(&Value::Str("hello".into())));
    }

    #[test]
    fn value_none_is_none() {
        assert!(Value::None.is_none());
        assert!(!Value::Int(0).is_none());
    }
}
```

### Verificação intermédia

```bash
cargo test -p typst-core -- entities::value
cargo build
crystalline-lint .
# ✓ Value com 5 variantes, zero violations
```

---

## Tarefa 3 — Travessia AST parcial em eval()

**Pré-condição**: Value subset compila com testes a passar.

### Fronteira deliberada — `_ => Ok(Value::None)`

O padrão `_ => Ok(Value::None)` no match de `eval_expr` não é código
incompleto. É a fronteira que separa o que este passo implementa do
que requer tipos não migrados. Nós que requerem `Content`, `Func`,
ou `Styles` retornam `None` sem erro — a travessia continua, os testes
não falham com "unimplemented!".

Ignorar trivia em markup permite encontrar `#let x = 1` dentro de
texto sem tentar avaliar o texto à volta. Isto torna os testes de
integração mais próximos de documentos reais sem requerer `Content`.

### Expandir eval.rs

**Antes de escrever código**, confirmar com o diagnóstico:
- O nome exacto das variantes de `Expr` para literais (ex: `Expr::Int`,
  `Expr::Lit`, etc. — depende da AST migrada no Passo 5)
- A API de `ast::LetBinding` — métodos `kind()`, `name()`, `init()`
- Como `Ident` expõe o nome (`as_str()`, `get()`, ou outro)

Se a API diferir do código abaixo, ajustar antes de compilar.

```rust
// 01_core/src/rules/eval.rs — substituir o esqueleto

use comemo::{Tracked, TrackedMut};
use ecow::EcoString;
use crate::contracts::world::TrackedWorld;
use crate::entities::{
    ast::{self, AstNode, Expr},
    module::Module,
    scope::Scope,
    source::Source,
    source_result::{SourceDiagnostic, SourceResult},
    value::Value,
    world_types::{Engine, Route, Routines, Sink, Traced},
};
use crate::rules::scopes::Scopes;

pub fn eval(
    _routines: &Routines,
    world: Tracked<dyn TrackedWorld + '_>,
    _traced: Tracked<Traced>,
    _sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module> {
    let root = source.root();
    let global = Scope::new();  // stdlib vazia — Library(()) stub
    let mut scopes = Scopes::new(&global);
    scopes.push();

    eval_markup(root, &mut scopes, world)?;

    let top_scope = scopes.pop().unwrap_or_default();
    Ok(Module::new(
        source.id().into_raw().get().to_string(),
        top_scope,
    ))
}

fn eval_markup(
    node: &crate::entities::syntax_node::SyntaxNode,
    scopes: &mut Scopes<'_>,
    world: Tracked<dyn TrackedWorld + '_>,
) -> SourceResult<Value> {
    for child in node.children() {
        if child.kind().is_trivia() { continue; }
        // Tentar como expressão; se não for, ignorar (texto puro, etc.)
        if let Some(expr) = Expr::from_untyped(child) {
            eval_expr(expr, scopes, world)?;
        }
    }
    Ok(Value::None)
}

fn eval_expr(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    world: Tracked<dyn TrackedWorld + '_>,
) -> SourceResult<Value> {
    match expr {
        Expr::Int(node)   => Ok(Value::Int(node.get())),
        Expr::Float(node) => Ok(Value::Float(node.get())),
        Expr::Str(node)   => Ok(Value::Str(EcoString::from(node.get()))),
        Expr::Bool(node)  => Ok(Value::Bool(node.get())),
        Expr::None(_)     => Ok(Value::None),

        Expr::Ident(ident) => {
            let name = ident.as_str();
            scopes.get(name)
                .cloned()
                .ok_or_else(|| vec![SourceDiagnostic::error(
                    ident.span(),
                    format!("unknown variable: {name}"),
                )])
        }

        Expr::Let(binding) => eval_let(binding, scopes, world),

        Expr::Code(code) => {
            // Bloco de código — avaliar filhos sequencialmente
            let mut last = Value::None;
            for child in code.body().children() {
                if child.kind().is_trivia() { continue; }
                if let Some(expr) = Expr::from_untyped(child) {
                    last = eval_expr(expr, scopes, world)?;
                }
            }
            Ok(last)
        }

        // Fronteira deliberada — requer tipos não migrados
        // Content, FuncCall, Set, Show, Import, etc. → Value::None
        _ => Ok(Value::None),
    }
}

fn eval_let(
    binding: ast::LetBinding<'_>,
    scopes: &mut Scopes<'_>,
    world: Tracked<dyn TrackedWorld + '_>,
) -> SourceResult<Value> {
    // Confirmar API de LetBinding com o diagnóstico antes de compilar
    // Nomes tentativas baseados na AST migrada — ajustar se necessário
    let value = match binding.init() {
        Some(init) => eval_expr(init, scopes, world)?,
        None => Value::None,
    };

    // binding.kind() retorna LetBindingKind — confirmar variante para nome simples
    use ast::LetBindingKind;
    if let LetBindingKind::Normal(pattern) = binding.kind() {
        // pattern é um Pattern — confirmar se tem .name() para Ident simples
        if let Some(ident) = pattern.ident() {
            scopes.define(ident.as_str(), value);
        }
    }

    Ok(Value::None)
}
```

### Aviso sobre comemo nos testes de eval()

A integração de `comemo::Tracked<>` com MockWorld nos testes pode
revelar comportamentos inesperados — a proc-macro de comemo gera código
que interactua com o sistema de tipos de formas não óbvias.

**Estratégia de teste**: se `comemo::track(&world)` não compilar
directamente nos testes unitários, usar uma abordagem alternativa:

```rust
// Opção A — se comemo expõe track() como método de trait
use comemo::Track;
let tracked = world.track();

// Opção B — se comemo usa função livre
let tracked = comemo::track(&world);

// Opção C — se nenhuma das anteriores compila em contexto de teste,
// testar eval_expr directamente sem passar pelo eval() público:
// criar Scopes<'_> directamente, chamar eval_expr com um nó AST
// construído via Source::detached()
```

Se a integração com comemo bloquear completamente os testes de eval():
- Os testes de Value (Tarefa 2) e de Scope (Passo 11) cobrem a mecânica
- Registar o bloqueio como item de dívida e avançar para o Passo 14
- Não gastar mais de 30 minutos a tentar resolver a API de comemo
  nos testes — é um problema de infraestrutura de teste, não de lógica

### Prompt L0

**Actualizar**: `00_nucleo/prompts/rules/eval.md`

Documentar:
- Variantes de Expr suportadas: Int, Float, Str, Bool, None, Ident, Let, Code
- Fronteira: `_ => Ok(Value::None)` — lista dos nós não implementados
- Aviso sobre comemo nos testes
- Plano de expansão incremental

### Testes de eval()

```rust
// Testes que NÃO dependem de comemo — usar Source + eval_expr directamente
// se a integração comemo bloquear

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{scope::Scope, source::Source};

    /// Helper: avaliar expressão num âmbito vazio via eval_expr.
    /// Não passa por eval() público — evita dependência de comemo nos testes.
    fn eval_str(text: &str) -> SourceResult<Value> {
        let source = Source::detached(text);
        let root = source.root();
        let global = Scope::new();
        let mut scopes = Scopes::new(&global);
        scopes.push();

        // Encontrar a primeira expressão no root
        for child in root.children() {
            if child.kind().is_trivia() { continue; }
            if let Some(expr) = crate::entities::ast::Expr::from_untyped(child) {
                // Nota: eval_expr requer world — usar um mock ou
                // reorganizar para que literais não precisem de world
                // Se não compilar, testar via Scope directamente
                let _ = expr;
            }
        }
        Ok(Value::None)
    }

    // Testes que testam a mecânica via Scope — não dependem de comemo
    #[test]
    fn scope_define_via_value_real() {
        let mut scope = Scope::new();
        scope.define("x", Value::Int(42));
        scope.define("s", Value::Str("hello".into()));
        assert_eq!(scope.get("x"), Some(&Value::Int(42)));
    }

    #[test]
    fn scopes_lookup_em_pilha() {
        let global = Scope::new();
        let mut scopes = Scopes::new(&global);
        scopes.push();
        scopes.define("x", Value::Int(1));
        scopes.push();
        scopes.define("y", Value::Int(2));
        assert_eq!(scopes.get("x"), Some(&Value::Int(1)));
        assert_eq!(scopes.get("y"), Some(&Value::Int(2)));
        assert_eq!(scopes.get("z"), None);
    }

    // Testes de integração com eval() público — tentar; se comemo bloquear,
    // mover para ficheiro separado e marcar com #[ignore]
    #[test]
    #[ignore = "requer integração com comemo — desbloquear no Passo 14 se necessário"]
    fn eval_let_int_via_world() {
        // Implementar quando a API de comemo estiver confirmada
    }
}
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critérios de conclusão obrigatórios:
- `Value::Int(42)`, `Value::Str(...)`, `Value::Bool(true)` são variantes reais
- `ecow` em `[l1_allowed_external]`, crystalline-lint confirma sem V14
- `Scope::get("x")` retorna `Some(&Value::Int(42))` após `define("x", 42.into())`
- `eval.rs` compila com as variantes de Expr suportadas
- Zero violations
- Testes não regridem (216 base + novos)

Critérios desejáveis (não bloqueantes):
- Testes de integração de eval() com comemo passam
- `#[ignore]` nos testes de comemo justificado com issue/DEBT.md

---

## Ao terminar, reportar

**Do diagnóstico:**
- Variantes reais de `Value` no original — lista completa
- Confirmação de que `Value::Str` usa `EcoString` no original
- Nomes reais das variantes de `Expr` para literais (Int, Float, etc.)
- API real de `ast::LetBinding` — `kind()`, `init()`, `pattern vs name`

**Da implementação:**
- Se `eval_let` compilou com a API real de LetBinding ou precisou de ajustes
- Estado dos testes de comemo — passam, bloqueados, ou marcados com `#[ignore]`
- Se foi necessário reorganizar `eval_expr` para que literais não dependam de world
- Número total de testes
- Zero violations confirmado

**Go/No-Go para o Passo 14:**
- **GO — operações**: Value subset funciona com comemo integrado;
  Passo 14 implementa operações aritméticas (Int+Int, Str+Str, etc.)
- **GO — comemo adiado**: eval() compila mas testes de integração
  marcados com `#[ignore]`; Passo 14 resolve a integração de comemo
  como primeira tarefa antes de operações
- **NO-GO — API AST**: variantes de Expr ou API de LetBinding
  diferem significativamente do esperado; Passo 14 começa com
  diagnóstico AST mais profundo
