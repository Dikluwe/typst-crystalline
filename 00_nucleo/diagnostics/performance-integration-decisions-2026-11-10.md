# Destinação dos experimentos de desempenho — 2026-11-10

Consolidação da conversa `01a1b22c-8a18-7c32-af10-ecc4c2846e4d` e dos relatórios finais de laboratório. Este registro governa a fila e acompanha a integração C28 autorizada posteriormente pelo usuário. Não autoriza reverter mudanças recebidas nem alterar contratos. Substitui os estados históricos do catálogo de 05/11 para planejamento; os relatórios antigos continuam válidos para seus snapshots.

## Manter o que já foi incorporado

Confirmado pela análise final da conversa e por inspeção do checkout atual. Incorporado significa presente em source/L0, podendo estar não commitado; não significa publicado nem revalidado nesta tarefa.

| Candidato | Resultado a manter |
|---|---|
| C01 | Identidade de imagens limitada à invocação; correção de isolamento. |
| C03 | Chave estruturada de shaping com identidade completa. |
| C06 | Glifos compartilhados imutáveis. |
| C07 | Walker emite no destino do chamador, preservando buffers necessários de contêineres. |
| C12 | Clone do fallback de fonte apenas quando necessário. |
| C13 | Base da stdlib reutilizada com instanciação/identidades por arquivo. |
| C14 | Reserva de escopos por cardinalidade conhecida. |
| C15 | Empréstimo curto para len(), preservando fallbacks e mutação independente. |
| C19 | UTF-16BE/hex em uma String. |
| C22 | Transferência da tabela própria de recursos SVG/PNG. |
| C23 | Reserva única do buffer projetado de fontes. |
| C25 | Identidade e retenção do host na memoização de plugins; correção de isolamento. |

Manter também read_at e TilingBody::Image(Box<ImageElem>), incorporados antes desta rodada. Não retirar testes de correção junto com protótipos descartados.

## C28 integrado — única nova integração desta consolidação

JSON direto sobre a projeção imutável, sem construir a árvore intermediária serde_json::Value. O relatório apresenta redução de CPU e memória temporária nos callers reais medidos de query e eval, testes independentes, controles adversariais e propagação de erros. Os números são do trecho público de emissão/projeção, não da CLI completa. YAML/Raw e casos pequenos não sustentam promessa de aceleração geral.

Escopo autorizado e executado:

1. Conferir os owners atuais, atualizar primeiro seus L0s e classificar o gate ADR-0127. Preservar assinaturas, formato, ordem/duplicatas, números, erros, compact/pretty e newline.
2. Materializar somente C28 com os testes independentes úteis; não transportar sondas, mutantes ou ferramentas de bancada para o produto.
3. Revalidar RED→GREEN, testes dos callers, lint/linhagem e controles YAML/Raw; compilar em RAM própria.
4. Medir o produto consolidado com entradas idênticas. Separar o ganho da emissão do tempo total; não somar percentuais de campanhas distintas.

A execução da integração foi autorizada pelo usuário após esta consolidação; o estado e as medidas atuais são registrados ao final.

## Candidatos retirados da fila ativa — tentativas preservadas

“Arquivar” significa não continuar investindo no candidato atual. Inconclusivo não significa incorreto; uma ideia não ensaiada não pode ser chamada de refutada. Reabrir somente com novo caller relevante, orçamento medido ou hipótese diferente que enfrente a limitação registrada.

| ID | Hipótese/escopo encerrado | Resultado e motivo de encerramento | Evidência |
|---|---|---|---|
| C02 | Reduzir custo das consultas de frescor sem retirar provas. | Ganho local não demonstra aceleração geral; manter somente a atribuição histórica. | [resultado](code-performance-tests-2026-11-05.md) |
| C04 | Amortizar hashes preparando a identidade de estilos. | Benefício específico com eixos; preparo adiciona trabalho em estilos alternados. Não integrar como política geral. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c04-pipeline-style-reuse-lab/README.md) |
| C05 | Reusar buffers com reset completo. | Menos alocações não qualifica custo de reset e retenção. Encerrar o candidato atual. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c05-buffer-reuse-lab/README.md) |
| C08 | Mover mapas da tentativa já rejeitada. | Reduz alocações, sem melhora geral de CPU/pico. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c08-map-transfer-comparison-lab/relatorio.md) |
| C09 | Substituir snapshots por journal LIFO e compactar ordens. | Troca snapshots por histórico retido; compactação acrescenta realocações. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c09-order-compact-comparison-lab/relatorio.md) |
| C10 | Reusar scratch exclusivamente dentro de try_parts. | Redução de alocações sem ganho consistente de CPU/pico global. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c10-scratch-per-batch-comparison-lab/relatorio.md) |
| C11 | Reservar um elemento nos pequenos Vecs de preparo. | Economia localizada com crescimento adicional em outros casos. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c11-capacity-one-comparison-lab/relatorio.md) |
| C16 | Construir visões por metadados e reservar seus destinos. | Reduções locais de cópias/pico; CPU inconclusiva. Não está aprovado para integração. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c16-reserved-views-lab/README.md) |
| C17 | Congelar uma árvore por arquivo com índices u32. | Descartar o protótipo: conversão/pico/retenção por folha pioram. Não refuta toda representação por índices. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c17-frozen-tree-lab/README.md) |
| C18 | Evitar concatenação recursiva onde callers reais a justifiquem. | Atribuição feita; alternativa ainda não justificada pelo uso real. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c18-cst-text-attribution-lab/README.md) |
| C20 | Emitir texto/atributo em uma única String. | Benefício em cargas Crystalline densas; não consistente nos demais modos e cargas. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c20-single-string-comparison-lab/README.md) |
| C21 | Eliminar encode/decode intermediário na composição PNG. | Não encontrado caller produtivo da composição investigada; fora da fila. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c21-png-composition-attribution-lab/README.md) |
| C24 | Persistir catálogo respeitando frescor e versão. | Fora do contrato atual; adiado, não experimento refutado. | [resultado](code-performance-candidates-2026-11-05.md#c24--persistência-do-catálogo-em-disco-p3) |
| C26 | Evitar thrashing mantendo limites da sessão. | Thrashing atribuído; não há política alternativa qualificada para integração. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c26-source-session-attribution-lab/medidas.md) |
| C27 | Emprestar texto e preparar índices de diagnósticos sob demanda. | Cópias atribuídas; alternativa ainda não comparada. Não tratar como ganho demonstrado. | [resultado](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c27-diagnostic-attribution-lab/medidas.md) |

O rollback do Vec transacional de shaping também fica arquivado como tentativa separada: C07 integrado não autoriza eliminar todos os Vecs. Hipótese: evitar o Vec transacional emitindo com rollback e controlar capacidade após recusa. Resultado/motivo: a política atual não foi qualificada para integração; preservar None versus sucesso vazio e os custos de retenção/realocação. Evidências: [transação](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/shaping-transaction-lab/) e [capacidade após rollback](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/shaping-rollback-capacity-lab/).

## Destino das oito ideias

| Ideia | Decisão desta consolidação |
|---|---|
| 1 — Índices u32 | Arquivar o protótipo C17; sem migração global. |
| 2 — Arenas por fase | Sem candidato qualificado; fora da fila atual, não refutado. |
| 3 — Enums menores | Manter Box em TilingBody; sem expansão indiscriminada. |
| 4 — Despacho estático | Sem custo isolado que justifique trocar World; fora da fila atual. |
| 5 — Buffers, empréstimos e reúso | Manter melhorias incorporadas e C28 integrado; demais candidatos atuais arquivados. |
| 6 — Bounds checks | Sem ganho de pipeline qualificado; guardar inspeções, sem alteração geral. |
| 7 — Layout paralelo | Fora da fila; dependências/convergência não resolvidas por estes experimentos. |
| 8 — Persistência | Adiar; frescor e contrato precisam de decisão própria. |

## Registro mínimo e preservação

Para cada tentativa conservar hipótese, escopo, resultado, motivo de encerramento e referência à evidência/proveniência. Não manter candidatos arquivados como tarefas abertas. Os links abaixo registram as evidências existentes; esta consolidação não duplica patches, executáveis ou caches.

Não foi feita exclusão física de artefatos externos nem reversão do checkout. “Descartar” nesta decisão é retirar da integração e da fila ativa. A limpeza física dos pacotes históricos é separada: não apagar código/testes úteis ou provas únicas junto com temporários.

## Evidências dos labs

- [c04-key-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c04-key-attribution-lab/README.md)
- [c04-pipeline-style-reuse-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c04-pipeline-style-reuse-lab/README.md)
- [c04-prepared-style-comparison-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c04-prepared-style-comparison-lab/README.md)
- [c05-buffer-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c05-buffer-attribution-lab/README.md)
- [c05-buffer-reuse-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c05-buffer-reuse-lab/README.md)
- [c06-glyph-copy-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c06-glyph-copy-attribution-lab/README.md)
- [c06-immutable-glyphs-comparison-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c06-immutable-glyphs-comparison-lab/README.md)
- [c08-map-transfer-comparison-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c08-map-transfer-comparison-lab/relatorio.md)
- [c09-admissions-journal-comparison-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c09-admissions-journal-comparison-lab/relatorio.md)
- [c09-checkpoint-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c09-checkpoint-attribution-lab/relatorio.md)
- [c09-order-compact-comparison-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c09-order-compact-comparison-lab/relatorio.md)
- [c09-peak-origin-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c09-peak-origin-lab/relatorio.md)
- [c10-planning-batches-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c10-planning-batches-lab/relatorio.md)
- [c10-scratch-per-batch-comparison-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c10-scratch-per-batch-comparison-lab/relatorio.md)
- [c10-selector-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c10-selector-attribution-lab/relatorio.md)
- [c11-capacity-one-comparison-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c11-capacity-one-comparison-lab/relatorio.md)
- [c11-preparation-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c11-preparation-attribution-lab/relatorio.md)
- [c12-lazy-font-fallback-comparison-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c12-lazy-font-fallback-comparison-lab/relatorio.md)
- [c12-style-resolution-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c12-style-resolution-attribution-lab/relatorio.md)
- [c14-import-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c14-import-attribution-lab/relatorio.md)
- [c14-import-phase-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c14-import-phase-attribution-lab/relatorio.md)
- [c14-scope-reserve-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c14-scope-reserve-lab/relatorio.md)
- [c14-stdlib-clone-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c14-stdlib-clone-attribution-lab/relatorio.md)
- [c15-borrowed-len-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c15-borrowed-len-lab/README.md)
- [c15-clone-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c15-clone-attribution-lab/README.md)
- [c16-carrier-census-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c16-carrier-census-lab/README.md)
- [c16-metadata-views-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c16-metadata-views-lab/README.md)
- [c16-reserved-views-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c16-reserved-views-lab/README.md)
- [c16-tail-cost-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c16-tail-cost-lab/README.md)
- [c16-textstyle-clones-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c16-textstyle-clones-lab/README.md)
- [c17-frozen-tree-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c17-frozen-tree-lab/README.md)
- [c18-cst-text-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c18-cst-text-attribution-lab/README.md)
- [c19-pdf-serialization-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c19-pdf-serialization-lab/README.md)
- [c19-utf16-single-string-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c19-utf16-single-string-lab/README.md)
- [c20-html-escaping-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c20-html-escaping-attribution-lab/README.md)
- [c20-single-string-comparison-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c20-single-string-comparison-lab/README.md)
- [c21-png-composition-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c21-png-composition-attribution-lab/README.md)
- [c22-export-fonts-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c22-export-fonts-attribution-lab/README.md)
- [c22-owned-table-transfer-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c22-owned-table-transfer-lab/README.md)
- [c23-catalog-phases-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c23-catalog-phases-lab/medidas.md)
- [c23-catalog-traffic-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c23-catalog-traffic-lab/medidas.md)
- [c23-single-reserve-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c23-single-reserve-lab/medidas.md)
- [c23-world-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c23-world-attribution-lab/README.md)
- [c26-source-session-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c26-source-session-attribution-lab/medidas.md)
- [c27-diagnostic-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c27-diagnostic-attribution-lab/medidas.md)
- [c28-direct-json-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c28-direct-json-lab/medidas.md)
- [c28-query-attribution-lab](/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c28-query-attribution-lab/medidas.md)

O catálogo original e sua primeira rodada permanecem em [candidatos](code-performance-candidates-2026-11-05.md) e [testes](code-performance-tests-2026-11-05.md). Custos históricos do produto em [comparação de 06/11](tekt-vanilla-current-2026-11-06.md); não são novas medidas do estado atual. A divergência de outline continua impedindo comparação de trabalho equivalente com vanilla.

## Estado da integração C28 — 10/11/2026

**Integrado no working tree, sem commit/push. Fila ativa desta campanha encerrada.**
Somente C28 foi materializado; todos os demais candidatos atuais seguem arquivados
ou já incorporados conforme as tabelas. Nenhuma campanha arquivada foi reaberta,
nenhum protótipo adicional foi integrado e nenhum artefato histórico foi apagado.
Os catálogos de05/11 receberam somente uma nota que remete a esta decisão atual.

### Mudança e gate

Owners `00_nucleo/prompts/shell/cli.md` e
`00_nucleo/prompts/shell/cli/semantic_json_tests.md` foram gravados antes de código/testes.
ADR-0127: fluxo contínuo de mudança privada e reversível, sem nova assinatura,
flag/default, fase ou quebra de compatibilidade. Vista privada SemanticJson
empresta a projeção e emite com serde_json no Vec final. HashMap raso por objeto
normaliza duplicados mantendo primeira posição e último valor; seu custo é contado.
A projeção completa, --field/--one, YAML/Raw e os callers permanecem iguais.
O conversor antigo fica somente cfg(test) como oráculo. Nenhuma sonda, flag
experimental, dependência ou unsafe novo entrou no produto.

### Correção e linhagem

Tekt: A implementou em cópia congelada; B portou testes úteis independentemente,
sem ver A; C aplicou e executou. As novas regressões deram RED por ausência do
adaptador privado no código recebido, antes da implementação. Isso não é uma
falha semântica do algoritmo antigo: seus96 testes de CLI continuam passando.
Após implementar:7 regressões novas passam; módulo CLI103 testes; suíte real
`cargo test --release --offline --locked -p typst-shell --lib -- --test-threads=1`:
**120 passaram**, nenhum ignorado/falho. Cargo build release offline/locked do
binário typst-wiring passou em baseline e integrado; todas as camadas produtivas
são dependências desse build. V15/V26 e lint estrito com fail-on warning passaram.

Goldens independentes e oráculo cobrem ordem/duplicatas não adjacentes e aninhadas,
i64 extremos/>2^53, floats finitos/subnormais/-0 e não finitos→null, Unicode/escapes,
vazios/profundidade/objetos largos, snapshots None/Somevazio/Somecampos,
func/label, cardinalidade/field, YAML/Raw, bytes owned após descarte e writer que
falha em cada prefixo de compact/pretty sem retry. Mutações dirigidas rejeitadas
**6/6**: primeiro duplicado vence, ordenar chaves, i64 via f64, não finitos string,
ignorar erro de filho e retirar newline. Mutantes só em unidades descartáveis RAM.

31 casos públicos: **372 processos /1.824 ciclos** com bytes completos, status,
comprimento e capacidade idênticos entre variantes/rodadas. Mais10 casos CLI:
**140 processos medidos +20 aquecimentos**, stdout/stderr/status completos iguais.
Os JSONs dos callers CLI foram também decodificados e conferidos por valores
esperados (ids/texto/nested/cardinalidade). Nenhuma nova certificação de paridade
global com vanilla; outline continua semanticamente divergente.

### Emissão pública — produto consolidado nas duas variantes


Baseline é o checkout recebido com todas as melhorias anteriores; integrado
acrescenta somente C28. Driver chama a biblioteca real. Geração de Value/World,
fontes, eval/layout/seleção e escrita ficam fora deste intervalo; projeção,
seleção de campo/cardinalidade e emissão ficam dentro. Query real usa64 metadata
com text64B e nested; eval real usa512 registros equivalentes e pretty.
Cinco rodadas de processos pareados, ordem alternada;5 ciclos por processo
(4 nos callers reais), primeiro descartado; CPU=mediana das medianas restantes.
Relógio CLOCK_PROCESS_CPUTIME_ID sem medidor de heap. Faixa da razão é mínimo–máximo
dos cinco pares, não intervalo de confiança. Heap em execução separada:
A=alloc/alloc_zeroed; R=realloc; tráfego=allocBytes+reallocNew; pico adicional=
heap Rust vivo máximo acima do inicial. Não é RSS ou memória física do allocator.

| Caso | CPU base→integrado µs | Razão pareada mediana [mín;máx] | A base→integrado | R | Tráfego B base→integrado | Pico adicional B base→integrado |
|---|---:|---:|---:|---:|---:|---:|
| actual-query-json | 110.40→68.42 | 0.6203 [0.6134;0.6324] | 1411→770 | 71→71 | 144128→94656 | 124288→60200 |
| actual-eval | 512.17→355.00 | 0.6904 [0.6812;0.8932] | 6659→3586 | 10→10 | 860032→546688 | 729088→356468 |
| eval-json | 2329.74→1100.43 | 0.4668 [0.4377;0.5310] | 22531→12290 | 11→11 | 2393984→1568640 | 2131968→966988 |
| query-id | 439.34→286.84 | 0.6527 [0.5999;0.6560] | 5123→5122 | 4→4 | 393088→356224 | 352256→352256 |
| query-text | 329.21→291.12 | 0.8916 [0.7594;0.9422] | 6147→5634 | 9→9 | 585600→515968 | 352320→352320 |
| actual-query-yaml | 152.40→148.33 | 0.9705 [0.7800;1.0132] | 3074→3074 | 139→139 | 186982→186982 | 87508→87508 |
| raw-text | 18.36→17.91 | 0.9793 [0.9497;1.0284] | 1→1 | 0→0 | 65536→65536 | 65536→65536 |

Capacidade e retenção de saída iguais em todos os casos: query real16.384B,
eval real131.072B. Ganho de heap é temporário. query-id/text continuam com pico
dominado pela projeção completa. YAML/Raw, missing e recusa preservam exatamente
contadores/tráfego/pico/retenção; oscilações temporais desses controles não são
ganho atribuível a C28. As cinco razões de CPU dos dois callers JSON reais foram
menores que1; não se extrapola para todos os documentos ou CLI completa.

### CLI inteira — ganho distinto do trecho de emissão

Executáveis produtivos reais com as mesmas entradas/flags/versão e todas as
melhorias recebidas. Cada amostra é processo novo, com1 processo de aquecimento
por lado/caso e7 pares medidos, ordem alternada. CPU=ru_utime+ru_stime por wait4,
wall inclui lançamento, inicialização, eval ou compile/layout/query, emissão e
escrita stdout/stderr em arquivos RAM. RSS é máximo do processo, não da emissão.
Tempos abaixo são medianas; razões são medianas dos7 quocientes pareados.
Nenhuma instrumentação de heap ou compilação concorrente.

Ambiente explícito e igual: Fontconfig privado vazio, fontes embutidas disponíveis,
RAYON_NUM_THREADS=1, SOURCE_DATE_EPOCH=0, TZ=UTC, NO_COLOR=1 e cache/tmp privados RAM.
Não é perfil com o catálogo de fontes do sistema padrão. Sem flush do page cache,
controle de frequência ou afinidade: pequenas diferenças são inconclusivas.
Query fonte `#for i in range(N) { metadata((id: i, text: PAYLOAD, nested: (i, none, true))) }`;
eval `range(N).map(i => (id: i, text: PAYLOAD, nested: (i, none, true)))`;
PAYLOAD é `abc_xyz-12` repetido7 vezes (63B). query64-pretty usa --pretty;
eval512 usa --pretty, eval4096 compacto. YAML e Raw usam os flags correspondentes;
eval-scalar usa9007199254740993. Erro query64 com --one é controle anterior à emissão.

| CLI | CPU base→integrado ms | Razão pareada mediana [mín;máx] | Wall base→integrado ms | RSS processo base→integrado KiB |
|---|---:|---:|---:|---:|
| query64 | 7.007→7.233 | 1.0323 [0.8511;1.1134] | 7.209→7.396 | 16556→16508 |
| query1024 | 93.617→91.761 | 0.9904 [0.9548;1.0072] | 94.160→92.122 | 29932→29780 |
| query64-yaml | 6.890→7.040 | 1.0197 [0.9527;1.0661] | 7.056→7.196 | 16604→16468 |
| query64-pretty | 6.916→6.941 | 1.0244 [0.8963;1.0676] | 7.084→7.109 | 16556→16680 |
| query-one-error | 6.986→7.163 | 1.0338 [0.9091;1.0679] | 7.175→7.326 | 16440→16296 |
| eval512 | 5.758→5.568 | 0.9893 [0.9229;1.0280] | 5.927→5.724 | 14144→14144 |
| eval4096 | 19.586→16.367 | 0.8363 [0.8160;0.8503] | 19.842→16.600 | 22700→19560 |
| eval512-yaml | 5.372→5.498 | 1.0302 [0.9827;1.0843] | 5.534→5.663 | 15160→15160 |
| eval-scalar | 3.879→3.837 | 0.9887 [0.9270;1.0432] | 4.048→3.991 | 15160→15160 |
| eval-raw | 3.824→3.988 | 1.0304 [0.8610;1.1010] | 3.975→4.133 | 15160→15160 |

**Conclusão restrita:** eval4096 favoreceu o integrado nos7 pares de CPU; RSS
22508–22772→19404–19676KiB. Query1024 favoreceu5/7 pares, com faixa incluindo
piora; query pequena/pretty e eval menor são mistos, assim como YAML/Raw e erro.
O lote não demonstra aceleração geral da CLI. O ganho do trecho de emissão não
pode ser aplicado ao tempo total, nem somado a percentuais de campanhas antigas.
A integração reduz a árvore temporária com comportamento preservado; o ganho total
depende da participação dessa emissão na carga.

### Proveniência desta revalidação

HEAD `0a15ccebf861e3cc0ea16d0397d55dc057664556`, **working tree não commitado**. Snapshot baseline em `2026-11-10T18:58:57.128143+00:00`.
Matriz emissão CPU/heap e CLI em 2026-11-10T19:34:27.582963+00:00 a 2026-11-10T19:34:33.238897+00:00 UTC.
CLI especificamente `2026-11-10T19:34:30.614738+00:00` a `2026-11-10T19:34:33.214185+00:00`. Código ficou estável durante todas as amostras; testes/mutantes terminaram antes.
Rust1.92.0, x86_64 Linux, serde_json1.0.150/indexmap2.13.0; cargo release opt-level=s/lto=thin/codegen-units1, job1 e incremental0. Drivers opt-level=s/lto=off/codegen-units1.
Baselines e integrado usam caches físicos separados em `/dev/shm/tekt-c28-integration-20261110`; bibliotecas têm features correspondentes. Fonte baseline congelada antes de editar L0/código; fonte integrada confere com os quatro arquivos produtivos/L0 atuais.
Alterações rastreadas em relação ao HEAD no estado medido (nota de fila já incluída; registro de decisão é não rastreado):

```text
 00_nucleo/diagnostics/code-performance-candidates-2026-11-05.md |   2 +
 00_nucleo/diagnostics/code-performance-tests-2026-11-05.md      |   2 +
 00_nucleo/prompts/core/compiler/eval/call_dispatch/call.md      |  57 +++++++++++++++++-
 00_nucleo/prompts/core/compiler/eval/stdlib_snapshot.md         |  29 ++++++++-
 00_nucleo/prompts/core/compiler/eval/stdlib_snapshot/tests.md   |  31 +++++++++-
 00_nucleo/prompts/core/compiler/layout/link.md                  |   9 ++-
 00_nucleo/prompts/core/compiler/layout/optical_margin.md        |   9 ++-
 00_nucleo/prompts/core/compiler/layout/text/style.md            |  42 ++++++++++++-
 00_nucleo/prompts/core/entities/layout_types.md                 |  11 +++-
 00_nucleo/prompts/core/entities/mod.md                          |   9 ++-
 00_nucleo/prompts/core/entities/scope.md                        |  38 +++++++++++-
 00_nucleo/prompts/infra/export/builder/fonts.md                 |   9 ++-
 00_nucleo/prompts/infra/export/builder/serialization.md         |  48 ++++++++++++++-
 00_nucleo/prompts/infra/export/font_binding.md                  |   9 ++-
 00_nucleo/prompts/infra/export/fonts.md                         |   9 ++-
 00_nucleo/prompts/infra/export/render.md                        |   9 ++-
 00_nucleo/prompts/infra/export/stream/draw.md                   |   9 ++-
 00_nucleo/prompts/infra/export/stream/text.md                   |   9 ++-
 00_nucleo/prompts/infra/export/stream/text/runs.md              |   9 ++-
 00_nucleo/prompts/infra/export/svg/glyphs.md                    |   9 ++-
 00_nucleo/prompts/infra/export/svg/tests.md                     |   9 ++-
 00_nucleo/prompts/infra/export/tests/mod.md                     |   9 ++-
 00_nucleo/prompts/infra/fontdb/selective.md                     |  57 +++++++++++++++++-
 00_nucleo/prompts/infra/pipeline/pdf.md                         |   9 ++-
 00_nucleo/prompts/infra/pipeline/resources.md                   |  62 ++++++++++++++++---
 00_nucleo/prompts/infra/shaper/cache.md                         |   9 ++-
 00_nucleo/prompts/infra/shaper/shaping.md                       |   9 ++-
 00_nucleo/prompts/infra/shaper/tests.md                         |  10 +++-
 00_nucleo/prompts/infra/shaper/text_identity.md                 |   9 ++-
 00_nucleo/prompts/shell/cli.md                                  |  44 +++++++++++++-
 01_core/src/compiler/eval/call_dispatch/call.rs                 | 299 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/eval/stdlib_snapshot.rs                    |   6 +-
 01_core/src/compiler/eval/stdlib_snapshot/tests.rs              | 146 ++++++++++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/layout/link.rs                             |   6 +-
 01_core/src/compiler/layout/optical_margin.rs                   |   6 +-
 01_core/src/compiler/layout/text/style.rs                       |   9 ++-
 01_core/src/entities/layout_types.rs                            |  11 ++--
 01_core/src/entities/mod.rs                                     |   5 +-
 01_core/src/entities/scope.rs                                   | 103 +++++++++++++++++++++++++++++++-
 02_shell/src/cli.rs                                             |  55 ++++++++++++++++-
 03_infra/src/export/builder/fonts.rs                            |  10 ++--
 03_infra/src/export/builder/serialization.rs                    | 415 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++---
 03_infra/src/export/font_binding.rs                             |   6 +-
 03_infra/src/export/fonts.rs                                    |   6 +-
 03_infra/src/export/render.rs                                   |   6 +-
 03_infra/src/export/stream/draw.rs                              |  16 ++---
 03_infra/src/export/stream/text.rs                              |  12 ++--
 03_infra/src/export/stream/text/runs.rs                         |   8 +--
 03_infra/src/export/svg/glyphs.rs                               |   8 +--
 03_infra/src/export/svg/tests.rs                                |   8 +--
 03_infra/src/export/tests/mod.rs                                |  12 ++--
 03_infra/src/fontdb/selective.rs                                |  58 ++++++++++++++----
 03_infra/src/pipeline/pdf.rs                                    |   6 +-
 03_infra/src/pipeline/resources.rs                              | 373 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 03_infra/src/shaper/cache.rs                                    |  10 ++--
 03_infra/src/shaper/shaping.rs                                  |   9 +--
 03_infra/src/shaper/tests.rs                                    |  14 +++--
 03_infra/src/shaper/text_identity.rs                            |   6 +-
 58 files changed, 2065 insertions(+), 150 deletions(-)
```

Arquivos não rastreados do estado medido:

```text
00_nucleo/diagnostics/performance-integration-decisions-2026-11-10.md
00_nucleo/prompts/core/compiler/layout/text/style/font_fallback_tests.md
00_nucleo/prompts/core/entities/shared_glyphs.md
00_nucleo/prompts/core/entities/shared_glyphs/tests.md
00_nucleo/prompts/infra/fontdb/selective/single_reserve_tests.md
00_nucleo/prompts/infra/shaper/tests/shared_glyphs.md
00_nucleo/prompts/infra/tests/projected_buffer_reserve.md
00_nucleo/prompts/infra/tests/support/projected_buffer_alloc.md
00_nucleo/prompts/shell/cli/semantic_json_tests.md
01_core/src/compiler/layout/text/style/font_fallback_tests.rs
01_core/src/entities/shared_glyphs.rs
01_core/src/entities/shared_glyphs/tests.rs
02_shell/src/cli/semantic_json_tests.rs
03_infra/src/fontdb/selective/single_reserve_tests.rs
03_infra/src/shaper/tests/shared_glyphs.rs
03_infra/tests/projected_buffer_reserve.rs
03_infra/tests/support/projected_buffer_alloc.rs
```

As alterações anteriores foram preservadas por conteúdo; somente os arquivos C28 e os três diagnósticos de fila foram modificados por esta integração. Código/testes úteis dos labs e evidências históricas externas permanecem. Caches, executáveis, logs, mutantes e outputs descartáveis próprios são removidos ao encerrar; nada disso entra no Git.
