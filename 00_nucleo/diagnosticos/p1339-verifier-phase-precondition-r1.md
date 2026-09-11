# P1339 — pré-condição circular de closed_state no contrato R1

Verificador `/root/p1311_review`, sob manifesto r2 SHA-256
`842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b`.
Contrato examinado: `p1339-contract.json`, SHA-256
`e062fb551fcffd78d21c1ae2a0a185aa376dd790c3e0825cec045a7f1e65b509`.
Estado e integridade medidos em `p1339-verifier-intake-r1.json`, SHA-256
`f0c4e210c7717a453294419370bc99d01f7c57ed4ee05daedb3cf0e291d361e9`.
Nenhum selo, execução completa de discriminação ou crédito de cobertura foi
emitido pelo verificador. O consumer produtivo continua o baseline com
headers de linhagem atualizados, conforme os hashes do freeze.

O contrato exige mapa de toda obrigação W01–W10 antes do selo. Isso é
coerente. Porém `adapters.closed_state` exige execução de testes de APIs reais
dos carriers/relação privada aprovados e termina com a regra de que a ausência
de capacidade de observar uma propriedade obrigatória bloqueia o selo.
Algumas dessas APIs só existirão após o candidato. Simultaneamente,
`gates.preimplementation` exige selo e RED anteriores à implementação.
Interpretar a frase como execução compilada prévia de toda API futura produz
uma pré-condição circular. Compilar mock dessa implementação futura não a
resolve; falha de build não é RED e não demonstra a propriedade.

Critério concreto para a revisão pelo autor do contrato, sem reduzir o escopo:

1. Congelar antes do selo o mapa integral de obrigações e seus predicados,
   fixtures, proveniência e fase de execução. Nenhuma obrigação privada pode
   desaparecer sob uma classificação genérica de “posterior”.
2. Distinguir explicitamente os testes executáveis pré-selo sobre a referência
   e carriers reais existentes dos gates finais que exigem APIs do candidato.
   Uma exigência final futura não recebe Preserved ou cobertura executada no
   recibo pré-selo.
3. A discriminação anterior ao candidato continua real: controles positivos,
   opacos, famílias semânticas e M20 em programas compilados; M12 usa fonte,
   resolução de chamadas e artefatos compilados distintos. Nenhuma expectativa
   inventada, mock JSON ou branch apenas compilado recebe crédito.
4. Após implementação, todas as obrigações privadas pendentes devem ser
   exercitadas em testes compilados do produto real e/ou no gate estrutural
   especificado de antemão. Falha, observação ausente ou Unknown obrigatório
   bloqueia o fechamento; não há dispensa retrospectiva.

Esta é uma solicitação de redesenho da ordem dos gates, não autorização para
alterar L0 ou expectativas semânticas. O autor mantém R1 imutável e produz
sucessor canônico com predecessor e motivo. O verificador audita a revisão;
não corrige o contrato que julga. A autorização existente cobre a continuação
do P1339 e não exige nova confirmação humana para essa correção protocolar.
