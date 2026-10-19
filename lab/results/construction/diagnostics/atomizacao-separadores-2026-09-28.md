# Separadores textuais: correção e atomização da primeira fatia funcional

O defeito reproduzido foi corrigido no percurso fonte → conteúdo → layout → PDF.
O layout conserva agora a ocorrência do espaço, além do avanço geométrico que
já aplicava. Os três casos RED (`A B C`, `A  B   C` e `#text("A B C")`)
passam a extrair palavras separadas; `A#("B")C` continua a extrair `ABC`.

## Medição e proveniência anteriores à conclusão

Working tree não commitado sobre HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`.
Os resultados numéricos abaixo pertencem exclusivamente às árvores, binários,
fontes e horários arquivados em
[proveniencia.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/atomizacao-separadores-2026-09-28/after/proveniencia.json)
e ao recibo final `atomizacao-separadores-2026-09-28/final-receipt.json`.
A referência permanece vanilla `9f2b6e8715237cb086899a42873660fe744622e8`,
árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`; seu binário tem SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

A evidência anterior permanece em `fatia-funcional-tekt-2026-09-28/`.
A posterior foi gravada em pasta distinta, `atomizacao-separadores-2026-09-28/after/`.
Os PDFs posteriores usam o binário release final; sua identidade foi conferida
contra o SHA-256 registrado pela medição. Fontes, comandos, hashes dos PDFs,
texto extraído, páginas, linhas e coordenadas constam nos JSONs.

| Verificação | Resultado observado | Evidência |
|---|---|---|
| Retenção/origem, adjacência, neutralidade e rollback | RED por ausência de carrier; depois cinco testes passaram | `test-separator-red.log`, `test-separator-green.log` |
| Suíte L1 final | 6.293 passaram, zero falharam, dois ignorados | `tests-core-all-final.log` |
| Sondas bilaterais fonte→PDF | 24/24 retiveram o texto esperado em cada compilador | `after/resultados.json` |
| Mesmos PDFs antes/depois | 14/14 tiveram pixels idênticos na primeira página a 144 dpi | `after/visual/comparison.json` |
| Compacto, UA1 e compacto+UA1 | 15/15 controles de extração passaram | `after/export-modes.json` |
| Direção textual final | 13 testes passaram | `tests-bidi-final.log` |
| Suíte L3 com um teste raster excluído | 1.111 passaram, dez falharam | `tests-infra-bounded.log` |
| Build de desenvolvimento e release | Concluídos | `build-final.log`, `build-release-final.log` |
| Linter completo final | Zero erros; 497 avisos | `lint-full-final.log` |

A sonda com página estreita preservou o texto e produziu seis páginas e 16 linhas
nos dois compiladores. Isso não prova igualdade de toda a geometria do corpus.
A comparação de pixels cobre os 14 controles anteriores, todos de uma página.
PDF/UA1 aqui verifica compilação e texto extraível, não certificação completa de
acessibilidade. O controle RTL usa texto latino com direção RTL; não fecha
paridade de todos os scripts bidirecionais. Não foi medido ganho de desempenho.

## O que o prompt permitia perder

O L0 histórico `8658aa…:00_nucleo/prompts/rules/layout.md` aceitava presença de
palavras e passagem pelo pipeline sem crash. O layout inicial reduziu Space a
avanço de cursor. Os L0 vigentes de Space/Text descreviam divisão e avanço;
o de exportação descrevia agrupamento e compensações geométricas. Nenhum deles
impunha a conservação da proveniência lexical através da fronteira layout→PDF.
As fontes históricas e o rastreio estão no diagnóstico
[primeiro alvo](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/primeiro-alvo-fatia-funcional-tekt-2026-09-28.md).

A conclusão é uma lacuna de contrato e aceitação entre unidades. O prompt não
mandava apagar espaços. Tampouco a história demonstra a motivação de quem
escolheu a representação. Inferir preferência pelo caminho mais fácil continua
sendo hipótese; o que foi medido é a perda da ocorrência, mascarada em alguns
casos pelo leitor PDF e pelo `plain_text()` que fabrica espaços entre fragmentos.

Com os mesmos prompts incompletos, uma ordem de construção diferente poderia
produzir outra representação compatível com aqueles testes. Isso é inferência,
não resultado de reconstruções comparadas. Especificar a ocorrência e o resultado
observável reduz essa liberdade e permite comparar implementações futuras pelo
mesmo comportamento, sem exigir o mesmo algoritmo ou buffer.

## Unidades materializadas

O usuário aprovou explicitamente o contrato público em 2026-09-28:
`FrameItem::TextSeparator { pos, advance, style, origin }` e
`TextSeparatorOrigin::{Markup, Newline, Literal}`. O recibo está em
`atomizacao-separadores-2026-09-28/gate-approval.json`.

Space é o produtor comum: acrescenta o carrier sem modificar o prefixo certificado,
aplica o avanço uma única vez e conserva a regra local de wrapping. Text delega
os espaços literais à mesma unidade, mantendo a ausência de flush imediato desse
percurso. O carrier não altera tinta, baseline, ascent/descent, altura ou largura
terminal. A justificação atualiza seu intervalo sem mudar a fórmula das posições
visíveis. Grupos, links, tradução, slicing, matemática externa, shaping e bidi
conservam a ocorrência. Recursos incluem U+0020 no subset e no ToUnicode; UA1
segue o tratamento textual de Span e a coordenação existente de MCIDs.

Foram individualizadas três responsabilidades, com owner L0 exclusivo:

- `compiler/layout/text/style.rs`: resolução pura do estilo efetivo, extraída de
  Text com a precedência existente; markup resolve uma vez e literal reutiliza
  o estilo já resolvido, evitando reaplicar a escala de scripts.
- `export/stream/text/runs.rs`: compatibilidade e emissão de agrupamentos de runs,
  extraídas do exportador de operadores textuais.
- `export/stream/separator.rs`: emissão de um U+0020 causal, em coordenadas top/local
  e com a fonte preparada, sem inferir separadores a partir de gaps.

O Núcleo Tekt `prompts/_nuclei/layout/text-separators.toml` compartilha invariantes
entre os owners por pins completos. O grafo e os selos seguem a cardinalidade 1:1.
O match continua estático e exaustivo. `plain_text()` conserva seu diagnóstico
aproximado legado e não é usado como oráculo lexical.

## Falhas que permanecem abertas

As dez falhas L3 foram reproduzidas na cópia isolada anterior à mudança:
cinco refutadores de Place e cinco snapshots P307b. Os logs são
`tests-place-baseline.log` e `tests-snapshots-baseline.log`; a receita de restauração
é `baseline-recipe.py`, com snapshots preservados em `before/` e hashes no recibo
final. Os snapshots não foram regenerados para disfarçar essas falhas.

O teste raster `p1431_raster_mapeia_butt_round_square_e_auto_square` foi interrompido
após execução prolongada e excluído da rodada limitada. Não há resultado GREEN
para ele. A primeira rodada concorrente também mostrou falhas adicionais que não
reapareceram na rodada limitada; não foram usadas para declarar regressão nova.

O linter completo não está sem violations: os avisos remanescentes incluem deriva
em arquivos fora do escopo e cláusulas de decisão já existentes. A validação de
V5/V6/V15/V26 não aponta os consumers alterados; todos os selos deste trabalho
foram revalidados, sem resselo global. A correção localizada está comprovada;
paridade global, os problemas de Place, os snapshots e o teste raster continuam
sendo frentes distintas.
