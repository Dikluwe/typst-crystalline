# Passo 5 — Source real e AST tipada (v2)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/0001-estrategia-migracao.md`
- `00_nucleo/adr/0004-passo1-descobertas.md` (ecow → Opção C já decidida)
- `00_nucleo/adr/0005-packagespec-world.md` (Source stub e World trait)
- `00_nucleo/adr/0015-ecow.md` (EcoString → SyntaxText na fronteira)

Pré-condição: `cargo test -p typst-core` — 126 testes, zero violations.
Passo 4 concluído — `parse()`, `parse_code()`, `parse_math()` em L1.

### Atenção ao stub de Source

`world_types.rs` tem actualmente:
```rust
pub struct Source { pub id: FileId, pub text: String }
```

Antes de substituir, verificar quantos testes usam este stub:
```bash
grep -rn "Source {" 01_core/src/
grep -rn "Source::new\|Source {" 01_core/src/
```

Se testes existentes dependem dos campos públicos `id` e `text` do
stub — eles vão quebrar quando Source real chegar (campos privados).
Isso é esperado e correcto: os testes estavam a depender de detalhes
de implementação do stub. Corrigir os testes, não a Source real.

### Decisão sobre ecow já tomada

Se `source.rs` usar `EcoString` — a decisão é a mesma do ADR-0015:
converter na fronteira, `SyntaxText`/`String` em L1. Não parar para
reportar — aplicar directamente e registar no relatório final.

---

## Parte A — Source real (substitui stub)

### Diagnóstico obrigatório

```bash
# Dependências externas de source.rs
grep "^use\|^extern" lab/typst-original/crates/typst-syntax/src/source.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Como Source usa parse() internamente
grep -n "parse\|SyntaxNode\|root\|numberize" \
  lab/typst-original/crates/typst-syntax/src/source.rs | head -30

# Estado global
grep -n "^static\|OnceLock\|LazyLock\|Mutex" \
  lab/typst-original/crates/typst-syntax/src/source.rs

# Interface pública completa
grep -n "^pub fn\|^pub struct\|^pub type" \
  lab/typst-original/crates/typst-syntax/src/source.rs

# Tamanho
wc -l lab/typst-original/crates/typst-syntax/src/source.rs
```

Se aparecer qualquer externo não coberto pelas ADRs 0006–0015
— parar e reportar. Criar ADR antes de continuar.

### Tarefa A1 — Prompt L0

**Criar**: `00_nucleo/prompts/entities/source.md`

`Source` é um ficheiro de texto carregado em memória com a sua CST.
Não é I/O — o carregamento acontece em L3. `Source` em L1 recebe
o texto já carregado e chama `parse()` internamente.

Interface pública mínima (confirmar contra diagnóstico):
```rust
pub struct Source { /* privado */ }

impl Source {
    /// Cria Source com FileId explícito — usado por L3 ao carregar ficheiros.
    pub fn new(id: FileId, text: String) -> Self;

    /// Cria Source sem FileId — para testes e contextos sem filesystem.
    pub fn detached(text: impl Into<String>) -> Self;

    pub fn id(&self) -> FileId;
    pub fn text(&self) -> &str;
    pub fn root(&self) -> &SyntaxNode;
    pub fn len_bytes(&self) -> usize;

    /// Localiza o range de bytes de um span nesta Source.
    pub fn find(&self, span: Span) -> Option<Range<usize>>;
}
```

Critérios de verificação:
```
Dado Source::new(id, "Hello".into())
Quando root() for chamado
Então SyntaxNode com kind() == SyntaxKind::Markup, erroneous() == false

Dado Source::new(id, "".into())
Quando len_bytes() for chamado
Então 0

Dado Source::detached("= Heading")
Quando root().children() for iterado
Então existe filho com kind() == SyntaxKind::Heading

Dado Source::new(id, "Hello *world*".into())
Quando root().erroneous() for chamado
Então false

Dado Source::detached("#{{{broken")
Quando root().erroneous() for chamado
Então true
```

### Tarefa A2 — Migrar Source

**Origem**: `lab/typst-original/crates/typst-syntax/src/source.rs`
**Destino**: `01_core/src/entities/source.rs`

Substituições a aplicar durante a migração:

| Original | Substituição | ADR |
|----------|-------------|-----|
| `EcoString` | `String` ou `SyntaxText` conforme contexto | 0015 |
| `eco_format!(...)` | `format!(...)` | 0015 |
| `use ecow::*` | remover | 0015 |

Após criar `source.rs`, actualizar `entities/mod.rs`:
```rust
pub mod source;
```

E remover o stub de `world_types.rs`:
```rust
// Remover:
// pub struct Source { pub id: FileId, pub text: String }
```

`World::source()` retorna `FileResult<Source>` — a assinatura não
muda, apenas o tipo `Source` passa a ser o real. Se `cargo build`
emitir erros de tipo após esta remoção — são erros esperados e
indicam usos incorrectos do stub que precisam de ser corrigidos.

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

# Macro node! — quantas invocações
grep -c "node!" lab/typst-original/crates/typst-syntax/src/ast.rs

# Verificar se node! é macro_rules! ou proc macro
grep -n "macro_rules!\s*node" \
  lab/typst-original/crates/typst-syntax/src/ast.rs | head -5
```

### Decisão de estrutura

- `ast.rs` > 2000 linhas **ou** > 100 tipos públicos → submódulos temáticos
- `ast.rs` ≤ 2000 linhas **e** ≤ 100 tipos → ficheiro único

```
# Se submódulos:
01_core/src/entities/ast/
  mod.rs      — re-exports e AstNode trait
  markup.rs   — Markup, Text, Heading, List*, Term*, ...
  math.rs     — Math, MathAttach, MathFrac, ...
  code.rs     — Ident, Let, Set, Show, If, For, ...
  expr.rs     — Unary, Binary, FuncCall, Closure, ...

# Se ficheiro único:
01_core/src/entities/ast.rs
```

Decidir após o diagnóstico. Documentar a decisão no relatório final.

### Tarefa B1 — Prompt L0

**Criar**: `00_nucleo/prompts/entities/ast.md`
(ou `00_nucleo/prompts/entities/ast/mod.md` se submódulos)

A AST tipada são wrappers com lifetime sobre `SyntaxNode` — zero
externos esperados. Usa apenas tipos já em L1:

```rust
pub trait AstNode<'a>: Sized {
    fn from_untyped(node: &'a SyntaxNode) -> Option<Self>;
    fn to_untyped(self) -> &'a SyntaxNode;
    fn span(&self) -> Span {
        self.to_untyped().span()
    }
}
```

A macro `node!` implementa `AstNode` para cada tipo concreto.
Verificar se migra directamente ou precisa de adaptação para
os tipos já em L1 (`SyntaxKind`, `SyntaxNode`, `Span`).

Critérios de verificação mínimos:
```
Dado SyntaxNode com kind() == SyntaxKind::Markup
Quando Markup::from_untyped(&node) for chamado
Então Some(markup)

Dado SyntaxNode com kind() == SyntaxKind::Text
Quando Markup::from_untyped(&node) for chamado
Então None (kind errado)

Dado AstNode válido
Quando span() for chamado
Então mesmo span que to_untyped().span()
```

### Tarefa B2 — Migrar ast.rs

**Origem**: `lab/typst-original/crates/typst-syntax/src/ast.rs`
**Destino**: `01_core/src/entities/ast.rs` (ou submódulos)

Se `ast.rs` usar `EcoString` — aplicar ADR-0015 directamente
(substituir por `SyntaxText`/`String`). Não parar para reportar.

Adicionar a `entities/mod.rs`:
```rust
pub mod ast;  // ou pub mod ast; com mod ast { pub mod markup; ... }
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

Testes de paridade obrigatórios para Source:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU16;
    use crate::entities::{
        file_id::FileId,
        syntax_kind::SyntaxKind,
    };

    fn test_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    #[test]
    fn source_root_markup() {
        let src = Source::new(test_id(), "Hello *world*".into());
        assert_eq!(src.root().kind(), SyntaxKind::Markup);
        assert!(!src.root().erroneous());
    }

    #[test]
    fn source_vazia() {
        let src = Source::new(test_id(), "".into());
        assert_eq!(src.len_bytes(), 0);
        assert_eq!(src.text(), "");
    }

    #[test]
    fn source_detached_heading() {
        let src = Source::detached("= Heading");
        let has_heading = src.root()
            .children()
            .any(|n| n.kind() == SyntaxKind::Heading);
        assert!(has_heading);
    }

    #[test]
    fn source_com_erros() {
        let src = Source::detached("#{{{broken");
        assert!(src.root().erroneous());
    }

    #[test]
    fn source_id_roundtrip() {
        let id = test_id();
        let src = Source::new(id, "text".into());
        assert_eq!(src.id(), id);
    }

    #[test]
    fn source_text_preservado() {
        let src = Source::new(test_id(), "Hello *world*".into());
        assert_eq!(src.text(), "Hello *world*");
    }
}
```

---

## Ao terminar, reportar

- Se `source.rs` usava `ecow` e quantas substituições foram feitas
- Estrutura escolhida para AST (ficheiro único ou submódulos) e porquê
- Se a macro `node!` migrou sem alterações ou precisou de adaptação
- Quais campos do stub de `Source` estavam a ser usados e como foram corrigidos
- Número total de testes
- Se V14 disparou para algum externo novo — e o número do ADR criado

Esta informação vai para o Passo 6 (`eval()`).
