# Página automática com espaços literais — correção segregada

**Data:** 2026-09-29. **Estado:** correção implementada e medida; texto vazio, newline final e parágrafos só de espaços continuam abertos em frentes próprias.

## Medição antes da decisão

O oráculo é o vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. A árvore Tekt está em `17f13b1df79b87ea065ee62e78164b18e477da3d` com working tree não commitada; os estados completos, `git diff HEAD --stat`, fontes, comandos e hashes estão em [`before/provenance.json`](correcao-auto-espacos-2026-09-29/before/provenance.json) e [`after/provenance.json`](correcao-auto-espacos-2026-09-29/after/provenance.json). Os binários Tekt antes/depois têm SHA-256 `612f83167d0d4a40409fd66962ae48573b9b1c55bdbc9c3cb7131b1ea976da94` e `17d10ec3dbb6be616d6f7306ee1b78d730de95b97ff1863132681d172f4e2419`.

Nas 33 sondas fonte→PDF (11 formas × tracking `0`, `+0.2`, `−0.2` pt), as dimensões extraídas pelo Poppler coincidiam em 6 antes e 24 depois. As 18 novas coincidências cobrem `#text(" ")`, `#text("  ")`, `#text("   ")`, espaços isolados a 18 pt, espaços finais após `abc` e página mínima de um espaço de markup. Os 6 casos antes coincidentes permaneceram. Não houve regressão de dimensão nessa matriz. [`before/results.json`](correcao-auto-espacos-2026-09-29/before/results.json) e [`after/results.json`](correcao-auto-espacos-2026-09-29/after/results.json) preservam os valores caso a caso e os PDFs.

Exemplo causal: `#text("   ")` a 12 pt, margem zero, página auto e tracking `+0.2pt` dava PDF Tekt `0×0pt` (exibido pelo Poppler como fallback A4) contra `9.76×8.568pt` vanilla. Depois mede `9.76×8.568pt` em ambos. Para tracking zero, a largura é `9.36pt` em ambos; para `−0.2pt`, `8.96pt`. O mínimo físico PDF `3×3pt` está em `lab/typst-original/crates/typst-pdf/src/convert.rs:108-118`; antes, os escritores Tekt em `03_infra/src/export/builder.rs` e `builder/fonts.rs` podiam emitir MediaBox zero. A intenção histórica do vanilla não é inferida dessas saídas.

## Contrato e alteração

Os quatro owners e seus L0 estão no [`scope.json`](correcao-auto-espacos-2026-09-29/scope.json), com snapshots anteriores em `before-source/` e hashes em [`scoped-seals.json`](correcao-auto-espacos-2026-09-29/scoped-seals.json). Os L0 foram atualizados antes do código; o teste de página auto falhou em [`red.log`](correcao-auto-espacos-2026-09-29/red.log) e passou em [`green.log`](correcao-auto-espacos-2026-09-29/green.log).

- `compiler/layout/space.rs` conserva `TextSeparator` de origem `Literal` desde o primeiro espaço, inclusive com tracking zero. O carrier transporta avanço e estilo, sem desenhar tinta.
- `compiler/layout/page_geometry.rs` projeta somente esses carriers literais na largura da página auto; numa página só de carriers, preserva o topo tipográfico de cada estilo para a altura. A projeção de alinhamento RTL em `helpers::line_content_right` não muda.
- `export/builder.rs` centraliza o mínimo físico de 3 pt por eixo no MediaBox PDF depois do canvas com bleed. `export/builder/fonts.rs` reutiliza o helper nos caminhos CIDFont. A geometria lógica da página não recebe esse mínimo.

Esta é uma correção interna de paridade segundo ADR-0127, sem campo público, assinatura, flag ou fase nova. A aceitação é observável no nível da linguagem/PDF; não depende de igualdade mecânica de buffers ou bytes.

## Verificação e limites

`cargo build --offline` e `cargo build --offline --release -p typst-wiring` passaram. Core: 6375 testes unitários, 8 de integração e 3 doctests passaram; 2 testes existentes ignorados. Infraestrutura: 1137 unitários e todos os grupos de integração passaram em execução serial e na repetição paralela. A primeira execução paralela teve duas falhas intermitentes de determinismo de bytes em testes de gradiente sobre páginas `100×100pt`; ambos passaram isolados, na execução serial e na repetição paralela. A regra de mínimo de 3 pt não atua nessas páginas; os três logs ficaram arquivados.

`crystalline-lint . --format sarif` encerrou com zero erros, 504 warnings e 1559 notes em [`lint-final.sarif`](correcao-auto-espacos-2026-09-29/lint-final.sarif); o aviso V5 em `01_core/src/entities/page_geometry.rs` não pertence aos quatro owners alterados nesta frente.

Os controles em `uniform/` preservaram 36/36 origens de tinta contra o vanilla (erro máximo `0.00003pt`); `regressions/` preservou 401/401 entradas anteriores dentro de `0.001pt`; `auto-mixed/` preservou os 42 casos de escopo/mistura sem nova divergência; `auto-rtl/` manteve as 12 divergências preexistentes de hebraico e espaços literais à esquerda de texto RTL; `extra-after/` passou de 40/42 para 42/42 páginas com dimensões iguais ao vanilla, incluindo os dois casos só de espaços. Cada diretório contém sondas, resultados, PDF e proveniência executável.

As 9 divergências de dimensão que restam na matriz de 33 são três variantes de cada caso: `#text("")` (vanilla altura tipográfica sem caracteres; Tekt só mínimo físico), linha em branco terminal depois de tinta (duas quebras; altura Tekt excessiva) e dois parágrafos só de espaços (largura/altura diferem). Esses comportamentos têm causas e ownership distintos; não se deve absorvê-los em uma constante ou exceção dentro da projeção de espaços literais.
