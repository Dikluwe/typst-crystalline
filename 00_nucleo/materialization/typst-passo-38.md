# Passo 38 — Motor de equações: linha de fracção, MathDelimited, e frac() nativa

**Pré-condições**:
- Passo 37 concluído: 441 L1 + 55 L3 + 50 parity, zero violations
- `MathBox`, `layout_frac`, `layout_attach`, `hconcat` implementados
- `vertical_metrics(size)` retorna `(Pt, Pt)` — `.0 = ascender (size*0.8)`, `.1 = line_height (size*1.2)`
- Branch: `cristalino/migration`

**Verificação antes de começar**:
```bash
# Confirmar FrameItem — que variantes existem actualmente
grep -n "pub enum FrameItem\|Text\|Line\|Rect\|Shape" \
  01_core/src/entities/layout_types.rs | head -20

# Confirmar como descender é calculado actualmente em layout_frac
grep -n "descent\|descender\|\.1\|line_height" \
  01_core/src/rules/math/layout.rs | head -15

# Confirmar como frac() é tratado em eval_math_expr
grep -n "frac\|MathFrac\|Expr::FuncCall\|funcall" \
  01_core/src/rules/eval.rs | head -15

# Ver MathDelimited no AST
grep -n "MathDelimited\|delimited\|open\|close" \
  01_core/src/entities/ast/math.rs | head -15

# Ver SyntaxKind::MathDelimited
grep -n "MathDelimited" \
  01_core/src/entities/syntax_kind.rs
```

**Parar. Reportar output completo antes de qualquer código.**

Questões a responder:
1. `FrameItem` tem variante `Line` ou `Rect`? Se não, o que existe para
   desenhar uma linha horizontal?
2. `descender` em `layout_frac` — foi calculado como `line_height - ascender`
   ou outro método?
3. `frac()` em `eval_math_expr` — cai no arm `_` que produz
   `Content::Empty`, ou já tem tratamento parcial?
4. `MathDelimited` no AST — que métodos tem? (`open()`, `close()`, `body()`?)

---

## Contexto

Este passo tem três objectivos independentes:

1. **Linha de fracção**: adicionar `FrameItem::Line` (ou equivalente)
   para desenhar a linha horizontal entre numerador e denominador em
   `MathFrac`. O Passo 37 posicionou os textos mas omitiu a linha.

2. **`frac()` como função nativa de math**: a sintaxe `$frac(a, b)$`
   é uma chamada de função em modo matemático. O eval precisa de
   reconhecer `frac` no namespace de math e produzir `Content::MathFrac`.
   Sem isto, `$frac(a, b)$` produz `Content::Empty`.

3. **`MathDelimited`**: parênteses, colchetes e chavetas em modo
   matemático — `$(a + b)$`, `$[x]$`. O eval produz
   `Content::MathSequence` com os delimitadores como `MathText` e o
   corpo no meio.

---

## Tarefa 2 — Linha de fracção em `FrameItem`

### 2a — Verificar se `FrameItem::Line` existe

```bash
grep -n "Line\|Stroke\|Rule\|Rect" \
  01_core/src/entities/layout_types.rs
```

### 2b — Se `FrameItem::Line` não existe, adicionar

```rust
// Em entities/layout_types.rs
pub enum FrameItem {
    Text {
        pos:   Point,
        text:  EcoString,
        style: TextStyle,
    },
    /// Linha horizontal. Usada pela linha de fracção matemática.
    /// `start` e `end` são posições absolutas no Frame.
    /// `thickness` em pontos.
    Line {
        start:     Point,
        end:       Point,
        thickness: f64,
    },
}
```

### 2c — Actualizar `layout_frac` para emitir a linha

Localizar `layout_frac` em `math/layout.rs` e adicionar o item de linha:

```rust
// Dentro de layout_frac, depois de calcular posições de num e den:

// Linha de fracção posicionada ligeiramente acima da baseline (y=0 é a baseline).
// y negativo = acima da baseline (convenção do MathLayouter — Passo 37).
let thickness = style.size * 0.05;
let rule_y    = -(style.size * 0.25); // barra ligeiramente acima da baseline

items.push(FrameItem::Line {
    start:     Point { x: 0.0.into(),   y: rule_y.into() },
    end:       Point { x: width.into(), y: rule_y.into() },
    thickness,
});
```

### 2d — Actualizar `export.rs` para tratar `FrameItem::Line`

O export precisa de converter `FrameItem::Line` para operadores PDF.
Em PDF, uma linha horizontal é:

```
q                          % save graphics state
<thickness> w              % line width
<x1> <y1> m               % moveto
<x2> <y2> l               % lineto
S                          % stroke
Q                          % restore graphics state
```

Localizar em `export.rs` onde `FrameItem::Text` é processado e
adicionar o arm para `FrameItem::Line`:

```rust
FrameItem::Line { start, end, thickness } => {
    // IMPORTANTE: usar a mesma lógica de conversão de Y que FrameItem::Text já usa
    // — seja através de matrizes de transformação, page_height do contexto,
    // ou inversão de eixo. Não assumir que page_height está disponível directamente.
    // Copiar o padrão exacto do arm FrameItem::Text imediatamente acima.
    let x1 = start.x.to_pt();
    let y1 = /* mesma conversão de Y do FrameItem::Text — adaptar */ ;
    let x2 = end.x.to_pt();
    let y2 = /* mesma conversão de Y do FrameItem::Text — adaptar */ ;

    write!(stream,
        "q {} w {} {} m {} {} l S Q\n",
        thickness, x1, y1, x2, y2
    ).unwrap();
}
```

**Regra de ouro**: o arm de `Line` deve usar exactamente a mesma
transformação de coordenadas que o arm de `Text` — nem mais, nem menos.
Se `Text` usa `page_height - pos.y.to_pt()`, `Line` usa o mesmo para
`start.y` e `end.y`. Se `Text` usa uma matriz de transformação já activa
no stream, `Line` não precisa de fazer nada extra.

---

## Tarefa 3 — `frac()` como função nativa de math

A sintaxe `$frac(a, b)$` em modo math é parseada como uma chamada de
função (`Expr::FuncCall` com alvo `frac`) dentro do corpo de
`Expr::Equation`. O `eval_math_expr` precisa de reconhecer esta chamada.

### 3a — Diagnosticar como FuncCall aparece em math mode

```bash
# Ver como Expr::FuncCall é tratado em eval_math_expr
grep -n "FuncCall\|func_call\|Expr::Call" \
  01_core/src/rules/eval.rs | head -15

# Ver se eval_math_expr delega para eval_expr ou tem lógica separada
grep -n "fn eval_math_expr\|eval_math_content" \
  01_core/src/rules/eval.rs | head -5
```

### 3b — Adicionar tratamento de `frac` em `eval_math_expr`

Se `eval_math_expr` não trata `Expr::FuncCall`, adicionar:

```rust
// Em eval_math_expr ou eval_math_content, arm para FuncCall:
Expr::FuncCall(call) => {
    let name = call.callee()
        .to_untyped()
        .text()
        .to_string();

    match name.as_str() {
        "frac" => {
            // Extrair os dois primeiros argumentos posicionais via pattern matching.
            // call.args().items() retorna ast::Arg — pode ser Pos(Expr) ou Named(Named).
            let mut pos_args = call.args().items().filter_map(|arg| match arg {
                ast::Arg::Pos(expr) => Some(expr),
                _ => None,
            });

            if let (Some(num_expr), Some(den_expr)) = (pos_args.next(), pos_args.next()) {
                let num = eval_math_content(ctx, scopes, num_expr.to_untyped())?;
                let den = eval_math_content(ctx, scopes, den_expr.to_untyped())?;
                Ok(Content::MathFrac {
                    num: Box::new(num),
                    den: Box::new(den),
                })
            } else {
                Err(vec![SourceDiagnostic::error(
                    call.span(),
                    "frac() requer 2 argumentos posicionais",
                )])
            }
        }
        // Outros nomes: tratar como MathIdent (ex: sin, cos, lim)
        _ => Ok(Content::MathIdent(name.into())),
    }
}
```

**Nota**: `eval_math_node` pode ser o mesmo que `eval_math_expr` ou
uma variante que aceita `Expr<'a>`. Adaptar conforme a estrutura actual.

---

## Tarefa 4 — `MathDelimited` em `eval_math_expr`

`MathDelimited` representa expressões entre delimitadores:
`$(a + b)$`, `$[x, y]$`, `${a | b}$`.

Adicionar arm em `eval_math_expr`:

```rust
SyntaxKind::MathDelimited => {
    if let Some(delimited) = child.cast::<ast::MathDelimited>() {
        // .open() e .close() retornam nós da árvore sintáctica, não strings.
        // Usar .to_untyped().text() para extrair o texto do token.
        let open  = delimited.open().to_untyped().text().clone();
        let close = delimited.close().to_untyped().text().clone();
        let body  = eval_math_content(ctx, scopes, delimited.body().to_untyped())?;

        Ok(Content::MathSequence(
            vec![
                Content::MathText(open),
                body,
                Content::MathText(close),
            ].into()
        ))
    } else {
        Ok(Content::Empty)
    }
}
```

---

## Tarefa 5 — Actualizar `DEBT.md`

```markdown
### DEBT-8 — Motor de equações — PARCIALMENTE RESOLVIDO

**Resolvido no Passo 38**:
- Linha de fracção via `FrameItem::Line` em `layout_frac`
- `frac(a, b)` reconhecido em `eval_math_expr` → `Content::MathFrac`
- `MathDelimited` em `eval_math_expr` → `Content::MathSequence`
  com delimitadores como `MathText`
- `export.rs` trata `FrameItem::Line` com operadores PDF `w m l S`

**Ainda pendente**:
- Kern matemático entre símbolos
- Fontes OpenType MATH
- `MathPrimes`, `MathShorthand`, `MathAlignPoint` com semântica correcta
- Baseline correcta em relação ao x-height da fonte
- `sqrt()` como função nativa (análogo a `frac()`)
```

---

## Tarefa 6 — Testes

```rust
// ── linha de fracção ──────────────────────────────────────────────────────

#[test]
fn math_frac_tem_item_linha() {
    let metrics  = FixedMetrics;
    let layouter = MathLayouter::new(&metrics);
    let style    = TextStyle { bold: false, italic: false, size: 10.0 };

    let frac = Content::MathFrac {
        num: Box::new(Content::MathIdent("a".into())),
        den: Box::new(Content::MathIdent("b".into())),
    };

    let items = layouter.layout_equation(&frac, &style);
    let has_line = items.iter().any(|item| matches!(item, FrameItem::Line { .. }));
    assert!(has_line, "frac deve ter FrameItem::Line para a linha de fracção");
}

#[test]
fn math_frac_linha_horizontal() {
    let metrics  = FixedMetrics;
    let layouter = MathLayouter::new(&metrics);
    let style    = TextStyle { bold: false, italic: false, size: 10.0 };

    let frac = Content::MathFrac {
        num: Box::new(Content::MathIdent("a".into())),
        den: Box::new(Content::MathIdent("b".into())),
    };

    let items = layouter.layout_equation(&frac, &style);
    for item in &items {
        if let FrameItem::Line { start, end, .. } = item {
            // Linha horizontal: y deve ser igual em start e end
            assert_eq!(start.y.to_pt(), end.y.to_pt(),
                "linha de fracção deve ser horizontal");
            // Linha deve ter largura positiva
            assert!(end.x.to_pt() > start.x.to_pt(),
                "linha de fracção deve ter largura > 0");
        }
    }
}

// ── frac() como função nativa ─────────────────────────────────────────────

#[test]
fn eval_frac_funcao_nativa_produz_mathfrac() {
    let world = MockWorld::new("$frac(a, b)$");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "frac() deve ser reconhecido: {:?}", result);
    // Verificar que não é Content::Empty — adaptar conforme API de Module
}

// ── MathDelimited ─────────────────────────────────────────────────────────

#[test]
fn eval_math_delimited_parenteses() {
    let world = MockWorld::new("$(a + b)$");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "$(a + b)$ deve avaliar sem erro: {:?}", result);
}

#[test]
fn eval_math_delimited_colchetes() {
    let world = MockWorld::new("$[x]$");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "$[x]$ deve avaliar sem erro: {:?}", result);
}

// ── integração L3 ─────────────────────────────────────────────────────────

#[test]
fn pipeline_frac_funcao_nativa_gera_pdf() {
    let (world, _dir) = world_from_str("$frac(a, b)$");
    let source = world.source(world.main()).unwrap();
    let module = eval(&world, &source).unwrap();
    let doc    = layout(module.content(), &FixedMetrics).unwrap();
    let pdf    = export_pdf(&doc);
    assert!(!pdf.is_empty());
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn pipeline_linha_fraccao_no_pdf() {
    // PDF deve conter operadores de linha (w m l S) para a linha de fracção
    let (world, _dir) = world_from_str("$a/b$");
    let source = world.source(world.main()).unwrap();
    let module = eval(&world, &source).unwrap();
    let doc    = layout(module.content(), &FixedMetrics).unwrap();
    let pdf    = export_pdf(&doc);
    // Verificar que o stream PDF contém operadores de stroke
    let pdf_str = String::from_utf8_lossy(&pdf);
    assert!(pdf_str.contains(" S ") || pdf_str.contains("\nS\n"),
        "PDF deve conter operador S (stroke) para a linha de fracção");
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

# Confirmar FrameItem::Line
grep -n "Line\|thickness" 01_core/src/entities/layout_types.rs

# Confirmar que export trata Line
grep -n "FrameItem::Line\|stroke\|\" S \"" \
  03_infra/src/export.rs

# Confirmar frac() em eval_math_expr
grep -n "\"frac\"\|frac.*FuncCall\|FuncCall.*frac" \
  01_core/src/rules/eval.rs
```

Critérios de conclusão:
- `FrameItem::Line { start, end, thickness }` adicionado a `layout_types.rs` ✓
- `layout_frac` emite `FrameItem::Line` para a linha horizontal ✓
- `export.rs` trata `FrameItem::Line` com operadores PDF `w m l S` ✓
- `frac(a, b)` em `eval_math_expr` produz `Content::MathFrac` ✓
- `MathDelimited` em `eval_math_expr` produz `Content::MathSequence`
  com delimitadores ✓
- `math_frac_tem_item_linha` passa ✓
- `math_frac_linha_horizontal` passa ✓
- `eval_frac_funcao_nativa_produz_mathfrac` passa ✓
- `eval_math_delimited_parenteses` e `eval_math_delimited_colchetes` passam ✓
- `pipeline_linha_fraccao_no_pdf` passa ✓
- DEBT-8 actualizado ✓
- Zero violations ✓
- Testes não regridem (441 L1 + 55 L3 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- `FrameItem` já tinha variante `Line` ou foi adicionada neste passo?
- A convenção de y em `export.rs` para `FrameItem::Text` — inverte y
  ou não? Aplicar o mesmo para `Line`.
- `MathDelimited` no AST — métodos `open()`, `close()`, `body()` com
  que tipos?

**Da implementação:**
- `frac()` em math mode é `Expr::FuncCall` ou outro `Expr`?
- O teste `pipeline_linha_fraccao_no_pdf` encontrou `S` no stream PDF?
- Número final de testes e zero violations confirmado.

**DEBT-8 parcialmente resolvido. Go para Passo 39.**
