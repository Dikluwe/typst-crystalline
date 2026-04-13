# Prompt L0 — `entities/syntax-node`
Hash do Código: 5ebed19c

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/syntax_node.rs`
**Criado em**: 2026-03-22 (Passo 2)
**Atualizado em**: 2026-04-12 (restauro — expansão para documentar a implementação completa)
**ADRs relevantes**: ADR-0004 (SyntaxText como `Arc<str>`), ADR-0015 (remoção de ecow do parser)

---

## Contexto

`SyntaxNode` é o tipo central da **CST (Concrete Syntax Tree)** do Typst.
É a estrutura sobre a qual o parser produz a árvore e o `eval.rs` opera para
gerar `Content`.

A implementação usa três representações internas para otimizar diferentes casos:

| Variante | Uso | Alocação |
|----------|-----|----------|
| `Leaf` | Token terminal (texto + kind + span) | inline (sem Arc) |
| `Inner` | Nó com filhos | `Arc<InnerNode>` — partilhado entre revisões |
| `Error` | Token com erro de parse | `Arc<ErrorNode>` — raramente alocado |

`Arc` partilha nós entre revisões incrementais sem cópia pelo parser incremental.
`SyntaxText` é `Arc<str>` — clone O(1) (ADR-0004).

O `ecow` foi removido do nível do parser (ADR-0015). `SyntaxText` é wrapper
sobre `Arc<str>` com as mesmas propriedades de clone de que `EcoString`, mas
sem a dependência de crate externa.

Origem: `lab/typst-original/crates/typst-syntax/src/node.rs`

---

## Restrições Estruturais

- Camada **L1**: zero I/O, zero estado global.
- `Arc` em `Inner` e `Error` é gestão de RAM, não I/O (ADR-0029).
- `Rc` em `LinkedNode` é justificado por ser single-threaded (referência ao
  pai na travessia — sem `Send`). Não usar `Arc` aqui.
- `SyntaxNode` implementa `Clone + Eq + PartialEq + Hash`.
- `nodeize`, `replace_children`, `update_parent` são `pub(crate)` — usados
  exclusivamente pelo parser (Passo 4) no mesmo crate.
- **Invariante de erro**: `SyntaxNode::leaf()` e `SyntaxNode::inner()` fazem
  `debug_assert!(!kind.is_error())`. Nós de erro são criados apenas via
  `SyntaxNode::error()`.

---

## Instrução

### Tipos públicos

```rust
pub struct SyntaxNode(NodeKind);    // CST node

pub struct SyntaxError {            // erro sintáctico
    pub span: Span,
    pub message: SyntaxText,
    pub hints: Vec<SyntaxText>,
}

pub struct LinkedNode<'a> {         // nó em contexto com offset e parent
    node: &'a SyntaxNode,
    parent: Option<Rc<Self>>,
    index: usize,
    offset: usize,
}

pub enum Side { Before, After }     // âncora para leaf_at

pub(crate) type NumberingResult = Result<(), Unnumberable>;
pub(crate) struct Unnumberable;
```

### Enumeração interna

```rust
enum NodeKind {
    Leaf(LeafNode),
    Inner(Arc<InnerNode>),
    Error(Arc<ErrorNode>),
}
```

### Interface pública de `SyntaxNode`

```rust
// Construtores
pub fn leaf(kind: SyntaxKind, text: impl Into<SyntaxText>) -> Self
pub fn inner(kind: SyntaxKind, children: Vec<SyntaxNode>) -> Self
pub fn error(error: SyntaxError, text: impl Into<SyntaxText>) -> Self
pub fn placeholder(kind: SyntaxKind) -> Self   // nó de preenchimento vazio

// Leitores
pub fn kind(&self) -> SyntaxKind
pub fn len(&self) -> usize
pub fn is_empty(&self) -> bool
pub fn span(&self) -> Span
pub fn text(&self) -> SyntaxText          // O(1) — clone de Arc<str>
pub fn into_text(self) -> SyntaxText      // O(n) para Inner
pub fn text_str(&self) -> &str            // emprestado
pub fn children(&self) -> std::slice::Iter<'_, SyntaxNode>
pub fn erroneous(&self) -> bool
pub fn errors(&self) -> Vec<SyntaxError>

// Mutação
pub fn hint(&mut self, hint: impl Into<SyntaxText>)   // só actua em Error
pub fn synthesize(&mut self, span: Span)              // atribui span sintético
pub fn spanless_eq(&self, other: &Self) -> bool
```

### Interface `pub(crate)` (apenas para `parse.rs`)

```rust
pub(crate) fn convert_to_kind(&mut self, kind: SyntaxKind)
pub(crate) fn convert_to_error(&mut self, message: impl Into<SyntaxText>)
pub(crate) fn expected(&mut self, expected: &str)
pub(crate) fn unexpected(&mut self)
pub(crate) fn numberize(&mut self, id: FileId, within: Range<u64>) -> NumberingResult
pub(crate) fn is_leaf(&self) -> bool
pub(crate) fn descendants(&self) -> usize
pub(crate) fn children_mut(&mut self) -> &mut [SyntaxNode]
pub(crate) fn replace_children(&mut self, range: Range<usize>, replacement: Vec<SyntaxNode>) -> NumberingResult
pub(crate) fn update_parent(&mut self, prev_len, new_len, prev_descendants, new_descendants)
pub(crate) fn upper(&self) -> u64
```

### Interface `LinkedNode<'a>`

```rust
pub fn new(root: &'a SyntaxNode) -> Self
pub fn get(&self) -> &'a SyntaxNode
pub fn index(&self) -> usize
pub fn offset(&self) -> usize
pub fn range(&self) -> Range<usize>
pub fn children(&self) -> LinkedChildren<'a>
pub fn find(&self, span: Span) -> Option<LinkedNode<'a>>
pub fn parent(&self) -> Option<&Self>
pub fn prev_sibling(&self) -> Option<Self>
pub fn next_sibling(&self) -> Option<Self>
pub fn prev_sibling_with_trivia(&self) -> Option<Self>
pub fn next_sibling_with_trivia(&self) -> Option<Self>
pub fn parent_kind(&self) -> Option<SyntaxKind>
pub fn prev_sibling_kind(&self) -> Option<SyntaxKind>
pub fn next_sibling_kind(&self) -> Option<SyntaxKind>
// + leaf_at, leftmost_leaf, rightmost_leaf (ver implementação)

// Deref para SyntaxNode — dá acesso transparente a kind(), len(), span(), etc.
impl Deref for LinkedNode<'_> { type Target = SyntaxNode }
```

---

## Critérios de Verificação

```
SyntaxNode::leaf(SyntaxKind::Text, "hi").kind()      = SyntaxKind::Text
SyntaxNode::leaf(SyntaxKind::Text, "hi").text_str()  = "hi"
SyntaxNode::leaf(SyntaxKind::Text, "hi").len()       = 2
SyntaxNode::leaf(SyntaxKind::Text, "hi").erroneous() = false

SyntaxNode::inner(SyntaxKind::Markup, vec![leaf]).children().count() = 1
SyntaxNode::inner(kind, children).len() = soma dos len() dos filhos
SyntaxNode::inner(kind, children).erroneous() = true se qualquer filho for erróneo

SyntaxNode::error(SyntaxError::new("oops"), "x").erroneous() = true
SyntaxNode::error(...).errors().len() = 1
SyntaxNode::error(...).kind() = SyntaxKind::Error

// SyntaxError
SyntaxError::new("msg").span = Span::detached()
SyntaxError::new("msg").hints = []

// LinkedNode
LinkedNode::new(&leaf).kind() = SyntaxKind::Text
LinkedNode::new(&leaf).offset() = 0
LinkedNode::new(&root).children().count() = root.children().count()
```

---

## Resultado Esperado

- `01_core/src/entities/syntax_node.rs` com todos os tipos documentados acima
- Testes co-localizados em `#[cfg(test)]` cobrindo os critérios acima
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/entities/syntax-node.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação — Passo 2: migração de SyntaxNode, SyntaxError, LinkedNode | `syntax_node.rs` |
| 2026-04-12 | Restauro — expansão do prompt para documentar interface completa, `pub(crate)`, `LinkedNode`, `Side`, `NumberingResult` | `syntax-node.md` |
