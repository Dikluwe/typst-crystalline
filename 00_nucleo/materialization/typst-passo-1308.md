# Passo 1308 — concluir as quatro correções de paridade restantes

Autorizado pelo dono: “Pode escrever o passo e implementar autorizo”, após
explicação das quatro fronteiras no relatório P1307-R6. Este passo coordena
a execução; os prompts L0 proprietários legitimam o código. Não autoriza
stage, commit, push, limpeza de históricos ou expansão geral do compilador.

## Ponto de partida medido

HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais working tree P1306/P1307
não commitada; inventário, diff/stat e executáveis em
`00_nucleo/diagnosticos/p1308-baseline.json`, SHA-256
`62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394`.
O executável anterior foi preservado em `/dev/shm/p1308-baseline.XzjFgA/typst`.
Referência: vanilla upstream ratificado `a51e02804`, não string de versão.

O recibo independente `p1307-r6-verification-final.json` registra matriz
1926 Preserved / 56 Violated / 0 Unknown, 22 testes novos verdes e três
falhas históricas. Esses números pertencem às execuções e hashes pinados
nesse recibo, não constituem medição do passo novo.

## Trabalho autorizado

1. `Args + none` e seu par simétrico preservam integralmente Args: corrigir
   somente esses pares em Add; não generalizar toda a tabela de join.
2. Diagnósticos de callbacks de arguments: cast de filter usa nome longo e
   origem da função; panic usa chamada inteira, inclusive aliases/With.
   Medir definição, uso e argumentos sintéticos para não fabricar âncoras.
3. Eval de expressão disponibiliza sua Source numerizada ao World usado pelo
   Engine mediante overlay privado puro; demais serviços são delegados e o
   World externo não é modificado. Trace continua com regra de contenção.
4. Args causal usa o formatter curto/multilinha ratificado, sem elisão.
   Retificar independentemente expectativas históricas explicitamente
   supersedidas de Location, With e, após medição, Args longo.

L0 primeiro: arithmetic/join, eval, call_dispatch/math, collections/panic,
func/closures, repr e owners dos testes. Nenhuma API pública nova é necessária;
eventual âncora de Func é dado privado, não campo de ClosureRepr público.
Sem novo modo, flag, fase, whitelist ou reconstrução de origem por igualdade.

## Execução e verificação

Aplicar a skill tekt-materializacao-segregada no regime **executado sem
atestação de isolamento técnico**: root coordena L0/implementação; autor
independente mede contrato e escreve testes; verificador distinto sela e
julga sem corrigir entradas. Árvore compartilhada não prova isolamento.

Preservar todos os artefatos P1307 como históricos imutáveis. Reutilizar o
oráculo público R6 sem alterar expectativas; extensão P1308 independente cobre
fronteiras adicionais e a migração estreita dos testes. Congelar L0 e testes,
obter RED semântico no baseline antes do código, depois GREEN, matriz focal,
workspace, build, fmt, lint e repetição/ordem inversa dos observáveis.
Executar mutantes Rust reais proporcionais às quatro correções, sem contar
mutação de envelope como mutação do produto. Uma falha desconhecida permanece
bloqueante; nenhuma normalização nova pode transformar falha em sucesso.

Budget de calibração: duas tentativas por mesma causa; focal antes de corpus
completo; nova execução global exige delta funcional ou gate final.
Temporários podem usar /dev/shm; registrar fontes, horários, comandos e hashes.

O resultado vai para `00_nucleo/diagnosticos/p1308-final-report.md`, separando
correções concluídas, falhas reais, limites e dívida anterior. Concluir P1308
não inventa certificado de toda a campanha P1307 nem substitui mutantes seus
que não tenham sido executados. Não escrever outro passo sem necessidade.
