# P1340 — auditoria inicial independente

Regime: executado sem atestação de isolamento. Verificador `/root/p1311_review`; leitura de baseline, L0, código antecedente, contrato e evidências históricas, sem escrita dos artefatos julgados. Contexto P1311/P1339 herdado é declarado, não prova de isolamento. Esta decisão não é selo nem GO de implementação.

## Transferência e baseline

Baseline `p1340-baseline.json` SHA-256 `0869202e774bd7b76278365aac7292d45a1c930be42cf83270845c985cb5000a`, HEAD `2f42d64253547734564513a1159ee6b584c1c4b4` mais working tree integral congelada. Recibo V15/V26 real: 2026-09-10T13:22:43.134451–13:22:46.719561 UTC, exit 0. Hashes dos arquivos produtivos modificados/novos capturados continuam iguais na inspeção. Array e Style são antecedente, sem crédito novo de conclusão.

A seção P1339 foi extraída do diff literal congelado do antigo `infra/pipeline.md` e comparada ao novo `infra/pipeline/context_stabilization.md`: igualdade byte a byte, 11.075 bytes, SHA-256 `7e5aac98add0976920e777293c7799d3fa132bf30439f43e3672b8d03f0bf91b`. Novo owner SHA-256 `62f27ec604a0b9b0d4121ecf43ac556a18a59cd53947744c29187c4d49c1781c`; pai `e69691e7ced9e1cc26ddc0b752ff64a418322cad68c21a4b5bc68bbae6f66c1d`. Introdução/roteamento novos preservam coordenação privada L3, sem segundo algoritmo ou contrato público. Individualização é compatível com ADR-0109/0129; o consumer novo ainda terá de satisfazer ownership/linhagem após materialização.

## Evidência reutilizável, não execução futura presumida

O selo P1339 `35f00c4b9e15a010692017f5083ea4f451f4104a3730022136972e151ac0a8ee` e sua discriminação real permanecem antecedentes imutáveis. Transferência literal não exige executar novamente toda a matriz C por mudança de hash. O sucessor deve pinar seleção de casos/predicados/negativos relevantes e suas testemunhas já medidas; não atribuir todos os 20 ataques à integração nova indiscriminadamente.

Contrato proposto `p1340-contract-proposal.json`, SHA-256 `d4508967910e1a47e9e0f4b803bb559476f65f9a16cf7d18d7932174aab9cadc`: 18 pins de entrada conferidos. Os predicados lifecycle históricos permanecem obrigatórios, inclusive testemunhas de orçamento único, retenção real, erro e sink. Seu estado anterior NotDue/F tinha zero execução e zero crédito em C; não pode ser promovido a GREEN por reutilização documental.

RED local real confirmado no recibo `p1340-pipeline-red-r1.json`, SHA-256 `253101e4893e4fd8ba0bd90d0abd81e7e9b5f362b5b1ad7bc56265600c195bef`: 2026-09-10T13:24:41.291399–13:25:20.333052 UTC, 39,169781844 segundos; dois testes efetivos, erro independente preservado e dependência de update posterior falha semanticamente. Build não foi contado como RED. É teste local antecedente; a seleção pública independente ainda requer execução ou vínculo explícito de evidência atual suficiente.

## Limite concreto da interface

As três APIs aprovadas existem em `eval/mod.rs:5661,5786,5812`. Elas permitem registrar seleção, validar e obter diagnósticos; L3 pode conservar o EvalContext/resultado/recursos de cada geração. Porém `context_reads_valid_for:5798` reduz tanto Different como Unproven a `Ok(false)`. Diagnose só emite detalhes de mudança comprovada e não relata Query/Location. Portanto `false` ou ausência de detalhe não provam nem opacidade nem convergência.

F09 exige que Unproven não vire exportação bem-sucedida ou warning vanilla. Antes de selo/implementação, o desenho deve demonstrar caminho efetivo que cumpra essa obrigação, inclusive mistura de leitura Different com leitura Unproven. Não inferir a causa pela mensagem, não inspecionar registry em L3, não substituir discriminação produtiva por porta cfg(test). Se informação pública disponível for insuficiente, relatar ao autor da intenção a menor extensão necessária; não autorizar API ou política nova silenciosamente.

O collector lifecycle histórico é somente contrato de ligação `fn(&ActualPipelineInput)->Json`; não há porta produtiva ligada em `pipeline.rs`. Ligação test-only tardia é admissível sob a política já congelada, mas deverá projetar transições reais do algoritmo comum, nunca simular ciclo ou decidir retenção. Testes F04/F05 de dependência, portas F07/F08/F09 e testemunhas estruturais continuam devidos.

## Próxima decisão estreita

Preparar sucessor preservando selo antigo, transferindo apenas ownership das portas L3, pinando baseline/seleção/evidência reutilizada e explicitando gates ainda não executados. Resolver antes o limite de informação acima e a seleção discriminatória afetada. Sem nova matriz global por mero hash; sem redução das obrigações; sem aprovação de Array, repr, HTML ou P1339 geral. Nenhuma execução semântica foi iniciada pelo verificador nesta auditoria.
