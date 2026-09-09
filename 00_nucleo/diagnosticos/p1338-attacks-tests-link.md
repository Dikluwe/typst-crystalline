# P1338 — conferência privada de discriminadores antes de C

Após receber o freeze independente, li o módulo integral
`p1338-tests-module.rs.txt`, SHA-256
`2f19227ce32aab7066bca98f5200559c0e17d6e631593e37f054bc618fbfd8c4`,
e `p1338-tests-freeze.json`, SHA-256
`bd9f6a580251ba5dce783770657165354ab427736ceb4d2a4b87506958be04e3`.
Todos os seis testes novos estão sob `mod p1338_tests`, filtro já congelado
do runner adversarial. Não é necessário mudar testes, famílias ou filtro.

| Família congelada | Testemunha realmente selecionada |
|---|---|
| M1 mensagem Array errada | p1338_pure_missing_array_fields_message_and_received_span e p1338_ast_empty_heterogeneous_alias_unicode_multiline exigem texto exato |
| M2 span Array total | p1338_ast_empty_heterogeneous_alias_unicode_multiline compara byte range somente do field |
| M3 first puro trocado por last | p1338_pure_preserve_len_first_last_and_empty_none compara first em arrays com primeiro/último diferentes, sem pré-despacho |
| M4 extrapolar Length field-only | p1338_preserve_other_categories_diagnostics exige âncora integral de `(1pt).nope` |

Os controles de AST first/last vazios e métodos/chamadas reais também ficam
selecionados. O teste sucessor anterior fora do módulo é coberto pelos gates
do operador, não é contado como executado pelo filtro adversarial estreito.

Inferência verificada estruturalmente: cada família tem assertion capaz de
discriminar sua alteração; apenas execução posterior de C e mutantes comprova
essa capacidade. Nenhum kill é antecipado. Não houve edição de teste/oráculo,
leitura de C, geração de patches candidatos ou Cargo adversarial.

Plano, oráculos, perfil e runner originais permanecem intactos. O módulo de C
será conferido contra este arquivo congelado, ignorando apenas separadores
externos; em todas mutações o trecho de testes extraído será byte-idêntico.
Reviewer julga esta ligação e o RED antes do GO de C.
