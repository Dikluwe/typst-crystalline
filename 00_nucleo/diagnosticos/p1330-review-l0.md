# P1330 — gate independente do L0

Veredito: **L0 aprovado para o recorte P1330**. A aprovação não substitui
o congelamento A/B, a sucessão histórica, RED compilado nem os gates finais.
Revisor `/root/p1330_review`; regime A/B sem atestação de isolamento,
sem selo de refinamento. Somente este diagnóstico foi escrito pelo revisor.

## Entradas e ordem

Revisão em `2026-09-09T13:08:06.692Z` e conferência subsequente dos hashes,
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
L0 completo relido; a fonte do owner ainda é byte a byte igual a
`original_owner` em `p1330-baseline.json`. Portanto não havia C na fonte
no momento desta aprovação.

| Entrada | SHA-256 |
|---|---|
| L0 calc completo | cbebbece73abc68b416d00260cf952aaac2441c561d811e10b56c51f4d6a01bf |
| L0 calc sem linha Hash do Código | 10c00cbe8bfc815d77ab304c5b208205f6c6a316cd00624f2ff201b420299aa6 |
| owner calc baseline ainda vigente | 028017afa2f034409b437389469bea8782106a602affbea2531dd139d49d2d54 |
| p1330-baseline.json | a3f732bfb2fda2f177dbf3b33caddcd84b219af6fac22f96eae9644e784b6987 |
| p1330-baseline-public.json | c2f6500ed2f98fe2e37875b476fdb8570483ad516662cb804c4960c18a0781c2 |
| p1330-l0-preflight.json | 65443b4367f9d255474853328c0bef9b1fe5e90bdab6677d9d61baa19e7cb08f |
| p1330-l0-dry-run.json | 0532fcdc742f586ddaebd55f5405c3e4f73f94dc5acbd29dfc94d1f63089efcc |

Os recibos preflight/dry-run guardam HEAD, UTC e diff/stat exatos do estado
usado. Diferença do stat perante o baseline registrado na revisão preliminar:
somente calc.md passa a `247 +++++-`; o total registrado é
`14 files changed, 3240 insertions(+), 107 deletions(-)`.
Inventários produtivos before/after idênticos em cada recibo.

## Fundamentação

O novo texto precede a decisão com fonte e baseline reproduzível, classifica
a mudança no resultado/erro da linguagem e distingue observação de intenção
geral. As cláusulas anteriores de saturação foram substituídas em P1328,
P1329 e tabela de funções base, sem manter obrigações conflitantes para MIN.

O diagnóstico está completo: erro único Error, mensagem exata, hints e trace
nativos vazios, primeira ocorrência posicional value_span, ausência/detached
preservadas. A aridade e named continuam anteriores ao valor. A consequência
no trace externo está declarada, inclusive a dívida nominal calc.abs/abs.
Parsing do literal mínimo, NaN dimensional anterior à chamada, operadores,
entidades, dispatcher e outras funções permanecem fora do recorte.

A sucessão se limita à expectativa unitária MIN→MAX e observação CLI
histórica equivalente; os outros controles P1328/P1329 e os arquivos de
evidência ficam preservados. O texto requer comparações completas dos
diagnósticos e semântica tipo/valor, fronteiras e rotas, perfis e ordens.
Não transforma Unknown em sucesso nem afirma paridade geral.

ADR-0127 permite fluxo contínuo desta correção interna de paridade;
não há contrato público, mudança deliberada de default ou fase nova.
A inferência de suficiência do owner tem refutadores explícitos.

## Linhagem e próximos gates

Recibo recebido de `crystalline-lint --checks v15,v26 --fail-on warning .`:
exit 0, `No violations found`. Recibo recebido de
`crystalline-lint --fix-hashes --dry-run .`: exit 0 e somente o owner calc
proposto, `old=07f83fc2 hash-a=10c00cbe hash-b=483d55d0`.
Essas execuções foram auditadas por recibos, não repetidas por este revisor.
O resselo pré-código segue necessário; V5 ainda não é declarado fechado
pelo simples dry-run. Freeze A/B, sucessor e RED continuam pendentes de
revisão própria antes de C. Não foi lido nenhum passo/context histórico.
