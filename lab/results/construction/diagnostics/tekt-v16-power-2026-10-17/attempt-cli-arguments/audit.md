# Próximo lote independente — calc/power

Checkpoint sem alteração produtiva ou de L0 vigente. V16 emitidos em power.rs:36,41,84 (predicados de zero e dispatch numérico). Owner exclusivo: `00_nucleo/prompts/compiler/stdlib/calc/power.md`. O contrato público RG1 pendente não abrange nem bloqueia este owner privado.

Foram medidos 20 casos/40 invocações sob 12 GiB em before-measurement.json.gz, com HEAD, working tree/status/diffstat/fontes/binários/argv/UTC. Não alegar paridade geral a partir deles: mensagens e precedência de tipos/expoente precisam de classificação antes de escolher o reparo.

Antes de materializar: confrontar o L0 e fonte pinada de calc.pow, distinguir correções de linguagem de nominalização mecânica, prescrever no owner os tipos privados/famílias e todos os fallbacks (V19), escrever RED funcional e sonda de variante futura em target Cargo isolado. Preservar tipos/valores e precedência corretos, corrigindo divergências medidas em vez de congelá-las só para fechar V16. Sem API pública nova, sem supressão, sem alterar limiares/severidades.

Uma projeção privada dos operandos pode reduzir o produto cartesiano de Value e manter dispatch legível. É hipótese a avaliar pela fonte/medição; não há autorização para codificar um tipo/processo não prescrito no L0. Alternativas explícitas nos tipos numéricos pequenos permitem exigir decisão quando crescem; não inferir cobertura de todas as combinações por executar um corpo. A implementação continua pendente de auditoria/L0/testes, não do gate RG1.
