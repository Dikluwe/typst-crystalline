# P1328 — recibo final da autoridade A/B

Veredito: **PASS para o fragmento CLI congelado**, executado sem atestação
de isolamento. Auditoria independente em 2026-09-09T11:58:57Z, pela
autoridade `/root/p1328_tests`, sem leitura do runtime calc.rs, patch
candidato, inventários/diffs do root ou seus recibos de implementação.
Não constitui certificado de refinamento ou paridade geral de calc.

## Comparação auditada

A auditoria não confiou nos booleanos `candidate_matches_frozen_policy`.
Recalculou igualdade de cada triplo literal exit/stdout/stderr candidato
contra `p1328-ab-cli-expected-r1.json`; verificou as chaves únicas,
completude, classificações e expectativas incorporadas nos recibos.
Recomparou BASE e VANILLA com a medição independente anterior a C, e
comparou os candidatos entre execuções por chave. Conferiu argv completos,
os quatro perfis e a inversão real da ordem dos casos. Nenhuma saída foi
normalizada ou teve nomes, spans, espaços ou traces removidos.

Cada execução contém 28 casos × 4 perfis = 112 observações; as três somam
336 comparações candidatas literais aprovadas. Não houve chaves ausentes,
duplicadas, novas, alterações BASE/VANILLA, diferenças de repetição/ordem,
ou divergências em argv. Estes números pertencem precisamente aos recibos:

| Recibo | UTC inicial | SHA-256 |
| --- | --- | --- |
| `p1328-ab-cli-normal.json` | 2026-09-09T11:57:30.671863+00:00 | `280f6e22e8718b603a36c62636c4cf810bf06c80794f65432f08b1abfd3ba82e` |
| `p1328-ab-cli-repeat.json` | 2026-09-09T11:57:35.316089+00:00 | `68750febdc72247f41df142d20ea3bd1d2dbb7419090ac633a6dd7908f6cb103` |
| `p1328-ab-cli-reverse.json` | 2026-09-09T11:57:39.895314+00:00 | `20936e0112c39398bacd5306d8b72a5a75251e9c753829ced109263a97673778` |

HEAD informado uniformemente:
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
Os recibos completos registram argv e outputs. Esta autoridade não tem
permissão para ler o inventário de alterações produtivas; sua auditoria
identifica o código executado pelos hashes binários, recalculados dos
arquivos ao concluir. A proveniência fonte→binário e o diff/stat exato
continuam responsabilidade do coordenador/revisor com essa capacidade.

| Binário | Caminho | SHA-256 confirmado |
| --- | --- | --- |
| BASE | `/tmp/p1327-target.k9Mq0s/release/typst` | `75e8b97b3788c0feaf457cb4c06b1c2c6735cff5ef3b9804a75ec57f8b148e31` |
| VANILLA, upstream ratificado `a51e02804` | `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| CANDIDATE | `/tmp/p1328-target.T7Tg57/release/typst` | `94c3d8cec16dc98784757227f554b2851fad186a9e76605272bdaab0e6f925f9` |

## Classificação, por execução

| Classe congelada | Casos | Observações |
| --- | --- | --- |
| Correção com triplo integral vanilla | content-empty, content-markup, content-math-qualified, content-math-bare, content-alias, content-with-empty, content-spread-array, content-utf8-lines, content-distinct-origins, content-markup-route, warning-before-content | 44 |
| Correção com dívida de nome do trace preservada | content-with-bound, content-with-nested, content-spread-args | 12 |
| Paridade integral já existente | integer, float, decimal | 12 |
| Dívida/comportamento BASE preservado | named-before-arity, multiple, zero, string, symbol, length, angle, ratio, fraction, overflow, sqrt | 44 |

Nas três rotas com pré-binding/origem anterior à chamada, o diagnóstico
primário e o span correspondem ao vanilla, mas o trace externo conserva
`calc.abs`; o vanilla usa `abs`. Essa dívida foi identificada e congelada
antes de C em saídas sucessoras literais completas. Não se declara paridade
integral nessas rotas. Não houve compensação em tempo de comparação.

As demais dívidas preservam literalmente BASE: prioridade do guard named,
aridade, rejeições de tipos fora do recorte, saturating_abs do mínimo inteiro
e sqrt. A enumeração de tipos na nova mensagem não prova suporte a
Length/Angle/Ratio/Fr. `warning-before-content` confirma simultaneamente
o erro de conteúdo e o aviso P1327, inclusive ordem e apresentação completos.

## Integridade dos congelamentos

Pins recalculados e iguais aos freezes:

- Snippet R1: `fa971dded3237a1742905db2a45e3d94aef51a853fed17ca8046a0a8ba1cd353`.
- Snippet R2, sucessor somente de formatação edition 2021:
  `b689de73f3bbe572ed4cc9a0fea0fc29a792f91a8125cc8e1a657a19e676315e`.
- Runner: `326235f263b308a59cb3d50726270ad6c262afe8a3591f1b2c23498fc3b0875d`.
- Expectativas literais: `9d633ccf379d5a1124723ff1919f388b0603aab8cd4cd940456aa33144369623`.
- Baseline válido r1: `8f9ddaead847be551b7eb779fbf6719ece3043e84714d872b03c278c9e78db5c`.
- Freeze r1: `4eed4671d4649c0c71cd02632e3ecd55957b90059a5c4d0f3388d242b190542d`.
- Freeze r2: `6e1c5db370a44d1ba8f48bf07cbe0f0858cea65a4728e1db680affb9f12e4482`.

L0 vigente SHA-256
`deced74437190452be5af7ccf47d757cf7af569054f5b3b016782d48f8e45367`.
Restituir apenas o campo recíproco `Hash do Código: 4f58eb82` produz
exatamente o SHA-256 original
`2c7494c628038975cd52d6e093f66dc7e78a435d69effdc17ee6eb312213367e`;
portanto o restante da norma congelada está intacto. Essa verificação de
linhagem não é normalização de observáveis CLI.

## Capacidades e limites

Foram respeitadas as entradas públicas e test-only do freeze r1 e a
limitação de escrita aos diagnósticos A/B. Não houve leitura runtime antes
ou depois do freeze, nem edição dos oráculos após C. O workspace e as
ferramentas não impõem isolamento; isso impede alegação de atestação.

O coordenador informou GREEN dos testes congelados, cujos asserts incluem
diagnóstico nativo, Content/LocatedContent, origens sintéticas sem fallback,
math/markup, With/spread, UTF-8 e controles. Esta autoridade não recebeu os
recibos root de execução e não os atesta; seu veredito próprio acima é a
auditoria CLI e de integridade. Build, suíte geral, RED/GREEN executado e
crystalline-lint são gates do coordenador/revisor, não comprovados aqui.

O primeiro recibo sem sufixo r1 permanece excluído: foi uma tentativa
instrumental inválida por `--color` não suportado no vanilla. Nenhum Unknown
foi convertido em aprovação. Não se inferem mudanças de fase, cobertura
fora do corpus ou equivalência funcional geral a partir destes resultados.
