# Relatório P1454 — preservação de paint em `html.frame`

## Veredito

**PASS — hardcode produtivo eliminado e paint preservado bilateralmente.**

Execução sem atestação de isolamento. Baseline da implementação:
`22bccb9136054ada5033114573391ecab5c2a11e`, árvore
`6d31c4a55d6235057b6f5b293a9452b92937b47a`, limpa na medição de
2026-09-18T21:36:57-03:00.

## Causa e correção

A paleta pública não era a causa: as 18 cores retornaram a mesma lista de
hexadecimais no vanilla ratificado e no cristalino. O defeito era uma
pós-transformação exclusiva de `export_svg_html_frame_with_fonts`:
`.replace("#ff4136", "#ff0000")` sobre a string SVG completa.

P1454 removeu essa transformação. Nenhum valor substituto foi introduzido: o
writer SVG comum agora serializa diretamente o `Color` transportado. A suíte
P1424, que legitimava a saída adulterada, foi corrigida L0-first para exigir o
valor público ratificado.

## Auditoria ampliada de hardcodes

Uma varredura de L1–L4 procurou `.replace`/`replace_all` produtivos associados
a cor, fill e stroke, além do par `#ff4136`/`#ff0000`. Após a correção, não há
reescrita produtiva de cor. A única substituição semelhante restante está em
um mutante de teste P1250 que adultera `stroke-linecap` deliberadamente para
provar poder discriminatório; não pertence ao produto.

Constantes de cor em tabelas canônicas, writers e fixtures não foram tratadas
como hardcode defeituoso: elas representam dados ou oráculos e não traduzem o
valor em trânsito.

## Evidência RED → GREEN e paridade

- RED: o teste focal mostrou dois `#ff0000` e nenhum `#ff4136` antes da
  remoção;
- GREEN: o teste owner P1454 passou, preservando simultaneamente `#ff4136` e
  `#ff0000`;
- P1424: 4/4 testes passaram;
- paleta: 18/18 cores coincidiram bilateralmente;
- compilação pública: cada produto emitiu, nesta ordem, `#ff4136` e
  `#ff0000` para os dois frames da fixture;
- estabilidade: forward, reverse e repeat conservaram o mesmo vetor; os
  SHA-256 foram estáveis por produto.

Os HTML completos não são byte-idênticos e não receberam crédito por bytes;
a aceitação usa apenas o paint público e a morfologia P1424.

## Ataques

O predicado congelado rejeitou 8/8 ataques: tradução nos dois sentidos,
remoção de cada paint, aceitação unilateral de cada produto, omissão do teste
owner e promoção de `Unknown`. Positivos 3/3 permaneceram `Preserved`; opacos
3/3 permaneceram `Unknown`. Score `1.0`.

## Gates

Passaram `cargo fmt --all -- --check`, teste focal P1454, suíte P1424,
`cargo build --release --bin typst`, `cargo build`, `crystalline-lint --quiet
.`, `git diff --check` e a campanha bilateral em três ordens.

## Limite

O veredito cobre a ausência de reescrita de paint na variante SVG embutida de
`html.frame`. Não afirma igualdade byte a byte de HTML/SVG nem ausência de
constantes legítimas de domínio no repositório.
