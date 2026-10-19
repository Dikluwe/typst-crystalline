# Prompt L0 — `entities/elements/figure_caption` — `FigureCaptionElem`
Hash do Código: b5aa27a8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:923f1c9e4c2fd5934316ae2eef3dbb461c3bef3009de1d8931d951e793dda342

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

## Completar projeção da caption existente — 2026-10-17

Medição anterior à decisão: caption-shape-before.json e caption-after.json em
lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17 fixam fontes, UTC,
binários, HEAD/working tree de validation-result.json e pin
9f2b6e8715237cb086899a42873660fe744622e8. Projeção de caption já produz o
carrier, mas sua CLI imprime só func:figure.caption; body/getters/fields estão
ausentes. json.encode tem o mesmo defeito. Metadata de kind rejeita a função
de caption válida. O pin em model/figure.rs:510 declara caption Locatable e
em :511-594 declara position,separator,body e kind/supplement/numbering/counter
sintetizados públicos, com figure_location interno. Não sintetizar esses dados
nesta correção; a função de elemento já existe desde a aprovação P1473.

Responsabilidade exclusiva e aceitação:

Sobrescrever apenas Element::get_field existente: body sempre conhecido;
position Some Top/Bottom retorna alinhamento vertical, separator Some retorna
Content original; Option None continua desconhecido, sem default assado. Não
mudar struct, métodos do trait, construtor, síntese ou phase; não reabrir os
não-objetivos de síntese/location desta entidade. É completar a reflexão dos
campos que o contrato aprovado já conserva. Content delega ao owner.

Correção interna/paridade em fluxo contínuo ADR-0127. Não há novo campo/trait/
assinatura Rust pública, fase, modo ou default. L0 antes de testes/código, RED
real de getters/metadados/encoder/CLI, GREEN e corpus externo sem relaxar
expectativas de corpo/nome. Revalidar todos os gates da etapa sob 12 GiB, swap
zero, jobs1. Resselo só dos owners após V15/V26. FG1/SY1 permanecem incompletos
conforme follow-ups.md; nenhuma API pendente está autorizada por esta seção.
