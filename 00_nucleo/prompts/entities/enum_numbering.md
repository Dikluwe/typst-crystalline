# Prompt L0 — `entities/enum_numbering` — `EnumNumbering`
Hash do Código: 9346876e

**Camada**: L1 · **Alvo**: `01_core/src/entities/enum_numbering.rs`
**Origem**: P470 — Marcadores configuráveis de `enum`.

---

## Tipo

```rust
use ecow::EcoString;

/// Esquema de numeração de lista ordenada. Subset minimal P470.
#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub enum EnumNumbering {
    /// Arábico com ponto: `"1."`, `"2."`, ... (default).
    #[default]
    Decimal,
    /// Alpha minúsculo com parêntese: `"a)"`, `"b)"`, ...
    LowerAlpha,
    /// Alpha maiúsculo com parêntese: `"A)"`, `"B)"`, ...
    UpperAlpha,
    /// Romano minúsculo com parêntese: `"i)"`, `"ii)"`, ..., `"xii)"`.
    LowerRoman,
    /// Pattern não reconhecido — armazenado sem parse. Renderiza
    /// como Decimal (fallback). Scope-out: parse completo de patterns.
    Custom(EcoString),
}

impl EnumNumbering {
    /// Formata o número `n` (1-based) de acordo com o esquema.
    pub fn format(&self, n: u32) -> String {
        match self {
            Self::Decimal    => format!("{}.", n),
            Self::LowerAlpha => format!("{})", nth_alpha(n, false)),
            Self::UpperAlpha => format!("{})", nth_alpha(n, true)),
            Self::LowerRoman => format!("{})", to_roman_lower(n)),
            Self::Custom(_)  => format!("{}.", n),  // fallback Decimal
        }
    }

    /// Parse a partir de um pattern string Typst.
    /// `"1."` → `Decimal`, `"a)"` → `LowerAlpha`,
    /// `"A)"` → `UpperAlpha`, `"i)"` → `LowerRoman`,
    /// outro → `Custom(pattern)`.
    pub fn from_pattern(p: &str) -> Self {
        match p {
            "1." | "1"  => Self::Decimal,
            "a)"        => Self::LowerAlpha,
            "A)"        => Self::UpperAlpha,
            "i)"        => Self::LowerRoman,
            other       => Self::Custom(other.into()),
        }
    }
}
```

## Helpers privados no módulo

```rust
/// Letra do alfabeto para posição `n` (1-based).
/// n=1 → 'a'/'A', n=26 → 'z'/'Z', n=27 → 'aa'/'AA', etc.
fn nth_alpha(n: u32, upper: bool) -> String { ... }

/// Romano minúsculo simples para n ∈ 1..=12.
/// n > 12 → fallback decimal string.
fn to_roman_lower(n: u32) -> String { ... }
```

Implementação de `nth_alpha`: `(n-1) % 26` → índice em a-z/A-Z. Ciclos > 26: duplica a letra (aa, bb...) — divergência intencional (Typst usa sequências completas mas esse é scope-out P470).

Implementação de `to_roman_lower`: lookup estático para 1-12:
`["i","ii","iii","iv","v","vi","vii","viii","ix","x","xi","xii"]`. n > 12 → `n.to_string()`.

## Scope-out explícito (P470)

- Parse completo de patterns `"(1a)"`, offsets, numbered forms com
  prefixo/sufixo arbitrário.
- `UpperRoman` (`"I)"`, `"II)"`, ...) — variante reservada mas não
  implementada neste passo.
- Ciclos de `nth_alpha` para n > 26 (apenas primeira passagem por ora).

## Critério

- `EnumNumbering::Decimal.format(1)` == `"1."`.
- `EnumNumbering::Decimal.format(3)` == `"3."`.
- `EnumNumbering::LowerAlpha.format(1)` == `"a)"`.
- `EnumNumbering::LowerAlpha.format(2)` == `"b)"`.
- `EnumNumbering::UpperAlpha.format(1)` == `"A)"`.
- `EnumNumbering::LowerRoman.format(1)` == `"i)"`.
- `EnumNumbering::LowerRoman.format(4)` == `"iv)"`.
- `EnumNumbering::from_pattern("a)")` == `LowerAlpha`.
- `EnumNumbering::from_pattern("1.")` == `Decimal`.
- `EnumNumbering::from_pattern("?!")` == `Custom("?!")`.
- `Default::default()` == `Decimal`.
