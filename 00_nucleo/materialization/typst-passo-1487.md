# Passo 1487 — corrigir oráculo obsoleto e recalibrar a matriz de paridade

**Estado:** EXECUTADO em 2026-09-24, sem alterar comportamento produtivo.
O resultado mede P1480–P1486 sem crédito indevido; ver
`00_nucleo/diagnosticos/typst-p1487-recalibracao.md`.

**Baseline:** commit limpo `90aad760ba472cffb70063baa684c461a240e447`
em 2026-09-22; cristalino SHA-256
`b70537dbcfa4184d900892623c440c01b92ae5bfee32b752ea9f7921eaf64fba`;
vanilla pinado SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.

## 1. Medição anterior à decisão

`cargo test -p typst-wiring --test p1464_format_pdf
p1464_svg_png_global_pdf_html_and_homonymous_closure_controls` falha nos
quatro perfis: `04_wiring/tests/p1464_format_pdf.rs:372-376` ainda espera
`["module","<module pdf>",...]`, mas o produto devolve
`["function","pdf",...]`. P1471 mudou deliberadamente o contrato público
para função, confirmado pela fonte vanilla e pelo L0 atual
`compiler/eval.md` §P1471. Isso é um oráculo desatualizado, **não** uma
regressão de paridade.

Na matriz curta atual (`lab/parity/matrix/runner.py`, fixture
`lab/parity/matrix/fixtures/plain.typ`) o default teve 15 MATCH, 2
DIFFERENCE e 3 DISABLED entre 20 casos. Um DIFFERENCE é PNG com 11 de
2.005.644 pixels divergentes, delta máximo 1; outro é nome de fonte PDF
`LibertinusSerif-Regular` versus `LibertinusSerif-Regular-Identity-H`,
com páginas, tamanho e texto iguais. Nenhum dos dois é automaticamente
falha semântica (ADR-0107); o classificador atual conserva a diferença
observada. O inventário default registrou 1.552 MATCH, um membro ausente
e 468 `UNVERIFIED_METADATA`. Essa última classe **não** é 468 defeitos.

Classificação: o tipo/repr de `pdf` são morfologia pública; nome interno
de fonte e diferenças subvisuais podem ser mecânica. Inferência: o teste
P1464 precisa só da expectativa atualizada; refutação: a sonda vanilla
ratificada ou os demais asserts da mesma prova ainda divergirem.

## 2. Contrato da sanitização

1. Repetir a sonda vanilla e atualizar primeiro
   `00_nucleo/prompts/wiring/tests/p1464-format-pdf.md`, depois o teste
   proprietário, sem alterar `pdf` produtivo para satisfazer expectativa
   antiga. Cobrir kind, `repr`, membros e perfis gated.
2. Tornar a matriz explícita sobre `DIFFERENCE` observável versus
   `VIOLATED` semântico. Preservar as métricas PNG/PDF e o artefato de
   diferença; não converter diferença em MATCH por tolerância arbitrária.
   Qualquer tolerância deve vir de sonda visual/reprodutibilidade e decisão
   documentada.
3. Regenerar os catálogos nos perfis default/html/a11y-extras/
   html+a11y; após P1484/P1485 incluir bundle ativo e desativado.
   `UNVERIFIED_METADATA` deve continuar separado de ausência real.
   Acrescentar probes funcionais de `color.spot.tint`, `asset`, PDF/UA e
   rótulos de página, com controles negativos. Um binding que apenas
   existe, mas não executa, não é MATCH.

## 3. Gates

Testes primeiro: RED na expectativa antiga, GREEN só quando vanilla e
cristalino atuais coincidem. Atualização de L0 do owner de teste precede
o arquivo Rust e o resselo do hash. `git diff --check`, V5/V15/V26,
`cargo build`, teste focal e suíte de integração afetada devem passar.

Relatório de medição precisa incluir commit/tree ou diff exato, hora se
vários builds, hashes dos binários e comandos. Não usar o número de MATCH
da matriz curta como percentual de paridade total; ela cobre somente o
manifest corrente.
