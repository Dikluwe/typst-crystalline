# Passo 77 — Curvas de Bézier, Elipses e Origens Lógicas (DEBT-32)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/stdlib.rs` — Onde `native_rect` e `native_line` estão
  definidas. `native_ellipse` e `native_circle` serão adicionadas aqui.
- `03_infra/src/export.rs` — Onde o braço `ShapeKind::Line` será corrigido
  e o braço `ShapeKind::Ellipse` substituirá o placeholder com Bézier real.
- `01_core/DEBT.md` — Confirmar que DEBT-32 está registado e que DEBT-30/31
  estão em aberto.

Pré-condição: `cargo test` — 721 L1 + 147 L3, zero violations.
DEBT-32 registado. `ShapeKind::Ellipse` no exportador emite rectângulo
placeholder com comentário `TODO`.

---

## Contexto

O Passo 76 introduziu `rect` e `line` com exportação PDF funcional, mas
deixou dois problemas em aberto:

- **DEBT-32 — Deltas negativos em linhas:** o exportador desenha a linha
  a partir de `pos.x`, sem considerar o sinal de `dx`/`dy`. Se `dx < 0`,
  a linha sai para a esquerda da bounding box, sobrepondo-se ao conteúdo
  vizinho. A correcção calcula as coordenadas de início e fim dentro da
  bounding box com base no sinal dos deltas.

- **Elipse sem implementação real:** `ShapeKind::Ellipse` emitia um
  rectângulo placeholder. O PDF não tem um operador nativo para elipses —
  a aproximação usa quatro curvas de Bézier cúbicas com a constante
  κ ≈ 0.552284749831, que minimiza o erro de arredondamento para qualquer
  raio.

Este passo fecha DEBT-32 e adiciona `ellipse` e `circle` à stdlib com
exportação correcta.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar as funções nativas de formas já registadas
grep -n "register(\"rect\"\|register(\"line\"" \
  01_core/src/rules/eval.rs \
  01_core/src/rules/stdlib.rs 2>/dev/null

# 2. Confirmar a lógica actual do Line no exportador
grep -A 8 "ShapeKind::Line" 03_infra/src/export.rs

# 3. Confirmar a variante Value usada para comprimentos numéricos
# (necessário para empacotar o diâmetro do circle como Value)
grep -n "Value::Float\|Value::Length\|Value::Numeric" \
  01_core/src/entities/content.rs | head -10
```

Reportar o output completo antes de continuar. O diagnóstico 3 é crítico
para a Tarefa 1: `native_circle` converte `radius` em `diameter` e precisa
de o empacotar como a variante correcta de `Value` para `Content::Shape.width`
e `Content::Shape.height`.

---

## Tarefa 0 — Actualizar DEBT.md

Marcar DEBT-32 como `EM CURSO` antes de qualquer código:

```markdown
### DEBT-32 — Alinhamento da bounding box para linhas com deltas negativos — EM CURSO (Passo 77)
```

Será marcado `ENCERRADO ✓` no final da Tarefa 3.

---

## Tarefa 1 — `native_ellipse` e `native_circle` na stdlib (L1)

Em `01_core/src/rules/stdlib.rs`, adicionar as duas funções.

`native_ellipse` segue exactamente o padrão de `native_rect` — a diferença
é apenas `ShapeKind::Ellipse` em vez de `ShapeKind::Rect`:

```rust
pub fn native_ellipse(_ctx: &mut EvalContext, args: &Args) -> Result<Value, String> {
    let width  = args.named::<Value>("width");
    let height = args.named::<Value>("height");
    let fill   = args.named::<Value>("fill").and_then(|v| parse_color(&v));

    let parsed_stroke: Option<Stroke> = args.named::<Value>("stroke").and_then(|v| {
        parse_color(&v).map(|c| Stroke { paint: c, thickness: 1.0 })
    });

    // Fallback determinístico: mesmo padrão de native_rect.
    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke { paint: Color::Rgb { r: 0, g: 0, b: 0 }, thickness: 1.0 })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::Shape {
        kind: ShapeKind::Ellipse,
        width,
        height,
        fill,
        stroke: final_stroke,
    }))
}
```

`native_circle` é açúcar sintático sobre `native_ellipse`: o utilizador
passa `radius` e o código converte para `width = height = radius * 2`.
A variante de `Value` usada para empacotar o diâmetro depende do diagnóstico 3:

```rust
pub fn native_circle(_ctx: &mut EvalContext, args: &Args) -> Result<Value, String> {
    // radius em pontos — extrair como f64 do Value::Length ou equivalente.
    let radius = args.named::<f64>("radius");

    let (width, height) = match radius {
        Some(r) => {
            // Empacotar o diâmetro como a variante correcta de Value.
            // Adaptar ao resultado do diagnóstico 3 — pode ser Value::Float,
            // Value::Length, ou Value::Numeric conforme a definição real.
            let diameter = Value::Float(r * 2.0);
            (Some(diameter.clone()), Some(diameter))
        },
        None => (None, None), // sem radius → layouter usa available_width
    };

    let fill   = args.named::<Value>("fill").and_then(|v| parse_color(&v));
    let parsed_stroke: Option<Stroke> = args.named::<Value>("stroke").and_then(|v| {
        parse_color(&v).map(|c| Stroke { paint: c, thickness: 1.0 })
    });

    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke { paint: Color::Rgb { r: 0, g: 0, b: 0 }, thickness: 1.0 })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::Shape {
        kind:   ShapeKind::Ellipse, // circle é uma ellipse com width == height
        width,
        height,
        fill,
        stroke: final_stroke,
    }))
}
```

Registar ambas:

```rust
ctx.register("ellipse", native_ellipse);
ctx.register("circle",  native_circle);
```

---

## Tarefa 2 — Correcção de DEBT-32 no exportador (L3)

Em `03_infra/src/export.rs`, substituir o braço `ShapeKind::Line` actual.

O problema: o exportador desenhava de `pos.x` para `pos.x + dx` sem
considerar o sinal de `dx`. Se `dx < 0`, a linha saía para a esquerda
da bounding box.

A correcção mapeia as coordenadas de início e fim para dentro da bounding
box com base no sinal dos deltas:

```rust
ShapeKind::Line { dx, dy } => {
    // pdf_y é a base (canto inferior esquerdo) da bounding box no espaço PDF.
    // width e height são os valores absolutos calculados pelo layouter (dx.abs(), dy.abs()).

    // Eixo X:
    // dx > 0 → linha vai da esquerda para a direita da bounding box.
    // dx < 0 → linha vai da direita para a esquerda da bounding box.
    let start_offset_x = if *dx < 0.0 { *width } else { 0.0 };
    let end_offset_x   = if *dx < 0.0 { 0.0 }    else { *width };

    // Eixo Y (inversão obrigatória layout→PDF):
    // No layout Y cresce para baixo; no PDF Y cresce para cima.
    // dy > 0 → desce no layout → o início está no topo da caixa PDF (pdf_y + height)
    //          e o fim na base (pdf_y).
    // dy < 0 → sobe no layout → o início está na base da caixa PDF (pdf_y)
    //          e o fim no topo (pdf_y + height).
    let start_offset_y = if *dy > 0.0 { *height } else { 0.0 };
    let end_offset_y   = if *dy > 0.0 { 0.0 }     else { *height };

    let start_x = pos.x.0 + start_offset_x;
    let start_y = pdf_y   + start_offset_y;
    let end_x   = pos.x.0 + end_offset_x;
    let end_y   = pdf_y   + end_offset_y;

    page_stream.push_str(&format!("{:.3} {:.3} m\n", start_x, start_y));
    page_stream.push_str(&format!("{:.3} {:.3} l\n", end_x,   end_y));
},
```

---

## Tarefa 3 — Motor Bézier para elipses no exportador (L3)

Em `03_infra/src/export.rs`, substituir o placeholder de `ShapeKind::Ellipse`
pela aproximação com quatro curvas de Bézier cúbicas.

O PDF não tem um operador para elipses. A aproximação com κ ≈ 0.5523 é
a mais precisa possível com quatro segmentos Bézier — o erro máximo é
inferior a 0.03% do raio para qualquer tamanho:

```rust
ShapeKind::Ellipse => {
    // Constante de aproximação de Bézier para círculos/elipses.
    // κ = 4 * (√2 - 1) / 3 ≈ 0.552284749831
    // Minimiza o erro de arredondamento para qualquer raio.
    const KAPPA: f64 = 0.552284749831;

    // Centro da elipse no espaço PDF.
    let cx = pos.x.0 + (width  / 2.0);
    let cy = pdf_y   + (height / 2.0);

    // Semi-eixos.
    let rx = width  / 2.0;
    let ry = height / 2.0;

    // Deslocamentos dos pontos de controlo Bézier.
    let ox = rx * KAPPA;
    let oy = ry * KAPPA;

    // Mover para o topo da elipse (ponto inicial).
    page_stream.push_str(&format!("{:.3} {:.3} m\n", cx, cy + ry));

    // 1º quadrante: topo → direita
    page_stream.push_str(&format!(
        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
        cx + ox, cy + ry,  // ponto de controlo 1
        cx + rx, cy + oy,  // ponto de controlo 2
        cx + rx, cy,       // ponto final
    ));

    // 4º quadrante: direita → base
    page_stream.push_str(&format!(
        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
        cx + rx, cy - oy,
        cx + ox, cy - ry,
        cx,      cy - ry,
    ));

    // 3º quadrante: base → esquerda
    page_stream.push_str(&format!(
        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
        cx - ox, cy - ry,
        cx - rx, cy - oy,
        cx - rx, cy,
    ));

    // 2º quadrante: esquerda → topo (fecha a elipse)
    page_stream.push_str(&format!(
        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
        cx - rx, cy + oy,
        cx - ox, cy + ry,
        cx,      cy + ry,
    ));
},
```

---

## Tarefa 4 — Testes

### Teste L3 — linha com delta negativo respeita a bounding box (DEBT-32)

```rust
#[test]
fn export_line_com_delta_negativo_respeita_bounding_box() {
    let root = criar_dir_temporario();
    // Linha que vai para a esquerda e para cima.
    std::fs::write(root.join("main.typ"), "#line(dx: -50pt, dy: -30pt)").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);
    let pdf    = export_pdf(&doc);

    let pdf_str = String::from_utf8_lossy(&pdf);

    // Com dx < 0, o início da linha está no lado DIREITO da bounding box
    // e o fim está no lado ESQUERDO — end_x < start_x.
    let m_x = extrair_x_operador(&pdf_str, " m\n");
    let l_x = extrair_x_operador(&pdf_str, " l\n");

    assert!(l_x < m_x,
        "Linha com dx negativo deve terminar à esquerda do início: \
         end_x ({}) deve ser menor que start_x ({})",
        l_x, m_x);
}
```

### Teste L3 — elipse emite quatro curvas de Bézier

```rust
#[test]
fn export_ellipse_emite_quatro_operadores_bezier() {
    let root = criar_dir_temporario();
    std::fs::write(
        root.join("main.typ"),
        "#ellipse(width: 80pt, height: 40pt, fill: \"blue\")",
    ).unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);
    let pdf    = export_pdf(&doc);

    let pdf_str = String::from_utf8_lossy(&pdf);

    assert_eq!(
        pdf_str.matches(" c\n").count(), 4,
        "Elipse deve ser desenhada com exactamente 4 operadores Bézier 'c'"
    );
    assert!(pdf_str.contains(" m\n"),
        "Elipse deve ter um ponto inicial 'm'");
    // Não deve conter o operador re (rectângulo placeholder do Passo 76).
    assert!(!pdf_str.contains(" re\n"),
        "Elipse não deve emitir operador re — placeholder foi substituído");
}
```

### Teste L3 — circle produz o mesmo resultado que ellipse com width == height

```rust
#[test]
fn export_circle_emite_quatro_operadores_bezier() {
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "#circle(radius: 20pt)").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);
    let pdf    = export_pdf(&doc);

    let pdf_str = String::from_utf8_lossy(&pdf);

    assert_eq!(
        pdf_str.matches(" c\n").count(), 4,
        "Circle deve ser desenhado com exactamente 4 operadores Bézier 'c'"
    );
}
```

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint --fix-hashes .
crystalline-lint .
```

Critérios de conclusão:
- [ ] `native_ellipse` implementada e registada com `ctx.register("ellipse", ...)`.
- [ ] `native_circle` implementada e registada com `ctx.register("circle", ...)`.
  `circle` converte `radius` em `width = height = radius * 2` e emite
  `ShapeKind::Ellipse`.
- [ ] A variante de `Value` usada para o diâmetro em `native_circle`
  corresponde ao resultado do diagnóstico 3.
- [ ] Braço `ShapeKind::Line` no exportador corrigido: usa `start_offset_x`/
  `end_offset_x` e `start_offset_y`/`end_offset_y` com base no sinal dos deltas.
- [ ] Comentários no braço `Line` explicam a inversão de eixos para cada caso
  de sinal (`dx > 0`, `dx < 0`, `dy > 0`, `dy < 0`).
- [ ] Braço `ShapeKind::Ellipse` substituído pela aproximação Bézier com
  `KAPPA = 0.552284749831` e quatro operadores `c`.
- [ ] O placeholder `re` do Passo 76 foi removido do braço `Ellipse`.
- [ ] DEBT-32 marcado como **ENCERRADO ✓** em `01_core/DEBT.md`.
- [ ] Teste `export_line_com_delta_negativo_respeita_bounding_box` passa.
- [ ] Teste `export_ellipse_emite_quatro_operadores_bezier` passa.
- [ ] Teste `export_circle_emite_quatro_operadores_bezier` passa.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Variante de `Value` usada para comprimentos — `Value::Float`, `Value::Length`,
  ou `Value::Numeric`. Determina como o diâmetro é empacotado em `native_circle`.

**Da implementação:**
- Se o teste `export_ellipse_emite_quatro_operadores_bezier` detectou o
  operador `re` residual do placeholder e obrigou a remover o fallback.
- Número total de testes após o passo e zero violations confirmados.

**Go/No-Go para o Passo 78:**
- **GO — formas visíveis no PDF:** rectângulos, linhas, elipses e círculos
  aparecem com as cores correctas num leitor PDF real. Linhas com deltas
  negativos ficam dentro da bounding box.
- **NO-GO — elipse com re residual:** se o teste de elipse falha com
  `assert!(!pdf_str.contains(" re\n"))`, o placeholder ainda está activo.
  Verificar que o braço `Ellipse` no exportador foi substituído e não
  tem fallback para `re`.
- **NO-GO — circle sem dimensões:** se `native_circle` emite `width: None`
  quando `radius` é passado, o layouter usa `available_width` em vez do
  diâmetro. Verificar o empacotamento do diâmetro como `Value`.
