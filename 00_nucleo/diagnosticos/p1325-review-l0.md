# P1325 — revisão ex ante do L0

Veredito `PASS_L0_SCOPE`, restrito à legitimidade do recorte. Regime A/B sem
atestação técnica de isolamento, sem selo. Revisor `/root/p1325_review`,
somente leitura de artefatos julgados, escrita restrita a `p1325-review-*`.
Nenhum código candidato existia no instante da auditoria.

Recibo reproduzível: `p1325-review-l0-audit.json`, SHA-256
`6ce147a5b0c6f12059506ad722d87658070126e93f0d35eb8c8a1c880b49f100`,
produzido por `node 00_nucleo/diagnosticos/p1325-review-audit.cjs l0` em
`2026-09-09T00:52:41.272Z`. HEAD e diff/stat integral estão no recibo.
Baseline conferido:
`e2ed7ae9f8290ee9557b38d151ffffebae760b79fa1abb64979d6c1eb9d7998b`.
Manifesto conferido:
`b4057fe02a65105654a2379be04b826179ce4ee2573e69a4d1f347c089347dce`.
L0 normativo conferido:
`2648ff575db76ac3a98832a211bb24d65c1bdfac6d4510a96bc85e6bf36b1b35`.
Fonte inteira permanece idêntica a `baseline.original_owner`; todos os
demais arquivos produtivos protegidos permanecem idênticos ao inventário.

## Medição e julgamento

As saídas do baseline confirmam que `(x: 1).nope`, `[x].nope`,
`strong[x].absent`, `(1.0).nope` e aliases diferem pela âncora. Lookup de
chave presente e `strong[x].body` têm saída bilateral coincidente. O caso
`[x].text` tem erro cristalino e sucesso vanilla, portanto não pertence à
classe de paridade integral de saída: o L0 o individualiza antes do patch e
autoriza somente mudar sua âncora, preservando a dívida de disponibilidade.

A seleção de span em `field_access.rs:502-510` precede a delegação ao lookup
puro; Dict e Content consomem esse span em `:518-547`, e Float produz o erro
genérico com o mesmo carrier. Os três variants existentes e o span do field
já estão disponíveis. Não é necessário novo carrier, API, entidade, fase ou
owner para esse fragmento. A classificação contínua ADR-0127 está sustentada.

O L0 substitui expressamente a proteção antiga de Dict e de targets não
Module em P1301/P1303/P1306/P1311/P1324, somente para Dict, Content raw e
Float na delegação direta. Isso evita que a correção viole silenciosamente
uma obrigação preservada. LocatedContent, erro de callee e pré-despacho
ficam explicitamente fora. A fonte `:439-449` devolve os resultados de
pré-despacho antes da seleção; `field_callee_error` calcula sua âncora própria.
Portanto, método e leitura de campo não devem ser fundidos na implementação
nem no oráculo. Float/is-nan já é field-only e deve continuar assim.

## Gates ainda abertos

Resselo ainda pendente no instante da auditoria: o header observado é
`740a39f5`, o hash efetivo esperado é `6f7aad92`; o hash recíproco de código
`c4567498` confere. O pin do Núcleo confere. Isto é o estado ex ante esperado
depois de editar L0, não uma linhagem final aprovada. O revisor não executou
reparo nem alterou o material julgado.

É necessário congelar os testes independentes antes do candidato, verificar
RED real das três categorias e seus limites, e conferir candidato, resselo e
gates finais. Esta aprovação de escopo não antecipa esses resultados, não
aprova math.join e não transforma residuais/Unknown em paridade.
