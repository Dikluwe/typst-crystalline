# Iteração e parâmetros internos: correções de CI

Removidos 11 diagnósticos distintos de Clippy em seis consumers, sem
supressões adicionais, novos contratos públicos ou alterações de default.
Os L0 foram atualizados antes do código e resselados apenas para os
consumers explicitamente manifestados.

## Medição anterior à decisão

Branch Tekt, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. `iteracao-parametros-ci-2026-10-13/before.json.gz` contém
fontes, owners, referências, inventário selecionado, UTC, estado Git,
diffstat, rustc e hash do binário. Os spans dos 11 avisos foram conferidos
contra as linhas atuais antes de alterar código. O inventário de entrada
vem da rodada `representacao-medidas-ci-2026-10-13` (48 diagnósticos).

A inspeção estabeleceu que os dois fixups de alinhamento não redimensionam
items, que metrics só era repassado na recursão de strip_context_marker,
e que o helper de componentes de cor é privado e consome self. Os loops
selecionados não dependem da propriedade dos índices; dois deles precisam
das coordenadas absolutas para os diagnósticos de conflito.

## Mudanças e invariantes preservadas

| Consumer | Mudança | Obrigação preservada |
| --- | --- | --- |
| grid/cell_emission | Percorre referências mutáveis de grid_owner | Mesmo recorte por linhas/colunas, origens fora do limite vazias, mesma aritmética e precedência de ocupação |
| grid_placement | Enumera slices validados e preenche a área com true | Ordem linha→coluna, primeiro conflito, coordenadas, mensagem/hint/span; nenhuma marcação antes de concluir a verificação |
| placement | Fixups pub(super) recebem &mut [FrameItem] | Mesma fila, paths, ordem e deltas; buffer não é redimensionado |
| html_frame | Remove metrics e generic privado da limpeza recursiva | Mesmos wrappers, campos, filhos e ordem; captura de posições ocorre antes da remoção |
| paragraph_select_forward | Enumera slots a partir do índice um | Captura active antes do while; slot zero, fronteira, desempate e fallback intactos; continua cfg(test) |
| color/mixing | Helper privado passa a se chamar into_vec4_in_space | Consumo de self, clones, conversão, fallback e pesos idênticos |

O mapa de bordas já recortava a área ao tamanho alocado; o novo iterador
conserva esse recorte, inclusive origens que deixam o percurso vazio.
No motor de ocupação, os spans são normalizados e os limites verificados
antes de ensure_rows. Os slices usam esses limites validados e não
silenciam áreas inválidas. Verificação de conflitos e marcação continuam
em fases separadas; GridExplicitFirst, TableDocumentOrder, cursor e
padding não mudam.

A remoção do parâmetro de métricas afeta somente o helper privado que
limpa marcadores. Os entrypoints de layout continuam recebendo métricas;
a aquisição, medição e transformação não foram movidas. Wrappers vazios
e TextSeparator permanecem regidos pelos contratos existentes.

## Inventário do workspace inteiro

Após o lote, foi executado Clippy em workspace/all-targets com
`--cap-lints warn` exclusivamente para inventário: isso permite coletar
avisos dos crates posteriores mesmo com problemas no core. A configuração
do repositório e o workflow não mudaram. O gate real continua sendo
`cargo clippy --locked --workspace --all-targets -- -D warnings`.
Exit zero desta coleta não significa CI verde.

A rodada terminou em `2026-10-14T00:40:32.055795+00:00` e registrou 406 diagnósticos
distintos com spans primários locais, deduplicados por código/mensagem/spans.
Inclui avisos Rust e Clippy, com hashes das fontes antes/depois, commit,
working tree, UTC e comando em `workspace-clippy-before.json.gz` e
`workspace-clippy-results.json`. Inventário e saída completa estão em
`evidence.json.gz`.

| Diretório | Diagnósticos distintos |
| --- | --- |
| 01_core | 37 |
| 02_shell | 14 |
| 03_infra | 342 |
| 04_wiring | 13 |

Esse inventário orienta a continuação além do core; não autoriza mudanças
em contratos públicos para satisfazer avisos de estilo sem decisão de L0.

## Validação e limites

As mudanças são refatorações mecânicas equivalentes. A revisão dos diffs
não encontrou alterações em expectativas de testes. Não foram criados
testes que repetem os novos loops. A suíte existente inclui conflitos e
spans de células, fixups auto, transporte de marcadores, mistura de cores
e o teste diferencial forward_matches_exact. A checagem de nomes confirma
que nenhum teste foi retirado ou acrescentado.

Rodada principal concluída em `2026-10-14T00:39:39.488831+00:00` e rustdoc em
`2026-10-14T00:40:52.822280+00:00`. As fontes permaneceram inalteradas em cada rodada, com
fronteira em `pre-validation.json.gz` e recibo final em `after.json.gz`.
Logs, receitas, inventários e diffs completos estão em `evidence.json.gz`,
com hashes verificados após descompressão.

| Verificação | Resultado |
| --- | --- |
| Clippy core lib/testes, -D warnings | 48 → 37 diagnósticos distintos; ainda falha |
| Redução por categoria | 7 needless_range_loop, 2 ptr_arg, 1 only_used_in_recursion, 1 wrong_self_convention |
| Check de todos os targets do workspace | passou |
| Suíte core | 6.709 passaram, zero falhas, cinco ignorados |
| Nomes de testes | 6.714 antes/depois, nenhum removido |
| Build, formatação e diff check | passaram |
| Rustdoc workspace, -D warnings | passou |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter arquitetural completo | zero erros, 380 avisos, 1611 notas |

Este lote não é uma nova certificação bilateral de paridade nem uma
medição de desempenho. As pendências funcionais da auditoria continuam
abertas. A CI completa ainda depende dos demais diagnósticos de Clippy e
da matriz remota de release; nenhuma release foi publicada.
