# Passo 82 — Alinhamento 2D e Posicionamento Fora-de-Fluxo (`#align` e `#place`)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/layout_types.rs` — Onde as estruturas de alinhamento
  serão adicionadas.
- `01_core/src/rules/layout/mod.rs` — Onde `resolve_alignment` será
  implementado e onde `flush_line` usa `line_start_x` (introduzido no Passo
  81.5). Confirmar que o campo existe.
- `01_core/src/rules/stdlib.rs` — Onde `native_align` e `native_place` serão
  adicionadas.
- `01_core/DEBT.md` — Confirmar estado dos DEBTs activos.

Pré-condição: `cargo test` — 894 testes (731 L1 + 163 L3), zero violations.
Passo 81.5 concluído. `flush_line` usa `line_start_x` para o ponto de
reinício — não hardcoda `page_config.margin`.

---

## Contexto

O motor de layout coloca conteúdo em fluxo linear: cada bloco ocupa o seu
espaço e o cursor avança. Este passo adiciona dois mecanismos que desviam
desse fluxo:

**`#align`** — o conteúdo mantém-se no fluxo mas a sua posição horizontal
(e ocasionalmente vertical) é deslocada dentro do espaço disponível.
O cursor avança após o bloco, como com qualquer elemento em fluxo.

**`#place`** — o conteúdo é ancorado a uma posição absoluta na página mas
não consome espaço. O cursor não avança. Usado para cabeçalhos, rodapés,
marcas de água, e notas de margem.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar que line_start_x existe no Layouter (introduzido no Passo 81.5)
grep -n "line_start_x" 01_core/src/rules/layout/mod.rs | head -5

# 2. Confirmar como layout_sub_frame_with_width ancora os itens internos
# CRÍTICO: se os itens são retornados com coordenadas relativas a (margin, margin)
# em vez de (0, 0), target_x acumula a margem duas vezes.
grep -A 15 "fn layout_sub_frame_with_width" \
  01_core/src/rules/layout/mod.rs | head -20

# 3. Confirmar como strings são extraídas de Value nos argumentos posicionais
grep -n "cast_string\|as_str\|String" \
  01_core/src/entities/value.rs | head -8

# 4. Confirmar a assinatura de flush_line após o Passo 81.5
grep -A 5 "fn flush_line" 01_core/src/rules/layout/mod.rs | head -8
```

Reportar o output completo antes de continuar. O diagnóstico 2 é crítico:
se `layout_sub_frame_with_width` inicia o cursor interno em `(margin, margin)`
e retorna itens com esse offset, `target_x + item_pos.x.0` soma a margem
duas vezes. O ideal é que sub-frames retornem itens ancorados em `(0, 0)`.
Se não for o caso, o cálculo de `target_x` tem de compensar subtraindo o
offset inicial do sub-frame.

---

## Tarefa 0 — Actualizar DEBT.md

```markdown
### DEBT-36 — Operadores simbólicos de alinhamento (center + bottom) — EM ABERTO (Passo 82)
align e place aceitam strings ("center", "top-right") porque o parser ainda
não suporta operadores de composição simbólica como center + bottom.
Resolução: quando o parser suportar Value::Align com composição, substituir
Align2D::from_string pelo parse directo da variante.

### DEBT-37 — Place relativo ao contentor pai — EM ABERTO (Passo 82)
Content::Place ancora às margens absolutas da página. O Typst suporta
place relativo ao bloco pai (ex: dentro de um grid, place ancora na célula).
Resolução: passar a área de âncora como parâmetro ao processar Place.
```

---

## Tarefa 1 — Estruturas de alinhamento (L1)

Em `01_core/src/entities/layout_types.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VAlign {
    Top,
    /// Centro vertical — 'Horizon' é o termo interno do Typst.
    Horizon,
    Bottom,
}

/// Alinhamento 2D composto por componentes horizontal e vertical opcionais.
///
/// Ambos `None` equivale a `Left + Top` (comportamento por omissão).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Align2D {
    pub h: Option<HAlign>,
    pub v: Option<VAlign>,
}

impl Align2D {
    /// Parse de uma string composta por partes separadas por '-'.
    ///
    /// Exemplos: "center", "top-right", "bottom", "horizon".
    /// Partes não reconhecidas são ignoradas silenciosamente (DEBT-36:
    /// quando o parser suportar Value::Align, substituir este método).
    pub fn from_string(s: &str) -> Self {
        let mut align = Align2D::default();
        for part in s.split('-') {
            match part {
                "left"    => align.h = Some(HAlign::Left),
                "center"  => align.h = Some(HAlign::Center),
                "right"   => align.h = Some(HAlign::Right),
                "top"     => align.v = Some(VAlign::Top),
                "horizon" => align.v = Some(VAlign::Horizon),
                "bottom"  => align.v = Some(VAlign::Bottom),
                _         => {},
            }
        }
        align
    }
}
```

---

## Tarefa 2 — `Content::Align` e `Content::Place` (L1)

Em `01_core/src/entities/content.rs`:

```rust
// Na enum Content:

/// Altera a posição do conteúdo dentro do espaço disponível no fluxo.
/// O cursor avança após o bloco — o espaço é consumido normalmente.
Align {
    alignment: Align2D,
    body:      Box<Content>,
},

/// Posiciona o conteúdo de forma absoluta na página sem consumir espaço.
/// O cursor não avança. Usado para cabeçalhos, rodapés e marcas de água.
///
/// DEBT-37: ancora às margens da página, não ao contentor pai.
Place {
    alignment: Align2D,
    dx:        f64, // deslocamento X adicional em pontos
    dy:        f64, // deslocamento Y adicional em pontos
    body:      Box<Content>,
},
```

Actualizar todos os `match` sobre `Content` — adicionar
`Content::Align { .. } | Content::Place { .. } => {}` nos braços que não
tratam alinhamento (introspecção, show rules, map_content).

---

## Tarefa 3 — `native_align` e `native_place` na stdlib (L1)

Em `01_core/src/rules/stdlib.rs`:

```rust
pub fn native_align(_ctx: &mut EvalContext, args: &Args) -> Result<Value, String> {
    // Primeiro argumento posicional: string de alinhamento.
    // Adaptar a extracção de string ao método real confirmado pelo diagnóstico 3.
    let position_str = args.positional_items()
        .first()
        .and_then(|v| v.cast_string())
        .unwrap_or_else(|| "left".to_string());

    let body = args.positional_items()
        .get(1)
        .and_then(|v| v.cast_content())
        .ok_or_else(|| "align() exige um bloco de conteúdo no 2º argumento".to_string())?;

    Ok(Value::Content(Content::Align {
        alignment: Align2D::from_string(&position_str),
        body:      Box::new(body),
    }))
}

pub fn native_place(_ctx: &mut EvalContext, args: &Args) -> Result<Value, String> {
    let position_str = args.positional_items()
        .first()
        .and_then(|v| v.cast_string())
        .unwrap_or_else(|| "top-left".to_string());

    let dx = args.named::<Value>("dx").and_then(|v| extract_pt(&v)).unwrap_or(0.0);
    let dy = args.named::<Value>("dy").and_then(|v| extract_pt(&v)).unwrap_or(0.0);

    let body = args.positional_items()
        .get(1)
        .and_then(|v| v.cast_content())
        .ok_or_else(|| "place() exige um bloco de conteúdo".to_string())?;

    Ok(Value::Content(Content::Place {
        alignment: Align2D::from_string(&position_str),
        dx,
        dy,
        body: Box::new(body),
    }))
}
```

Registar:

```rust
ctx.register("align", native_align);
ctx.register("place", native_place);
```

---

## Tarefa 4 — `resolve_alignment` e helpers no layouter (L1)

Em `01_core/src/rules/layout/mod.rs`, adicionar os campos e métodos:

### Campo `is_height_unconstrained`

```rust
pub struct Layouter<'a> {
    // ... campos existentes ...

    /// Indica que o contexto de layout actual não tem altura delimitada
    /// (ex: célula de grid Auto, box sem height explícito).
    ///
    /// Quando true, VAlign::Bottom e VAlign::Horizon em Content::Align
    /// decaem para VAlign::Top — não existe "fundo" para ancorar.
    /// Definido como true por layout_sub_frame_with_width e restaurado
    /// ao regressar ao contexto pai.
    pub is_height_unconstrained: bool,
}
```

`layout_sub_frame_with_width` define `is_height_unconstrained = true` antes
de layoutar o corpo e restaura o valor anterior ao terminar.

### Helper `page_bottom_limit`

```rust
/// Limite inferior da página em pontos (height - margin).
///
/// Usar este método em vez de `page_config.height - page_config.margin`
/// inline — evita confundir com available_height() (que subtrai 2×margin).
fn page_bottom_limit(&self) -> f64 {
    self.page_config.height - self.page_config.margin
}
```

### Método `resolve_alignment`

```rust
/// Calcula a coordenada (X, Y) do canto superior esquerdo de um item
/// dado o alinhamento, as dimensões do conteúdo, e a área disponível.
///
/// `origin_x` e `origin_y` definem o canto superior esquerdo da área
/// de referência (line_start_x para Align; line_start_x/margin para Place).
fn resolve_alignment(
    &self,
    align:       Align2D,
    content_w:   f64,
    content_h:   f64,
    available_w: f64,
    available_h: f64,
    origin_x:    f64,
    origin_y:    f64,
) -> (f64, f64) {
    let x = match align.h.unwrap_or(HAlign::Left) {
        HAlign::Left   => origin_x,
        HAlign::Center => origin_x + (available_w - content_w) / 2.0,
        HAlign::Right  => origin_x + (available_w - content_w),
    };

    let y = match align.v.unwrap_or(VAlign::Top) {
        VAlign::Top     => origin_y,
        VAlign::Horizon => origin_y + (available_h - content_h) / 2.0,
        VAlign::Bottom  => origin_y + (available_h - content_h),
    };

    (x, y)
}
```

---

## Tarefa 5 — Processar `Content::Align` e `Content::Place` (L1)

```rust
Content::Align { alignment, body } => {
    // Garantir que não há texto inline pendente antes de posicionar o bloco.
    // flush_line usa line_start_x (Passo 81.5) — não hardcoda page_config.margin.
    self.flush_line();

    let avail_w = self.available_width();

    let sub_frame = self.layout_sub_frame_with_width(body, avail_w);
    // layout_sub_frame_with_width define is_height_unconstrained = true
    // e restaura o valor anterior ao terminar.

    // Verificar quebra de página com a altura do sub-frame.
    if self.cursor_y.0 + sub_frame.height.0 > self.page_bottom_limit() {
        self.new_page();
    }

    // Se estamos num contexto sem altura delimitada (grid Auto, box sem height),
    // VAlign::Bottom e VAlign::Horizon não têm "fundo" para ancorar —
    // decaem para Top e remaining_h é a altura do próprio conteúdo.
    let effective_v = if self.is_height_unconstrained {
        None // decai para VAlign::Top
    } else {
        alignment.v
    };

    let remaining_h = if self.is_height_unconstrained {
        sub_frame.height.0 // sem espaço extra: conteúdo ocupa exactamente a sua altura
    } else {
        f64::max(0.0, self.page_bottom_limit() - self.cursor_y.0)
    };

    // origin_x = line_start_x (não page_config.margin).
    // Dentro de uma célula de grid, line_start_x é cell_x, não a margem da página.
    // Usar page_config.margin aqui deslocaria o conteúdo para fora da célula.
    //
    // NOTA (diagnóstico 2): se layout_sub_frame_with_width retornar itens
    // com coordenadas relativas a (margin, margin) em vez de (0, 0),
    // subtrair o offset inicial aqui: origin_x = self.line_start_x.0 - sub_frame_origin_x.
    let effective_align = Align2D {
        h: alignment.h,
        v: effective_v,
    };

    let (target_x, target_y) = self.resolve_alignment(
        effective_align,
        sub_frame.width.0,
        sub_frame.height.0,
        avail_w,
        remaining_h,
        self.line_start_x.0,  // CORRIGIDO: era self.page_config.margin
        self.cursor_y.0,
    );

    for (item_pos, item) in sub_frame.items {
        self.push_frame_item(
            Point {
                x: Pt(target_x + item_pos.x.0),
                y: Pt(target_y + item_pos.y.0),
            },
            item,
        );
    }

    // Avançar cursor Y.
    // VAlign::Horizon e VAlign::Bottom consomem o resto da página — nenhum
    // bloco deve seguir-se sem nova quebra explícita.
    // Se is_height_unconstrained, effective_v é None → segue o ramo padrão.
    match effective_v {
        Some(VAlign::Horizon) | Some(VAlign::Bottom) => {
            self.cursor_y = Pt(self.page_bottom_limit());
        },
        _ => {
            self.cursor_y = Pt(target_y + sub_frame.height.0);
        },
    }
},

Content::Place { alignment, dx, dy, body } => {
    // Place NÃO chama flush_line e NÃO modifica cursor_x nem cursor_y.
    // O conteúdo é injectado absolutamente no frame — o fluxo ignora-o.

    let avail_w = self.available_width();
    let avail_h = self.available_height();

    let sub_frame = self.layout_sub_frame_with_width(body, avail_w);

    // origin_x = line_start_x (mitigação de DEBT-37).
    // Dentro de uma coluna de grid, line_start_x é cell_x — o Place
    // fica visualmente vinculado à coluna onde foi declarado.
    // origin_y ancora à margem da página (DEBT-37: futuramente relativo ao pai).
    let (base_x, base_y) = self.resolve_alignment(
        *alignment,
        sub_frame.width.0,
        sub_frame.height.0,
        avail_w,
        avail_h,
        self.line_start_x.0,   // CORRIGIDO: era self.page_config.margin
        self.page_config.margin,
    );

    let target_x = base_x + dx;
    let target_y = base_y + dy;

    for (item_pos, item) in sub_frame.items {
        self.push_frame_item(
            Point {
                x: Pt(target_x + item_pos.x.0),
                y: Pt(target_y + item_pos.y.0),
            },
            item,
        );
    }
    // cursor_y e cursor_x ficam estritamente intocados.
},
```

---

## Tarefa 6 — Testes

### Teste L1 — `align("center")` calcula X correctamente

```rust
#[test]
fn align_center_reposiciona_no_eixo_x() {
    // Página 400pt de largura, margem 20pt → available_width = 360pt.
    // Rectângulo de 100pt centrado: target_x = 20 + (360 - 100) / 2 = 150pt.
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        #set page(width: 400pt, height: 400pt, margin: 20pt)\n\
        #align(\"center\", rect(width: 100pt, height: 20pt))\n\
    ").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);

    let items = &doc.pages[0].items;
    assert!(!items.is_empty(), "Deve haver pelo menos um item");

    let rect_x = frame_item_pos(&items[0]).x.0;
    assert!(
        (rect_x - 150.0).abs() < 0.5,
        "Rectângulo centrado deve estar em x=150pt, obteve x={:.1}", rect_x
    );
}
```

### Teste L1 — `align("right")` ancora à margem direita

```rust
#[test]
fn align_right_ancora_a_margem_direita() {
    // Página 400pt, margem 20pt → available_width = 360pt.
    // Rectângulo 80pt: target_x = 20 + (360 - 80) = 300pt.
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        #set page(width: 400pt, height: 400pt, margin: 20pt)\n\
        #align(\"right\", rect(width: 80pt, height: 20pt))\n\
    ").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);

    let rect_x = frame_item_pos(&doc.pages[0].items[0]).x.0;
    assert!(
        (rect_x - 300.0).abs() < 0.5,
        "Rectângulo direita deve estar em x=300pt, obteve x={:.1}", rect_x
    );
}
```

### Teste L1 — `place` não altera cursor Y

```rust
#[test]
fn place_nao_altera_cursor_y() {
    // Sequência:
    // 1. rect(height: 50pt)  → cursor_y avança 50pt → cursor_y ≈ margin + 50
    // 2. place("bottom-right", rect(height: 20pt)) → cursor_y NÃO avança
    // 3. rect(height: 30pt)  → começa onde o cursor ficou após o rect 1
    //
    // Os itens 1 e 3 devem estar consecutivos em Y (separados por 50pt).
    // O item 2 (place) deve estar na zona de baixo-direita.
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        #set page(width: 400pt, height: 400pt, margin: 20pt)\n\
        #rect(width: 100pt, height: 50pt)\n\
        #place(\"bottom-right\", rect(width: 60pt, height: 20pt))\n\
        #rect(width: 100pt, height: 30pt)\n\
    ").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);

    let items = &doc.pages[0].items;
    assert_eq!(items.len(), 3, "Deve haver 3 FrameItems");

    let y0 = frame_item_pos(&items[0]).y.0; // rect 1
    let y2 = frame_item_pos(&items[2]).y.0; // rect 3

    // Rect 3 deve começar imediatamente abaixo do rect 1 (y0 + 50pt).
    assert!(
        (y2 - (y0 + 50.0)).abs() < 0.5,
        "Rect 3 deve começar em y={:.1} (y0={:.1} + 50), obteve y={:.1}",
        y0 + 50.0, y0, y2
    );

    // O item place (items[1]) deve estar na zona inferior da página.
    let y1 = frame_item_pos(&items[1]).y.0;
    assert!(
        y1 > 300.0,
        "Item place(bottom-right) deve estar na zona inferior (y > 300pt), obteve y={:.1}",
        y1
    );
}
```

### Teste L1 — `Align2D::from_string` parse de strings compostas

```rust
#[test]
fn align2d_from_string_parse_correcto() {
    let a = Align2D::from_string("top-right");
    assert_eq!(a.h, Some(HAlign::Right));
    assert_eq!(a.v, Some(VAlign::Top));

    let b = Align2D::from_string("center");
    assert_eq!(b.h, Some(HAlign::Center));
    assert_eq!(b.v, None);

    let c = Align2D::from_string("bottom");
    assert_eq!(c.h, None);
    assert_eq!(c.v, Some(VAlign::Bottom));

    let d = Align2D::from_string("horizon");
    assert_eq!(d.v, Some(VAlign::Horizon));

    // String inválida: nenhum campo deve ser preenchido.
    let e = Align2D::from_string("invalid");
    assert_eq!(e.h, None);
    assert_eq!(e.v, None);
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
- [ ] `HAlign`, `VAlign`, `Align2D` definidos. `Align2D::from_string` parse
  strings compostas com `-` e ignora partes inválidas.
- [ ] `Content::Align` e `Content::Place` adicionados. Todos os `match`
  actualizados.
- [ ] `native_align` e `native_place` implementadas e registadas.
- [ ] `is_height_unconstrained: bool` adicionado ao `Layouter`.
  `layout_sub_frame_with_width` define `true` antes e restaura ao terminar.
- [ ] `page_bottom_limit()` existe como método. Nenhum cálculo
  `page_config.height - page_config.margin` inline fora dele.
- [ ] `resolve_alignment` calcula `(x, y)` com base no alinhamento,
  dimensões do conteúdo, e área disponível.
- [ ] `Content::Align` usa `self.line_start_x.0` como `origin_x` —
  não `self.page_config.margin`.
- [ ] `Content::Align` consulta `is_height_unconstrained`: se `true`,
  `VAlign::Bottom`/`Horizon` decaem para `Top` e `remaining_h` é
  `sub_frame.height.0`.
- [ ] `Content::Align` chama `flush_line()` antes de posicionar e avança
  o cursor após o bloco.
- [ ] `VAlign::Horizon` e `VAlign::Bottom` em contexto delimitado consomem
  o resto da página via `page_bottom_limit()`.
- [ ] `Content::Place` usa `self.line_start_x.0` como `origin_x` —
  mitigação de DEBT-37: Place vincula-se à coluna, não à margem da página.
- [ ] `Content::Place` não chama `flush_line()`, não modifica `cursor_x`
  nem `cursor_y`.
- [ ] DEBT-36 e DEBT-37 registados em `01_core/DEBT.md`.
- [ ] Teste `align_center_reposiciona_no_eixo_x` passa.
- [ ] Teste `align_right_ancora_a_margem_direita` passa.
- [ ] Teste `place_nao_altera_cursor_y` passa.
- [ ] Teste `align2d_from_string_parse_correcto` passa.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `cast_string()` existe em `Value` ou se a extracção de string usa outro
  método — afecta `native_align` e `native_place`.
- Se `layout_sub_frame_with_width` retorna itens ancorados em `(0, 0)` ou
  em `(margin, margin)` — determina se há dupla injecção de margem.
- Se `frame_item_pos` já existia como helper ou foi adicionado agora para
  os testes (introduzido no Passo 81.5 como parte das divergências).

**Da implementação:**
- Se `align("center")` dentro de uma célula de grid produziu o X correcto
  relativo à célula — confirma que `line_start_x` está a ser usado como
  `origin_x` e não `page_config.margin`.
- Se `is_height_unconstrained` foi necessário activar na prática (ex: align
  dentro de grid explodiu a altura da linha antes da correcção).
- Número total de testes após o passo e zero violations confirmados.

**Go/No-Go para o Passo 83:**
- **GO — alinhamento visual correcto:** rectângulo centrado aparece ao
  centro da página num leitor PDF real; `place("bottom-right")` aparece no
  canto inferior direito sem deslocar o conteúdo seguinte.
- **NO-GO — `place` desloca cursor:** se o teste `place_nao_altera_cursor_y`
  falha com `y2 ≠ y0 + 50`, verificar que o braço `Content::Place` não
  contém `self.cursor_y +=`.
- **NO-GO — `align` dentro de grid usa margem da página:** se
  `align("center")` dentro de uma célula centra relativo à página inteira
  em vez da célula, `flush_line` ainda está a usar `page_config.margin` em
  vez de `line_start_x`. Confirmar o diagnóstico 1.
