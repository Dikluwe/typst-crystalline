# P1241 — re-selar a fronteira SVG multi-space

**Estado:** EXECUTADO — CONSOLIDAÇÃO DIAGNÓSTICA ACEITE  
**Predecessor:** P1240  
**Saída:** certificado final e mapa de paridade atualizado.

## Objetivo

Reexecutar como consolidação diagnóstica o universo fechado de
Linear/Radial × sRGB, Oklab, Oklch, LinearRgb, Luma, Hsl, Hsv e CMYK, mais
controles Conic. O saneamento deve vincular contrato, oráculos e entradas aos
artefatos predecessores e ao L0 vigente, sem converter verificações lógicas em
mutation score ou selo Tekt.

## Adjudicação

Classificar cada par como `Preserved`, `Unknown` ou `Violated`; CMYK continua
`Unknown-ADR0097` salvo ADR explícita nova. Não exigir igualdade de bytes/stops.
Exigir grafo, geometria, alpha e amostra numérica. O raster P1240 é auxiliar e
não substitui orçamento específico do par.

## Gates finais executados

Foram executados o harness P1240 (7 testes), os testes SVG de `typst-infra`, os
testes P1239 de `typst-core`, `cargo fmt --all -- --check`, V15/V26 e
`git diff --check`. Todos passaram. O V5 registrou duas derivas preexistentes e
externas ao escopo, em `visualize.rs` e `tiling.rs`; elas não foram mascaradas
como sucesso. O mapa, a fila e a whitelist ficam no último estado comprovado.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO quando houver filesystem compartilhado.

## Resultado

P1240 foi saneado como harness auxiliar retangular, sem selo ou mutation score.
A consolidação conservadora classificou Linear/sRGB e Radial/sRGB como `Preserved`;
os outros 14 pares permanecem `Unknown` (incluindo CMYK como
`Unknown-ADR0097`). Os controles Conic não promovem a família inteira.

Um runner reproduzível vinculou P1237–P1240 e o L0 SVG por hash. Percursos em
ordem normal e inversa produziram artefatos byte-idênticos. Os 12 ataques são
somente políticas lógicas, não mutações executadas; não há mutation score nem
certificado global selado.

Mapa, fila e whitelist permaneceram byte a byte inalterados porque já exprimem
o limite conservador adjudicado; nenhuma promoção adicional foi autorizada.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO. Os gates atuais registram separadamente
os dois V5 externos já conhecidos; V15/V26 e os gates do fragmento passam.

Recibo diagnóstico em `00_nucleo/diagnosticos/p1241-tekt-certificado.tsv`.
