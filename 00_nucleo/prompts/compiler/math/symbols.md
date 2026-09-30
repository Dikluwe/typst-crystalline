# L0 — math/symbols

Hash do Código: 53b8abb9

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/symbols.rs`

## Consumer evidence

`01_core/src/compiler/math/symbols.rs:10` locates `pub fn ident_to_unicode(name: &str) -> Option<&'static str>`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Pure symbol and operator classification

Preserve case-sensitive `ident_to_unicode(&str) -> Option<&'static str>` and
`shorthand_to_unicode` mappings; unknown names/forms return None, not a
fallback glyph. Preserve Greek, operator, relation, set, arrow and dotted
aliases already represented by the tables. Mathematical morphology and exact
Unicode codepoints are significant.

In particular: `product` and compatible alias `prod` map to ∏;
`integral` maps to ∫; `int` is not a symbol alias. `partial` maps to
𝜕 (U+1D715), not upright ∂; `nabla` remains ∇. Preserve omicron/digamma,
capital Greek names, NN/RR/ZZ/QQ/CC and the corrected dot and arrow forms.
Do not add aliases based only on typography or a plausible function name.

`is_math_function` identifies the existing upright function-name repertoire.
`is_single_letter_var` recognizes a single ASCII alphabetic byte, not any
Unicode word. `is_large_operator` identifies large operators including
integrals, but does not decide stacked attachments. `is_integral_char`
covers U+222B–U+2233 and U+2A0B–U+2A1C. `is_limit_function` recognizes
lim, max, min, sup, inf, limsup and liminf. Keep these distinct predicates.

The evaluator owns name precedence, call fallback and diagnostics under
`00_nucleo/prompts/compiler/eval/math.md`; this owner supplies lookup data
only. Attachment policy belongs to
`00_nucleo/prompts/compiler/math/layout/attach.md`.
Acceptance checks mappings, case sensitivity, unknown results and predicate
boundaries; table breadth is not evidence of global mathematical parity.

## Preserved ident_to_unicode mappings

| Names/forms | Unicode string |
| --- | --- |
| `"alpha"` | `α` |
| `"beta"` | `β` |
| `"gamma"` | `γ` |
| `"delta"` | `δ` |
| `"epsilon"` | `ε` |
| `"zeta"` | `ζ` |
| `"eta"` | `η` |
| `"theta"` | `θ` |
| `"iota"` | `ι` |
| `"kappa"` | `κ` |
| `"lambda"` | `λ` |
| `"mu"` | `μ` |
| `"nu"` | `ν` |
| `"xi"` | `ξ` |
| `"pi"` | `π` |
| `"rho"` | `ρ` |
| `"sigma"` | `σ` |
| `"tau"` | `τ` |
| `"upsilon"` | `υ` |
| `"phi"` | `φ` |
| `"chi"` | `χ` |
| `"psi"` | `ψ` |
| `"omega"` | `ω` |
| `"digamma"` | `ϝ` |
| `"omicron"` | `ο` |
| `"Alpha"` | `Α` |
| `"Beta"` | `Β` |
| `"Gamma"` | `Γ` |
| `"Delta"` | `Δ` |
| `"Epsilon"` | `Ε` |
| `"Theta"` | `Θ` |
| `"Lambda"` | `Λ` |
| `"Xi"` | `Ξ` |
| `"Pi"` | `Π` |
| `"Sigma"` | `Σ` |
| `"Phi"` | `Φ` |
| `"Psi"` | `Ψ` |
| `"Omega"` | `Ω` |
| `"Chi"` | `Χ` |
| `"Eta"` | `Η` |
| `"Iota"` | `Ι` |
| `"Kappa"` | `Κ` |
| `"Mu"` | `Μ` |
| `"Nu"` | `Ν` |
| `"Omicron"` | `Ο` |
| `"Rho"` | `Ρ` |
| `"Tau"` | `Τ` |
| `"Upsilon"` | `Υ` |
| `"Zeta"` | `Ζ` |
| `"sum"` | `∑` |
| `"prod"` | `∏` |
| `"product"` | `∏` |
| `"integral"` | `∫` |
| `"infty"` | `∞` |
| `"partial"` | `\u{1D715}` |
| `"nabla"` | `∇` |
| `"forall"` | `∀` |
| `"exists"` | `∃` |
| `"in"` | `∈` |
| `"notin"` | `∉` |
| `"subset"` | `⊂` |
| `"supset"` | `⊃` |
| `"union"` | `∪` |
| `"inter"` | `∩` |
| `"emptyset"` | `∅` |
| `"times"` | `×` |
| `"div"` | `÷` |
| `"pm"` | `±` |
| `"mp"` | `∓` |
| `"cdot"` | `·` |
| `"dots"` | `…` |
| `"ldots"` | `…` |
| `"cdots"` | `⋯` |
| `"vdots"` | `⋮` |
| `"ddots"` | `⋱` |
| `"approx"` | `≈` |
| `"sim"` | `∼` |
| `"cong"` | `≅` |
| `"equiv"` | `≡` |
| `"propto"` | `∝` |
| `"perp"` | `⊥` |
| `"parallel"` | `∥` |
| `"angle"` | `∠` |
| `"circ"` | `∘` |
| `"bullet"` | `•` |
| `"star"` | `★` |
| `"dagger"` | `†` |
| `"hbar"` | `ℏ` |
| `"ell"` | `ℓ` |
| `"Re"` | `ℜ` |
| `"Im"` | `ℑ` |
| `"aleph"` | `ℵ` |
| `"NN"` | `\u{2115}` |
| `"RR"` | `\u{211D}` |
| `"ZZ"` | `\u{2124}` |
| `"QQ"` | `\u{211A}` |
| `"CC"` | `\u{2102}` |

## Preserved shorthand_to_unicode mappings

| Names/forms | Unicode string |
| --- | --- |
| `"=>"` | `⇒` |
| `"==>"` | `⟹` |
| `"<=>"` | `⇔` |
| `"->>"` | `↠` |
| `"->"` | `→` |
| `"-->"` | `⟶` |
| `"<-"` | `←` |
| `"<--"` | `⟵` |
| `"<->"` | `↔` |
| `"<-->"` | `⟷` |
| `"&#124;->&#124;"` | `↦` |
| `"!="` | `≠` |
| `"<="` | `≤` |
| `">="` | `≥` |
| `"<<"` | `≪` |
| `"<<<"` | `⋘` |
| `">>"` | `≫` |
| `">>>"` | `⋙` |
| `":="` | `≔` |
| `"::="` | `⩴` |
| `"=:"` | `≕` |
| `"..."` | `…` |
| `".."` | `‥` |

Upright function names: `sin`, `cos`, `tan`, `cot`, `sec`, `csc`, `arcsin`, `arccos`, `arctan`, `sinh`, `cosh`, `tanh`, `log`, `ln`, `exp`, `lim`, `limsup`, `liminf`, `max`, `min`, `sup`, `inf`, `det`, `tr`, `rank`, `dim`, `ker`, `im`, `gcd`, `lcm`, `mod`, `div`, `Pr`, `Var`, `Cov`, `E`, `sqrt`, `root`.
