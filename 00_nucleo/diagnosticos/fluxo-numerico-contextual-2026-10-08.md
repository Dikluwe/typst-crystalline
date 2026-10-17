# Fluxo numérico contíguo: `context100`, `context300`, `plain300`

## Medição anterior à decisão

O diagnóstico `atual-vanilla-memoria5-2026-10-08.md` registrou uma linha visível no vanilla e várias no Tekt para os três fixtures. Nesta rodada, a sonda `#range(300).map(str).join()` ficou em uma linha nos dois motores; `#for i in range(300) { [#i] }` quebrou somente no Tekt. O vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8` coleta texto adjacente em `lab/typst-original/crates/typst-layout/src/inline/collect.rs:275-298`. No Tekt, `01_core/src/compiler/layout/paragraph_prepare.rs:130-157` já agrega os fragmentos, mas `paragraph_plan.rs:81-88,119-123` devolvia a seleção de uma linha ao fallback, que decide quebras por fragmento em `word_layout.rs:27-47`. No caso contextual, o conteúdo realizado continha `sequence([context(ContextBlockElem), text(...)])`; a preparação recusava o marcador, embora `context_block.rs:15-36` o trate como no-op no layout paginado ordinário. Essas observações não inferem intenção do vanilla.

## Decisão e correção

Correção interna de paridade pelo ADR-0127, sem contrato público ou fase nova. O preparo global admite `ContextBlock` como neutro apenas fora da captura de marcadores HTML. Uma palavra certificada, contígua e mais larga que a região permanece na seleção preparada mesmo quando tem uma só linha. O separador de markup terminal conserva sua emissão e não cria ponto de quebra dentro da palavra. O restante do fallback continua sujeito às guardas existentes. L0 atualizado antes do código em `prompts/compiler/layout/paragraph_prepare.md` e `paragraph_plan.md`, com hashes ressellados.

## Verificação

Os dois testes em `contiguous_overlong_tests` falharam antes da correção e passaram depois. O build de debug e o de release passaram. A compilação bilateral final usou os dois binários e a mesma fonte com `--pdf-tagged=false`, `--pretty=false`, `--font-path` comum, `RAYON_NUM_THREADS=1`, `SOURCE_DATE_EPOCH=0`, `TZ=UTC` e `LC_ALL=C`; comandos, ambiente, hashes de fontes/fixtures/binários, UTC e resultados individuais estão em `fluxo-numerico-contextual-2026-10-08/results.json`. Os PDFs finais estão no subdiretório `pdf/`.

| Fixture | Linhas visíveis vanilla / Tekt | Páginas vanilla / Tekt | Texto visível igual |
|---|---:|---:|---|
| `context100` | 1 / 1 | 1 / 1 | sim |
| `context300` | 1 / 1 | 1 / 1 | sim |
| `plain300` | 1 / 1 | 1 / 1 | sim |

O texto extraído visível tem 103 caracteres em cada PDF; o resto da palavra ultrapassa a área visível. A igualdade de extração não demonstra identidade de raster nem resolve as diferenças geométricas de outros casos. `cargo build` e `crystalline-lint .` terminaram com código zero; o linter ainda lista avisos preexistentes de V5 no working tree amplo, mas nenhum V5 para os dois consumers alterados. `git diff --check` passou nos quatro ficheiros produtivos/L0 desta correção.

Proveniência da medição final: HEAD e lista do working tree não commitado em `fluxo-numerico-contextual-2026-10-08/head.txt`, `working-tree-status.txt` e `working-tree-diff-stat.txt`. Esses ficheiros foram capturados junto do resultado final; este relatório foi escrito em seguida. A comparação de custo contextual deve ser repetida com estes fluxos equivalentes, pois os números anteriores mediam saídas diferentes.
