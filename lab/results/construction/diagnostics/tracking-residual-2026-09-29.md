# Tracking residual — auditoria causal, 2026-09-29

A perda de tracking entre fragmentos e o arredondamento dentro dos fragmentos explicam o desvio restante do controle latino. Não é evidência de um defeito do vanilla nem prova de paridade global. Esta rodada mede e identifica os contratos que precisam mudar; não altera código produtivo ou L0.

## Proveniência

Todas as contagens e posições desta rodada vêm de `tracking-residual-2026-09-29/results.json` e `summary.json`, gerados por `runner.py.txt`. `initial.json` registra UTC, HEAD, working tree não commitada, lista completa de arquivos alterados (`status` e `git diff HEAD --stat`), hashes de todas as fontes/L0, binários e pin vanilla `9f2b6e8715237cb086899a42873660fe744622e8`. `completed.json` comprova que as fontes não mudaram durante a medição.

Tekt SHA-256: `b9067a8a70e74f73876b65f583c0a422950e2c863b62ff9b6f5a25868c6d40e2`.
Vanilla SHA-256: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

36 entradas novas, 108 compilações/PDFs: palavra isolada, espaço markup, um/dois/três espaços literais e dois fragmentos adjacentes; tracking 0, ±0.2, 0.333 e ±1.2pt. Página larga, Noto Sans da fixture, 12pt, hifenização desligada. PDFs, comandos, texto extraído e traces MuPDF ficam arquivados. Os modelos independentes de quantização e fronteiras passam nos 36 controles com residual inferior a 0.00005pt, tolerância referente à precisão dos traces, não a uma certificação visual global. Os glifos visíveis de compacto/verboso coincidem dentro de 0.0001pt; carriers vazios não estão incluídos nessa afirmação.

## Medição antes da decisão

Fonte local `03_infra/src/shaper/shaping.rs:225-232` arredonda tracking para i32 em unidades da fonte; `:319-326` o aplica somente antes do próximo cluster diferente, nunca após o último glyph do sub-run. Fonte vanilla `lab/typst-original/crates/typst-layout/src/inline/shaping.rs:1269-1294` conserva tracking em Em e o aplica entre ranges distintos, inclusive espaços.

`01_core/src/compiler/layout/text.rs:49-83` divide o texto literal em palavras, encaminhando cada espaço ao owner space. `space.rs:15-33` avança somente a largura nominal do espaço. `text_measure.rs:41-42` mede essa largura por advance(" "). `reconciliation.rs:131-144` compara largura nominal estimada com a soma dos avanços inteiros do shaper; o separador retorna estimated == real. Assim, os pares que cruzam fragmentos não pertencem a nenhum caminho atual.

| Controle com tracking 0.2pt | Parcela de quantização da primeira palavra | Parcela perdida na fronteira |
|---|---:|---:|
| palavra isolada | +0.052pt | nenhuma |
| dois fragmentos adjacentes | +0.052pt | −0.2pt |
| um espaço markup/literal | +0.052pt | −0.4pt |
| dois espaços literais | +0.052pt | −0.6pt |
| três espaços literais | +0.052pt | −0.8pt |

A primeira palavra tem 13 pares internos: 0.2pt vira 17 unidades da fonte, equivalentes a 0.204pt, portanto 13 × 0.004 = 0.052pt. No início de `end`, após um espaço, o saldo é +0.052 − 0.4 = −0.348pt. Traces serializados acrescentam erro de poucos centésimos de milésimo; valores brutos não foram arredondados nos JSONs.

O controle ±1.2pt elimina a quantização nessa fonte/tamanho: a palavra isolada coincide, mas os fragmentos adjacentes continuam perdendo um intervalo e os espaços continuam perdendo n+1 intervalos. Os controles negativos invertem o sinal das duas parcelas. Isso discrimina uma causa de fronteira de uma causa de arredondamento e refuta a atribuição desse desvio visível apenas às coordenadas dos carriers de espaço.

## O prompt participa do problema?

Sim, no sentido de legitimar e manter o limite atual; não é prova de intenção histórica. `00_nucleo/prompts/infra/shaper/shaping.md:40` exige preservar os caminhos distintos de medição/emissão e o tracking i32 arredondado, restrito ao sub-run. `00_nucleo/prompts/compiler/layout/text_measure.md` declara explicitamente a discrepância codepoints/clusters adiada. `00_nucleo/prompts/compiler/layout/text.md` conserva a divisão em palavras e o avanço existente dos espaços. `00_nucleo/prompts/infra/shaper/reconciliation.md` exige deslocar carriers sem reaplicar seu avanço.

Não há deriva simples código/L0 neste ponto: há contratos de alcance limitado que agora têm prova suficiente para revisão. A hipótese de que esses adiamentos contribuíram para a falha é sustentada pelos caminhos medidos; atribuir preguiça ou motivação pessoal continua injustificado.

## Decisão e separação por comportamento

A correção deve tratar primeiro os intervalos que atravessam fragmentos e depois a precisão interna dos avanços, mantendo testes separados para cada comportamento. A regra é contar fronteiras reais de clusters no contexto da linha, não acrescentar uma constante a cada token de espaço. Com n espaços entre duas palavras são n+1 fronteiras; entre fragmentos adjacentes há uma. Isso não autoriza generalizar a regra latina para RTL, ligaduras entre runs ou estilos diferentes sem controles.

Medição/seleção precisa considerar a mesma extensão que a emissão consome; corrigir somente o PDF deslocaria tinta sem corrigir a decisão de quebra. A seleção global hoje recusa tracking (`paragraph_prepare.rs:90-94`), portanto ampliar esse caminho é uma frente distinta, não uma consequência automática da correção de fronteiras no fallback.

A atomização possível é por responsabilidades: resolver pares elegíveis e sua extensão; aplicar essa extensão na medição; transportar posições/avanços sem nova soma; serializar a geometria no PDF. O wire pode compor implementações equivalentes apenas depois de testar o mesmo contrato. Isso é um mapa de responsabilidades para a próxima materialização, não um novo owner ou contrato público aprovado.

Antes de materializar: controles de tracking com início/fim de linha, wrap, espaços repetidos, estilos/fontes distintos, ligaduras, SHY, local boxes e adjacência; verificar se o contrato existente comporta uma solução interna. Atualizar L0 primeiro, RED→GREEN, resselo restrito e regressões. A ADR-0127 permite fluxo contínuo para correção interna de paridade; eventual alteração pública exige o gate correspondente.

## Limites

Não foram alterados consumidores, fórmulas, snapshots ou hashes L0. Não há alegação de correção nesta rodada. O desvio dos carriers vazios, anteriormente medido em até 0.048pt no compacto, permanece independente e aberto. Esta auditoria não mede custo/performance, idiomas contextuais ou superioridade sobre vanilla. O caso de extração truncada no vanilla e a limitação y_advance são outras frentes e permanecem sem causa/impacto isolados.
