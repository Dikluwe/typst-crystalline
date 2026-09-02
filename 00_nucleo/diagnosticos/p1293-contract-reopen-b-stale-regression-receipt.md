# P1293 — recibo de reabertura B por regressão test-only obsoleta

## Estado

```text
status: ready-for-coordinator-fix-hashes
classification: stale-test-owner-not-product-failure
canonical-contract: unchanged
protected-oracle: unchanged-and-not-read
seal-476513cc: invalidated
```

Este recibo pertence ao papel segregado `autor_contrato_p1293`. A autoria não
leu patch candidato nem o corpo do oráculo protegido e não editou produto,
teste, oráculo, selo, ataque ou veredito.

## Inputs e proveniência

| Input | SHA-256 / identidade |
|---|---|
| manifesto recebido | `76a6e9dbfc74987648a20e4efe1e9eb6f4ddbe078f6a006630694c1914c2dfc0` |
| recibo B causal | `3772126a3880a174c588dd62897e77e89bbf955fdd917dcd6218215132044eef` |
| contrato canônico | `2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072` |
| recibo contratual atualizado | `17e216d445cd7d65d4ed0caf9843766e86a650ee23bf2bb6bd6df6b1fc262fe9` |
| oráculo protegido, hash-only | `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5` |
| RED congelado | `91f3e53f1f4b01b10522821018a704aec658f7c8c37b9dae3eb92c2ca7114b5e` |
| selo serial invalidado | `476513cc1d324c4319609fc70f9237d12e0e74af08ce920980a356a820e1df5c` |
| HEAD / branch | `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt` |
| instante desta autoria | `2026-09-01T17:20:33-03:00` |

A medição causal ocorreu em working tree não commitada às
`2026-09-01T17:14:34-03:00`, com `git status --short` SHA-256
`e7ce2e33d6683603012059deaf3e8254dbedee2a93c9b25663cf779b097df55a`
e `git diff HEAD --stat` SHA-256
`26d25cac81995d45a9c667fa181ba17259eb9f65327117d1fbaeffa0d1435b7e`.

## Medição `file:line` antes da decisão

- `p1293-implementation-receipt-b.md:141-151`: suites
  call/math-style/math-attach/math-frac/repr e controles anteriores verdes;
- `p1293-implementation-receipt-b.md:153-187`: filtro `p1105` em
  `5 passed / 1 failed / 5379 filtered`, exit `101`, com única falha em
  `p1105_attach_zero_ou_multiplos_args_posicionais_erro`;
- `p1293-implementation-receipt-b.md:170-183`: expectativa antiga portuguesa
  para zero/excesso contra `missing argument: base`/`unexpected argument`;
- `p1293-implementation-receipt-b.md:115-139`: 20/20 spans B coincidentes e
  semântica/morfologia/layout aprovadas;
- `compiler/eval/tests.md:10-18,58-62` antes desta edição: owner test-only
  genérico, sem P1293;
- `compiler/eval/tests.rs:2-3`, header-only: ownership exclusivo para esse L0,
  `@prompt-hash 2c87f3cf`; consumer SHA-256
  `7e5c362b1854214aaedb90e1e01949a78f812164da8fcf9f6a2cdcaa4b052017`.

## Inferência, refutadores e classificação

Inferência: a única falha é um oracle regressivo P1105 obsoleto; o produto
P1293 satisfaz o contrato congelado. Refutariam essa inferência falha em algum
dos outros cinco controles P1105, mensagem divergente do vanilla/contrato,
novo RED de semântica/morfologia/layout/span ou necessidade de mudar produto.
Nenhum refutador foi medido.

As mensagens exatas são observáveis da linguagem (ADR-0107). A retificação
de expectativa no consumer test-only é correção interna de paridade em fluxo
contínuo (ADR-0127): não altera API pública, campos, entidades, traits,
assinaturas, defaults, compatibilidade ou fase do pipeline e não exige novo
gate humano.

## Decisão L0 e ownership

Foi atualizado primeiro e somente o owner
`00_nucleo/prompts/compiler/eval/tests.md`:

| Estado | raw SHA-256 | canonical/header |
|---|---|---|
| antes | `92ac9be1bcaeb65d0cda76603ba7260ffaccda2f6ff7049bf57bff24d6d83871` | consumer `2c87f3cf` |
| agora | `fe2a82590d703db2c6ea4e984e3b3ea7dc594199415627822b5b2836da18046c` | dry-run `hash-a=97d4926d`, `hash-b=5c55b251` |

O L0 exige somente:

- zero posicionais: `missing argument: base`;
- dois posicionais: `unexpected argument`;
- os outros cinco controles P1105 e todas as demais asserções inalterados;
- nenhuma aceitação das frases antigas e nenhum fallback/reversão no produto.

O ownership permanece 1:1
`compiler/eval/tests.md` → `01_core/src/compiler/eval/tests.rs`. Não foi
inventado owner 1:N. Todos os demais L0s P1293 auditados permaneceram
byte-idênticos:

| L0 | SHA-256 |
|---|---|
| `compiler/stdlib/foundations/float.md` | `f28cb58e46ced68ddb29335a35e8ade4f03a7c30a573c7e79a86b1ef3bc99107` |
| `compiler/stdlib/structural/math.md` | `d12e6f40931aa60ccf0972f88d441aa439b115a649452839f5d78167147fce72` |
| `compiler/eval/math.md` | `90ae89477cc55ea25c4f90e2ceac24900a57ecc59e54c7672e637c5cfd7a1ad5` |
| `entities/elements/math_attach.md` | `53c54d6e23bf40497e066daf7a3e8ac9c2a5f86a60010c5df3ef5498ee3695b6` |
| `compiler/math/layout/attach.md` | `c9b6e3b3eb5724226ffe65597d0e66d7ab487f41907f2f9cf58cbad54381e9aa` |
| `compiler/stdlib/html.md` | `66aeb6c2c84ca0ab4c6ffff99b97c22de02a2c438ca9a40c797ee4237d5229de` |
| `compiler/eval.md` | `98d8255070dd4f23626d174ef3eef021299d62bd0a039ff521a7b5d363848f79` |
| `compiler/eval/repr.md` | `5ef385638f3d828025f18735cb180b57288c170a44ac131752ae92d79ebc4737` |
| `compiler/eval/call_dispatch.md` | `f683a20d0171fe82983ac20886b79d09710ce1c38b036a43e8ac56e58e8959f7` |
| `compiler/stdlib/math_style.md` | `4ef71193711435b3b732354ccfdb77973ae81275aae628c61ab272ed87a68b97` |
| `compiler/eval/bindings/field_access.md` | `3e5b96f5d62a693cb8291f63cd656a73ba19ebd69e29446858d36e475bff4a60` |

## Gates antes do resselo

- V15: PASS;
- V26: PASS;
- dry-run de hashes: exatamente
  `eval/tests.rs old=2c87f3cf hash-a=97d4926d hash-b=5c55b251`;
- nenhuma escrita de hash foi feita;
- contrato canônico/oráculo/RED/produto/teste/selo permanecem inalterados.

Próxima ação: o coordenador executa `--fix-hashes` e confirma que somente o
header de `eval/tests.rs` mudou. Depois, o papel autorizado para testes corrige
somente as duas expectativas P1105, prova `6/6` e revalida a cadeia. Um selo
serial substituto depende dessa lineage e desses gates; lote C continua
bloqueado.
