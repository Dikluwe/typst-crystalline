# Prompt L0 — `entities/elements/math_underline` — `MathUnderlineElem`

**Estado:** CONTRATO P1292 AGUARDA SELO ADR-0127 — sem consumer e sem
`Hash do Código` até a materialização posterior ao gate humano.

**Camada:** L1
**Alvo planejado:** `01_core/src/entities/elements/math_underline.rs`
**Regras comuns:** `entities/elements/_comum.md`
**Vanilla ratificado:** `a51e02804`

## Medição anterior à decisão

O cristalino possui `entities/elements/underline.rs`, mas essa unidade é a
decoração textual com `stroke`, `offset` e `extent`. O vanilla declara o
elemento matemático separado e body-only em
`lab/typst-original/crates/typst-library/src/math/underover.rs:4-14`; o probe
de linguagem de 2026-08-31 confirma `math.underline == underline` como `false`.
Logo a equivalência visual parcial não autoriza alias de identidade.

## Contrato público P1292

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathUnderlineElem {
    pub body: Content,
}
```

Elemento math estrutural não-locatável. `plain_text` delega ao body;
`map_content` recursa no body e reconstrói `MathUnderline`; `map_text` é
terminal; igualdade/hash incluem body; campos/introspection de elemento usam
os defaults não-locatáveis. O construtor público é
`Content::math_underline(body)`. A função `math.underline` aceita exatamente
um `Content` posicional obrigatório; strings seguem o cast canónico para
`Content`. Zero posicionais produz `missing argument: body`, tipo
não-convertível produz `expected content, found <tipo vanilla>`, segundo
posicional produz `unexpected argument` e qualquer named produz
`unexpected argument: <nome>`.

É proibido reutilizar `Content::Underline`, transportar campos textuais
`stroke/offset/extent` ou expor o nome de função textual. A geometria pertence
exclusivamente a `compiler/math/layout/underline.md`, e a variante ao owner
`entities/content.md`.

## Aceitação linguística

`math.underline([x])` preserva identidade `MathUnderline`, plain text `x` e um
único body. Sua forma pública é `underline(body: [x])`, e
`math.underline == underline` permanece `false`. Falta, excesso, tipo inválido
e named desconhecido coincidem com as mensagens medidas no recibo vanilla
P1292; não se aceita argumento ignorado.
