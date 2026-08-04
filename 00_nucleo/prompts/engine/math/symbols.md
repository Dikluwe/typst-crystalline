# Prompt L0 — `rules/math/symbols` — Resolução de Símbolos Matemáticos
Hash do Código: deb987d0

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/math/symbols.rs`
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
- Operadores e símbolos: `sum`→`∑`, `prod`→`∏` (não-canónico, mantido por
  compatibilidade — ver nota P780), `product`→`∏` (nome canónico, `codex`
  `sym.txt:525`), `integral`→`∫`, `infty`→`∞`, `partial`→`𝜕` (U+1D715,
  **P902** — não U+2202/∂ upright; confirmado por compilação directa contra
  o vanilla real, `partial` produz sempre a variante itálica, em qualquer
  contexto — diferente do mecanismo de itálico automático de `alpha`/etc.,
  que `apply_math_default`/`is_math_italic_default` aplica a `MathText` de 1
  carácter mas exclui `∂` por não ser classificado como "letra"; `nabla`→`∇`
  fica correctamente upright nos dois binários, confirma que nem todo
  símbolo italiciza — `partial` é caso à parte na tabela, mesmo padrão do
  `dot`/`⋅` corrigido em P894)
- Lógica: `forall`→`∀`, `exists`→`∃`, `in`→`∈`, `notin`→`∉`
- Conjuntos: `subset`→`⊂`, `union`→`∪`, `inter`→`∩`, `emptyset`→`∅`
- Aritmética: `times`→`×`, `div`→`÷`, `pm`→`±`, `cdot`→`·`
- Ellipses: `dots`/`ldots`→`…`, `cdots`→`⋯`, `vdots`→`⋮`, `ddots`→`⋱`
- Relações: `approx`→`≈`, `equiv`→`≡`, `propto`→`∝`, `perp`→`⊥`
- Especiais: `hbar`→`ℏ`, `ell`→`ℓ`, `Re`→`ℜ`, `Im`→`ℑ`, `aleph`→`ℵ`
- Double-struck de conjuntos numéricos (P812-D, paridade codex `sym.txt`):
  `NN`→`ℕ` (U+2115), `RR`→`ℝ` (U+211D), `ZZ`→`ℤ` (U+2124), `QQ`→`ℚ`
  (U+211A), `CC`→`ℂ` (U+2102). O restante do alfabeto double-struck do
  codex (`AA`, `BB`, `DD`, ...) fica scope-out — adicionar on-demand se o
  corpus exigir.

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

Retorna `true` se o caractere é um **operador grande** (classe `Large`
vanilla). Sozinho, **não decide** empilhamento — ver `is_integral_char`
abaixo; quem decide empilhamento é o caller (`math/layout/attach.rs`),
combinando os dois.

Operadores reconhecidos:
- Somatório/Produto: `∑` `∏` `∐`
- União/Intersecção: `⋃` `⋂` `⨄` `⨅` `⨆`
- Integrais: `∫` `∬` `∭` `∮` `∯` `∰`
- Outros: `⨁` (oplus) `⨂` (otimes) `⨀` (odot) `⋀` `⋁`

### `is_integral_char(c: char) -> bool` (P772w)

Retorna `true` se o caractere é um sinal de integral — faixas `'∫'..='∳'`
(U+222B–U+2233) e `'⨋'..='⨜'` (U+2A0B–U+2A1C), idênticas ao vanilla
(`math/attach.rs::is_integral_char`, `typst-library`).

**Uso**: em `math/layout/attach.rs`, `is_limits` só empilha limites
verticalmente para operadores da classe `Large` que **não** sejam
integrais — `is_large_operator(c) && !is_integral_char(c)`. Paridade
vanilla `Limits::for_char_with_class`: `MathClass::Large` → `Limits::Never`
se `is_integral_char`, senão `Limits::Display`. Integrais mantêm os
scripts **sempre ao lado**, mesmo em modo bloco/display — `∫_0^1` nunca
empilha `0`/`1` como `∑_0^1` empilha.

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
ident_to_unicode("alpha")    = Some("α")
ident_to_unicode("sum")      = Some("∑")
ident_to_unicode("integral") = Some("∫")  // P772w — era "int" (nome errado)
ident_to_unicode("int")      = None       // P772w — "int" é o tipo inteiro, não um símbolo
ident_to_unicode("product")  = Some("∏")  // P780 — nome canónico vanilla (codex sym.txt:525)
ident_to_unicode("pi")       = Some("π")
ident_to_unicode("foobar")   = None
ident_to_unicode("")         = None

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

// is_integral_char (P772w)
is_integral_char('∫') = true
is_integral_char('∮') = true  // contour integral
is_integral_char('∳') = true  // limite superior da faixa
is_integral_char('⨌') = true  // quadruple integral (segunda faixa)
is_integral_char('∑') = false
is_integral_char('∏') = false

// is_limit_function
is_limit_function("lim")    = true
is_limit_function("max")    = true
is_limit_function("limsup") = true
is_limit_function("sin")    = false
is_limit_function("x")      = false
```

---

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|-------------------|
| 2026-07-17 | P772w — `ident_to_unicode`: `"int"` (errado — colide com o tipo inteiro, não é um símbolo no vanilla) substituído por `"integral"` (correcto, paridade `sym.rs`). Nova função `is_integral_char` (paridade vanilla), consumida por `math/layout/attach.rs` para excluir integrais do empilhamento de limites em modo bloco (`is_large_operator(c) && !is_integral_char(c)`) — `∫_0^1` mantém os scripts ao lado, `∑_0^1` empilha | `symbols.md`, `symbols.rs`, `layout/attach.rs` |
| 2026-07-17 | P780 — `ident_to_unicode`: adicionado `"product"` → `"∏"` (nome canónico, `codex` `sym.txt:525`; confirmado que `$product$` resolve no vanilla real, `$prod$` erra). `"prod"` (não-canónico, sem entrada em `codex`) mantido por compatibilidade retroactiva — remoção não avaliada, fora de âmbito. Exposto por regressão de `layout_prod_com_limites_nao_panica` após P780 fechar o fallback silencioso de `MathIdent` desconhecido (`eval.md` §P780) | `symbols.md`, `symbols.rs` |


## §P958 — nomes gregos em falta + resolução de símbolo no fallback de `FuncCall`

**Medição** (`typst-passo-958` Fase A): os 7 casos reportados
(`Phi(𝑥)`, `chi(𝑀)`, `Gamma(𝑧)`, `zeta(𝑠)`, `Psi(𝑥,`, `chi(𝐺)`,
`omega(𝐺)` — todos `Nome(args)` a sair literal em vez do glifo grego) têm
**uma só causa**: o fallback do braço `Expr::FuncCall` de `eval_math_expr`
(`eval/math.rs`) testava apenas `lookup_math_op` antes de cair no literal
`Content::MathIdent(name)` — nunca consultava a tabela de símbolos. O
caminho standalone (`$ Gamma $`) resolve porque o braço `Expr::MathIdent`
consulta `ident_to_unicode` → `lookup_math_op` → `sym_lookup`. Não há
padrão maiúscula/minúscula — é uma causa única.

**Inventário contra o vanilla** (`codex` 0.3.0 `modules/sym.txt`): faltavam
em `ident_to_unicode` 13 nomes gregos canónicos — minúsculos `digamma` (ϝ)
e `omicron` (ο); maiúsculos `Chi` (Χ), `Eta` (Η), `Iota` (Ι), `Kappa` (Κ),
`Mu` (Μ), `Nu` (Ν), `Omicron` (Ο), `Rho` (Ρ), `Tau` (Τ), `Upsilon` (Υ),
`Zeta` (Ζ). `$ Chi $` standalone dava `unknown variable` (ausente das duas
tabelas consultadas).

**Correcção**:
1. `ident_to_unicode` ganha os 13 nomes em falta (paridade codex).
2. O fallback do `FuncCall` passa a espelhar a cadeia do braço standalone:
   `lookup_math_op` → **`ident_to_unicode`** → **`sym_lookup`** (com o
   warning de depreciação de P820, mesma paridade) → literal `MathIdent`
   (P303, preservado para nomes realmente desconhecidos). A prioridade de
   operadores (`sin`, `lim`) sobre símbolos mantém-se — a cadeia de símbolos
   entra DEPOIS de `lookup_math_op`.
