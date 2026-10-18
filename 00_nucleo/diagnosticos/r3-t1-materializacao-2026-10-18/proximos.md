# Entrega para limpeza e organização

A sessão de implementação está encerrada e o objetivo geral fica pausado por
pedido humano. Este documento é uma fila de organização; não autoriza código
fora dos L0s nem declara concluídas as frentes herdadas. Proveniência de cada
resultado no relatório e seus anexos, sempre distinguindo revisão 9 de 10.

## Ordem proposta para organizar o que temos

1. **Consolidar o estado local.** Usar `estado-encerramento.json.gz` e
   `escopo-etapa.json` para distinguir o diff acumulado dos arquivos deste lote.
   Preparar commits coerentes após revisão; não juntar todo o histórico em um
   commit genérico. Nenhum commit foi criado neste encerramento.
2. **Criar um índice único de situação.** Para cada frente registrar owner L0,
   aprovação aplicável, implementação, testes, prova CLI, pendências e dependências.
   Relatórios antigos permanecem históricos; não tratar seus “próximos passos”
   como estado vigente sem reconciliar com este relatório.
3. **Classificar os artefatos.** Separar evidência canônica, scripts reproduzíveis,
   rascunhos/propostas substituídas e caches reconstruíveis. Preservar as fontes
   exatas, dados importados, manifests e oráculos antes de apagar outputs/bin.
   Nenhuma lista de exclusão está aprovada ou executada.
4. **Reconciliar documentação de execução.** `andamento.md` e `progress.json`
   apontam agora ao encerramento; cópias anteriores foram preservadas. Relatórios
   mais antigos, como mixed-style/proximos e show-returns, conservam o contexto
   histórico e podem ter indicações de estado superadas por esta entrega.
5. **Priorizar o backlog por impacto e dependência**, usando a lista abaixo.
   Depois da organização, retomar apenas os lotes escolhidos, com L0 primeiro.

## O que ainda precisa ser feito

| Frente | Estado / próxima evidência necessária |
|---|---|
| Context: regras textuais | Corrigir herança global/local; as sondas context-global/context-local continuam em A contra R/L no vanilla. |
| Context: identidade/query | Corrigir contagem de equações em múltiplos contextos e após heading; não inventar Location sintética nem copiar contagem esperada no teste. |
| R3-T2: retornos de show | Investigação em ../show-returns-2026-10-18/relatorio.md; implementar coerção comum preservando morfologia Raw/Text/None/Module e regras aplicadas ao produto. |
| V16 | 105 erros herdados, auditoria por owner/L0; manter V19 informativo e testar alternativas pertinentes. |
| Bibliografia/layout herdado | Reconciliar ../mixed-style-metrics-2026-10-18/proximos.md: CSL plain/optimized, containers, regiões além do certificado, fontes, checkpoint com prefixo e decorações completas continuam abertos. O caso bib-content-alias foi corrigido neste lote. |
| Inventário geral de paridade | ../paridade-novo-vanilla-2026-10-17/inventario.md e ../sequence-r3-2026-10-18/escopo-geral.md continuam referências: RM1, RG, Figure/Paint/Symbol, reflexão, HTML/export e demais E01–E20 precisam de reconciliação por caso. Não considerar todos esses itens recém-medidos. |
| RAM de produção | Medir Tekt e vanilla em perfis comparáveis, com mesmos documentos/fontes/cache. O teto de 12 GiB foi respeitado; redução de RAM não foi demonstrada. |

## O que já pode ser usado como evidência

- Revisão 10: suíte completa, build, Clippy, Rustdoc, fmt e linhagem passaram.
- Revisão 9: corpus textual PDF/HTML, SVG de cores/precedência e preservação do
  corpus geométrico, com uma correção adicional de alias comprovada.
- V17 novos foram corrigidos; gate Tekt global segue bloqueado por V16.
- Aprovações e manifests anteriores continuam preservados. Não houve release.

Esta fila não é um Prompt L0 e não legitima materialização.
