# Passo 51 — MathAlignPoint (Alinhamento em Grelha 2D)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/content.rs` — variantes `Content::MathAlignPoint`, `Content::Linebreak` (confirmar se existem)
- `01_core/src/rules/eval.rs` — `eval_math_content`, arm `Expr::MathAlignPoint`
- `01_core/src/rules/math/layout.rs` — `MathLayouter`, loop principal de sequências, `layout_node`
- `01_core/src/rules/layout.rs` — onde `Content::Equation` é processada (Passo 48/50)

Pré-condição: `cargo test` — 560 L1 + 100 L3 + 50 parity, zero violations.

---

## Contexto

A sintaxe `$ a &= b \ c &= d $` usa `&` (align point) para alinhar colunas e
`\` para quebrar linhas dentro de uma equação de bloco. O resultado visual
correcto é que os `=` ficam na mesma coordenada X em todas as linhas,
independentemente da largura dos termos à esquerda.

O motor actual processa sequências matemáticas linearmente (esquerda para
a direita). Para suportar alinhamento, o layout de sequências precisa de
duas passagens: primeiro mede todas as células para descobrir a largura
máxima de cada coluna, depois posiciona com essas larguras fixas.

**Âmbito deste passo**: equações de bloco (`block == true`) com `&` e `\`.
Equações inline com `&` são raras e o comportamento inline é deixar o `&`
como espaço — não é necessário tratar neste passo.

**Regra de alinhamento Typst**: colunas ímpares (índice 0, 2, 4…) alinham
o seu conteúdo à direita; colunas pares (índice 1, 3, 5…) alinham à
esquerda. Isto produz o efeito de `a &= b` onde `a` fica alinhado à
direita do seu espaço e `= b` fica alinhado à esquerda do seu.

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# 1. Confirmar se MathAlignPoint já existe no Content e no eval
grep -n "MathAlignPoint\|AlignPoint" \
  01_core/src/entities/content.rs \
  01_core/src/rules/eval.rs | head -15

# 2. Confirmar como Linebreak/quebra de linha chega ao eval_math_content
grep -n "Linebreak\|MathNewline\|Linebreak\|Align" \
  01_core/src/rules/eval.rs | head -15

# 3. Como MathSequence é construída e iterada no layout
grep -n "MathSequence\|layout_sequence\|layout_node\|Content::Math" \
  01_core/src/rules/math/layout.rs | head -20

# 4. Confirmar se existe um loop de sequência separado ou se layout_node
# trata Content::MathSequence recursivamente
grep -A 10 "MathSequence" 01_core/src/rules/math/layout.rs | head -30

# 5. Como hconcat é implementado — recebe Vec<MathBox> e retorna MathBox
grep -A 8 "fn hconcat" 01_core/src/rules/math/layout.rs

# 6. Confirmar helpers de teste disponíveis
grep -n "fn layout_test\|fn compile_to_pdf" \
  01_core/src/rules/math/layout.rs \
  03_infra/src/integration_tests.rs | head -10
```

**Reportar o output antes de continuar.**

Se `MathAlignPoint` já existir no `Content` mas não estiver mapeado no layout,
a Tarefa 1 é apenas adicionar o mapeamento no eval. Se não existir em lado
nenhum, é necessário adicionar a variante ao enum e ao eval.

Se quebras de linha matemáticas usarem um nó diferente de `Content::Linebreak`
(ex: `Content::MathNewline`), adaptar a Tarefa 2 antes de codificar.

---

## Tarefa 1 — Representação no eval (L1)

### MathAlignPoint

Se `Expr::MathAlignPoint` ainda não estiver mapeado em `eval_math_content`:

```rust
// Em eval.rs — eval_math_content:
Expr::MathAlignPoint(_) => Ok(Content::MathAlignPoint),
```

Se `Content::MathAlignPoint` não existir em `content.rs`, adicionar como
variante unitária (sem campos):

```rust
// Em content.rs:
MathAlignPoint,
```

### Quebras de linha em matemática

Confirmar no diagnóstico como `\` chega ao eval. Se não estiver mapeado,
adicionar:

```rust
// O nó exacto depende do diagnóstico — pode ser Expr::Linebreak ou outro
Expr::MathNewline(_) => Ok(Content::Linebreak),
// ou, se Linebreak já existe:
Expr::Linebreak(_) => Ok(Content::Linebreak),
```

`Content::Linebreak` provavelmente já existe (usado em markup). Se não
existir em contexto matemático, confirmar e adicionar.

---

## Tarefa 2 — Detecção de grelha no MathLayouter (L1)

No ponto de entrada do layout de sequências (provavelmente onde
`Content::MathSequence` é processada), adicionar detecção de grelha:

```rust
/// Verifica se uma sequência matemática contém pontos de alinhamento
/// ou quebras de linha — i.e., precisa de layout em grelha.
fn needs_grid_layout(items: &[Content]) -> bool {
    items.iter().any(|c| matches!(c,
        Content::MathAlignPoint | Content::Linebreak
    ))
}
```

Se `needs_grid_layout` retornar `false`, usar o layout linear existente
(`hconcat` sobre todos os items) — sem alteração de performance.

---

## Tarefa 3 — Partição em linhas e colunas (L1)

```rust
/// Particiona uma sequência flat em linhas e colunas.
///
/// Retorna `Vec<Vec<Vec<Content>>>` onde:
///   - dim 0: linhas (separadas por Linebreak)
///   - dim 1: colunas (separadas por MathAlignPoint)
///   - dim 2: items da célula
fn partition_grid(items: &[Content]) -> Vec<Vec<Vec<Content>>> {
    let mut lines: Vec<Vec<Vec<Content>>> = vec![vec![vec![]]];

    for item in items {
        match item {
            Content::Linebreak => {
                // Nova linha com uma célula vazia
                lines.push(vec![vec![]]);
            }
            Content::MathAlignPoint => {
                // Nova célula na linha actual
                lines.last_mut().unwrap().push(vec![]);
            }
            other => {
                // Adicionar ao último item da última célula da última linha
                lines.last_mut().unwrap()
                     .last_mut().unwrap()
                     .push(other.clone());
            }
        }
    }

    // Remover células finais vazias em cada linha (trailing align points)
    for line in &mut lines {
        while line.last().map(|c| c.is_empty()).unwrap_or(false) {
            line.pop();
        }
    }

    lines
}
```

---

## Tarefa 4 — Layout de 2 passagens (L1)

Novo método no `MathLayouter`:

```rust
fn layout_grid(&self, items: &[Content]) -> MathBox {
    let grid = partition_grid(items);
    let n_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    // ── Passagem 1: medir células e calcular largura máxima por coluna ──
    // grid_boxes[linha][coluna] = MathBox da célula
    let mut grid_boxes: Vec<Vec<MathBox>> = grid.iter()
        .map(|row| {
            row.iter()
               .map(|cell| self.layout_sequence(cell))
               .collect()
        })
        .collect();

    // col_widths[i] = largura máxima da coluna i entre todas as linhas
    let mut col_widths = vec![Pt(0.0); n_cols];
    for row in &grid_boxes {
        for (col_idx, cell_box) in row.iter().enumerate() {
            if cell_box.width.0 > col_widths[col_idx].0 {
                col_widths[col_idx] = cell_box.width;
            }
        }
    }

    // ── Passagem 2: posicionar células ──
    // Regra de alinhamento: colunas ímpares (0, 2, …) → direita;
    //                       colunas pares  (1, 3, …) → esquerda.
    // (índice base 0: col 0 é a primeira, alinha à direita)
    let mut all_items: Vec<FrameItem> = Vec::new();
    let mut cursor_y = Pt(0.0);

    for (row_idx, row) in grid_boxes.iter().enumerate() {
        let row_ascent  = row.iter().map(|b| b.ascent).fold(Pt(0.0), |a,b| if b.0>a.0{b}else{a});
        let row_descent = row.iter().map(|b| b.descent).fold(Pt(0.0), |a,b| if b.0>a.0{b}else{a});

        let mut cursor_x = Pt(0.0);
        for (col_idx, cell_box) in row.iter().enumerate() {
            let col_w = col_widths[col_idx];
            // Alinhamento interno da célula
            let cell_x = if col_idx % 2 == 0 {
                // Coluna ímpar (0-based) → alinha à direita
                cursor_x + (col_w - cell_box.width)
            } else {
                // Coluna par → alinha à esquerda
                cursor_x
            };

            // Centrar verticalmente na linha por ascent
            let cell_y = cursor_y;

            // Transladar todos os items da célula para a posição final
            for item in &cell_box.items {
                all_items.push(offset_item(item, cell_x, cell_y));
            }

            cursor_x = cursor_x + col_w;
        }

        // Avançar Y para a linha seguinte
        // Espaçamento inter-linhas: usar line_gap da constante ou fallback
        let line_gap = self.size * Pt(0.2); // fallback: 20% do corpo
        cursor_y = cursor_y + row_ascent + row_descent + line_gap;
    }

    // Calcular ascent/descent/width totais do MathBox resultante
    let total_width = col_widths.iter().fold(Pt(0.0), |a, &w| Pt(a.0 + w.0));
    let total_ascent = grid_boxes.first()
        .and_then(|r| r.iter().map(|b| b.ascent).reduce(|a,b| if b.0>a.0{b}else{a}))
        .unwrap_or(Pt(0.0));
    let total_descent = Pt(cursor_y.0); // y acumulado é a altura total abaixo da baseline

    MathBox {
        width: total_width,
        ascent: total_ascent,
        descent: total_descent,
        items: all_items,
    }
}
```

**Nota sobre `offset_item`**: confirmar no diagnóstico se `offset_item` aceita
dois deltas `(dx, dy)` — foi generalizado no Passo 44 para suportar `dy`.
Se aceitar apenas `dx`, usar dois `offset_item` sequenciais ou adaptar.

**Nota sobre `layout_sequence`**: se não existir como método separado, extrair
do corpo actual do loop de sequência. O fallback linear deve chamar este mesmo
método.

---

## Tarefa 5 — Integração no loop principal (L1)

No sítio onde `Content::MathSequence` é processada (provavelmente em
`layout_node` ou no corpo principal do `MathLayouter`):

```rust
Content::MathSequence(items) => {
    if self.block && needs_grid_layout(items) {
        self.layout_grid(items)
    } else {
        // Layout linear existente
        let boxes: Vec<MathBox> = items.iter()
            .map(|c| self.layout_node(c))
            .collect();
        hconcat(boxes)
    }
}
```

A condição `self.block &&` garante que equações inline com `&` não usam
layout de grelha — o `&` é ignorado (ou tratado como espaço) em modo inline.

---

## Tarefa 6 — Testes

### Testes unitários em L1

```rust
#[cfg(test)]
mod tests_align {
    use super::*;

    #[test]
    fn align_simples_contem_conteudo() {
        // $ a &= b \ c &= d $ — dois lados de duas linhas presentes
        let doc = layout_test("$ a &= b \\ c &= d $");
        let text = doc.plain_text();
        assert!(text.contains('a'), "a: {}", text);
        assert!(text.contains('b'), "b: {}", text);
        assert!(text.contains('c'), "c: {}", text);
        assert!(text.contains('d'), "d: {}", text);
    }

    #[test]
    fn align_sem_ampersand_nao_regride() {
        // Equação sem & — layout linear mantido
        let doc = layout_test("$ x + 1 $");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('1'));
    }

    #[test]
    fn align_com_frac_nao_panica() {
        let doc = layout_test("$ frac(a, b) &= c \\ d &= e $");
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn align_linha_unica_com_ampersand() {
        // Uma linha com & — duas colunas mas sem quebra de linha
        let doc = layout_test("$ a &= b $");
        let text = doc.plain_text();
        assert!(text.contains('a'));
        assert!(text.contains('b'));
    }

    #[test]
    fn align_inline_nao_usa_grelha() {
        // inline: & ignorado, não deve panicar
        let doc = layout_test("$a &= b$");
        assert!(!doc.pages.is_empty());
    }

    // Regressão: testes dos passos anteriores
    #[test]
    fn frac_nao_regride() {
        let doc = layout_test("$ frac(1, 2) $");
        let text = doc.plain_text();
        assert!(text.contains('1'));
        assert!(text.contains('2'));
    }

    #[test]
    fn sum_com_limites_nao_regride() {
        let doc = layout_test("$ sum_(i=0)^n $");
        let text = doc.plain_text();
        assert!(
            text.contains('∑') || text.contains('i') || text.contains('n'),
            "sum: {}", text
        );
    }
}
```

### Testes em L3

```rust
#[cfg(test)]
mod tests_align_pdf {

    #[test]
    fn pdf_align_duas_linhas_nao_vazio() {
        let pdf = compile_to_pdf("$ a &= b + c \\ alpha &= x $");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_align_linha_unica_nao_vazio() {
        let pdf = compile_to_pdf("$ a &= b $");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_align_com_frac_nao_vazio() {
        let pdf = compile_to_pdf("$ frac(1,2) &= x \\ y &= z $");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_align_contem_bt_et() {
        let pdf = compile_to_pdf("$ a &= b \\ c &= d $");
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("BT"));
        assert!(s.contains("ET"));
    }

    #[test]
    fn pdf_sem_align_nao_regride() {
        let pdf = compile_to_pdf("$ x^2 + y_i $");
        assert!(!pdf.is_empty());
    }
}
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critérios de conclusão:

- [ ] `Content::MathAlignPoint` existe como variante unitária em `content.rs`
- [ ] `Expr::MathAlignPoint` está mapeado em `eval_math_content`
- [ ] Quebras de linha matemáticas (`\`) produzem `Content::Linebreak` em contexto math
- [ ] `needs_grid_layout` detecta correctamente sequências com `&` ou `\`
- [ ] `partition_grid` divide correctamente em linhas e colunas
- [ ] `layout_grid` executa duas passagens: medição e posicionamento
- [ ] Larguras máximas por coluna garantem alinhamento vertical de `&`
- [ ] Colunas ímpares (0-based) alinham à direita; colunas pares à esquerda
- [ ] Equações inline com `&` não usam layout de grelha (`self.block &&`)
- [ ] Layout linear existente continua inalterado para sequências sem `&`/`\`
- [ ] Todos os testes de regressão dos Passos 49–50 passam
- [ ] `ttf-parser` não aparece em `01_core/Cargo.toml`
- [ ] Zero violations no linter

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `MathAlignPoint` já existia no `Content` ou foi adicionado
- Nó exacto usado para quebras de linha em contexto matemático (`\`)
- Se `layout_sequence` existia como método separado ou foi necessário extrair
- Se `offset_item` já aceitava `dy` (Passo 44) ou foi necessário adaptar

**Da implementação:**
- Se a regra de alinhamento direita/esquerda alternada produziu resultado visual correcto
- Se o espaçamento inter-linhas (fallback 20% do corpo) foi suficiente ou requereu ajuste
- Se foi necessário tratar o caso de linhas com número diferente de colunas

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 52:**
- **GO — Espaçamento inter-linhas via MathConstants**: se grelha funciona, Passo 52 substitui o fallback 20% por `axis_height` ou constante dedicada da tabela MATH
- **GO — Kern diferenciado por quadrante para left-scripts**: refinamento da simplificação do Passo 46
- **NO-GO — partition_grid com linhas de colunas diferentes**: se linhas com número diferente de `&` causaram misalignment, resolver antes de avançar
