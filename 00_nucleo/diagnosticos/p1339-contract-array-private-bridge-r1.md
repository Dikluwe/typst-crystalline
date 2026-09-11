# Ponte privada prospectiva — sem crédito de execução

Autor: `/root/p1312_tests`, mesma autoridade combinada contrato/oráculo já declarada. Este documento não altera as 28 fontes públicas nem seus resultados históricos, não é implementação e não encerra a dimensão pendente.

Medição precede decisão: o raw focal `e969bffc9da5f796498fdd5bf635eaea468efac7cb67c4d0f5d04810c4a71196` mostra E08 alcançando o constructor Array, mas não revela o carrier. O verificador informou, após auditoria do baseline HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, que `collections.rs:2435` conserva `occurrence.span` em `arguments_map` e detacha somente `value_span`; `collections.md:485` determina essa distinção. Esta informação é atribuição da auditoria do verificador, não leitura produtiva deste autor. Sobra usa occurrence.span, logo E08 não constrói sobra detached. O seu Unknown histórico permanece intacto; nenhum expected vanilla é promovido por conveniência.

Decisão estreita: o adversário pode construir uma fixture executável independente, por API legítima de `Args` (`Args::from_parts`, conforme ABI verificada no baseline), e congelar seu código/expectativas antes do owner produtivo. O verificador audita a ligação efetiva ao owner, sem escrever algoritmo de controle. Estes vetores são insumos semânticos, não um placeholder contado como teste executado.

Vetores obrigatórios com SourceId legítimo compartilhado; offset/ranges deliberadamente disjuntos:

- `ARR-U01-detached-positional`: Args.span = 100..140; primeiro positional Bytes `(0, 255)`; única sobra positional Int `9`, occurrence.span detached, value_span conhecido 122..123. Esperado erro exatamente `unexpected argument`, span agregado 100..140. Usar value_span ou permanecer detached é incorreto.
- `ARR-U02-known-positional`: mesmo input, mas occurrence.span conhecido 120..125 e value_span 122..123. Esperado mesmo erro, span 120..125; fallback agregado incorreto.
- `ARR-U03-detached-named`: mesma primeira posição Bytes; única sobra named `other: 9`, occurrence.span detached, value_span conhecido 122..123. Esperado `unexpected argument: other`, span 100..140.
- `ARR-U04-known-named`: mesma sobra named, occurrence.span 118..130, value_span 127..128. Esperado erro named e span 118..130, nunca apenas valor nem agregado.

Os testes devem criar a ordem e categorias efetivas de Args sem lookup de ID de fixture no produto. Não basta construir spans e afirmar sua igualdade sem chamar o owner Array legítimo. A contraparte positiva aceita Bytes sem sobras e retorna Array de Int, para distinguir owner realmente alcançado de falha anterior. O controle real e o candidato devem executar esses mesmos vetores congelados; o negativo de origem deve ser compilado e rejeitado causalmente por eles. O autor/adversário deve resolver a ABI exata e publicar fonte executável pinada; este documento não fabrica uma assinatura Rust.

Custos permanecem no mesmo orçamento: 1 de 2 focais Array já gasto, 56 processos CLI executados, 0 expansões. A ponte não abre novo orçamento nem dispensa as famílias de negativos. A instrução de implementar a fixture não é autorização para alterar L0, Args.map, as 28 fontes ou os 70 oráculos originais. Nenhum selo ou PASS final é emitido aqui.
