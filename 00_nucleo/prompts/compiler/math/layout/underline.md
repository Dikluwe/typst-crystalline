# Prompt L0 — `compiler/math/layout/underline` — underline matemático

**Estado:** MATERIALIZADO P1292; mecanismo inline v3 refutado e substituído
pelo amendment-3/v4 no owner `compiler/layout/equation.md`.
Hash do Código: 99b6401e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:42a441e59c53fdc5bf619005c0e018eb63251533fb6dc6c16c13d000e592e40c

**Camada:** L1
**Alvo:** `01_core/src/compiler/math/layout/underline.rs`
**Owner comum:** `compiler/math/layout/_comum.md`
**Entidade:** `entities/elements/math_underline.md`

## Medição anterior à decisão

No vanilla ratificado, `UnderlineElem` resolve para uma line item abaixo do
body e o layout usa `underbar_vertical_gap`, `underbar_rule_thickness` e
`underbar_extra_descender` em
`lab/typst-original/crates/typst-layout/src/math/line.rs:11-80`. O body de
underline não é cramped. O cristalino já tem constantes MATH equivalentes,
mas o braço inline atual recebe a identidade textual, que não pode servir de
owner do elemento math.

## Decisão P1292

Uma free function `underline::layout(layouter, elem, style)` dispõe o body no
mesmo estilo matemático, sem forçar cramped, e adiciona uma linha horizontal
abaixo da tinta. Espessura, gap e descender extra vêm exclusivamente das
constantes MATH/font metrics pinadas; não há constante visual em pt. A largura
considera a correção itálica negativa conforme a morfologia vanilla, sem assar
coordenadas de fixture. A linha não altera o conteúdo do body e o resultado
preserva suas propriedades matemáticas relevantes.

O match exaustivo em `compiler/math/layout/_comum.md` somente delega. Toda a
geometria deste elemento vive aqui conforme ADR-0109. O caminho textual pode
reutilizar um helper privado puramente mecânico, mas não pode compartilhar
identidade nem tornar `entities/elements/underline.md` owner deste comportamento.
Não há `dyn`, vtable, registry nem importo reverso `entities→compiler`.

## Aceitação linguística

`math.underline(1 + 2)` conserva body, classe/estilo matemático e produz regra
inferior derivada da fonte; difere semanticamente do underline textual mesmo
quando um render particular parece igual.

## P1292 amendment-3 — largura da regra não redimensiona a caixa

### Medição anterior à decisão

Em 2026-09-01T00:57:33-03:00, com página auto/margem zero e a fonte matemática
pinada, `$underline(f)$` produziu bilateralmente caixa/root de `6.38pt` e
regra de `5.39pt`: o ajuste `−italics_correction` mede `0.99pt`, mas não
encolhe a caixa. `$underline(x)$` manteve caixa e regra em `6.292pt`.

Isso coincide com o vanilla ratificado em
`typst-layout/src/math/line.rs:20-67`, SHA-256
`d71623f47d8d3d9d9b5435820e5ca05399563642a4a48627f76ce0c8ed997b34`:
`frame.width = content.width` e somente
`line_width = content.width - content.italics_correction`. Se uma fonte
reportar correção realmente negativa, a regra pode extrapolar a caixa; essa
extrapolação continua sem aumentar `MathBox.width`.

### Decisão e refutadores

A free function preserva `body.width` como largura do `MathBox` e calcula
somente a regra como
`max(0, body.width - italics_correction(body))`. Correção positiva encurta a
regra; zero mantém; negativa alarga a regra para fora do frame. A linha, sua
espessura e sua extrapolação não alteram a largura lógica, não são clampadas à
root SVG e não participam da normalização vertical do frame de parágrafo em
`compiler/layout/equation.md`.

Refutam esta regra: root/frame acompanhar `line_width`, correção negativa ser
clampada ao body, a regra usar largura de tinta em vez de largura lógica, ou
qualquer ajuste horizontal mudar ascent/descent. Não há constante em pt nem
dependência nova; o owner continua exclusivamente este módulo.
