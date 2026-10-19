# Correção das falhas de Place, snapshots e raster — 2026-09-28

As falhas deixadas pela validação dos separadores foram corrigidas. Os cinco refutadores de Place passam; o corpus P307b volta a comparar bytes determinísticos; o raster completa sem exclusão. A atomização existente foi preservada: comportamento de imagem/forma fica nos seus owners, transporte no helper, origem isolada no sublayout e política de página no coordenador.

## Medição anterior à decisão

O estado inicial é a working tree não commitada sobre `17f13b1df79b87ea065ee62e78164b18e477da3d`. `correcao-falhas-2026-09-28/initial.json` guarda UTC, lista exata de arquivos alterados e `git diff HEAD --stat`; `before/` conserva os consumers e L0 afetados, assim como as referências PDF anteriores. Os resultados iniciais de Place estão em `place-red.log`; a reprodução anterior aos separadores está documentada no diagnóstico de atomização. Não se atribui aos separadores a origem dessas falhas.

| Observação | Causa medida | Correção |
| --- | --- | --- |
| Página vazia ignorava 160×180pt | `layout/mod.rs:1978` omitira o primeiro Page sem material; o exportador supria outra página | Comitar a página inicial com PageConfig; conservar supressão da cauda vazia entre page runs |
| Cover perdia o clip em Place | `helpers.rs:185` reconstruía Image com clip_rect=None | Preservar o clip e transportar x/y junto com a imagem, mantendo a extensão |
| Outras translações deixavam o clip para trás | `clip-red.log` refuta offset e shifts do mesmo owner | Corrigir os três transportes; a contraprova passa na suíte completa |
| Place de texto cancelava o top edge | `placement.rs:327` subtraía o ascender do sublayout | Sublayout privado com origem geométrica e baseline deferida ao texto |
| Place de forma/imagem adquiria catálogo de fontes | `sub_frame.rs:143` e `placement.rs:327` pediam ascenders incondicionais | O caminho geométrico não pede métricas para produzir uma origem não textual |
| Raster não concluía na execução anterior | `p1431_line_cap_tests.rs:204` rasterizava A4 a 16 pixels por ponto para uma única linha | Janela limitada aos endpoints com margem para caps e antialiasing; mesma resolução e mesmas verificações |
| Cinco snapshots divergiam | PDFs antigos não continham os novos separadores emitidos nem o seu glifo/mapeamento no subset CIDFont | Regenerar somente 01/02/03/07/09 sob controles fixos e verificar novamente sem UPDATE |

Os caminhos/linhas da tabela referem-se ao código arquivado antes da correção. A sonda do vanilla pinado para `#set page(width:160pt,height:180pt,margin:8pt)` produziu uma página de 160×180pt, registrada em `probes/empty-vanilla.pdf` e `vanilla-observations.json`.

## O que o L0 precisava corrigir

O L0 de helpers explicitava como limite a reconstrução de imagem que descartava clip_rect. Preservar esse texto legitimava a continuidade da falha; ele foi substituído antes da materialização por uma obrigação de conservar e transportar o clip. Isso demonstra um problema no contrato vigente, mas não prova que o prompt tenha originado o defeito: a especificação anterior também registrava comportamento já existente.

Os L0 de sublayout e placement preservavam uma origem baseada em ascender para todos os corpos. A correção distingue a necessidade geométrica da necessidade tipográfica. O caminho novo é privado e restrito a Place na raiz, sem célula e sem linha material pendente. Sublayouts aninhados, células, floats e Place após texto mantêm seus caminhos anteriores. Nenhum tipo público foi acrescentado nesta correção.

A suíte completa refutou a aplicação do caminho novo a uma linha já pendente. A produção foi restringida e o controle p1292 permaneceu intacto. Duas expectativas históricas foram corrigidas com L0 atualizado primeiro: documento vazio conserva uma página; o controle p898 de alinhamento inferior soma o top edge para observar a baseline, mantendo dy e a tolerância original. Nenhum teste foi excluído para fechar estas falhas.

## Snapshots controlados

Os cinco PDFs alterados passaram `qpdf --check`, conservaram exatamente o texto extraído por `pdftotext -layout` e os pixels da primeira página a 144 dpi. A inspeção QDF identifica emissão de espaços e o respectivo mapeamento/subset; os quatro controles restantes conservaram seus bytes. Uma execução posterior sem `UPDATE_P307B_SNAPSHOTS` aprovou os nove casos, verificando a reprodução dos bytes no mesmo ambiente. O MANIFEST operacional foi atualizado.

Esses snapshots medem regressão cristalina controlada, não igualdade binária com o vanilla. As comparações, hashes e PDFs anteriores estão em `snapshot-comparison.json`, `snapshot-raster/` e `before/reference/`.

## Validação final e limites

- `cargo test --offline -p typst-core --lib`: **6.294 passaram, zero falhas, dois ignorados preexistentes** (`core-final.log`).
- `cargo test --offline -p typst-infra --lib -- --test-threads=1`: **1.122 passaram, zero falhas, nenhum ignorado**; inclui Place, nove snapshots e raster (`infra-final.log`).
- `cargo build --offline` e build release de typst-wiring: passaram.
- `crystalline-lint --format sarif .`: **zero erros**, **497 avisos preexistentes**; zero V5/V6/V15/V26 nos owners deste escopo (`lint-summary.json`). Não significa zero violations globais.
- Os **24 controles fonte→PDF**, com fonte versionada, passaram tanto no Tekt final quanto no vanilla pinado (`after/resultados.json`). Esse conjunto verifica o texto esperado; não demonstra paridade universal de layout.

Vanilla: pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. A receita do build vanilla permanece no diagnóstico P1489. Os binários, fontes, comandos, estado completo da working tree e hashes dos arquivos usados estão em `after/proveniencia.json` e `final-receipt.json`. `scoped-change.diff` distingue esta correção das alterações anteriores já presentes.

A correção aproxima os objetivos do Tekt ao tornar explícitas responsabilidades e refutadores de cada unidade. Não é uma demonstração de estado da arte nem um ganho de desempenho geral; demanda tipográfica dos demais sublayouts continua fora deste reparo.
