# P1338 — sequência real de integração antes de C

Entre o manifesto R0 (`2026-09-09T20:09:20.180288+00:00`) e a correção R1
(`2026-09-09T20:12:27.603928+00:00`), o autor integrou provisoriamente o
módulo novo e a migração autorizada. A instrução de pausar a integração chegou
depois dessa escrita. O autor retirou exclusivamente suas alterações; a fonte
foi medida novamente com SHA-256
`639abd73c7392ed65088d8a77ce3c39a3523a2ca6f33a7945cd9757f5be0877d`,
igual ao baseline R0. Não se afirma inexistência desse intervalo provisório;
as chamadas não registraram UTC próprio mais preciso que essa janela causal.

O operador confirmou que sua assertiva source == baseline R0 passou antes de
aplicar R1. A fonte R1 ficou
`34c0a3bba193092f6976d3f7ac0951fd7bb3a178c2b754453dbf2ecfdf2a115c`.
Após o pin R1, os testes foram reintegrados mantendo esse header causal.
Nunca houve implementação C nem restauração de header R1 para R0.

O primeiro check de formatação provisório encontrou somente a chamada legada
migrada, cujo novo comprimento requer uma linha. Após a reintegração R1,
essa formatação mecânica foi aplicada dentro da migração e o check completo
passou antes do freeze. O módulo novo já havia sido formatado isoladamente.
Uma tentativa de patch com hunks fora de ordem foi rejeitada integralmente;
nenhuma escrita parcial ocorreu. A prova final prefreeze compara a fonte
inteira com baseline R1 + módulo novo + migração restrita.

Nenhum teste funcional Cargo foi executado pelo autor nesta sequência.
