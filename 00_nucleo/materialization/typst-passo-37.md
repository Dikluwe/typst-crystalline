# Passo 37 — Motor de equações: MathFrac e MathAttach com posicionamento vertical

**Pré-condições**:
- Passo 36 concluído: 437 L1 + 53 L3 + 50 parity, zero violations
- `rules/math/layout.rs` existe com `MathLayouter`
- `FrameItem::Text { pos: Point, text: EcoString, style: TextStyle }` — pos embutida no item
- `FontMetrics` tem `advance(text, size)` e `vertical_metrics(size)`
- Branch: `cristalino/migration`

**Verificação antes de começar**:
```bash
# Confirmar estrutura actual de MathLayouter
grep -n "pub struct MathLayouter\|fn layout_equation\|fn layout_node\|fn emit_text" \
  01_core/src/rules/math/layout.rs

# Confirmar assinatura exacta de vertical_metrics
grep -n "fn vertical_metrics\|fn advance" \
  01_core/src/rules/layout.rs \
  01_core/src/entities/layout_types.rs 2>/dev/null | head -10

# Confirmar o que vertical_metrics retorna
grep -n -A 5 "fn vertical_metrics" \
  01_core/src/rules/layout.rs 2>/dev/null | head -15

# Confirmar FrameItem::Text com pos embutida
grep -n "FrameItem\|pos.*Point\|text.*Eco\|style.*Text" \
  01_core/src/entities/layout_types.rs | head -15

# Confirmar como MathLayouter calcula pos actualmente para MathIdent/MathText
grep -n "pos\|Point\|cursor\|x_relativo\|advance" \
  01_core/src/rules/math/layout.rs | head -20
```

**Parar. Reportar output completo antes de qualquer código.**

Questões a responder:
1. `vertical_metrics(size)` — que struct/tupla retorna? Tem `ascender`,
   `descender`, `x_height`? Com que tipos (`f64`, `Pt`, `Abs`)?
2. `advance(text, size)` — retorna `f64`, `Pt`, ou `Abs`?
3. Como `MathLayouter` atribui `pos` ao `FrameItem::Text` actualmente —
   acumula `x` e usa `y` fixo, ou tem outra estrutura?
4. `MathFrac` e `MathAttach` no Passo 36 caem no arm `_` que retorna
   `0.0` — confirmar.

---

## Contexto

O Passo 36 implementou `MathIdent` e `MathText` com posicionamento
horizontal simples. `MathFrac` e `MathAttach` caem no arm `_` e não
produzem output visível.

Este passo implementa posicionamento vertical básico para:
- **`MathFrac`**: numerador centrado acima da linha de fracção;
  denominador centrado abaixo.
- **`MathAttach`**: base na baseline; sup elevado a `ascender × 0.5`;
  sub baixado a `descender × 0.5`.

Não é tipografia matemática completa — é posicionamento relativo
suficiente para tornar as equações legíveis. Fontes OpenType MATH
e kern matemático ficam para passos posteriores.

---

## Tarefa 2 — Estrutura de métricas verticais para math

Antes de implementar, definir como o `MathLayouter` representa a
posição de cada item relativamente à baseline da equação.

Criar em `rules/math/layout.rs` uma struct interna de métricas:

```rust
/// Caixa tipográfica de um nó matemático.
/// Todas as medidas são em pontos, relativas à baseline da equação.
/// `ascent` > 0 (acima da baseline), `descent` > 0 (abaixo da baseline).
#[derive(Debug, Clone)]
struct MathBox {
    width:   f64,  // largura horizontal
    ascent:  f64,  // altura acima da baseline
    descent: f64,  // profundidade abaixo da baseline
    /// Items a adicionar ao Frame, com posições relativas a este MathBox.
    /// Posição (0, 0) = canto superior esquerdo do MathBox.
    items: Vec<FrameItem>,
}

impl MathBox {
    fn height(&self) -> f64 {
        self.ascent + self.descent
    }

    /// Converte para FrameItems com posição absoluta, dado o offset
    /// (x_origin, baseline_y) no frame pai.
    fn place(self, x_origin: f64, baseline_y: f64) -> Vec<FrameItem> {
        self.items.into_iter().map(|mut item| {
            if let FrameItem::Text { ref mut pos, .. } = item {
                pos.x = (pos.x.to_pt() + x_origin).into();
                // y no FrameItem é medido a partir do topo da página (para baixo)
                // baseline_y é a posição da baseline no frame pai
                // pos.y dentro do MathBox é relativo ao topo do box (ascent)
                pos.y = (baseline_y - self.ascent + pos.y.to_pt()).into();
            }
            item
        }).collect()
    }
}
```

**Nota sobre coordenadas**: verificar no diagnóstico se `y` cresce para
baixo (PDF standard) ou para cima. Ajustar `place()` conforme necessário.

---

## Tarefa 3 — Refactorizar `MathLayouter` para usar `MathBox`

Mudar a assinatura interna de `layout_node` para retornar `MathBox`
em vez de `f64`:

```rust
impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub fn layout_equation(
        &self,
        body:  &Content,
        style: &TextStyle,
    ) -> Vec<FrameItem> {
        let math_box = self.layout_node(body, style);
        // Baseline no centro da caixa — simplificado para Passo 37
        // Passo 38+ usará x-height da fonte para posicionar baseline correctamente
        let baseline_y = math_box.ascent;
        math_box.place(0.0, baseline_y)
    }

    fn layout_node(&self, content: &Content, style: &TextStyle) -> MathBox {
        match content {
            Content::MathIdent(text) | Content::MathText(text) => {
                self.layout_text_node(text, style)
            }
            Content::MathSequence(nodes) => {
                self.layout_sequence(nodes, style)
            }
            Content::MathFrac { num, den } => {
                self.layout_frac(num, den, style)
            }
            Content::MathAttach { base, sub, sup } => {
                self.layout_attach(base, sub.as_deref(), sup.as_deref(), style)
            }
            Content::MathRoot { index, radicand } => {
                // Passo 37: placeholder sem crash
                // Passo 38+: radical com extensão
                let inner = self.layout_node(radicand, style);
                let prefix = self.layout_text_node(&"√".into(), style);
                self.hconcat(vec![prefix, inner])
            }
            other => {
                // Fallback: texto plano
                let text: EcoString = other.plain_text().into();
                if text.trim().is_empty() {
                    MathBox { width: 0.0, ascent: 0.0, descent: 0.0, items: vec![] }
                } else {
                    self.layout_text_node(&text, style)
                }
            }
        }
    }

    fn layout_text_node(&self, text: &EcoString, style: &TextStyle) -> MathBox {
        if text.is_empty() {
            return MathBox { width: 0.0, ascent: 0.0, descent: 0.0, items: vec![] };
        }
        let width   = self.metrics.advance(text, style.size);
        let vm      = self.metrics.vertical_metrics(style.size);
        // Adaptar conforme o que vertical_metrics retorna — ver diagnóstico
        let ascent  = vm.ascender;   // ou vm.ascent, vm.0, etc.
        let descent = vm.descender;  // ou vm.descent, vm.1, etc.

        MathBox {
            width,
            ascent,
            descent,
            items: vec![FrameItem::Text {
                pos:   Point { x: 0.0.into(), y: 0.0.into() }, // relativo ao MathBox
                text:  text.clone(),
                style: style.clone(),
            }],
        }
    }

    fn layout_sequence(&self, nodes: &[Content], style: &TextStyle) -> MathBox {
        let boxes: Vec<MathBox> = nodes.iter()
            .map(|n| self.layout_node(n, style))
            .collect();
        self.hconcat(boxes)
    }

    /// Concatenação horizontal: posiciona MathBoxes lado a lado.
    fn hconcat(&self, boxes: Vec<MathBox>) -> MathBox {
        let mut x      = 0.0;
        let mut ascent  = 0.0f64;
        let mut descent = 0.0f64;
        let mut items   = Vec::new();

        for b in boxes {
            ascent  = ascent.max(b.ascent);
            descent = descent.max(b.descent);
            // Posicionar items do box com offset x
            for mut item in b.items {
                if let FrameItem::Text { ref mut pos, .. } = item {
                    pos.x = (pos.x.to_pt() + x).into();
                }
                items.push(item);
            }
            x += b.width;
        }

        MathBox { width: x, ascent, descent, items }
    }
}
```

---

## Tarefa 4 — Implementar `layout_frac`

```rust
fn layout_frac(
    &self,
    num:   &Content,
    den:   &Content,
    style: &TextStyle,
) -> MathBox {
    // Estilo reduzido para numerador e denominador (tamanho 70%)
    let sub_style = TextStyle { size: style.size * 0.7, ..*style };

    let num_box = self.layout_node(num, &sub_style);
    let den_box = self.layout_node(den, &sub_style);

    // Largura da fracção = máximo entre numerador e denominador
    let width = num_box.width.max(den_box.width);

    // Espessura da linha de fracção (simplificado)
    let rule_thickness = style.size * 0.05;
    // Gap entre numerador/denominador e a linha
    let gap = style.size * 0.1;

    // Altura total:
    // ascent = num_box.height() + gap + rule_thickness/2
    // descent = den_box.height() + gap + rule_thickness/2
    let ascent  = num_box.height() + gap + rule_thickness / 2.0;
    let descent = den_box.height() + gap + rule_thickness / 2.0;

    // Centrar numerador horizontalmente
    let num_x = (width - num_box.width) / 2.0;
    // Numerador: y=0 é o topo do MathBox; numerador fica no topo
    let num_y = 0.0;

    // Denominador: abaixo da linha de fracção
    let den_x = (width - den_box.width) / 2.0;
    let den_y = num_box.height() + gap + rule_thickness + gap;

    let mut items = Vec::new();

    // Items do numerador com offset
    for mut item in num_box.items {
        if let FrameItem::Text { ref mut pos, .. } = item {
            pos.x = (pos.x.to_pt() + num_x).into();
            pos.y = (pos.y.to_pt() + num_y).into();
        }
        items.push(item);
    }

    // TODO Passo 38: linha de fracção como FrameItem::Line ou rect
    // Por agora, linha omitida — só numerador e denominador posicionados

    // Items do denominador com offset
    for mut item in den_box.items {
        if let FrameItem::Text { ref mut pos, .. } = item {
            pos.x = (pos.x.to_pt() + den_x).into();
            pos.y = (pos.y.to_pt() + den_y).into();
        }
        items.push(item);
    }

    MathBox { width, ascent, descent, items }
}
```

---

## Tarefa 5 — Implementar `layout_attach`

```rust
fn layout_attach(
    &self,
    base: &Content,
    sub:  Option<&Content>,
    sup:  Option<&Content>,
    style: &TextStyle,
) -> MathBox {
    let base_box  = self.layout_node(base, style);

    // Estilo reduzido para sub/sup (tamanho 65%)
    let script_style = TextStyle { size: style.size * 0.65, ..*style };

    // Deslocamento vertical: sup sobe, sub desce
    // Aproximação: sup a 50% do ascender, sub a 30% do descender
    let vm         = self.metrics.vertical_metrics(style.size);
    let sup_offset = vm.ascender * 0.5;   // adaptar ao tipo real
    let sub_offset = vm.descender * 0.3;  // adaptar ao tipo real

    let mut x      = base_box.width;
    let mut ascent  = base_box.ascent;
    let mut descent = base_box.descent;
    let mut items   = base_box.items;

    if let Some(sup_content) = sup {
        let sup_box = self.layout_node(sup_content, &script_style);
        ascent = ascent.max(sup_offset + sup_box.ascent);
        for mut item in sup_box.items {
            if let FrameItem::Text { ref mut pos, .. } = item {
                pos.x = (pos.x.to_pt() + x).into();
                // sup elevado: y negativo (acima da baseline)
                // Ajustar sinal conforme convenção de coordenadas
                pos.y = (pos.y.to_pt() - sup_offset).into();
            }
            items.push(item);
        }
        x += sup_box.width;
    }

    if let Some(sub_content) = sub {
        let sub_box = self.layout_node(sub_content, &script_style);
        descent = descent.max(sub_offset + sub_box.descent);
        for mut item in sub_box.items {
            if let FrameItem::Text { ref mut pos, .. } = item {
                pos.x = (pos.x.to_pt() + x).into();
                pos.y = (pos.y.to_pt() + sub_offset).into();
            }
            items.push(item);
        }
    }

    MathBox { width: x, ascent, descent, items }
}
```

---

## Tarefa 6 — Integrar com o layouter principal

O Passo 36 implementou a integração. Verificar se `layout_equation`
ainda retorna `Vec<FrameItem>` ou se mudou para `Frame`. Adaptar
a chamada em `layout.rs` se a assinatura mudou.

```bash
# Confirmar assinatura actual de layout_equation após Passo 36
grep -n "pub fn layout_equation" \
  01_core/src/rules/math/layout.rs
```

---

## Tarefa 7 — Actualizar `DEBT.md`

```markdown
### DEBT-8 — Motor de equações — PARCIALMENTE RESOLVIDO

**Resolvido no Passo 37**:
- `MathBox` como unidade de layout matemático (width, ascent, descent)
- `MathFrac`: numerador acima, denominador abaixo, largura máxima,
  tamanho 70% do texto base
- `MathAttach`: sup elevado a 50% do ascender, sub baixado a 30%
  do descender, tamanho 65% do texto base
- `hconcat`: concatenação horizontal de MathBoxes

**Ainda pendente**:
- Linha de fracção (`FrameItem::Line` ou rect) — Passo 38
- Kern matemático entre símbolos
- Fontes OpenType MATH (tabelas MATH, variantes de tamanho)
- `MathDelimited`, `MathPrimes`, `MathShorthand`, `MathAlignPoint`
- Baseline correcta em relação ao x-height da fonte
```

---

## Tarefa 8 — Testes

```rust
// ── MathBox e layout_frac ────────────────────────────────────────────────

#[test]
fn math_frac_tem_ascent_e_descent_nao_zero() {
    let metrics  = FixedMetrics;
    let layouter = MathLayouter::new(&metrics);
    let style    = TextStyle { bold: false, italic: false, size: 10.0 };

    let frac = Content::MathFrac {
        num: Box::new(Content::MathIdent("a".into())),
        den: Box::new(Content::MathIdent("b".into())),
    };

    let items = layouter.layout_equation(&frac, &style);
    // Fracção deve produzir pelo menos 2 items (num e den)
    assert!(items.len() >= 2, "frac deve ter >= 2 items, tem {}", items.len());
}

#[test]
fn math_frac_numerador_acima_denominador() {
    let metrics  = FixedMetrics;
    let layouter = MathLayouter::new(&metrics);
    let style    = TextStyle { bold: false, italic: false, size: 10.0 };

    let frac = Content::MathFrac {
        num: Box::new(Content::MathIdent("a".into())),
        den: Box::new(Content::MathIdent("b".into())),
    };

    let items = layouter.layout_equation(&frac, &style);
    // O primeiro item (numerador) deve ter y menor que o segundo (denominador)
    // (y cresce para baixo em coordenadas PDF)
    let ys: Vec<f64> = items.iter().filter_map(|item| {
        if let FrameItem::Text { pos, .. } = item {
            Some(pos.y.to_pt())
        } else { None }
    }).collect();

    assert!(ys.len() >= 2, "deve ter pelo menos 2 posições y");
    assert!(ys[0] < ys[1],
        "numerador (y={}) deve estar acima do denominador (y={})", ys[0], ys[1]);
}

#[test]
fn math_attach_sup_elevado() {
    let metrics  = FixedMetrics;
    let layouter = MathLayouter::new(&metrics);
    let style    = TextStyle { bold: false, italic: false, size: 10.0 };

    let attach = Content::MathAttach {
        base: Box::new(Content::MathIdent("x".into())),
        sub:  None,
        sup:  Some(Box::new(Content::MathIdent("2".into()))),
    };

    let items = layouter.layout_equation(&attach, &style);
    assert!(items.len() >= 2, "x^2 deve ter >= 2 items");

    let ys: Vec<f64> = items.iter().filter_map(|item| {
        if let FrameItem::Text { pos, .. } = item { Some(pos.y.to_pt()) }
        else { None }
    }).collect();

    // sup deve estar acima da base (y menor)
    assert!(ys[1] < ys[0],
        "sup (y={}) deve estar acima da base (y={})", ys[1], ys[0]);
}

#[test]
fn math_attach_sub_baixado() {
    let metrics  = FixedMetrics;
    let layouter = MathLayouter::new(&metrics);
    let style    = TextStyle { bold: false, italic: false, size: 10.0 };

    let attach = Content::MathAttach {
        base: Box::new(Content::MathIdent("x".into())),
        sub:  Some(Box::new(Content::MathIdent("i".into()))),
        sup:  None,
    };

    let items = layouter.layout_equation(&attach, &style);
    assert!(items.len() >= 2);

    let ys: Vec<f64> = items.iter().filter_map(|item| {
        if let FrameItem::Text { pos, .. } = item { Some(pos.y.to_pt()) }
        else { None }
    }).collect();

    // sub deve estar abaixo da base (y maior)
    assert!(ys[1] > ys[0],
        "sub (y={}) deve estar abaixo da base (y={})", ys[1], ys[0]);
}

// ── integração L3 ─────────────────────────────────────────────────────────

#[test]
fn pipeline_frac_gera_pdf_sem_panic() {
    let (world, _dir) = world_from_str("$ frac(a, b) $");
    let source = world.source(world.main()).unwrap();
    let module = eval(&world, &source).unwrap();
    let doc    = layout(module.content(), &FixedMetrics).unwrap();
    let pdf    = export_pdf(&doc);
    assert!(!pdf.is_empty());
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn pipeline_attach_sup_gera_pdf_sem_panic() {
    let (world, _dir) = world_from_str("$ x^2 $");
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

# Confirmar MathBox em math/layout.rs
grep -n "struct MathBox\|fn layout_frac\|fn layout_attach\|fn hconcat" \
  01_core/src/rules/math/layout.rs

# Confirmar que layout.rs ainda delega a math::layout::MathLayouter
grep -n "MathLayouter\|math::layout" \
  01_core/src/rules/layout.rs
```

Critérios de conclusão:
- `MathBox { width, ascent, descent, items }` definido em `math/layout.rs` ✓
- `layout_frac`: numerador acima da baseline, denominador abaixo,
  tamanho 70% ✓
- `layout_attach`: sup elevado 50% do ascender, sub baixado 30%
  do descender, tamanho 65% ✓
- `hconcat` implementado ✓
- `math_frac_numerador_acima_denominador` passa ✓
- `math_attach_sup_elevado` passa ✓
- `math_attach_sub_baixado` passa ✓
- `pipeline_frac_gera_pdf_sem_panic` passa ✓
- DEBT-8 actualizado ✓
- Zero violations ✓
- Testes não regridem (437 L1 + 53 L3 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- `vertical_metrics(size)` — struct exacta retornada e nomes dos campos
  usados para `ascender` e `descender`.
- Convenção de coordenadas `y` — cresce para baixo (PDF) ou para cima?
  Como foi ajustado em `layout_attach` para `sup` vs `sub`?

**Da implementação:**
- `layout_equation` ainda retorna `Vec<FrameItem>` ou foi alterada?
- O teste `math_frac_numerador_acima_denominador` — o sinal de `ys[0]`
  vs `ys[1]` confirmou que y cresce para baixo?
- Número final de testes e zero violations confirmado.

**DEBT-8 parcialmente resolvido. Go para Passo 38 — linha de fracção e
`MathDelimited`, ou revisão de ADRs pendentes.**
