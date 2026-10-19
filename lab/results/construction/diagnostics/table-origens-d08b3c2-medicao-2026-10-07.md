# D08B3-C2 — medição e proposta de origem/forma

Estado histórico da medição: medição concluída; código ainda não corrigido naquele momento. Aguardando ratificação do contrato público e da fronteira causal descritos em `00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`. Os quatro candidatos centrais completos e bases SHA-256 estão em manifest.json; são rascunhos sob materialization, não L0 vigente. Owners auxiliares de transparência serão auditados/individualizados antes da instalação e de sua materialização; este pacote não legitima código sem o respectivo owner L0.

## Atualização de estado

Contrato confirmado e C2 executado em 2026-10-07. Esta página conserva abaixo a medição/proposta anterior à decisão; suas afirmações de pendência e preservação dizem respeito àquele momento. Estado atual, testes, proveniência e limite geométrico: `00_nucleo/diagnosticos/table-origens-d08b3c2-execucao-2026-10-07.md`.

## Proveniência

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. UTC 2026-10-07T17:40:26.485941+00:00 até 2026-10-07T17:40:27.328561+00:00. Estado/diff HEAD --stat/lista e hashes completos, fonte de cada sonda, argv/cwd/ambiente/stdout/stderr/status: `table-origens-d08b3c2-2026-10-07/baseline.json.gz`. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Tekt congelado C SHA-256 23eb4227d17b332af08414506b81c685c63cfa440d18b18e0c2ec69fd23c675c; igualdade dos hashes produtivos/L0 contra tested-final C verificada em source-verification.json. Binário target raiz ausente nesta retomada; usada cópia congelada validada, sem alegar rebuild. Coleta before continha str(function) inválido; descartada como prova de func. baseline corrigiu para repr(function).

## Resultados anteriores à decisão

| Sonda | Saída completa igual |
|---|---|
| bare-conflict | não |
| alias-conflict | não |
| spread-conflict | não |
| local-body | não |
| plain-func | sim |
| set-func | não |
| show-func | não |
| set-show-func | não |

As três rejeições têm mensagens/hints iguais e caret/trace diferentes. Alias usa origem [a] na definição, spread usa origem do item; Args.occurrences isolado não basta. local-body conserva cores e ordem dos metadados neste caso, mas publica fonte sequence em vez de styled cru. set/show locais também divergem diretamente em c.func(). Não transformar esse controle pontual em prova universal de execução/ordem.

Fonte e decisão estão no contrato candidato, com file:line. Proposta: novo SourceContent/Content::Source para origem transparente conservada por alias/spread; preservar Styled lexical da cauda local e executar snapshot causal na inserção. Nenhuma Location nova ou campo Table/Cell. Não alterar Realized globalmente para observar original. A mudança de contrato público e da fronteira causal exige confirmação ADR-0127 §2.1/2.3/2.4; os pedidos genéricos anteriores não ratificaram este desenho ainda inexistente. A correção continua aberta até RED→GREEN/recoletas/build/suítes/lint.

## Preservação

Nenhum arquivo produtivo ou L0 vigente mudou nesta etapa; sem commit/reset/resselo. Foram escritos somente script/evidências, relatório, plano e proposta/candidatos. Métricas horizontais anteriores, rich casts, Table geral/multipágina e PDFUA continuam abertas. Manifesto SHA-256: 5e5086404c6359dad6658b77dc465aa4a4a6bba4c99f7a505245092a0e07c79c.
