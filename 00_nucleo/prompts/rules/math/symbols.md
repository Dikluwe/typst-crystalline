# Prompt L0 — `rules/math/symbols` — Resolução de Símbolos Matemáticos

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/math/symbols.rs`
**Passo de origem**: Passo 36 (identificadores), Passo 49 (operadores grandes, limit functions)
**ADRs relevantes**: ADR-0011 (MathClass no L1), ADR-0017 (adiamento de Context completo)

---

## Contexto e Objetivo

Durante o layout de equações, o `MathLayouter` precisa de tomar decisões
tipográficas baseadas na **identidade semântica** dos símbolos — não apenas no
texto bruto. Este módulo centraliza todo o conhecimento sobre símbolos
matemáticos: qual `char` Unicode corresponde ao nome `alpha`? Este identificador
é uma função (upright) ou uma variável (itálico)? O `∑` deve ter limites
verticais em display mode?

Estas funções são chamadas pelo `MathLayouter` em `layout.rs` para decidir
estilo tipográfico, posicionamento de limits e emissão de `FrameItem`.

---

## Interface Pública

### `ident_to_unicode(name: &str) -> Option<&'static str>`

Converte um identificador matemático (`&str`) para o caractere Unicode
correspondente. Retorna `None` se o identificador não é um símbolo reconhecido.

**Cobertura**:
- 23 letras gregas minúsculas: `alpha`→`α`, `beta`→`β`, ..., `omega`→`ω`
- 13 letras gregas maiúsculas: `Alpha`→`Α`, `Gamma`→`Γ`, ..., `Omega`→`Ω`
- Operadores e símbolos: `sum`→`∑`, `prod`→`∏`, `int`→`∫`, `infty`→`∞`
- Lógica: `forall`→`∀`, `exists`→`∃`, `in`→`∈`, `notin`→`∉`
- Conjuntos: `subset`→`⊂`, `union`→`∪`, `inter`→`∩`, `emptyset`→`∅`
- Aritmética: `times`→`×`, `div`→`÷`, `pm`→`±`, `cdot`→`·`
- Ellipses: `dots`/`ldots`→`…`, `cdots`→`⋯`, `vdots`→`⋮`, `ddots`→`⋱`
- Relações: `approx`→`≈`, `equiv`→`≡`, `propto`→`∝`, `perp`→`⊥`
- Especiais: `hbar`→`ℏ`, `ell`→`ℓ`, `Re`→`ℜ`, `Im`→`ℑ`, `aleph`→`ℵ`

### `shorthand_to_unicode(text: &str) -> Option<&'static str>`

Converte shorthands textuais para Unicode. Usado para `MathShorthand` da AST.

**Cobertura**:
- Setas simples: `"->"→"→"`, `"<-"→"←"`, `"<->"→"↔"`
- Setas duplas: `"=>"→"⇒"`, `"<=>"→"⇔"`, `"==>"→"⟹"`
- Setas longas: `"-->"→"⟶"`, `"<--"→"⟵"`, `"<-->"→"⟷"`
- Setas especiais: `"->>"→"↠"`, `"|->"→"↦"` (mapsto)
- Comparações: `"!="→"≠"`, `"<="→"≤"`, `">="→"≥"`, `"<<"→"≪"`, `">>"→"≫"`
- Definição: `":="→"≔"`, `"::="→"⩴"`, `"=:"→"≕"`
- Ellipses: `"..."→"…"`, `".."→"‥"`

### `is_math_function(name: &str) -> bool`

Retorna `true` se o identificador é uma **função matemática** que deve ser
renderizada em texto não-itálico (upright).

Funções reconhecidas: `sin`, `cos`, `tan`, `cot`, `sec`, `csc`, `arcsin`,
`arccos`, `arctan`, `sinh`, `cosh`, `tanh`, `log`, `ln`, `exp`, `lim`,
`limsup`, `liminf`, `max`, `min`, `sup`, `inf`, `det`, `tr`, `rank`, `dim`,
`ker`, `im`, `gcd`, `lcm`, `mod`, `div`, `Pr`, `Var`, `Cov`, `E`, `sqrt`, `root`

### `is_single_letter_var(name: &str) -> bool`

Retorna `true` se o identificador é uma variável de uma única letra ASCII
(`a`-`z`, `A`-`Z`) — deve ser renderizada em **itálico matemático**.

Regra: `name.len() == 1 && name.chars().next().map(|c| c.is_ascii_alphabetic())`

### `is_large_operator(c: char) -> bool`

Retorna `true` se o caractere é um **operador grande** que, em display mode,
deve receber limites (`sup`/`sub`) empilhados verticalmente (não à direita).

Operadores reconhecidos:
- Somatório/Produto: `∑` `∏` `∐`
- União/Intersecção: `⋃` `⋂` `⨄` `⨅` `⨆`
- Integrais: `∫` `∬` `∭` `∮` `∯` `∰`
- Outros: `⨁` (oplus) `⨂` (otimes) `⨀` (odot) `⋀` `⋁`

### `is_limit_function(s: &str) -> bool`

Retorna `true` se o identificador é uma função com limites verticais em display
mode: `"lim"`, `"max"`, `"min"`, `"sup"`, `"inf"`, `"limsup"`, `"liminf"`.

---

## Integração com o Layout

```
MathLayouter::layout_node(Content::MathIdent("x")) →
  is_single_letter_var("x") = true
  is_math_function("x") = false
  → TextStyle { italic: true }

MathLayouter::layout_node(Content::MathIdent("sin")) →
  is_math_function("sin") = true
  → TextStyle { italic: false }

MathLayouter::layout_attach(base=Content::MathText("∑"), ..., block=true) →
  is_large_operator('∑') = true
  → layout vertical (limits empilhados)
```

---

## Critérios de Verificação

```
// ident_to_unicode
ident_to_unicode("alpha")  = Some("α")
ident_to_unicode("sum")    = Some("∑")
ident_to_unicode("pi")     = Some("π")
ident_to_unicode("foobar") = None
ident_to_unicode("")        = None

// shorthand_to_unicode
shorthand_to_unicode("->")  = Some("→")
shorthand_to_unicode("=>")  = Some("⇒")
shorthand_to_unicode("!=")  = Some("≠")
shorthand_to_unicode("???") = None

// is_math_function
is_math_function("sin") = true
is_math_function("x")   = false
is_math_function("Sin") = false  // case-sensitive

// is_single_letter_var
is_single_letter_var("x")  = true
is_single_letter_var("1")  = false  // dígito
is_single_letter_var("xx") = false  // multi-letra
is_single_letter_var("")   = false

// is_large_operator
is_large_operator('∑') = true
is_large_operator('∏') = true
is_large_operator('∫') = true
is_large_operator('x') = false
is_large_operator('+') = false

// is_limit_function
is_limit_function("lim")    = true
is_limit_function("max")    = true
is_limit_function("limsup") = true
is_limit_function("sin")    = false
is_limit_function("x")      = false
```
