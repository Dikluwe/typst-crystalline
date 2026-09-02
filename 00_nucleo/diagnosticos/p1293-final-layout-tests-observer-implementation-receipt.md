# P1293 — recibo do implementador do observer final de `layout/tests.rs`

Data: 2026-09-02  
Papel: implementador test-only estrito  
Estado: **CANDIDATO FUNCIONAL E MECÂNICO GREEN; GATES SOLICITADOS GREEN, AGUARDANDO VERIFICAÇÃO INDEPENDENTE**. Este recibo não aprova o Lote B, não aprova P1293 e não constitui certificado final.

## Regime e segregação

- Regime: protocolo completo da skill `tekt-materializacao-segregada`, executado sobre manifesto e selo congelados.
- Papel exercido: somente implementador; não autor de contrato, adversário ou verificador final.
- Isolamento técnico de leitura atestado: `false`; filesystem compartilhado: `true`.
- `04_wiring/tests/p1292_contract.rs` e `04_wiring/tests/p1293_contract.rs` não foram abertos, inspecionados diretamente ou executados.
- Receipts/oráculos protegidos de P1292/P1293 não foram abertos ou executados.
- Nenhum L0, Núcleo Tekt, manifesto, selo, expectation, fixture, produto ou `equation.rs` foi alterado.

## Autoridade e inputs validados

- Manifesto `00_nucleo/diagnosticos/p1293-manifest.json`: `7d2062f06953336bc11ed0df7badc4434008225b8167c93b64e0970792450871`.
- Selo `00_nucleo/diagnosticos/p1293-contract-seal.json`: `e8e38e9e4c21dd8a50d16054f9622b468623effd143c54933893bc599dafc13d`.
- Bloco canónico `$.final_layout_tests_observer`, serializado por `json.dumps(..., sort_keys=True, separators=(',', ':'), ensure_ascii=False)`: `4bca721d0d256c0111e6ade4e6ab621d43df3ae449f1992aeb203dba840c6409`.
- L0 `00_nucleo/prompts/compiler/layout/tests.md`, lido integralmente: `35d0fa736f5cdbfaacff3645b559836149ce8ffdf6d0e76c988dbd5a90b36153`; header esperado `0d853401`; `Hash do Código` declarado `e19a69a6`.
- Consumer preimage `01_core/src/compiler/layout/tests.rs`: `654a7fcec47fe20bb1f5b67884f11961756f60c5cf0b183dfd37efdc5ddfb3fd`, exatamente o valor completo do selo.
- Produto congelado `01_core/src/compiler/layout/equation.rs`: `ea81dc9e291d29e1ac6d17a240b98929228af31dc2a2a1563efae20ba25d9e23`, reconfirmado após o ensaio.
- Não foi identificado gap de cardinalidade L0↔consumer; o bloqueio abaixo é de capacidade/allowlist causal do selo.

## RED independente reproduzido

Comando, antes de qualquer escrita:

```text
cargo test -p typst-core --lib
```

Resultado exato: `running 5416 tests`; `5403 passed; 13 failed; 0 ignored; 0 measured; 0 filtered out`; exit `101`; fase de testes `3.72s` após build de `45.51s`.

Os mesmos treze REDs foram reproduzidos:

1. `p813_equacao_bloco::equacao_bloco_centrada_horizontalmente`
2. `p813_equacao_bloco::equacao_bloco_espacamento_vertical_1_2em`
3. `p813_equacao_bloco::equacao_bloco_no_topo_do_documento_sem_spacing_acima`
4. `p813_equacao_bloco::equacao_inline_sem_alteracao_de_baseline`
5. `p813_equacao_bloco::equacao_numerada_centragem_nao_quebra_numero`
6. `p813_equacao_bloco::p952_equacao_equacao_spacing_inclui_descent_da_anterior`
7. `p896_equacoes_de_bloco_centram_contra_a_largura_final_da_pagina`
8. `p987_tests::p987_numero_acompanha_conteudo_em_pagina_auto`
9. `p997_tests::p1088_heading_para_equacao_bloco_generic_margin_collapse`
10. `tests_inline_baseline::equacao_inline_baseline_coincide_com_texto`
11. `tests_inline_baseline::if_com_math_inline_attach_baseline_coincide`
12. `tests_limits::sum_block_limites_empilhados_verticalmente`
13. `tests_limits_context::sum_inline_usa_right_scripts`

Proveniência do RED: HEAD `7dd25ff0e222b6c7c640d6bc7957b98f94227507`, working tree compartilhado não commitado; snapshot em `2026-09-02T19:03:15-03:00`; SHA-256 de `git status --porcelain=v1` = `52e042b681f01a58bbb259aa57bc6029172ae3942c3c724247049c0c5ee7cedd`; SHA-256 de `git diff HEAD --stat` = `5e1453472071c61e8150bca0cb5811f975dddb1bfc806ea30d9f4c7ce9f0a5ed`.

## Candidato test-only materializado dentro da allowlist

Somente `frame_items_recursive`, auxiliares privados locais e um teste próprio no mesmo ficheiro foram alterados:

- o walker transporta `TransformMatrix` ancestral;
- ao entrar em `Group`, compõe `ancestral.concat(translate(Group.pos)).concat(Group.matrix)`;
- `Text`, `TextShaped`, `Glyph`, `Image`, `Shape`, `Group`, `Link`, `Line.start` e `Line.end` são projetados por `TransformMatrix::apply`;
- `Semantic` e filhos de `Link` conservam o transform ancestral, sem deslocamento inventado;
- a API privada histórica `Vec<&FrameItem>` foi preservada por cópias projetadas limitadas ao processo de teste;
- foi adicionado `p1293_final_frame_items_recursive_compoe_grupos_aninhados`, com dois `Group`, posições não zero e matriz de escala/translação.

O teste próprio ficou GREEN: `1 passed; 0 failed; 5416 filtered out`, exit `0`.

Hash do candidato parcial `01_core/src/compiler/layout/tests.rs`: `2a9c5347660e10f56fb2104fc9c53b571e8e51813bd72c400654ecceebf1207e`.

Nenhuma expectation existente foi editada. O diff focal contém apenas o header de linhagem que já fazia parte do preimage selado, a implementação/auxiliares autorizados e o teste próprio autorizado.

## GREEN incompleto e descoberta bloqueante

O core completo foi repetido após o candidato:

```text
cargo test -p typst-core --lib
```

Resultado: `running 5417 tests`; `5416 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out`; exit `101`; `3.69s`.

Doze dos treze REDs históricos ficaram GREEN. Permaneceu somente:

```text
compiler::layout::tests::p997_tests::p1088_heading_para_equacao_bloco_generic_margin_collapse
gap H1->Eq simples (mock): 73.9461pt vs esperado 20.9000pt
```

A inspeção read-only do próprio consumer mostrou a causa precisa:

- P1088 usa `p997_tests::text_items` em `01_core/src/compiler/layout/tests.rs:22021`, não usa `frame_items_recursive`;
- esse helper contém um segundo walker recursivo privado, em `:22022-22038`, que atravessa `Group` descartando `Group.pos` e `Group.matrix`;
- portanto a afirmação causal do selo de que os treze witnesses fechariam somente pela escrita autorizada em `frame_items_recursive`/auxiliares é incompleta para P1088;
- corrigir `p997_tests::text_items` exigiria escrever num helper independente que não é auxiliar de `frame_items_recursive` nem teste próprio novo, fora da allowlist textual exata do selo.

Isto não demonstra divergência produtiva: demonstra que o décimo terceiro witness possui outro observer local com o mesmo defeito de transporte. Ainda assim, `Unknown`/falha não pode ser convertido em sucesso, e alargar a allowlist por inferência invalidaria a segregação.

## Stop condition aplicada

Após a falha residual, nenhuma nova correção foi tentada. Em particular:

- `p997_tests::text_items` não foi alterado;
- nenhuma expectation/tolerância foi adaptada;
- nenhum ficheiro produtivo, L0, selo, manifesto ou oráculo foi tocado;
- os gates arquiteturais finais e o `cargo fmt --all -- --check` não foram usados para alegar fecho, porque o pré-requisito core completo GREEN falhou;
- `rustfmt --check` focal identificou somente a quebra mecânica da assinatura de `project_item`; ela não foi escrita após a stop condition;
- `git diff --check` permaneceu PASS.

## Próxima ação necessária

O autor do contrato/selo precisa decidir e, se confirmar a intenção já expressa no L0 para “todo helper recursivo deste consumer”, emitir nova autoridade que inclua explicitamente `p997_tests::text_items` (ou autorize sua delegação a `frame_items_recursive`) e fixe o novo preimage. Um novo implementador poderá então completar o RED→GREEN e executar os gates finais.

Este implementador para sem aprovar Lote B/P1293 e sem emitir certificado.

---

## Continuação residual P1088 sob replacement seal

Esta secção substitui somente a conclusão operacional anterior. O histórico acima permanece como proveniência do candidato que se tornou o preimage da continuação.

### Nova autoridade validada

- Manifesto `00_nucleo/diagnosticos/p1293-manifest.json`: `4c63d5b33280802b711b2aa0e4f9e767e7ea5b4d8263959be3b67bdb57b36b23`.
- Selo `00_nucleo/diagnosticos/p1293-contract-seal.json`: `5cc81fac6d0d01f2add818077625ea8c1c07945aa1cbd2dad02bc24cdacbbede`.
- Bloco canónico `$.final_layout_tests_p997_observer`, serializado por `json.dumps(..., sort_keys=True, separators=(',', ':'), ensure_ascii=False)`: `645e0b3fc770433b360dc2f45241ba7ed1ec54e092d629fddd16c506c96992fc`.
- L0 `00_nucleo/prompts/compiler/layout/tests.md`, lido integralmente e inalterado: `35d0fa736f5cdbfaacff3645b559836149ce8ffdf6d0e76c988dbd5a90b36153`.
- Novo preimage congelado de `01_core/src/compiler/layout/tests.rs`: `2a9c5347660e10f56fb2104fc9c53b571e8e51813bd72c400654ecceebf1207e`.
- Receipt anterior congelado como input pelo replacement seal: `07500b54d47bf54f7c1aecc19521ca1efc32e75b9b0f8762375ace10ff0e333b`.
- Produto congelado `01_core/src/compiler/layout/equation.rs`, reconfirmado sem alteração: `ea81dc9e291d29e1ac6d17a240b98929228af31dc2a2a1563efae20ba25d9e23`.

### RED residual reproduzido

Antes da escrita autorizada, `cargo test -p typst-core --lib` produziu exatamente `running 5417 tests`; `5416 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out`; exit `101`; fase de testes `3.40s`.

O único RED foi `compiler::layout::tests::p997_tests::p1088_heading_para_equacao_bloco_generic_margin_collapse`, em `01_core/src/compiler/layout/tests.rs:22266`, com `73.9461pt vs esperado 20.9000pt`. Nenhum outro RED ou owner gap apareceu.

Não foi acrescentado outro teste: o próprio P1088 discrimina o segundo walker defeituoso, enquanto `p1293_final_frame_items_recursive_compoe_grupos_aninhados`, já congelado no preimage, cobre composição aninhada com posições não zero.

### Escrita autorizada e GREEN

Somente `p997_tests::text_items` foi alterado. O walker local que descartava `Group.pos` e `Group.matrix` foi removido e o helper passou a delegar, por referência, a `frame_items_recursive`, já transform-aware. A filtragem final continua aceitando apenas `Text` e `TextShaped`, preservando a forma histórica do retorno `(x, y, texto)`.

Nenhuma expectation, tolerância, fixture, L0, manifesto, selo, produto, `equation.rs` ou primeiro helper foi alterado nesta continuação.

Resultados:

- focal P1088: `1 passed; 0 failed; 5416 filtered out`; exit `0`; teste `0.01s`, após build `16.69s`;
- `cargo test -p typst-core --lib`: `running 5417 tests`; `5417 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`; exit `0`; testes `3.51s`.

Hash do candidato funcional `01_core/src/compiler/layout/tests.rs`: `b8086886c7ea7298153b626c1f36f9280d710485ef7e257da4304d688ca797e0`.

### Bloqueio literal de formatação

Os dois checks de formatação foram executados em modo somente leitura:

```text
rustfmt --edition 2021 --config skip_children=true --check 01_core/src/compiler/layout/tests.rs
cargo fmt --all -- --check
```

Ambos terminaram com exit `1` e apontaram uma única diferença: a assinatura preexistente de `project_item` deve ser quebrada em múltiplas linhas. Essa assinatura pertence ao primeiro helper/auxiliar congelado byte a byte no preimage `2a9c5347...f1207e`; o replacement seal só autoriza escrita em `p997_tests::text_items`/walker local ou delegação ao helper transform-aware e proíbe alterar o primeiro helper.

Aplicar `rustfmt` nessa linha violaria a allowlist; não aplicar impede declarar os gates de formato GREEN. A stop condition foi, portanto, acionada sem inferir autoridade adicional. V3/V4/V5/V7/V13/V14/V15/V26 e dry-run não foram executados depois deste bloqueio e não são alegados como PASS. `git diff --check` permaneceu PASS.

### Proveniência final desta continuação

- HEAD: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`.
- Working tree compartilhado não commitado.
- Snapshot funcional/formatação: `2026-09-02T19:15:22-03:00`.
- SHA-256 de `git status --porcelain=v1`: `01ace9ce733479b68e5cd9801cc8075ef16a0f83191ce7516f3b19e14658385d`.
- SHA-256 de `git diff HEAD --stat`: `987b2a44a011a289fe357cbc93cbc2fe63fad2d2507e1e1d9d34188d1e7bd19b`.
- Os hashes de status/stat foram recolhidos antes da autoatualização deste receipt; os ficheiros materiais do candidato já estavam no estado descrito.
- `04_wiring/tests/p1292_contract.rs`, `04_wiring/tests/p1293_contract.rs` e receipts/oráculos protegidos não foram abertos, inspecionados diretamente ou executados.

### Próxima autoridade necessária

O autor do replacement seal deve autorizar explicitamente a única normalização mecânica de `rustfmt` na assinatura congelada de `project_item`, ou adotar como novo preimage a forma normalizada. Só então um implementador pode executar os gates restantes sem violar a allowlist.

Este implementador entrega o candidato funcional GREEN e para no conflito de autoridade, sem aprovar P1293 e sem emitir veredito final.

---

## Fecho mecânico sob selo final de `rustfmt`

Esta secção supera somente o bloqueio de formatação descrito na continuação anterior. Toda a evidência histórica permanece preservada.

### Autoridade e preimages validados

- Manifesto: `5abf0071c2cd83f7373559b02967fa1398d55eb2b2572c64514ade2780d5fcff`.
- Selo: `416a9ad5fc711468d0d6e45f7bec252b23cabcdaf110a86f039e116ec77d7113`.
- Bloco canónico `$.final_layout_tests_observer_rustfmt`, serializado por `json.dumps(..., sort_keys=True, separators=(',', ':'), ensure_ascii=False)`: `9f73378a85c6e71e732255ff45f5dfa3a3751eff1ffbb2c128daf081a84bf7d8`.
- Consumer preimage `01_core/src/compiler/layout/tests.rs`: `b8086886c7ea7298153b626c1f36f9280d710485ef7e257da4304d688ca797e0`.
- Receipt preimage: `fa471fa8631c9f7dd8b80f34eb068bb8c7aa25809619ce320f36e4322579aa8b`.
- L0 inalterado: `35d0fa736f5cdbfaacff3645b559836149ce8ffdf6d0e76c988dbd5a90b36153`.
- Produto `equation.rs` inalterado: `ea81dc9e291d29e1ac6d17a240b98929228af31dc2a2a1563efae20ba25d9e23`.

O regime continua a ser protocolo completo com papel estrito de implementador. O filesystem compartilhado impede atestação técnica de isolamento; não foi exercido papel de verificador ou aprovador final.

### Delta mecânico aplicado

Antes da escrita, `rustfmt --edition 2021 --config skip_children=true --check 01_core/src/compiler/layout/tests.rs` exibiu exatamente o único hunk selado: quebra da assinatura privada `project_item` de uma linha para três linhas. Nenhum token semântico, corpo, expressão, expectativa ou tolerância mudou.

Foi aplicado exclusivamente esse wrap de newline/indentação. O delta unificado pinado pelo selo é `3a9f81402a1640a4469076b6cb6e93ff9621ecce4aca16dd2b566ec3326b23a1`.

Postimage obrigatório e observado de `01_core/src/compiler/layout/tests.rs`: `020020cd6d749a4b00949587efeb2a1e37b1c9a52d03628fe9b9b4c96753cf81`.

### Gates executados

- P1088 focal: `1 passed; 0 failed; 5416 filtered out`; exit `0`; teste `0.01s`, após build `12.56s`.
- `cargo test -p typst-core --lib`: `5417 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`; exit `0`; `3.44s`.
- `cargo fmt --all -- --check`: PASS, exit `0`, sem hunks.
- `crystalline-lint --checks v3,v4,v5,v7,v13,v14,v15,v26 .`: PASS, exit `0`, `No violations found`.
- `crystalline-lint --checks v5 --fix-hashes --dry-run .`: PASS, exit `0`, `Nothing to fix`.
- `git diff --check`: PASS, exit `0`.

Não houve outro RED, `Unknown`, owner gap ou necessidade de escrita fora da allowlist.

### Proveniência do fecho

- Instante: `2026-09-02T19:25:12-03:00`.
- HEAD: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`.
- Working tree compartilhado não commitado.
- SHA-256 de `git status --porcelain=v1`: `01ace9ce733479b68e5cd9801cc8075ef16a0f83191ce7516f3b19e14658385d`.
- SHA-256 de `git diff HEAD --stat`: `e3d170410960d31d1b9fb5aff31823c58f6e3623ff9b8fc686e1a8abb304f0db`.
- SHA-256 de `git diff -- 01_core/src/compiler/layout/tests.rs`: `e2716629340471b67dcb3f3021d36be7c8bd152a41b7d36f62312a1776c1a3b9`.
- Os hashes de status/stat foram recolhidos antes da autoatualização deste receipt; o receipt é untracked e não altera esses dois observáveis.
- Manifesto e selo foram reconfirmados após os gates nos hashes acima.
- `04_wiring/tests/p1292_contract.rs`, `04_wiring/tests/p1293_contract.rs` e receipts/oráculos protegidos não foram abertos, inspecionados diretamente ou executados.

O candidato test-only encerra os gates solicitados em GREEN e é devolvido para verificação independente. Este implementador para sem aprovar o Lote B/P1293 e sem emitir certificado final.
