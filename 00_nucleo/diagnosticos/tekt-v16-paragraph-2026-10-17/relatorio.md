# Relatório — V16 e parágrafos vazios

Etapa concluída. **V16 128 → 125**, **V19 1060 → 1065 notas**, V20 **867 notas** preservadas. O objetivo geral continua aberto: o gate global ainda falha pelos 125 V16. Não houve commit/push, merge ou release.

## Alterações

Os L0s proprietários foram atualizados antes do código, em fluxo contínuo ADR-0127. Apenas dois consumers e seus dois owners mudaram desde o checkpoint: paragraph.rs e stdlib/structural/par.rs. Nenhum contrato público, fase, flag, dependência ou núcleo mudou. A árvore anterior, incluindo o lote de potência e a proposta RG1, foi preservada.

- Source é transparente no reconhecimento do parágrafo e do corpo vazio. Uma projeção privada emprestada enumera as 120 variantes Content em uma fronteira única, com decisões exaustivas nos consumidores. O grupo de tipos opacos mantém sua recusa; variantes futuras exigem revisão do compilador.
- Par vazio passa a ter fim causal próprio, sem tinta. As margens acima e abaixo colapsam separadamente com as unidades vizinhas; a margem terminal permanece pendente. Isso preserva dois vazios consecutivos, inclusive sem texto, e o controle com margens fortes de Block.
- Text vazio conserva a cadeia lexical da última ocorrência que fornece o strut, incluindo tamanho e arestas locais. O frame virtual usa contribuições não negativas de top e descida, sem consultar fontes quando não há Text vazio nem emitir tinta.
- O constructor par valida spacing como Length, inclusive em/zero/negativo, e o transporta lexicalmente para o layout. Tipos inválidos recebem mensagem/hint existentes e span da última ocorrência nomeada, com fallback sintético.

Os grupos V19 estão prescritos no L0 por política comum de admissão/recusa. O diferencial remove três V16 e substitui dois grupos antigos por sete grupos prescritos (saldo de cinco notas), sem supressões ou outros diagnósticos alterados. Executar um braço não prova todas as alternativas: os grupos permanecem visíveis.

## Evidência e testes

**25 testes novos** passaram. O primeiro RED funcional teve quatro falhas/cinco controles; o RED de margens e transporte teve nove falhas/doze controles; os quatro testes finais de métricas locais também falharam antes da implementação. A hipótese prospectiva de máximo entre Text vazios foi refutada pela fonte/coleta antes do código e não conta como prova funcional; o teste foi corrigido para o fallback observado.

A repetição mediu **70 pares por corpus**, com inputs repetidos entre famílias: não são 70 defeitos independentes. **68 pares construíveis** passaram na geometria de fluxo: 42 observações antes divergentes corrigidas e 26 controles preservados. Páginas e palavras exatas; altura de página, xMin/yMin/yMax com tolerância fixa de0.002pt. Os dois casos restantes são rejeições anteriores de Block com margens negativas, mantidas explicitamente abertas. Dez valores de spacing foram medidos nos dois binários: sete rejeições indevidas corrigidas; erros/hints agora concordam, e os três Length válidos permanecem aceitos.

O xMax extraído pelo pdftotext apresenta resíduo anterior de até 0.005062pt; ele permaneceu inalterado dentro de Tekt, sem alargar a tolerância para chamá-lo de equivalente. Sua natureza (glyph/metadado/medição) permanece por classificar. A correção certifica fluxo vertical e preservação horizontal, não paridade PDF universal ou raw fields de Par.

Sonda rustc isolada: controle com todas as formas Content compila; Content futura é rejeitada na projeção; projeção futura é rejeitada nos três consumidores. Corpos produtivos extraídos, payloads externos mínimos; prova estrutural, não execução completa do workspace.

Suíte completa: **8775 passaram, zero falhas, sete ignorados**. Build workspace, fmt, Clippy all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, V1/V5/V15/V26 estrito, linhagem reversa, SARIF diferencial completo e diff check passaram.

## Recursos e proveniência

Comandos pesados sequenciais, cada gate em um novo cgroup: MemoryMax 12 GiB, MemoryHigh 10 GiB, swap zero e Cargo jobs=1. Pico do grupo **9798672384 bytes (9.126GiB)** em workspace-resources.json. Eventos memory.max **0**, memory.high **0**, sem OOM ou OOM-kill. Não é uma medição de RSS isolado nem comparação de desempenho com o lote anterior.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before.json.gz, extended-before.json.gz, coletas por fase, validation-result.json e final-state.json.gz guardam UTC, status/diffstat exatos, fontes/L0/configurações, comandos, fontes tipográficas e SHA-256 de binários/artefatos. O oráculo é o pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. O linter é 12797073972e375d9ab078cf6750c8b30b71e2b9, SHAe2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2.

O primeiro candidato e suas regressões foram preservados. Erros de escopo/string nos harnesses e interpretação de guard no verificador diferencial ficam separados; não contam como RED funcional. As fontes/L0 ficaram congeladas durante os gates finais; retomada corrigiu somente o verificador diagnóstico e não repetiu gates aprovados.

## Pendências preservadas

[Inventário V16 restante](inventario-v16-restante.md):125 casos em 96 consumers. [Pendências observadas](pendencias.json): text(tamanho) posicional, margens negativas no constructor Block e classificação do xMax extraído. Indentação/limites de justificação e raw fields genéricos de Par não foram certificados.

[Auditorias seguintes](next-audit.md) registram text relativo/posicional, a interação específica Par vazio→Box inline e a classificação de xMax; pontos ainda não medidos não são apresentados como bugs confirmados.

A proposta RG1 continua aguardando aprovação ADR-0127. Seus consumers/owners vigentes e o manifesto a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9 foram conferidos e permanecem intactos. Ao retomá-la, preservar os pares independentes power e paragraph/par que passaram depois de sua medição inicial.
