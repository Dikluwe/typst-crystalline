# Prompt L0 — `entities/source.rs`
Hash do Código: 56021ba0

**Camada**: L1 — domínio puro
**Módulo**: `01_core/src/entities/source.rs`
**Origem**: `lab/typst-original/crates/typst-syntax/src/source.rs`

---

## Contexto

`Source` representa um ficheiro de texto carregado em memória com
a sua CST (Concrete Syntax Tree) associada. É criado em L3 quando
o filesystem carrega um ficheiro, mas o tipo em si é domínio puro:
recebe texto já carregado e chama `parse()` internamente.

Substituí o stub `pub struct Source { pub id: FileId, pub text: String }`
de `world_types.rs` pelo tipo real.

---

## Decisões de migração

| Original | Substituição | ADR |
|----------|-------------|-----|
| `Arc<LazyHash<SourceInner>>` | `Arc<SourceInner>` | 0016 |
| `#[derive(Clone, Hash)]` | apenas `Clone` (sem `Hash`) | 0016 |
| `reparse()` incremental | omitido neste passo | — |
| `LinkedNode::find()` em `find()` | omitido neste passo | — |

`LazyHash` é infraestrutura de hashing para `comemo` — não pertence
a L1 (ADR-0016). `Hash`/`Eq` serão adicionados em Passo 10 quando
`comemo` for isolado em L3.

`reparse()` e `LinkedNode` estão fora do âmbito deste passo —
a interface pública mínima abaixo é suficiente para o Passo 5.

---

## Interface pública

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
}
```

`detached()` usa `FileId::from_raw(NonZeroU16::new(1).unwrap())` como
id sentinel. A convenção é idêntica ao original (`Source::detached`
chama `FileId::detached()` se existir, ou usa um id sentinel fixo).

---

## Critérios de verificação

```
Dado Source::new(id, "Hello *world*".into())
Quando root() for chamado
Então SyntaxNode com kind() == SyntaxKind::Markup e erroneous() == false

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

Dado Source::new(id, "text".into())
Quando id() for chamado
Então o mesmo FileId passado a new()

Dado Source::new(id, "Hello *world*".into())
Quando text() for chamado
Então "Hello *world*"
```

---

## Estrutura interna

```rust
struct SourceInner {
    id: FileId,
    text: String,
    root: SyntaxNode,
}
```

`parse()` é chamado em `new()` e `detached()` para construir `root`.
O módulo `parse` já existe em `01_core/src/rules/parse.rs`.
