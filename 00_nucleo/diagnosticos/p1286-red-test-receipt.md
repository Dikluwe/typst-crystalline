# P1286 — receipt da suite independente RED

**Natureza:** autoria e execução pré-implementação dos testes. Não é Prompt
L0, implementação, campanha de mutações executada, selo nem veredito final.

## 1. Papel, ordem e fronteira

- Executor: agente independente `/root/testes_red_p1286`, em checkout
  compartilhado e sem atestação de isolamento físico do host.
- A suite só foi escrita depois de congelados e lidos o runner, baseline e
  receipt do oráculo. Nenhuma implementação candidata P1286 nova foi lida ou
  escrita por este papel.
- Escritas deste papel: somente
  `04_wiring/tests/p1286_contract.rs` e este receipt.
- Produção, L0, runner e baseline permaneceram fora da capacidade de escrita.
- `Unknown` nunca é sucesso. O teste exige `Unknown=0`, `Violated=0` e
  `verdict=Preserved` no runner congelado.

## 2. Entradas protegidas

| entrada | SHA-256 |
|---|---|
| contrato v2 | `16597543965ca13d37fdabf9be184f3477df92da4f6702ba14f0a4ca4918f9bb` |
| plano adversarial | `c107c2ef6c7776196ef3604ce078281c66d7d182c28ec6b32a08a531b64e1d83` |
| receipt L0 pós-gate | `27e2f5b931d898396319d7a62875ca377511e93ebe0e8b97cdd4f6817d6663c0` |
| runner black-box | `cc0d7986804a7abf006d03e44ffb6eb12a736b794db557b9d60d52e5cc02bf61` |
| baseline vanilla | `7452eea8ac3bc055a6f67586820a0754925ca3a1a80f1244e693cae2989e19fb` |
| receipt do oráculo | `b9ac9deffc38fee3dad850789572a95727c684d3222d57f050b653e5f9d370c8` |
| suite RED | `8936d84bcea19279257bd77df76dbb430b08d8591eec5dd808ded9bd1f96e06b` |

Os vinte L0s P1286 manifestados nos receipts foram lidos e tiveram os hashes
validados contra o contrato v2/pós-gate antes da escrita. Nenhuma outra pasta
de materialização/context foi listada ou lida.

## 3. Cobertura e mapa dos 25 mutantes

A suite dedicada executa os 61 casos do baseline congelado em duas ordens e
adiciona quatro witnesses independentes: `quotes:auto` apagando herança,
graphemes Unicode, path virtual/metadata de attachment e Formula descendente
de artifact com Formula irmã de controlo.

| mutante | witness |
|---|---|
| SQ-PRECEDENCE-01 | `smartquote_explicit_beats_alternative` |
| SQ-AUTO-02 | `p1286_smartquote_explicit_auto_erases_inherited_quotes` |
| SQ-GRAPHEME-03 | `p1286_smartquote_counts_unicode_graphemes` |
| SQ-LANG-04 | `smartquote_default_de` |
| SQ-ALTERNATIVE-05 | `smartquote_alternative_de` |
| LN-MAX-01 | `line_positive_origin` |
| LN-ABS-02 | `line_inverted_vector` |
| LN-NORMALIZE-03 | `line_negative_origin` |
| LN-START-04 | `line_positive_origin` |
| LN-END-PRECEDENCE-05 | `line_end_precedence` |
| CM-SEQUENTIAL-01 | `color_float_weights` |
| CM-RENORM-02 | `color_ratio_weights` |
| CM-NEGATIVE-03 | `color_negative_positive_sum` |
| CM-HUE-NGT2-04 | `color_error_hsl_n3`, `color_error_hsv_n3`, `color_error_oklch_n3` |
| PA-DROP-01 | `attach_path` |
| PA-BYTES-02 | `attach_bytes_metadata` |
| PA-PATH-03 | `p1286_attachment_preserves_virtual_path_and_metadata` |
| PA-METADATA-04 | `p1286_attachment_preserves_virtual_path_and_metadata` |
| PA-DUPLICATE-05 | `attach_duplicate` |
| AR-PASSTHROUGH-01 | `artifact_default_other` |
| AR-FORMULA-02 | `artifact_header` |
| AR-MCID-03 | `artifact_default_other` |
| AR-DESCENDANT-MCID-04 | `p1286_artifact_suppresses_descendant_formula_structure_only_locally` |
| AR-TAGS-OFF-VISUAL-05 | `artifact_tags_disabled` |
| AR-TAGS-OFF-MARK-06 | `artifact_tags_disabled` |

O teste meta `p1286_mutation_manifest_has_25_distinct_kill_witnesses` verifica
que há exatamente 25 IDs únicos e que cada witness congelado existe no
baseline. Isto prova o mapa discriminatório ex-ante; o score `25/25 = 1.0`
só pode ser declarado depois da injeção isolada futura de cada mutante.

## 4. Prova RED reproduzível

Comando final:

```text
cargo test -p typst-wiring --test p1286_contract -- --nocapture
```

Resultado em `2026-08-30T20:03:24-03:00`:

```text
exit=101
test result: FAILED. 1 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out
runner: Preserved=6; Violated=52; Unknown=0; NotApplicable=1; BaselineOnly=2
runner verdict=Violated
```

Falhas RED independentes:

1. runner congelado divergiu em 52 casos;
2. smartquote ainda rejeitou `quotes:auto` explícito;
3. smartquote ainda rejeitou string de dois graphemes/três scalars;
4. attachment ainda rejeitou metadata e não criou name tree/Filespec;
5. artifact ainda não criou envelope `/Artifact` nem a barreira local para a
   Formula descendente.

O teste meta 25/25 passou. A suite compilou; portanto o RED provém de
semântica/ausência P1286, não de erro sintático do teste.

## 5. Proveniência da medição

- `HEAD`: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Working tree: não commitada e compartilhada.
- Binário candidato executado: `target/debug/typst`, SHA-256
  `08e0b7cd1967b00a26178c7daa240989f0c1e0e7c418ddc268c552b427a43509`.
- `git diff HEAD --stat`: `106 files changed, 646136 insertions(+), 1442
  deletions(-)`, SHA-256 do output
  `96b81f2df4c11863144b9ad4058ecd4add63e2ca03d7b4b6708755bca47a8e9d`.
- SHA-256 de `git diff HEAD --name-only`:
  `f604a320860698abd79db5576b7c0047b47669883aefe5300459198aa2b7cb3c`.
- SHA-256 de `git status --short`:
  `9e15b90a1905b69f71fd8298f6b72c6285f59cd8da7f99eabf707cdc49546f4e`.
- A lista global incluía alterações concorrentes P1281–P1286. A identidade
  causal desta execução é o binário mais as entradas protegidas individualmente
  pinadas acima; a única alteração deste papel antes do run era o ficheiro de
  teste untracked.

`git diff --check -- 04_wiring/tests/p1286_contract.rs` terminou com exit 0 e
stream vazio. Este receipt autoriza somente a próxima fase causal
implementação→GREEN; não autoriza declarar o gate de mutação ou selo.
