# Passo 5 — Source real e AST tipada

## Contexto

Ler antes de começar:
- `00_nucleo/adr/0005-packagespec-world.md` (stubs e Source stub)
- `lab/typst-original/crates/typst-syntax/src/source.rs`
- `lab/typst-original/crates/typst-syntax/src/ast.rs`

Pré-condição: Passo 4 concluído — `parse()` em `01_core/rules/parse.rs`.

Este passo tem duas partes independentes que podem ser feitas
em qualquer ordem, mas `Source` real precisa de existir antes
do Passo 6 (`eval()`).

---

## Parte A — Source real (substitui stub)

### Diagnóstico obrigatório

```bash
# Dependências externas de source.rs
grep "^use\|^extern" lab/typst-original/crates/typst-syntax/src/source.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Como Source usa parse() internamente
grep -n "parse\|SyntaxNode\|root" \
  lab/typst-original/crates/typst-syntax/src/source.rs | head -20

# Estado global
grep -n "^static\|OnceLock\|LazyLock\|Mutex" \
  lab/typst-original/crates/typst-syntax/src/source.rs

# Interface pública de Source
grep -n "^pub fn\|^pub struct" \
  lab/typst-original/crates/typst-syntax/src/source.rs
```

Se aparecer `ecow` — Opção C (converter na fronteira L3→L1).
Se aparecer outros externos — parar e reportar.

### Tarefa A1 — Prompt L0

**Criar**: `00_nucleo/prompts/entities/source.md`

`Source` é um ficheiro de texto carregado em memória com a sua CST.
Não é I/O — o carregamento acontece em L3. `Source` em L1 recebe
o texto já carregado e chama `parse()` internamente.

Interface mínima:
```rust
pub struct Source { /* privado */ }

impl Source {
    pub fn new(id: FileId, text: String) -> Self;
    pub fn detached(text: impl Into<String>) -> Self; // para testes
    pub fn id(&self) -> FileId;
    pub fn text(&self) -> &str;
    pub fn root(&self) -> &SyntaxNode;
    pub fn len_bytes(&self) -> usize;
    pub fn find(id: FileId, span: Span) -> Option<Range<usize>>;
}
```

Critérios:
```
Dado Source::new(id, "Hello".into())
Quando root() for chamado
Então retorna SyntaxNode com kind() == SyntaxKind::Markup

Dado Source::new(id, "".into())
Quando len_bytes() for chamado
Então 0

Dado Source::detached("= Heading")
Quando root().children() for iterado
Então encontra SyntaxKind::Heading
```

### Tarefa A2 — Migrar Source

**Origem**: `lab/typst-original/crates/typst-syntax/src/source.rs`
**Destino**: `01_core/src/entities/source.rs`

Substituir o stub em `world_types.rs`:
```rust
// Remover de world_types.rs:
// pub struct Source { pub id: FileId, pub text: String }

// Adicionar a entities/mod.rs:
pub mod source;
```

`World::source()` já retorna `FileResult<Source>` — a assinatura
não muda, apenas o tipo `Source` passa a ser o real.

---

## Parte B — AST tipada

### Diagnóstico obrigatório

```bash
# Dependências externas de ast.rs
grep "^use\|^extern" lab/typst-original/crates/typst-syntax/src/ast.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Tamanho e número de tipos públicos
wc -l lab/typst-original/crates/typst-syntax/src/ast.rs
grep -c "^pub struct\|^pub enum\|^pub trait" \
  lab/typst-original/crates/typst-syntax/src/ast.rs

# Macro interna node!
grep -n "macro_rules!" lab/typst-original/crates/typst-syntax/src/ast.rs
```

### Decisão de estrutura

Se `ast.rs` tiver > 100 tipos públicos ou > 2000 linhas,
dividir em submódulos temáticos:

```
01_core/src/entities/ast/
  mod.rs      — re-exports e AstNode trait
  markup.rs   — Markup, Text, Heading, List*, Term*, ...
  math.rs     — Math, MathAttach, MathFrac, ...
  code.rs     — Ident, Let, Set, Show, If, For, ...
  expr.rs     — Unary, Binary, FuncCall, Closure, ...
```

Se < 100 tipos e < 2000 linhas, um único `ast.rs` é suficiente.
**Decidir após o diagnóstico, não antes.**

### Tarefa B1 — Prompt L0

**Criar**: `00_nucleo/prompts/entities/ast.md`
(ou `00_nucleo/prompts/entities/ast/mod.md` se dividido)

A AST tipada são wrappers com lifetime sobre `SyntaxNode`:
```rust
pub trait AstNode<'a>: Sized {
    fn from_untyped(node: &'a SyntaxNode) -> Option<Self>;
    fn to_untyped(self) -> &'a SyntaxNode;
    fn span(&self) -> Span { self.to_untyped().span() }
}
```

Zero externos esperados — usa apenas `SyntaxNode`, `SyntaxKind`,
`Span` já em L1.

### Tarefa B2 — Migrar ast.rs

**Origem**: `lab/typst-original/crates/typst-syntax/src/ast.rs`
**Destino**: `01_core/src/entities/ast.rs` (ou submódulos)

A macro `node!` provavelmente gera implementações de `AstNode`.
Verificar se migra directamente ou precisa de adaptação.

---

## Actualizar world_types.rs

Após Source real estar migrado, remover o stub e verificar que
`World::source()` retorna o tipo correcto:

```bash
cargo build
# Erros de tipo aqui são esperados e correctos —
# indicam onde o stub estava a ser usado de forma incorrecta
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Testes de paridade para Source:
```rust
#[test]
fn source_root_e_parse() {
    use std::num::NonZeroU16;
    let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
    let src = Source::new(id, "Hello *world*".into());
    assert_eq!(src.root().kind(), SyntaxKind::Markup);
    assert!(!src.root().erroneous());
}
```

---

## Ao terminar, reportar

- Se `source.rs` usava `ecow` e como foi tratado
- Estrutura escolhida para AST (ficheiro único ou submódulos)
- Se a macro `node!` migrou sem alterações
- Número total de testes
- Tipos de `Source` que mudaram face ao stub

Essa informação vai para ADR-0006 e para o Passo 6 (`eval()`).
