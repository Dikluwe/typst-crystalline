# P1293 — recibo de implementação das correções finais

Data: 2026-09-02  
Papel: implementador estrito das correções finais  
Estado: implementação e verificações locais concluídas; este recibo não aprova o Lote B, não aprova P1293 e não constitui certificado final.

## Segregação e autoridade

- Protocolo seguido: `tekt-materializacao-segregada`, com segregação por capacidades e artefactos.
- Isolamento técnico de leitura: `false`; filesystem compartilhado: `true`.
- Não foram abertos, inspecionados diretamente nem executados `04_wiring/tests/p1292_contract.rs` ou `04_wiring/tests/p1293_contract.rs`; os gates arquiteturais project-wide exigidos abaixo não foram usados para extrair conteúdo desses oráculos.
- Não foram abertos ou inspecionados receipts/oráculos protegidos nem executados gates protegidos.
- Não foram alterados L0, Núcleos Tekt, manifesto, selo, expectativas, fixtures, `equation.rs`, lotes A/C/D ou equações produtivas.
- Escritas efetuadas somente no helper privado autorizado de `03_infra/src/integration_tests.rs`, nos quinze alvos de `rustfmt`, neste receipt e na remoção literal de `./-`.

## Autoridade e inputs validados

- `00_nucleo/diagnosticos/p1293-manifest.json`: `7e89b6a9fc883b872ad476c259d0b3c5e465b5533c66f0cb70f33c044092d4ed`.
- `00_nucleo/diagnosticos/p1293-contract-seal.json`: `5f65415442128fa8b2d801b83ef13486f27d09c62a7949e0361f598a8810a2a5`.
- Bloco canónico `$.final_corrective_observers_and_hygiene` do selo, serializado por `json.dumps(..., sort_keys=True, separators=(',', ':'), ensure_ascii=False)`: `e26d9e57cfbbe8a7cc6620fb611c0cc071b986c0c958267d3d84bdaf6e8d8093`.
- L0 `00_nucleo/prompts/infra/integration_tests.md`, lido integralmente: `c7eb49160cb7516df502eb99124c93c2759d9666e573c638bfcf0b1e9d639bab`; header canónico esperado `75a971ae`; hash canónico do consumer selado `99985da2`.
- Preimage de `03_infra/src/integration_tests.rs`: `9aedc6f8a12c2b506ec712585b8c95f02cefb510687fa46818b1c630ed97efc9`.
- Inputs congelados reconfirmados no fim: `01_core/src/compiler/layout/equation.rs` = `ea81dc9e291d29e1ac6d17a240b98929228af31dc2a2a1563efae20ba25d9e23`; `00_nucleo/prompts/compiler/layout/equation.md` = `9efca34585e1776267286b66dc74ed6ca213832b91568a8ffd79ccd2b8578179`.
- Não surgiu owner gap: V15 e V26 ficaram verdes antes do fecho deste recibo.

## RED independente

No estado selado, antes da correção do observer, foi executado:

```text
cargo test -p typst-infra --lib
```

Resultado reproduzido exatamente: `910 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out`, exit `101`, em `5.77s`. Os oito witnesses existentes e não protegidos que ficaram RED foram:

1. `p1132e_cases_conteudo_preserva_centro_da_chave_sec09`
2. `p1132g_binom_sec07_sem_barra_e_com_posicao_vanilla`
3. `p1132k_sec12_limite_unilateral_preserva_altura_vanilla`
4. `p1133b_sec04_dif_com_sup_preserva_altura_vanilla`
5. `p1136_sec17_operadores_large_alinham_conteudo_adjacente_no_eixo`
6. `p1132n_sec18_chave_externa_seleciona_variante_vanilla`
7. `p1132o_sec18_fracao_aninhada_suprime_spacing_em_script`
8. `p1132p_sec10_assembly_de_bracket_usa_attachment_central`

Não foi alterada nenhuma expectativa. A proveniência produtiva focal do RED é o HEAD `7dd25ff0e222b6c7c640d6bc7957b98f94227507`, working tree não commitado, com os preimages congelados acima e no quadro de `rustfmt` abaixo. A medição foi feita nesta sessão em 2026-09-02 antes da primeira escrita do helper.

## Correção implementada

Somente `frame_items_recursive` e auxiliares privados locais foram alterados em `03_infra/src/integration_tests.rs`:

- a recursão passou a transportar `TransformMatrix` ancestral;
- para um `Group`, a composição é `ancestral.concat(translate(Group.pos)).concat(Group.matrix)`;
- as coordenadas observadas de `Text`, `TextShaped`, `Glyph`, `Image`, `Shape`, `Group` e `Link` são projetadas por `TransformMatrix::apply`;
- `Line.start` e `Line.end` são projetados do mesmo modo;
- `Semantic` e os filhos de `Link` preservam o transform ancestral, sem inventar translação;
- quando o transform é identidade, o helper devolve a referência original;
- quando há projeção, o helper cria uma cópia exclusivamente observacional e preserva a API histórica `Vec<&FrameItem>`; essas cópias vivem somente até ao fim do processo de teste e não entram em produto ou estado persistente.

Hash final do consumer corrigido e formatado: `472f0573ea01abb2d5dfbc12b012965917cdedb973b8161d4d1ec94604737d77`.

GREEN imediato após a correção e antes da higiene: `918 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`, exit `0`, em `5.92s`.

## Higiene `rustfmt`

Ferramenta: `rustfmt 1.8.0-stable (ded5c06cf2 2025-12-08)`, edition 2021. O helper corrigido foi formatado focalmente; em seguida foi aplicado `rustfmt` mecânico somente aos quinze alvos selados:

| Caminho | SHA-256 preimage | SHA-256 após `rustfmt` |
|---|---|---|
| `01_core/src/compiler/math/layout/accent.rs` | `5a824a64db2e236e9a61d0e6fb69f5bde0fd0196d94ef449b4ae44880e1d1282` | `8c6311089ed117e972d886b0d945a648630ea638d3c75d356bd9ed90dc81d3b0` |
| `01_core/src/compiler/math/layout/cancel.rs` | `bdca3371181309a818a3ca25f86048d5c6a0159c4249bc850f33b9a888ff7fae` | `5f11e75acb614be77c18bf81a0d563bcb9140440e4cac19dbef4ebc413d330ca` |
| `01_core/src/compiler/math/layout/cases.rs` | `91b8fc4d0710a140aaa894c9fa7829b7790ff34d9401ff8c3dd5877bc15ce797` | `34691c2bfcf5d141819de0fc6f1e18505e16cf46d3c7e53a455372abe5ebe11e` |
| `01_core/src/compiler/math/layout/frac.rs` | `3d23b4dfb85dca35056bd2dc09c29b462a6691ad154263766c026c51f5608e34` | `d6fdffb6a119aa8bb63b60f72de7156e3f16126dc460602fa269df0ef6f8e7d2` |
| `01_core/src/compiler/math/layout/matrix.rs` | `e930baa799f3c05a9acda4a32627da83ee86146ae7ef602b5e18ffe0ae3f76d6` | `a7facbf2f648778bf4ab29e985deadb68177558685828e16b1f3ad3a54584c65` |
| `01_core/src/compiler/math/layout/mod.rs` | `1c386e48028c85d1a93ab35829b910a9f03d1b8e12155bae4c14efadf75492a4` | `b492234e34ec39f5a0f26b2ecb0b87b7ecd64f46c672d75226aff37481a3aa41` |
| `01_core/src/compiler/math/layout/root.rs` | `951d8215e06f92e92d77039f2ae6f5c6ccf27fce2aedfd575e1f627b5687c85f` | `8d835a98dfe43045a6ab7a86b8c30ebf7b28c631581744dba9b8ba37453da71c` |
| `01_core/src/compiler/math/layout/spacing.rs` | `3a525ea69566d52d9345d4242bfdaf4e439f66d6d8fed2e151c24f449f0a00d8` | `31a184f4736722b74977f5e29ea9dbf4e0d4887424cd1ea310ffff89f4387dd9` |
| `01_core/src/compiler/math/layout/tests.rs` | `0a19d473687fdab69883d99c982c4f40492949f4f1b69337705346f6d680e95f` | `aac986049d7a5fc059ba8881ebc4f6845aaaf6022d63eaba1d43ad8834acdabd` |
| `01_core/src/compiler/math/layout/underover.rs` | `b4090c1d2752ab4c46d1013ec7a0014501bf1588d9af6bb728c6241867fe1147` | `e6a42dae479791e681a6b76a96f797d594017191441a752f7b02fd02872f1f8a` |
| `01_core/src/compiler/math/layout/vec.rs` | `b0ffc612fe16c00896be3717097bcde5e808dd870483d165d57356f5f610eec7` | `d1368de3e887cdd26fd1ee6277054bdf68bac20b8b0e7ee2915b4b8f554ec98e` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `657b33efd5a08c5bc845d9190cfdfa5fc62442349894336fc377b19d3aa85bec` | `3e9ed614fd93d472f261f57a98bfc63d3ded706c11b2ea78a4fded944576a4a1` |
| `01_core/src/entities/elements/math_attach.rs` | `d9b45f7534f6ec3fd363594c6316077cb3a41917fa99125141aec02d80f31b09` | `1c3419a919efccbcc444feb76ebc3dbe4cb708cfc23195aa62024913dd52efb4` |
| `03_infra/src/font_metrics.rs` | `f18bff7c5637f0d94eb48c6d184090d04e0446ef9a71132f8dfef2a050fef990` | `1523f9e4328167b063b65c657f3e605aa711b69554038d626205669bac08008a` |
| `03_infra/src/shaper.rs` | `1b7917b1efb442e1d21862d3117cd95196f640eabdede9ea83d9e78ca2c7074e` | `d80147ef1d10e720191f37d6e58a25755e3355966270c238e143624193bb7d00` |

Os oito caminhos `import_only_exact` (`accent`, `cancel`, `cases`, `frac`, `matrix`, `root`, `underover`, `vec`) ficaram sem qualquer delta contra `HEAD`: `git diff --quiet -- <oito caminhos>` devolveu exit `0`. Assim, a normalização restaurou exatamente o conteúdo versionado e não deixou mudança semântica nem residual nesses oito ficheiros.

## Remoção do artefacto órfão

- Antes da remoção, `sha256sum ./-` confirmou `414e2d883b7561996d3986f61c9e63ff210862f0e4cfb15f773b45973da36967`.
- Foi removido somente o caminho literal não rastreado `./-`.
- `test ! -e ./-` devolveu exit `0`.
- A remoção é irrecuperável pela árvore Git porque se tratava de artefacto gerado não rastreado.

## Verificações finais

| Verificação | Resultado |
|---|---|
| `cargo test -p typst-infra --lib` | PASS: `918 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`, `5.54s` |
| `cargo test -p typst-infra --lib integration_tests::integration::p1132` | PASS: `9 passed; 0 failed; 909 filtered out` |
| focal exato `p1133b_sec04_dif_com_sup_preserva_altura_vanilla` | PASS: `1 passed; 0 failed; 917 filtered out` |
| focal exato `p1136_sec17_operadores_large_alinham_conteudo_adjacente_no_eixo` | PASS: `1 passed; 0 failed; 917 filtered out` |
| `rustfmt --edition 2021 --config skip_children=true --check` nos 15 alvos + consumer corrigido | PASS |
| `cargo fmt --all -- --check` | PASS |
| `crystalline-lint --fail-on warning --checks v3,v4,v5,v7,v13,v14,v15,v26 .` | PASS: `No violations found` |
| `crystalline-lint --fix-hashes --dry-run .` | PASS: `Nothing to fix` |
| `git diff --check` | PASS |

Os testes emitiram warnings preexistentes, mas nenhum erro. P1292/P1293 protegidos não foram abertos diretamente nem executados por proibição expressa; este implementador não converte essa ausência de execução em sucesso nem em Unknown resolvido.

## Proveniência da medição final

- Commit base: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`.
- Working tree: não commitado e compartilhado entre papéis/agentes.
- Instante do snapshot que sustenta os resultados finais anteriores ao receipt: `2026-09-02T18:00:35-03:00`.
- `git status --porcelain=v1` nesse snapshot: 113 entradas; SHA-256 do output: `db19ccd0f1386cff0e713eecb904cc1a40b2074ad95b975b8a2061999bececa7`.
- SHA-256 do output de `git diff HEAD --stat` nesse snapshot: `d394f6237620b0005970f5eb6e0023c17eabf6c297027cd5c336ec445a1ae756`.
- A lista produtiva/test-only/higiene sob responsabilidade deste recibo é exatamente a enumerada nas secções acima; as demais alterações visíveis pertencem ao working tree compartilhado e não foram apropriadas nem modificadas por esta correção, salvo a normalização mecânica expressamente selada.

## Encerramento de papel

As correções autorizadas estão implementadas e os gates locais não protegidos estão verdes. O implementador para aqui: não emite aprovação do Lote B, não executa adversário, não executa gates protegidos e não adjudica P1293.
