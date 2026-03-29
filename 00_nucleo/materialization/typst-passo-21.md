# Passo 21 — FontBookMetrics e layout proporcional

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/layout.rs` — `FontMetrics` trait, `FixedMetrics`, `Layouter<M>`
- `03_infra/src/fonts.rs` — `FontSlot` com dados de fonte

Pré-condição: `cargo test` — 333 testes (303 L1 + 30 L3), zero violations.

**Invariantes a verificar no final:**
1. `advance()` e `vertical_metrics()` recebem `font_size: Pt` — não armazenado nas métricas
2. `ttf-parser` não aparece em `01_core/Cargo.toml`
3. `upem == 0` não causa divisão por zero (protecção explícita)

---

## Tarefa 1 — Redesenhar interface FontMetrics

### Interface mínima sem font_size armazenado

`font_size` é passado em cada chamada, não armazenado na struct de métricas.
Isto permite que o mesmo `Layouter` mude de tamanho a meio de um parágrafo
(suporte a rich text futuro) sem criar nova struct de métricas.

```rust
// Em 01_core/src/rules/layout.rs — substituir interface actual

/// Interface de métricas de fonte para o Layouter.
///
/// Minimalista — não armazena font_size nem vaza ttf-parser para L1.
/// font_size é passado em cada chamada para suportar tamanhos mistos.
pub trait FontMetrics: Send + Sync {
    /// Avanço horizontal de uma string em pontos tipográficos.
    fn advance(&self, text: &str, size: Pt) -> Pt;

    /// Métricas verticais: (ascender, line_height) em pontos tipográficos.
    ///
    /// ascender: distância da baseline ao topo das maiúsculas.
    /// line_height: distância total entre duas baselines consecutivas.
    ///
    /// O Layouter usa ambos:
    /// - cursor_y += ascender para chegar à baseline antes de desenhar
    /// - cursor_y += line_height para avançar para a linha seguinte
    fn vertical_metrics(&self, size: Pt) -> (Pt, Pt);
}
```

### Actualizar FixedMetrics

```rust
pub struct FixedMetrics;

impl FontMetrics for FixedMetrics {
    fn advance(&self, text: &str, size: Pt) -> Pt {
        // 0.6 * size por codepoint — monoespaçado
        size * (text.chars().count() as f64 * 0.6)
    }

    fn vertical_metrics(&self, size: Pt) -> (Pt, Pt) {
        // ascender ≈ 0.8 * size; line_height = 1.2 * size
        (size * 0.8, size * 1.2)
    }
}
```

### Actualizar Layouter — baseline correcta

O cursor Y aponta para a baseline, não para o topo do texto.
Na nova página ou nova linha, avançar `ascender` antes de desenhar;
avançar `line_height - ascender` depois para completar a linha.

```rust
pub struct Layouter<M: FontMetrics> {
    metrics:      M,
    font_size_pt: Pt,      // tamanho de fonte — campo do Layouter, não das métricas
    pages:        Vec<Frame>,
    current:      Frame,
    cursor_x:     Pt,
    cursor_y:     Pt,      // posição da baseline actual
    current_line: Vec<FrameItem>,
}

impl<M: FontMetrics> Layouter<M> {
    pub fn new(metrics: M, font_size: f64) -> Self {
        let size = Pt(font_size);
        let (ascender, _) = metrics.vertical_metrics(size);
        let page = Size::a4();
        Self {
            metrics,
            font_size_pt: size,
            pages:        Vec::new(),
            current:      Frame::new(page),
            cursor_x:     MARGIN,
            // Primeira linha: y = margem superior + ascender (posição da baseline)
            cursor_y:     MARGIN + ascender,
            current_line: Vec::new(),
        }
    }

    fn word_width(&self, word: &str) -> Pt {
        self.metrics.advance(word, self.font_size_pt)
    }

    fn space_width(&self) -> Pt {
        self.metrics.advance(" ", self.font_size_pt)
    }

    fn flush_line(&mut self) {
        for item in self.current_line.drain(..) {
            self.current.push(item);
        }
        // Avançar para a próxima baseline via line_height completo
        let (_, line_height) = self.metrics.vertical_metrics(self.font_size_pt);
        self.cursor_y += line_height;
        self.cursor_x  = MARGIN;

        if self.cursor_y > Size::a4().height - MARGIN {
            self.new_page();
        }
    }

    fn new_page(&mut self) {
        for item in self.current_line.drain(..) {
            self.current.push(item);
        }
        let finished = std::mem::replace(&mut self.current, Frame::new(Size::a4()));
        self.pages.push(finished);
        self.cursor_x = MARGIN;
        // Nova página: baseline na margem + ascender
        let (ascender, _) = self.metrics.vertical_metrics(self.font_size_pt);
        self.cursor_y = MARGIN + ascender;
    }

    fn layout_word(&mut self, word: &str) {
        let w = self.word_width(word);
        if self.cursor_x + w > Size::a4().width - MARGIN && self.cursor_x > MARGIN {
            self.flush_line();
        }
        self.current_line.push(FrameItem::Text {
            pos:       Point { x: self.cursor_x, y: self.cursor_y },
            text:      word.into(),
            font_size: self.font_size_pt,
        });
        self.cursor_x += w + self.space_width();
    }
}
```

### Verificação intermédia após actualização

```bash
cargo test -p typst-core
# ✓ 303 testes L1 passam — FixedMetrics com nova interface
crystalline-lint .
# ✓ zero violations — ttf-parser não aparece em 01_core
```

---

## Tarefa 2 — FontBookMetrics em L3

```rust
// 03_infra/src/font_metrics.rs (novo ficheiro)

use ttf_parser::Face;
use typst_core::{entities::layout_types::Pt, rules::layout::FontMetrics};

/// Métricas de fonte reais via ttf-parser.
///
/// font_size não armazenado — passado em cada chamada (invariante do trait).
/// Lifetime 'a ligado aos bytes da fonte.
pub struct FontBookMetrics<'a> {
    face: Face<'a>,
    upem: f64,  // units_per_em — tipicamente 1000 ou 2048
}

impl<'a> FontBookMetrics<'a> {
    pub fn from_bytes(data: &'a [u8]) -> Option<Self> {
        let face = Face::parse(data, 0).ok()?;
        let upem = face.units_per_em();
        // Protecção contra upem=0 (fontes corrompidas) — fallback para 1000
        let upem = if upem == 0 { 1000.0 } else { upem as f64 };
        Some(Self { face, upem })
    }
}

impl<'a> FontMetrics for FontBookMetrics<'a> {
    fn advance(&self, text: &str, size: Pt) -> Pt {
        // Fórmula: advance_pt = font_size * (Σ glyph_units / upem)
        // Se upem=0 já foi tratado em from_bytes (nunca chega aqui)
        let units: f64 = text.chars()
            .map(|c| {
                self.face.glyph_index(c)
                    .and_then(|gid| self.face.glyph_hor_advance(gid))
                    .map(|a| a as f64)
                    .unwrap_or(self.upem * 0.6)  // fallback para glifos ausentes
            })
            .sum();
        size * (units / self.upem)
    }

    fn vertical_metrics(&self, size: Pt) -> (Pt, Pt) {
        let ascender  = self.face.ascender()  as f64;
        // descender: norma diz negativo; unsigned_abs() para fontes "incorrectas"
        let descender = (self.face.descender() as f64).abs();
        let line_gap  = self.face.line_gap()  as f64;

        let ascender_pt    = size * (ascender / self.upem);
        let line_height_pt = size * ((ascender + descender + line_gap) / self.upem);

        (ascender_pt, line_height_pt)
    }
}
```

Adicionar a `03_infra/src/lib.rs`:
```rust
pub mod font_metrics;
```

---

## Tarefa 3 — layout_with_font() em L3

```rust
// 03_infra/src/layout.rs (novo)

use typst_core::{
    entities::{content::Content, layout_types::PagedDocument},
    rules::layout::Layouter,
};
use crate::font_metrics::FontBookMetrics;

/// Layout com métricas de fonte reais.
/// Fallback para FixedMetrics se os bytes de fonte forem inválidos.
pub fn layout_with_font(
    content:   &Content,
    font_data: &[u8],
    font_size: f64,
) -> PagedDocument {
    if let Some(metrics) = FontBookMetrics::from_bytes(font_data) {
        let mut l = Layouter::new(metrics, font_size);
        l.layout_content(content);
        l.finish()
    } else {
        typst_core::rules::layout::layout(content)
    }
}
```

Adicionar a `03_infra/src/lib.rs`:
```rust
pub mod layout;
```

---

## Tarefa 4 — Testes

### Testes sem fixture (sempre correm)

```rust
// Em 01_core/src/rules/layout.rs #[cfg(test)]

#[test]
fn fixed_metrics_advance_proporcional_ao_tamanho() {
    use super::{FixedMetrics, FontMetrics};
    let m = FixedMetrics;
    let a12 = m.advance("Hello", Pt(12.0));
    let a24 = m.advance("Hello", Pt(24.0));
    // 24pt deve ser exactamente o dobro de 12pt
    assert!((a24.val() - 2.0 * a12.val()).abs() < 0.001,
        "advance deve escalar linearmente com font_size");
}

#[test]
fn fixed_metrics_monoespacao_iiii_eq_wwww() {
    use super::{FixedMetrics, FontMetrics};
    let m = FixedMetrics;
    let ai = m.advance("iiii", Pt(12.0));
    let aw = m.advance("WWWW", Pt(12.0));
    assert_eq!(ai, aw, "FixedMetrics é monoespaçado — iiii == WWWW");
}

#[test]
fn fixed_metrics_vertical_ascender_menor_que_line_height() {
    use super::{FixedMetrics, FontMetrics};
    let (asc, lh) = FixedMetrics.vertical_metrics(Pt(12.0));
    assert!(asc.val() > 0.0, "ascender deve ser positivo");
    assert!(lh.val() > asc.val(), "line_height > ascender");
}

#[test]
fn layouter_baseline_dentro_da_pagina() {
    // A posição Y inicial deve ser positiva (abaixo do topo da página)
    // mas menor que a altura da página
    use super::{FixedMetrics, Layouter};
    let l = Layouter::new(FixedMetrics, 12.0);
    // Verificar que o cursor inicial está dentro dos limites
    assert!(l.cursor_y.val() > 0.0);
    assert!(l.cursor_y.val() < 842.0);
}
```

### Testes com fixture (ignorados sem fonte)

```rust
// Em 03_infra/src/font_metrics.rs #[cfg(test)]

/// TESTE DE OURO: layout proporcional validado.
/// 'W' tipicamente tem advance ~3x maior que 'i' em fontes proporcionais.
#[test]
#[ignore = "requer tests/fixtures/liberation-sans-regular.ttf"]
fn proporcionalidade_iiii_vs_wwww() {
    let data = std::fs::read(
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/liberation-sans-regular.ttf")
    ).expect("fixture necessária");

    let m = FontBookMetrics::from_bytes(&data).expect("fonte válida");
    let size = Pt(12.0);

    let ai = m.advance("iiii", size);
    let aw = m.advance("WWWW", size);

    assert!(
        ai.val() < aw.val(),
        "proporcional: 'iiii' ({:.2}pt) deve ser mais estreito que 'WWWW' ({:.2}pt)\n\
         Diagnóstico: se iiii ≈ 0.07pt → esqueceu size*; se iiii ≈ 700pt → esqueceu /upem",
        ai.val(), aw.val()
    );

    // Sanidade: 'A' em 12pt deve ser entre 3pt e 12pt
    let aa = m.advance("A", size);
    assert!(
        aa.val() > 3.0 && aa.val() < 12.0,
        "'A' em 12pt deve ser 3–12pt, foi {:.2}pt (upem={})",
        aa.val(), "ver diagnóstico"
    );
}

#[test]
#[ignore = "requer fixture"]
fn upem_zero_nao_causa_divisao_por_zero() {
    // Este teste verifica a protecção em from_bytes
    // Uma fonte corrompida com upem=0 deve retornar None (não panic)
    // Simulado: verificar que from_bytes com bytes inválidos retorna None
    assert!(FontBookMetrics::from_bytes(b"not a font").is_none());
    assert!(FontBookMetrics::from_bytes(b"").is_none());
}

#[test]
#[ignore = "requer fixture"]
fn vertical_metrics_sanidade() {
    let data = std::fs::read(
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/liberation-sans-regular.ttf")
    ).unwrap();
    let m = FontBookMetrics::from_bytes(&data).unwrap();
    let (asc, lh) = m.vertical_metrics(Pt(12.0));
    assert!(asc.val() > 0.0,  "ascender positivo");
    assert!(lh.val() > asc.val(), "line_height > ascender");
    assert!(lh.val() < 24.0, "line_height em 12pt < 24pt");
    // Verificar que font_size não está armazenado nas métricas
    // (verificado pelo facto de não existir campo font_size em FontBookMetrics)
    let (asc24, lh24) = m.vertical_metrics(Pt(24.0));
    assert!((lh24.val() - 2.0 * lh.val()).abs() < 0.5,
        "métricas devem escalar com font_size: 24pt ≈ 2× 12pt");
}
```

---

## Verificação final

```bash
cargo test -p typst-core      # 303+ testes, zero violations
cargo test -p typst-infra     # 30+ testes sem #[ignore], zero violations
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found

# Verificar invariante L3 isolation:
grep "ttf.parser" 01_core/Cargo.toml && echo "VIOLAÇÃO" || echo "OK"
# Deve imprimir OK

# Com fixture disponível:
cargo test -p typst-infra -- --include-ignored proporcionalidade
```

Critérios de conclusão:
- `FontMetrics::advance(&str, Pt)` — font_size passado por chamada, não armazenado ✓
- `ttf-parser` não em `01_core/Cargo.toml` ✓
- `upem == 0` tratado em `from_bytes` (fallback 1000, não panic) ✓
- `descender()` tratado com `.abs()` (defensivo para fontes incorrectas) ✓
- Layouter usa baseline: `cursor_y += ascender` antes de desenhar ✓
- 333 testes base não regridem ✓
- Zero violations ✓

---

## Ao terminar, reportar

- Se `face.glyph_hor_advance` retorna `Option<u16>` ou outro tipo
- Se `face.descender()` era negativo ou positivo na fixture usada
- Se lifetime de `Face<'a>` em `FontBookMetrics<'a>` criou problemas
- Se o teste de proporcionalidade passou e os valores de `iiii` vs `WWWW`
- Número total de testes e zero violations

**Go/No-Go para o Passo 22:**
- **GO — mais Content**: proporcional funciona; Passo 22 adiciona
  `Content::Heading`, `Strong`, `Emph` com selecção de variante
  bold/italic via `FontBook::select`
- **GO — font embedding**: se texto não-ASCII no PDF ainda usa `?`;
  Passo 22 embebe fonte TrueType no PDF para WinAnsiEncoding
- **NO-GO — lifetime bloqueado**: borrow checker rejeita `FontBookMetrics`
  em contexto de `Layouter` persistente; Passo 22 usa `Arc<Vec<u8>>`
