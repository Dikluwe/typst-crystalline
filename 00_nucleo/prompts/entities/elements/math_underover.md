# Prompt L0 — `entities/elements/math_underover` — `MathUnderoverElem`
Hash do Código: 294b148a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_underover.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub. Variant agregado P297
(ADR-0054 graded; cristalino unifica o cluster vanilla under/over).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathUnderoverElem {
    pub base:  Content,            // era Box<Content>
    pub under: Option<Content>,    // era Option<Box<Content>>
    pub over:  Option<Content>,    // era Option<Box<Content>>
}
```

`Content::MathUnderover { base, under, over }` →
`Content::MathUnderover(Arc<MathUnderoverElem>)`. Construtor ergonómico:
`Content::math_underover(base: Content, under: Option<Content>, over: Option<Content>)`.

## `impl Element for MathUnderoverElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | ordem visual `over + base + under` (`Option` vazio = `""`; `content.rs:1648`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** nos 3 campos preservando `None` (`content.rs:2139`): `base.map_content(f)?`; `under`/`over` via `.as_ref().map(map_content).transpose()?` |
| `map_text` | **terminal** (math structural; `content.rs:2635`): `Content::MathUnderover(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `base + under + over` (paridade `content.rs:1835`).

## Critério

`plain_text` `over‖base‖under`; `map_content` recurse os 3 preservando `None`;
`map_text` terminal; igualdade estrutural.

## P1439 — identidade pública `MathOverbraceElem` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

O carrier agregado vigente representa a geometria, mas apaga a função pública
`overbrace` e não distingue anotação omitida de `annotation: none`. O vanilla
preserva ambos os estados e usa U+23DE acima do body.

### Decisão proposta

No mesmo owner coeso, adicionar `MathOverbraceElem { body: Content,
annotation: Option<Content>, annotation_explicit: bool }` com constructor que
preserva presença. `plain_text` segue a ordem visual `annotation + ⏞ + body`;
`map_content` desce no body e na anotação; `map_text` conserva a variante.
Igualdade/hash incluem os três campos. O carrier genérico `MathUnderoverElem`
permanece separado e não ganha identidade inferida.

## P1440 — identidade pública `MathOverbracketElem` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O carrier agregado representa a geometria, mas apaga a função pública
`overbracket` e não distingue anotação omitida de `annotation: none`. O
vanilla preserva ambos os estados e usa U+23B4 acima do body.

### Decisão proposta

No mesmo owner coeso, adicionar `MathOverbracketElem { body: Content,
annotation: Option<Content>, annotation_explicit: bool }` com constructor que
preserva presença. `plain_text` segue a ordem visual `annotation + ⎴ + body`;
`map_content` desce no body e na anotação; `map_text` conserva a variante.
Igualdade/hash incluem os três campos. Os carriers `MathOverbraceElem` e
`MathUnderoverElem` permanecem identidades separadas.

## P1442 — identidade pública `MathOverparenElem` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O carrier agregado realiza spreaders, mas apaga a função pública
`overparen` e não distingue annotation omitida de `annotation: none`. O
vanilla preserva ambos os estados e usa U+23DC acima do body.

### Decisão proposta

No mesmo owner coeso, adicionar `MathOverparenElem { body: Content,
annotation: Option<Content>, annotation_explicit: bool }` com construtor que
preserva presença. `plain_text` segue a ordem visual `annotation + ⏜ + body`;
`map_content` desce no body e na annotation; `map_text` conserva a variante.
Igualdade/hash incluem os três campos. `MathOverbraceElem`,
`MathOverbracketElem` e `MathUnderoverElem` permanecem identidades separadas.

## P1443 — identidade pública `MathOvershellElem` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O carrier agregado realiza spreaders, mas apaga a função pública `overshell`
e não distingue annotation omitida de `annotation: none`. O vanilla preserva
ambos os estados e usa U+23E0 acima do body.

### Decisão proposta

No mesmo owner coeso, adicionar `MathOvershellElem { body: Content,
annotation: Option<Content>, annotation_explicit: bool }` com construtor que
preserva presença. `plain_text` segue a ordem visual `annotation + ⏠ + body`;
`map_content` desce no body e na annotation; `map_text` conserva a variante.
Igualdade/hash incluem os três campos. Os outros spreaders públicos e
`MathUnderoverElem` permanecem identidades separadas.

## P1448 — identidade pública `MathUnderbraceElem` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O carrier agregado vigente realiza a geometria, mas apaga a função pública
`underbrace` e não distingue annotation omitida de `annotation: none`. O
vanilla preserva ambos os estados e usa U+23DF abaixo do body.

### Decisão proposta

No mesmo owner coeso, adicionar `MathUnderbraceElem { body: Content,
annotation: Option<Content>, annotation_explicit: bool }` com construtor que
preserva presença. `plain_text` segue a ordem visual `body + ⏟ + annotation`;
`map_content` desce no body e na annotation; `map_text` conserva a variante.
Igualdade/hash incluem os três campos. Os outros spreaders públicos e
`MathUnderoverElem` permanecem identidades separadas.
