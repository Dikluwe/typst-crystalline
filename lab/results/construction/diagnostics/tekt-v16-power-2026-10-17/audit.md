# Auditoria inicial — calc/power

Estado histórico do checkpoint, anterior à prescrição e implementação. O estado final fica em relatorio.md, validation-result.json e final-state.json.gz.

Três V16 nas linhas 36,41,84. Owner `00_nucleo/prompts/compiler/stdlib/calc/power.md`, sem interseção com os 53 owners RG1. Nenhuma mudança de código/L0.

A primeira tentativa usou dois argumentos posicionais para eval e não avaliou as expressões: erro de setup, sem crédito funcional. Artefatos preservados em attempt-cli-arguments/. A sintaxe foi conferida no --help: eval EXPRESSION --in INPUT. A medição precisa terminar com códigos 0/1 e resultados reais antes de classificar linguagem versus mecânica. Depois: ler fonte pinada/L0, prescrever complementos e V19, RED funcional e sonda futura isolada antes do código. Não preservar uma divergência medida só para fechar lint.

Medição corrigida concluída: 20 casos/40 invocações reais, todos com exit0/1; controle int-positive tem sucesso nos dois binários. Fontes permanecem iguais ao checkpoint. before-measurement.json.gz contém o corpus funcional; o artefato antigo de argumentos permanece sem crédito. Próxima ação: classificar os erros/precedência antes de prescrever o L0; não houve implementação.
