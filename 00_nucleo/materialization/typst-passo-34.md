# Passo 34 — Preparação de Content para equações: tipos fundamentais

**Pré-condições**:
- Passo 33 concluído: 418 testes L1 + 49 testes L3, zero violations
- Branch: `cristalino/migration`

**Verificação antes de começar**:
```bash
# Estado actual de Content
grep -n "^pub enum Content" -A 30 \
  01_core/src/entities/content.rs

# Ver se Expr::Math / MathElem já tem arm em eval_expr
grep -n "Math\|Equation\|math" \
  01_core/src/rules/eval.rs | head -20

# Ver o que o parser já produz para $ x^2 $
grep -n "Equation\|Math\b\|SyntaxKind::Math" \
  01_core/src/entities/syntax_kind.rs | head -10

# Ver a estrutura AST de Math no oráculo
grep -n "^pub struct Equation\|^pub struct Math\b" \
  01_core/src/entities/ast/math.rs | head -10
```

**Parar se qualquer pré-condição falhar.**

---

## Contexto

O Passo 36 inicia o motor de equações (ADR-0032, fase obrigatória).
Antes disso, `Content` precisa de ter as variantes que o motor vai
produzir, e `eval_expr` precisa de tratar `Expr::Equation` sem panic
ou catch-all silencioso.

Este passo **não** implementa o motor matemático. Implementa:
1. As variantes de `Content` para representar equações e nós matemáticos
2. O arm `Expr::Equation` em `eval_expr` — produz um `Content::Equation`
   placeholder correcto
3. O tratamento de `Expr::Math` (modo matemático) — stub com Err limpo
   ou placeholder, não catch-all

O motor real (renderização, layout de equações, fontes matemáticas)
começa no Passo 36.

---

## Tarefa 1 — Diagnóstico do oráculo

```bash
# Estrutura AST de Equation no oráculo e os seus métodos
grep -rn "^pub struct Equation\|Equation" \
  01_core/src/entities/ast/math.rs | head -20
grep -A 10 "impl.*Equation" 01_core/src/entities/ast/math.rs | head -20

# Que SyntaxKinds existem para matemática
grep -n "Math\|Equation\|Attach\|Frac\|Root\|Primes" \
  01_core/src/entities/syntax_kind.rs | head -30

# Como o oráculo representa equações em Content/layout
grep -rn "Equation\|MathElem\|EquationElem" \
  lab/typst-original/crates/typst-library/src/ 2>/dev/null | head -20

# Que variantes de Expr existem para math
grep -n "Equation\|Math\b\|MathIdent\|MathAttach\|MathFrac" \
  01_core/src/entities/ast/expr.rs | head -20
```

**Parar. Reportar output antes de qualquer código.**

Questões a responder:
1. `Expr::Equation` existe no AST do cristalino? Com que campos
   (`body`, `block: bool`)?
2. `Expr::Equation` tem um método chamado exatamente `body()` que retorna um `SyntaxNode` ou `AstNode` tipado? Se não, qual é o nome do método equivalente?
3. Que `SyntaxKind`s matemáticos existem — lista completa.
4. `eval_expr` tem arm para `Expr::Equation` ou cai no catch-all?
5. O oráculo usa `EquationElem` ou outro tipo para representar equações?
6. Qual é a estrutura interna exata que o AST define para extrair base, sub e sup de um `MathAttach`?

---

## Tarefa 2 — Novas variantes de `Content`

Adicionar em `01_core/src/entities/content.rs`:

```rust
pub enum Content {
    // ... variantes existentes ...

    /// Equação matemática.
    /// `block: true` → equação em linha própria (display mode)
    /// `block: false` → equação inline
    ///
    /// `body` é a árvore de nós matemáticos produzida pelo eval.
    /// O motor de equações (Passo 36+) processa este nó.
    Equation {
        body:  Box<Content>,
        block: bool,
    },

    /// Sequência de nós matemáticos (corpo de uma equação).
    MathSequence(Arc<[Content]>),

    /// Identificador matemático: variável, função, símbolo.
    /// Ex: `x`, `sin`, `alpha`
    MathIdent(EcoString),

    /// Texto literal dentro de modo matemático.
    /// Ex: o "texto" em `$"texto"$`
    MathText(EcoString),

    /// Fracção: numerador / denominador.
    MathFrac {
        num: Box<Content>,
        den: Box<Content>,
    },

    /// Índice/expoente: base com sub e/ou sup.
    /// `sub` e `sup` são `None` se ausentes.
    MathAttach {
        base: Box<Content>,
        sub:  Option<Box<Content>>,
        sup:  Option<Box<Content>>,
    },

    /// Raiz: `root(index, radicand)`.
    /// `index` é None para raiz quadrada.
    MathRoot {
        index:    Option<Box<Content>>,
        radicand: Box<Content>,
    },
}
```

Actualizar `plain_text()` para as novas variantes:

```rust
Content::Equation { body, block } => {
    if *block {
        format!("\n{}\n", body.plain_text())
    } else {
        body.plain_text()
    }
}
Content::MathSequence(nodes) => {
    nodes.iter().map(|n| n.plain_text()).collect::<String>()
}
Content::MathIdent(s)  => s.to_string(),
Content::MathText(s)   => s.to_string(),
Content::MathFrac { num, den } => {
    format!("({})/({})", num.plain_text(), den.plain_text())
}
Content::MathAttach { base, sub, sup } => {
    let mut s = base.plain_text();
    if let Some(sub) = sub { s.push_str(&format!("_{}", sub.plain_text())); }
    if let Some(sup) = sup { s.push_str(&format!("^{}", sup.plain_text())); }
    s
}
Content::MathRoot { index, radicand } => {
    match index {
        None    => format!("sqrt({})", radicand.plain_text()),
        Some(i) => format!("root({}, {})", i.plain_text(), radicand.plain_text()),
    }
}
```

---

## Tarefa 3 — Arm `Expr::Equation` em `eval_expr`

Localizar ou criar o arm para `Expr::Equation`. O eval produz
`Content::Equation` a partir do body da equação:

```rust
Expr::Equation(eq) => {
    let block = eq.block();
    // NOTA: Se o método para aceder ao conteúdo se chamar algo diferente de `body()`,
    // adapte a chamada abaixo conforme o diagnóstico.
    let body  = eval_math_content(ctx, scopes, eq.body())?;
    Ok(Value::Content(Content::Equation {
        body:  Box::new(body),
        block,
    }))
}
```

### Função `eval_math_content`

Avaliar o corpo de uma equação — produz `Content::MathSequence` com
os nós matemáticos:

```rust
fn eval_math_content(
    ctx:    &mut EvalContext<impl TrackedWorld>,
    scopes: &mut Scopes,
    node:   &SyntaxNode,
) -> SourceResult<Content> {
    // Percorrer os filhos do nó de math e avaliar cada um
    let mut nodes: Vec<Content> = Vec::new();

    for child in node.children() {
        let kind = child.kind();
        match kind {
            SyntaxKind::MathIdent => {
                let text = child.text().to_string();
                nodes.push(Content::MathIdent(text.into()));
            }
            SyntaxKind::Text | SyntaxKind::MathText => {
                let text = child.text().to_string();
                nodes.push(Content::MathText(text.into()));
            }
            SyntaxKind::MathFrac => {
                // Dois filhos: numerador e denominador
                let children: Vec<_> = child.children().collect();
                if children.len() >= 2 {
                    let num = eval_math_content(ctx, scopes, &children[0])?;
                    let den = eval_math_content(ctx, scopes, &children[1])?;
                    nodes.push(Content::MathFrac {
                        num: Box::new(num),
                        den: Box::new(den),
                    });
                }
            }
            SyntaxKind::MathAttach => {
                // Base + sub opcional + sup opcional
                let base_node = child.children().next();
                if let Some(base_node) = base_node {
                    let base = eval_math_content(ctx, scopes, base_node)?;
                    // IMPORTANTE: Deixar sub e sup como None intencionalmente.
                    // A extração correta baseada no diagnóstico (se é caret/underscore)
                    // será escrita no Passo 36. Registar a forma da árvore no relatório.
                    nodes.push(Content::MathAttach {
                        base: Box::new(base),
                        sub:  None,
                        sup:  None,
                    });
                }
            }
            SyntaxKind::Space | SyntaxKind::Linebreak => {
                // ignorar espaços e quebras de linha em modo math
            }
            _ => {
                // Nós matemáticos não implementados neste passo:
                // produzir MathText com o texto raw como placeholder
                let text = child.text().to_string();
                if !text.trim().is_empty() {
                    nodes.push(Content::MathText(text.into()));
                }
            }
        }
    }

    match nodes.len() {
        0 => Ok(Content::Empty),
        1 => Ok(nodes.remove(0)),
        _ => Ok(Content::MathSequence(nodes.into())),
    }
}
```

**Nota**: `eval_math_content` é um stub intencional. O motor real
(Passo 36+) vai substituir esta função. O que importa aqui é que:
- `Expr::Equation` não cai no catch-all
- O `Content::Equation` produzido tem estrutura correcta
- O layout e export passam por `Content::Equation` sem panic

---

## Tarefa 4 — Tratar `Content::Equation` em layout e export

### Layout

`layout.rs` precisa de um arm para `Content::Equation`. Por agora,
renderiza como texto plano (placeholder):

```rust
Content::Equation { body, block } => {
    // Placeholder: renderizar como texto até o motor de equações
    // (Passo 36+) tratar correctamente.
    let text = body.plain_text();
    if *block {
        // adicionar marcadores [ ] para visualização temporária do bloco
        self.layout_text(ctx, &format!("[{}]", text), style)
    } else {
        self.layout_text(ctx, &text, style)
    }
}

Content::MathSequence(_)
| Content::MathIdent(_)
| Content::MathText(_)
| Content::MathFrac { .. }
| Content::MathAttach { .. }
| Content::MathRoot { .. } => {
    // Nós matemáticos internos — não aparecem directamente no layout
    // fora de Content::Equation. Se aparecerem, renderizar como texto.
    self.layout_text(ctx, &content.plain_text(), style)
}
```

### Export

`export.rs` trata `FrameItem::Text` — não precisa de alterações se
layout já converte equações para texto. Verificar e confirmar.

---

## Tarefa 5 — Actualizar `DEBT.md`

```markdown
### DEBT-8 — Motor de equações não implementado — INTENCIONAL

**Estado**: Pendente — Passo 36+

**Descrição**: `Content::Equation` e variantes matemáticas estão
definidas (Passo 34). O eval produz a estrutura correcta. O layout
renderiza como texto plano (placeholder). O motor real de equações
(layout tipográfico de matemática, fontes matemáticas, MathML) começa
no Passo 36 conforme ADR-0032. `eval_math_content` é um stub que descarta propriedades sub/sup em anexos matemáticos até ser reescrita.

**Comportamento actual**: equações renderizam como texto plano entre
`[ ]` (display) ou inline. Não é erro — é placeholder documentado.

**Ficheiros a alterar no Passo 36+**: `rules/layout.rs`, `rules/eval.rs`,
`entities/content.rs` (possível extensão de variantes matemáticas),
`03_infra/src/export.rs`
```

---

## Tarefa 6 — Testes

```rust
// ── Content::Equation — testes directos ──────────────────────────────────

#[test]
fn content_equation_inline_plain_text() {
    let eq = Content::Equation {
        body:  Box::new(Content::MathIdent("x".into())),
        block: false,
    };
    assert_eq!(eq.plain_text(), "x");
}

#[test]
fn content_equation_block_plain_text() {
    let eq = Content::Equation {
        body:  Box::new(Content::MathIdent("x".into())),
        block: true,
    };
    assert_eq!(eq.plain_text(), "\nx\n");
}

#[test]
fn content_math_frac_plain_text() {
    let frac = Content::MathFrac {
        num: Box::new(Content::MathIdent("a".into())),
        den: Box::new(Content::MathIdent("b".into())),
    };
    assert_eq!(frac.plain_text(), "(a)/(b)");
}

#[test]
fn content_math_attach_plain_text() {
    let attach = Content::MathAttach {
        base: Box::new(Content::MathIdent("x".into())),
        sub:  None,
        sup:  Some(Box::new(Content::MathIdent("2".into()))),
    };
    assert_eq!(attach.plain_text(), "x^2");
}

// ── eval de equações ──────────────────────────────────────────────────────

#[test]
fn eval_equation_inline_nao_da_err() {
    let world = MockWorld::new("O valor de $x$ é 1.");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "equação inline falhou: {:?}", result);
}

#[test]
fn eval_equation_block_nao_da_err() {
    let world = MockWorld::new("$ x^2 + y^2 = r^2 $");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "equação block falhou: {:?}", result);
}

#[test]
fn eval_equation_frac_nao_da_err() {
    let world = MockWorld::new("$ frac(a, b) $");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "equação com frac falhou: {:?}", result);
}

#[test]
fn eval_equation_nao_cai_no_catch_all() {
    // Verificar que Expr::Equation tem arm próprio (não retorna Value::None)
    let world = MockWorld::new("$x$");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    // O módulo deve ter Content não-vazio
    // Adaptar verificação conforme API de Module::content()
    let _ = m;
}

// ── integração L3 ─────────────────────────────────────────────────────────
// Adicionar em 03_infra/src/integration_tests.rs:

#[test]
fn pipeline_equacao_inline_gera_pdf() {
    let (world, _dir) = world_from_str("A equação $x^2$ é famosa.");
    let source = world.source(world.main()).unwrap();
    let module = eval(&world, &source).unwrap();
    let doc = layout(module.content(), &FixedMetrics).unwrap();
    let pdf = export_pdf(&doc);
    assert!(!pdf.is_empty());
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn pipeline_equacao_block_gera_pdf() {
    let (world, _dir) = world_from_str("$ E = m c^2 $");
    let source = world.source(world.main()).unwrap();
    let module = eval(&world, &source).unwrap();
    let doc = layout(module.content(), &FixedMetrics).unwrap();
    let pdf = export_pdf(&doc);
    assert!(!pdf.is_empty());
}
```

---

## Verificação final

```bash
cargo test -p typst-core 2>&1 | tail -5
cargo test -p typst-infra 2>&1 | tail -5
cargo build 2>&1 | tail -5
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found

# Confirmar novas variantes em Content
grep -n "Equation\|MathIdent\|MathFrac\|MathAttach\|MathRoot\|MathSequence\|MathText" \
  01_core/src/entities/content.rs

# Confirmar arm em eval_expr
grep -n "Expr::Equation\|eval_math_content" \
  01_core/src/rules/eval.rs

# Confirmar que não há panic! em caminhos matemáticos
grep -n "panic!\|todo!\|unimplemented!" \
  01_core/src/rules/eval.rs | grep -i "math\|equation"
# Deve retornar vazio
```

Critérios de conclusão:
- `Content::Equation`, `MathSequence`, `MathIdent`, `MathText`,
  `MathFrac`, `MathAttach`, `MathRoot` definidos em `content.rs` ✓
- `plain_text()` implementado para todas as variantes matemáticas ✓
- `Expr::Equation` tem arm próprio em `eval_expr` ✓
- `eval_math_content` implementado como stub (sem panic) ✓
- `layout.rs` trata `Content::Equation` sem panic (placeholder texto) ✓
- `eval_equation_inline_nao_da_err` e `eval_equation_block_nao_da_err` passam ✓
- `pipeline_equacao_inline_gera_pdf` e `pipeline_equacao_block_gera_pdf` passam ✓
- DEBT-8 registado em `DEBT.md` ✓
- Zero violations ✓
- Testes não regridem (418 L1 + 49 L3 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- `Expr::Equation` já existia no AST ou estava ausente?
- O método para obter o corpo de `Expr::Equation` chama-se exactamente `body()` e retorna um `SyntaxNode`/`AstNode` tipado? Se o nome for outro no AST actual, reportar o nome real utilizado na adaptação.
- Que `SyntaxKind`s matemáticos existem no cristalino — lista completa.
- `eval_expr` tinha arm para `Expr::Equation` ou caía no catch-all?
- Qual é a estrutura interna exata que o AST define para extrair base, sub e sup de um `MathAttach` para ser usada no Passo 36?

**Da implementação:**
- Quantas variantes matemáticas de `SyntaxKind` foram mapeadas em
  `eval_math_content` (vs quantas caem no placeholder)?
- O placeholder de layout `[equação]` aparece no PDF gerado (confirmar que os testes L3 não falham e os arquivos seriam íntegros)?
- Número final de testes e zero violations confirmado.

**DEBT-8 registado. Go para Passo 35 — paridade de parsing via oráculo.**
