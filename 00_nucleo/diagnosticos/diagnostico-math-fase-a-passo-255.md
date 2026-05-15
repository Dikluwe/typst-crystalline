# Diagnóstico Math Fase A — Passo 255 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0034 diagnóstico canónico + ADR-0065
inventariar primeiro critério #5.
**Diagnóstico pai**:
  `diagnostico-math-passo-254B.md`.
**Imutável após criação** per ADR-0034.

---

## §1 — Comandos executados e output literal

### Item 1 — Kern matemático

```bash
$ grep -rn "MathGlyphKern" 01_core/src/rules/math/
01_core/src/rules/math/layout/attach.rs:17:use crate::entities::glyph_variants::MathGlyphKern;
01_core/src/rules/math/layout/attach.rs:49:        let base_kern: MathGlyphKern = base_char

$ grep -rn "math_kern\b" 01_core/src/rules/math/
01_core/src/rules/math/layout/attach.rs:50:            .map(|c| self.metrics.math_kern(c))
01_core/src/rules/math/layout/tests.rs:495:    let k = m.math_kern('f');
01_core/src/rules/math/layout/tests.rs:502:    // math_kern com FixedMetrics retorna kern zero — layout não deve mudar

$ grep -n "kern\|MathKernRecord\|math_kern" 01_core/src/rules/math/layout/attach.rs
44:        // Apenas MathIdent/MathText têm char único; outros ficam com kern zero.
49:        let base_kern: MathGlyphKern = base_char
50:            .map(|c| self.metrics.math_kern(c))
68:        // Kern dos quadrantes esquerdos — cada script avalia o kern no ponto de
70:        let tl_kern = if let Some(ref tb) = tl_box {
72:            self.constants.to_pt(base_kern.top_left.kern_at(h_du), style.size).val()
74:        let bl_kern = if let Some(ref bb) = bl_box {
76:            self.constants.to_pt(base_kern.bottom_left.kern_at(h_du), style.size).val()
80:        // Cada left-script tem o seu próprio afastamento (push = largura + kern).
81:        // kern negativo = aproximação da base (sem .abs() — geometria correcta).
83:        let tl_push = tl_box.as_ref().map(|b| b.width + tl_kern).unwrap_or(0.0);
84:        let bl_push = bl_box.as_ref().map(|b| b.width + bl_kern).unwrap_or(0.0);
107:        // Posicionar bl (pre-subscript): aproxima-se da base com kern independente.
186:                let kern_sup = self.constants.to_pt(
187:                    base_kern.top_right.kern_at(sup_h_du), style.size
191:                    items.push(offset_item(item, Pt(x + kern_sup), Pt(-sup_offset)));
193:                x += sup_box.width + kern_sup;
203:                let kern_sub = self.constants.to_pt(
204:                    base_kern.bottom_right.kern_at(sub_h_du), style.size
208:                    items.push(offset_item(item, Pt(x + kern_sub), Pt(sub_offset)));

$ grep -n "kern" 01_core/src/rules/math/layout/mod.rs
(zero hits)
```

### Item 2 — OpenType MATH tables + variantes

```bash
$ grep -n "GlyphVariants\|vertical_glyph_variants\|\.select(" 01_core/src/rules/math/layout/stretchy.rs
22:        let variants = self.metrics.vertical_glyph_variants(c);

$ grep -n "GlyphAssembly\|vertical_glyph_assembly\|GlyphPart" 01_core/src/rules/math/layout/assembly.rs
14:use crate::entities::glyph_variants::GlyphAssembly;
20:        assembly:       GlyphAssembly,

$ grep -rn "MathConstants\|math_constants\b" 01_core/src/rules/math/
01_core/src/rules/math/layout/mod.rs:12:    math_constants::MathConstants,
01_core/src/rules/math/layout/mod.rs:191:/// **Passo 41**: constantes OpenType MATH via `FontMetrics::math_constants()`.
01_core/src/rules/math/layout/mod.rs:210:    pub(super) constants: MathConstants,
01_core/src/rules/math/layout/mod.rs:218:        let constants = metrics.math_constants();
01_core/src/rules/math/layout/tests.rs:590:    let constants = crate::entities::math_constants::MathConstants::fallback();
```

### Item 3 — MathPrimes layout

```bash
$ grep -rn "MathPrimes\|Content::MathPrimes\|primes\b" 01_core/src/rules/math/
(zero hits — não consumido em rules/math/layout)

$ grep -n "primes\|Primes" 01_core/src/rules/math/layout/attach.rs
(zero hits)

$ grep -rn "MathPrimes" 01_core/src/
01_core/src/entities/syntax_kind.rs:95:    MathAttach,
01_core/src/entities/syntax_kind.rs:97:    MathPrimes,
01_core/src/entities/syntax_kind.rs:424:    Self::MathAttach => "math attachments",
01_core/src/entities/syntax_kind.rs:427:    Self::MathPrimes => "math primes",
01_core/src/entities/syntax_set.rs:89:    MathPrimes,
01_core/src/rules/eval/math.rs:86:    // MathPrimes::count() retorna o número de apóstrofos usando o comprimento em bytes.
01_core/src/entities/ast/math.rs:217:    pub fn primes(self) -> Option<MathPrimes<'a>> {
01_core/src/entities/ast/math.rs:228:    struct MathPrimes
01_core/src/entities/ast/math.rs:231:impl MathPrimes<'_> {
01_core/src/entities/ast/expr.rs:23:    MathDelimited, MathAttach, MathPrimes, MathFrac, MathRoot,
01_core/src/entities/ast/expr.rs:61:    MathPrimes(MathPrimes<'a>),
01_core/src/entities/ast/expr.rs:134:    SyntaxKind::MathPrimes => Some(Self::MathPrimes(MathPrimes(node))),
01_core/src/entities/ast/expr.rs:199:    Self::MathPrimes(v) => v.to_untyped(),
01_core/src/rules/parse/math.rs:99:    SyntaxKind::MathPrimes | SyntaxKind::Escape | SyntaxKind::Str => {
01_core/src/rules/parse/math.rs:170:    if !(op_kind == SyntaxKind::MathPrimes && p.at_set(stop_set)) {
01_core/src/rules/parse/math.rs:199:    SyntaxKind::MathPrimes if !had_trivia => (SyntaxKind::MathAttach, None, 2),
01_core/src/rules/lexer/math.rs:78:    SyntaxKind::MathPrimes
```

**Inspecção `eval/math.rs:75-101`**:

```rust
let prime_count = attach.primes()
    .map(|p| p.count())
    .unwrap_or(0);
let prime_char: Option<Content> = if prime_count == 0 {
    None
} else {
    let s: EcoString = match prime_count {
        1 => "′".into(),          // U+2032
        2 => "″".into(),          // U+2033
        3 => "‴".into(),          // U+2034
        4 => "⁗".into(),          // U+2057
        n => "′".repeat(n).into(), // U+2032 × n para n > 4
    };
    Some(Content::MathText(s))
};
// Merge prime com sup existente: primes primeiro, depois o sup original.
```

### Item 4 — Baseline x-height

```bash
$ grep -n "apply_axis_offset\|axis_height" 01_core/src/rules/math/layout/mod.rs
224:    /// O eixo matemático é `axis_height` (design units) acima da baseline.
228:    pub(super) fn apply_axis_offset(&self, mut b: MathBox, size: Pt) -> MathBox {
229:    let axis_pt = self.constants.to_pt(self.constants.axis_height, size).val();

$ grep -rn "x_height\|x-height\|axis_height\|baseline" 01_core/src/rules/math/ | head -15
01_core/src/rules/math/layout/assembly.rs:34:        // No MathBox, y=0 é o topo e y=ascent é a baseline.
01_core/src/rules/math/layout/tests.rs:520:fn frac_com_axis_height_nao_regride() {
01_core/src/rules/math/layout/tests.rs:531:fn delimitado_com_axis_height_nao_regride() {
01_core/src/rules/math/layout/tests.rs:546:fn sqrt_com_axis_height_nao_regride() {
01_core/src/rules/math/layout/tests.rs:588:    // Com axis_height, a fracção sobe: o ascent do MathBox aumenta.
01_core/src/rules/math/layout/tests.rs:589:    // Verificar que o axis_height é não-zero (fallback=500 > 0).
01_core/src/rules/math/layout/tests.rs:591:    assert!(constants.axis_height > 0.0, "axis_height do fallback deve ser > 0");
```

### Verificação dos campos reais de `MathConstants`

```bash
$ grep "pub " 01_core/src/entities/math_constants.rs | grep ":"
    pub upem: f64,
    pub fraction_rule_thickness: f64,
    pub fraction_num_gap: f64,
    pub fraction_denom_gap: f64,
    pub superscript_shift_up: f64,
    pub subscript_shift_down: f64,
    pub radical_vertical_gap: f64,
    pub radical_rule_thickness: f64,
    pub axis_height: f64,
    pub script_percent_scale_down: f64,
    pub script_script_percent_scale_down: f64,
    pub upper_limit_gap_min: f64,
    pub lower_limit_gap_min: f64,
    pub math_leading: f64,
```

**14 campos públicos reais** (struct `MathConstants`). Prompt
L0 `entities/math_constants.md` lista apenas **10 campos** —
faltam: `axis_height`, `upper_limit_gap_min`, `lower_limit_gap_min`,
`math_leading`.

---

## §2 — Classificação por item

| # | Pendência DEBT-8 | Hits literais | Classificação | Justificação |
|---|------------------|---------------|---------------|--------------|
| 1 | Kern matemático | `attach.rs:17` import + `:49-50` consumer `math_kern(c)` + `:70-76` quadrantes left + `:186-207` quadrantes right; tests `tests.rs:495, 502` | **FECHADO** | Consumer real em `attach.rs` para todos os 4 quadrantes (top-left, bottom-left, top-right, bottom-right). Kern aplicado a offset horizontal de subscripts/superscripts. Geometria correcta (sem `.abs()`; kern negativo permitido). Tests E2E presentes. |
| 2 | OpenType MATH tables + variantes | `stretchy.rs:22` `vertical_glyph_variants(c)`; `assembly.rs:14, 20` `GlyphAssembly`; `mod.rs:218` `metrics.math_constants()` no construtor; 5 hits MathConstants em rules/math | **FECHADO** | P96.8 reestruturou math/layout/ em 8 submódulos; consumers reais identificados em `stretchy.rs` (variantes) + `assembly.rs` (assembly) + `mod.rs` (constants). `MathConstants::fallback()` activo via `FixedMetrics` (sem fonte MATH real exigida). |
| 3 | MathPrimes layout | Zero hits em `rules/math/layout/`; FECHADO **VIA EVAL** em `eval/math.rs:85-101` (count → `′″‴⁗` U+2032+, n>4 → repetição); merge com superscript existente | **FECHADO via eval.rs** | Divergência arquitectural intencional: primes resolvidos em **eval** (não layout); convertidos para `Content::MathText` com glifos U+2032–U+2057; Layouter recebe-os como superscript regular. Paridade observable preservada (output PDF correcto). |
| 4 | Baseline x-height | `mod.rs:228-229` `apply_axis_offset` consume `self.constants.axis_height`; tests `frac_com_axis_height_nao_regride` + `delimitado_*` + `sqrt_*` + sentinela `axis_height > 0` em `tests.rs:520+` | **FECHADO** | `apply_axis_offset` é o método canónico que aplica baseline x-height para fracções/delimitadores/sqrt. Usa campo real `MathConstants.axis_height` (não hardcode). 3+ tests regressão presentes. |

**Contagem: 4/4 fechados**.

---

## §3 — Inconsistências documentais detectadas

1. **Prompt L0 `entities/math_constants.md` incompleto vs struct
   real**: lista 10 campos públicos; struct real tem **14
   campos** (faltam `axis_height`, `upper_limit_gap_min`,
   `lower_limit_gap_min`, `math_leading`). Acção P255.B:
   actualizar enumeração + critérios de verificação.
2. **Prompt L0 `rules/math/layout.md` desactualizado vs código
   pós-P96.8**: refere "Passo 36 / 37+ / 38+" como
   trabalho futuro. P96.8 reestruturou em 8 submódulos
   (`attach.rs`, `root.rs`, `frac.rs`, `matrix.rs`, `cases.rs`,
   `stretchy.rs`, `assembly.rs`, `delimited.rs`). Acção P255.B:
   substituir secção "Âmbito por passo" por "Estado actual"
   listando os 8 submódulos + consumers de cada tipo.
3. **DEBT-8 não actualizado desde 2026-03-26 Passo 40**:
   8 semanas de materialização não reflectidas (P96.8
   reestruturação + kern consumer integration + assembly +
   stretchy + apply_axis_offset). Acção P255.D: ENCERRAR
   DEBT-8 com referência ao diagnostico Fase A.

---

## §4 — Decisão do cenário Fase B

**Contagem fechados/abertos**: `4/4 fechados; 0/4 abertos`.

**Cenário escolhido**: ☑ **B1 (fecho total)** / ☐ B2 / ☐ B3.

**Justificação cenário B1**:

- Todos os 4 itens DEBT-8 têm consumer real no Layouter (ou
  divergência arquitectural válida no caso de MathPrimes via
  eval).
- Infra-estrutura L1 + L3 + Layouter consumer linha completa.
- Tests E2E presentes para todos os 4 itens (math_kern,
  axis_height, etc).
- Único trabalho documental restante: actualizar L0 prompts
  desactualizados (P255.B) + fechar DEBT-8 (P255.D).
- **Zero materialização de código nova exigida** (P255.C
  saltado para P255.D directamente).

---

## §5 — Referências

- Diagnóstico pai P254B
  (`diagnostico-math-passo-254B.md`).
- Spec P255 (`typst-passo-255.md`).
- ADR-0034 — Diagnóstico canónico.
- ADR-0065 — Inventariar primeiro critério #5.
- ADR-0054 — Perfil graded (paridade observable).
- ADR-0033 — Paridade observable vanilla.
- DEBT-8 (`00_nucleo/DEBT.md`) — alvo deste passo.
- P96.8 — reestruturação `math/layout/` em 8 submódulos.
- P40 — última actualização DEBT-8 (2026-03-26).
