# P1339 — revisão focal independente do contrato r3

Verificador `/root/p1311_review`, 2026-09-10 03:23:43 UTC. Regime completo,
executado sem atestação de isolamento. HEAD e working tree de referência
permanecem os registrados em `p1339-verifier-intake-r1.json`; nenhum candidato
funcional foi fornecido. Zero matrizes completas pré-selo pelo verificador.

Contrato `p1339-contract-r3.json`: SHA-256
`c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17`.
Desenho r3: `854942ed6ef360af29fc021a0919e1300feb09e826ee5f4e9e8192488c0ed918`.
Recibo autor r3: `4316321665abb5eee0f41f8846ffa951009b0724c468b3b8c69878dd4d5fc51f`.

## Medição e leitura

Verificados 53 inputs de `input_sha256` e os 31 L0 brutos, sem divergências.
Comparação estrutural com r2 confirma rotas, obrigações, W01–W10, 539 IDs,
vinte famílias negativas, budget, hashes L0, perfis, ordens, scope-out,
preservação global e F01–F10 inalterados. Desenho e delta r2→r3 lidos
integralmente. Adições dos 31 owners L0 também lidas substantivamente.

Os treze programas de `p1339-ab-w02-derived-inputs-r1.json` foram lidos.
Suas alterações bare→where são expressamente novos testes, não transporte
do mesmo grafo. Seus resultados frescos ainda precisam tornar-se predicados
canônicos nos quatro perfis; nenhuma extrapolação do query default é feita.

## Juízo focal

Aceita a correção de fronteira causal: ancestors que falham primeiro na rota
bare protegida conservam o baseline integral, enquanto fixtures separados
exercem consumidores where com produtores admitidos. O caso `unrelated_error`
é distinto: a construção where autorizada deve passar antes do panic legado;
preservar o erro de ausência inicial seria congelar comportamento a corrigir.
A cobertura dessas classes é conjuntiva e ainda será auditada nos oráculos.

O contrato fecha a semântica das portas test-only F06–F09, sem prescrever
nomes privados Rust. Bindings futuros são candidato, não oráculo: somente
delegação e projeção sem perda da operação/transição realmente tomada. Exige
templates concretos, DTOs, cenários e assertions antes do selo. Uma lista de
slots ou de variantes não cumpre esses requisitos por si mesma.

Há label residual `Independent r2 seal valid` em `gates.preimplementation[0]`.
As cláusulas específicas r3 `gates.preseal[-1]` e
`authority_successor_resolution` exigem explicitamente pin do r3 com r1/r2
imutáveis. A interpretação estrita desta auditoria é somente selo que pinar
r3 e ambos os predecessores; nenhum selo baseado apenas em r2 é aceito.
Este apontamento não modifica contrato, expected, budget ou gate.

**Sem selo e sem GO de implementação.** Estão pendentes cobertura concreta,
resolução do inventário, templates/harness completos, registry negativo e
discriminação real. Três revisões contratuais publicadas consomem o budget;
nova insuficiência substantiva exige redesenho explícito, não quarta revisão
oculta. R1/r2 e as revisões focais anteriores permanecem históricas intactas.
