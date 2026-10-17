# Revisão de too_many_arguments — 2026-10-15

A auditoria dos **57 avisos**, distribuídos por **28 L0**, classificou **11 correções internas** e **46 exceções pontuais justificadas**. Todas as decisões já estão prescritas nos prompts proprietários, com motivo e aceitação por função. As **22 assinaturas públicas** deste conjunto ficam preservadas.

[Lista dos 57 casos, com evidência e L0](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/too-many-arguments-2026-10-15/lista.md) · [CSV](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.csv) · [JSON completo](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.json)

## Exceções justificadas

| Motivo | Casos |
|---|---:|
| Compatibilidade pública declarada no L0 | 22 |
| Fronteiras internas explícitas: travessia, seletores, pintura, shaping e bundle | 21 |
| Helpers exclusivos de testes | 3 |

O transporte de _routines/_traced/_route em eval e _registry em realize é mantido por compatibilidade; não se afirma que os parâmetros ignorados sejam necessários ao algoritmo atual. As exceções propostas são `#[expect(clippy::too_many_arguments, reason = "...")]` na função nominal, sem supressão por módulo/crate nem aumento global de threshold.

## Ordem sugerida para as correções

| Ordem | IDs | Intervenção prescrita |
|---|---|---|
| 1 | C047, C066 | Remover _text sem uso dos helpers raster/SVG; justificar pontualmente os oito inputs restantes. |
| 2 | C102 | Pedido PDF privado com opções nomeadas; preservar None/Some, precedência e todos os wrappers públicos. |
| 3 | C045 | Collector local do walk de gradientes para mapa, saídas e contadores sincronizados. |
| 4 | C062, C064 | Reutilizar FontScenario na construção interna de PageContext, sem desmontar/remontar seus campos em argumentos. |
| 5 | C014 | Pedido emprestado de Grid/Table com campos nomeados, preservando genéricos H/V e o motor único. |
| 6 | C021 | Plano privado nomeado para a composição de indentação do outline, sem recalcular contexto/métricas. |
| 7 | C067, C069, C071 | Sessão SVG privada emprestada entre hub/grupos/links; inner dimensions usa Size existente. |

São 11 casos; os dois do primeiro lote ainda terão exceção residual após a limpeza. O desfecho projetado para os 57 é **9 ocorrências removidas pela reorganização + 46 expectativas diretas + 2 condicionadas**. Não se promete que toda função tenha no máximo sete argumentos. A manutenção estrutural é uma decisão de projeto apoiada nos dados; não foram demonstrados 11 bugs funcionais nem ganho de desempenho.

## Escopo realizado e pendente

Foram atualizados os **28 L0**, com medição antes da decisão e cláusula nominal para cada caso. O resselo ficou restrito aos respectivos **28 headers**. A comparação byte a byte, normalizando somente @prompt-hash, confirma **zero alteração de API, corpo, cfg ou atributo Rust**. Nenhuma supressão foi inserida; o inventário continua com 57 too_many_arguments e 174 avisos Clippy totais. O código será materializado em etapa posterior à entrega desta lista; novos contratos públicos exigem proposta concreta e confirmação ADR-0127.

As futuras migrações Grid/Table, PageContext e sessão SVG também precisarão atualizar os owners de seus chamadores antes do código. Esses quatro owners foram lidos e preservados em dependencies.json.gz; nenhuma autorização de API pública é inferida deles.

## Validação e RAM

Build, formato, Clippy all-targets para inventário, linhagem estrita/reversa, relatório Tekt e diff-check passaram. Tekt permaneceu em **380 avisos e 1.668 notas**, sem erro. O Clippy com -D warnings continua pendente da dívida, como esperado nesta etapa de análise. A suíte funcional não foi repetida, pois não houve alteração funcional ou de testes; as correções futuras exigem os controles prescritos nos owners.

Todos os serviços de validação/resselo usaram **MemoryMax=12 GiB**, **MemorySwapMax=0** e **CARGO_BUILD_JOBS=1**. Pico observado: **4.84 GiB**; **zero OOM**. Os limites e memory.events constam nos resultados baseline/seal/validation.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt, **working tree não commitado**. Medição inicial em 2026-10-15T16:39:18.873477+00:00; validação de 2026-10-15T16:59:19.322702+00:00 a 2026-10-15T17:02:00.393918+00:00. before.json.gz conserva fontes/L0, hashes de toda a árvore relevante, status e lista exata git diff HEAD --stat. classification.json associa caso, assinatura, cláusula L0 vigente, anchors de chamadores, hashes e prescrição atual. validation-details.json registra os comandos/retornos e a prova de preservação dos corpos/APIs; stage.diff contém somente as alterações desta etapa sobre a árvore anterior. summary.json consolida os resultados e manifest-sha256.json permite verificar os arquivos do dossiê.
