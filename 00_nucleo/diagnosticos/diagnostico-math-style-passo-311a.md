# Diagnóstico — Passo 311a — 12 funções math style (`bb`/`cal`/`frak`/...)

**Data**: 2026-05-20
**Spec**: `00_nucleo/materialization/typst-passo-311.md`
**Sub-tipo**: diagnóstico-primeiro per ADR-0065
**Escopo escolhido**: **α** — todas as decisões §3.1-§3.5 da spec
**Caminho preliminar escolhido**: **α** — sem preferência; diagnóstico
recomenda
**Output**: este ficheiro
**Pendência**: aguarda decisão humana pós-P311a para arrancar P311b

---

## §1 — Inventário vanilla (12 funções math style)

Catálogo em `lab/typst-original/crates/typst-library/src/math/style.rs`
(312 linhas; 14 funções `#[func]` totais — spec P311 lista 12; vanilla
adicionais `display`/`inline` ficam fora do escopo P311).

Lista das 12 + 2 extras vanilla:

| # | Função | Mecanismo vanilla | Categoria |
|---:|---|---|---|
| 1 | `bold(body)` | `body.set(EquationElem::bold, true)` | **Flag** |
| 2 | `upright(body)` | `body.set(EquationElem::italic, Some(false))` | **Flag** |
| 3 | `italic(body)` | `body.set(EquationElem::italic, Some(true))` | **Flag** |
| 4 | `serif(body)` | `body.set(EquationElem::variant, Some(MathVariant::Plain))` | **Variant** |
| 5 | `sans(body)` | `body.set(EquationElem::variant, Some(MathVariant::SansSerif))` | **Variant** |
| 6 | `cal(body)` | `body.set(EquationElem::variant, Some(MathVariant::Chancery))` | **Variant** |
| 7 | `scr(body)` | `body.set(EquationElem::variant, Some(MathVariant::Roundhand))` | **Variant** |
| 8 | `frak(body)` | `body.set(EquationElem::variant, Some(MathVariant::Fraktur))` | **Variant** |
| 9 | `mono(body)` | `body.set(EquationElem::variant, Some(MathVariant::Monospace))` | **Variant** |
| 10 | `bb(body)` | `body.set(EquationElem::variant, Some(MathVariant::DoubleStruck))` | **Variant** |
| 11 | `script(body, cramped=true)` | `body.set(EquationElem::size, MathSize::Script).set(EquationElem::cramped, cramped)` | **Size** |
| 12 | `sscript(body, cramped=true)` | `body.set(EquationElem::size, MathSize::ScriptScript)` | **Size** |
| (extra) | `display(body, cramped=false)` | `MathSize::Display` | **Size** (fora P311) |
| (extra) | `inline(body, cramped=false)` | `MathSize::Text` | **Size** (fora P311) |

**Vanilla `EquationElem` fields relevantes** (deduzidos):
- `bold: bool` (default `false`)
- `italic: Option<bool>` (default `None` = lang-aware)
- `variant: Option<MathVariant>` (default `None` = Plain via context)
- `size: MathSize` (default `Text` ou `Display` conforme bloco)
- `cramped: bool`

**`MathVariant` enum** (codex::styling — crate externa não em
`lab/typst-original/crates/`; valores observados em uso):
- `Plain` (serif default — usado por `serif()`)
- `SansSerif`
- `Chancery` (cal)
- `Roundhand` (scr)
- `Fraktur` (frak)
- `Monospace` (mono)
- `DoubleStruck` (bb)

Total: **7 variants observados**. `bold` e `italic` são flags
ortogonais aplicadas sobre variant — permitindo combinatórias como
"Bold Italic Sans" via `SansSerif + bold=true + italic=Some(true)`.

**Vanilla mecanismo de resolução glyph**:
- `lab/typst-original/crates/typst-layout/src/math/text.rs:88-100`
  invoca `codex::styling::to_style(c, MathStyle::select(c, variant,
  bold, italic))` para cada caractere.
- Crate `codex` contém tabela Unicode + lookup logic.
- Cristalino **não tem** crate `codex` autorizada (P309 §5.1
  identificou `f64::*` directos como DEBT; equivalente para codex).

**`MathSize` enum vanilla** (`math/style.rs:253-260`):
```rust
pub enum MathSize { ScriptScript, Script, Text, Display }
```

**Conclusão §1**: vanilla usa **3 dimensões ortogonais** (`variant`,
`bold`, `italic`) + 1 dimensão de tamanho (`size`) + flag auxiliar
(`cramped`). Total: 5 fields independentes em `EquationElem`.

---

## §2 — Tabela Unicode variant (mapping ASCII → 12 codepoints)

### §2.1 — Plano `MATHEMATICAL ALPHANUMERIC SYMBOLS` (U+1D400-U+1D7FF)

**Estrutura geral**: 26 caracteres uppercase + 26 lowercase por
variant, blocos contíguos de 52 codepoints cada. Greek + dígitos
intercalados.

| Variant | Bloco uppercase Latin | Bloco lowercase Latin | Comentário |
|---|---|---|---|
| Bold | U+1D400-U+1D419 | U+1D41A-U+1D433 | Regular |
| Italic | U+1D434-U+1D44D | U+1D44E-U+1D467 | **`h` itálico** redireccionado para U+210E (PLANCK CONSTANT) |
| Bold Italic | U+1D468-U+1D481 | U+1D482-U+1D49B | Regular |
| Script | U+1D49C-U+1D4B5 | U+1D4B6-U+1D4CF | **9 excepções BMP** (B,E,F,H,I,L,M,R,e,g,o) |
| Bold Script | U+1D4D0-U+1D4E9 | U+1D4EA-U+1D503 | Regular |
| Fraktur | U+1D504-U+1D51D | U+1D51E-U+1D537 | **5 excepções BMP** (C,H,I,R,Z) |
| Double-struck | U+1D538-U+1D551 | U+1D552-U+1D56B | **7 excepções BMP** (C,H,N,P,Q,R,Z) |
| Bold Fraktur | U+1D56C-U+1D585 | U+1D586-U+1D59F | Regular |
| Sans-Serif | U+1D5A0-U+1D5B9 | U+1D5BA-U+1D5D3 | Regular |
| SansSerif Bold | U+1D5D4-U+1D5ED | U+1D5EE-U+1D607 | Regular |
| SansSerif Italic | U+1D608-U+1D621 | U+1D622-U+1D63B | Regular |
| SansSerif Bold Italic | U+1D63C-U+1D655 | U+1D656-U+1D66F | Regular |
| Monospace | U+1D670-U+1D689 | U+1D68A-U+1D6A3 | Regular |

**Greek capitals** (Α-Ω + ∇):
- Bold Greek: U+1D6A8-U+1D6E1
- Italic Greek: U+1D6E2-U+1D71B
- Bold Italic Greek: U+1D71C-U+1D755
- SansSerif Bold Greek: U+1D756-U+1D78F
- SansSerif Bold Italic Greek: U+1D790-U+1D7C9

**Dígitos**:
- Bold 0-9: U+1D7CE-U+1D7D7
- Double-struck 0-9: U+1D7D8-U+1D7E1
- SansSerif 0-9: U+1D7E2-U+1D7EB
- SansSerif Bold 0-9: U+1D7EC-U+1D7F5
- Monospace 0-9: U+1D7F6-U+1D7FF

### §2.2 — Excepções BMP (~21 codepoints redireccionados)

Letters que não estão no plano U+1D400-U+1D7FF e usam BMP characters:

| Variant | Excepção | Codepoint BMP |
|---|---|---|
| Italic | `h` | U+210E (ℎ PLANCK CONSTANT) |
| Script | `B` | U+212C (ℬ SCRIPT CAPITAL B) |
| Script | `E` | U+2130 (ℰ SCRIPT CAPITAL E) |
| Script | `F` | U+2131 (ℱ SCRIPT CAPITAL F) |
| Script | `H` | U+210B (ℋ SCRIPT CAPITAL H) |
| Script | `I` | U+2110 (ℐ SCRIPT CAPITAL I) |
| Script | `L` | U+2112 (ℒ SCRIPT CAPITAL L) |
| Script | `M` | U+2133 (ℳ SCRIPT CAPITAL M) |
| Script | `R` | U+211B (ℛ SCRIPT CAPITAL R) |
| Script | `e` | U+212F (ℯ SCRIPT SMALL E) |
| Script | `g` | U+210A (ℊ SCRIPT SMALL G) |
| Script | `o` | U+2134 (ℴ SCRIPT SMALL O) |
| Fraktur | `C` | U+212D (ℭ BLACK-LETTER CAPITAL C) |
| Fraktur | `H` | U+210C (ℌ BLACK-LETTER CAPITAL H) |
| Fraktur | `I` | U+2111 (ℑ BLACK-LETTER CAPITAL I) |
| Fraktur | `R` | U+211C (ℜ BLACK-LETTER CAPITAL R) |
| Fraktur | `Z` | U+2128 (ℨ BLACK-LETTER CAPITAL Z) |
| DoubleStruck | `C` | U+2102 (ℂ DOUBLE-STRUCK CAPITAL C) |
| DoubleStruck | `H` | U+210D (ℍ DOUBLE-STRUCK CAPITAL H) |
| DoubleStruck | `N` | U+2115 (ℕ DOUBLE-STRUCK CAPITAL N) |
| DoubleStruck | `P` | U+2119 (ℙ DOUBLE-STRUCK CAPITAL P) |
| DoubleStruck | `Q` | U+211A (ℚ DOUBLE-STRUCK CAPITAL Q) |
| DoubleStruck | `R` | U+211D (ℝ DOUBLE-STRUCK CAPITAL R) |
| DoubleStruck | `Z` | U+2124 (ℤ DOUBLE-STRUCK CAPITAL Z) |

**Total excepções**: ~21 codepoints BMP requirentes tabela de override.

### §2.3 — Análise de opções

**Opção α — Inline tabela completa em `entities/math_style.rs`**:
- ~52 letras × ~13 variants = ~676 entradas; em prática 7 variants
  × 52 + Greek (24) × 5 + dígitos (10) × 5 ≈ 564 entradas.
- ~250-300 LOC tabela + ~50 LOC lookup logic.
- Sem dependência externa.
- Manutenção: mapping é Unicode standard estável.

**Opção β — Crate externa**:
- Candidatos: `unicode-math-symbols`, `codex` (Typst own).
- ~3 KB binário típico.
- Autorização ADR-0018-style obrigatória — crate **deve ser pura**:
  zero I/O, zero estado mutável, zero efeitos colaterais.
- Validação adicional do mantedor + version pinning.
- **Risco**: cristalino actual tem 7 crates autorizadas L1 (ver
  ADR-0018 `[l1_allowed_external]`). Adicionar 8ª é decisão
  arquitectural não-trivial.

**Opção γ — Geração on-the-fly + tabela compacta de excepções**:
- Plano U+1D400-U+1D7FF largely regular: offset = base + (c -
  'A') ou (c - 'a').
- Tabela compacta de ~21 excepções (§2.2).
- Greek + dígitos têm bases adicionais.
- **~80-120 LOC total** (lookup function + match table).
- Sem dependência externa.

### §2.4 — Recomendação §2

**Opção γ** primária — geração on-the-fly com tabela de excepções.

Racional:
1. **Volume**: ~80-120 LOC vs ~250-300 LOC (opção α). Cristalino
   prefere código compacto onde matemática Unicode é regular.
2. **Manutenção**: lógica regular + tabela de excepções é mais
   transparente para futuros leitores que tabela ~564 entradas.
3. **Sem dependência externa**: evita ADR-0018-style decisão.
4. **Performance**: lookup O(1) por match em ~21 excepções é
   trivial.

**Opção α** secundária se diagnóstico subsequente revelar que
geração on-the-fly tem casos de borda complexos não tratáveis em
~120 LOC.

**Opção β** rejeitada: ganho marginal vs custo de dependência
externa nova.

---

## §3 — Decisões arquitecturais (§3.1-§3.5 da spec)

### §3.1 — Como representar a transformação? — Caminho I/II/III

Avaliação dos 3 caminhos propostos na spec:

#### Caminho I — Variant `Content::MathStyled`

```rust
Content::MathStyled {
    kind:    MathStyleKind,  // Bb, Cal, Frak, Sans, Mono, Scr, Plain, ...
    bold:    Option<bool>,   // None = inherit; Some(true)/Some(false) = override
    italic:  Option<bool>,   // idem
    body:    Box<Content>,
    cramped: Option<bool>,   // None = inherit; relevante para script/sscript
}
```

**Prós**:
- Composicional natural: `bb(cal(x))` → nested `MathStyled`.
- Right-to-left binding via outermost wrapping (paridade vanilla).
- MathLayouter resolve variant em walk recursivo (passing context).
- Hash content.rs **quebra** mas é mudança fundacional aceitável.
- Paralelo arquitectural directo a `Content::Styled` (já existente
  em cristalino) e `Content::MathOp` (P298).

**Contras**:
- Hash content.rs quebra (28º consecutivo termina).
- Variant adicional em enum Content (24 → 25).
- Família script/sscript precisa de variant separado OU usar
  `MathSize` field (ver §3.5).

#### Caminho II — Mapping Unicode eager

Cada função substitui ASCII por codepoint variant logo no `native_*`:
```rust
fn native_bb(args: &Args) -> SourceResult<Value> {
    let body: Content = ...; // arg
    let new_body = walk_replace_chars(body, MathStyleKind::DoubleStruck);
    Ok(Value::Content(new_body))
}
```

**Prós**:
- Sem variant novo; preserva hash.
- Implementação isolada (12 funções com mesmo template).

**Contras crítica**:
- **`bb(x)` onde `x` é `MathIdent` binding falha**: substituir
  ASCII só funciona em `MathText`/`MathIdent` literais. Se `x =
  some_expr`, o walk não tem o body resolvido ainda.
- **`bb(cal(x))` não-composicional**: outer wrap eager substituiu já
  os chars; inner walk veria os codepoints unicode e não saberia
  re-mapear.
- Limitado a strings literais.

**Conclusão**: Caminho II **rejeitado**. Cristalino tem `MathIdent`
para bindings (`MathLayouter` resolve em walk time), e eager
substitution destrói a composição.

#### Caminho III — Style enum extension

```rust
// em entities/style.rs
pub enum Style {
    // ... existentes
    MathVariant(MathStyleKind),
    MathBold(bool),
    MathItalic(Option<bool>),
}

// Aplicado via Content::Styled(body, [MathVariant(Bb)])
```

**Prós**:
- Reusa infraestrutura `Style`/`StyleChain` existente.
- Composição via merge de styles (mais "típico" da arquitectura
  vanilla).
- Hash content.rs preservado.

**Contras**:
- **DEBT-1 / StyleChain materialização**: ADR-0054 fixa perfil
  graded; StyleChain cristalina é **plana** (sem chain real). 5
  fields actualmente em `TextStyle` (bold, italic, size, fill,
  heading_level). Adicionar 3 novos fields (variant, mathbold,
  mathitalic) **estende TextStyle** mas mecanismo de consumer
  StyleChain real continua adiado.
- **Sintetiza o problema P266 §"capture sem consumer"**: capturar
  variant em StyleChain sem `MathLayouter` real consumer = estado
  intermédio, não solução.
- Mais código L1 por menos benefício directo.

**Conclusão**: Caminho III **rejeitado**. Caminho I é mais directo
para o problema concreto sem reabrir DEBT-1.

#### Tabela comparativa final

| Critério | Caminho I (Variant) | Caminho II (Eager) | Caminho III (Style) |
|---|---|---|---|
| Composicional `bb(cal(x))` | ✓ natural | ✗ falha | ✓ via merge |
| Funciona com bindings (`bb(x)` onde x var) | ✓ | ✗ | ✓ |
| Hash content.rs preservado | ✗ quebra | ✓ | ✓ |
| Reusa scaffolding existente | parcial | total | total |
| Acopla com DEBT-1 (StyleChain) | n/a | n/a | sim |
| LOC L1 estimado | ~250-350 | ~200 | ~300-400 |
| ADR nova obrigatória | sim (Math-Style-Mechanism) | não | sim (Style extension) |

#### Recomendação §3.1

**Caminho I — Variant `Content::MathStyled`** primária.

Racional:
1. **Composicional + funciona com bindings** — propriedades não-
   negociáveis para math style funcional.
2. **Paralelo arquitectural directo** ao P298 (`Content::MathOp`) e
   demais variants math em Content (MathFrac, MathAttach, etc.).
3. **Não reabre DEBT-1** — evita extender StyleChain sem consumer
   real (anti-padrão P266 §"capture sem consumer").
4. Hash content.rs quebra é **mudança fundacional aceitável** per
   ADR-0033 (paridade observable; forma diverge sem afectar output).

Caminho II rejeitado; Caminho III adiado para futura migração
quando StyleChain real for materializada.

### §3.2 — Tabela Unicode — onde fica?

Ver §2.4 acima. Recomendação: **Opção γ** (geração on-the-fly +
~21 excepções tabela).

Implementação proposta em `01_core/src/entities/math_style.rs`:

```rust
pub fn map_glyph(c: char, kind: MathStyleKind, bold: bool, italic: bool) -> char {
    // 1. Verifica excepções BMP primeiro.
    if let Some(replacement) = exceptions(c, kind, bold, italic) {
        return replacement;
    }
    // 2. Calcula offset baseado em (kind, bold, italic).
    let base = base_for(kind, bold, italic);
    let offset = if c.is_ascii_uppercase() {
        c as u32 - 'A' as u32
    } else if c.is_ascii_lowercase() {
        26 + (c as u32 - 'a' as u32)
    } else if c.is_ascii_digit() {
        return map_digit(c, kind, bold);  // dígitos só em alguns variants
    } else {
        return c;  // não-letra: passa-through
    };
    char::from_u32(base + offset).unwrap_or(c)
}

fn exceptions(c: char, kind: MathStyleKind, bold: bool, italic: bool) -> Option<char> {
    match (c, kind, bold, italic) {
        // Italic h → Planck constant
        ('h', _, false, true) => Some('ℎ'),
        // Script excepções...
        ('B', MathStyleKind::Chancery, false, _) => Some('ℬ'),
        // ... ~20 mais
        _ => None,
    }
}
```

### §3.3 — Composição entre variants

**Vanilla mecanismo confirmado**: `body.set(EquationElem::variant,
Some(X))` aplica último wrap. Aninhamento `bb(cal(x))`:
1. Inner `cal(x)` faz `body.set(variant, Chancery)`.
2. Outer `bb(...)` faz `body.set(variant, DoubleStruck)`.
3. Resolução durante `layout_glyph`: `styles.get(EquationElem::
   variant)` retorna o **último set** (outermost wrap).
4. Resultado: `bb` ganha (right-to-left binding via StyleChain).

**Cristalino com Caminho I**:
- `bb(cal(x))` → `MathStyled { kind: Bb, body: MathStyled { kind:
  Cal, body: ... }}`.
- `layout_node` walk: encontra outer `MathStyled { kind: Bb }`
  primeiro → fixa variant = Bb → desce body.
- Inner `MathStyled { kind: Cal }` durante walk recursivo:
  variant já fixado → **inner sobrescreve** se mesma família OU
  **outer ganha** se diferentes.
- **Decisão proposta**: outer ganha (paridade vanilla).
- Implementação: `layout_styled` toma variant active como
  parâmetro; ao entrar em filho `MathStyled` apenas substitui se
  ainda `None`.

**Casos especiais**:
- `bold(italic(x))`: ambos são flags ortogonais. `bold = Some(true)`
  + `italic = Some(true)` simultâneos → glyph "Bold Italic" variant.
  Implementação: outer bold passa `bold: Some(true)` para body;
  inner italic passa `italic: Some(true)` para seu body. Ambos
  flags são honrados em `layout_glyph`.
- `upright(italic(x))`: `italic` é `Option<bool>`. Outer wraps
  inner — outer ganha. `upright` põe `italic = Some(false)`.
  Inner `italic` põe `italic = Some(true)` mas é shadowed pelo
  outer durante walk. Resultado: `upright` ganha.

**Recomendação §3.3**: aplicar **outer-wins para flags
incompatíveis** (italic vs upright) e **bitwise merge para flags
ortogonais** (bold + italic = bold italic). Documentar regra em
ADR-Math-Style-Composition se humano aceitar Caminho I.

### §3.4 — Integração com `is_single_letter_var`

**Sítio cristalino afectado**: `01_core/src/engine/math/layout/mod.rs:
257-268`:

```rust
Content::MathIdent(name) => {
    let is_var  = symbols::is_single_letter_var(name)
                  && symbols::ident_to_unicode(name).is_none();
    let is_func = symbols::is_math_function(name);
    let math_style = if is_var && !is_func {
        TextStyle { italic: true, ..style.clone() }
    } else {
        TextStyle { italic: false, ..style.clone() }
    };
    self.layout_text_node(name, &math_style)
}
```

**Pós-P311 com Caminho I**:
- `bb(x)` aplica wrap `MathStyled { kind: Bb, body: MathIdent("x") }`.
- `layout_node` entra outer wrap antes de chegar a `MathIdent("x")`.
- Resolução do glyph deve aplicar variant Bb **em vez de** itálico
  automático.

**Sítio único afectado**: `layout_node` linha 257-268. Modificação
necessária:

```rust
// Tomar variant context da pilha do walker
let active_variant = self.current_math_style.variant;  // novo
let active_bold = self.current_math_style.bold;
let active_italic_override = self.current_math_style.italic;

Content::MathIdent(name) => {
    let italic_implicit = is_var && !is_func;
    let final_italic = active_italic_override.unwrap_or(italic_implicit);
    let final_bold = active_bold;
    // Mapping char-by-char via math_style::map_glyph
    let styled = name.chars().map(|c| map_glyph(c, active_variant, final_bold, final_italic)).collect();
    self.layout_text_node(&styled, &style)
}
```

**Risco**: refactor de `layout_node` toca todos os Math* variants
(MathFrac, MathAttach, etc.) para propagar context. **Magnitude
estimada do refactor**: M (sub-passo dentro de P311b).

**Sítios secundários afectados** (potencialmente):
- `MathOp`: operadores texto (e.g. `sin`) não devem receber variant
  glyph mapping mesmo dentro de `bb(...)`. Solução: `MathOp` é
  opaque ao variant; passa-through.
- `MathFrac`/`MathAttach`/etc.: propagam variant para sub-elementos.

### §3.5 — Família `script`/`sscript` — tamanho vs variant

Vanilla usa `MathSize` enum separado de `MathVariant`. **Família
distinta semanticamente**.

**Opção A — Variant separado** `MathStyleKind::Script | SScript`:
- `script(x)` → `MathStyled { kind: Script, body: x }`.
- `layout_node` ao processar `MathStyled { kind: Script }` reduz
  `style.size` por factor 0.7.
- Aninhamento `script(sscript(x))` aceitável: outer 0.7, inner 0.5
  → ambos aplicados (compose).
- **Anti-padrão potencial**: misturar variant glyph (Bb, Cal) com
  size (Script) no mesmo enum.

**Opção B — Style::Size dedicado**:
- Não usar `MathStyleKind` para script/sscript; introduzir variant
  separado `Content::MathSized { size: MathSize, body: Box<Content> }`.
- Família `script`/`sscript` é arquitecturalmente distinta de
  `bb`/`cal`/etc.

**Opção C — Excluir do escopo P311**:
- Implementar só 10 funções `bb`/`cal`/etc.; `script`/`sscript`
  ficam para passo dedicado P312.
- Cobertura calc 41/41 + 10 math style = ~95% A.7 categoria.

**Recomendação §3.5**: **Opção A** (variant separado dentro do
mesmo enum `MathStyleKind`).

Racional:
- User-facing: `script(x)`/`sscript(x)` são funções da mesma
  família spec P311 (12 funções).
- Implementação interna: pode usar dispatch no `layout_node` para
  size-related variants.
- Anti-padrão "misturar variant glyph + size" mitigado por
  comentário explicito no enum:
  ```rust
  pub enum MathStyleKind {
      // Variants glyph (afectam codepoint Unicode):
      Plain, SansSerif, Chancery, Roundhand, Fraktur, Monospace, DoubleStruck,
      // Variants size (afectam style.size factor; NÃO afectam codepoint):
      Script, SScript,
  }
  ```
- `map_glyph` retorna passa-through para Script/SScript;
  `layout_node` aplica factor multiplicativo.

Opção B adiada para futuro refactor se família scripts crescer
(adicionar `display`/`inline` vanilla).

Opção C rejeitada: spec P311 lista 12 funções; excluir 2 seria
divergência arbitrária.

---

## §4 — Comparação caminhos I/II/III (matriz prós/contras consolidada)

| Critério | Caminho I | Caminho II | Caminho III |
|---|---|---|---|
| Composição `bb(cal(x))` | ✓ outer-wins natural | ✗ destrói chars | ✓ via Style merge |
| Funciona com bindings | ✓ | ✗ critico | ✓ |
| Hash content.rs | ✗ quebra (28º consec termina) | ✓ preservado | ✓ preservado |
| Família script/sscript | ✓ variant separado | ✓ size factor | ✓ Style::Size |
| Acopla com DEBT-1 | não | não | sim |
| Risco anti-padrão "capture sem consumer" | nenhum | n/a | alto (StyleChain plana) |
| ADRs novas necessárias | 2 (Mechanism + Composition) | 0 | 1 (Style extension) |
| LOC L1 estimado | ~350-450 | ~200 | ~400-500 |
| Magnitude P311b | M-M+ | M | M+ |
| Paridade observable vanilla | ✓ idêntica | ✗ parcial | ✓ idêntica |
| Custo Unicode tabela | partilhado | partilhado | partilhado |

**Veredicto**: **Caminho I** é único viável (Caminho II falha em
bindings; Caminho III activa anti-padrão "capture sem consumer").

---

## §5 — Integração com `is_single_letter_var` (resumo)

Ver §3.4 completo. Resumo:

- **Sítio único principal**: `layout_node` em `rules/math/layout/
  mod.rs:257-268`.
- **Modificação**: walker mantém context variant/bold/italic; itálico
  automático suprimido quando `active_variant.is_some()` ou
  `active_italic_override.is_some(false)`.
- **Sítios secundários**: propagação de context para sub-elementos
  (`MathFrac`/`MathAttach`/etc.). ~5-10 sítios para context
  passing.
- **Risco testes pré-existentes**: ~5 testes em
  `rules/math/layout/tests.rs` verificam itálico automático;
  preservar comportamento default (sem wrap math style) é
  obrigatório.

---

## §6 — Composição cross-variant (resumo)

Ver §3.3 completo. Resumo:

**Regras**:
1. **Variant glyph** (`Plain`/`SansSerif`/`Chancery`/...): outer-wins
   em conflito. `bb(cal(x))` → outer Bb ganha.
2. **Bold flag**: ortogonal. `bold(bb(x))` → ambos aplicados =
   "Bold Double-Struck".
3. **Italic flag**: outer-wins (Option<bool>). `upright(italic(x))`
   → outer Upright ganha (italic = `Some(false)`).
4. **Size variant** (Script/SScript): compõe multiplicativamente.
   `script(sscript(x))` → factor 0.7 × 0.5 = 0.35.

**ADR candidata**: `ADR-Math-Style-Composition` formaliza estas 4
regras para preservar paridade vanilla. Anotação cumulativa em
ADR-0033 (paridade observable) pode bastar se humano preferir
não criar ADR nova.

---

## §7 — Família `script`/`sscript` (resumo)

Ver §3.5 completo. Resumo:

- Família semanticamente distinta (size, não glyph).
- Recomendação: **Opção A** (variant dentro do mesmo enum
  `MathStyleKind` com sub-categoria size).
- Custo: ~30 LOC para handler size factor em `layout_node`.
- Anti-padrão "misturar glyph + size" mitigado por comentário
  explícito + sub-categoria no enum.

---

## §8 — Recomendação operacional para P311b

### §8.1 — Combinação primária recomendada

| Decisão | Opção primária | Opção secundária |
|---|---|---|
| §3.1 Representação | **Caminho I** (Variant `Content::MathStyled`) | n/a (II rejeitado; III activa anti-padrão) |
| §3.2 Tabela Unicode | **Opção γ** (geração + ~21 excepções) | Opção α (inline completa ~300 LOC) |
| §3.3 Composição | **Outer-wins + ortogonalidade flags** | n/a (paridade vanilla obrigatória) |
| §3.4 `is_single_letter_var` | **Context passing em `layout_node`** | n/a (refactor obrigatório) |
| §3.5 Script/SScript | **Opção A** (variant separado mesmo enum) | Opção B (variant separado outro Content) |

### §8.2 — Estrutura proposta P311b

**Magnitude estimada**: **M+** (300-450 LOC L1 + ~30 tests + 2 ADRs
novas).

**Sub-passos sugeridos** (dentro de P311b):

| Sub-passo | Conteúdo | LOC | Tests |
|---|---|---:|---:|
| P311b.1 | `entities/math_style.rs` (enum + map_glyph + excepções) | ~120 | 8-10 |
| P311b.2 | `entities/content.rs` (variant `MathStyled`) | ~30 | 0 (variant test em P311b.4) |
| P311b.3 | `rules/stdlib/structural.rs` (12 funções nativas + registo em `math_module`) | ~150 | 12 |
| P311b.4 | `rules/math/layout/mod.rs` (handler + context passing) | ~80 | 8-10 |
| P311b.5 | E2E paridade (`lab/parity/corpus/`) — 3-5 ficheiros | n/a | 3-5 |
| P311b.6 | ADR-Math-Style-Mechanism + ADR-Math-Style-Composition + L0 actualização + relatório | n/a | n/a |

**Granularidade**: P311b pode ser passo único M+ OU série de 3-6
sub-passos S cada. Recomendação: **passo único** se humano confiar
no caminho recomendado; **série** se preferir validação intermédia.

### §8.3 — Invariantes pós-P311b

| Invariante | Estado pós-P311b |
|---|---|
| `entities/content.rs` hash | **quebra** (28º consec termina; aceitável per ADR-0033 — variant novo) |
| `export/*` snapshots | preservados (math style não toca PDF; output via texto) |
| Cobertura calc 41/41 | preservada |
| Tests pré-existentes (2 409) | preservados — refactor `layout_node` cuidadoso |
| ADRs novas | 2 (Math-Style-Mechanism + Math-Style-Composition) |
| `crystalline-lint .` | zero violations |

### §8.4 — Riscos identificados (re-avaliação P311a)

| Risco | P311 spec | P311a reavaliação |
|---|---|---|
| Caminho I exige refactor M+ adicional | Média | **Confirmado M+** — 350-450 LOC |
| Opção γ não-regular força α | Média | **Refutado** — γ é viável com ~21 excepções tabela |
| `bb(cal(x))` vanilla mais complexa que right-to-left | Baixa | **Confirmado baixa** — outer-wins via outer wrap natural |
| `layout_node` refactor quebra tests | Média | **Mitigável** — context default = None preserva comportamento |
| ADR nova bloqueada por discussão | Possível | **2 ADRs propostas** — humano decide promoção |
| Hash content.rs quebra após 27 | Aceitável | **Confirmado aceitável** — primeira quebra desde P282 (28º) |

---

## §9 — ADRs candidatas

### §9.1 — ADR-Math-Style-Mechanism (obrigatória se Caminho I)

Formaliza `Content::MathStyled` como variant arquitectural:
- Justifica preferência sobre Caminho III (StyleChain) per
  anti-padrão "capture sem consumer".
- Justifica preferência sobre Caminho II (Eager) per
  composicional/bindings.
- Documenta interacção com DEBT-1 (StyleChain) — quando StyleChain
  for materializada, `MathStyled` pode migrar para Style merge
  (paridade vanilla).
- Cross-ref ADR-0033 (paridade), ADR-0054 (graded), ADR-0026
  (Content como enum), ADR-0098 (single source of truth).

Magnitude estimada: ~180-220 linhas (paralelo ADR-0091 / ADR-0101).

### §9.2 — ADR-Math-Style-Composition (recomendada)

Formaliza 4 regras de composição §3.3:
- Variant glyph outer-wins.
- Bold ortogonal (bitwise merge).
- Italic outer-wins (Option<bool>).
- Size variant compõe multiplicativamente.

Alternativa: anotação cumulativa em ADR-0033 (paridade) sem ADR
nova. Recomendação: **ADR dedicada** porque regras são não-triviais
e precisam de visibilidade própria.

Magnitude estimada: ~100-130 linhas.

### §9.3 — Possível ADR-Math-Style-Unicode (opcional)

Documenta opção γ vs α:
- Tabela compacta de excepções inline.
- Plano U+1D400-U+1D7FF regular.
- Decisão contra crate externa.

Recomendação: **anotação inline no L0** `entities/math_style.md` em
vez de ADR dedicada. Magnitude trivial.

### §9.4 — ADRs reusadas (sem novas necessárias)

| ADR | Reuso |
|---|---|
| ADR-0017 | Estratégia gradual typst-library |
| ADR-0026 | Content como enum fechado (justifica variant novo) |
| ADR-0033 | Paridade observable (math style preserva paridade vanilla) |
| ADR-0036 | Atomização `&Args` |
| ADR-0037 | Coesão por domínio (entrada em `stdlib/structural.rs` ou novo `stdlib/math.rs`) |
| ADR-0040 | (StyleChain — referenciado para justificar Caminho III rejeitado) |
| ADR-0054 | Perfil graded (composição variant é paridade observable, não bit-exact) |
| ADR-0059 | `Args` como input vehicle |
| ADR-0098 | Single source of truth (variant em content.rs como SSOT math style) |

---

## §10 — Recomendação operacional final

**P311b deve arrancar com**:

1. **Caminho I** — Variant `Content::MathStyled { kind, bold,
   italic, body, cramped }`.
2. **Opção γ** — geração on-the-fly em `entities/math_style.rs`
   + tabela compacta de ~21 excepções BMP.
3. **Composição** — outer-wins (variant + italic) + ortogonal (bold);
   multiplicativo (size).
4. **Refactor `layout_node`** — context passing recursivo;
   suprimir itálico automático quando context define italic.
5. **Script/SScript** — variant separado no mesmo enum
   `MathStyleKind` com sub-categoria "size".
6. **2 ADRs novas**: ADR-Math-Style-Mechanism (Caminho I) +
   ADR-Math-Style-Composition (4 regras §3.3).
7. **Hash content.rs quebra explicitamente assumida** — 28º
   consecutivo termina; aceitável per ADR-0033.

**Magnitude P311b**: **M+** (350-450 LOC L1 + ~30 tests + 2 ADRs).

**Granularidade sugerida**: passo único OU 3-6 sub-passos S.
Decisão fica para humano.

**Trava arquitectural**: P311b não pode arrancar antes de humano
aceitar (ou rejeitar) o caminho recomendado.

---

## §11 — Pendências adiadas explicitamente

Decisões fora do escopo P311a:

1. **Promoção das 2 ADRs candidatas** — humano decide se aceita
   Math-Style-Mechanism + Math-Style-Composition.
2. **`display`/`inline`** (2 funções extra vanilla) — adiadas para
   futuro passo se cobertura A.7 estendida for desejada.
3. **Greek + dígitos** — implementação inicial pode cobrir só
   Latin; Greek/dígitos como sub-passo se complexidade emergir.
4. **`MathOp` interaction** — operadores texto dentro de `bb(...)`
   passam-through (não recebem variant). Validar em testes E2E.
5. **Refactor `layout_node` granularidade** — sub-passo dentro de
   P311b vs separado.

---

## §12 — Invariantes preservados (P311a)

P311a é diagnóstico-primeiro; **zero código tocado**.

| Invariante | Estado |
|---|---|
| `entities/content.rs` hash `82d3c47d` | inalterado (**27º consecutivo**) |
| `export/*` snapshots | inalterados (**3º consecutivo pós-P307**) |
| Cobertura calc 41/41 | inalterada |
| ADRs meta novas | **0** (**17ª vez consecutiva** anti-padrão P273.17 §0) |
| `crystalline-lint .` | continua zero violations |
| Testes 2 409 verdes | preservados |

---

## §13 — Sub-padrão observado

**"Diagnóstico-primeiro factual antes de materialização arquitectural
de magnitude M+"** — N=5 cumulativo (P156B Layout + P154A Model +
P307a Export + P309 IEEE 754 + **P311a Math Style**).

Limiar tentativo N=3 ultrapassado. Pattern maduro:
- Diagnóstico cataloga vanilla + cristalino.
- Avalia caminhos arquiteturais com matriz prós/contras.
- Recomenda primária + secundária.
- Humano decide arranque P*-b.

**Adiamento de promoção formal a meta-ADR** justificado: pattern já é
prática estabelecida cristalina (ADR-0065 documenta diagnóstico-
primeiro), não exige meta-ADR adicional.

---

## §14 — Fecho

P311a fechado com:

- **1 ficheiro** publicado: este diagnóstico
  (`diagnostico-math-style-passo-311a.md`).
- **Inventário §1**: 12 funções + 2 extras vanilla com signatures
  + 7 variants `MathVariant` + 4 sizes `MathSize`.
- **§2 tabela Unicode**: plano U+1D400-U+1D7FF largely regular +
  ~21 excepções BMP catalogadas; opção γ recomendada
  (~80-120 LOC).
- **§3 decisões arquitecturais §3.1-3.5 todas resolvidas**:
  Caminho I + Opção γ + Outer-wins + Context passing + Variant
  separado script/sscript.
- **§4 matriz comparativa** — Caminho II rejeitado (não-
  composicional), Caminho III rejeitado (anti-padrão capture sem
  consumer).
- **§5+§6+§7 detalhes** integração + composição + família
  script/sscript.
- **§8 recomendação operacional explícita** com sub-passos sugeridos
  para P311b.
- **§9 ADRs candidatas** — 2 novas obrigatórias (Mechanism +
  Composition) + 1 opcional (Unicode).
- **§10 trava arquitectural** explícita — P311b aguarda decisão
  humana.

**Conclusão substantiva**:

Caminho I (Variant `Content::MathStyled`) é único viável dentro das
restrições cristalinas:
- Caminho II falha em bindings (problema crítico).
- Caminho III activa anti-padrão "capture sem consumer" (DEBT-1
  StyleChain plana).

Hash `content.rs` quebra após 27 passos consecutivos é **mudança
fundacional aceitável** per ADR-0033 — paralelo arquitectural
directo a P298 (MathOp) que também adicionou variant Math.

Magnitude P311b estimada **M+** (350-450 LOC) com 2 ADRs novas.
Decisão de arranque fica para humano per ADR-0065 (diagnóstico não
força).

**Próxima acção**: humano lê P311a, decide arrancar P311b com
recomendação primária (Caminho I + Opção γ) ou ajustar. P311a fica em
standby até decisão.
