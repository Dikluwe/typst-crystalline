# Passo 76 — Primitivas Gráficas e Exportação Vectorial (DEBT-30, DEBT-31)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/content.rs` — AST actual. Confirmar se `Content::Shape`
  já existe de alguma forma ou se é inteiramente novo.
- `01_core/src/rules/layout/frame.rs` — Display list actual. Confirmar a
  estrutura de `FrameItem` e como `pos` (posição) é armazenada.
- `03_infra/src/export.rs` — Serializador manual de PDF. Confirmar os
  operadores actuais no `page_stream` (ex: `q`, `cm`, `Do`, `Q`).
- `01_core/DEBT.md` — Confirmar que DEBT-25, DEBT-14, DEBT-15 estão
  encerrados e que DEBT-30/31 ainda não existem.

Pré-condição: `cargo test` — ~700+ testes a passar. Passos 71–75 concluídos.
Zero dívidas técnicas críticas abertas no motor visual.

---

## Contexto

Os passos 71–75 completaram o suporte a imagens e figuras numeradas. O motor
de layout sabe posicionar conteúdo na página e exportá-lo para PDF, mas todos
os elementos visuais são texto ou imagens raster. Documentos reais precisam de
primitivas vectoriais: rectângulos, elipses, linhas — com cores de
preenchimento e contornos.

Este passo introduz três sistemas novos em coordenação:

- **Tipos de traço** (`Stroke`, `ShapeKind`) em L1, reutilizando o tipo
  `Color` já existente em `layout_types.rs` — sem criar tipos duplicados.
- **Layout de formas** — dimensionamento e cálculo de bounding box para
  `Content::Shape` no layouter.
- **Operadores PDF vectoriais** — tradução de `FrameItem::Shape` para os
  operadores nativos do PDF (`rg`, `RG`, `w`, `re`, `m`, `l`, `B`, `f`, `S`).

Duas dívidas técnicas são registadas para funcionalidades que ficam fora do
escopo deste passo:

- **DEBT-30 — Clipping paths:** `clip: true` em contentores requer o operador
  `W` e sequências `W n` que alteram o clipping path activo. Adiado.
- **DEBT-31 — Transformações afins:** rotação e escala em primitivas requerem
  a matriz `cm` com valores não triviais. Adiado.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar como pos (posição) é armazenada em FrameItem
grep -A 5 "enum FrameItem" 01_core/src/rules/layout/frame.rs | head -15

# 2. Confirmar os operadores de página actuais no exportador
grep -n "page_stream.push_str" 03_infra/src/export.rs | head -10

# 3. Confirmar a estrutura do tipo Color existente em layout_types
# (Stroke vai reutilizar este tipo — confirmar variantes Rgb/Rgba e campos)
grep -n "Color\|Rgb\|Rgba" 01_core/src/entities/layout_types.rs | head -10

# CRÍTICO: confirmar a tipagem dos canais — u8 (0–255) ou f64 (0.0–1.0)?
# Se for f64, a divisão por 255.0 no exportador está ERRADA.
# Nesse caso, color_to_rgb deve retornar (f64, f64, f64) e os format!()
# usam os valores directamente sem divisão.
grep -A 5 "Rgb\b" 01_core/src/entities/layout_types.rs | head -10

# 4. Confirmar como width/height são resolvidos no layouter para Content::Image
# (padrão a replicar para Content::Shape)
grep -n "calculate_dimensions\|width_pt\|cursor_y" \
  01_core/src/rules/layout/mod.rs | head -10

# 5. Confirmar o tipo de page_height no exportador — f64, Pt, ou outro
grep -n "page_height" 03_infra/src/export.rs | head -5
```

Reportar o output completo antes de continuar. O diagnóstico 1 é crítico:
se `FrameItem` armazena `pos` como campo do próprio item, o código do
exportador acede a `item.pos`; se `pos` é o primeiro elemento de um tuple
`(Point, FrameItem)`, o acesso é `pos.x.0` e `pos.y.0`. O código da Tarefa 4
usa o segundo padrão — confirmar antes de codificar.

---

## Tarefa 0 — Actualizar DEBT.md

Antes de qualquer código, registar em `01_core/DEBT.md`:

```markdown
### DEBT-30 — Suporte a clipping paths (clip: true) — EM ABERTO (Passo 76)
Contentores com clip: true requerem o operador W (clipping path) e a sequência
W n antes de desenhar o conteúdo interno. O PDF mantém um clipping path activo
por estado gráfico (q/Q), portanto clip: true exige um push/pop de estado
adicional. Resolução: passo futuro de layout de contentores.

### DEBT-31 — Transformações afins (rotate, scale) em nós — EM ABERTO (Passo 76)
Rotação e escala requerem a matriz cm com valores não-identidade:
[cos -sin sin cos tx ty] para rotação. A bounding box de um nó rodado não é
um rectângulo alinhado aos eixos — o layouter precisaria de calcular o
bounding box transformado. Resolução: passo futuro de transformações.

### DEBT-32 — Alinhamento da bounding box para linhas com deltas negativos — EM ABERTO (Passo 76)
O layouter usa dx.abs() e dy.abs() para o tamanho da bounding box, mas o
exportador desenha a partir de pos.x. Se dx < 0, a linha é desenhada para a
esquerda do cursor, saindo da sua bounding box e sobrepondo-se ao conteúdo
vizinho. Resolução: ajustar a coordenada inicial m no exportador ou a posição
de origem pos.x no layouter com base no sinal do delta.
```

---

## Tarefa 1 — Tipos geométricos e AST (L1)

### 1a — `Stroke` e `ShapeKind` (novo ficheiro L1)

Criar `01_core/src/entities/geometry.rs`. O tipo `Color` já existe em
`layout_types.rs` com variantes `Rgb`/`Rgba` — reutilizá-lo em `Stroke`
evita duplicação. Não criar `RgbaColor` separado.

Confirmar com o diagnóstico 3 as variantes e campos exactos de `Color`
antes de escrever o código abaixo.

```rust
use crate::entities::layout_types::Color;

/// Contorno de uma forma: cor e espessura.
///
/// Usa `Color` de `layout_types` — sem tipo RgbaColor separado.
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub paint:     Color,
    /// Espessura do contorno em pontos (pt).
    pub thickness: f64,
}

/// Tipo de forma geométrica.
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeKind {
    /// Rectângulo alinhado aos eixos.
    Rect,
    /// Elipse. Scaffolding presente; exportador PDF adiado (DEBT-31).
    Ellipse,
    /// Segmento de recta com deslocamento relativo à origem.
    ///
    /// `dx` e `dy` são deslocamentos no sistema do layout:
    /// positivo = direita (dx) ou baixo (dy).
    /// A bounding box usa os valores absolutos — ver Tarefa 3.
    Line { dx: f64, dy: f64 },
}
```

Expor em `01_core/src/entities/mod.rs`:

```rust
pub mod geometry;
pub use geometry::{Stroke, ShapeKind};
```

`Color` não precisa de ser re-exportado aqui — já é exportado via `layout_types`.

### 1b — `Content::Shape` em `content.rs`

Em `01_core/src/entities/content.rs`, adicionar a variante:

```rust
use crate::entities::geometry::{ShapeKind, Stroke};
use crate::entities::layout_types::Color;

// Na enum Content:
/// Forma geométrica primitiva.
///
/// Os campos `width` e `height` são opcionais no AST porque o utilizador
/// pode omiti-los (ex: #rect() sem tamanho). O layouter resolve os valores
/// finais e emite FrameItem::Shape com f64 concretos.
Shape {
    kind:   ShapeKind,
    width:  Option<Value>,
    height: Option<Value>,
    fill:   Option<Color>,
    stroke: Option<Stroke>,
},
```

Actualizar todos os `match` sobre `Content` — o compilador lista os locais.
Nos braços que não precisam de tratar formas (ex: introspecção, show rules),
adicionar `Content::Shape { .. } => {}` ou equivalente.

---

## Tarefa 2 — Primitivas na stdlib (L1)

Em `01_core/src/rules/stdlib.rs`, implementar `native_rect`, `native_line`,
e o helper `parse_color`.

**Regra de ouro:** o `Content::Shape` emitido pela stdlib sai completamente
resolvido — fill e stroke são `Option<Color>`/`Option<Stroke>` concretos,
nunca strings ou `Value` por resolver. O layouter e o exportador nunca adivinham
fallbacks.

Confirmar com o diagnóstico 3 a sintaxe exacta para construir `Color::Rgb`
(ou `Color::Rgba`) antes de escrever os valores abaixo.

```rust
/// Converte um Value em Color.
///
/// Reutiliza o tipo Color de layout_types — sem RgbaColor separado.
/// Por agora suporta apenas nomes de cor conhecidos. Valores hex (#rrggbb)
/// ficam para passo futuro — o parser real de cores Typst requer um lexer
/// dedicado.
fn parse_color(val: &Value) -> Option<Color> {
    match val {
        Value::String(s) => match s.as_str() {
            // Adaptar a construção de Color à variante real (Rgb/Rgba)
            // confirmada pelo diagnóstico 3.
            "red"   => Some(Color::Rgb { r: 255, g: 0,   b: 0   }),
            "green" => Some(Color::Rgb { r: 0,   g: 128, b: 0   }),
            "blue"  => Some(Color::Rgb { r: 0,   g: 0,   b: 255 }),
            "black" => Some(Color::Rgb { r: 0,   g: 0,   b: 0   }),
            "white" => Some(Color::Rgb { r: 255, g: 255, b: 255 }),
            _       => None,
        },
        _ => None,
    }
}

pub fn native_rect(_ctx: &mut EvalContext, args: &Args) -> Result<Value, String> {
    let width  = args.named::<Value>("width");
    let height = args.named::<Value>("height");
    let fill   = args.named::<Value>("fill").and_then(|v| parse_color(&v));

    let parsed_stroke: Option<Stroke> = args.named::<Value>("stroke").and_then(|v| {
        parse_color(&v).map(|c| Stroke { paint: c, thickness: 1.0 })
        // thickness configurável fica para passo futuro
    });

    // Fallback determinístico: sem fill nem stroke → stroke preta de 1pt.
    // Este é o único local onde este fallback pode existir — nem o layouter
    // nem o exportador têm permissão para inventar cores ou espessuras.
    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke {
            paint:     Color::Rgb { r: 0, g: 0, b: 0 }, // preto
            thickness: 1.0,
        })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::Shape {
        kind:   ShapeKind::Rect,
        width,
        height,
        fill,
        stroke: final_stroke,
    }))
}

pub fn native_line(_ctx: &mut EvalContext, args: &Args) -> Result<Value, String> {
    // dx/dy são obrigatórios para uma linha ter direcção definida.
    // Valores omitidos → 0.0 (linha degenerada, invisível mas válida).
    let dx: f64 = args.named::<f64>("dx").unwrap_or(0.0);
    let dy: f64 = args.named::<f64>("dy").unwrap_or(0.0);

    let stroke_color = args.named::<Value>("stroke")
        .and_then(|v| parse_color(&v))
        .unwrap_or(Color::Rgb { r: 0, g: 0, b: 0 }); // preto por omissão

    Ok(Value::Content(Content::Shape {
        kind:   ShapeKind::Line { dx, dy },
        width:  None, // bounding box calculada pelo layouter a partir de dx/dy
        height: None,
        fill:   None, // linhas não têm fill — apenas stroke
        stroke: Some(Stroke { paint: stroke_color, thickness: 1.0 }),
    }))
}
```

Registar as funções nativas no mapa de stdlib:

```rust
ctx.register("rect", native_rect);
ctx.register("line", native_line);
```

---

## Tarefa 3 — Bounding box e `FrameItem::Shape` (L1)

### 3a — Variante `FrameItem::Shape`

Em `01_core/src/rules/layout/frame.rs`:

```rust
// Na enum FrameItem:
/// Forma geométrica com dimensões resolvidas em pontos.
///
/// Todos os campos são concretos — sem Option<Value>. O layouter resolve
/// os valores do AST antes de emitir este item.
Shape {
    kind:   ShapeKind,
    /// Largura da bounding box em pontos.
    width:  f64,
    /// Altura da bounding box em pontos.
    height: f64,
    fill:   Option<Color>,
    stroke: Option<Stroke>,
},
```

Actualizar todos os `match` sobre `FrameItem`.

### 3b — Processamento de `Content::Shape` no layouter

Em `01_core/src/rules/layout/mod.rs`:

```rust
Content::Shape { kind, width, height, fill, stroke } => {
    // Resolver width e height conforme o tipo de forma.
    let (resolved_w, resolved_h) = match kind {
        ShapeKind::Rect | ShapeKind::Ellipse => {
            // Rect/Ellipse: width e height são explícitos ou fallback.
            // Se width for None, ocupar a largura disponível (100%).
            // Se height for None, usar 0 — uma forma sem altura é invisível
            // mas não é um erro; o utilizador especificou mal os argumentos.
            let w = resolve_pt(width.as_ref(), self.available_width());
            let h = resolve_pt(height.as_ref(), 0.0);
            (w, h)
        },
        ShapeKind::Line { dx, dy } => {
            // A bounding box de uma linha é o valor absoluto dos deltas.
            // dx < 0 ou dy < 0 são válidos (linha em direcção oposta) —
            // a posição de origem não muda, apenas a direcção do traço.
            (dx.abs(), dy.abs())
        },
    };

    // Verificar quebra de página antes de emitir (padrão de Content::Image).
    if self.cursor_y.0 + resolved_h > self.page_height.0 - self.margin.0 {
        self.new_page();
    }
    self.flush_line();

    let pos = Point { x: self.cursor_x, y: self.cursor_y };

    self.push_frame_item(pos, FrameItem::Shape {
        kind:   kind.clone(),
        width:  resolved_w,
        height: resolved_h,
        fill:   fill.clone(),
        stroke: stroke.clone(),
    });

    self.cursor_y += Pt(resolved_h);
},
```

`resolve_pt` é uma função auxiliar que extrai o valor numérico em pontos de
um `Option<&Value>`, com um fallback explícito. Adaptar ao padrão existente
para `Content::Image` (onde `calculate_dimensions` faz trabalho semelhante).

---

## Tarefa 4 — Operadores PDF vectoriais (L3)

Em `03_infra/src/export.rs`, adicionar o braço `FrameItem::Shape` ao loop de
exportação. A ordem dos operadores dentro do bloco `q/Q` é obrigatória no
PDF — alterar a sequência produz resultados incorrectos ou documentos
rejeitados por alguns leitores.

Adicionar primeiro o helper `color_to_rgb` junto às funções auxiliares do
exportador. A implementação concreta depende das variantes de `Color` — adaptar
ao resultado do diagnóstico 3:

```rust
/// Extrai os canais RGB de um Color para uso nos operadores PDF rg/RG.
/// Alpha é ignorado — transparência vectorial requer ca/CA (PDF 1.4).
///
/// A tipagem dos canais (u8 vs f64) depende da definição real de Color
/// em layout_types.rs — confirmar com o diagnóstico 3 antes de compilar.
/// Se Color::Rgb usar f64 no intervalo 0.0–1.0, remover a divisão por 255.0
/// no ponto de chamada e passar os valores directamente aos format!().
fn color_to_rgb(c: &Color) -> (u8, u8, u8) {
    match c {
        Color::Rgb  { r, g, b }     => (*r, *g, *b),
        Color::Rgba { r, g, b, .. } => (*r, *g, *b), // alpha descartado
        // TODO: suportar outras variantes de Color quando adicionadas (DEBT futura)
        _ => (0, 0, 0), // fallback defensivo — preto para variantes desconhecidas
    }
}
```

```rust
FrameItem::Shape { kind, width, height, fill, stroke } => {
    // Inverter eixo Y: layout tem Y crescente para baixo,
    // PDF tem Y crescente para cima.
    // pdf_y é o canto inferior esquerdo da bounding box no espaço PDF.
    let pdf_y = page_height - pos.y.0 - height;

    // Ordem obrigatória: push state → cores → path → paint operator → pop state.
    page_stream.push_str("q\n");

    // Cor de preenchimento (operador rg — RGB para fills).
    // Alpha ignorado por agora — transparência vectorial requer ca/CA (PDF 1.4).
    // Adaptar a extracção dos canais r/g/b à variante real de Color
    // confirmada pelo diagnóstico 3 (ex: Color::Rgb { r, g, b }).
    if let Some(c) = fill {
        let (r, g, b) = color_to_rgb(&c); // helper que extrai canais u8
        // Se Color usar f64 (0.0–1.0): remover a divisão por 255.0 abaixo.
        // Se Color usar u8 (0–255): manter como está.
        page_stream.push_str(&format!(
            "{:.3} {:.3} {:.3} rg\n",
            r as f64 / 255.0,
            g as f64 / 255.0,
            b as f64 / 255.0,
        ));
    }

    // Cor e espessura do contorno (operadores RG + w).
    if let Some(s) = stroke {
        let (r, g, b) = color_to_rgb(&s.paint);
        // Mesma nota de tipagem: ajustar divisão conforme o tipo dos canais.
        page_stream.push_str(&format!(
            "{:.3} {:.3} {:.3} RG\n{:.2} w\n",
            r as f64 / 255.0,
            g as f64 / 255.0,
            b as f64 / 255.0,
            s.thickness,
        ));
    }

    // Path — depende do tipo de forma.
    match kind {
        ShapeKind::Rect => {
            // Operador re: x y width height re
            // Define um rectângulo como sub-path fechado.
            page_stream.push_str(&format!(
                "{:.2} {:.2} {:.2} {:.2} re\n",
                pos.x.0, pdf_y, width, height,
            ));
        },

        ShapeKind::Ellipse => {
            // Aproximação por Bézier (constante kappa ≈ 0.5523).
            // Adiado — DEBT-31. Emitir um rectângulo como placeholder
            // para manter o PDF válido sem pânico.
            // TODO: substituir por aproximação real no passo futuro.
            page_stream.push_str(&format!(
                "{:.2} {:.2} {:.2} {:.2} re\n",
                pos.x.0, pdf_y, width, height,
            ));
        },

        ShapeKind::Line { dx, dy } => {
            // Ponto de início: canto superior esquerdo da bounding box.
            // start_y: Y do ponto de início no espaço PDF.
            // end_y:   Y do ponto de fim. dy positivo no layout = desce =
            //          subtrai no PDF (Y PDF cresce para cima).
            let start_y = page_height - pos.y.0;
            let end_y   = page_height - (pos.y.0 + dy);
            // dy positivo = desce no layout → subtrai no PDF (eixos opostos)

            page_stream.push_str(&format!("{:.2} {:.2} m\n", pos.x.0, start_y));
            page_stream.push_str(&format!("{:.2} {:.2} l\n", pos.x.0 + dx, end_y));
        },
    }

    // Paint operator — determina o que é preenchido/contornado.
    // (true, true)  → B: fill e stroke (fill first, then stroke over it)
    // (true, false) → f: fill only (non-zero winding rule)
    // (false, true) → S: stroke only
    // (false, false) → sem operador: path definido mas não pintado.
    //   Não deve acontecer com o fallback da stdlib (Tarefa 2), mas o código
    //   é defensivo — um path não pintado não corrompe o PDF.
    match (fill.is_some(), stroke.is_some()) {
        (true,  true)  => page_stream.push_str("B\n"),
        (true,  false) => page_stream.push_str("f\n"),
        (false, true)  => page_stream.push_str("S\n"),
        (false, false) => {},
    }

    page_stream.push_str("Q\n");
},
```

---

## Tarefa 5 — Testes

### Teste L1 — fallback determinístico de stroke (Tarefa 2)

```rust
#[test]
fn rect_sem_cores_tem_stroke_preta_1pt() {
    // #rect() sem fill nem stroke → stroke preta de 1pt.
    // Confirma que a stdlib é o único local onde este fallback existe.
    let content = eval_expr("#rect()"); // adaptar ao helper de teste existente

    if let Value::Content(Content::Shape { fill, stroke, .. }) = content {
        assert!(fill.is_none(), "rect sem fill deve ter fill: None");
        let s = stroke.expect("rect sem cores deve ter stroke de fallback");
        // Verificar que a cor é preta — adaptar à variante real de Color
        let (r, g, b) = color_to_rgb(&s.paint);
        assert_eq!((r, g, b), (0, 0, 0), "stroke de fallback deve ser preta");
        assert_eq!(s.thickness, 1.0, "espessura de fallback deve ser 1pt");
    } else {
        panic!("Esperado Content::Shape");
    }
}
```

### Teste L3 — coordenadas Y da linha no PDF

```rust
#[test]
fn line_coordenada_y_fim_inferior_ao_inicio() {
    // #line(dx: 100, dy: 50) — dy positivo = desce no layout.
    // No espaço PDF (Y cresce para cima), end_y < start_y.
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "#line(dx: 100pt, dy: 50pt)").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);
    let pdf    = export_pdf(&doc);

    let pdf_str = String::from_utf8_lossy(&pdf);

    // Extrair as coordenadas Y dos operadores m e l.
    // O operador m define o ponto inicial, l define o ponto final.
    // Num PDF com apenas esta linha, a primeira ocorrência de "m" e "l"
    // pertencem a esta forma.
    //
    // Verificação: a linha contém " m\n" e " l\n", e a Y do l é menor
    // que a Y do m (fim mais baixo no espaço PDF = dy positivo).
    assert!(pdf_str.contains(" m\n"), "PDF deve conter operador m");
    assert!(pdf_str.contains(" l\n"), "PDF deve conter operador l");

    // Extrair os valores Y de m e l para comparação numérica.
    // Adaptar o parsing conforme o formato exacto gerado pelo exportador.
    let m_y = extrair_y_operador(&pdf_str, " m\n");
    let l_y = extrair_y_operador(&pdf_str, " l\n");
    assert!(l_y < m_y,
        "Y do ponto final ({}) deve ser inferior ao Y do início ({}) — dy positivo desce no layout, subtrai no PDF",
        l_y, m_y);
}
```

### Teste L3 — ordem obrigatória dos operadores no PDF

```rust
#[test]
fn rect_ordem_operadores_pdf() {
    // #rect(fill: "red", stroke: "black") deve produzir:
    // q → rg (fill) → RG (stroke) → w (largura) → re (path) → B (paint) → Q
    // A ordem é obrigatória — PDF processa o stack de forma sequencial.
    let root = criar_dir_temporario();
    std::fs::write(
        root.join("main.typ"),
        "#rect(width: 100pt, height: 50pt, fill: \"red\", stroke: \"black\")",
    ).unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);
    let pdf    = export_pdf(&doc);

    let pdf_str = String::from_utf8_lossy(&pdf);

    // Verificar presença de todos os operadores.
    assert!(pdf_str.contains("q\n"),  "PDF deve ter push state (q)");
    assert!(pdf_str.contains(" rg\n"), "PDF deve ter operador de fill (rg)");
    assert!(pdf_str.contains(" RG\n"), "PDF deve ter operador de stroke (RG)");
    assert!(pdf_str.contains(" w\n"),  "PDF deve ter operador de espessura (w)");
    assert!(pdf_str.contains(" re\n"), "PDF deve ter operador de rectângulo (re)");
    assert!(pdf_str.contains("B\n"),   "PDF deve ter paint operator B (fill+stroke)");
    assert!(pdf_str.contains("Q\n"),   "PDF deve ter pop state (Q)");

    // Verificar a ordem relativa — cada operador deve aparecer antes do seguinte.
    let pos_q   = pdf_str.find("q\n").unwrap();
    let pos_rg  = pdf_str.find(" rg\n").unwrap();
    let pos_rg_upper = pdf_str.find(" RG\n").unwrap();
    let pos_re  = pdf_str.find(" re\n").unwrap();
    let pos_b   = pdf_str.find("B\n").unwrap();
    let pos_q_close = pdf_str.rfind("Q\n").unwrap();

    assert!(pos_q   < pos_rg,       "q deve preceder rg");
    assert!(pos_rg  < pos_rg_upper, "rg (fill) deve preceder RG (stroke)");
    assert!(pos_rg_upper < pos_re,  "RG deve preceder re");
    assert!(pos_re  < pos_b,        "re deve preceder B");
    assert!(pos_b   < pos_q_close,  "B deve preceder Q final");
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
- [ ] `Stroke` e `ShapeKind` definidos em `entities/geometry.rs` e exportados
  via `entities/mod.rs`. `Stroke.paint` é do tipo `Color` — sem `RgbaColor`.
- [ ] `Color` não é redefinido — reutilizado de `layout_types.rs`.
- [ ] `color_to_rgb` definido no exportador; extrai canais RGB de todas as
  variantes de `Color`. Alpha descartado com comentário explicativo.
- [ ] `Content::Shape` adicionado ao AST com `fill: Option<Color>`. Todos os `match` actualizados.
- [ ] `native_rect` e `native_line` registados na stdlib.
- [ ] O fallback de stroke preta 1pt existe **apenas** em `native_rect` —
  não no layouter nem no exportador.
- [ ] `FrameItem::Shape` com campos concretos (`f64`, `Option<Color>`).
  Todos os `match` actualizados.
- [ ] O layouter calcula a bounding box de `Line` com `abs()` dos deltas.
- [ ] O layouter verifica quebra de página antes de emitir `FrameItem::Shape`.
- [ ] O exportador emite `q → cores → path → paint → Q` nesta ordem exacta.
- [ ] `Ellipse` no exportador emite um rectângulo placeholder com comentário
  `TODO` a apontar DEBT-31.
- [ ] Linha: `end_y = page_height - (pos.y.0 + dy)` com comentário
  `// dy positivo = desce no layout → subtrai no PDF`.
- [ ] DEBT-30, DEBT-31, e DEBT-32 registados em `01_core/DEBT.md`.
  DEBT-32 descreve o problema de bounding box para linhas com deltas negativos.
- [ ] Teste `rect_sem_cores_tem_stroke_preta_1pt` passa.
- [ ] Teste `line_coordenada_y_fim_inferior_ao_inicio` passa.
- [ ] Teste `rect_ordem_operadores_pdf` passa.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Como `pos` é armazenada em `FrameItem` — tuple `(Point, FrameItem)` ou
  campo interno. Determina a sintaxe usada no exportador (`pos.x.0` vs outro).
- Tipo de `page_height` no exportador — `f64`, `Pt`, ou outro. Afecta as
  fórmulas de inversão do eixo Y.

**Da implementação:**
- Se `resolve_pt` foi uma função nova ou se o padrão existente para imagens
  já tinha um equivalente reutilizável.
- Se algum leitor PDF (Adobe Acrobat, Chrome, Evince) foi testado
  manualmente com o PDF gerado — confirmar que formas aparecem visualmente.
- Número total de testes após o passo e zero violations confirmados.

**Go/No-Go para o Passo 77:**
- **GO — primitivas visíveis no PDF:** rectângulos e linhas aparecem com as
  cores correctas num leitor PDF real.
- **NO-GO — ordem de operadores incorrecta:** se o PDF falha ao abrir ou
  mostra formas sem cor, verificar que `rg`/`RG` aparecem antes de `re` e
  que `B`/`f`/`S` aparecem antes de `Q`.
- **NO-GO — bounding box errada para linhas:** se `layout_image_gera_frameitem`
  ou testes de cursor falham após este passo, verificar que a bounding box
  de `Line` usa `abs()` e não os valores brutos de `dx`/`dy`.
