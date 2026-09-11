# P1339 — autorização de Array a partir de Bytes

Em 2026-09-10, após `p1339-verifier-array-prerequisite-gap-r1.json`, o dono
respondeu “Pode aplicar e concertar os testes” à pergunta explícita sobre
ampliar o L0 para `array(bytes)` e revalidar mantendo os testes originais.

A autorização abrange essa conversão e correções dos adaptadores de teste.
Não inclui Array a partir de Version/Array, construtor geral, alteração de
expectativas para acomodar candidato, commit ou fechamento antecipado.
Permite owner dedicado, exportações internas e ligação por Type::Array.

O selo R3 permanece evidência histórica. Exigir sucessão normativa explícita,
discriminação, RED e validação. Autor de contrato e verificador mantêm suas
autoridades; custos e resultados anteriores não são apagados.

Medição preparatória: `p1339-array-prerequisite-measure-r2.json`. A tentativa
anterior usou `--diagnostic-format short`, não aceito pelo CLI baseline:
seus doze processos baseline pararam no parser, sem avaliar Typst. A saída
de ferramenta foi truncada, sem recibo completo. A repetição r2 retirou
somente essa opção e registrou expressões, canais, hashes, HEAD/árvore/UTC.
Medição do implementador não recebe crédito discriminatório independente.
