# Passo 11 — indexmap, Scope e Module real

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0023-indexmap.md`
- `00_nucleo/adr/typst-adr-0016-adiamento-eval-typst-library.md`
- `lab/typst-original/crates/typst-library/src/foundations/scope.rs`
- `lab/typst-original/crates/typst-library/src/foundations/module.rs`

Pré-condição: `cargo test` — 195 testes (173 L1 + 22 L3), zero violations.

Três tarefas sequenciais com dependência estrita:
Scope depende de indexmap; Module real depende de Scope.
Não avançar para a tarefa seguinte sem a anterior compilar e passar testes.

---

## Tarefa 1 — Diagnósticos (Go/No-Go para o Passo 12)

**Parar aqui. Reportar output completo antes de qualquer código.**

O resultado desta tarefa determina o âmbito do Passo 12:
- Se `Scope` e `Module` compilam com `Value(())` stub → Passo 12 pode atacar `eval()` directamente
- Se exigem `Content` real para compilar → Passo 12 é "Limpeza de Foundations" antes de `eval()`

```bash
# scope.rs — estrutura completa
grep -n "^pub struct\|^pub enum\|^pub fn\|^impl" \
  lab/typst-original/crates/typst-library/src/foundations/scope.rs \
  | head -50

# Dependências externas de scope.rs
grep "^use\|^extern" \
  lab/typst-original/crates/typst-library/src/foundations/scope.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# O que Binding contém — este é o diagnóstico mais crítico
grep -A 20 "pub struct Binding\|pub enum Binding" \
  lab/typst-original/crates/typst-library/src/foundations/scope.rs

# Binding usa Content, Func, ou outros tipos não migrados?
grep -n "Content\|Func\|Element\|NativeFunc" \
  lab/typst-original/crates/typst-library/src/foundations/scope.rs \
  | head -20

# module.rs — estrutura completa
grep -n "^pub struct\|^pub enum\|^pub fn\|^impl" \
  lab/typst-original/crates/typst-library/src/foundations/module.rs \
  | head -30

# Dependências externas de module.rs
grep "^use\|^extern" \
  lab/typst-original/crates/typst-library/src/foundations/module.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Module usa Arc internamente?
grep -n "Arc\|Rc\|clone\|EcoString" \
  lab/typst-original/crates/typst-library/src/foundations/module.rs \
  | head -15

# Module tem campo content: Content?
grep -n "content\|Content" \
  lab/typst-original/crates/typst-library/src/foundations/module.rs \
  | head -10

# Versão de indexmap no original
grep "indexmap" lab/typst-original/Cargo.toml
grep "indexmap" \
  lab/typst-original/crates/typst-library/Cargo.toml 2>/dev/null | head -5

# Confirmar hasher usado no IndexMap de Scope
grep -n "IndexMap\|FxBuildHasher\|BuildHasher\|FxHash" \
  lab/typst-original/crates/typst-library/src/foundations/scope.rs \
  | head -10
```

### Questões críticas a responder

1. **`Binding`** — wraps apenas `Value`, ou tem campos que requerem
   `Content`/`Func`/`Element`? Se sim, quais campos e podem ser omitidos
   neste passo sem quebrar a interface de `Scope`?

2. **`Module`** — usa `Arc<ModuleInner>` ou valor directo? Tem campo
   `content: Content`? Se sim, pode ser omitido ou precisa de stub?

3. **Externals surpresa** — aparece alguma crate além de `indexmap`
   que não está em `[l1_allowed_external]`? Se sim, criar ADR (0024+)
   antes de continuar.

---

## Tarefa 2 — Executar ADR-0023 (indexmap)

### 2a — crystalline.toml

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
    "indexmap",  # ADR-0023 — ordem de inserção em Scope; sem substituto no std
]
```

### 2b — Workspace Cargo.toml

```toml
[workspace.dependencies]
# versão confirmada no diagnóstico:
indexmap = { version = "2", features = [] }
```

### 2c — 01_core/Cargo.toml

```toml
[dependencies]
indexmap = { workspace = true }
```

### 2d — Verificação intermédia

```bash
cargo build
crystalline-lint .
# ✓ indexmap visível em L1, zero violations
```

---

## Tarefa 3 — Scope em L1

### Princípio de implementação: infraestrutura de escopo, não semântica de valores

O foco deste passo é o **mecanismo de armazenamento e recuperação** —
`Scope` como container com ordem de inserção. A semântica completa de
`Value` (os 40+ tipos do enum) não é migrada agora. `Binding` mantém-se
acoplado ao stub `Value(())`.

Esta separação é intencional: forçar a migração de `Value` real para
fazer `Scope` compilar seria inverter a ordem de dependências correcta.
`Scope` é infraestrutura de nomes; `Value` é semântica de valores.
São decisões independentes.

### Binding — acoplado ao stub Value(())

```rust
/// Valor ligado a um nome num Scope.
///
/// Mantido acoplado ao stub Value(()) neste passo.
/// Campos adicionais do original (span, kind, visibilidade) são
/// adicionados quando Value real migrar — não antecipar.
pub struct Binding {
    value: Value,  // Value(()) — stub intencional, ver ADR-0016
}

impl Binding {
    pub fn new(value: Value) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn into_value(self) -> Value {
        self.value
    }
}
```

Se o diagnóstico revelar campos adicionais no `Binding` original
(ex: `span: Span`, `kind: BindingKind`) que são domínio puro (sem
dependências externas) → incluir. Campos que dependem de tipos não
migrados → omitir com comentário `// ADR-0016: adiado`.

### Scope

```rust
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

/// Âmbito de nomes do compilador Typst.
///
/// Usa IndexMap para preservar ordem de inserção — a ordem de declaração
/// de bindings em Typst é semanticamente significativa (ADR-0023).
/// Hasher: FxBuildHasher de rustc_hash (ADR-0018) — rápido para
/// identificadores curtos.
pub struct Scope {
    map: IndexMap<String, Binding, FxBuildHasher>,
    // parent: confirmar no diagnóstico se Scope tem parent inline
    // ou se há um tipo Scopes separado no original
}

impl Scope {
    pub fn new() -> Self {
        Self {
            map: IndexMap::with_hasher(FxBuildHasher::default()),
        }
    }

    /// Define um binding. Se o nome já existe, substitui no lugar
    /// (mantendo a posição na ordem de inserção).
    pub fn define(&mut self, name: impl Into<String>, value: Value) {
        self.map.insert(name.into(), Binding::new(value));
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.map.get(name).map(|b| b.value())
    }

    pub fn get_binding(&self, name: &str) -> Option<&Binding> {
        self.map.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Binding)> {
        self.map.iter().map(|(k, v)| (k.as_str(), v))
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
```

Se o original tem `Scopes` (pilha de `Scope`) como tipo separado →
avaliar se pertence a L1 ou se é detalhe de implementação de `eval()`.
Incluir apenas se for necessário para `Module`.

### Prompt L0

**Criar**: `00_nucleo/prompts/entities/scope.md`

Documentar:
- `Scope` como container de nomes com ordem de inserção preservada
- `Binding` acoplado a `Value(())` neste passo — justificação explícita
- Por que `IndexMap` e não `HashMap` (ADR-0023)
- Por que `FxBuildHasher` (ADR-0018)
- Campos omitidos de `Binding` e razão
- Critérios de verificação

### Ficheiro

**Criar**: `01_core/src/entities/scope.rs`

Adicionar a `entities/mod.rs`:
```rust
pub mod scope;
```

### Testes de Scope

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::value::Value;

    #[test]
    fn define_e_get() {
        let mut scope = Scope::new();
        scope.define("x", Value(()));
        assert!(scope.get("x").is_some());
        assert!(scope.get("y").is_none());
    }

    #[test]
    fn ordem_preservada() {
        // Propriedade central de IndexMap — obrigatório verificar
        let mut scope = Scope::new();
        scope.define("z", Value(()));
        scope.define("a", Value(()));
        scope.define("m", Value(()));
        let names: Vec<&str> = scope.iter().map(|(n, _)| n).collect();
        assert_eq!(names, vec!["z", "a", "m"]);
    }

    #[test]
    fn redefine_mantem_posicao() {
        // IndexMap::insert em chave existente mantém posição
        let mut scope = Scope::new();
        scope.define("a", Value(()));
        scope.define("b", Value(()));
        scope.define("a", Value(()));  // redefinição
        let names: Vec<&str> = scope.iter().map(|(n, _)| n).collect();
        assert_eq!(names, vec!["a", "b"]);
    }

    #[test]
    fn vazio() {
        let scope = Scope::new();
        assert!(scope.is_empty());
        assert_eq!(scope.len(), 0);
    }

    #[test]
    fn get_binding() {
        let mut scope = Scope::new();
        scope.define("x", Value(()));
        assert!(scope.get_binding("x").is_some());
        assert!(scope.get_binding("missing").is_none());
    }
}
```

### Verificação intermédia

```bash
cargo test -p typst-core -- entities::scope
cargo build
crystalline-lint .
# ✓ Scope compila, testes passam, zero violations
```

---

## Tarefa 4 — Module real em L1

**Pré-condição**: Scope compila com testes a passar.

### Arc<ModuleInner> — decisão de domínio de alta performance

Se o original usa `Arc<ModuleInner>`, replicar sem hesitação.

Justificação: em eval(), módulos são clonados e passados entre ramos
da árvore de avaliação. Copiar um `IndexMap` inteiro em cada clone
seria O(n) onde n é o número de bindings do módulo. `Arc` torna o
clone O(1) — é uma propriedade de domínio do compilador, não
infraestrutura. V13 não dispara para `Arc` em campos de struct
(apenas para `static Mutex<T>`).

Se o original usa valor directo (sem Arc) → seguir o original.
Não introduzir Arc que não existe no original.

### Estrutura (ajustar conforme diagnóstico)

**Opção com Arc** (se o original usa):
```rust
use std::sync::Arc;
use crate::entities::scope::Scope;

pub struct Module(Arc<ModuleInner>);

struct ModuleInner {
    name:  String,  // EcoString → String (ADR-0015)
    scope: Scope,
    // content: omitir se depende de Content não migrado
    // adicionar com comentário "// ADR-0016: adiado" se presente no original
}

impl Module {
    pub fn new(name: impl Into<String>, scope: Scope) -> Self {
        Self(Arc::new(ModuleInner {
            name: name.into(),
            scope,
        }))
    }

    pub fn name(&self) -> &str  { &self.0.name }
    pub fn scope(&self) -> &Scope { &self.0.scope }

    /// Clone é O(1) — incrementa contagem de Arc.
    /// Necessário porque módulos são passados entre ramos de eval().
}

impl Clone for Module {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
```

**Opção sem Arc** (se o original usa valor directo):
```rust
#[derive(Clone)]
pub struct Module {
    name:  String,
    scope: Scope,
}

impl Module {
    pub fn new(name: impl Into<String>, scope: Scope) -> Self {
        Self { name: name.into(), scope }
    }
    pub fn name(&self) -> &str    { &self.name }
    pub fn scope(&self) -> &Scope { &self.scope }
    pub fn into_scope(self) -> Scope { self.scope }
}
```

### Campo content

Se `module.rs` original tem `content: Content` e `Content` não
está migrado:

```rust
// Omitir o campo neste passo:
// content: Content,  // ADR-0016: adiado — Content não migrado
```

Documentar a omissão no prompt e no DEBT.md. `eval()` parcial sem
`Content` ainda permite testar a estrutura de scoping.

### Substituir stub

```bash
# Verificar onde Module(()) é construído actualmente
grep -rn "Module(())" 01_core/src/ 03_infra/src/
```

Substituir cada ocorrência pelo construtor real.
Os usos esperados são apenas em testes — ajustar os testes para
usar `Module::new("test", Scope::new())`.

### Prompt L0

**Actualizar**: `00_nucleo/prompts/entities/module.md`

Substituir a doc de stub pela doc do tipo real.

### Testes de Module

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{scope::Scope, value::Value};

    #[test]
    fn nome_e_scope() {
        let mut scope = Scope::new();
        scope.define("x", Value(()));
        let m = Module::new("my-file", scope);
        assert_eq!(m.name(), "my-file");
        assert!(m.scope().get("x").is_some());
    }

    #[test]
    fn clone_consistente() {
        let scope = Scope::new();
        let m1 = Module::new("test", scope);
        let m2 = m1.clone();
        assert_eq!(m1.name(), m2.name());
        // Se Arc: m1 e m2 partilham o mesmo ModuleInner
        // Se valor: m2 é cópia independente
    }

    #[test]
    fn scope_vazio_valido() {
        let m = Module::new("empty", Scope::new());
        assert!(m.scope().is_empty());
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

Critérios de conclusão:
- `Scope::define` + `Scope::get` funciona com `Value(())` stub
- Teste de ordem de inserção passa (obrigatório — justifica IndexMap)
- `Module::new("f", scope).scope().get("x")` funciona
- `Module(())` stub substituído — `cargo build` sem erros
- Se Arc: `Module::clone()` compila sem derive
- Zero violations
- Testes não regridem (195 base + novos)

---

## Ao terminar, reportar

**Do diagnóstico (Tarefa 1):**
- O que `Binding` contém além de `Value` — quais campos foram incluídos e quais omitidos
- Se `Module` usa `Arc` ou valor directo no original
- Se `Module` tem campo `content: Content` — e como foi tratado
- Se apareceu alguma crate além de `indexmap` — número de ADR criada

**Da implementação:**
- Se `Scopes` (pilha de scopes) existe como tipo separado e foi ou não incluído
- Número total de testes
- Zero violations confirmado

**Go/No-Go para o Passo 12:**
Com base no diagnóstico, uma das duas conclusões:
- **GO**: `Scope` e `Module` compilam sem `Content` real → Passo 12 analisa `Routines`/`Engine` e tenta `eval()` parcial
- **NO-GO**: `Content` é necessário para compilar → Passo 12 é análise e migração de `Content`/`Styles` antes de `eval()`
