# Prompt L0 — Content
Hash do Código: e6b6f0af

## Módulo
`01_core/src/entities/content.rs`

## Propósito
`Content` representa a estrutura declarativa do documento Typst produzida
por `eval()`. É puramente declarativa — não desenha, não mede, não renderiza.
Qualquer operação que precise de métricas de fonte ou I/O pertence a L3.

## Divergência do original (Opção D)
O `Content` original (`typst-library/foundations/content/`) usa:
- `pub struct Content(raw::RawContent)` com vtable `unsafe trait NativeElement`
- Proc macros `#[elem]` que geram implementações de `NativeElement`
- Arc manual (fat pointer com ref counting customizado, não `std::sync::Arc`)
- Styles como camada separada via `StyledElem` wrapper

Replicar esta metaprogramação em L1 traria toda a complexidade de
`typst_macros` sem benefício arquitectural. O cristalino diverge
intencionalmente: usa um enum linear com variantes declarativas.

Decisão registada em ADR-0026 (a criar).

## Representação
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    Empty,
    Text(EcoString),         // TextElem mínimo
    Space,                   // SpaceElem / espaço entre palavras
    Sequence(Vec<Content>),  // sequência de elementos
    // Variantes futuras — NÃO implementar sem ADR:
    // Styled(Box<Content>, Styles),             // requer Styles real
    // Heading { level: u8, body: Box<Content> },
    // Strong(Box<Content>),
    // Emph(Box<Content>),
    // Raw { text: EcoString, lang: Option<EcoString> },
    // Elem(Arc<dyn NativeElement>),              // vtable — Passo 20+
}
```

## Interface pública obrigatória
```rust
impl Content {
    pub fn text(s: impl Into<EcoString>) -> Self;
    pub fn empty() -> Self;
    pub fn sequence(parts: Vec<Content>) -> Self;
    pub fn is_empty(&self) -> bool;
    pub fn plain_text(&self) -> String;
}
```

### `sequence()` — normalização
- 0 partes → `Empty`
- 1 parte → desembrulha (evita `Sequence([x])`)
- n > 1 partes → `Sequence(parts)`

### `plain_text()` — para verificação em testes
- `Empty` → `""`
- `Text(s)` → `s.to_string()`
- `Space` → `" "`
- `Sequence(v)` → concatenação recursiva

## Método `map_content` (Passo 69 — DEBT-19)

Percorre a árvore AST de baixo para cima (bottom-up), aplicando uma closure
a cada nó após processar os seus filhos.

```rust
pub fn map_content<F>(&self, transform: &mut F) -> SourceResult<Self>
where
    F: FnMut(&Content) -> SourceResult<Option<Content>>;
```

Semântica:
- `transform` retorna `Some(new_content)` → substituir; o novo nó NÃO é reavaliado.
- `transform` retorna `None` → manter o nó processado (com filhos já transformados).
- O `match` lista explicitamente containers (recursão) e terminais (clone). Sem `_ =>`.

Containers (recursão bottom-up): `Sequence`, `Strong`, `Emph`, `Heading`,
`ListItem`, `EnumItem`, `Link`, `Labelled`, `Figure`, `Equation`, `MathSequence`,
`MathFrac`, `MathAttach`, `MathRoot`, `MathDelimited`, `MathMatrix`, `MathCases`.

Terminais (clone directo): `Text`, `Space`, `Empty`, `Linebreak`, `Outline`,
`Raw`, `Ref`, `SetHeadingNumbering`, `CounterUpdate`, `CounterDisplay`,
`MathAlignPoint`, `MathIdent`, `MathText`.

## Variante `Content::Image` (Passo 71 — DEBT-24)

```rust
Image {
    path:   String,
    data:   std::sync::Arc<Vec<u8>>,
    width:  Option<Box<Value>>,   // Box quebra ciclo Content→Value→Content
    height: Option<Box<Value>>,
},
```

Terminal — sem filhos Content. `Arc<Vec<u8>>` partilhado: clones do AST não copiam bytes.
- `plain_text` → `""`
- `is_empty` → `false`
- `map_content` → terminal: `clone()`
- `map_text` → terminal: `clone()`
- Layouter: placeholder 100×100 pt (DEBT-24b).

## Método `get_field` (Passo 68)

Acesso a campos de elementos estruturados — usado pelas show rules.
Suporta `.body` e `.level` em `Heading`, `.body` em `Figure`.

## Critérios de verificação
- `Content::text("hello").plain_text() == "hello"`
- `Content::empty().is_empty() == true`
- `Content::sequence(vec![]).is_empty() == true`
- `Content::sequence(vec![Content::text("a")]) == Content::text("a")` (desembrulha)
- `Content::sequence(vec![Content::text("a"), Content::Space, Content::text("b")]).plain_text() == "a b"`
- `Content::Empty` e `Content::Space` — clone e PartialEq funcionam
