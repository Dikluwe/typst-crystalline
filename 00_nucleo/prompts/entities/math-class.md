# Prompt L0 — `math_class` entity

**Camada**: L1
**Ficheiro**: `01_core/src/entities/math_class.rs`
**ADR**: `00_nucleo/adr/typst-adr-0009-math-class.md`

---

## Contexto

`MathClass` é a classificação Unicode de símbolos matemáticos, definida
em Unicode Math Technical Report 25 (TR25). Determina o comportamento
tipográfico de cada símbolo no modo math do Typst: espaçamento automático,
prioridade de operador, associatividade.

Esta classificação é uma regra de domínio tipográfico — pertence a L1.
Movida de `typst_utils::default_math_class` (ADR-0009).

---

## Interface pública

```rust
/// Classificação Unicode de símbolos matemáticos.
///
/// Define o comportamento tipográfico de cada símbolo em modo math:
/// espaçamento automático, prioridade de operador, associatividade.
///
/// Especificação: https://www.unicode.org/reports/tr25/
///
/// Movido de `typst_utils::MathClass` — ADR-0009.
/// Completado com delegação a `unicode_math_class` — ADR-0011.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MathClass {
    Normal,
    Alphabetic,
    Binary,
    Closing,
    Diacritic,   // adicionado ADR-0011 (presente em unicode_math_class)
    Fence,
    GlyphPart,   // adicionado ADR-0011 (presente em unicode_math_class)
    Large,
    Opening,
    Punctuation,
    Relation,
    Space,
    Unary,
    Vary,
    Special,
}

/// Retorna a classe math padrão de Typst para um caractere Unicode.
///
/// Aplica primeiro os overrides Typst-específicos (ADR-0009); para os
/// restantes delega na tabela TR25 via `unicode_math_class` (ADR-0011).
/// Retorna `None` para caracteres sem classe math definida.
///
/// Origem: inlinado de `typst_utils::default_math_class` — ADR-0009.
/// Licença da tabela de overrides: Apache-2.0 (upstream Typst).
pub fn default_math_class(c: char) -> Option<MathClass>;

/// Conversão interna — não é API pública.
fn from_unicode_math_class(c: unicode_math_class::MathClass) -> MathClass;
```

---

## Critérios de verificação

### Dado/Quando/Então

**Override: dois pontos como Relação**
- Dado: `c = ':'`
- Quando: `default_math_class(':')`
- Então: `Some(MathClass::Relation)`

**Override: ponto como Normal**
- Dado: `c = '.'`
- Quando: `default_math_class('.')`
- Então: `Some(MathClass::Normal)`

**Override: barra como Normal**
- Dado: `c = '/'`
- Quando: `default_math_class('/')`
- Então: `Some(MathClass::Normal)`

**Override: reticências como Normal**
- Dado: `c = '⋯'`
- Quando: `default_math_class('⋯')`
- Então: `Some(MathClass::Normal)`

**Override: ⅋ como Binary**
- Dado: `c = '⅋'`
- Quando: `default_math_class('⅋')`
- Então: `Some(MathClass::Binary)`

**Override: ⎰ como Opening**
- Dado: `c = '⎰'`
- Quando: `default_math_class('⎰')`
- Então: `Some(MathClass::Opening)`

**Override: ⎱ como Closing**
- Dado: `c = '⎱'`
- Quando: `default_math_class('⎱')`
- Então: `Some(MathClass::Closing)`

**Override: ، como Punctuation**
- Dado: `c = '،'` (vírgula árabe U+060C)
- Quando: `default_math_class('،')`
- Então: `Some(MathClass::Punctuation)`

**Delegação para letra**
- Dado: `c = 'a'` (sem override Typst)
- Quando: `default_math_class('a')`
- Então: `Some(MathClass::Alphabetic)` — via `unicode_math_class::class` (ADR-0011)

**Delegação para emoji sem classe**
- Dado: `c = '😃'`
- Quando: `default_math_class('😃')`
- Então: `None`

**MathClass é Copy e PartialEq**
- `MathClass::Normal == MathClass::Normal`
- Variantes diferentes são diferentes

---

## Política de sincronização

Quando `lab/typst-original` for actualizado, inspeccionar diff em
`typst_utils/src/lib.rs`. Se `default_math_class` mudar, actualizar
a tabela de overrides aqui e rever este prompt.

Se `unicode-math-class` adicionar novas variantes ao enum `MathClass`,
`from_unicode_math_class` deixará de compilar (match não exaustivo) —
o compilador detecta automaticamente.

---

## Notas de implementação

- `MathClass` de L1 tem as mesmas variantes que `unicode_math_class::MathClass`
  (incluindo `Diacritic` e `GlyphPart` adicionados em ADR-0011) mas é um tipo
  distinto — L1 não expõe tipos do crate externo em assinaturas públicas
- `from_unicode_math_class` é privada (`fn`, não `pub fn`) — detalhe de impl
- Os overrides Typst-específicos têm precedência sobre a tabela TR25:
  `:` como Relation, `.`/`/` como Normal, `⋯`/`⋱`/`⋰`/`⋮` como Normal,
  `\u{22A5}` como Normal, `⅋`/`⟇` como Binary, `⎰`/`⟅` como Opening,
  `⎱`/`⟆` como Closing, `،` como Punctuation
