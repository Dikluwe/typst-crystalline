# Correção da composição de parágrafos — 2026-09-28

A frente básica está implementada e validada: linhas continuadas justificam por espaços, linhas finais mantêm largura natural e `par` conserva identidade e fronteiras próprias. A realização mantém a precedência lexical dos estilos de `set` e `show par`. A otimização global e a admissão completa de conteúdo em parágrafos continuam abertas.

## Proveniência e critério

HEAD: `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitada. A árvore já continha alterações antes deste lote. `correcao-composicao-paragrafos-2026-09-28/initial.json` registra o início; `before/` conserva os consumers e L0 anteriores; `final-receipt.json`, de `2026-09-28T18:34:27.355896+00:00`, contém os hashes das fontes, a lista exata de alterações, `git diff HEAD --stat`, status, logs, binários e recorte visual. O confronto dos hashes encontrou somente os 25 arquivos produtivos/L0 deste escopo; as alterações anteriores fora dele foram preservadas.

Oráculo: upstream pinado `9f2b6e8715237cb086899a42873660fe744622e8`, árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489. SHA-256 do binário vanilla: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. SHA-256 do release Tekt validado: `547f79bac3cb46fa627aa819735c2a6a9a39ce9ac5cc585c822db93a64f3423f`. Os runners conservam fontes, comandos, fontes tipográficas, stderr, PDFs, caixas e proveniência em cada lote. A string de versão não foi usada como prova de identidade.

Aceitação ADR-0107: identidade observável de `par`, palavras, quebras, coordenadas, dimensões e precedência dos estilos nas sondas selecionadas. O extrator pode dividir uma linha justificada em vários agrupamentos; a comparação agrupa palavras por faixa vertical e compara suas caixas individualmente, com tolerância de 0.0005pt. Em colunas, isso compara posições e palavras, sem tomar agrupamentos XML como contagem de linhas. Bytes de PDF não são requisito. O recorte rasterizado do controle `simple-140-True` coincide e serve apenas de inspeção visual auxiliar.

## Medição anterior e resultado

Resultados em `before-pdf/resultados.json`, `after-pdf/resultados.json`, `empty-details/results.json`, `empty-details-after/results.json` e suas proveniências.

| Controle | Tekt anterior | Tekt corrigido | Vanilla pinado |
|---|---:|---:|---:|
| Fim da primeira linha justificada, página 180pt | 145.520pt | 172.000pt | 172.000002pt |
| `BEFORE #par[MIDDLE] AFTER`, linhas físicas | 2 | 3 | 3 |
| Mesmo conteúdo, altura automática | 40.936pt | 70.504pt | 70.504pt |
| `BEFORE #par[]`, altura automática | 24.568pt | 38.968pt | 38.968pt |
| `repr(par[hello])` | `[hello]` | `par(body: [hello])` | `par(body: [hello])` |

Parágrafos vazios exigiram uma segunda medição: `par[]` e `par("")` não são equivalentes. O texto vazio reserva seu top-edge sem emitir tinta; sozinho, com margens 8pt, mede 24.568pt, enquanto `par[]` mede 16pt. Espaçamento inicial fraco é descartado no término inteiramente vazio, mas conserva-se entre fluxos com conteúdo. Os controles terminais com estilo e os vazios consecutivos foram medidos separadamente.

A regressão da precedência foi detectada pelo corpus anterior: `show par: set text(hyphenate: false)` perdeu prioridade sobre um `set text` exterior quando havia configuração de página. `show-par-full-tree.log` localiza a causa: o escopo exterior entrou no body de um Par artificial. O produtor agora conserva a origem de escopos `set` em `eval.flow.scope`, privado. O realizador mantém esses escopos exteriores ao primeiro parágrafo material. Wrappers que cobrem todo o material também fornecem o escopo inicial; fragments com material fora deles permanecem inline. Texto singleton integral precisa formar Par para receber a recipe. O lote `precedence-pdf` confirma tanto a desativação no escopo inteiro como o override local no fragmento inline e no `set` a meio do parágrafo. A diferença de extração `-`/U+AD permanece registrada no lote anterior.

## A falha vinha do prompt?

Na construção de `par`, sim: o L0 anterior autorizava explicitamente devolver o body diretamente e afirmava que não havia elemento próprio (`before/00_nucleo/prompts/compiler/stdlib/structural/par.md.txt:30-44`). O consumer obedecia a esse contrato; só modificar código deixaria a fonte arquitetural incorreta. O L0 foi corrigido antes da implementação.

Na justificação, a composição tinha sido adiada expressamente: o L0 anterior declarava-a incompleta. A operação de distribuir espaços já existia em `line_commit.rs`, mas as quebras automáticas em `word_layout.rs` chamavam fechamento ordinário. A correção conecta a continuação material à operação existente e conserva o fechamento natural nas fronteiras finais.

Isso demonstra uma simplificação documentada e uma ligação incompleta entre peças. Não demonstra a motivação pessoal de quem as escreveu. A ordem de construção é uma hipótese sobre a origem histórica; reconstruir por outra ordem não garante eliminar a falha se os mesmos contratos incompletos forem mantidos.

## Atomização e contrato vigente

`01_core/src/compiler/layout/paragraph.rs` é a unidade dona da composição de `Content::Par`, legitimada por seu L0 exclusivo. O match exaustivo do hub delega à free function da unidade, conforme ADR-0109. O constructor conserva o nó; `style_scope` fecha antes de instalar opções locais; `word_layout` pede fechamento continuado; `line_commit` distribui espaços. A realização contextual permanece no owner `eval/rules.rs`; o hub de eval apenas marca a origem de escopos no fold existente.

O núcleo `paragraph-composition.toml` compartilha invariantes e pins entre os owners; não legitima código diretamente. Os estados de fechamento terminal e espaçamento inicial fraco são privados e participam do checkpoint/rollback. O término usa a baseline causal, preservando o orçamento conservador de notas. Assinaturas públicas e fases do pipeline foram conservadas. Correções internas/de paridade seguiram ADR-0127 em fluxo contínuo, com L0 primeiro e resselo limitado ao escopo.

## Validação final

- `core-verified.log`: 6.329 passaram, 2 ignorados preexistentes.
- `infra-verified.log`: 1.126 passaram. Foram acrescentados 12 testes de núcleo e 3 de integração; testes antigos de identidade/estilos foram fortalecidos.
- `build-verified.log` e `release-verified.log`: builds offline debug e release concluídos.
- `lint.sarif`: zero violations; 502 warnings e 1.511 notas permanecem registrados, sem tratá-los como erros.
- `acceptance.json`: 170 verificações selecionadas passaram em 49 entradas bilaterais. Dos 34 controles principais, 30 coincidem nas palavras, caixas e dimensões verificadas; quatro são explicitamente abertos.
- `regression-acceptance.json`: 193 verificações anteriores passaram em 58 entradas bilaterais, incluindo hifenização, diagnósticos e paginação. Total: 107 entradas bilaterais e 363 verificações selecionadas, sem falha de aceitação.

RED→GREEN está conservado em `core-red.log`, `empty-expanded-red.log`, `styled-empty-red.log`, `precedence-red.log`, `whole-style-red.log`, `whole-style-first-failure.log` e nos logs GREEN/finais. Os erros intermediários e a comparação prematura de identidade durante runners concorrentes foram conservados; a aceitação final foi reexecutada após concluir ambos os lotes.

## Limites e próxima frente

1. **Preparação global e `simple`/`optimized`:** o controle `native-justify` ainda escolhe quebras diferentes quando o vanilla usa otimização automática. A justificação por espaços funciona; a escolha global permanece simples. Justificação CJK entre glifos e limites de justificação também estão abertos.
2. **Admissão de parágrafos em páginas:** `paragraph-pages` conserva as páginas e as palavras, mas a proteção de órfãs/viúvas difere. Precisa de decisão e prova próprias, mantendo os orçamentos de notas/colunas.
3. **Conteúdo aninhado:** o stderr vanilla e `lab/typst-original/crates/typst-layout/src/inline/collect.rs:247` mostram que Par dentro de Par é inválido, gera warning e é ignorado. A hipótese inicial de perda acidental foi refutada. O Tekt ainda renderiza B no controle aninhado; isso não é paridade aceita. Falta admissão e diagnóstico próprios.
4. **Realização de `show par` por função:** o controle com `repr(it)` permanece divergente e sem nova decisão de aceitação. Os controles corrigidos de show-set não fecham toda a realização funcional.

As propriedades nativas ainda declaradas incompletas, como `par(spacing:)` e indentações, não foram implicitamente implementadas por este lote. Tampouco há alegação de ganho de desempenho ou estado da arte já atingido.

A próxima frente é tornar explícita a preparação do parágrafo e comparar selecionadores de quebra sob o mesmo conteúdo e as mesmas invariantes. Isso permitirá avaliar ganhos distintos entre peças e sua composição, mantendo a separação entre preparação, seleção e materialização. A admissão inválida e seus diagnósticos devem ser auditados nesse contrato, em vez de permanecerem efeitos incidentais do render.
