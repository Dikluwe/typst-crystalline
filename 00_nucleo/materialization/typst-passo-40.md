# Passo 40 — sqrt() nativa e MathRoot com símbolo radical

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/eval.rs` — `eval_math_content`, arm de `FuncCall` com `frac`
- `01_core/src/rules/math/layout.rs` — `MathLayouter`, `layout_frac`, `layout_attach`
- `01_core/src/rules/math/symbols.rs` — `ident_to_unicode`, `is_math_function`
- `01_core/src/entities/content.rs` — variante `MathRoot { index, radicand }`
- `03_infra/src/export.rs` — `FrameItem::Line` → operadores PDF `w m l S`
- `lab/typst-original/crates/typst-syntax/src/ast.rs` — `MathRoot` API

Pré-condição: `cargo test` — 461 L1 + 59 L3 + 50 parity, zero violations.

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# 1. API de MathRoot no AST tipado do original
grep -n "MathRoot\|fn radicand\|fn index" \
  lab/typst-original/crates/typst-syntax/src/ast.rs | head -20

# 2. API de MathRoot no AST tipado do cristalino
grep -n "MathRoot\|fn radicand\|fn index" \
  01_core/src/entities/ast/math.rs | head -20

# 3. Como o original avalia MathRoot em eval
grep -rn "MathRoot\|sqrt\|root" \
  lab/typst-original/crates/typst-eval/src/ | head -20

# 4. Variante MathRoot já existe no Content cristalino?
grep -n "MathRoot" 01_core/src/entities/content.rs | head -10

# 5. Como frac é tratado actualmente em eval_math_content (padrão a seguir)
grep -n "frac\|FuncCall" 01_core/src/rules/eval.rs | head -20

# 6. MathLayouter — métodos existentes
grep -n "pub fn\|fn layout_" 01_core/src/rules/math/layout.rs | head -20

# 7. FrameItem::Line — confirmar que já existe
grep -n "Line" 01_core/src/entities/layout_types.rs | head -10

# 8. SyntaxKind::MathRoot — confirmar existência
grep -n "MathRoot" 01_core/src/entities/syntax_kind.rs | head -5
```

**Reportar o output antes de continuar.**

---

## Tarefa 1 — sqrt() e root() como funções nativas de math

Padrão idêntico a `frac(a, b)`: em `eval_math_content`, o arm de
`Expr::FuncCall` identifica `MathIdent("sqrt")` e `MathIdent("root")`
e produz `Content::MathRoot`.

```rust
// Em eval_math_content, dentro do arm Expr::FuncCall:

"sqrt" => {
    // sqrt(x) — 1 argumento posicional
    let args: Vec<_> = fc.args().items()
        .filter_map(|a| match a {
            ast::Arg::Pos(e) => Some(e),
            _ => None,
        })
        .collect();
    if args.len() != 1 {
        return Err(/* erro: sqrt espera exactamente 1 argumento */);
    }
    let radicand = eval_math_content(args[0].to_untyped(), scopes, ctx)?;
    Content::MathRoot {
        index: None,
        radicand: Box::new(radicand),
    }
}
"root" => {
    // root(n, x) — 2 argumentos posicionais: índice, radicando
    let args: Vec<_> = fc.args().items()
        .filter_map(|a| match a {
            ast::Arg::Pos(e) => Some(e),
            _ => None,
        })
        .collect();
    if args.len() != 2 {
        return Err(/* erro: root espera exactamente 2 argumentos */);
    }
    let index = eval_math_content(args[0].to_untyped(), scopes, ctx)?;
    let radicand = eval_math_content(args[1].to_untyped(), scopes, ctx)?;
    Content::MathRoot {
        index: Some(Box::new(index)),
        radicand: Box::new(radicand),
    }
}
```

Verificar: se `Content::MathRoot` já tem `index: Option<Box<Content>>` e
`radicand: Box<Content>`. Se não, adicionar a variante (o documento de
estado indica que já existe).

---

## Tarefa 2 — MathLayouter::layout_root

Em `01_core/src/rules/math/layout.rs`, adicionar `layout_root`.

Estrutura visual de `sqrt(x)`:

```
  ┌────────┐
√ │ conteúdo│
  └────────┘
```

Componentes:
- Símbolo `√` (U+221A) como `FrameItem::Text` à esquerda
- Linha horizontal (overline) sobre o radicando, usando `FrameItem::Line`
- Radicando posicionado à direita do símbolo radical

Para `root(n, x)`:
```
 n┌────────┐
√ │ conteúdo│
  └────────┘
```
- Índice `n` posicionado acima e à esquerda do símbolo radical,
  com tamanho reduzido (65% do tamanho base, como MathAttach sup)

```rust
/// Layout de raiz quadrada/n-ésima.
///
/// Componentes: símbolo √, overline, radicando, índice opcional.
pub fn layout_root(
    &mut self,
    index: Option<&Content>,
    radicand: &Content,
) -> MathBox {
    let base_size = self.size;

    // 1. Layout do radicando
    let rad_box = self.layout_node(radicand);

    // 2. Símbolo √ — texto com tamanho base
    let radical_char = "√";
    let radical_width = self.metrics.advance(radical_char, base_size);
    let radical_item = FrameItem::Text {
        pos: Point::ZERO,  // ajustar depois
        text: radical_char.into(),
        style: TextStyle::regular(base_size),
    };

    // 3. Overline — linha horizontal sobre o radicando
    //    Espessura: ~0.5pt (ou proporcional ao tamanho)
    let line_thickness = Pt(base_size.val() * 0.04);
    let gap = Pt(base_size.val() * 0.1);  // espaço entre overline e radicando

    // 4. Dimensões totais
    let total_ascent = rad_box.ascent + gap + line_thickness;
    let total_width = radical_width + rad_box.width;

    // 5. Posicionar itens
    let mut items = Vec::new();

    // Símbolo √ à esquerda, centrado verticalmente
    items.push(FrameItem::Text {
        pos: Point {
            x: Pt::ZERO,
            y: Pt(total_ascent.val() - rad_box.ascent.val()),
        },
        text: radical_char.into(),
        style: TextStyle::regular(base_size),
    });

    // Overline — de radical_width até radical_width + rad_box.width
    let line_y = Pt::ZERO;  // topo
    items.push(FrameItem::Line {
        start: Point { x: radical_width, y: line_y },
        end: Point { x: Pt(radical_width.val() + rad_box.width.val()), y: line_y },
        thickness: line_thickness,
    });

    // Radicando — à direita do símbolo, abaixo da overline
    let rad_offset_x = radical_width;
    let rad_offset_y = line_thickness + gap;
    for item in rad_box.items {
        items.push(offset_item(item, rad_offset_x, rad_offset_y));
    }

    // 6. Índice opcional (para root(n, x))
    let mut final_width = total_width;
    if let Some(idx_content) = index {
        let old_size = self.size;
        self.size = Pt(base_size.val() * 0.65);
        let idx_box = self.layout_node(idx_content);
        self.size = old_size;

        // Posicionar índice acima e à esquerda do símbolo √
        let idx_x = Pt(radical_width.val() * 0.2);
        let idx_y = Pt::ZERO;  // topo — acima da overline
        for item in idx_box.items {
            items.push(offset_item(item, idx_x, idx_y));
        }
        // Se o índice exceder a largura do radical, expandir
        let idx_end = Pt(idx_x.val() + idx_box.width.val());
        if idx_end.val() > radical_width.val() {
            // Não expandir — o índice fica sobreposto ao radical
            // (comportamento consistente com LaTeX para índices pequenos)
        }
    }

    MathBox {
        width: final_width,
        ascent: total_ascent,
        descent: rad_box.descent,
        items,
    }
}
```

Nota: `offset_item` é uma função auxiliar que desloca a posição de
um `FrameItem`. Se não existir, criar:

```rust
fn offset_item(item: FrameItem, dx: Pt, dy: Pt) -> FrameItem {
    match item {
        FrameItem::Text { pos, text, style } => FrameItem::Text {
            pos: Point {
                x: Pt(pos.x.val() + dx.val()),
                y: Pt(pos.y.val() + dy.val()),
            },
            text,
            style,
        },
        FrameItem::Line { start, end, thickness } => FrameItem::Line {
            start: Point {
                x: Pt(start.x.val() + dx.val()),
                y: Pt(start.y.val() + dy.val()),
            },
            end: Point {
                x: Pt(end.x.val() + dx.val()),
                y: Pt(end.y.val() + dy.val()),
            },
            thickness,
        },
    }
}
```

---

## Tarefa 3 — Integrar layout_root no dispatch do Layouter

Em `layout_node` (ou equivalente) do `MathLayouter`, adicionar o arm
para `Content::MathRoot`:

```rust
Content::MathRoot { ref index, ref radicand } => {
    self.layout_root(index.as_deref(), radicand)
}
```

Verificar: o `layout_content` principal em `rules/layout.rs` delega
`Content::Equation` ao `MathLayouter`. `MathRoot` só aparece dentro de
equações, portanto o dispatch existe dentro de `MathLayouter::layout_node`.

---

## Tarefa 4 — Adicionar sqrt/root a is_math_function

Em `01_core/src/rules/math/symbols.rs`, adicionar `"sqrt"` e `"root"`
à lista de `is_math_function` (se não estiverem já). Isto garante que
estes identificadores recebem `italic: false` no layout.

```rust
// Em is_math_function:
"sqrt" | "root" => true,
```

---

## Tarefa 5 — Testes

### Testes unitários em L1

```rust
#[cfg(test)]
mod tests_sqrt {
    use super::*;

    // ── eval ──────────────────────────────────────────────────

    #[test]
    fn eval_sqrt_basico() {
        // $sqrt(x)$ → Equation contendo MathRoot { index: None, radicand: MathIdent "x" }
        let result = eval_test("$sqrt(x)$");
        let content = result.content().unwrap();
        assert!(matches_math_root(content, None));
    }

    #[test]
    fn eval_root_com_indice() {
        // $root(3, x)$ → MathRoot { index: Some("3"), radicand: MathIdent "x" }
        let result = eval_test("$root(3, x)$");
        let content = result.content().unwrap();
        assert!(matches_math_root(content, Some("3")));
    }

    #[test]
    fn eval_sqrt_expressao_composta() {
        // $sqrt(x^2 + 1)$ → MathRoot cujo radicando é MathSequence
        let result = eval_test("$sqrt(x^2 + 1)$");
        let content = result.content().unwrap();
        assert!(content_contains_math_root(content));
    }

    #[test]
    fn eval_sqrt_aninhado() {
        // $sqrt(sqrt(x))$ → MathRoot cujo radicando é outro MathRoot
        let result = eval_test("$sqrt(sqrt(x))$");
        let content = result.content().unwrap();
        assert!(content_contains_math_root(content));
    }

    #[test]
    fn eval_sqrt_zero_args_erro() {
        // $sqrt()$ → erro
        let result = try_eval_test("$sqrt()$");
        assert!(result.is_err());
    }

    #[test]
    fn eval_sqrt_dois_args_erro() {
        // $sqrt(x, y)$ → erro
        let result = try_eval_test("$sqrt(x, y)$");
        assert!(result.is_err());
    }

    #[test]
    fn eval_root_um_arg_erro() {
        // $root(3)$ → erro (precisa de 2)
        let result = try_eval_test("$root(3)$");
        assert!(result.is_err());
    }

    // ── layout ────────────────────────────────────────────────

    #[test]
    fn layout_sqrt_contem_radical() {
        // Layout de $sqrt(x)$ deve conter o caractere √
        let doc = layout_test("$sqrt(x)$");
        let text = doc.plain_text();
        assert!(text.contains('√'), "plain_text deve conter √: {}", text);
    }

    #[test]
    fn layout_sqrt_contem_radicando() {
        // Layout de $sqrt(x)$ deve conter "x"
        let doc = layout_test("$sqrt(x)$");
        let text = doc.plain_text();
        assert!(text.contains('x'), "plain_text deve conter x: {}", text);
    }

    #[test]
    fn layout_sqrt_tem_overline() {
        // Layout de $sqrt(x)$ deve gerar pelo menos um FrameItem::Line
        let doc = layout_test("$sqrt(x)$");
        let has_line = doc.pages.iter().any(|p| {
            p.items.iter().any(|i| matches!(i, FrameItem::Line { .. }))
        });
        assert!(has_line, "sqrt deve gerar FrameItem::Line para overline");
    }

    #[test]
    fn layout_root_com_indice() {
        // Layout de $root(3, x)$ deve conter "3", "√", "x"
        let doc = layout_test("$root(3, x)$");
        let text = doc.plain_text();
        assert!(text.contains('3'), "deve conter índice 3: {}", text);
        assert!(text.contains('√'), "deve conter √: {}", text);
        assert!(text.contains('x'), "deve conter radicando x: {}", text);
    }
}
```

Nota: `eval_test`, `layout_test`, `try_eval_test` são helpers que já
existem nos testes anteriores. `matches_math_root` e
`content_contains_math_root` são helpers novos — implementar como
pattern match sobre `Content::MathRoot`.

### Teste de integração em L3

```rust
#[test]
fn pdf_sqrt_basico() {
    // Pipeline completo: .typ → eval → layout → PDF
    let pdf = compile_to_pdf("$sqrt(x^2 + 1)$");
    assert!(!pdf.is_empty());
    // Verificação: PDF válido (não panic no export)
}

#[test]
fn pdf_root_com_indice() {
    let pdf = compile_to_pdf("$root(3, x)$");
    assert!(!pdf.is_empty());
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
- [ ] `sqrt(x)` produz `Content::MathRoot { index: None, radicand }`
- [ ] `root(n, x)` produz `Content::MathRoot { index: Some(n), radicand }`
- [ ] `sqrt()` e `sqrt(x, y)` retornam erro
- [ ] `root(x)` retorna erro (precisa de 2 argumentos)
- [ ] `MathLayouter::layout_root` produz MathBox com símbolo `√`, overline e radicando
- [ ] `root(3, x)` inclui índice com tamanho reduzido
- [ ] PDF gerado com `$sqrt(x)$` não faz panic
- [ ] `sqrt` e `root` retornam `italic: false` em `is_math_function`
- [ ] Zero violations no linter

---

## Ao terminar, reportar

**Do diagnóstico:**
- API exacta de `MathRoot` no AST cristalino (`radicand()`, `index()`)
- Se `Content::MathRoot` já existia ou foi adicionado neste passo
- Se `offset_item` já existia ou é novo

**Da implementação:**
- Número de testes novos adicionados
- Se o símbolo `√` aparece no PDF gerado (verificação visual se possível)
- Se a overline tem largura correcta (cobre todo o radicando)
- Se o índice em `root(3, x)` fica posicionado acima/esquerda do radical

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 41:**
- **GO — Fontes OpenType MATH**: sqrt funciona com Helvetica fallback;
  Passo 41 introduz tabela MATH para kern matemático e métricas correctas
- **NO-GO — layout_root incorrecto**: posicionamento visual errado
  ou overline não alinhada; corrigir antes de avançar
