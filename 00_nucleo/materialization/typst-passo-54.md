# Passo 54 — Matrizes Matemáticas (`mat`)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/eval.rs` — arm `Expr::FuncCall` (onde `frac`, `sqrt`, etc., são interceptados)
- `01_core/src/entities/content.rs` — enum `Content`
- `01_core/src/rules/math/layout.rs` — métodos `layout_grid` e `layout_stretchy_delimiter`

Pré-condição: `cargo test` — 575 L1 + 107 L3 + 50 parity, zero violations.

---

## Contexto

A sintaxe de matrizes em Typst é feita através da chamada de função `mat(...)`.
As linhas são separadas por ponto e vírgula (`;`), e as células por vírgulas
(`,`). Exemplo: `$ mat(1, 2; 3, 4) $`.

No parser do Typst, o uso de `;` dentro dos argumentos de uma função converte
os grupos separados num formato específico (frequentemente `Array` de expressões
ou uma lista estruturada).

Para desenhar a matriz, são necessários três passos:
1. Criar a variante `Content::MathMatrix`.
2. Converter os argumentos da chamada `mat` num `Vec<Vec<Content>>` (linhas e células).
3. Fazer o layout do conteúdo reutilizando a lógica do `layout_grid`, envolvendo a grelha em parênteses extensíveis via `layout_stretchy_delimiter`.

O trabalho duro já existe: `layout_grid` (Passo 51) sabe alinhar colunas e
espaçar linhas; `layout_stretchy_delimiter` (Passos 42/43) sabe desenhar
parênteses extensíveis por assembly. A tarefa é colar estas duas infra-estruturas.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Verificar como o parser expõe ponto e vírgula nos argumentos
grep -n "Semicolon" lab/typst-original/crates/typst-syntax/src/ast.rs | head -5
grep -A 5 "pub struct Args" lab/typst-original/crates/typst-syntax/src/ast.rs

# 2. Confirmar a assinatura actual do layout_grid
grep -A 2 "fn layout_grid" 01_core/src/rules/math/layout.rs
```

Reportar o output antes de continuar. Se não for óbvio como extrair as linhas
da AST, criar um pequeno teste no L1 que faça `dbg!(call.args())` para
`$ mat(1, 2; 3, 4) $` e partilhar o output.

---

## Tarefa 1 — Expandir a estrutura L1

Em `01_core/src/entities/content.rs`, adicionar a nova variante:

```rust
MathMatrix {
    /// Linhas da matriz, onde cada linha é um array de células
    rows: Vec<Vec<Content>>,
    /// Delimitadores (ex: '(' e ')')
    delim: (char, char),
},
```

---

## Tarefa 2 — Intercepção no eval

Em `01_core/src/rules/eval.rs`, no match para `Expr::FuncCall` (onde já trata
`frac` e `sqrt`), adicionar a função `"mat"`:

Analisar os argumentos passados a `call.args()`. O Typst normalmente parseia os
argumentos delimitados por `;` como múltiplos argumentos posicionais onde cada
um é um `Array` (i.e., `Arg::Pos(Expr::Array(...))`). O diagnóstico confirmará
isto.

Extrair os elementos para construir `rows: Vec<Vec<Content>>`, convertendo cada
célula via `eval_math_expr`. Devolver:

```rust
Ok(Content::MathMatrix { rows, delim: ('(', ')') })
```

---

## Tarefa 3 — Layout da matriz (MathLayouter)

Em `01_core/src/rules/math/layout.rs`, adicionar o match para
`Content::MathMatrix`.

### Ajuste de assinatura do layout_grid

Refactorar a lógica de medição/posicionamento para uma função separada que
suporte diferentes políticas de alinhamento e espaçamento entre colunas:

```rust
enum GridAlign {
    /// Usado para `MathAlignPoint` (&): colunas pares à direita, ímpares à esquerda
    Alternating,
    /// Usado para `MathMatrix` (mat): todas as colunas centradas no seu espaço
    Center,
}

fn layout_grid_rows(
    &self,
    rows: &[Vec<Content>],
    align: GridAlign,
    column_gap: Pt,
) -> MathBox {
    // Passagem 1: medir largura máxima W_i de cada coluna (igual ao Passo 51)

    // Passagem 2: posicionar células
    // — ao avançar o cursor_x base de cada coluna, somar `column_gap`
    // — ao posicionar a célula dentro de W_i, verificar a política de `align`:
    //     GridAlign::Center      → x_offset = (W_i - cell_w) / 2
    //     GridAlign::Alternating → col par: encosta à direita (W_i - cell_w)
    //                              col ímpar: encosta à esquerda (0)
}
```

O `layout_grid` existente (para `MathAlignPoint`) passa a chamar:
```rust
self.layout_grid_rows(rows, GridAlign::Alternating, Pt(0.0))
```

O `MathMatrix` chama:
```rust
self.layout_grid_rows(&rows, GridAlign::Center, col_gap)
```

### Sequência de layout

```rust
Content::MathMatrix { rows, delim } => {
    // 1. Espaçamento padrão entre colunas (fallback 0.5em se não houver constante MATH)
    let col_gap = self.size * 0.5;

    // 2. Grelha central centrada
    let grid_box = self.layout_grid_rows(&rows, GridAlign::Center, col_gap);

    // 3. Delimitadores extensíveis com a altura total da grelha
    let left  = self.layout_stretchy_delimiter(delim.0, grid_box.ascent, grid_box.descent);
    let right = self.layout_stretchy_delimiter(delim.1, grid_box.ascent, grid_box.descent);

    // 4. Composição horizontal com padding para não colar () aos números
    let padding = self.size * 0.1;
    let mut items = Vec::new();
    let mut x = Pt(0.0);

    items.extend(left.items.iter().map(|i| offset_item(i, x, Pt(0.0))));
    x = x + left.width + padding;

    items.extend(grid_box.items.iter().map(|i| offset_item(i, x, Pt(0.0))));
    x = x + grid_box.width + padding;

    items.extend(right.items.iter().map(|i| offset_item(i, x, Pt(0.0))));
    let total_width = x + right.width;

    // 5. Bloco final e eixo matemático
    // apply_axis_offset centra a matriz no eixo matemático,
    // tal como em layout_frac e layout_delimited
    let mut final_box = MathBox {
        width: total_width,
        ascent: grid_box.ascent,
        descent: grid_box.descent,
        items,
    };
    apply_axis_offset(&mut final_box, &self.constants, self.size);
    final_box
}
```

---

## Tarefa 4 — Testes L1 e L3

### Testes no L1

```rust
#[test]
fn matrix_2x2_nao_vazio() {
    let doc = layout_test("$ mat(a, b; c, d) $");
    let text = doc.plain_text();
    assert!(text.contains('a'), "a: {}", text);
    assert!(text.contains('d'), "d: {}", text);
}

#[test]
fn matrix_1x1_nao_panica() {
    let doc = layout_test("$ mat(x) $");
    assert!(!doc.pages.is_empty());
}

#[test]
fn matrix_linha_unica_nao_panica() {
    let doc = layout_test("$ mat(1, 2, 3) $");
    assert!(!doc.pages.is_empty());
}

// Regressão — MathAlignPoint do Passo 51 não regride
#[test]
fn align_grid_nao_regride_apos_matrix() {
    let doc = layout_test("$ a &= b \\ c &= d $");
    let text = doc.plain_text();
    assert!(text.contains('a'));
    assert!(text.contains('d'));
}
```

### Teste no L3

```rust
#[test]
fn pipeline_math_matrix_gera_pdf() {
    let (world, _dir) = world_from_str("$ mat(1, 2; 3, 4) $");
    let source = world.source(world.main()).unwrap();
    let module = do_eval(&world, &source).unwrap();
    let content = module.content().expect("deve ter content");
    let doc = layout(content);
    let pdf = export_pdf(&doc);
    assert!(!pdf.is_empty());
}
```

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

- [ ] `Content::MathMatrix { rows, delim }` existe em `content.rs`
- [ ] O eval captura chamadas a `mat(...)` e traduz o separador `;` correctamente para linhas
- [ ] `GridAlign` existe com variantes `Alternating` e `Center`
- [ ] `layout_grid_rows` aceita `align: GridAlign` e `column_gap: Pt`
- [ ] `layout_grid` (para `MathAlignPoint`) chama `layout_grid_rows` com `GridAlign::Alternating` e `column_gap = Pt(0.0)`
- [ ] `MathMatrix` chama `layout_grid_rows` com `GridAlign::Center` e `column_gap = self.size * 0.5`
- [ ] Os delimitadores extensíveis envolvem a matriz cobrindo toda a sua altura
- [ ] Padding de `self.size * 0.1` separa os delimitadores da grelha
- [ ] O eixo matemático está alinhado no bloco final (`apply_axis_offset` aplicado)
- [ ] Todos os testes de regressão dos Passos 51–53 passam
- [ ] Zero violations no linter e no clippy

---

## Ao terminar, reportar

**Do diagnóstico:**
- Como o parser expõe `;` nos argumentos de `mat` (array de arrays, argumentos posicionais múltiplos, ou outro formato)
- Se `layout_grid` já tinha assinatura compatível com refactoração para `layout_grid_rows` ou exigiu alterações maiores

**Da implementação:**
- Se `layout_stretchy_delimiter` aceitou directamente `ascent`/`descent` da grelha ou foram necessários ajustes de altura
- Se o `apply_axis_offset` foi suficiente para centrar a matriz no eixo matemático

**Número total de testes e zero violations.**

**Go/No-Go para Passo 55:**
- **GO — Casos (`cases(...)`)**: estrutura análoga à matriz mas sem delimitador direito e com alinhamento diferente
- **GO — Vectores coluna (`vec(...)`)**: caso especial de matriz de uma coluna
- **NO-GO**: se a refactoração de `layout_grid` / `layout_grid_rows` causou regressões nos testes do Passo 51

---

## Anexo — Relatório do Passo 53

### Do diagnóstico

- `left_col_width` estava escrito como variável única, calculada com `max(tl_w, bl_w)` onde cada largura usava `.abs()` no kern. Os kerns `TopLeft`/`BottomLeft` já eram extraídos correctamente (linhas 532–539) desde o Passo 46, mas eram combinados em valor opaco com `.abs()`, distorcendo a geometria quando o kern era negativo.

### Da implementação

- Com `FixedMetrics` os kerns são sempre zero, portanto `tl_push == tl_box.width` e `bl_push == bl_box.width`. Se os scripts tiverem larguras diferentes, `tl_x != bl_x`. Com scripts iguais os X coincidem — comportamento correcto e esperado.
- Ausência de `tl` ou `bl`: `unwrap_or(0.0)` em `tl_push`/`bl_push` — quando apenas um está presente, o outro é zero e `base_offset_x` é determinado pelo presente. Sem caso especial adicional.

### Totais

575 L1 + 107 L3 + 50 parity, zero violations. 4 novos testes do Passo 53 passam. Regressões Passo 46 mantidas (5 testes passam).
