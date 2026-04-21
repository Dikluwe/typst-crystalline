# Passo 78 — Transformações Afins e Bounding Boxes Dinâmicas (DEBT-31)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/layout_types.rs` — Onde `TransformMatrix` e `Point`
  viverão.
- `01_core/src/entities/content.rs` — Onde a variante `Transform` será
  adicionada.
- `01_core/src/entities/value.rs` — Confirmar se `Value::Angle` já existe e
  como expõe o valor em radianos (diagnóstico 1).
- `03_infra/src/export.rs` — Onde o operador `cm` será introduzido e o
  desenho será dividido em `draw_item_global` / `draw_item_local`.
- `01_core/DEBT.md` — Confirmar que DEBT-31 está registado.

Pré-condição: `cargo test` — 721 L1 + 147 L3, zero violations. DEBT-31
registado. Elipses e linhas com deltas negativos a funcionar (Passo 77).

---

## Contexto

As funções `#move`, `#scale` e `#rotate` do Typst aplicam transformações
geométricas ao conteúdo que envolvem. Este passo resolve dois problemas
arquitecturais:

**O desafio do layouter — AABB:** quando um rectângulo de 100×100 é rodado
45°, a bounding box que o contém cresce para ~141×141 (a diagonal). O layouter
precisa de projectar os quatro cantos da bounding box original através da
matriz e calcular o novo mínimo/máximo para reservar o espaço correcto na
página.

**O desafio do PDF — operador `cm`:** o PDF aplica transformações via a matriz
`cm` ([a b c d tx ty]). O layouter usa Y crescente para baixo; o PDF usa Y
crescente para cima. Esta inversão tem de ser resolvida na matriz `cm` sem
que os elementos filhos a apliquem uma segunda vez — daí a divisão entre
`draw_item_global` e `draw_item_local`.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar se Value::Angle existe e como expõe o valor numérico
grep -n "Angle\|Ratio\|to_radians\|to_deg" \
  01_core/src/entities/value.rs | head -10

# 2. Localizar onde os FrameItems são desenhados no exportador
grep -n "match item\|draw_item\|FrameItem::" 03_infra/src/export.rs | head -15

# 3. Confirmar a estrutura actual do loop de exportação de páginas
# (necessário para perceber onde draw_item_global e draw_item_local encaixam)
grep -n "for.*item\|page_stream\|page_height" \
  03_infra/src/export.rs | head -15
```

Reportar o output completo antes de continuar. O diagnóstico 1 é crítico
para a Tarefa 2: se `Value::Angle` não existe, o ângulo chega como
`Value::Float` em radianos e a conversão é trivial; se existe, a conversão
usa o método que o diagnóstico revelar.

---

## Tarefa 0 — Actualizar DEBT.md

Marcar DEBT-31 como `EM CURSO` antes de qualquer código:

```markdown
### DEBT-31 — Transformações afins (rotate, scale) em nós — EM CURSO (Passo 78)
```

Será marcado `ENCERRADO ✓` no final da Tarefa 5.

---

## Tarefa 1 — `TransformMatrix` em L1

Em `01_core/src/entities/layout_types.rs` (ou `geometry.rs`), adicionar
a estrutura de matriz afim 2D:

```rust
/// Matriz de transformação afim 2D: [a, b, c, d, tx, ty].
///
/// Representa a transformação:
///   x' = a*x + c*y + tx
///   y' = b*x + d*y + ty
///
/// Esta convenção segue o formato do operador `cm` do PDF.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformMatrix {
    pub a: f64, pub b: f64,
    pub c: f64, pub d: f64,
    pub tx: f64, pub ty: f64,
}

impl Default for TransformMatrix {
    fn default() -> Self { Self::identity() }
}

impl TransformMatrix {
    pub fn identity() -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0 }
    }

    pub fn translate(dx: f64, dy: f64) -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: dx, ty: dy }
    }

    pub fn scale(sx: f64, sy: f64) -> Self {
        Self { a: sx, b: 0.0, c: 0.0, d: sy, tx: 0.0, ty: 0.0 }
    }

    pub fn rotate(radians: f64) -> Self {
        let cos = radians.cos();
        let sin = radians.sin();
        // Convenção Y-down (sistema do layouter):
        //   x' =  cos*x - sin*y
        //   y' =  sin*x + cos*y
        Self { a: cos, b: sin, c: -sin, d: cos, tx: 0.0, ty: 0.0 }
    }

    /// Compõe esta matriz com `other`: o resultado aplica `other` primeiro,
    /// depois `self`.
    ///
    /// Atenção: composição de matrizes não é comutativa.
    /// `m.concat(rotate).concat(scale)` ≠ `m.concat(scale).concat(rotate)`.
    /// A ordem de chamada determina a ordem de aplicação das transformações.
    pub fn concat(&self, other: &Self) -> Self {
        Self {
            a:  self.a * other.a  + self.c * other.b,
            b:  self.b * other.a  + self.d * other.b,
            c:  self.a * other.c  + self.c * other.d,
            d:  self.b * other.c  + self.d * other.d,
            tx: self.a * other.tx + self.c * other.ty + self.tx,
            ty: self.b * other.tx + self.d * other.ty + self.ty,
        }
    }

    /// Aplica a matriz a um ponto 2D.
    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.a * x + self.c * y + self.tx,
            self.b * x + self.d * y + self.ty,
        )
    }
}
```

Expor em `entities/mod.rs` se `layout_types` ainda não for re-exportado
globalmente.

---

## Tarefa 2 — AST e stdlib (L1)

### 2a — `Content::Transform`

Em `01_core/src/entities/content.rs`:

```rust
// Na enum Content:
/// Aplica uma transformação afim ao conteúdo interno.
///
/// O layouter calcula a AABB do conteúdo transformado e reserva o espaço
/// correcto na página. O exportador emite q → cm → conteúdo → Q.
Transform {
    matrix: TransformMatrix,
    body:   Box<Content>,
},
```

Actualizar todos os `match` sobre `Content` — adicionar
`Content::Transform { .. } => {}` nos braços que não tratam transformações
(introspecção, show rules, etc.).

### 2b — `FrameItem::Group`

Em `01_core/src/rules/layout/frame.rs`:

```rust
// Na enum FrameItem:
/// Grupo de itens com transformação afim aplicada.
///
/// O exportador emite q → cm → itens filhos (em espaço local) → Q.
/// Os itens filhos NÃO devem subtrair page_height — a matriz cm já
/// tratou a inversão Y. Ver draw_item_local no exportador.
Group {
    matrix: TransformMatrix,
    items:  Vec<(Point, FrameItem)>,
},
```

### 2c — Funções nativas na stdlib

Em `01_core/src/rules/stdlib.rs`:

```rust
pub fn native_move(_ctx: &mut EvalContext, args: &Args) -> Result<Value, String> {
    let dx = args.named::<f64>("dx").unwrap_or(0.0);
    let dy = args.named::<f64>("dy").unwrap_or(0.0);

    let body = args.positional_content()
        .ok_or_else(|| "move() exige um bloco de conteúdo".to_string())?;

    Ok(Value::Content(Content::Transform {
        matrix: TransformMatrix::translate(dx, dy),
        body:   Box::new(body),
    }))
}

pub fn native_rotate(_ctx: &mut EvalContext, args: &Args) -> Result<Value, String> {
    // Extrair o ângulo com conversão explícita de graus para radianos.
    // Adaptar ao resultado do diagnóstico 1 — pode ser Value::Angle ou Value::Float.
    let angle_rad = match args.named::<Value>("angle") {
        Some(Value::Angle(a)) => a.to_radians(), // se Value::Angle existe
        Some(Value::Float(f)) => f,              // fallback: valor já em radianos
        _                     => 0.0,
    };

    let body = args.positional_content()
        .ok_or_else(|| "rotate() exige um bloco de conteúdo".to_string())?;

    Ok(Value::Content(Content::Transform {
        matrix: TransformMatrix::rotate(angle_rad),
        body:   Box::new(body),
    }))
}

pub fn native_scale(_ctx: &mut EvalContext, args: &Args) -> Result<Value, String> {
    let sx = args.named::<f64>("x").unwrap_or(1.0);
    let sy = args.named::<f64>("y").unwrap_or(sx); // escala uniforme se y omitido

    let body = args.positional_content()
        .ok_or_else(|| "scale() exige um bloco de conteúdo".to_string())?;

    Ok(Value::Content(Content::Transform {
        matrix: TransformMatrix::scale(sx, sy),
        body:   Box::new(body),
    }))
}
```

Registar as três funções:

```rust
ctx.register("move",   native_move);
ctx.register("rotate", native_rotate);
ctx.register("scale",  native_scale);
```

---

## Tarefa 3 — AABB dinâmica no layouter (L1)

Em `01_core/src/rules/layout/mod.rs`, processar `Content::Transform`:

```rust
Content::Transform { matrix, body } => {
    // 1. Layoutar o corpo interno num frame isolado para obter as suas
    //    dimensões sem afectar o cursor da página principal.
    let inner_frame = self.layout_sub_frame(body);
    // layout_sub_frame deve retornar um frame com width, height, e items.
    // Adaptar ao método de layout isolado existente no layouter.

    let orig_w = inner_frame.width.0;
    let orig_h = inner_frame.height.0;

    // 2. Projectar os quatro cantos da bounding box original através da matriz.
    // Os quatro cantos definem completamente o rectângulo antes da transformação.
    let corners = [
        matrix.apply(0.0,    0.0),
        matrix.apply(orig_w, 0.0),
        matrix.apply(0.0,    orig_h),
        matrix.apply(orig_w, orig_h),
    ];

    // 3. Calcular a AABB (Axis-Aligned Bounding Box) da forma transformada.
    // min_x/max_x/min_y/max_y são os extremos nos eixos alinhados.
    let min_x = corners.iter().map(|(x, _)| *x).fold(f64::INFINITY,  f64::min);
    let max_x = corners.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = corners.iter().map(|(_, y)| *y).fold(f64::INFINITY,  f64::min);
    let max_y = corners.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    let new_w = max_x - min_x;
    let new_h = max_y - min_y;

    // 4. Verificar quebra de página com a nova altura.
    if self.cursor_y.0 + new_h > self.page_height.0 - self.margin.0 {
        self.new_page();
    }
    self.flush_line();

    let pos = Point { x: self.cursor_x, y: self.cursor_y };

    // 5. Compensar a origem negativa.
    // Se min_x ou min_y forem negativos (ex: rotação que move cantos para
    // coordenadas negativas), a forma sairia da bounding box reservada.
    // A translação de compensação garante que o canto mais à esquerda/acima
    // da forma transformada coincide com pos.
    let align = TransformMatrix::translate(-min_x, -min_y);
    let final_matrix = align.concat(matrix);

    self.push_frame_item(pos, FrameItem::Group {
        matrix: final_matrix,
        items:  inner_frame.items,
    });

    self.cursor_y += Pt(new_h);
},
```

---

## Tarefa 4 — Operador `cm` no exportador (L3)

### 4a — Dividir o desenho em `draw_item_global` e `draw_item_local`

Esta divisão é obrigatória para evitar a dupla inversão de Y.

- **`draw_item_global`**: o comportamento actual. Usa
  `pdf_y = page_height - pos.y.0 - height` para inverter Y.
- **`draw_item_local`**: usado dentro de um `Group`. Como a matriz `cm`
  já posicionou e inverteu o sistema de coordenadas, os filhos usam
  `pos.y.0` directamente sem subtrair `page_height`.

Se o exportador actual usa uma função ou método para desenhar cada item,
extrair duas variantes. Se o código está inline num loop, criar duas funções
auxiliares que diferem apenas na fórmula de Y.

### 4b — Braço `FrameItem::Group`

```rust
FrameItem::Group { matrix, items } => {
    // Guardar o estado gráfico antes de aplicar a transformação.
    // A matriz cm afecta todos os elementos desenhados até ao Q correspondente.
    page_stream.push_str("q\n");

    // Calcular a posição do grupo no espaço PDF (inversão Y global).
    // pdf_y é o canto inferior da bounding box do grupo na página.
    let pdf_y = page_height - pos.y.0;

    // Emitir a matriz cm com compensação do eixo Y entre layouter e PDF.
    //
    // O layouter usa Y-down; o PDF usa Y-up. Para que a rotação funcione
    // correctamente no PDF, os componentes de cisalhamento (b e c) têm
    // de ser invertidos:
    //   b_pdf = -b_layout   (cisalhamento y afecta x no sentido oposto)
    //   c_pdf = -c_layout   (cisalhamento x afecta y no sentido oposto)
    //
    // A translação ty também é invertida: pdf usa Y-up, layout usa Y-down.
    page_stream.push_str(&format!(
        "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} cm\n",
        matrix.a,
        -matrix.b,               // inversão Y para componente de rotação
        -matrix.c,               // inversão Y para componente de rotação
        matrix.d,
        pos.x.0 + matrix.tx,
        pdf_y   - matrix.ty,     // inversão da translação Y
    ));

    // Desenhar os itens filhos em espaço LOCAL.
    // A matriz cm já aplicou a transformação e a inversão Y.
    // Os filhos NÃO devem subtrair page_height — fazê-lo causaria
    // dupla inversão e projectaria os elementos fora da página.
    for (child_pos, child_item) in items {
        draw_item_local(page_stream, child_pos, child_item, /* parâmetros locais */);
    }

    // Restaurar o estado gráfico — a matriz cm deixa de ter efeito.
    page_stream.push_str("Q\n");
},
```

---

## Tarefa 5 — Testes

### Teste L1 — AABB de rotação 90° não altera dimensões de um quadrado

```rust
#[test]
fn transform_matrix_rotacao_90_graus_quadrado_mantem_dimensoes() {
    let matrix = TransformMatrix::rotate(std::f64::consts::FRAC_PI_2); // 90°

    // Quadrado 100×100. Após rotação de 90°, a AABB continua 100×100.
    // Cantos originais: (0,0), (100,0), (0,100), (100,100)
    // Após rotação 90° (cos=0, sin=1): (0,0), (0,100), (-100,0), (-100,100)
    // min_x = -100, max_x = 0, min_y = 0, max_y = 100
    // new_w = 100, new_h = 100
    let corners = [
        matrix.apply(0.0,   0.0),
        matrix.apply(100.0, 0.0),
        matrix.apply(0.0,   100.0),
        matrix.apply(100.0, 100.0),
    ];

    let min_x = corners.iter().map(|(x, _)| *x).fold(f64::INFINITY,     f64::min);
    let max_x = corners.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = corners.iter().map(|(_, y)| *y).fold(f64::INFINITY,     f64::min);
    let max_y = corners.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    let new_w = max_x - min_x;
    let new_h = max_y - min_y;

    assert!((new_w - 100.0).abs() < 0.001,
        "Quadrado 100×100 rodado 90° deve ter largura 100, obteve {}", new_w);
    assert!((new_h - 100.0).abs() < 0.001,
        "Quadrado 100×100 rodado 90° deve ter altura 100, obteve {}", new_h);
}
```

### Teste L1 — AABB de rotação 45° aumenta para a diagonal

```rust
#[test]
fn transform_matrix_rotacao_45_graus_aumenta_bounding_box() {
    let matrix = TransformMatrix::rotate(std::f64::consts::FRAC_PI_4); // 45°

    // Quadrado 100×100 rodado 45°.
    // A diagonal de um quadrado de lado L é L * √2 ≈ 141.42.
    // Tanto new_w como new_h devem ser aproximadamente 141.42.
    let corners = [
        matrix.apply(0.0,   0.0),
        matrix.apply(100.0, 0.0),
        matrix.apply(0.0,   100.0),
        matrix.apply(100.0, 100.0),
    ];

    let min_x = corners.iter().map(|(x, _)| *x).fold(f64::INFINITY,     f64::min);
    let max_x = corners.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = corners.iter().map(|(_, y)| *y).fold(f64::INFINITY,     f64::min);
    let max_y = corners.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    let new_w = max_x - min_x;
    let new_h = max_y - min_y;

    let diagonal = 100.0_f64 * std::f64::consts::SQRT_2; // ≈ 141.42

    assert!((new_w - diagonal).abs() < 0.01,
        "Quadrado 100×100 rodado 45° deve ter largura ≈ {:.2}, obteve {:.4}",
        diagonal, new_w);
    assert!((new_h - diagonal).abs() < 0.01,
        "Quadrado 100×100 rodado 45° deve ter altura ≈ {:.2}, obteve {:.4}",
        diagonal, new_h);
}
```

### Teste L1 — `concat` aplica `other` antes de `self`

```rust
#[test]
fn transform_matrix_concat_ordem_correta() {
    // Transladar 10 em X, depois rodar 90°.
    // Se concat aplica other primeiro: ponto (0,0) → translate → (10,0) → rotate90 → (0,10)
    // Se a ordem estivesse invertida:  ponto (0,0) → rotate90 → (0,0) → translate → (10,0)
    let translate = TransformMatrix::translate(10.0, 0.0);
    let rotate90  = TransformMatrix::rotate(std::f64::consts::FRAC_PI_2);

    // rotate90.concat(translate): aplica translate primeiro, depois rotate90
    let composed = rotate90.concat(&translate);
    let (rx, ry) = composed.apply(0.0, 0.0);

    // Esperado: (0, 10) — translate move para (10,0), rotate90 move para (0,10)
    assert!((rx - 0.0).abs() < 0.001, "x esperado 0.0, obteve {}", rx);
    assert!((ry - 10.0).abs() < 0.001, "y esperado 10.0, obteve {}", ry);
}
```

### Teste L3 — exportador emite `q`, `cm`, `Q` para transformações

```rust
#[test]
fn pdf_export_emite_q_cm_q_para_transformacoes() {
    let root = criar_dir_temporario();
    std::fs::write(
        root.join("main.typ"),
        "#rotate(90deg, rect(width: 100pt, height: 100pt, fill: \"red\"))",
    ).unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);
    let pdf    = export_pdf(&doc);

    let pdf_str = String::from_utf8_lossy(&pdf);

    assert!(pdf_str.contains("q\n"),   "Falta guardar o estado gráfico (q)");
    assert!(pdf_str.contains(" cm\n"), "Falta a matriz de transformação (cm)");
    assert!(pdf_str.contains("Q\n"),   "Falta restaurar o estado gráfico (Q)");

    // A ordem q → cm → Q deve ser respeitada.
    let pos_q  = pdf_str.find("q\n").unwrap();
    let pos_cm = pdf_str.find(" cm\n").unwrap();
    let pos_q_close = pdf_str.rfind("Q\n").unwrap();

    assert!(pos_q < pos_cm,       "q deve preceder cm");
    assert!(pos_cm < pos_q_close, "cm deve preceder Q");
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
- [ ] `TransformMatrix` implementada em `layout_types.rs` com `identity`,
  `translate`, `scale`, `rotate`, `concat`, `apply`.
- [ ] Docstring de `concat` explica que `other` é aplicado antes de `self`.
- [ ] `Content::Transform` adicionado ao AST. Todos os `match` actualizados.
- [ ] `FrameItem::Group` adicionado à display list. Todos os `match` actualizados.
- [ ] `native_move`, `native_rotate`, `native_scale` implementadas e registadas.
- [ ] `native_rotate` usa `Value::Angle` se existir; fallback explícito para
  `Value::Float` com comentário — sem conversão silenciosa.
- [ ] O layouter projecta os quatro cantos e calcula a AABB com `f64::min`/
  `f64::max` (sem `fold` com valores iniciais incorrectos).
- [ ] A translação de compensação `translate(-min_x, -min_y)` é aplicada
  antes de `matrix` via `align.concat(matrix)`.
- [ ] O exportador tem `draw_item_global` e `draw_item_local` separados.
  `draw_item_local` não subtrai `page_height`.
- [ ] Braço `FrameItem::Group` emite `q → cm → filhos (local) → Q`.
- [ ] Os componentes `b` e `c` da matriz `cm` são invertidos no exportador.
- [ ] DEBT-31 marcado como **ENCERRADO ✓** em `01_core/DEBT.md`.
- [ ] Teste `transform_matrix_rotacao_90_graus_quadrado_mantem_dimensoes` passa.
- [ ] Teste `transform_matrix_rotacao_45_graus_aumenta_bounding_box` passa.
- [ ] Teste `transform_matrix_concat_ordem_correta` passa.
- [ ] Teste `pdf_export_emite_q_cm_q_para_transformacoes` passa.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `Value::Angle` existe e como expõe o valor numérico — determina a
  implementação de `native_rotate`.
- Como o exportador actual estrutura o loop de itens — determina se
  `draw_item_global`/`draw_item_local` são métodos, closures, ou funções
  livres.

**Da implementação:**
- Se a inversão de `b` e `c` na matriz `cm` foi suficiente para rotações
  correctas, ou se foi necessário ajuste adicional na fórmula de `ty`.
- Se `layout_sub_frame` já existia ou foi adicionado agora para suportar
  o layout isolado do corpo de `Content::Transform`.
- Número total de testes após o passo e zero violations confirmados.

**Go/No-Go para o Passo 79:**
- **GO — transformações visíveis no PDF:** rectângulo rodado 90° aparece
  correctamente orientado num leitor PDF real sem sair da página.
- **NO-GO — dupla inversão Y:** se o conteúdo do grupo aparece espelhado
  verticalmente ou fora da página, `draw_item_local` ainda está a subtrair
  `page_height`. Verificar que os filhos de `Group` usam a variante local.
- **NO-GO — AABB incorrecta:** se a página fica em branco após rotação, o
  cursor avançou com `new_h = 0` — verificar que `layout_sub_frame` retorna
  as dimensões correctas antes de calcular os cantos.
