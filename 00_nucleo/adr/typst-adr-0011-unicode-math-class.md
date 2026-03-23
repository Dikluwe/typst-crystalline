# ⚖️ ADR-0011: `unicode_math_class` → `[l1_allowed_external]`

**Status**: `PROPOSTO`
**Data**: 2026-03-23

---

## Contexto

O lexer e o parser matemático usam `unicode_math_class::MathClass`
e a lookup associada para classificar símbolos matemáticos Unicode
segundo o standard Unicode Math (TR25).

Esta crate é distinta de `entities/math_class.rs` (ADR-0009):

| Módulo | Propósito |
|--------|-----------|
| `entities/math_class.rs` | Overrides tipográficos do Typst — subconjunto com decisões de design próprias |
| `unicode_math_class` | Classificação exaustiva de todos os caracteres math Unicode segundo TR25 |

O fluxo no lexer é: consultar `unicode_math_class` para a classe
base do caractere → aplicar overrides de `default_math_class`
(ADR-0009) se existirem → usar a classe resultante para decisões
de espaçamento e parsing.

Sem `unicode_math_class`, o parser matemático do Typst não consegue
classificar a vasta maioria dos símbolos math Unicode. Implementar
a tabela TR25 completa em L1 manualmente seria reproduzir dados de
um standard extenso com centenas de entradas.

---

## Análise de pureza

| Propriedade | Estado |
|-------------|--------|
| Zero I/O | ✓ — tabelas compiladas em tempo de compilação |
| Zero estado global mutável | ✓ — funções puras sobre dados estáticos |
| Determinismo total | ✓ — mesma entrada, mesma saída em qualquer ambiente |
| Dependências transitivas | ✓ — zero dependências externas |

---

## Decisão

`unicode_math_class` é adicionado a `[l1_allowed_external]`:

```toml
[l1_allowed_external]
rust = ["thiserror", "comemo", "unicode_ident", "unicode_math_class"]
```

Com esta autorização, `default_math_class` em `entities/math_class.rs`
pode completar o wildcard que ficou pendente em ADR-0009:

```rust
// entities/math_class.rs — antes (ADR-0009, incompleto)
pub fn default_math_class(c: char) -> Option<MathClass> {
    // overrides Typst...
    _ => None   // ← placeholder até ADR-0011
}

// entities/math_class.rs — após ADR-0011
use unicode_math_class::UnicodeClass;

pub fn default_math_class(c: char) -> Option<MathClass> {
    // overrides Typst primeiro...
    _ => UnicodeClass::of(c).map(math_class_from_unicode)
}
```

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/entities/math-class.md` | Actualizar — completar `default_math_class` com fallback via `unicode_math_class`; referenciar ADR-0011 |
| `00_nucleo/prompts/rules/parse.md` | Documentar `unicode_math_class` como externo autorizado |

---

## Consequências

**Positivas**: `default_math_class` fica completo (ADR-0009 fecha
a dívida do wildcard `_ => None`); parser matemático classifica
correctamente todos os símbolos Unicode.

**Negativas**: Segunda crate Unicode em `[l1_allowed_external]` —
o padrão está estabelecido (ADR-0010) e a justificação é idêntica.

**Neutras**: `MathClass` de L1 e `unicode_math_class::UnicodeClass`
coexistem — o primeiro é o tipo de domínio do Typst, o segundo é
a fonte de dados do standard. A conversão `math_class_from_unicode`
fica em `entities/math_class.rs`.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Inlining da tabela TR25 em L1 | Zero dependências | Tabela extensa; risco de divergência com standard |
| Mover classificação math para L3 | L1 sem crate | O parser matemático de L1 deixa de conseguir classificar operadores — semanticamente incorrecto |

---

## Referências

- Unicode Math TR25: https://www.unicode.org/reports/tr25/
- `unicode-math-class`: https://github.com/typst/unicode-math-class
- ADR-0009 — `default_math_class` em `entities/math_class.rs`
- Diagnóstico Passo 4 — `MathClass` em lexer.rs e parser.rs
