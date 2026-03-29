# Passo 22 — Content::Strong, Emph, Heading e rich text

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/content.rs` — enum Content actual
- `01_core/src/entities/font_book.rs` — `FontBook::select(family, variant)`
- `01_core/src/rules/layout.rs` — `Layouter<M>`, `FrameItem`

Pré-condição: `cargo test` — 340 testes (307 L1 + 33 L3), zero violations.

### Decisão arquitectural a registar ANTES de escrever qualquer código

**Registar em DEBT.md** antes de começar a implementação:

```markdown
## StyleChain — dívida estrutural do sistema de estilos

`TextStyle { bold, italic, size }` é um struct plano.
O Typst real tem centenas de propriedades de estilo (kerning, tracking,
cores, stroke, fallback fonts, propriedades de tabela, etc.).
Manter um struct plano significa que cada nó da árvore copia N bytes
onde N cresce linearmente com propriedades — inaceitável a longo prazo.

O Typst original usa StyleChain: lista ligada construída de trás para
a frente. Cada nó carrega apenas o "delta" (o que mudou); o Layouter
sobe a cadeia para encontrar o primeiro valor definido. Custo: O(1)
de alocação por nó, não O(N).

A sintaxe `#set text(font: "Arial", size: 10pt)` requer StyleChain —
é incompatível com struct plano.

Estimativa de refactorização: Passo 30+, após o pipeline básico estar
estável. Não tentar antes.

Ficheiros a refactorizar quando chegar a hora:
- 01_core/src/entities/layout_types.rs (TextStyle → StyleChain)
- 01_core/src/rules/layout.rs (Layouter, contexto de estilo)
- 01_core/src/entities/content.rs (Content::Styled com StyleChain)
- 03_infra/src/export.rs (resolução de estilos para PDF)
```

Este registo é a "bomba-relógio" que impede que a dívida seja
esquecida. A implementação que se segue usa `TextStyle` plano — é
a escolha correcta para este passo. Não tentar implementar StyleChain
agora.

---

## Tarefa 1 — Diagnóstico de API da AST

```bash
# API de Strong, Emph, Heading na AST cristalina
grep -n "pub fn\|Strong\|Emph\|Heading" \
  01_core/src/entities/ast/markup.rs | head -30

# .body() retorna SyntaxNode, Markup, ou outro?
grep -n "fn body\|fn level" \
  01_core/src/entities/ast/markup.rs | head -10

# SyntaxKind para Strong/Emph/Heading
grep -n "Strong\|Emph\|Heading" \
  01_core/src/entities/syntax_kind.rs | head -10
```

---

## Tarefa 2 — Novas variantes de Content

```rust
// Em 01_core/src/entities/content.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    Empty,
    Text(EcoString),
    Space,
    Sequence(Vec<Content>),

    // ── Rich text (Passo 22) ────────────────────────────────────────────
    Strong(Box<Content>),
    Emph(Box<Content>),
    Heading { level: u8, body: Box<Content> },

    // Variantes futuras — não implementar sem ADR:
    // Styled(Box<Content>, StyleChain),  // requer StyleChain — Passo 30+
    // Raw { text: EcoString, lang: Option<EcoString> },
    // Link { url: EcoString, body: Box<Content> },
    // List(Vec<Content>),
}

impl Content {
    pub fn strong(body: Content) -> Self { Self::Strong(Box::new(body)) }
    pub fn emph(body: Content)   -> Self { Self::Emph(Box::new(body)) }
    pub fn heading(level: u8, body: Content) -> Self {
        Self::Heading { level: level.clamp(1, 6), body: Box::new(body) }
    }
}
```

Actualizar `plain_text()` e `is_empty()` para as novas variantes.

---

## Tarefa 3 — TextStyle e FrameItem

```rust
// Em 01_core/src/entities/layout_types.rs

/// Estilo de texto — struct plano.
///
/// DEBT: deve ser substituído por StyleChain (lista ligada de deltas)
/// antes de implementar `#set text(...)`. Ver DEBT.md.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextStyle {
    pub bold:   bool,
    pub italic: bool,
    pub size:   Pt,
}

impl TextStyle {
    pub fn regular(size: Pt) -> Self { Self { bold: false, italic: false, size } }
    pub fn bold(size: Pt)    -> Self { Self { bold: true,  italic: false, size } }
    pub fn italic(size: Pt)  -> Self { Self { bold: false, italic: true,  size } }
}

// Actualizar FrameItem:
pub enum FrameItem {
    Text {
        pos:   Point,
        text:  EcoString,
        style: TextStyle,  // substitui font_size: Pt
    },
}
```

**Breaking change**: substituir todos os usos de `font_size: Pt` em
`FrameItem::Text` por `style: TextStyle`. Afecta `layout.rs`,
`export.rs`, e testes existentes — actualizar antes de continuar.

---

## Tarefa 4 — eval_markup processa Strong/Emph/Heading

```rust
// Em eval.rs — dentro de eval_markup, expandir o match:

SyntaxKind::Strong => {
    if let Some(node) = ast::Strong::from_untyped(child) {
        // Confirmar API: .body() retorna o que?
        // Assumir que retorna um nó que pode ser passado a eval_markup
        let body = eval_markup_body(node.body(), scopes, ctx)?;
        parts.push(Content::strong(body));
    }
}
SyntaxKind::Emph => {
    if let Some(node) = ast::Emph::from_untyped(child) {
        let body = eval_markup_body(node.body(), scopes, ctx)?;
        parts.push(Content::emph(body));
    }
}
SyntaxKind::Heading => {
    if let Some(node) = ast::Heading::from_untyped(child) {
        let level = node.level().get() as u8;
        let body  = eval_markup_body(node.body(), scopes, ctx)?;
        parts.push(Content::heading(level, body));
    }
}
```

```rust
/// Avalia o corpo de um elemento de markup como Content.
fn eval_markup_body(
    node: /* tipo confirmado no diagnóstico */,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Content> {
    match eval_markup(&node.to_untyped(), scopes, ctx)? {
        Value::Content(c) => Ok(c),
        _                 => Ok(Content::Empty),
    }
}
```

---

## Tarefa 5 — Layouter com contexto de estilo

```rust
// Em Layouter — campo de estilo actual e restauração

pub struct Layouter<M: FontMetrics> {
    // ... campos existentes ...
    style: TextStyle,
}

impl<M: FontMetrics> Layouter<M> {
    pub fn new(metrics: M, font_size: f64) -> Self {
        let size = Pt(font_size);
        // ... inicialização existente ...
        // Novo:
        style: TextStyle::regular(size),
        // ...
    }

    pub fn layout_content(&mut self, content: &Content) {
        match content {
            // ... casos existentes ...

            Content::Strong(body) => {
                let prev = self.style;
                self.style = TextStyle::bold(self.font_size_pt);
                self.layout_content(body);
                self.style = prev;
            }

            Content::Emph(body) => {
                let prev = self.style;
                self.style = TextStyle::italic(self.font_size_pt);
                self.layout_content(body);
                self.style = prev;
            }

            Content::Heading { level, body } => {
                let scale = heading_scale(*level);
                let heading_size = self.font_size_pt * scale;
                let prev = self.style;
                self.style = TextStyle { bold: true, italic: false, size: heading_size };
                if self.cursor_x > MARGIN { self.flush_line(); }
                self.layout_content(body);
                self.flush_line();
                self.style = prev;
            }
        }
    }

    fn layout_word(&mut self, word: &str) {
        let w = self.metrics.advance(word, self.style.size);
        if self.cursor_x + w > Size::a4().width - MARGIN && self.cursor_x > MARGIN {
            self.flush_line();
        }
        self.current_line.push(FrameItem::Text {
            pos:   Point { x: self.cursor_x, y: self.cursor_y },
            text:  word.into(),
            style: self.style,
        });
        self.cursor_x += w + self.metrics.advance(" ", self.style.size);
    }
}

fn heading_scale(level: u8) -> f64 {
    match level { 1 => 2.0, 2 => 1.667, 3 => 1.333, 4 => 1.167, _ => 1.0 }
}
```

---

## Tarefa 6 — Actualizar export_pdf para TextStyle

```rust
// Em 03_infra/src/export.rs — actualizar build_page_stream:

FrameItem::Text { pos, text, style } => {
    let pdf_y = page_height - pos.y.val();
    let safe  = escape_pdf_string(text.as_str());
    if safe.is_empty() { continue; }

    let font_ref = match (style.bold, style.italic) {
        (true,  _)     => "F2",  // Helvetica-Bold
        (false, true)  => "F3",  // Helvetica-Oblique
        (false, false) => "F1",  // Helvetica
    };

    ops.push_str(&format!(
        "BT\n/{font_ref} {:.1} Tf\n{:.1} {:.1} Td\n({safe}) Tj\nET\n",
        style.size.val(), pos.x.val(), pdf_y
    ));
}
```

Adicionar F2 e F3 ao Resources de cada página e aos objectos de
fonte do PDF (Helvetica-Bold, Helvetica-Oblique).

---

## Testes

```rust
// Testes de Content

#[test]
fn strong_plain_text_preservado() {
    assert_eq!(Content::strong(Content::text("bold")).plain_text(), "bold");
}

#[test]
fn heading_level_clamped() {
    assert!(matches!(Content::heading(0, Content::Empty), Content::Heading { level: 1, .. }));
    assert!(matches!(Content::heading(9, Content::Empty), Content::Heading { level: 6, .. }));
}

// Testes de Layouter

#[test]
fn strong_produz_bold_style() {
    let doc = layout(&Content::strong(Content::text("Bold")));
    let bold = doc.pages.iter()
        .flat_map(|p| p.items.iter())
        .any(|i| matches!(i, FrameItem::Text { style, .. } if style.bold));
    assert!(bold, "Strong deve produzir FrameItem com bold=true");
}

#[test]
fn heading_h1_tamanho_maior() {
    let doc = layout(&Content::Sequence(vec![
        Content::heading(1, Content::text("Title")),
        Content::text("body"),
    ]));
    let sizes: Vec<f64> = doc.pages.iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| match i {
            FrameItem::Text { style, .. } => Some(style.size.val()),
            _ => None,
        })
        .collect();
    let max_size = sizes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_size = sizes.iter().cloned().fold(f64::INFINITY, f64::min);
    assert!(max_size > min_size, "H1 deve ter tamanho maior que o texto normal");
}

#[test]
fn estilo_restaurado_apos_strong() {
    // Texto após Strong deve ter estilo regular novamente
    let doc = layout(&Content::Sequence(vec![
        Content::strong(Content::text("Bold")),
        Content::text("normal"),
    ]));
    let items: Vec<_> = doc.pages.iter()
        .flat_map(|p| p.items.iter())
        .collect();
    // Último item deve ser regular (não bold)
    if let Some(FrameItem::Text { style, text, .. }) = items.last() {
        if text.as_str() == "normal" {
            assert!(!style.bold, "texto após Strong deve ser regular");
        }
    }
}

// Pipeline end-to-end
#[test]
fn pipeline_rich_text_plain_text_correcto() {
    let world = MockWorld::new("Hello *bold* and _italic_");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let content = module.content().expect("deve ter content");
    let text = content.plain_text();
    assert!(text.contains("Hello"),  "{:?}", text);
    assert!(text.contains("bold"),   "{:?}", text);
    assert!(text.contains("italic"), "{:?}", text);
}
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra
cargo build
crystalline-lint .
# ✓ No violations found

# Verificar DEBT.md tem a entrada StyleChain
grep -l "StyleChain" 01_core/DEBT.md && echo "OK" || echo "FALTA"
```

Critérios de conclusão:
- DEBT.md tem entrada StyleChain com estimativa Passo 30+ ✓
- `Content::Strong`, `Emph`, `Heading` no enum ✓
- `FrameItem::Text` usa `style: TextStyle` ✓
- Strong → `bold=true`, Emph → `italic=true` nos FrameItems ✓
- H1 tem `size > 12pt` ✓
- Estilo restaurado após bloco Strong/Emph ✓
- Pipeline "Hello *bold* and _italic_" → plain_text correcto ✓
- Zero violations ✓
- Testes não regridem (340 base + novos) ✓

---

## Ao terminar, reportar

**Da implementação:**
- API real de `ast::Strong::body()`, `ast::Heading::level()` — confirmada
- Se breaking change em FrameItem (font_size→style) causou problemas inesperados
- Se o PDF tem Helvetica-Bold/Oblique visíveis num leitor

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 23:**
- **GO — mais Content**: Rich text funciona; Passo 23 adiciona
  `Content::Raw`, listas, e links
- **GO — font embedding**: se texto não-ASCII ainda usa `?`;
  Passo 23 embebe fonte TrueType no PDF para WinAnsiEncoding
- **ATENÇÃO StyleChain**: qualquer tentativa de implementar `#set`
  ou `#show` rules antes do Passo 30 vai esbarrar na dívida registada.
  Não antecipar.
