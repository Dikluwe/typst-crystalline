# Passo 36 — Motor de equações: arquitectura base e MathIdent/MathText

**Pré-condições**:
- Passo 35 concluído: 430 L1 + 51 L3 + 50 parity, zero violations
- `DEBT-8` registado em `DEBT.md`
- Baseline de paridade em `00_nucleo/materialization/parity-baseline-passo-35.md`
- Branch: `cristalino/migration`

**Verificação antes de começar**:
```bash
# Confirmar placeholder [equação] ainda activo
grep -n "block.*\[.*\]\|equação.*placeholder\|\[.*plain_text" \
  01_core/src/rules/layout.rs | head -5

# Confirmar estrutura actual de Content::Equation e MathIdent
grep -n "Equation\|MathIdent\|MathText\|MathSequence" \
  01_core/src/entities/content.rs | head -15

# Confirmar que não existe módulo math em rules/
ls 01_core/src/rules/ 2>/dev/null

# Ver como layout.rs está estruturado actualmente
grep -n "pub struct Layouter\|pub fn layout\|fn layout_text\|fn layout_content" \
  01_core/src/rules/layout.rs | head -20
```

**Parar se qualquer pré-condição falhar.**

---

## Contexto

O Passo 36 estabelece a fronteira arquitectural entre o layout normal e
o layout matemático. O resultado observável é simples: `$x$` renderiza
`x` no PDF em vez de `[x]`. A estrutura que torna isso possível é o que
importa para os passos seguintes.

**Princípio**: o motor matemático é um módulo L1 separado
(`rules/math/`) que recebe `Content::Equation` e produz `Frame`s.
O layouter principal delega — não implementa matemática directamente.

**Âmbito deste passo**:
- Criar `01_core/src/rules/math/mod.rs` e `math/layout.rs`
- `MathLayouter` que processa `Content::Equation`
- Implementar `MathIdent` e `MathText` → `FrameItem::Text`
- Restantes variantes (`MathFrac`, `MathAttach`, `MathRoot`,
  `MathSequence`) → placeholder sem `[ ]`, apenas texto plano
- Remover o placeholder `[equação]` do layouter principal

**Não implementar neste passo**: espaçamento matemático, fontes OpenType
MATH, algoritmos de altura de linha para frações ou attachments.

---

## Tarefa 1 — Diagnóstico

```bash
# Ver como Layouter usa FontMetrics actualmente
grep -n "FontMetrics\|M: FontMetrics\|self\.metrics\|layout_text" \
  01_core/src/rules/layout.rs | head -20

# Ver FrameItem — que variantes existem
grep -n "pub enum FrameItem\|Text\|Image\|Shape" \
  01_core/src/entities/layout_types.rs | head -15

# Ver como FrameItem::Text é construído
grep -n "FrameItem::Text\|FrameItem::text" \
  01_core/src/rules/layout.rs | head -10

# Ver se TextStyle é usado directamente ou via referência
grep -n "TextStyle\|text_style\|\.style" \
  01_core/src/rules/layout.rs | head -15

# Ver como o layouter principal retorna Frame/PagedDocument
grep -n "pub fn layout\b\|PagedDocument\|Frame\|push.*frame" \
  01_core/src/rules/layout.rs | head -15
```

**Parar. Reportar output antes de qualquer código.**

Questões a responder:
1. `Layouter` é uma struct com `M: FontMetrics` genérico, ou usa trait
   objects?
2. `FrameItem::Text` — que campos tem? `(EcoString, TextStyle, Point)`
   ou outro?
3. O layouter principal tem um método `layout_text` reutilizável, ou a
   lógica está inline?
4. Como o layouter acumula itens num `Frame` — `Vec<FrameItem>` ou
   outro mecanismo?

---

## Tarefa 2 — Criar módulo `rules/math/`

Criar a estrutura de ficheiros:

```
01_core/src/rules/math/
├── mod.rs
└── layout.rs
```

### `rules/math/mod.rs`

```rust
// @layer: L1
// @updated: YYYY-MM-DD
// Motor de layout matemático.
// Recebe Content::Equation e produz Frame com FrameItems.
// Passo 36: MathIdent e MathText implementados.
// Passo 37+: MathFrac, MathAttach, MathRoot, espaçamento, fontes MATH.

pub mod layout;
pub use layout::MathLayouter;
```

### `rules/math/layout.rs`

```rust
// @layer: L1
// @updated: YYYY-MM-DD

use crate::entities::{
    content::Content,
    layout_types::{Frame, FrameItem, Point, TextStyle},
};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::rules::layout::FontMetrics;
use ecow::EcoString;

/// Motor de layout matemático.
///
/// Recebe `Content::Equation` e produz um `Frame` com os itens
/// tipográficos correspondentes.
///
/// Passo 36: apenas `MathIdent` e `MathText` produzem `FrameItem::Text`.
/// Restantes variantes produzem texto plano sem placeholder `[...]`.
pub struct MathLayouter<'a, M: FontMetrics> {
    metrics: &'a M,
    /// Posição actual de escrita dentro do frame de equação.
    cursor: Point,
    /// Items acumulados.
    items:  Vec<(Point, FrameItem)>,
}

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub fn new(metrics: &'a M) -> Self {
        Self {
            metrics,
            cursor: Point { x: 0.0.into(), y: 0.0.into() },
            items:  Vec::new(),
        }
    }

    /// Ponto de entrada: recebe o body de uma equação e produz um Frame.
    pub fn layout_equation(
        &mut self,
        body:  &Content,
        style: &TextStyle,
    ) -> SourceResult<Frame> {
        self.layout_math_content(body, style)?;
        Ok(self.build_frame())
    }

    /// Percorre a árvore de Content matemático recursivamente.
    fn layout_math_content(
        &mut self,
        content: &Content,
        style:   &TextStyle,
    ) -> SourceResult<()> {
        match content {
            Content::MathIdent(text) | Content::MathText(text) => {
                self.emit_text(text.clone(), style);
            }

            Content::MathSequence(nodes) => {
                for node in nodes.iter() {
                    self.layout_math_content(node, style)?;
                }
            }

            // Variantes não implementadas neste passo:
            // renderizar como texto plano, sem [ ].
            // Passo 37+ implementará layout tipográfico correcto.
            Content::MathFrac { num, den } => {
                self.emit_text(
                    EcoString::from(
                        format!("{}/{}", num.plain_text(), den.plain_text())
                    ),
                    style,
                );
            }

            Content::MathAttach { base, sub, sup } => {
                self.layout_math_content(base, style)?;
                if let Some(s) = sup {
                    self.emit_text(EcoString::from("^"), style);
                    self.layout_math_content(s, style)?;
                }
                if let Some(s) = sub {
                    self.emit_text(EcoString::from("_"), style);
                    self.layout_math_content(s, style)?;
                }
            }

            Content::MathRoot { index, radicand } => {
                match index {
                    None    => self.emit_text(EcoString::from("√"), style),
                    Some(i) => self.emit_text(
                        EcoString::from(format!("{}√", i.plain_text())),
                        style,
                    ),
                }
                self.layout_math_content(radicand, style)?;
            }

            // Content não-matemático dentro de uma equação (raro):
            // usar plain_text como fallback.
            other => {
                let text = other.plain_text();
                if !text.trim().is_empty() {
                    self.emit_text(EcoString::from(text), style);
                }
            }
        }
        Ok(())
    }

    /// Emite um FrameItem::Text na posição actual e avança o cursor.
    fn emit_text(&mut self, text: EcoString, style: &TextStyle) {
        if text.is_empty() { return; }
        let width   = self.metrics.char_width(' ', style.size); // largura média por char
        let advance = width * text.len() as f64;
        let pos = self.cursor;
        self.items.push((pos, FrameItem::Text {
            text,
            style: style.clone(),
        }));
        // Avançar cursor horizontalmente
        // Simplificado: largura uniforme por carácter
        // Passo 37+ usará métricas de glyph correctas
        self.cursor.x = (self.cursor.x.to_pt() + advance).into();
    }

    /// Constrói o Frame final com os items acumulados.
    fn build_frame(self) -> Frame {
        let width  = self.cursor.x;
        let height = self.metrics.line_height(12.0).into(); // simplificado
        Frame {
            size:  crate::entities::layout_types::Size { x: width, y: height },
            items: self.items,
        }
    }
}
```

**Nota sobre `FontMetrics::char_width`**: verificar se este método
existe na trait actual. Se não existir, adicionar:

```rust
pub trait FontMetrics: Send + Sync {
    // ... métodos existentes ...

    /// Largura aproximada de um carácter em pontos, para um dado tamanho.
    /// Passo 36: aproximação provisória — 0.6 × size, ignorando o glyph.
    /// Passo 37: substituir por lookup real nas tabelas de métricas da fonte.
    fn char_width(&self, _c: char, size: f64) -> f64 {
        size * 0.6
    }

    /// Altura de linha em pontos para um dado tamanho.
    fn line_height(&self, size: f64) -> f64 {
        size * 1.2
    }
}
```

Se a trait já tem estes métodos com outros nomes, usar os existentes
e adaptar. Reportar.

---

## Tarefa 3 — Integrar `MathLayouter` no layouter principal

Localizar o arm `Content::Equation` em `layout.rs` (actualmente com
placeholder `[...]`) e substituir pela delegação ao `MathLayouter`:

```rust
// ANTES
Content::Equation { body, block } => {
    let text = body.plain_text();
    if *block {
        self.layout_text(ctx, &format!("[{}]", text), style)
    } else {
        self.layout_text(ctx, &text, style)
    }
}

// DEPOIS
Content::Equation { body, block } => {
    let mut math_layouter = MathLayouter::new(self.metrics);
    let frame = math_layouter.layout_equation(body, style)?;

    if *block {
        // Equação display: frame numa linha própria
        self.push_frame_block(frame);
    } else {
        // Equação inline: frame integrado na linha actual
        self.push_frame_inline(frame);
    }
}
```

**Nota sobre `push_frame_block` e `push_frame_inline`**: verificar se
o layouter principal tem métodos para integrar um `Frame` filho. Se não,
simplificar: extrair os `FrameItem`s do frame matemático e adicioná-los
directamente ao frame principal com offset calculado.

```rust
// Alternativa se push_frame_* não existe:
Content::Equation { body, block } => {
    let mut math_layouter = MathLayouter::new(self.metrics);
    let math_frame = math_layouter.layout_equation(body, style)?;
    // Incorporar items do frame matemático no frame actual com offset
    let offset_x = self.cursor_x(); // posição actual
    let offset_y = self.cursor_y();
    for (pos, item) in math_frame.items {
        self.push_item(
            Point { x: (offset_x + pos.x.to_pt()).into(), y: offset_y.into() },
            item,
        );
    }
    if *block { self.newline(); }
}
```

**Parar. Mostrar o arm actual de `Content::Equation` em layout.rs
antes de alterar.**

---

## Tarefa 4 — Expor módulo em `rules/mod.rs`

```rust
// Em 01_core/src/rules/mod.rs
pub mod math;
```

Verificar que `crystalline-lint` não dispara V14 ou outras violations
com a nova estrutura de módulo.

---

## Tarefa 5 — Actualizar `DEBT.md`

```markdown
### DEBT-8 — Motor de equações não implementado — PARCIALMENTE RESOLVIDO

**Resolvido no Passo 36**:
- `MathLayouter` criado em `rules/math/layout.rs` (L1)
- `MathIdent` e `MathText` → `FrameItem::Text` (sem placeholder)
- `MathFrac`, `MathAttach`, `MathRoot` → texto plano (sem `[...]`)
- Delegação de `Content::Equation` do layouter principal ao `MathLayouter`

**Ainda pendente**:
- Espaçamento matemático correcto (kern entre símbolos)
- Fontes OpenType MATH (tabelas MATH, variantes de tamanho)
- Layout tipográfico de `MathFrac` (numerador sobre denominador)
- Layout tipográfico de `MathAttach` (sup elevado, sub baixado)
- Layout tipográfico de `MathRoot` (radical com extensão)
- `MathDelimited`, `MathPrimes`, `MathAlignPoint`, `MathShorthand`

**Passo seguinte**: Passo 37 — `MathFrac` e `MathAttach` com layout
tipográfico básico (posicionamento vertical).
```

---

## Tarefa 6 — Testes

```rust
// ── MathLayouter — testes directos em L1 ─────────────────────────────────

#[cfg(test)]
mod math_layout_tests {
    use super::*;
    use crate::rules::layout::FixedMetrics;

    #[test]
    fn math_layouter_math_ident_produz_frame_nao_vazio() {
        let mut ml = MathLayouter::new(&FixedMetrics);
        let style  = TextStyle::default();
        let frame  = ml.layout_equation(
            &Content::MathIdent("x".into()),
            &style,
        ).unwrap();
        assert!(!frame.items.is_empty(), "MathIdent deve produzir pelo menos 1 item");
    }

    #[test]
    fn math_layouter_math_text_produz_frame_nao_vazio() {
        let mut ml = MathLayouter::new(&FixedMetrics);
        let style  = TextStyle::default();
        let frame  = ml.layout_equation(
            &Content::MathText("sin".into()),
            &style,
        ).unwrap();
        assert!(!frame.items.is_empty());
    }

    #[test]
    fn math_layouter_sequence_produz_multiplos_items() {
        let mut ml = MathLayouter::new(&FixedMetrics);
        let style  = TextStyle::default();
        let seq = Content::MathSequence(
            vec![
                Content::MathIdent("x".into()),
                Content::MathText("+".into()),
                Content::MathIdent("y".into()),
            ].into()
        );
        let frame = ml.layout_equation(&seq, &style).unwrap();
        assert_eq!(frame.items.len(), 3, "x + y deve produzir 3 items");
    }

    #[test]
    fn math_layouter_frac_sem_placeholder_colchetes() {
        let mut ml = MathLayouter::new(&FixedMetrics);
        let style  = TextStyle::default();
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let frame = ml.layout_equation(&frac, &style).unwrap();
        // Verificar que nenhum FrameItem::Text contém "["
        for (_, item) in &frame.items {
            if let FrameItem::Text { text, .. } = item {
                assert!(!text.contains('['),
                    "frac não deve conter '[': {}", text);
            }
        }
    }
}

// ── pipeline: equação sem placeholder ────────────────────────────────────

#[test]
fn eval_e_layout_equation_sem_colchetes() {
    let world = MockWorld::new("$x$");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    // layout deve processar Content::Equation sem produzir "[x]"
    // Verificar ausência de "[" nos FrameItems
    // (adaptar conforme API de acesso ao Content/Frame)
    let _ = m;
}

// ── integração L3 ─────────────────────────────────────────────────────────
// Adicionar / actualizar em 03_infra/src/integration_tests.rs:

#[test]
fn pipeline_equacao_inline_sem_placeholder() {
    let (world, _dir) = world_from_str("$x + y$");
    let source = world.source(world.main()).unwrap();
    let module = eval(&world, &source).unwrap();
    let doc    = layout(module.content(), &FixedMetrics).unwrap();
    // O PDF deve conter texto "x" e "y" sem "[" 
    let pdf = export_pdf(&doc);
    assert!(!pdf.is_empty());
    assert_eq!(&pdf[..5], b"%PDF-");
    // Verificar ausência de "[" no stream de texto do PDF (opcional — se possível)
}

#[test]
fn pipeline_equacao_com_frac_sem_panic() {
    let (world, _dir) = world_from_str("$ frac(a, b) $");
    let source = world.source(world.main()).unwrap();
    let module = eval(&world, &source).unwrap();
    let doc    = layout(module.content(), &FixedMetrics).unwrap();
    let pdf    = export_pdf(&doc);
    assert!(!pdf.is_empty());
}
```

---

## Verificação final

```bash
cargo test -p typst-core 2>&1 | tail -5
cargo test -p typst-infra 2>&1 | tail -5
cargo test -p parity_runner 2>&1 | tail -3
cargo build 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found

# Confirmar módulo math em L1
ls 01_core/src/rules/math/

# Confirmar que placeholder [  ] foi removido
grep -n "\[.*plain_text\|\[.*equação\|format.*\[" \
  01_core/src/rules/layout.rs
# Deve retornar vazio

# Confirmar que SystemWorld não aparece em L1
grep -rn "SystemWorld" 01_core/src/ 2>/dev/null
# Deve retornar vazio

# Confirmar que MathLayouter está em L1
grep -n "std::fs\|std::net\|std::env" \
  01_core/src/rules/math/layout.rs
# Deve retornar vazio
```

Critérios de conclusão:
- `01_core/src/rules/math/mod.rs` e `math/layout.rs` criados, layer L1 ✓
- `MathLayouter` com `layout_equation()` implementado ✓
- `MathIdent` e `MathText` → `FrameItem::Text` (sem placeholder) ✓
- `MathFrac`, `MathAttach`, `MathRoot` → texto plano sem `[...]` ✓
- `Content::Equation` no layouter principal delega ao `MathLayouter` ✓
- Placeholder `[...]` removido de `layout.rs` ✓
- `math_layouter_math_ident_produz_frame_nao_vazio` passa ✓
- `math_layouter_sequence_produz_multiplos_items` passa ✓
- `math_layouter_frac_sem_placeholder_colchetes` passa ✓
- `pipeline_equacao_inline_sem_placeholder` passa ✓
- DEBT-8 marcado como parcialmente resolvido ✓
- Zero violations ✓
- Testes não regridem (430 L1 + 51 L3 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- `FontMetrics` já tinha `char_width` e `line_height`, ou foram adicionados?
- O layouter principal tinha `push_frame_block/inline` ou foi necessária
  a alternativa de extracção de items?
- `FrameItem::Text` — campos exactos confirmados.

**Da implementação:**
- Como `Content::Equation` inline vs block é diferenciado no frame
  final (posicionamento, newline)?
- Número final de testes e zero violations confirmado.

**DEBT-8 parcialmente resolvido. Go para Passo 37 — `MathFrac` e
`MathAttach` com layout tipográfico básico (posicionamento vertical).**
