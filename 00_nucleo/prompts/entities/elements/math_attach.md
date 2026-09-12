# Prompt L0 — `entities/elements/math_attach` — `MathAttachElem`
Hash do Código: 82736684

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Camada**: L1
**Alvo**: `01_core/src/entities/elements/math_attach.rs`
**ADRs**: ADR-0105, ADR-0107, ADR-0129

## Responsabilidade

`MathAttachElem` é o payload fechado e não-locatável dos anexos matemáticos.
Ele preserva, para cada slot, a diferença morfológica entre argumento omitido,
`none` explícito e conteúdo presente.

## Tipos canônicos

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum MathAttachSlot {
    Omitted,
    ExplicitNone,
    Present(Content),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathAttachElem {
    pub base: Content,
    pub t: MathAttachSlot,
    pub b: MathAttachSlot,
    pub tl: MathAttachSlot,
    pub bl: MathAttachSlot,
    pub tr: MathAttachSlot,
    pub br: MathAttachSlot,
}
```

`Content::MathAttach(Arc<MathAttachElem>)` é o único payload. O construtor
canônico é `Content::math_attach(base, t, b, tl, bl, tr, br)`.

`Omitted` representa ausência do argumento, `ExplicitNone` representa o
valor Typst `none` e `Present(content)` inclui `Present(Content::Empty)`
para markup `[]`. É proibido substituir os três estados por sentinel de
`Content`, texto, span ou heurística.

`sup()` devolve primeiro `tr: Present` e depois `t: Present`; `sub()`
faz o paralelo `br`/ `b`. Estados omitidos ou `none` não bloqueiam esse
fallback.

## Element

- `plain_text` concatena `^tl _bl base ^t _b ^tr _br` somente para slots
  `Present`.
- `map_content` transforma recursivamente a base e cada conteúdo `Present`,
  preservando os três estados.
- `map_text` é terminal e devolve o elemento clonado.
- `is_empty`, `get_field`, `element_kind` e `to_payload` conservam os
  defaults da família não-locatável.
- Igualdade e hash incluem base e os seis slots.

## Aceitação

Sintaxe e `math.attach` compartilham o payload. `repr` omite `Omitted`,
imprime `none` para `ExplicitNone` e `[]` para conteúdo vazio presente.
Layout materializa somente `Present`; por isso markup vazio ainda reserva
`SpaceAfterScript`, enquanto omissão e `none` são equivalentes no layout.
