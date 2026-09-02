# P1293 — recibo de reabertura contratual por owner gap de spans no lote A

## Estado

```text
status: ready-for-coordinator-fix-hashes
contract-core: unchanged
protected-oracle: unchanged-and-not-read
prior-seal: invalidated
product-code: not-edited-by-contract-author
```

Emissão: `2026-09-01T15:17:37-03:00`.

## Papel, regime e capacidades

- papel: `autor_contrato_p1293`, reabertura causal após o primeiro candidato A;
- regime: protocolo Tekt completo, segregado por capacidades e artefatos, sem
  isolamento técnico de leitura no filesystem compartilhado;
- entradas lidas: manifesto/recibos P1293, owners L0 vigentes e somente os
  consumers baseline não modificados `call_dispatch.rs`/`field_access.rs` para
  medição `file:line`;
- escritas: os três owners L0 abaixo, `p1293-contract-receipt.md`, este recibo e
  `p1293-manifest.json`;
- não lidos/editados: `04_wiring/tests/p1293_contract.rs`, patch candidato em
  `float.rs`, ataques, veredito e qualquer produto B–D;
- predecessor: manifesto SHA-256
  `7fd0b482c23ddd639f317a74bf9e07275d61ab5924a322e8fc39f224ba685404`;
- recibo A parcial: SHA-256
  `14ca51a7b440ce65e46eda9dadd7b47a21e15dd7a991b193e5d103beac42ced1`.

## Medição anterior à decisão

Estado medido: HEAD `7dd25ff0e222b6c7c640d6bc7957b98f94227507`, branch
`Tekt`, working tree não commitida. O recibo A executou nove testes próprios:
quatro positivos GREEN e cinco RED apenas no range diagnóstico, com valores,
tipos, `repr` e mensagens corretos.

| Família | Expressão | Candidato | Vanilla/contrato |
|---|---|---:|---:|
| missing | `float.is-nan()` | `12..14` | `0..14` |
| extra | `float.is-nan(0.0, 1.0)` | `12..22` | `18..21` |
| named ligado | `float("NaN").is-nan(other: true)` | `19..32` | `20..31` |
| cast | `float.is-nan("x")` | `12..17` | `13..16` |
| acesso sem chamada | `float("NaN").is-nan` | `0..19` | `13..19` |

O consumer baseline sem patch `call_dispatch.rs:899-909` agrega a chamada
ligada em `Args`; `:1213-1240` faz o mesmo no caminho estático. Antes dessa
perda, `:964-985` demonstra o precedente `CollectionCallSpans`, recolhendo
call, positional e named completos. No outro consumer baseline,
`field_access.rs:106,441-446` usa o span total, e `:99-101` demonstra que
`access.field().span()` já está disponível.

## Decisão ADR-0107 / ADR-0127

O span de erro é observável da linguagem sob ADR-0107. O contrato A já exige
mensagem e span medidos, logo não há intenção nova. A correção é preservação
interna de paridade em fluxo contínuo ADR-0127: não adiciona campo, entidade,
trait, assinatura Rust pública, default ou fase de pipeline.

- `call_dispatch` preserva, somente para a identidade estática/ligada
  `float.is-nan`, call/positional/named antes de `eval_args` e escolhe call para
  missing, segundo positional para extra, named completo para named e primeiro
  positional para cast;
- `field_access` usa `access.field().span()` somente no acesso
  `Value::Float.is-nan` sem chamada;
- `float` conserva fórmula, coerção, mensagens e validação e usa o `Args.span`
  já selecionado, sem parsing de source.

`entities::Args`, avaliação, mensagens, valores, defaults e fase são
explicitamente imutáveis. Caminhos sintéticos mantêm fallback agregado.

## Ownership 1:1 e L0s pré-resselo

| Prompt L0 | Consumer exclusivo | raw SHA-256 atual | hash L0 canônico esperado |
|---|---|---|---|
| `00_nucleo/prompts/compiler/eval/call_dispatch.md` | `01_core/src/compiler/eval/call_dispatch.rs` | `12a5ead390651556e64353e636e2e5d8a64cc8466a789a14cd97e7770dd5e58b` | `548a37bb` |
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `01_core/src/compiler/eval/bindings/field_access.rs` | `5c406bb155dee43a1dafa247952b12bb56121eccf9f49f8724175f45a8ce7b2b` | `27bf3c60` |
| `00_nucleo/prompts/compiler/stdlib/foundations/float.md` | `01_core/src/compiler/stdlib/foundations/float.rs` | `0cb585699c80ab7d8f8c9bdab0bb0a00d1f3919e895d5db4aa8896e2afb4d6bd` | `7f4ef0b6` |

V15/V26 confirmam as três relações existentes; não foi criado prompt
compartilhado, segundo consumer ou owner 1:N. O dry-run prevê exatamente os
três ressellos acima e os correspondentes `Hash do Código` atuais
`d61e59b2`, `d37b05cb`, `c95f8eb6`; esta autoria não os escreveu.

Os outros seis owners produtivos já afetados por P1293 foram auditados nas
suas cláusulas P1293 e permanecem byte-idênticos: `structural/math.md`
`d12e6f40931aa60ccf0972f88d441aa439b115a649452839f5d78167147fce72`,
`eval/math.md`
`90ae89477cc55ea25c4f90e2ceac24900a57ecc59e54c7672e637c5cfd7a1ad5`,
`math_attach.md`
`53c54d6e23bf40497e066daf7a3e8ac9c2a5f86a60010c5df3ef5498ee3695b6`,
layout `attach.md`
`c9b6e3b3eb5724226ffe65597d0e66d7ab487f41907f2f9cf58cbad54381e9aa`,
`stdlib/html.md`
`66aeb6c2c84ca0ab4c6ffff99b97c22de02a2c438ca9a40c797ee4237d5229de`
e `compiler/eval.md`
`98d8255070dd4f23626d174ef3eef021299d62bd0a039ff521a7b5d363848f79`.
Eles pertencem aos lotes B–D ou a morfologia/layout B e não recebem autoridade
sobre spans A.

## Contrato, oráculo e invalidação

O recibo contratual reaberto possui SHA-256
`75bbce1912f08574069c21e7e6dd9486201f659d054ee9705f66df066183450e`.
O núcleo canônico de lots/comparison/mutações continua
`df0f4f6ab5e50495c95b2d0531a397913e58026583f63aaa1f54e74c33ade41a`:
os cinco ranges já pertencem à comparação exata de diagnósticos. O oráculo
protegido permanece byte-congelado no hash registrado
`6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5`
e não foi aberto nesta reabertura.

O selo histórico
`5996cd712dd33def5873e207a0fe3067b9f6c701c12398ca09bd67aa81619402`
está invalidado porque três inputs L0 e o recibo contratual mudaram. O gate
pré-selo `bb264d7a54850385b70f0a6d66643f5f081bf32d9ac6586b49c365c0eb2a352b`
não é reutilizável no novo lineage. Nenhum deles foi editado.

Refutador explícito: se o oráculo congelado não discriminar os cinco spans,
o contrato/oráculo terá de ser reaberto e esta cadeia deve parar antes de
qualquer adaptação. Com a comparação já congelada, a ação esperada é
revalidar o mesmo oráculo sem alteração.

## Gates desta autoria

- `crystalline-lint --checks v15,v26 --fail-on warning .`: PASS;
- `crystalline-lint --fix-hashes --dry-run .`: exatamente três ressellos,
  nenhum write;
- V5: exatamente três drifts esperados enquanto o coordenador não executar o
  resselo mecânico;
- `git diff --check`: PASS.

Próxima ação autorizada: o coordenador executa `--fix-hashes`, registra os
novos hashes raw dos três L0s/consumers e repete V5/V15/V26. Depois o
implementador pode fazer os cinco REDs passarem dentro dos três owners; o
testador/verificador independente deve revalidar o mesmo oráculo e emitir novo
gate antes de um novo selo. Lotes B–D permanecem bloqueados até então.
