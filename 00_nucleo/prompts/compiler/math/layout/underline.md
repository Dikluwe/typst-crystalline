# Prompt L0 — `compiler/math/layout/underline` — underline matemático

**Estado:** RASCUNHO NORMATIVO NO GATE ADR-0127 — sem consumer e sem Hash do
Código até selo humano.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:42a441e59c53fdc5bf619005c0e018eb63251533fb6dc6c16c13d000e592e40c

**Camada:** L1
**Alvo planejado:** `01_core/src/compiler/math/layout/underline.rs`
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

## Decisão proposta

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

## Aceitação linguística

`math.underline(1 + 2)` conserva body, classe/estilo matemático e produz regra
inferior derivada da fonte; difere semanticamente do underline textual mesmo
quando um render particular parece igual.
