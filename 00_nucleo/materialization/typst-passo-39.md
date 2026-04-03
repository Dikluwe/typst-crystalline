# Passo 39 — Motor de equações: símbolos matemáticos

**Pré-condições**:
- Passo 38 concluído: 446 L1 + 57 L3 + 50 parity, zero violations
- `rules/math/layout.rs` com `MathLayouter` funcional
- Branch: `cristalino/migration`

**Verificação antes de começar**:
```bash
# Confirmar que MathShorthand cai no _ em eval_math_content
grep -n "MathShorthand\|Shorthand" \
  01_core/src/rules/eval.rs | head -10

# Confirmar estrutura de rules/math/
ls 01_core/src/rules/math/

# Verificar se unicode_math_class está em Cargo.toml de 01_core
grep "unicode_math_class" 01_core/Cargo.toml

# Confirmar como MathIdent é tratado actualmente em eval_math_content
grep -n "MathIdent\|SyntaxKind::Math" \
  01_core/src/rules/eval.rs | head -15
```

**Parar se qualquer pré-condição falhar.**

---

## Contexto

Actualmente, `alpha` em modo math produz `Content::MathIdent("alpha")` e
renderiza o texto literal "alpha" no PDF. `=>` produz `Content::Empty`
(cai no arm `_`). Este passo implementa a conversão para Unicode correcto.

**Três objectivos independentes**:
1. Tabela de símbolos em `rules/math/symbols.rs`
2. `MathShorthand` → Unicode em `eval_math_content`
3. `MathIdent` → símbolo Unicode ou itálico/não-itálico conforme o tipo

---

## Tarefa 1 — Diagnóstico

```bash
# Ver todos os SyntaxKind matemáticos tratados em eval_math_content
grep -n "SyntaxKind::Math\|MathIdent\|MathText\|MathShorthand\|MathFrac\|MathAttach" \
  01_core/src/rules/eval.rs | head -30

# Ver como MathIdent é construído actualmente
grep -n -A 5 "MathIdent" \
  01_core/src/rules/eval.rs | head -20

# Ver se TextStyle tem campo italic já funcional
grep -n "italic" \
  01_core/src/entities/layout_types.rs | head -5

# Ver SyntaxKind::MathShorthand — existe no cristalino?
grep -n "MathShorthand" \
  01_core/src/entities/syntax_kind.rs
```

**Parar. Reportar output antes de qualquer código.**

Questões a responder:
1. `SyntaxKind::MathShorthand` existe no cristalino?
2. `eval_math_content` usa `SyntaxKind` directamente ou `Expr` tipado?
3. `TextStyle::italic` existe e é usado no export/layout?
4. `unicode_math_class` está em `Cargo.toml` ou precisa de ser adicionada?

---

## Tarefa 2 — Criar `rules/math/symbols.rs`

```rust
// @layer: L1
// @updated: YYYY-MM-DD
// Tabela de símbolos matemáticos: identificadores e shorthands → Unicode.

/// Converte um identificador matemático (ex: "alpha") para o carácter
/// Unicode correspondente, se existir na tabela.
/// Retorna `None` se o identificador não é um símbolo conhecido.
pub fn ident_to_unicode(name: &str) -> Option<&'static str> {
    match name {
        // Letras gregas minúsculas
        "alpha"   => Some("α"),
        "beta"    => Some("β"),
        "gamma"   => Some("γ"),
        "delta"   => Some("δ"),
        "epsilon" => Some("ε"),
        "zeta"    => Some("ζ"),
        "eta"     => Some("η"),
        "theta"   => Some("θ"),
        "iota"    => Some("ι"),
        "kappa"   => Some("κ"),
        "lambda"  => Some("λ"),
        "mu"      => Some("μ"),
        "nu"      => Some("ν"),
        "xi"      => Some("ξ"),
        "pi"      => Some("π"),
        "rho"     => Some("ρ"),
        "sigma"   => Some("σ"),
        "tau"     => Some("τ"),
        "upsilon" => Some("υ"),
        "phi"     => Some("φ"),
        "chi"     => Some("χ"),
        "psi"     => Some("ψ"),
        "omega"   => Some("ω"),
        // Letras gregas maiúsculas
        "Alpha"   => Some("Α"),
        "Beta"    => Some("Β"),
        "Gamma"   => Some("Γ"),
        "Delta"   => Some("Δ"),
        "Epsilon" => Some("Ε"),
        "Theta"   => Some("Θ"),
        "Lambda"  => Some("Λ"),
        "Xi"      => Some("Ξ"),
        "Pi"      => Some("Π"),
        "Sigma"   => Some("Σ"),
        "Phi"     => Some("Φ"),
        "Psi"     => Some("Ψ"),
        "Omega"   => Some("Ω"),
        // Operadores e símbolos comuns
        "sum"     => Some("∑"),
        "prod"    => Some("∏"),
        "int"     => Some("∫"),
        "infty"   => Some("∞"),
        "partial" => Some("∂"),
        "nabla"   => Some("∇"),
        "forall"  => Some("∀"),
        "exists"  => Some("∃"),
        "in"      => Some("∈"),
        "notin"   => Some("∉"),
        "subset"  => Some("⊂"),
        "supset"  => Some("⊃"),
        "union"   => Some("∪"),
        "inter"   => Some("∩"),
        "emptyset" => Some("∅"),
        "times"   => Some("×"),
        "div"     => Some("÷"),
        "pm"      => Some("±"),
        "mp"      => Some("∓"),
        "cdot"    => Some("·"),
        "dots"    => Some("…"),
        "ldots"   => Some("…"),
        "cdots"   => Some("⋯"),
        "vdots"   => Some("⋮"),
        "ddots"   => Some("⋱"),
        "approx"  => Some("≈"),
        "sim"     => Some("∼"),
        "cong"    => Some("≅"),
        "equiv"   => Some("≡"),
        "propto"  => Some("∝"),
        "perp"    => Some("⊥"),
        "parallel" => Some("∥"),
        "angle"   => Some("∠"),
        "circ"    => Some("∘"),
        "bullet"  => Some("•"),
        "star"    => Some("★"),
        "dagger"  => Some("†"),
        "hbar"    => Some("ℏ"),
        "ell"     => Some("ℓ"),
        "Re"      => Some("ℜ"),
        "Im"      => Some("ℑ"),
        "aleph"   => Some("ℵ"),
        _ => None,
    }
}

/// Converte um shorthand matemático (ex: "=>") para o carácter Unicode.
/// Retorna `None` se o shorthand não é reconhecido.
pub fn shorthand_to_unicode(text: &str) -> Option<&'static str> {
    match text {
        "=>"  => Some("⇒"),
        "<="  => Some("⇐"),
        "<=>" => Some("⇔"),
        "->"  => Some("→"),
        "<-"  => Some("←"),
        "<->" => Some("↔"),
        "!="  => Some("≠"),
        "<="  => Some("≤"),
        ">="  => Some("≥"),
        "<<"  => Some("≪"),
        ">>"  => Some("≫"),
        ":="  => Some("≔"),
        "=:"  => Some("≕"),
        "~="  => Some("≃"),
        "~~"  => Some("≈"),
        "++"  => Some("⧺"),
        "--"  => Some("—"),
        "..."  => Some("…"),
        ".."  => Some("‥"),
        _     => None,
    }
}

/// Retorna true se o identificador é uma função matemática conhecida
/// (deve ser renderizado em texto normal, não itálico).
pub fn is_math_function(name: &str) -> bool {
    matches!(name,
        "sin" | "cos" | "tan" | "cot" | "sec" | "csc" |
        "arcsin" | "arccos" | "arctan" |
        "sinh" | "cosh" | "tanh" |
        "log" | "ln" | "exp" |
        "lim" | "limsup" | "liminf" |
        "max" | "min" | "sup" | "inf" |
        "det" | "tr" | "rank" | "dim" | "ker" | "im" |
        "gcd" | "lcm" | "mod" | "div" |
        "Pr" | "Var" | "Cov" | "E"
    )
}

/// Retorna true se o identificador é uma variável de uma letra
/// (deve ser renderizado em itálico matemático).
pub fn is_single_letter_var(name: &str) -> bool {
    name.len() == 1 && name.chars().next().map(|c| c.is_ascii_alphabetic()).unwrap_or(false)
}
```

Expor em `rules/math/mod.rs`:

```rust
pub mod layout;
pub mod symbols;
pub use layout::MathLayouter;
```

---

## Tarefa 3 — Usar tabela em `eval_math_content`

### 3a — `MathIdent` com lookup de símbolo e estilo

Localizar o arm de `MathIdent` (ou `SyntaxKind::MathIdent`) em
`eval_math_content` e actualizar:

```rust
// Antes
Content::MathIdent(text)  // ou SyntaxKind::MathIdent => { ... }

// Depois
// Para Expr::MathIdent ou SyntaxKind::MathIdent:
let name = /* texto do nó */;

if let Some(sym) = symbols::ident_to_unicode(name) {
    // Símbolo Unicode — não é variável, não é função
    // Usar itálico = false (símbolos como ∑, π não são itálico)
    Content::MathText(sym.into())
} else if symbols::is_math_function(name) {
    // Função conhecida — não-itálico
    Content::MathIdent(name.into())  // layout usará italic: false
} else if symbols::is_single_letter_var(name) {
    // Variável de uma letra — itálico matemático
    // Marcar de alguma forma para o layout aplicar italic: true
    // Opção mais simples: Content::MathIdent com flag, ou
    // usar Content::MathText com o carácter itálico matemático Unicode
    // (ex: 'x' → '𝑥' U+1D465)
    // Passo 39: usar Content::MathIdent e deixar o layout decidir
    Content::MathIdent(name.into())
} else {
    // Identificador multi-letra não reconhecido — não-itálico
    Content::MathIdent(name.into())
}
```

**Nota sobre itálico matemático Unicode**: os caracteres itálicos
matemáticos (𝑎𝑏𝑐…𝑥𝑦𝑧) estão no bloco Mathematical Alphanumeric Symbols
(U+1D400–U+1D7FF). Usar estes caracteres directamente é a forma mais
simples — não requer alteração de `TextStyle`. Implementar opcionalmente
se a fonte actual os suportar; caso contrário, deixar para Passo 41
(fontes OpenType MATH).

### 3b — `MathShorthand` arm em `eval_math_content`

Adicionar arm explícito para `SyntaxKind::MathShorthand`:

```rust
SyntaxKind::MathShorthand => {
    let text = child.text(); // ou node.text(), conforme a API
    if let Some(sym) = symbols::shorthand_to_unicode(text) {
        Content::MathText(sym.into())
    } else {
        // Shorthand não reconhecido — manter texto original
        Content::MathText(text.into())
    }
}
```

---

## Tarefa 4 — Itálico em `MathLayouter`

O `MathLayouter` actualmente usa o `TextStyle` passado pelo caller para
todos os items. Para aplicar itálico a variáveis, `layout_node` precisa
de poder modificar o estilo:

```rust
// Em math/layout.rs, arm de MathIdent em layout_node:
Content::MathIdent(name) => {
    // Variáveis de uma letra → itálico
    // Funções conhecidas → não-itálico
    let is_var = symbols::is_single_letter_var(name)
        && symbols::ident_to_unicode(name).is_none();
    let is_func = symbols::is_math_function(name);

    let math_style = if is_var && !is_func {
        TextStyle { italic: true, ..*style }
    } else {
        TextStyle { italic: false, ..*style }
    };
    self.layout_text_node(name, &math_style)
}
```

**Nota**: `MathText` (símbolos Unicode, shorthands) usa o estilo base
sem modificação — símbolos como `∑` ou `→` não são itálico.

---

## Tarefa 5 — Actualizar `DEBT.md`

```markdown
### DEBT-8 — Motor de equações — PARCIALMENTE RESOLVIDO

**Resolvido no Passo 39**:
- `rules/math/symbols.rs` com tabelas `ident_to_unicode`, `shorthand_to_unicode`,
  `is_math_function`, `is_single_letter_var`
- `MathShorthand` → Unicode em `eval_math_content`
- `MathIdent` de símbolo (alpha, sum, etc.) → `Content::MathText` com Unicode
- Variáveis de uma letra → `TextStyle { italic: true }` em `MathLayouter`
- Funções (sin, cos, lim, etc.) → `TextStyle { italic: false }`

**Ainda pendente**:
- `sqrt()` como função nativa com símbolo radical geométrico
- Itálico matemático Unicode (𝑎𝑏𝑐) — Passo 41 (fontes OpenType MATH)
- Kern matemático entre símbolos
- `MathPrimes`, `MathAlignPoint` com semântica correcta
```

---

## Tarefa 6 — Testes

```rust
// ── tabela de símbolos — testes directos ─────────────────────────────────

#[test]
fn alpha_converte_para_unicode() {
    assert_eq!(symbols::ident_to_unicode("alpha"), Some("α"));
}

#[test]
fn sum_converte_para_unicode() {
    assert_eq!(symbols::ident_to_unicode("sum"), Some("∑"));
}

#[test]
fn identificador_desconhecido_retorna_none() {
    assert_eq!(symbols::ident_to_unicode("foobar"), None);
}

#[test]
fn shorthand_seta_direita() {
    assert_eq!(symbols::shorthand_to_unicode("->"), Some("→"));
}

#[test]
fn shorthand_diferente() {
    assert_eq!(symbols::shorthand_to_unicode("!="), Some("≠"));
}

#[test]
fn sin_e_funcao_nao_variavel() {
    assert!(symbols::is_math_function("sin"));
    assert!(!symbols::is_single_letter_var("sin"));
}

#[test]
fn x_e_variavel_de_uma_letra() {
    assert!(symbols::is_single_letter_var("x"));
    assert!(!symbols::is_math_function("x"));
}

// ── eval com símbolos ─────────────────────────────────────────────────────

#[test]
fn eval_alpha_produz_unicode() {
    let world = MockWorld::new("$alpha$");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "eval de $alpha$ falhou: {:?}", result);
    // Verificar que o Content produzido contém "α" e não "alpha"
    // Adaptar conforme API de Module::content()
}

#[test]
fn eval_shorthand_seta() {
    let world = MockWorld::new("$x -> y$");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "$x -> y$ falhou: {:?}", result);
}

#[test]
fn eval_equacao_com_sum_e_sigma() {
    let world = MockWorld::new("$sum_(i=0)^n x_i$");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_ok(), "equação com sum falhou: {:?}", result);
}

// ── integração L3 ─────────────────────────────────────────────────────────

#[test]
fn pipeline_simbolos_gregos_gera_pdf() {
    let (world, _dir) = world_from_str("$alpha + beta = gamma$");
    let source = world.source(world.main()).unwrap();
    let module = eval(&world, &source).unwrap();
    let doc    = layout(module.content(), &FixedMetrics).unwrap();
    let pdf    = export_pdf(&doc);
    assert!(!pdf.is_empty());
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn pipeline_funcao_sin_nao_italico() {
    // sin deve aparecer no PDF em não-itálico; x em itálico
    // Verificação básica: não dá panic e gera PDF
    let (world, _dir) = world_from_str("$sin(x)$");
    let source = world.source(world.main()).unwrap();
    let module = eval(&world, &source).unwrap();
    let doc    = layout(module.content(), &FixedMetrics).unwrap();
    let pdf    = export_pdf(&doc);
    assert!(!pdf.is_empty());
}
```

---

## Verificação final

```bash
cargo test -p typst-core 2>&1 | tail -5
cargo test -p typst-infra 2>&1 | tail -5
cargo test -p parity_runner 2>&1 | tail -3
cargo build 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found

# Confirmar symbols.rs em rules/math/
ls 01_core/src/rules/math/

# Confirmar que "alpha" não aparece literalmente em testes de equação
grep -rn "\"alpha\"" 01_core/src/rules/math/symbols.rs | head -3

# Confirmar MathShorthand tem arm próprio
grep -n "MathShorthand\|Shorthand" 01_core/src/rules/eval.rs
```

Critérios de conclusão:
- `rules/math/symbols.rs` com `ident_to_unicode`, `shorthand_to_unicode`,
  `is_math_function`, `is_single_letter_var` ✓
- `MathShorthand` tem arm próprio em `eval_math_content` ✓
- `MathIdent` de símbolo → `Content::MathText` com Unicode ✓
- Variáveis de uma letra → `TextStyle { italic: true }` em `MathLayouter` ✓
- Funções → `TextStyle { italic: false }` ✓
- Testes de tabela passam ✓
- `eval_alpha_produz_unicode` passa ✓
- `pipeline_simbolos_gregos_gera_pdf` passa ✓
- DEBT-8 actualizado ✓
- Zero violations ✓
- Testes não regridem (446 L1 + 57 L3 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- `SyntaxKind::MathShorthand` existe no cristalino?
- `eval_math_content` usa `SyntaxKind` ou `Expr` tipado — qual o padrão exacto?
- `unicode_math_class` estava em `Cargo.toml` ou não foi necessária?

**Da implementação:**
- Quantos símbolos da tabela foram confirmados como correctos via parity_runner?
- O itálico em variáveis de uma letra aparece visualmente diferente no PDF
  (com a fonte actual)? Sim/não.
- Número final de testes e zero violations confirmado.

**DEBT-8 parcialmente resolvido. Go para Passo 40 — `sqrt()` nativa e
`MathRoot` com símbolo radical.**
