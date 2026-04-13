# Prompt L0 — `entities/syntax_text`
Hash do Código: 8a032a0b

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/syntax_text.rs`
**Criado em**: 2026-03-22 (Passo 2)
**Atualizado em**: 2026-04-12 (restauro — prompt dedicado criado; antes coberto por `syntax-node.md`)
**ADRs relevantes**: ADR-0004 (SyntaxText como `Arc<str>`), ADR-0015 (remoção de ecow do parser)

---

## Contexto e Objetivo

`SyntaxText` encapsula fragmentos de texto puro (folhas terminais) na CST
(Concrete Syntax Tree). Num parser resiliente, os nós terminais não podem
ser strings despidas — precisam de tipagem forte para interagir com `SyntaxNode`
e garantir compatibilidade no cálculo de `Span` e offsets.

A representação interna (`Arc<str>`) é um detalhe privado do módulo. L1 define
o que uma string de domínio *é*, não como é armazenada. Se no futuro a
performance exigir um backing diferente (ex: `ecow::EcoString`), apenas este
módulo muda — a interface pública de L1 permanece estável.

**Decisão ADR-0015**: `EcoString` foi removido do parser. `SyntaxText` usa
`Arc<str>` directamente — sem dependência externa, clone O(1), semântica
idêntica. Em contraste, `Value::Str` usa `EcoString` (ADR-0024) por ser no
hot path de eval (contextos de uso opostos).

Origem: `lab/typst-original/crates/typst-syntax/src/text.rs`

---

## Restrições Estruturais

- Camada **L1**: zero I/O, zero estado global.
- Representação interna: `Arc<str>` — clone O(1), partilha de texto sem cópia.
- Sem dependências externas (apenas `std::sync::Arc` e `std::fmt`).
- `SyntaxText` é `Clone + Eq + PartialEq + Hash`.
- `len()` retorna bytes (compatível com o cálculo de offsets do parser),
  **não** grafemas nem caracteres Unicode (codepoints).
- Os testes não devem expor `Arc` directamente — apenas testar a interface pública.

---

## Instrução

### Interface pública completa

```rust
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SyntaxText(Arc<str>);

impl SyntaxText {
    /// Cria uma string vazia. Aloca um único Arc partilhado vazio.
    pub fn new() -> Self

    /// Vista do conteúdo como slice `&str`.
    pub fn as_str(&self) -> &str

    /// Comprimento em bytes (não grafemas).
    pub fn len(&self) -> usize

    /// Se a string está vazia.
    pub fn is_empty(&self) -> bool
}

impl Default for SyntaxText        // → SyntaxText::new()

// Conversões From para ergonomia na construção de nós
impl From<&str>   for SyntaxText   // → Arc::from(s)
impl From<String> for SyntaxText   // → Arc::from(s.as_str())

// Comparação directa com &str (sem alocar)
impl PartialEq<str>  for SyntaxText
impl PartialEq<SyntaxText> for str
impl PartialEq<&str> for SyntaxText

// Display para diagnósticos
impl fmt::Display for SyntaxText   // → f.write_str(&self.0)
impl fmt::Debug   for SyntaxText   // → fmt::Debug::fmt(&*self.0, f)
```

---

## Critérios de Verificação

```
SyntaxText::from("hello").as_str() = "hello"
SyntaxText::from("hello").len()    = 5        // bytes, não grafemas
SyntaxText::from("hello").is_empty() = false

SyntaxText::new().is_empty()       = true
SyntaxText::new().len()            = 0
SyntaxText::new().as_str()         = ""

// Clone partilha o Arc
let a = SyntaxText::from("shared");
let b = a.clone();
a == b                             → true     // igualdade de conteúdo

// Display
SyntaxText::from("display-me").to_string() = "display-me"

// From<String>
SyntaxText::from(String::from("world")).as_str() = "world"

// PartialEq<str>
SyntaxText::from("hi") == "hi"     → true
SyntaxText::from("hi") == "bye"    → false

// Comprimento UTF-8 (bytes)
// "olá" em UTF-8 = 4 bytes (o=1, l=1, á=2)
SyntaxText::from("olá").len() = 4
```

---

## Resultado Esperado

- `01_core/src/entities/syntax_text.rs` com `SyntaxText` e testes co-localizados
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/entities/syntax-text.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação — Passo 2: `SyntaxText(Arc<str>)` como tipo de domínio | `syntax_text.rs` |
| 2026-04-12 | Restauro — prompt dedicado criado (antes coberto por `syntax-node.md`) | `syntax-text.md` |
