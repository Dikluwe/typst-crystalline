# Prompt L0 — `entities/elements/figure_caption` — `FigureCaptionElem`
Hash do Código: 3ad4663a

**Estado**: MATERIALIZADO (confirmação humana 2026-09-21, posterior ao commit
causal somente-L0 `3ac60a578`; execução segregada A/B/C — ver
`typst-passo-1473.md` §8).

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/figure_caption.rs`
**Origem**: **P1473** (`typst-passo-1473.md`), família das oito rotas públicas não
matemáticas restantes. Owner do consumer: `compiler/stdlib/figure_image.md`
(construção); síntese/realização permanecem fora desta fatia — ver lá.
**Vanilla ratificado**: `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
`typst-library/src/model/figure.rs:511-591`.

---

## Escopo reduzido — construção sem síntese

Ver `compiler/stdlib/figure_image.md` §P1473 para a medição completa e a
justificação do corte de fase. Esta entidade só dá forma pública a
`figure.caption(...)`; não participa da síntese (`kind`/`supplement`/
`numbering`/`counter`) nem do layout de figuras nesta fatia.

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FigureCaptionElem {
    /// Vanilla default: `bottom`. Settable; sem efeito de layout nesta fatia.
    pub position:  Option<CaptionPosition>,  // enum { Top, Bottom }
    /// Vanilla default: Auto (idioma/região). `None` aqui = não definido
    /// explicitamente; o default por idioma não é resolvido nesta fatia.
    pub separator: Option<Content>,
    pub body:      Content,
}
```

`Content::FigureCaption(Arc<FigureCaptionElem>)`.

Construtor: `Content::figure_caption(body: Content, position:
Option<CaptionPosition>, separator: Option<Content>) -> FigureCaptionElem`.

`CaptionPosition` é um enum fechado de dois variantes (`Top`, `Bottom`) —
não reutilizar `Align2D`/`OuterVAlignment` genérico de layout se esse tipo
carregar mais variantes que os dois válidos aqui; cast deve rejeitar
qualquer string fora de `"top"`/`"bottom"` com a mensagem medida
(`` expected `top` or `bottom`, found <valor> ``).

## `Element` — mínimo viável

`plain_text()` delega a `body.plain_text()`. `is_empty()` = `body.is_empty()`.
`map_content`/`map_text` recursam em `body`; se `separator` vier `Some`,
recursar nele também, por simetria com os demais elementos de conteúdo do
crate.

## `eq`/`Hash`

`#[derive(PartialEq, Hash)]` sobre os três campos.

## Não-objetivos

Não implementa síntese (`kind`/`supplement`/`numbering`/`counter`/
`figure_location`), `.realize()` (texto final combinando supplement +
numeração via `Counter::display_at`), resolução do `separator` default por
idioma/região, nem o erro "not known at this point" do vanilla para campos
ainda não sintetizados (aqui, ausência de sintetizado é simplesmente
ausência de campo — divergência mecânica aceitável, ADR-0107). Reabertura
de qualquer um destes exige medição própria e gate ADR-0127.

## Materialização

Agente A (`01a82593d`) criou este ficheiro e a entidade `FigureCaptionElem`
(struct, `Element`, `Content::FigureCaption`/constructor) sem ver os
testes. O ficheiro não tinha nenhum teste próprio até a integração (papel
C, 2026-09-21) — `crystalline-lint` V2 exige verificação simultânea por
módulo do núcleo; acrescentados 5 testes locais (`plain_text`/`is_empty`,
preservação de `position`/`separator`, `map_text`, `map_content`,
`eq`/`Hash`), cobrindo o mesmo padrão já usado por `list_item.rs`. Suíte:
GREEN.
