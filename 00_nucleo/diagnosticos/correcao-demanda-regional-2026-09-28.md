# Correção de paginação e continuação — 2026-09-28

A primeira rodada corrige a criação de páginas por uma baseline futura, a continuação que reaproveitava coordenadas de uma região fechada e a perda de texto entre colunas. Não fecha toda a auditoria da primeira implementação funcional. Os resultados abaixo foram medidos com fontes Noto Sans e com o vanilla pinado, antes da classificação.

## Medição

Os controles completos, fontes, PDFs, caixas de texto e comandos estão em [before-pdf](correcao-demanda-regional-2026-09-28/before-pdf/resultados.json) e [after-pdf](correcao-demanda-regional-2026-09-28/after-pdf/resultados.json). A tabela conta páginas, não igualdade de bytes de PDF.

| Caso | Tekt antes | Tekt corrigido | Vanilla | Resultado delimitado |
|---|---:|---:|---:|---|
| Texto terminal | 2 | 1 | 1 | Texto completo, sem página sucessora vazia |
| Continuação | 3 | 2 | 2 | Texto completo, caixas dentro da página |
| Quebra explícita terminal | 3 | 2 | 2 | Quebra explícita preservada |
| Quebra explícita com conteúdo | 3 | 2 | 2 | Texto completo na região correta |
| Duas quebras explícitas | 4 | 3 | 3 | Ambas preservadas |
| Parágrafo terminal | 2 | 1 | 1 | Sem sucessora artificial |
| Duas colunas | 2 | 1 | 1 | Recupera `Three`; hifenização ainda diverge |
| Quebra de linha terminal | 2 | 1 | 1 | Texto completo |
| Nota terminal | 2 | 2 | 2 | Preserva separação corpo/nota; distribuição ainda diverge |
| Flutuante inferior | 2 | 2 | 2 | Distribuição anterior preservada; ainda diverge |

A repetição dos controles anteriores também passou `paragraphs` de 2 para 1 página e `mixed-size` de 5 para 4, iguais ao vanilla em páginas e texto. As variantes sem newline final e três controles de isolamento tiveram a mesma correção. Os demais observáveis desse corpus permaneceram estáveis. Esses resultados estão em [acceptance.json](correcao-demanda-regional-2026-09-28/acceptance.json).

## Causa e correção

`line_commit.rs:654` passa a certificar que a tinta real da linha cabe no limite efetivo. Uma previsão da baseline seguinte deixa de obrigar uma transição. `region_progress.rs:129`, chamado por `cursor.rs:135` e pelos caminhos de quebra em `word_layout.rs:40/67`, admite a linha seguinte antes de emitir seu conteúdo. Não consulta conteúdo futuro nem cria estado persistente.

`region_progress.rs:27/141` limpa os quatro campos de espaçamento/encadeamento também ao mudar de coluna. Assim, a próxima palavra não reconstrói sua posição a partir do parágrafo da coluna fechada. O reset existente da mudança física de página usa a mesma função privada.

A primeira implementação introduziu sobreposição entre a nota terminal e o texto: o cursor da última baseline tinha substituído o orçamento conservador usado para encaixar notas. A medição foi preservada em `first-after-pdf/`. `mod.rs:1939/1973` conserva esse orçamento localmente, sem alterar a geometria terminal; `mod.rs:1995` drena a nota ao abrir a região necessária, evitando uma sucessora vazia. Os testes de integração capturaram tanto a sobreposição quanto a página extra antes da versão final. O PDF final da nota voltou ao comportamento anterior deste controle; não é prova de paridade da distribuição de notas.

Os L0 da atomização preservavam explicitamente a limitação terminal. Isso documentava o escopo daquela separação, não uma intenção desejável do produto. Os seis L0 proprietários foram atualizados antes do código para autorizar a correção medida. A causa demonstrada está no predicado de progresso e no reaproveitamento de estado; esta rodada não demonstra que os prompts originais causaram esses defeitos.

A composição estática e os owners permanecem: seis consumers e seis prompts, em relação 1:1. Nenhum contrato público, flag, fase de pipeline ou campo de checkpoint foi adicionado. O resselo ficou restrito a esses pares.

## Validação

- RED inicial: cinco regressões falharam e dois controles passaram antes da implementação.
- RED de integração: dois testes adicionais falharam, cobrindo coluna e nota.
- GREEN final: 6.308 testes de domínio passaram, zero falhas, dois ignorados; 1.122 de infraestrutura passaram, zero falhas ou ignorados.
- `cargo build --offline` e build release concluídos.
- `crystalline-lint`: zero violações; 497 avisos permanecem no estado da árvore.
- 99 execuções bilaterais de entradas, com 336 verificações delimitadas passando. Não são 99 casos de paridade plena. Fontes e observáveis do oráculo permaneceram iguais às medições de referência.
- Inspeção visual dos PDFs de colunas e das duas páginas da nota confirma o texto recuperado e a ausência de sobreposição corpo/nota nesses controles.

Os logs RED e das versões intermediárias permanecem arquivados. Ajustes de compilação e das novas fixtures também estão registrados: a fixture de coluna explicita fonte de 12pt e verifica texto mais posições, pois `plain_text` introduz espaços entre items. Nenhuma asserção ou tolerância de teste preexistente foi enfraquecida.

## Limites e próxima frente

Continuam abertos: propagação de `hyphenate`, `justify` e `linebreaks`; validação de argumentos; fronteira do `par` nativo; origem de palavras largas; medição por clusters e tracking; escolha e implementação do otimizador. O controle de colunas ainda divide `final` onde o vanilla não divide. Notas e flutuantes ainda distribuem conteúdo de modo diferente. A próxima frente é conservar e validar a configuração até as unidades que escolhem as quebras, antes de comparar algoritmos intercambiáveis.

## Proveniência

Estado: **working tree não commitado**, HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`. A árvore já continha alterações anteriores; foram preservadas. O inventário inicial e seus hashes são [initial.json](correcao-demanda-regional-2026-09-28/initial.json), medido em `2026-09-28T16:16:50.744931+00:00`. A lista exata de arquivos alterados, `git diff HEAD --stat`, UTC e hashes finais estão em [final-validation-state.json](correcao-demanda-regional-2026-09-28/final-validation-state.json). [final-receipt.json](correcao-demanda-regional-2026-09-28/final-receipt.json) verifica que as únicas alterações de código/L0 desde o inventário inicial são os seis pares de [scope.json](correcao-demanda-regional-2026-09-28/scope.json).

As execuções bilaterais finais ocorreram entre `2026-09-28T16:35:58.929862+00:00` e `2026-09-28T16:36:43.406786+00:00`, registradas por lote em [probe-runs.json](correcao-demanda-regional-2026-09-28/probe-runs.json). Cada lote registra comandos, fontes e identidade dos binários.

Vanilla: pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore ratificada `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita histórica P1489; binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Tekt corrigido: `target/release/typst`, SHA-256 `b236a1079db991defdf2a21a65a319d3de82de6e41f4a59eb189311a272bbc28`. As strings de versão não foram usadas como identidade dos builds.
