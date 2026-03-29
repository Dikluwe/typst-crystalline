# Passo 23 — Content::Raw, listas e links

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/content.rs` — enum com Strong/Emph/Heading
- `01_core/src/entities/ast/markup.rs` — API dos nós de markup
- `DEBT.md` — inventário completo de dívida (Passos 24–32)

Pré-condição: `cargo test` — 351 testes (318 L1 + 33 L3), zero violations.

**Contexto de dívida**: este é o último passo incremental de Content
antes de começar o pagamento de dívida. Os Passos 24+ focam em
DEBT-5 (Unicode PDF), DEBT-4 (Value), e DEBT-3 (safety rails).
Não introduzir nova dívida arquitectural neste passo.

---

## Tarefa 1 — Diagnóstico de API da AST

```bash
# Tipos disponíveis em markup.rs
grep -n "^pub struct\|^pub enum" \
  01_core/src/entities/ast/markup.rs | head -40

# Raw — métodos text(), lang(), block()
grep -n "Raw\b" \
  01_core/src/entities/ast/markup.rs | head -10
grep -n "fn text\|fn lang\|fn block" \
  01_core/src/entities/ast/markup.rs | head -10

# Listas — ListItem, EnumItem
grep -n "ListItem\|EnumItem" \
  01_core/src/entities/ast/markup.rs | head -10
grep -n "fn body\|fn number" \
  01_core/src/entities/ast/markup.rs | head -10

# Links — SyntaxKind::Link e métodos
grep -n "Link\|Url\b" \
  01_core/src/entities/syntax_kind.rs \
  01_core/src/entities/ast/markup.rs | head -10
```

---

## Tarefa 2 — Novas variantes de Content

```rust
// Em 01_core/src/entities/content.rs

pub enum Content {
    // ... existentes ...

    Raw {
        text:  EcoString,
        lang:  Option<EcoString>,
        block: bool,
    },
    ListItem(Box<Content>),
    EnumItem { number: Option<u32>, body: Box<Content> },
    Link { url: EcoString, body: Box<Content> },
}

impl Content {
    pub fn raw(text: impl Into<EcoString>, lang: Option<EcoString>, block: bool) -> Self {
        Self::Raw { text: text.into(), lang, block }
    }
    pub fn list_item(body: Content) -> Self { Self::ListItem(Box::new(body)) }
    pub fn enum_item(number: Option<u32>, body: Content) -> Self {
        Self::EnumItem { number, body: Box::new(body) }
    }
    pub fn link(url: impl Into<EcoString>, body: Content) -> Self {
        Self::Link { url: url.into(), body: Box::new(body) }
    }
}
```

`plain_text()` para as novas variantes:
```rust
Self::Raw { text, .. }               => text.to_string(),
Self::ListItem(c)                    => format!("• {}", c.plain_text()),
Self::EnumItem { number, body }      => {
    let n = number.map(|n| format!("{}. ", n)).unwrap_or_default();
    format!("{}{}", n, body.plain_text())
}
Self::Link { body, .. }              => body.plain_text(),
```

---

## Tarefa 3 — eval_markup para novos nós

```rust
// Confirmar SyntaxKind exactos com o diagnóstico antes de compilar

SyntaxKind::Raw => {
    if let Some(raw) = ast::Raw::from_untyped(child) {
        parts.push(Content::raw(raw.text(), raw.lang().map(|l| l.into()), raw.block()));
    }
}
SyntaxKind::ListItem => {
    if let Some(item) = ast::ListItem::from_untyped(child) {
        let body = eval_markup_body(item.body(), scopes, ctx)?;
        parts.push(Content::list_item(body));
    }
}
SyntaxKind::EnumItem => {
    if let Some(item) = ast::EnumItem::from_untyped(child) {
        let number = item.number().map(|n| n as u32);
        let body   = eval_markup_body(item.body(), scopes, ctx)?;
        parts.push(Content::enum_item(number, body));
    }
}
SyntaxKind::Link => {
    if let Some(link) = ast::Link::from_untyped(child) {
        let url = link.url().to_string();
        parts.push(Content::link(url.clone(), Content::text(url)));
    }
}
```

---

## Tarefa 4 — Layouter para novos elementos

```rust
Content::Raw { text, block, .. } => {
    let prev = self.style;
    // Raw: tamanho 90%, sem bold/italic
    // DEBT: seleccionar fonte monospace real quando FontBook tiver uma
    self.style = TextStyle { bold: false, italic: false, size: self.font_size_pt * 0.9 };
    if *block {
        if self.cursor_x > MARGIN { self.flush_line(); }
        self.cursor_x = MARGIN + self.font_size_pt;
    }
    for word in text.split_whitespace() { self.layout_word(word); }
    if *block { self.flush_line(); }
    self.style = prev;
}

Content::ListItem(body) => {
    if self.cursor_x > MARGIN { self.flush_line(); }
    // Bullet: "•" é Unicode U+2022 — vai aparecer como ? no PDF até DEBT-5
    // usar "-" ASCII como fallback para o PDF actual
    self.current_line.push(FrameItem::Text {
        pos:   Point { x: MARGIN, y: self.cursor_y },
        text:  "-".into(),  // ASCII fallback — DEBT-5: substituir por "•" com CIDFont
        style: self.style,
    });
    self.cursor_x = MARGIN + self.font_size_pt * 1.5;
    self.layout_content(body);
    self.flush_line();
    self.cursor_x = MARGIN;
}

Content::EnumItem { number, body } => {
    if self.cursor_x > MARGIN { self.flush_line(); }
    let label: EcoString = match number {
        Some(n) => format!("{}.", n).into(),
        None    => "-".into(),
    };
    self.current_line.push(FrameItem::Text {
        pos:   Point { x: MARGIN, y: self.cursor_y },
        text:  label,
        style: self.style,
    });
    self.cursor_x = MARGIN + self.font_size_pt * 2.0;
    self.layout_content(body);
    self.flush_line();
    self.cursor_x = MARGIN;
}

Content::Link { url: _, body } => {
    // DEBT: sublinhado e cor de link — requer FrameItem::Decoration (futuro)
    self.layout_content(body);
}
```

**Nota sobre o bullet**: usar `-` ASCII em vez de `•` Unicode porque
o PDF actual (DEBT-5) não suporta caracteres além de Latin-1.
`plain_text()` pode continuar a usar `•` para representação textual —
o fallback é apenas no Layouter para o PDF.

---

## Tarefa 5 — Testes

```rust
// Content
#[test]
fn raw_plain_text() {
    assert_eq!(Content::raw("fn main() {}", None, false).plain_text(), "fn main() {}");
}

#[test]
fn list_item_tem_bullet_em_plain_text() {
    assert!(Content::list_item(Content::text("Apple")).plain_text().contains("Apple"));
}

#[test]
fn enum_item_com_numero() {
    let t = Content::enum_item(Some(1), Content::text("First")).plain_text();
    assert!(t.contains("1") && t.contains("First"));
}

#[test]
fn link_plain_text_e_o_corpo() {
    assert_eq!(Content::link("https://typst.app", Content::text("Typst")).plain_text(), "Typst");
}

// Layout
#[test]
fn layout_list_item_tem_dash() {
    let doc = layout(&Content::list_item(Content::text("Item")));
    let has_marker = doc.pages.iter()
        .flat_map(|p| p.items.iter())
        .any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "-"));
    assert!(has_marker, "ListItem deve ter marcador '-'");
}

#[test]
fn layout_raw_block_tamanho_menor() {
    let content = Content::Sequence(vec![
        Content::text("normal"),
        Content::raw("code", None, true),
    ]);
    let doc = layout(&content);
    let sizes: std::collections::HashSet<u64> = doc.pages.iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| match i {
            FrameItem::Text { style, .. } => Some(style.size.val().to_bits()),
            _ => None,
        })
        .collect();
    assert!(sizes.len() > 1, "Raw deve ter tamanho diferente do texto normal");
}

// Pipeline
#[test]
fn pipeline_raw_inline() {
    let world = MockWorld::new("Use `cargo build` to compile");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let text = module.content().unwrap().plain_text();
    assert!(text.contains("cargo") && text.contains("build"), "{:?}", text);
}

#[test]
fn pipeline_lista_bullets() {
    let world = MockWorld::new("- item 1\n- item 2");
    let src = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let text = module.content().unwrap().plain_text();
    assert!(text.contains("item 1") && text.contains("item 2"), "{:?}", text);
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
- `Content::Raw`, `ListItem`, `EnumItem`, `Link` no enum ✓
- `plain_text()` actualizado para todas ✓
- `eval_markup` processa `` `raw` ``, `- list`, `+ enum` ✓
- Bullet usa `-` ASCII no Layouter (compatível com PDF actual) ✓
- Comentário DEBT-5 no Layouter para o bullet Unicode ✓
- Zero violations ✓
- Testes não regridem (351 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:** API real de `ast::Raw`, `ListItem`, `EnumItem`, `Link`.

**Da implementação:**
- Se `plain_text()` usa `•` (unicode) ou `-` (ASCII) — notar a distinção
- Número total de testes e zero violations

**Go/No-Go para o Passo 24 — pagamento de DEBT-5:**
Com este passo concluído, o inventário de Content básico está completo.
O Passo 24 começa o pagamento de dívida com DEBT-5 (Unicode no PDF):
embedding de fonte TrueType e CIDFont/ToUnicode para eliminar os `?`.
Não há GO/NO-GO alternativo — a ordem de pagamento está definida.
