# Prompt L0 — `entities/ast/markup`

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/ast/markup.rs`
**Criado em**: 2026-03-25 (Passo 5)
**Atualizado em**: 2026-04-12 (restauro — prompt dedicado criado; antes coberto por `ast/mod.md`)
**ADRs relevantes**: ADR-0006 (zero-copy AST)

---

## Contexto e Objetivo

Define os nós tipados do modo de formatação textual (Markup). A ordem e
aninhamento dos nós é delegada à árvore sintática (`SyntaxNode`); este módulo
apenas providencia a **lente de acesso** tipada via `AstNode<'a>`.

Todos os nós são gerados pela macro `node!` de `ast/mod.rs` e implementam
`AstNode<'a>` automaticamente. Os métodos adicionados a cada struct extraem
propriedades essenciais de forma tipada e segura.

---

## Restrições Estruturais

- Camada **L1**: zero I/O. Depende de `AstNode`, `SyntaxNode`, `SyntaxKind`, `Scanner` (todos L1).
- Nenhum nó aloca memória — todos são wrappers de `&'a SyntaxNode`.
- `exprs()` de `Markup` usa `Expr::cast_with_space` (de `expr.rs`) para incluir
  espaços entre statements e filtrar `Space` pós-statement.

---

## Instrução — Nós públicos e interface

```rust
// ── Trivia ──────────────────────────────────────────────────────────────────
node! { struct LineComment }   // "// ..."
impl LineComment<'a>  { fn text(self) -> &'a str }  // sem o prefixo "//"

node! { struct BlockComment }  // "/* ... */"
impl BlockComment<'a> { fn text(self) -> &'a str }  // sem "/*" e "*/"

// ── Estrutura do documento ──────────────────────────────────────────────────
node! { struct Markup }        // raiz do documento / corpo de strong/emph/etc.
impl Markup<'a> {
    fn exprs(self) -> impl DoubleEndedIterator<Item = Expr<'a>>
    // Itera exprs, filtrando Space pós-statement (is_stmt())
}

// ── Texto e espaço ──────────────────────────────────────────────────────────
node! { struct Text }
impl Text<'a>       { fn get(self) -> &'a str }

node! { struct Space }
node! { struct Linebreak }     // "\"
node! { struct Parbreak }      // linha em branco

// ── Caracteres especiais ────────────────────────────────────────────────────
node! { struct Escape }
impl Escape<'_>     { fn get(self) -> char }  // resolve "\#", "\u{1F5FA}"

node! { struct Shorthand }
impl Shorthand<'_>  {
    const LIST: &'static [(&'static str, char)]  // "..." → '…', "~" → NBSP, etc.
    fn get(self) -> char                          // lookup em LIST
}

node! { struct SmartQuote }
impl SmartQuote<'_> { fn double(self) -> bool }   // true se \"

// ── Conteúdo formatado ──────────────────────────────────────────────────────
node! { struct Strong }        // "*bold*"
impl Strong<'a>     { fn body(self) -> Markup<'a> }

node! { struct Emph }          // "_italic_"
impl Emph<'a>       { fn body(self) -> Markup<'a> }

// ── Raw ─────────────────────────────────────────────────────────────────────
node! { struct Raw }           // "`code`" ou "```block```"
impl Raw<'a> {
    fn lines(self) -> impl DoubleEndedIterator<Item = Text<'a>>
    fn lang(self) -> Option<RawLang<'a>>   // None se delim len < 3
    fn block(self) -> bool                 // true para delim >= 3 backticks com newline
}
node! { struct RawLang }
impl RawLang<'a>    { fn get(self) -> &'a str }
node! { struct RawDelim }      // pub(crate) len()

// ── Hiperligações e Etiquetas ───────────────────────────────────────────────
node! { struct Link }
impl Link<'a>       { fn get(self) -> &'a str }   // URL literal

node! { struct Label }         // "<intro>"
impl Label<'a> {
    fn get(self) -> &'a str   // texto sem < e >
    // strip_prefix('<').trim_end_matches('>')
}

node! { struct Ref }           // "@target" ou "@target[supplement]"
impl Ref<'a> {
    fn target(self) -> &'a str             // strip_prefix('@') do RefMarker
    fn supplement(self) -> Option<ContentBlock<'a>>
}

// ── Estruturas de documento ─────────────────────────────────────────────────
node! { struct Heading }       // "== Título"
impl Heading<'a> {
    fn body(self) -> Markup<'a>
    fn depth(self) -> NonZeroUsize   // número de '=' em HeadingMarker
}

node! { struct ListItem }      // "- item"
impl ListItem<'a>   { fn body(self) -> Markup<'a> }

node! { struct EnumItem }      // "+ item" ou "1. item"
impl EnumItem<'a> {
    fn number(self) -> Option<u64>   // None se não tiver número explícito
    fn body(self) -> Markup<'a>
}

node! { struct TermItem }      // "/ Termo: Descrição"
impl TermItem<'a> {
    fn term(self) -> Markup<'a>
    fn description(self) -> Markup<'a>
}

node! { struct ContentBlock }  // "[*conteúdo*]"
impl ContentBlock<'a> { fn body(self) -> Markup<'a> }
```

---

## Critérios de Verificação

```
// Strong body acessível
Source::detached("*bold*").root().children().find_map(Strong::from_untyped)
    .is_some() = true
strong.body() → Markup (corpo sem os *)

// Heading depth
Source::detached("= Heading") → heading.depth().get() = 1
Source::detached("== Sub")    → heading.depth().get() = 2

// Text get
Source::detached("hello").root().children().find_map(Text::from_untyped)
    .map(Text::get) = Some("hello")

// Label.get() remove < e >
// (contrato: ast::markup::Label.get() retorna &str sem delimitadores)
Label::from_untyped; // tipo existe

// from_untyped errado retorna None
Markup::from_untyped(text_node) = None
```

---

## Resultado Esperado

- `01_core/src/entities/ast/markup.rs` com todos os nós documentados
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/entities/ast/markup.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-25 | Criação — Passo 5: nós de markup básicos | `ast/markup.rs` |
| 2026-04-12 | Restauro — prompt dedicado; interface completa, Shorthand::LIST, Raw.block(), EnumItem.number() | `ast/markup.md` |
