# Estado do objetivo geral — corrigir as divergências

Este registro mantém o escopo do pedido “Faça todas as correções”. Fechar um
lote não fecha o objetivo. A referência de requisitos continua sendo o
[inventário contra o pin atual](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/paridade-novo-vanilla-2026-10-17/inventario.md),
evidências E01–E20, mais os erros arquiteturais e os refutadores encontrados
nos lotes seguintes. Não há evidência de conclusão global.

As correções A1, pilha, R1 e R2 têm relatórios e freezes próprios. São provas
limitadas daqueles estados; a aceitação global futura exigirá repetir os
corpora pertinentes no mesmo freeze final. Nesta etapa, R3 corrige filhos
imediatos de ContentBlock, slots textuais consumidos e None em markup. A
validação geométrica adicional encontrou um reparo necessário no certificado
de prefixo Strong/Sequence/None antes de 1fr. Os gates finais deste lote passaram; o relatório registra os limites e as pendências restantes.

| Frente do inventário | Estado que ainda impede conclusão global |
|---|---|
| A1 / E02–E04 | Revalidar os reparos de callbacks/getters/seletores no freeze final; reflexão de Styled/Styles, Symbol e demais presenças ainda não é universal. |
| A2 / E05 | Bibliografia: cache CSL sob carriers/receitas e hanging-indent ainda precisam de correção e contraprovas. |
| A3 / E11–E13 | Overflow, include/valor de módulo, igualdade de supplement e identidade std continuam a medir/corrigir. |
| A4 / E09–E10/E16 | Sub/super, altura/estilo de Heading, CJK local, precedência/revogação textual e stretch/argumentos matemáticos continuam. As sondas R3 ampliaram os refutadores de texto. |
| A5 / E17/E19/E20 | Export multipágina/template, coerções HTML e diagnósticos/rastros ainda exigem os reparos correspondentes. |
| B1 / E01 | Transportes relativos RM1: revisar proposta/L0 e obter os gates públicos aplicáveis antes do código. |
| B2 / E07 | RG1/RG2/RG3/Table: revisar propostas e aprovação vigentes; spans, linhas, metadados, repetição e propriedades ricas não estão universalmente fechados. |
| B3 / E08/E14/E15 | Paint textual, Figure tipada e Symbol preservado precisam dos contratos completos e gates próprios. R2 não os autorizou. |
| B4 / E06 | Document format/string/auto/reflexão e fronteira construção/export continuam. |
| C1 / E18 | Junções PDF de tiling, subpixel e alpha variável precisam de controle analítico. Conservar a composição Tekt correta onde já comprovada. |
| Arquitetura | O baseline de R2 tem 106 V16. Conferir o diferencial de R3; fechar todas as violações exige auditoria por owner, sem supressões ou mudança de critérios. |
| Escopo adiado | Imagens SVG/PDF, MathML, padding negativo e demais lacunas mencionadas no inventário não viram “vantagens Tekt” por estarem adiadas. Rever os contratos e medir antes de decidir. |

Pendências R3 confirmadas pelas sondas atuais:

- **Textual:** literal/regex deve escolher posição e precedência corretas; há
  saída constante/alias que não recebe regra restante, saída funcional mostrada
  duas vezes, cruzamento indevido de regiões com guardas e show-set literal
  rejeitado. Inverter somente uma lista de regras é insuficiente.
- **TextualGroups:** com múltiplas caudas, pacotes sintéticos ainda aparecem como
  um filho Sequence no lugar de seus filhos imediatos. A máscara de consumo
  resolve os slots Empty consumidos, não esse agrupamento nem a igualdade dele.
- **Reflection:** repr/campos de Styled/Styles e labels permanecem divergentes.
- **Symbol:** o cast continua produzindo Text e igualdade indevida com texto;
  uma variante pública exige proposta e aprovação próprias.
- **Diagnostics:** rastros de show e mensagens de seletores compostos permanecem.

Os resultados novos estão em before/markup-after/transport-after-main,
textual-before/textual-after-transport, transport-before/transport-after e
markup-geometry-before, todos com fontes, SHA dos binários, HEAD/diffstat e UTC.
As diferenças abertas são defeitos/dívidas a corrigir, não expectativas de
paridade copiadas do candidato. As exceções Tekt deliberadas continuam exigindo
comportamento e fonte próprios, conforme o inventário e o relatório de R2.
