# Passo 55 — Vectores e Casos (`vec` e `cases`)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/eval.rs` — arm `Expr::FuncCall` em `eval_math_expr`
- `01_core/src/entities/content.rs` — enum `Content`, variante `MathMatrix`
- `01_core/src/rules/math/layout.rs` — `GridAlign`, `layout_grid_rows`, `layout_stretchy_delimiter`

Pré-condição: `cargo test` — 579 L1 + 108 L3 + 50 parity, zero violations.

---

## Contexto

Com `layout_grid_rows` e `layout_stretchy_delimiter` operacionais, `vec` e
`cases` são principalmente um problema de mapeamento semântico no eval.

**`vec(1, 2, 3)`**: vector coluna — visualmente idêntico a `mat(1; 2; 3)`.
Reutiliza `Content::MathMatrix` existente: cada argumento torna-se uma linha
de uma única célula.

**`cases(x & "if" y, 0 & "else")`**: função definida por ramos. Diferenças
em relação à matriz:
- Delimitador esquerdo: `{` (apenas um)
- Sem delimitador direito
- Alinhamento das colunas: esquerda, não centrado

Para `cases`, é necessária uma variante dedicada em `Content` e uma nova
variante em `GridAlign`.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar se o parser agrupa args de vec e cases por vírgula
#    (planos) ou por ponto e vírgula (arrays) — pode diferir do mat
grep -n "\"vec\"\|\"cases\"" lab/typst-original/crates/typst-library/src/math/ -r | head -10

# 2. Ver as variantes de GridAlign criadas no Passo 54
grep -A 4 "enum GridAlign" 01_core/src/rules/math/layout.rs

# 3. Confirmar como & dentro de um argumento chega ao eval
#    (MathAlignPoint dentro de Expr::Array, ou outro nó)
grep -n "MathAlignPoint\|AlignPoint" 01_core/src/rules/eval.rs | head -10
```

Reportar o output antes de continuar.

O comportamento de `&` dentro dos argumentos de `cases` depende do diagnóstico:
se cada argumento de `cases` já é um `Expr::Array` (como em `mat`), o `&` pode
aparecer como `MathAlignPoint` dentro desse array; se os argumentos são planos,
cada argumento completo é uma célula da única coluna (a segunda coluna só existe
se houver `&` explícito).

---

## Tarefa 1 — Expandir a estrutura L1

### Em `01_core/src/entities/content.rs`

Adicionar a variante dedicada para casos (o vector reutiliza `MathMatrix`):

```rust
MathCases {
    /// Linhas de casos; cada linha é um array de células (separadas por &)
    rows: Vec<Vec<Content>>,
},
```

Actualizar `plain_text()` e `PartialEq` para a nova variante, seguindo o
padrão das variantes `MathMatrix` e `MathSequence` existentes.

### Em `01_core/src/rules/math/layout.rs`

Adicionar a variante de alinhamento à esquerda:

```rust
pub enum GridAlign {
    Alternating, // MathAlignPoint (&): colunas pares à direita, ímpares à esquerda
    Center,      // MathMatrix (mat): todas as colunas centradas
    Left,        // MathCases (cases): todas as colunas à esquerda
}
```

No corpo de `layout_grid_rows`, adicionar o arm para `GridAlign::Left`:

```rust
GridAlign::Left => Pt(0.0), // encostar ao início da coluna
```

(Os arms `Center` e `Alternating` já existem do Passo 54.)

---

## Tarefa 2 — Intercepção no eval

Em `01_core/src/rules/eval.rs`, no match para `Expr::FuncCall` em
`eval_math_expr`, adicionar os dois arms:

### `"vec"`

Os argumentos de `vec` são separados por vírgula — o parser expõe uma
lista plana de expressões (não `Expr::Array` como em `mat`). Cada
argumento torna-se uma linha de uma única célula:

```rust
"vec" => {
    let rows = call.args()
        .items()
        .filter_map(|arg| match arg {
            Arg::Pos(expr) => Some(expr),
            _ => None,
        })
        .map(|expr| Ok(vec![eval_math_expr(ctx, expr)?]))
        .collect::<SourceResult<Vec<Vec<Content>>>>()?;

    Ok(Content::MathMatrix { rows, delim: ('(', ')') })
}
```

Se o diagnóstico mostrar que `vec` usa `;` como `mat`, adaptar para
extrair `Expr::Array` como no Passo 54.

### `"cases"`

O separador de linhas de `cases` é vírgula. O `&` dentro de cada linha,
se existir, é um `MathAlignPoint` que o eval já produz. A extracção de
células dentro de cada linha depende do diagnóstico:

```rust
"cases" => {
    let rows = call.args()
        .items()
        .filter_map(|arg| match arg {
            Arg::Pos(expr) => Some(expr),
            _ => None,
        })
        .map(|expr| {
            // Cada argumento é avaliado como sequência math.
            // Se contiver MathAlignPoint, partir em células.
            // Se não contiver, a linha tem uma única célula.
            let content = eval_math_expr(ctx, expr)?;
            let cells = match content {
                Content::MathSequence(items) => {
                    // Partir por MathAlignPoint para obter colunas
                    let mut cols: Vec<Vec<Content>> = vec![vec![]];
                    for item in items {
                        match item {
                            Content::MathAlignPoint => cols.push(vec![]),
                            other => cols.last_mut().unwrap().push(other),
                        }
                    }
                    // Filtrar colunas vazias acidentais (trailing &)
                    cols.retain(|c| !c.is_empty());
                    cols.into_iter()
                        .map(|c| Content::MathSequence(c.into()))
                        .collect()
                }
                other => vec![other],
            };
            Ok(cells)
        })
        .collect::<SourceResult<Vec<Vec<Content>>>>()?;

    // ATENÇÃO: linhas podem ter números diferentes de células (ex: uma linha
    // sem & e outra com &). layout_grid_rows deve usar row.get(col) e tratar
    // ausência como largura zero — não row[col] — para evitar panic.
    Ok(Content::MathCases { rows })
}
```

Se o diagnóstico mostrar uma estrutura diferente, adaptar antes de
codificar — não assumir.

---

## Tarefa 3 — Layout de `MathCases` (MathLayouter)

Em `01_core/src/rules/math/layout.rs`, adicionar o arm para
`Content::MathCases` no match de `layout_node` ou `layout_content`:

```rust
Content::MathCases { rows } => {
    // Espaçamento entre colunas
    let col_gap = self.size * 0.5;

    // Grelha com alinhamento à esquerda.
    // ATENÇÃO: layout_grid_rows deve usar row.get(col) e tratar largura zero
    // para células ausentes — linhas com número diferente de colunas (ex:
    // `cases(x, 0 & "else")`) causam panic se o acesso for row[col].
    let grid_box = self.layout_grid_rows(&rows, GridAlign::Left, col_gap);

    // Converter altura para Design Units — layout_stretchy_delimiter espera f64 DU,
    // não Pt (confirmado no Passo 54)
    let height_du = (grid_box.ascent + grid_box.descent).val()
        * self.constants.upem / self.size.val();

    // Apenas delimitador esquerdo: {
    let left = self.layout_stretchy_delimiter('{', height_du);

    // Composição: [Delim Esquerdo] + [Padding] + [Grelha]
    // Sem delimitador direito
    let padding = self.size * 0.1;
    let mut items = Vec::new();
    let mut x = Pt(0.0);

    // into_iter() para consumir os FrameItems (owned) — .iter() passaria
    // referências e offset_item não aceita &FrameItem
    items.extend(left.items.into_iter().map(|i| offset_item(i, x, Pt(0.0))));
    x += left.width + padding;

    items.extend(grid_box.items.into_iter().map(|i| offset_item(i, x, Pt(0.0))));
    let total_width = x + grid_box.width;

    // Bloco final
    let mut final_box = MathBox {
        width: total_width,
        ascent: grid_box.ascent,
        descent: grid_box.descent,
        items,
    };

    // Alinhar no eixo matemático — usar o método exacto implementado no layouter.
    // Se apply_axis_offset(&mut MathBox, ...) existir, chamar directamente.
    // Se o método for self.axis_offset() que devolve dy: Pt, aplicar manualmente:
    let dy = self.axis_offset(&final_box);
    for item in &mut final_box.items {
        *item = offset_item(item.clone(), Pt(0.0), dy);
    }
    final_box.ascent += dy;
    final_box.descent -= dy;

    final_box
}
```

---

## Tarefa 4 — Testes L1 e L3

### Testes no L1

```rust
#[test]
fn vec_tres_elementos_nao_vazio() {
    let doc = layout_test("$ vec(1, 2, 3) $");
    let text = doc.plain_text();
    assert!(text.contains('1'));
    assert!(text.contains('3'));
}

#[test]
fn vec_elemento_unico_nao_panica() {
    let doc = layout_test("$ vec(x) $");
    assert!(!doc.pages.is_empty());
}

#[test]
fn cases_dois_ramos_nao_vazio() {
    let doc = layout_test("$ cases(1, 0) $");
    assert!(!doc.pages.is_empty());
}

#[test]
fn cases_nao_panica_com_align_point() {
    // & dentro de cases separa a condição do valor
    let doc = layout_test("$ cases(x &, 0 &) $");
    assert!(!doc.pages.is_empty());
}

// Regressão — mat do Passo 54 não regride
#[test]
fn mat_nao_regride_apos_vec_cases() {
    let doc = layout_test("$ mat(1, 2; 3, 4) $");
    let text = doc.plain_text();
    assert!(text.contains('1'));
    assert!(text.contains('4'));
}

// Regressão — align grid do Passo 51 não regride
#[test]
fn align_grid_nao_regride_apos_passo55() {
    let doc = layout_test("$ a &= b \\ c &= d $");
    let text = doc.plain_text();
    assert!(text.contains('a'));
    assert!(text.contains('d'));
}
```

### Testes no L3

```rust
#[test]
fn pipeline_math_vec_gera_pdf() {
    let (world, _dir) = world_from_str("$ vec(1, 2, 3) $");
    let source = world.source(world.main()).unwrap();
    let module = do_eval(&world, &source).unwrap();
    let content = module.content().expect("deve ter content");
    let doc = layout(content);
    let pdf = export_pdf(&doc);
    assert!(!pdf.is_empty());
}

#[test]
fn pipeline_math_cases_gera_pdf() {
    let (world, _dir) = world_from_str("$ cases(1, 0) $");
    let source = world.source(world.main()).unwrap();
    let module = do_eval(&world, &source).unwrap();
    let content = module.content().expect("deve ter content");
    let doc = layout(content);
    let pdf = export_pdf(&doc);
    assert!(!pdf.is_empty());
}
```

O teste com `& "if"` e string literal dentro de `cases` é deixado para
quando o motor suportar `Content::Str` dentro de contexto math — não
forçar neste passo.

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critérios de conclusão:

- [ ] `Content::MathCases { rows }` existe em `content.rs` com `plain_text()` e `PartialEq` actualizados
- [ ] `GridAlign::Left` existe e `layout_grid_rows` implementa o arm correspondente
- [ ] `layout_grid_rows` usa `row.get(col)` com largura zero para células ausentes — sem panic em linhas assimétricas
- [ ] O eval captura `"vec"` e converte para `Content::MathMatrix` com uma célula por linha
- [ ] O eval captura `"cases"` e converte para `Content::MathCases`; colunas vazias (trailing `&`) são filtradas
- [ ] `layout_grid_rows` é chamado com `GridAlign::Left` e `col_gap` para `MathCases`
- [ ] `layout_stretchy_delimiter` é chamado com `height_du: f64` (Design Units), não com `Pt`
- [ ] `.into_iter()` é usado ao extender `items` — não `.iter()`
- [ ] `MathCases` desenha apenas o delimitador esquerdo `{` — sem delimitador direito
- [ ] Eixo matemático ajustado via `axis_offset` no bloco final de `MathCases`
- [ ] Todos os testes de regressão dos Passos 51–54 passam
- [ ] Zero violations no linter e no clippy

---

## Ao terminar, reportar

**Do diagnóstico:**
- Como o parser agrupa os argumentos de `vec` e `cases` (planos ou arrays)
- Se `&` dentro de `cases` aparece como `MathAlignPoint` dentro de `Expr::Array` ou de outro nó
- Se `GridAlign` já tinha a estrutura esperada ou exigiu alterações adicionais

**Da implementação:**
- Se a extracção de células dentro de cada linha de `cases` funcionou via `MathSequence` ou exigiu outro mecanismo
- Se `layout_stretchy_delimiter` para `{` aceitou os mesmos parâmetros que `(` e `)` no Passo 54

**Número total de testes e zero violations.**

**Go/No-Go para Passo 56:**
- **GO — Introspection e estado**: motor de equações considerado completo para a fase actual; iniciar Passos 56–75
- **NO-GO**: se a extracção de células de `cases` com `&` causou falhas não resolvidas nos testes de regressão do Passo 51

---

## Anexo — Relatório do Passo 54

### Do diagnóstico

- `;` em `mat(1, 2; 3, 4)`: o parser (linha 494–506 de `parse.rs`) agrupa cada grupo separado por vírgula num `Expr::Array`. Resultado: `Arg::Pos(Expr::Array([1, 2]))` e `Arg::Pos(Expr::Array([3, 4]))`. Sem `;`, os args são `Arg::Pos(expr)` individuais.
- Refactoração de `layout_grid`: exigiu converter cada célula-sequência em `Content::MathSequence` para que `layout_grid_rows` (que chama `layout_node`) a pudesse processar.

### Da implementação

- `layout_stretchy_delimiter` espera `min_height_du: f64` em Design Units. Foi necessário converter `grid_box.ascent + grid_box.descent` de `Pt` para DU com `* upem / style.size.val()` — mesma fórmula de `layout_delimited`.
- `apply_axis_offset` foi suficiente para centrar a matriz no eixo matemático.

### Totais

579 L1 + 108 L3 + 50 parity, zero violations. 4 novos testes L1 (`matrix_*`) + 1 teste L3 (`pipeline_math_matrix_gera_pdf`). Regressão Passo 51 mantida (`align_grid_nao_regride_apos_matrix` passa).
