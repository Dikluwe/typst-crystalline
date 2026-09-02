# P1293 — recibo do implementador da correção CLI multiline

Data: 2026-09-02
Papel: implementador test-only estrito
Estado: **CANDIDATO GREEN PARA VERIFICAÇÃO INDEPENDENTE**. Este recibo não aprova o Lote C, não aprova P1293 e não constitui certificado final.

## Regime e segregação

- Regime: protocolo completo da skill `tekt-materializacao-segregada`, sob contrato já selado.
- Papel exercido: somente implementador; não autor de contrato, adversário ou verificador final.
- Isolamento técnico de leitura atestado: `false`; filesystem compartilhado: `true`.
- Escrita limitada a `04_wiring/tests/cli.rs`, dentro das sete funções nomeadas pelo selo, e a este receipt.
- `04_wiring/tests/p1292_contract.rs`, `04_wiring/tests/p1293_contract.rs` e receipts/oráculos protegidos não foram abertos, inspecionados diretamente ou executados.

## Autoridade e inputs congelados

- Manifesto `00_nucleo/diagnosticos/p1293-manifest.json`: `e0fb525df782e0e1d1fcfd8c8cbe1c6e1d3a91a1b4d8707b1bf8ccbb9b1006d7`.
- Selo `00_nucleo/diagnosticos/p1293-contract-seal.json`: `231a0dd12f6ea445d1c3ac89cce17ab7017a84322155c2a21da4b4387eae391f`.
- Bloco canónico `$.final_cli_multiline_test_only`, serializado por `json.dumps(..., sort_keys=True, separators=(',', ':'), ensure_ascii=False)`: `b832ec0ed953da37635fcf53df1dcf9fa7d7c988c49f4951f9718836fe264c22`.
- L0 `00_nucleo/prompts/wiring/tests/cli.md`, lido integralmente: `7f48645d50982a0adc2bdeabbc5b62b40390da795d8ee9f2caca4c793e351e7b`; `Hash do Código` declarado `d4efbe07`.
- Consumer preimage `04_wiring/tests/cli.rs`: `9463ee1b71783166afe3498543c3bbf66e29d793659ed71d676830409f5a234f`.
- Inventário efetivo reconstruído somente a partir dos mapas canónicos do manifesto, sem abrir oráculos protegidos: `74` caminhos, SHA-256 canónico `34c905e7db72a248597d404bb37ffd7dd983994a1085252ae82bbc24c9b8a8b6`.
- Política: `Unknown` nunca conta como sucesso; observado `Unknown = 0`.

## RED reproduzido

Comando executado no preimage congelado:

```text
cargo test -p typst-wiring --test cli
```

Resultado exato: `running 71 tests`; `64 passed; 7 failed; 0 ignored; 0 measured; 0 filtered out`; exit não zero; `4.73s`.

Os únicos REDs foram os sete autorizados:

1. `p1168_html_typed_batch_repr_and_dom`
2. `p1173_1_html_lote_global_only_2_e_whitespace_block`
3. `p1174_1_html_lote_global_only_3`
4. `p1175_1_html_lote_global_only_4_e_espaco_protegido`
5. `p1176_1_html_lote_residual_normal`
6. `p1177_1_html_familia_ruby`
7. `p1178_1_html_familia_documento`

P1168 falhou porque buscas parciais monolinha não reconheciam o membro `p` em bloco multiline. Os outros seis falharam em igualdade exata entre a forma canónica multiline observada e a expectativa monolinha obsoleta. Não apareceu outro RED, `Unknown` ou owner gap.

## Implementação test-only

Foram alteradas exclusivamente as sete expectativas/buscas de `repr`:

- P1168 agora compara por igualdade o tuple canónico completo, com os doze membros na ordem original, incluindo o bloco multiline de `p`, seus atributos ordenados e `body`;
- P1173–P1178 agora comparam por igualdade os blocos multiline completos de `abbr`, `mark`, `picture`, `summary`, `ruby` e `title`, incluindo abertura, indentação, ordem `tag`/`attrs`/`body`, vírgulas e fecho;
- não foi criado helper e não há normalização de whitespace, busca parcial, tolerância ou alternativa frouxa.

Inputs, comandos, fixtures, conteúdo, escaping, ordem, output HTML e todas as expectativas DOM posteriores ficaram inalterados. Nenhum produto, L0, manifesto, selo, surface, contrato ou outro teste foi alterado.

Postimage de `04_wiring/tests/cli.rs`: `6a0075c7021f01de3888b1e50f40ee35c8e03a4d507f90b62ae2524e9aafaaca`.

## GREEN e cobertura não protegida

- `cargo test -p typst-wiring --test cli`: `71 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`; exit `0`; `5.05s`.
- `cargo test --workspace --exclude typst-wiring`: PASS, exit `0`; inclui core `5417/5417`, infra `918/918`, shell `62/62`, o target infra `p1250_svg_integrated` `1/1` e doctests.
- `cargo test -p typst-wiring --test crystalline_lint --test p1286_contract --test p1289_float_is_infinite --test p1291_callback_runtime`: PASS, exit `0`; respetivamente `2/2`, `6/6`, `1/1` e `5/5`.

O workspace literal não foi executado porque incluiria os targets protegidos P1292/P1293. A cobertura acima executa todos os pacotes e todos os targets wiring não protegidos enumerados por `cargo metadata`; a CLI já foi executada separadamente. Uma tentativa inicial de combinar esses targets com `--lib` foi rejeitada antes de executar testes porque `typst-wiring` não possui target de biblioteca; o comando corrigido acima passou integralmente.

## Gates finais do implementador

- `cargo fmt --all -- --check`: PASS, exit `0`.
- `crystalline-lint --checks v3,v4,v5,v7,v13,v14,v15,v26 .`: PASS, exit `0`, `No violations found`.
- `crystalline-lint --checks v5 --fix-hashes --dry-run .`: PASS, exit `0`, `Nothing to fix`.
- `git diff --check`: PASS, exit `0`.

## Proveniência

- Instante final dos gates: `2026-09-02T20:04:30-03:00`.
- HEAD: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`.
- Working tree compartilhado não commitado.
- SHA-256 de `git status --porcelain=v1`: `c975e15fa11ab16cefe6217a399b45fea9301fb05906291b50f6c573404ad52c`.
- SHA-256 de `git diff HEAD --stat`: `2455be7f0baa922287b036e0dcb1a0acfbc392000d56ee1dbaec278f3ee806f5`.
- SHA-256 de `git diff -- 04_wiring/tests/cli.rs`: `e9150f69a2733a524c04c66f37787158d699fa9747f0236f6d3d67cef6d80676`.
- Os hashes de status/stat foram recolhidos antes da criação deste receipt; como ele é untracked, sua criação não altera o conjunto de paths de status nem o diff stat de ficheiros tracked.
- O diff focal contra HEAD contém também o header de linhagem `7396cebb`, já integrante do preimage selado; a escrita deste implementador começa somente nos sete hunks de expectativas.
- Manifesto, selo e L0 foram reconfirmados nos hashes congelados depois dos gates.

O candidato é devolvido para verificação independente. Este implementador para sem aprovar o Lote C/P1293 e sem emitir certificado final.
