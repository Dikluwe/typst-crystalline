# Prompt — typst-passo-855: completar a validação de `rect(radius:)` — geometria real do canto (resto do achado #62 / P852)

**Origem**: P852 implementou `radius:` em `rect()`/`square()`, mas o próprio relatório admite que só validou a aceitação do argumento e a construção do `ShapeKind::RoundedRect` — não a geometria real do canto no PDF exportado, que era parte explícita do Passo 3 do prompt original de P852.
**Estado**: aguardando execução — código já existe, isto é fechar a validação que ficou faltando, não reimplementar.

---

## O que já está feito (não refazer)

`native_rect`/`native_square` aceitam `radius:` (uniforme ou dicionário por canto), constroem `ShapeKind::RoundedRect` com `Corners<Length>`, e o exportador já suportava esse `ShapeKind` desde P242. Os testes de P852 confirmam a construção do elemento correto.

## O que falta

Confirmar que o canto **desenhado de fato no PDF** tem a curva certa — raio certo, nos cantos certos, com a forma de arco esperada — comparando com o vanilla, não só que o argumento foi aceito sem erro.

---

## Passo 1 — Sonda geométrica

1. Gerar PDF com `#rect(radius: 10pt, width: 100pt, height: 60pt)` nos dois binários. Extrair o path do retângulo do content stream (`mutool clean -d` ou `qpdf --qdf`, mesmo método já usado em achados anteriores de geometria) e comparar a estrutura: quantos segmentos de curva Bézier, em que cantos, com que controle de pontos — não precisa bater byte a byte, mas a forma geométrica (arco de 10pt de raio) precisa corresponder.
2. Testar o caso de raios diferentes por canto (`radius: (top-left: 15pt, top-right: 5pt, bottom-left: 5pt, bottom-right: 15pt)`) e confirmar que cada canto tem o raio correto, não só que os quatro existem.
3. Testar `radius: 0pt` — confirmar que continua produzindo um `ShapeKind::Rect` reto (sem custo de curva), como o relatório de P852 afirma, mas sem ter mostrado a saída.
4. Testar um raio maior que metade do lado menor (ex.: `radius: 40pt` num retângulo de `width: 60pt`) — confirmar se o vanilla faz clamp (limita o raio ao máximo possível) e se o cristalino faz o mesmo.

## Passo 2 — Corrigir se houver divergência

Se a sonda encontrar que a curva não bate (raio errado, canto errado, ou ausência de clamp onde o vanilla faz), corrigir o ponto exato no exportador ou na construção do `Corners<Length>`.

## Passo 3 — Validação final

Repetir os casos do Passo 1, confirmando visualmente (renderização a baixa resolução, `mutool draw`, comparação de imagem) e estruturalmente (path do PDF) que os cantos batem com o vanilla. Suíte completa, comando + contagem antes/depois — **usando o comando real `cargo test --workspace` e reportando os números de cada crate separadamente** (`typst-core`, `typst-infra`, `typst-shell`, os demais), não um número único sem dizer de onde vem.

## Relatório

`00_nucleo/diagnosticos/typst-passo-855-relatorio.md` com a comparação geométrica completa (Passo 1), qualquer correção feita (Passo 2), e a validação final com as contagens de teste discriminadas por crate.
