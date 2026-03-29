# Passo 19 — Frame, Page e layout() para PagedDocument

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0026-content-divergencia.md`
- `01_core/src/rules/layout.rs` — stub actual
- `01_core/src/entities/content.rs` — enum Content
- `lab/typst-original/crates/typst-layout/` — estrutura de referência

Pré-condição: `cargo test` — 312 testes (290 L1 + 22 L3), zero violations.

---

## Tarefa 1 — Diagnóstico de layout original

**Parar aqui. Reportar output antes de qualquer código.**

```bash
# Frame e FrameItem no original
grep -rn "^pub struct Frame\b" lab/typst-original/crates/typst-layout/src/ | head -3
grep -rA 20 "^pub struct Frame\b" lab/typst-original/crates/typst-layout/src/ | head -25
grep -rn "^pub enum FrameItem\b" lab/typst-original/crates/ | head -3
grep -rA 20 "^pub enum FrameItem\b" lab/typst-original/crates/ | head -25

# PagedDocument
grep -rn "^pub struct PagedDocument\b" lab/typst-original/crates/ | head -3
grep -rA 12 "^pub struct PagedDocument\b" lab/typst-original/crates/ | head -15

# Abs/Pt no original — newtype ou outro?
grep -rn "^pub struct Abs\b\|^pub struct Pt\b" \
  lab/typst-original/crates/typst-layout/src/ \
  lab/typst-original/crates/typst-library/src/ | head -10

# Como layout mede texto — métricas ou constante?
grep -rn "advance\|glyph_width\|metrics\|em_per_unit\|char_width" \
  lab/typst-original/crates/typst-layout/src/ | head -15
```

Questões a responder: estrutura de `Frame`, variantes de `FrameItem`
para texto, e se `Abs` no original é newtype f64 (confirmar unidade
— pt, px, ou unidades internas?).

---

## Tarefa 2 — `Pt` como newtype tipado

### Invariante de tipos: `Pt` não implementa `Add<f64>`

`Pt` opera apenas com outros `Pt`. Qualquer escalar bruto exige
conversão explícita `Pt(valor)`. Isto força o código de layout a
ser explícito sobre o que está a calcular — prevenindo o erro
clássico de somar uma largura a um índice de carácter.

```rust
// 01_core/src/entities/layout_types.rs (novo ficheiro)

/// Ponto tipográfico — unidade interna de layout.
/// 1 pt = 1/72 inch.
///
/// Não implementa Add<f64> — escalares brutos requerem Pt(valor) explícito.
/// Isto é intencional: evita misturar coordenadas com índices ou contagens.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pt(pub f64);

impl Pt {
    pub const ZERO: Self = Self(0.0);
    pub fn val(self) -> f64 { self.0 }
}

impl std::ops::Add for Pt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) }
}

impl std::ops::Sub for Pt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0) }
}

impl std::ops::Mul<f64> for Pt {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self { Self(self.0 * rhs) }
}

impl std::ops::AddAssign for Pt {
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

// Deliberadamente NÃO implementado:
// impl Add<f64> for Pt — escalares requerem Pt(valor) explícito

/// Posição 2D na página.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point { pub x: Pt, pub y: Pt }

impl Point {
    pub const ZERO: Self = Self { x: Pt::ZERO, y: Pt::ZERO };
}

/// Tamanho 2D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size { pub width: Pt, pub height: Pt }

impl Size {
    pub fn a4() -> Self {
        Self { width: Pt(595.0), height: Pt(842.0) }
    }
}
```

---

## Tarefa 3 — Frame, FrameItem e PagedDocument

```rust
// Continuação de layout_types.rs

use ecow::EcoString;  // ADR-0024

/// Item posicionado num frame.
#[derive(Debug, Clone)]
pub enum FrameItem {
    /// Texto posicionado.
    Text {
        pos:       Point,
        text:      EcoString,
        font_size: Pt,
    },
    // Variantes futuras:
    // Shape { pos: Point, geometry: Geometry },
    // Image { pos: Point, size: Size, data: Bytes },
}

/// Canvas de uma página — colecção de itens com posições absolutas.
#[derive(Debug, Clone)]
pub struct Frame {
    pub size:  Size,
    pub items: Vec<FrameItem>,
}

impl Frame {
    pub fn new(size: Size) -> Self {
        Self { size, items: Vec::new() }
    }

    pub fn push(&mut self, item: FrameItem) {
        self.items.push(item);
    }

    pub fn plain_text(&self) -> String {
        self.items.iter()
            .filter_map(|i| match i {
                FrameItem::Text { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Documento paginado — resultado de layout().
#[derive(Debug, Clone)]
pub struct PagedDocument {
    pub pages: Vec<Frame>,
}

impl PagedDocument {
    pub fn new(pages: Vec<Frame>) -> Self { Self { pages } }
    pub fn is_empty(&self) -> bool { self.pages.is_empty() }

    pub fn plain_text(&self) -> String {
        self.pages.iter()
            .map(|p| p.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
```

Adicionar a `entities/mod.rs`:
```rust
pub mod layout_types;
```

---

## Tarefa 4 — Layouter preparado para injecção de métricas

O `Layouter` usa `CHAR_WIDTH` como constante agora, mas a estrutura
é preparada para receber métricas reais por injecção no Passo 20.
A constante é isolada num trait `FontMetrics` — quando `FontBook`
real migrar, substituir `FixedMetrics` por `FontBookMetrics` sem
alterar o `Layouter`.

```rust
// 01_core/src/rules/layout.rs

use crate::entities::{
    content::Content,
    layout_types::{Frame, FrameItem, PagedDocument, Point, Pt, Size},
};

// ── Métricas de fonte ──────────────────────────────────────────────────────

/// Interface de métricas de fonte.
///
/// Implementação actual: FixedMetrics (monoespaçado).
/// Passo 20: FontBookMetrics com métricas reais (atracção para L3).
pub trait FontMetrics {
    fn char_width(&self, _c: char) -> Pt;
    fn line_height(&self) -> Pt;
    fn font_size(&self) -> Pt;
}

/// Métricas fixas monoespaçadas — para layout sem FontBook real.
/// Geometricamente correcto; visualmente "feio" (como esperado).
pub struct FixedMetrics {
    size: Pt,
}

impl FixedMetrics {
    pub fn new(font_size: f64) -> Self {
        Self { size: Pt(font_size) }
    }
}

impl FontMetrics for FixedMetrics {
    fn char_width(&self, _c: char) -> Pt { self.size * 0.6 }
    fn line_height(&self) -> Pt          { self.size * 1.2 }
    fn font_size(&self) -> Pt            { self.size }
}

// ── Constantes de página ───────────────────────────────────────────────────

const DEFAULT_FONT_SIZE: f64 = 12.0;
const MARGIN: Pt = Pt(72.0);  // 1 inch

// ── Layouter ──────────────────────────────────────────────────────────────

/// Máquina de estado de layout.
///
/// Consome Content e produz PagedDocument.
/// Preparado para receber FontMetrics real por injecção (Passo 20).
pub struct Layouter<M: FontMetrics> {
    metrics:      M,
    pages:        Vec<Frame>,
    current:      Frame,
    cursor_x:     Pt,
    cursor_y:     Pt,
    current_line: Vec<FrameItem>,  // acumulador antes de flush para Frame
}

impl<M: FontMetrics> Layouter<M> {
    pub fn new(metrics: M) -> Self {
        let page_size = Size::a4();
        let font_size = metrics.font_size();
        Self {
            metrics,
            pages:        Vec::new(),
            current:      Frame::new(page_size),
            cursor_x:     MARGIN,
            cursor_y:     MARGIN + font_size,
            current_line: Vec::new(),
        }
    }

    pub fn layout_content(&mut self, content: &Content) {
        match content {
            Content::Empty => {}

            Content::Text(text) => {
                for word in text.split_whitespace() {
                    self.layout_word(word);
                }
            }

            Content::Space => {
                self.cursor_x += self.metrics.char_width(' ');
                if self.cursor_x > Size::a4().width - MARGIN {
                    self.flush_line();
                }
            }

            Content::Sequence(parts) => {
                for part in parts {
                    self.layout_content(part);
                }
            }
        }
    }

    fn layout_word(&mut self, word: &str) {
        let word_width: Pt = word.chars()
            .map(|c| self.metrics.char_width(c))
            .fold(Pt::ZERO, |acc, w| acc + w);

        // Quebrar linha se a palavra não cabe
        if self.cursor_x + word_width > Size::a4().width - MARGIN
            && self.cursor_x > MARGIN
        {
            self.flush_line();
        }

        self.current_line.push(FrameItem::Text {
            pos:       Point { x: self.cursor_x, y: self.cursor_y },
            text:      word.into(),
            font_size: self.metrics.font_size(),
        });

        self.cursor_x += word_width + self.metrics.char_width(' ');
    }

    fn flush_line(&mut self) {
        // Mover items da linha actual para o frame
        for item in self.current_line.drain(..) {
            self.current.push(item);
        }
        // Avançar para a linha seguinte
        self.cursor_x = MARGIN;
        self.cursor_y += self.metrics.line_height();

        // Nova página se necessário
        if self.cursor_y > Size::a4().height - MARGIN {
            self.new_page();
        }
    }

    fn new_page(&mut self) {
        // Flush da linha actual antes de mudar de página
        for item in self.current_line.drain(..) {
            self.current.push(item);
        }
        let finished = std::mem::replace(
            &mut self.current,
            Frame::new(Size::a4()),
        );
        self.pages.push(finished);
        self.cursor_x = MARGIN;
        self.cursor_y = MARGIN + self.metrics.font_size();
    }

    pub fn finish(mut self) -> PagedDocument {
        // Flush final
        for item in self.current_line.drain(..) {
            self.current.push(item);
        }
        if !self.current.items.is_empty() {
            self.pages.push(self.current);
        }
        PagedDocument::new(self.pages)
    }
}

// ── API pública ────────────────────────────────────────────────────────────

/// Layout com métricas fixas monoespaçadas.
///
/// Geometricamente correcto (margens respeitadas, word-wrap).
/// Visualmente imperfeito — sem kerning ou métricas de fonte reais.
/// Passo 20: substituir FixedMetrics por FontBookMetrics.
pub fn layout(content: &Content) -> PagedDocument {
    let mut l = Layouter::new(FixedMetrics::new(DEFAULT_FONT_SIZE));
    l.layout_content(content);
    l.finish()
}
```

---

## Tarefa 5 — Testes

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{content::Content, layout_types::FrameItem};

    #[test]
    fn layout_texto_simples_tem_items() {
        let doc = layout(&Content::text("Hello world"));
        assert!(!doc.pages.is_empty());
        let total = doc.pages.iter().flat_map(|p| p.items.iter()).count();
        assert!(total >= 2, "Hello e world devem ser itens separados");
        assert!(doc.plain_text().contains("Hello"));
        assert!(doc.plain_text().contains("world"));
    }

    #[test]
    fn layout_documento_vazio_zero_paginas() {
        let doc = layout(&Content::Empty);
        assert_eq!(doc.pages.len(), 0, "documento vazio → sem páginas");
    }

    /// Teste de Ouro: itens dentro dos limites da página.
    /// Se a linha ultrapassar page_width, deve mover para a linha seguinte.
    #[test]
    fn layout_items_dentro_limites_da_pagina() {
        let words = (0..100)
            .map(|i| format!("palavra{i}"))
            .collect::<Vec<_>>()
            .join(" ");
        let doc = layout(&Content::text(&words));

        for page in &doc.pages {
            for item in &page.items {
                if let FrameItem::Text { pos, .. } = item {
                    assert!(
                        pos.x.val() >= 0.0 && pos.x.val() < 595.0,
                        "x={} fora dos limites da página", pos.x.val()
                    );
                    assert!(
                        pos.y.val() >= 0.0 && pos.y.val() < 842.0,
                        "y={} fora dos limites da página", pos.y.val()
                    );
                }
            }
        }
    }

    #[test]
    fn layout_texto_longo_word_wrap() {
        let words = (0..50)
            .map(|i| format!("w{i}"))
            .collect::<Vec<_>>()
            .join(" ");
        let doc = layout(&Content::text(&words));
        let items = doc.pages.iter().flat_map(|p| p.items.iter()).count();
        // 50 palavras → deve haver pelo menos 2 posições y distintas (múltiplas linhas)
        let y_values: std::collections::HashSet<u64> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { pos, .. } => Some(pos.y.val().to_bits()),
                _ => None,
            })
            .collect();
        assert!(y_values.len() > 1, "texto longo deve ter múltiplas linhas: {} items", items);
    }

    #[test]
    fn pt_tipagem_nao_permite_add_f64() {
        // Este teste compila apenas — verifica que a API não expõe Add<f64>
        // Se Pt::add(Pt, f64) existisse, o compilador não forçaria conversão
        let a = Pt(10.0);
        let b = Pt(5.0);
        let c = a + b;  // Add<Pt> — OK
        assert_eq!(c, Pt(15.0));
        // a + 5.0  ← isto NÃO deve compilar (sem impl Add<f64>)
        // Verificado pelo facto de o código não ter esse impl
    }

    // Pipeline end-to-end
    #[test]
    fn pipeline_parse_eval_layout() {
        use crate::{
            entities::{source::Source, scope::Scope},
            rules::eval::eval_for_test,
        };
        let world = crate::contracts::world::tests::MockWorld::new("Olá mundo");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        assert!(doc.plain_text().contains("Olá") || doc.plain_text().contains("mundo"),
            "texto deve estar no output: {:?}", doc.plain_text());
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
- `Pt + Pt` compila; `Pt + f64` não compila ✓
- `layout_items_dentro_limites_da_pagina` passa (word-wrap respeita margens) ✓
- `layout_texto_longo_word_wrap` — múltiplas linhas detectadas ✓
- Pipeline parse→eval→layout end-to-end sem crash ✓
- `FontMetrics` trait permite substituição por `FontBookMetrics` no Passo 20 ✓
- Zero violations ✓
- Testes não regridem (312 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- Estrutura de `Frame` e `FrameItem` no original — confirmar variantes
- Se `Abs` no original é newtype f64 (e qual unidade interna)
- Se `PagedDocument` tem campos adicionais além de `Vec<Frame>`

**Da implementação:**
- Se `current_line` foi necessário ou o flush directo para Frame bastou
- Se `flush_line` na transição de página funcionou correctamente
- Se `FontMetrics` trait compilou sem problemas de lifetimes

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 20:**
- **GO — export PDF esqueleto**: PagedDocument real; Passo 20 implementa
  `export_pdf()` como stub que serializa o texto dos frames num PDF mínimo
  válido (sem fontes embebidas — texto em PDF como strings literais)
- **GO — FontBookMetrics**: se o posicionamento monoespaçado é insuficiente
  para testes úteis; Passo 20 implementa `FontBookMetrics` ligando `FontBook`
  ao `Layouter` (atracção de layout() para L3)
- **GO — mais Content**: se Strong/Emph/Heading são urgentes; Passo 20
  adiciona variantes ao enum Content e ao Layouter
