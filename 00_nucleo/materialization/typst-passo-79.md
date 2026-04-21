# Passo 79 — Polígonos, Caminhos Arbitrários e Clipping Paths (DEBT-30)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/geometry.rs` — Onde `ShapeKind` ganhará a variante
  `Path` e `PathItem` será definido.
- `01_core/src/rules/layout/frame.rs` — Onde `FrameItem::Group` ganhará o
  campo `clip_mask`.
- `03_infra/src/export.rs` — Onde `emit_shape_path` será dividido em
  `_global` e `_local`, e o operador `W n` será introduzido.
- `01_core/DEBT.md` — Confirmar que DEBT-30 está registado e DEBT-31
  encerrado.

Pré-condição: `cargo test` — ~721 L1 + ~147 L3, zero violations.
DEBT-30 registado. `FrameItem::Group` com `matrix` e `items` implementado
(Passo 78). `draw_item_local` existe como função separada.

---

## Contexto

O Passo 78 introduziu transformações afins e separou `draw_item_global` de
`draw_item_local`. Este passo aproveita essa separação para dois objectivos:

- **Caminhos livres (`ShapeKind::Path`):** polígonos e formas arbitrárias
  compostas por segmentos `MoveTo`, `LineTo`, `CubicTo`, e `ClosePath`. A
  bounding box é calculada pelos pontos de controlo (DEBT-33 regista que
  esta aproximação é conservadora para curvas Bézier).

- **Clipping paths (DEBT-30):** o operador `W n` do PDF restringe o desenho
  à área de uma forma. O clip tem de ocorrer no espaço local do `Group`
  (após `cm`), não no espaço global da página — razão pela qual
  `emit_shape_path_local` é essencial.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar como arrays/listas são extraídos de Args no avaliador
grep -n "Array\|Vec\|List" 01_core/src/entities/value.rs | head -10

# 2. Localizar o braço FrameItem::Group no exportador
grep -A 10 "FrameItem::Group" 03_infra/src/export.rs

# 3. Confirmar a assinatura actual de draw_item_local
grep -n "fn draw_item_local\|fn draw_item_global" 03_infra/src/export.rs | head -5

# 4. Confirmar como Point é importado em geometry.rs
grep -n "use.*Point\|use.*layout_types" \
  01_core/src/entities/geometry.rs | head -5
```

Reportar o output completo antes de continuar. O diagnóstico 1 determina
como `native_polygon` extrai os pontos dos argumentos posicionais — se
`Value::Array` existe com elementos acessíveis, ou se os pontos chegam
como argumentos individuais. O diagnóstico 3 confirma a assinatura de
`draw_item_local` para saber quais parâmetros passar na Tarefa 4.

---

## Tarefa 0 — Actualizar DEBT.md

Registar DEBT-33 e marcar DEBT-30 como `EM CURSO`:

```markdown
### DEBT-30 — Suporte a clipping paths (clip: true) — EM CURSO (Passo 79)

### DEBT-33 — Bounding Box de curvas Bézier (Passo 79) — EM ABERTO
A bounding box de ShapeKind::Path é calculada verificando o min/max dos pontos
de controlo. Para CubicTo, a curva real pode ultrapassar a caixa delimitadora
dos pontos de controlo, causando vazamento visual subtil. Resolução futura:
cálculo analítico dos extremos da curva paramétrica B(t) para obter a AABB exacta.
```

DEBT-30 será marcado `ENCERRADO ✓` no final da Tarefa 5.

---

## Tarefa 1 — `PathItem` e `ShapeKind::Path` (L1)

Em `01_core/src/entities/geometry.rs`, adicionar `PathItem` e a nova
variante de `ShapeKind`:

```rust
use crate::entities::layout_types::Point;

/// Segmento de um caminho vectorial.
#[derive(Debug, Clone, PartialEq)]
pub enum PathItem {
    /// Mover o cursor para o ponto sem traçar.
    MoveTo(Point),
    /// Traçar um segmento de recta até ao ponto.
    LineTo(Point),
    /// Curva de Bézier cúbica: dois pontos de controlo e o ponto final.
    CubicTo(Point, Point, Point),
    /// Fechar o sub-path com uma recta de volta ao último MoveTo.
    ClosePath,
}

// Adicionar à enum ShapeKind existente:
// Path(Vec<PathItem>),
```

Actualizar a enum `ShapeKind` para incluir:

```rust
/// Caminho geométrico livre — polígonos e formas arbitrárias.
///
/// A bounding box é calculada pelos pontos de controlo (DEBT-33:
/// pode ser conservadora para segmentos CubicTo).
Path(Vec<PathItem>),
```

Actualizar todos os `match` sobre `ShapeKind` nas camadas L1 e L3 — o
compilador lista os locais. Nos braços que não tratam `Path`, adicionar
`ShapeKind::Path(_) => {}` ou equivalente.

---

## Tarefa 2 — `native_polygon` na stdlib (L1)

Em `01_core/src/rules/stdlib.rs`, implementar `native_polygon`.

A função precisa de um helper `extract_coordinate` para converter um `Value`
em `(f64, f64)`. A implementação concreta depende do diagnóstico 1 — adaptar
ao formato real dos argumentos posicionais:

```rust
/// Extrai um par de coordenadas (x, y) de um Value.
/// Adaptar ao resultado do diagnóstico 1 — pode ser Value::Array com dois
/// elementos, ou uma tupla, ou outro formato.
fn extract_coordinate(val: &Value) -> Option<(f64, f64)> {
    match val {
        Value::Array(arr) if arr.len() == 2 => {
            let x = arr[0].as_f64()?; // adaptar ao método real
            let y = arr[1].as_f64()?;
            Some((x, y))
        },
        _ => None,
    }
}

pub fn native_polygon(_ctx: &mut EvalContext, args: &Args) -> Result<Value, String> {
    let mut path_items = Vec::new();
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for (i, val) in args.positional_items().iter().enumerate() {
        let (x, y) = extract_coordinate(val)
            .ok_or_else(|| format!("Argumento {} não é uma coordenada válida", i))?;

        if i == 0 {
            path_items.push(PathItem::MoveTo(Point { x: Pt(x), y: Pt(y) }));
        } else {
            path_items.push(PathItem::LineTo(Point { x: Pt(x), y: Pt(y) }));
        }

        // DEBT-33: tracking da bounding box pelos pontos de controlo.
        // Para CubicTo, a curva real pode sair destes limites.
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    // Protecção: polígono sem pontos é inválido.
    if path_items.is_empty() {
        return Err("polygon() requer pelo menos um ponto".to_string());
    }

    path_items.push(PathItem::ClosePath);

    let fill   = args.named::<Value>("fill").and_then(|v| parse_color(&v));
    let stroke = args.named::<Value>("stroke").and_then(|v| {
        parse_color(&v).map(|c| Stroke { paint: c, thickness: 1.0 })
    });

    // Bounding box: None se todos os pontos forem coincidentes.
    let width  = if max_x > min_x { Some(Value::Float(max_x - min_x)) } else { None };
    let height = if max_y > min_y { Some(Value::Float(max_y - min_y)) } else { None };

    Ok(Value::Content(Content::Shape {
        kind: ShapeKind::Path(path_items),
        width,
        height,
        fill,
        stroke,
    }))
}
```

Registar a função:

```rust
ctx.register("polygon", native_polygon);
```

---

## Tarefa 3 — `clip_mask` em `FrameItem::Group` (L1)

Em `01_core/src/rules/layout/frame.rs`, adicionar o campo `clip_mask` à
variante existente:

```rust
Group {
    matrix:    TransformMatrix,
    /// Forma que restringe o desenho visual à sua área interna.
    ///
    /// Se Some, o exportador emite o path da máscara seguido de `W n`
    /// (clip sem pintar) no espaço local do Group, após a matriz `cm`.
    /// Se None, sem recorte — comportamento do Passo 78.
    clip_mask: Option<ShapeKind>,
    items:     Vec<(Point, FrameItem)>,
},
```

Actualizar todos os locais que constroem `FrameItem::Group` — o compilador
lista onde `clip_mask` tem de ser passado. Nos locais do Passo 78, passar
`clip_mask: None` para preservar o comportamento existente.

---

## Tarefa 4 — Emissão global/local e operador `W n` (L3)

### 4a — `emit_shape_path_global`

Extrair o código actual de emissão de path do braço `FrameItem::Shape` para
uma função auxiliar. Esta função usa `page_height` para inverter Y:

```rust
/// Emite os operadores de path de uma forma no espaço GLOBAL da página.
///
/// Usa page_height para inverter o eixo Y (layout Y-down → PDF Y-up).
/// Chamada apenas fora de Groups.
fn emit_shape_path_global(
    page_stream: &mut String,
    kind:        &ShapeKind,
    pos:         Point,
    width:       f64,
    height:      f64,
    page_height: f64,
) {
    let pdf_y = page_height - pos.y.0 - height;

    match kind {
        ShapeKind::Rect => {
            page_stream.push_str(&format!(
                "{:.2} {:.2} {:.2} {:.2} re\n",
                pos.x.0, pdf_y, width, height,
            ));
        },
        ShapeKind::Path(items) => {
            for item in items {
                match item {
                    PathItem::MoveTo(p) => page_stream.push_str(&format!(
                        "{:.2} {:.2} m\n",
                        pos.x.0 + p.x.0,
                        page_height - (pos.y.0 + p.y.0),
                    )),
                    PathItem::LineTo(p) => page_stream.push_str(&format!(
                        "{:.2} {:.2} l\n",
                        pos.x.0 + p.x.0,
                        page_height - (pos.y.0 + p.y.0),
                    )),
                    PathItem::CubicTo(p1, p2, p3) => page_stream.push_str(&format!(
                        "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                        pos.x.0 + p1.x.0, page_height - (pos.y.0 + p1.y.0),
                        pos.x.0 + p2.x.0, page_height - (pos.y.0 + p2.y.0),
                        pos.x.0 + p3.x.0, page_height - (pos.y.0 + p3.y.0),
                    )),
                    PathItem::ClosePath => page_stream.push_str("h\n"),
                }
            }
        },
        // Ellipse e Line: manter a lógica actual do braço Shape, refactorizada aqui.
        _ => { /* manter lógica existente */ }
    }
}
```

### 4b — `emit_shape_path_local`

No espaço local de um `Group`, a matriz `cm` já tratou a translação e a
inversão de Y. Os filhos apenas invertem o sinal de Y sem subtrair
`page_height`:

```rust
/// Emite os operadores de path de uma forma no espaço LOCAL de um Group.
///
/// NÃO usa page_height. A matriz `cm` já inverteu o eixo Y e posicionou
/// a origem. Usar page_height aqui causaria dupla inversão.
fn emit_shape_path_local(
    page_stream: &mut String,
    kind:        &ShapeKind,
    width:       f64,
    height:      f64,
) {
    match kind {
        ShapeKind::Rect => {
            // No espaço local, a origem é (0, 0). Como o PDF usa Y-up,
            // a base do rectângulo fica em -height.
            page_stream.push_str(&format!(
                "0.00 {:.2} {:.2} {:.2} re\n",
                -height, width, height,
            ));
        },
        ShapeKind::Path(items) => {
            for item in items {
                match item {
                    // Apenas inverter Y. A posição absoluta é gerida pela matriz cm.
                    PathItem::MoveTo(p)  => page_stream.push_str(&format!(
                        "{:.2} {:.2} m\n", p.x.0, -p.y.0,
                    )),
                    PathItem::LineTo(p)  => page_stream.push_str(&format!(
                        "{:.2} {:.2} l\n", p.x.0, -p.y.0,
                    )),
                    PathItem::CubicTo(p1, p2, p3) => page_stream.push_str(&format!(
                        "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                        p1.x.0, -p1.y.0,
                        p2.x.0, -p2.y.0,
                        p3.x.0, -p3.y.0,
                    )),
                    PathItem::ClosePath => page_stream.push_str("h\n"),
                }
            }
        },
        _ => { /* Ellipse e Line: adaptar de forma análoga — origem 0,0 e Y invertido */ }
    }
}
```

### 4c — Braço `FrameItem::Group` actualizado com clip

```rust
FrameItem::Group { matrix, clip_mask, items } => {
    page_stream.push_str("q\n");

    let pdf_y = page_height - pos.y.0;

    // Aplicar a matriz de transformação.
    page_stream.push_str(&format!(
        "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} cm\n",
        matrix.a,
        -matrix.b,
        -matrix.c,
        matrix.d,
        pos.x.0 + matrix.tx,
        pdf_y   - matrix.ty,
    ));

    // Aplicar clipping path no espaço LOCAL (após cm).
    // W = definir clipping path; n = fechar sem pintar.
    // A ordem cm → W n → filhos é obrigatória: o clip tem de ocorrer no
    // espaço já transformado para que a máscara coincida com o conteúdo.
    if let Some(mask) = clip_mask {
        // A bounding box da máscara é a do Group transformado.
        // width e height aqui são as dimensões do Group antes da transformação
        // (a máscara é definida no espaço local, não no espaço da página).
        let mask_w = /* largura do Group antes da transformação — ver Tarefa 3 */;
        let mask_h = /* altura do Group antes da transformação */;
        emit_shape_path_local(page_stream, mask, mask_w, mask_h);
        page_stream.push_str("W n\n"); // clip sem pintar
    }

    // Desenhar filhos no espaço local.
    for (child_pos, child_item) in items {
        draw_item_local(page_stream, child_pos, child_item, /* parâmetros */);
    }

    page_stream.push_str("Q\n");
},
```

**Nota sobre as dimensões da máscara:** para que `emit_shape_path_local`
receba `mask_w` e `mask_h` correctos, o `FrameItem::Group` precisa de
armazenar as dimensões do frame interno (antes da transformação). Se já estão
disponíveis nos itens filhos, calcular `max_x - min_x` e `max_y - min_y`
a partir das suas posições. Se não, adicionar `inner_width: f64` e
`inner_height: f64` ao `FrameItem::Group` no Passo anterior — confirmar
com o diagnóstico 2 o que está disponível.

---

## Tarefa 5 — Testes

### Teste L1 — `polygon` sem pontos gera erro

```rust
#[test]
fn polygon_sem_pontos_gera_erro() {
    // Nenhum argumento posicional → path vazio → erro antes de ClosePath.
    let result = eval_expr("#polygon()"); // adaptar ao helper existente
    assert!(result.is_err(),
        "polygon() sem pontos deve retornar Err");
    let msg = result.unwrap_err();
    assert!(msg.contains("pelo menos um ponto"),
        "Mensagem de erro deve mencionar 'pelo menos um ponto', obteve: {}", msg);
}
```

### Teste L1 — `polygon` com um ponto gera `MoveTo` + `ClosePath`

```rust
#[test]
fn polygon_com_um_ponto_gera_moveto_e_closepath() {
    // Um ponto → MoveTo + ClosePath (triângulo degenerado — válido mas invisível).
    let result = eval_expr("#polygon((10, 20))").unwrap();

    if let Value::Content(Content::Shape { kind: ShapeKind::Path(items), .. }) = result {
        assert_eq!(items.len(), 2,
            "Um ponto deve gerar MoveTo + ClosePath");
        assert!(matches!(items[0], PathItem::MoveTo(_)),
            "Primeiro item deve ser MoveTo");
        assert!(matches!(items[1], PathItem::ClosePath),
            "Último item deve ser ClosePath");
    } else {
        panic!("Esperado Content::Shape com ShapeKind::Path");
    }
}
```

### Teste L3 — ordem dos operadores em `Group` com clip

```rust
#[test]
fn export_group_com_clip_verifica_ordem_operadores() {
    // Configurar um Group com clip_mask directamente na display list
    // para testar o exportador sem depender do parser de #clip.
    // Adaptar conforme o helper de testes existente para criar FrameItems.
    let root = criar_dir_temporario();
    std::fs::write(
        root.join("main.typ"),
        // Usar uma sintaxe que produza FrameItem::Group com clip_mask Some(...)
        // — adaptar ao que o Passo 79 suportar na stdlib.
        "#rect(width: 100pt, height: 100pt, fill: \"blue\")",
    ).unwrap();

    // ... setup world, eval, layout, export ...
    let pdf_str = String::from_utf8_lossy(&pdf);

    // Se o PDF contiver um Group com clip, verificar a ordem:
    if pdf_str.contains("W n\n") {
        let pos_cm    = pdf_str.find(" cm\n")
            .expect("Deve conter matriz de transformação");
        let pos_clip  = pdf_str.find("W n\n")
            .expect("Deve emitir operador de clip");
        let pos_child = pdf_str.find(" m\n").or_else(|| pdf_str.find(" re\n"))
            .expect("Deve conter operação de desenho do filho");
        let pos_q     = pdf_str.rfind("Q\n").unwrap();

        assert!(pos_cm    < pos_clip,  "cm deve preceder W n");
        assert!(pos_clip  < pos_child, "W n deve preceder o desenho dos filhos");
        assert!(pos_child < pos_q,     "filhos devem ser desenhados antes de Q");
    }
}
```

### Teste L3 — `Path` com `CubicTo` emite operador `c`

```rust
#[test]
fn export_path_com_cubicto_emite_operador_c() {
    // Criar um Content::Shape com ShapeKind::Path contendo CubicTo
    // directamente (sem stdlib) para testar o exportador de forma isolada.
    let path = vec![
        PathItem::MoveTo(Point { x: Pt(0.0),  y: Pt(0.0)  }),
        PathItem::CubicTo(
            Point { x: Pt(10.0), y: Pt(0.0)  },
            Point { x: Pt(20.0), y: Pt(10.0) },
            Point { x: Pt(20.0), y: Pt(20.0) },
        ),
        PathItem::ClosePath,
    ];

    // ... construir documento com este shape, exportar ...

    assert!(pdf_str.contains(" c\n"),
        "CubicTo deve emitir operador Bézier 'c' no PDF");
    assert!(pdf_str.contains("h\n"),
        "ClosePath deve emitir operador 'h' no PDF");
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
- [ ] `PathItem` definido em `geometry.rs` com `MoveTo`, `LineTo`, `CubicTo`,
  `ClosePath`.
- [ ] `ShapeKind::Path(Vec<PathItem>)` adicionado. Todos os `match` sobre
  `ShapeKind` actualizados.
- [ ] `native_polygon` implementada com protecção contra lista vazia e
  `ClosePath` automático no fim.
- [ ] `extract_coordinate` adapta ao formato real dos argumentos (diagnóstico 1).
- [ ] `FrameItem::Group` tem o campo `clip_mask: Option<ShapeKind>`. Todos
  os locais de construção actualizados com `clip_mask: None` para preservar
  comportamento do Passo 78.
- [ ] `emit_shape_path_global` usa `page_height` para inverter Y.
- [ ] `emit_shape_path_local` usa apenas `-p.y.0` sem `page_height`.
- [ ] `CubicTo` emitido como `c` em ambas as funções de emissão.
- [ ] Braço `FrameItem::Group` segue a ordem `q → cm → W n (se clip) → filhos → Q`.
- [ ] `W n` é emitido apenas quando `clip_mask` é `Some`.
- [ ] DEBT-30 marcado como **ENCERRADO ✓** em `01_core/DEBT.md`.
- [ ] DEBT-33 registado em `01_core/DEBT.md`.
- [ ] Teste `polygon_sem_pontos_gera_erro` passa.
- [ ] Teste `polygon_com_um_ponto_gera_moveto_e_closepath` passa.
- [ ] Teste `export_path_com_cubicto_emite_operador_c` passa.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Formato dos argumentos posicionais de `polygon` — `Value::Array` com
  dois elementos, ou outro formato. Determina `extract_coordinate`.
- Se `FrameItem::Group` já armazena `inner_width`/`inner_height` ou se
  foi necessário adicioná-los para a Tarefa 4c.

**Da implementação:**
- Se `emit_shape_path_local` para `Ellipse` e `Line` precisou de ajuste
  nas fórmulas além de `-p.y.0` (ex: o centro da elipse em espaço local).
- Se o teste `export_group_com_clip_verifica_ordem_operadores` foi possível
  sem stdlib de clip — ou se `clip_mask` só é activado via construção
  directa de `FrameItem::Group`.
- Número total de testes após o passo e zero violations confirmados.

**Go/No-Go para o Passo 80:**
- **GO — clip visível no PDF:** conteúdo fora da máscara não aparece num
  leitor PDF real; `W n` está presente no stream.
- **NO-GO — dupla inversão no clip:** se a máscara aparece deslocada do
  conteúdo, `emit_shape_path_local` ainda usa `page_height`. Verificar que
  usa apenas `-p.y.0`.
- **NO-GO — `CubicTo` sem operador `c`:** se o teste falha com "deve conter
  'c'", verificar que o braço `PathItem::CubicTo` está implementado em ambas
  as funções de emissão (global e local) e não cai no braço `_ => {}`.
