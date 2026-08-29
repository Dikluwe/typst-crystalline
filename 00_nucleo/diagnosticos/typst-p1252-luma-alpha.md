# P1252 — alpha Luma preservado e defeito upstream delimitado

**Veredito:** `Known-Upstream-Bug` para perda de alpha não-Luma → Luma;
delta histórico de luminância resolvido posteriormente pelo P1239 no fragmento
Linear/Radial P1236; SVG Luma permanece `Unknown`.

## Medição antes da decisão

No vanilla ratificado `a51e02804`, `red.transparentize(60%)` conserva alpha
40%, mas `luma(color)` e a normalização dos três constructors de gradient
devolvem Luma opaco. Uma cor criada diretamente por `luma(54.02%, 40%)`
mantém 40%. A fonte explica a assimetria: `ProcessColor::to_luma` devolve o
próprio valor para `Self::Luma`, mas usa `Luma::from_color` para as demais
variantes (`lab/typst-original/crates/typst-library/src/visualize/color.rs:1769-1781`);
`process_stops` converte cada stop por `to_space` antes de construir qualquer
variant (`gradient.rs:1362-1399`).

No cristalino, `Color::to_space(ColorSpace::Luma)` já preservava alpha, mas a
superfície `luma` não aceitava alpha nem conversão de cor. P1252 completou essa
superfície com `luma(lightness, alpha: alpha)` e `luma(color)`, e protegeu
Linear, Radial e Conic por teste E2E público. Na execução original, o valor
cristalino era 54.01%, distinto dos 54.02% vanilla; a escolha de alpha não
fechava nem perdoava esse gap. P1239 corrigiu posteriormente os coeficientes L1
para os valores de `palette 0.7.6`: o corpus P1236 passou a ter 42/42 pares sem
delta de luminância. Essa correção posterior não transforma a perda de alpha do
vanilla em comportamento normativo.

O PR upstream https://github.com/typst/typst/pull/3438 introduziu alpha em
Luma. O PR https://github.com/typst/typst/pull/4424 tratou outra correção de
Luma e não decide esta conversão. Busca limitada no tracker oficial por Luma,
alpha, conversion e `from_color` não encontrou issue específica. Ausência de
issue não é prova e permanece `Unknown` contratual.

## Decisão

- preservar alpha na conversão e na normalização dos três gradients;
- classificar a divergência de transparência como `Known-Upstream-Bug`;
- manter alpha e luminância separados; a luminância foi fechada posteriormente
  pelo P1239 no fragmento medido;
- não alterar `interpolate_luma`, constructors de gradient ou whitelist SVG;
- não abrir issue upstream neste passo.

Os cinco ataques foram rejeitados (`5/5`, score focal `1.0`). A execução
não recebe atestação de isolamento porque uma única autoridade operou no
filesystem compartilhado.

## Proveniência

- commit: `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`;
- hora: `2026-08-27T16:36:44-03:00`;
- estado: working tree não commitada; `git diff HEAD --stat` reportou 50
  ficheiros, 1870 inserções e 140 remoções;
- fonte da sonda: SHA-256
  `e27b95523a535a1d68a27aaa62bd9462ee401edeefa8a76b43178c0c2f8ec9bf`;
- texto vanilla: SHA-256
  `83ec0e82fda0faff795e079959caaa587bbb58889958e59832805449dbcab1fe`.
- texto cristalino após a implementação: SHA-256
  `5ec7c77bc03f9d3c0f1581ef4629b6c7eae06e15f228c9ec0de903d03a8ad9cb`;
  saída observada: source/direct/converted/Linear/Radial/Conic com alpha 40%,
  mantendo 54.01% nas conversões e 54.02% na construção direta.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.

## Revalidação aditiva — 2026-08-28

Os quatro testes P1252 vigentes continuam verdes. Os hashes L0 do manifesto
original divergiram legitimamente após P1239/P1250B; por isso aquele manifesto
permanece recibo histórico e não é reutilizado como selo atual. O certificado
`p1252-revalidation-certificate.tsv` congela as entradas atuais e registra a
reexecução dos gates.
