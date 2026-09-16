# Prompt L0 — `entities/elements/math_accent` — `MathAccentElem`
Hash do Código: 5c99c7dd

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_accent.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub. Mecanismo vanilla:
`AccentElem` minimal em P296; P1429 propõe fechar `size`, `dotless`,
normalização e orientação (gate ADR-0127 pendente).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathAccentElem {
    pub base:   Content,   // era Box<Content>
    pub accent: Content,   // era Box<Content>
}
```

`Content::MathAccent { base, accent }` → `Content::MathAccent(Arc<MathAccentElem>)`.
Construtor ergonómico: `Content::math_accent(base: Content, accent: Content)`.

## `impl Element for MathAccentElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `format!("{}{}", self.base.plain_text(), self.accent.plain_text())` (`content.rs:1645`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em `base`/`accent` (`content.rs:2131`) |
| `map_text` | **terminal** (math structural; `content.rs:2631`): `Content::MathAccent(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `base + accent` (paridade `content.rs:1831`).

## Critério

`plain_text` concat `base+accent`; `map_content` recurse ambos; `map_text`
terminal; igualdade estrutural.

## P1429 — carrier completo (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O carrier P296 guarda o acento como `Content` e não consegue provar
single-codepoint, orientação, tamanho, dotless nem presença explícita. O
vanilla ratificado guarda um `Accent(char)` normalizado, `size` relativo com
default `100%` e `dotless=true`.

### Contrato público proposto

`MathAccentElem` passa a guardar `base: Content`, `accent: char`,
`size: Rel<Length>`, `dotless: bool` e `MathAccentExplicit { size, dotless }`.
O tipo de presença é público, `Copy + Eq + Hash`; o elemento implementa
`PartialEq`/`Hash` observacional incluindo valores e bits, com floats
normalizados pelo padrão vigente dos carriers relativos. `map_content` desce
somente em `base`; `map_text` continua terminal; `plain_text` concatena base e
codepoint canônico. Toda reconstrução preserva valores e presença.
