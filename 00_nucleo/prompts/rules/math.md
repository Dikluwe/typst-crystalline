# Prompt L0 — motor de equações

## Módulo
`01_core/src/rules/math/`

## Propósito
Motor de layout matemático — recebe `Content::Equation` e produz `Frame`s
com `FrameItem::Text` posicionados.

## Âmbito por passo
- **Passo 36**: `MathIdent`, `MathText` → `FrameItem::Text`. Restantes variantes → texto plano.
- **Passo 37+**: `MathFrac`, `MathAttach`, `MathRoot` com posicionamento vertical.
- **Passo 38+**: fontes OpenType MATH, espaçamento matemático correcto.

## Restrição arquitectural
L1 puro. Não depende de L3. Usa `FontMetrics` trait injectável.
Sem I/O de sistema. `MathLayouter` é genérico sobre `M: FontMetrics`.

## Interface pública

```rust
pub struct MathLayouter<'a, M: FontMetrics>;

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub fn new(metrics: &'a M) -> Self;
    pub fn layout_equation(
        &mut self,
        body:  &Content,
        style: &TextStyle,
    ) -> Frame;
}
```

## Critérios de verificação

- `MathIdent("x")` → `Frame` com `FrameItem::Text { text: "x", .. }` não vazio
- `MathSequence([x, +, y])` → `Frame` com 3 items
- `MathFrac { num: a, den: b }` → sem `[` nos items
- Integração no layouter principal: `Content::Equation` delega ao `MathLayouter`
