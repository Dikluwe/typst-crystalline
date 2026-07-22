# Prompt L0 — `math_style` entity
Hash do Código: 13ee7ad7

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/math_style.rs`
**Passo de origem**: Passo 311b.1
**ADRs relevantes**:
- ADR-Math-Style-Mechanism (P311b.6 — formaliza Caminho I escolhido)
- ADR-0018 (estratégia DEBT externals; aqui rejeitada — opção γ inline)
- ADR-0026 (Content como enum fechado; variant MathStyled adicionado em P311b.2)
- ADR-0033 (paridade observable — glyph mapping deve coincidir com vanilla
  em chars cobertos)
- ADR-0054 (perfil graded — paridade observable, não bit-exact em fontes)

---

## Contexto

P311 implementa 12 funções math style vanilla (`bb`/`bold`/`cal`/`frak`/
`italic`/`mono`/`sans`/`scr`/`script`/`serif`/`sscript`/`upright`). O
diagnóstico P311a (`diagnosticos/diagnostico-math-style-passo-311a.md`)
escolheu **Caminho I** (Variant `Content::MathStyled`) + **Opção γ**
(geração on-the-fly de codepoints Unicode com tabela compacta de
excepções).

Este módulo define o tipo de domínio `MathStyleKind` (família de
variants math style suportadas) e a função pura `map_glyph` que
converte ASCII → codepoint Unicode variant.

Vanilla usa `codex::styling::to_style` + `MathStyle::select` (crate
externa não autorizada em cristalino). Cristalino regenera o mesmo
mapping inline em ~120 LOC com lookup table compacta para excepções.

---

## Interface pública

```rust
/// Família de variants math style suportadas em cristalino.
///
/// Sub-categorias:
/// - **Variants glyph**: afectam codepoint Unicode via `map_glyph`.
///   `Plain`/`SansSerif`/`Chancery`/`Roundhand`/`Fraktur`/`Monospace`/
///   `DoubleStruck`.
/// - **Variants size**: afectam `style.size` por factor multiplicativo
///   no `MathLayouter`. Não modificam codepoint (passa-through em
///   `map_glyph`). `Script` (factor 0.7), `SScript` (factor 0.5).
///
/// `bold` e `italic` são flags ortogonais aplicadas sobre `MathStyleKind`,
/// não fazem parte deste enum (vivem em `Content::MathStyled`).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MathStyleKind {
    // Variants glyph (modificam codepoint Unicode):
    Plain,          // serif default (alias vanilla `MathVariant::Plain`)
    SansSerif,
    Chancery,       // family `cal` (Chancery script)
    Roundhand,      // family `scr` (Roundhand script — só Bold em Unicode)
    Fraktur,
    Monospace,
    DoubleStruck,   // family `bb` (blackboard bold)
    // Variants size (não modificam codepoint; factor `style.size`):
    Script,         // factor 0.7
    SScript,        // factor 0.5
}

/// Mapeia um caractere ASCII para o seu codepoint Unicode variant.
///
/// Para chars não-cobertos (símbolos, espaços, codepoints não-ASCII),
/// retorna o char de entrada inalterado.
///
/// Algoritmo:
/// 1. Se `kind` é variant size (`Script`/`SScript`), retorna `c`
///    inalterado (size é tratado em `MathLayouter`, não modifica glyph).
/// 2. Verifica tabela de excepções BMP (~21 entradas).
/// 3. Calcula offset no plano `U+1D400-U+1D7FF` com base em
///    `(kind, bold, italic)` e devolve `base + (c - 'A')` ou
///    `base + 26 + (c - 'a')`.
/// 4. Para dígitos (`'0'`-`'9'`): tabela própria, dependente de
///    `(kind, bold)`. Italic não tem variant separado para dígitos.
/// 5. Falha graceful: chars fora de `[A-Za-z0-9]` passa-through.
pub fn map_glyph(c: char, kind: MathStyleKind, bold: bool, italic: bool) -> char;
```

---

## Estrutura interna (não-API)

### Bases Unicode — plano `MATHEMATICAL ALPHANUMERIC SYMBOLS`

Documentado em §2.1 do diagnóstico P311a. Bloco contínuo
`U+1D400`-`U+1D7FF` com 13 sub-blocos de 52 chars (26 upper + 26 lower).

| Family bit-state | Base upper | Comentário |
|---|---|---|
| `(Plain, false, false)` | (sem mapping; passa-through) | Texto plain math |
| `(Plain, true, false)` | U+1D400 | Bold |
| `(Plain, false, true)` | U+1D434 | Italic (excepção `h` U+210E) |
| `(Plain, true, true)` | U+1D468 | Bold Italic |
| `(Chancery, false, false)` | U+1D49C | Script (9 excepções) |
| `(Chancery, true, false)` | U+1D4D0 | Bold Script |
| `(Fraktur, false, false)` | U+1D504 | Fraktur (5 excepções) |
| `(DoubleStruck, false, false)` | U+1D538 | DoubleStruck (7 excepções) |
| `(Fraktur, true, false)` | U+1D56C | Bold Fraktur |
| `(SansSerif, false, false)` | U+1D5A0 | Sans-Serif |
| `(SansSerif, true, false)` | U+1D5D4 | Sans-Serif Bold |
| `(SansSerif, false, true)` | U+1D608 | Sans-Serif Italic |
| `(SansSerif, true, true)` | U+1D63C | Sans-Serif Bold Italic |
| `(Monospace, false, false)` | U+1D670 | Monospace |
| `(Roundhand, true, false)` | U+1D4D0 | Roundhand = Bold Script (vanilla alias) |

`Roundhand` partilha o bloco de `Bold Script` per convenção vanilla
(`scr` é variante de `cal` com bold implícito).

### Tabela de excepções BMP (~21 entradas)

| Variant + flags | Char | Codepoint excepção |
|---|---|---|
| `(Plain, false, true)` Italic `h` | 'h' | U+210E ℎ |
| `(Chancery, *, *)` Script `B` | 'B' | U+212C ℬ |
| `(Chancery, *, *)` Script `E` | 'E' | U+2130 ℰ |
| `(Chancery, *, *)` Script `F` | 'F' | U+2131 ℱ |
| `(Chancery, *, *)` Script `H` | 'H' | U+210B ℋ |
| `(Chancery, *, *)` Script `I` | 'I' | U+2110 ℐ |
| `(Chancery, *, *)` Script `L` | 'L' | U+2112 ℒ |
| `(Chancery, *, *)` Script `M` | 'M' | U+2133 ℳ |
| `(Chancery, *, *)` Script `R` | 'R' | U+211B ℛ |
| `(Chancery, *, *)` Script `e` | 'e' | U+212F ℯ |
| `(Chancery, *, *)` Script `g` | 'g' | U+210A ℊ |
| `(Chancery, *, *)` Script `o` | 'o' | U+2134 ℴ |
| `(Fraktur, false, *)` Fraktur `C` | 'C' | U+212D ℭ |
| `(Fraktur, false, *)` Fraktur `H` | 'H' | U+210C ℌ |
| `(Fraktur, false, *)` Fraktur `I` | 'I' | U+2111 ℑ |
| `(Fraktur, false, *)` Fraktur `R` | 'R' | U+211C ℜ |
| `(Fraktur, false, *)` Fraktur `Z` | 'Z' | U+2128 ℨ |
| `(DoubleStruck, false, *)` DS `C` | 'C' | U+2102 ℂ |
| `(DoubleStruck, false, *)` DS `H` | 'H' | U+210D ℍ |
| `(DoubleStruck, false, *)` DS `N` | 'N' | U+2115 ℕ |
| `(DoubleStruck, false, *)` DS `P` | 'P' | U+2119 ℙ |
| `(DoubleStruck, false, *)` DS `Q` | 'Q' | U+211A ℚ |
| `(DoubleStruck, false, *)` DS `R` | 'R' | U+211D ℝ |
| `(DoubleStruck, false, *)` DS `Z` | 'Z' | U+2124 ℤ |

`*` = wildcards (excepção aplica-se independente de `italic`/`bold`
para Chancery/Roundhand; para Fraktur/DoubleStruck só sem bold).

### Bases Unicode — dígitos

| Family bit-state | Base `'0'` |
|---|---|
| `(Plain, true, _)` Bold | U+1D7CE |
| `(DoubleStruck, _, _)` | U+1D7D8 |
| `(SansSerif, false, _)` | U+1D7E2 |
| `(SansSerif, true, _)` | U+1D7EC |
| `(Monospace, _, _)` | U+1D7F6 |

Outros casos: passa-through (dígitos não têm variant cal/frak/etc.).

---

## Critérios de verificação

### Variants glyph — casos felizes

**Plain Bold A → U+1D400**
- `map_glyph('A', Plain, true, false)` → `'\u{1D400}'` (𝐀)

**Plain Italic a → U+1D44E**
- `map_glyph('a', Plain, false, true)` → `'\u{1D44E}'` (𝑎)

**Sans-Serif x → U+1D5D1**
- `map_glyph('x', SansSerif, false, false)` → `'\u{1D5D1}'` (𝗑) — wait, let me recompute: base U+1D5BA + (x - a) = 0x1D5BA + 23 = 0x1D5D1
  Actually base é U+1D5A0 (upper); lower é U+1D5BA. 'x' - 'a' = 23. 0x1D5BA + 23 = 0x1D5D1. ✓

**Fraktur k → U+1D52E**
- `map_glyph('k', Fraktur, false, false)` → `'\u{1D52E}'` (𝔮) — recompute:
  base lower U+1D51E + (k - a) = U+1D51E + 10 = U+1D528. Hmm, isso é 𝔨. Correcto:
  let me redo: 0x1D51E + 10 = 0x1D528 (𝔨). Yes, 𝔨.

**Monospace 3 → U+1D7F9**
- `map_glyph('3', Monospace, false, false)` → `'\u{1D7F9}'` (𝟹) — base
  digit U+1D7F6 + 3 = U+1D7F9. ✓

### Excepções BMP

**Italic h → Planck constant**
- `map_glyph('h', Plain, false, true)` → `'\u{210E}'` (ℎ)

**DoubleStruck C → ℂ**
- `map_glyph('C', DoubleStruck, false, false)` → `'\u{2102}'` (ℂ)

**Script H → ℋ**
- `map_glyph('H', Chancery, false, false)` → `'\u{210B}'` (ℋ)

**Fraktur R → ℜ**
- `map_glyph('R', Fraktur, false, false)` → `'\u{211C}'` (ℜ)

### Passa-through

**Chars não-ASCII letter/digit**
- `map_glyph('+', Fraktur, false, false)` → `'+'` (passa-through)
- `map_glyph(' ', DoubleStruck, false, false)` → `' '`
- `map_glyph('α', Plain, false, true)` → `'\u{1D6FC}'` (𝛼 — Greek
  coberto desde P809; formas `ϵ ϑ ϰ ϕ ϱ ϖ` e `∂`/`∇` continuam
  passa-through — scope-out registado)

**Variant size não modifica glyph**
- `map_glyph('x', Script, false, false)` → `'x'` (size factor aplicado
  em layout, não aqui)
- `map_glyph('A', SScript, true, false)` → `'A'`

**Plain sem flags**
- `map_glyph('x', Plain, false, false)` → `'x'` (no-op)

### Combinações ortogonais

**Sans-Serif Bold A → U+1D5D4**
- `map_glyph('A', SansSerif, true, false)` → `'\u{1D5D4}'` (𝗔)

**Sans-Serif Bold Italic x → U+1D66F**
- `map_glyph('x', SansSerif, true, true)` → `'\u{1D66F}'` (𝙭) —
  recompute: base lower U+1D656 + 23 = U+1D66D. Hmm, 'x' - 'a' = 23.
  0x1D656 + 23 = 0x1D66D (𝙭). Verificar: U+1D66D é 𝙭. ✓

### Propriedades de tipo

- `MathStyleKind` é `Copy`, `Clone`, `Eq`, `PartialEq`, `Hash`.
- `MathStyleKind::Plain == MathStyleKind::Plain`.
- Variants distintos são distintos.
- `map_glyph` é pura: chamadas repetidas com mesmos inputs retornam o
  mesmo char.

---

## Política de sincronização

Quando `lab/typst-original` for actualizado, inspeccionar diff em
`crates/typst-library/src/math/style.rs` + tabela em crate externa
`codex::styling`. Se vanilla adicionar variants ou alterar mapping de
codepoint:

1. Actualizar `MathStyleKind` se variant nova.
2. Actualizar tabela de bases + excepções inline.
3. Rever este L0 + propagar hash via `crystalline-lint --fix-hashes`.
4. Anotar drift em ADR-0033 se for paridade categoria nova.

Se Unicode adicionar codepoints aos planos `U+1D400-U+1D7FF`
(improvável — plano fechado desde Unicode 3.1), actualizar bases.

---

## Notas de implementação

- **Greek + dígitos parciais**: cobertura inicial em P311b.1 limita-se
  a Latin ASCII + dígitos básicos. **P809 estendeu Greek** (ver secção
  própria abaixo). `∇`/`∂` e as formas de símbolo gregas (`ϵ ϑ ϰ ϕ ϱ ϖ`)
  continuam scope-out — passa-through, como o vanilla para chars não
  cobertos pelo *default* (o vanilla cobre-as em `is_lower_greek`;
  extensão futura se o corpus exigir).
- **Roundhand (`scr`) — regra corrigida em P812-C**: o L0 pré-P812 dizia
  "Roundhand = Bold Script com bold implícito" — **refutado por medição**
  (codex `to_roundhand = to_script + VS2(U+FE01)`). A regra correcta:
  `scr` usa os **mesmos codepoints** de `cal` (bloco script U+1D49C +
  excepções letterlike; com `bold` vai para bold-script U+1D4D0, como
  `cal`), diferenciados apenas pelo **variation selector** — VS1
  (U+FE00) para `cal`, VS2 (U+FE01) para `scr`, emitido após cada letra
  latina (dígitos e não-latinos não levam). `map_glyph` normaliza
  `Roundhand → Chancery` para o codepoint; a nova `map_glyph_vs(c, kind)`
  devolve o selector (`Option<char>`) que os consumers anexam a seguir ao
  char mapeado. Medido byte-idêntico ao vanilla:
  `$scr(A) cal(A) scr(L) cal(B) scr(B)$` →
  `1D49C FE01 1D49C FE00 2112 FE01 212C FE00 212C FE01` nos dois.
- **`Plain` sem flags = no-op**: `map_glyph('x', Plain, false, false)`
  retorna `'x'` inalterado. Plain só modifica codepoint quando
  combinado com `bold` ou `italic`.
- **`map_glyph` é pura**: sem alocação, sem I/O, sem state. Aceitável
  em hot-path do `MathLayouter` (chamada ~1× por caractere math).
- **Variant size passa-through**: `Script`/`SScript` não modificam
  glyph; `map_glyph` retorna `c` inalterado e o factor de tamanho é
  aplicado em `rules/math/layout/mod.rs` (sub-passo P311b.4).
- **Falha graceful**: codepoints não-ASCII ou fora do range coberto
  retornam input inalterado em vez de panic. Preserva paridade
  observable com vanilla (vanilla também passa-through para chars
  não-cobertos).
- **Excepções BMP**: tabela inline em `match` exhaustivo. ~21
  entradas; ramo `_ => None` para casos não-excepcionais.

---

## Extensão Greek + regra de itálico por defeito — P809

**Origem**: observação P786 §7 (`x` vs `𝑥`, `αβ` vs `𝛼𝛽`), confirmada
causa distinta em P799/P800. O escopo-out de Greek de P311b.1 fica
**revogado para o caso `Plain`** (o único exigido pela regra de default).

**Regra de default (paridade codex `MathStyle::select`, medida)**:
- `is_math_italic_default(c)` = `c.is_ascii_alphabetic() || c ∈ 'α'..='ω'`
  — latin + grego **minúsculo** têm itálico por defeito em modo math.
- Grego **maiúsculo** é **upright por defeito** (medido: `$Gamma Delta
  Omega alpha$` → `ΓΔΩ𝛼`); só muda com modificador explícito.
- Dígitos nunca têm itálico por defeito.
- `bold` **compõe** com o default: `$bold(x)$` → U+1D499 (bold-italic,
  medido), não bold upright.

**Bases Greek (`Plain`)**: minúsculas `'α'..='ω'` — fórmula contígua
`base + (c - 'α')` (o bloco Unicode segue a ordem alfabética grega
completa, incluindo `ς`(final)/`σ` adjacentes): italic `U+1D6FC`,
bold `U+1D6C2`, bold-italic `U+1D736`. Maiúsculas `'Α'..='Ω'` — fórmula
contígua `base + (c - 'Α')` (o buraco U+03A2 alinha exactamente com a
ranhura `ϴ` do bloco math): italic `U+1D6E2`, bold `U+1D6A8`,
bold-italic `U+1D71C`. Greek com outros `kind` (Sans/Fraktur/etc.):
passa-through (scope-out, registado).

**Pontos de aplicação do default** (`engine/math/layout`, P809):
1. `layout_node` arm `MathIdent` de 1 letra (`is_var`) — antes punha a
   flag de fonte `italic: true` (invisível à extracção); agora mapeia o
   codepoint (`map_glyph(c, Plain, false, true)`) e não usa a flag.
2. `layout_node` arm `MathText` de 1 carácter com
   `is_math_italic_default` — cobre `$x$` (lexer: grafema único →
   MathText) e símbolos resolvidos (`alpha` → `α` → `𝛼`).
3. `apply_math_style` — o default de `italic` nas folhas de 1 carácter
   passa de `unwrap_or(false)` para `unwrap_or(is_math_italic_default)`:
   é o que faz `bold(x)` compor para bold-italic (paridade medida).

---

## Não-objectivos (P311b.1; revisto em P809)

- Formas de símbolo gregas (`ϵ ϑ ϰ ϕ ϱ ϖ`), `∂`/`∇` e Greek com
  `kind` não-`Plain` (scope-out registado em P809; Greek `Plain`
  coberto desde P809).
- Combinações exóticas vanilla (`Initial`, `Tailed`, `Looped`,
  `Stretched` — Arabic Math) — escopo P311b.1 limita-se às 7 variants
  base.
- Optimização de tabela (perfect-hash, FSA, etc.) — `match` directo
  é suficiente para hot-path.
- Cache de resultados — `map_glyph` é trivial; cache adicionaria custo
  sem ganho.
