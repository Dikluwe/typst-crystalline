# P1293 — recibo da segunda reabertura B / owner gap de spans

## Estado

```text
status: ready-for-coordinator-fix-hashes-and-replacement-b-span-gate-seal
active-seal: none
invalidated-seal: 6370e740cd0e53e5e56c9de87afff32730cdddc64287b907e4830642522fc182
canonical-contract: unchanged
protected-oracle: unchanged-and-unread
productive-code-written-by-this-role: none
lot-c: forbidden
```

Este recibo reabre somente os diagnósticos do lote B. Não aprova o produto,
não autoriza C e não é selo nem veredito.

## Autoridade, tempo e entradas

- papel: `autor_contrato_p1293`, autoria L0/contrato segregada;
- abertura causal medida pelo implementador: `2026-09-01T16:38:33-03:00`;
- medição própria dos owners: `2026-09-01T16:44:26-03:00`;
- fechamento deste recibo: `2026-09-01T16:48:24-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree: não commitada; o recibo B registra SHA-256 de status
  `b9590c755653afb309dd85d0d39fbcc7d0a9df5b1895d1e7a1a39e2135c8ddce`
  e de `git diff HEAD --stat`
  `721eac40e0d4b047cccc5a3cc65d4c3c5168c7c9bd627b76ea63762a4b784e80`;
- manifesto predecessor: SHA-256
  `6ff412f4de1356a5f17f0c59f4c7b65430c6f169ba78aea3d7b2af37abdf14d6`;
- recibo B causal: SHA-256
  `7ebff1e223ceebe789224ed18f2c0518990aae787d7f3e48c3f96bbfb3d792dd`;
- selo serial invalidado: SHA-256
  `6370e740cd0e53e5e56c9de87afff32730cdddc64287b907e4830642522fc182`;
- contrato canônico: SHA-256
  `2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072`;
- oráculo protegido, não aberto por esta autoridade: SHA-256
  `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5`;
- recibo contratual atualizado: SHA-256
  `ca655b3fcf1b454da7df1dfba3e34a491beb464f4ae14795032606393e7e827e`.

Esta autoridade não abriu o patch candidato nos quatro consumers B, nem o
oráculo/testes privados. Leu somente os dois consumers causalmente ainda
intocados por B-spans e os artefatos autorizados.

## Medir antes de decidir

O produto preserva dez testes próprios GREEN:
`10 passed / 0 failed / 5375 filtered`, exit `0`; saída SHA-256
`62cb3409c9202601d1e4716344a882ab141a3fe9ced9d7182bf22c67ed04919a`.
Os consumers no ponto de bloqueio são:

| Consumer B candidato, não aberto | SHA-256 |
|---|---|
| `compiler/stdlib/structural/math.rs` | `437980d9843ed9159b7312efe47a09d9336d13c74d182ccc9d6c4d268554867d` |
| `compiler/eval/math.rs` | `9a24894c3341371a42afe782ad39c505aeea25d689f590cd0a6c9d60b5bbd80b` |
| `compiler/eval/repr.rs` | `e9aad3461a6d0dc3152a1349f4f83fcb988e51666016b6ef4aa23e29ac05d21b` |
| `compiler/math/layout/attach.rs` | `5e3ed835337affe7aa8399d6ad8dc895ad1fe9f34344d23f7d367682f395241f` |

Os negativos black-box conservaram mensagens e bloquearam apenas spans:

| Caso | Candidato | Vanilla/contrato |
|---|---|---|
| `math.attach(1)` | agregado `1:11` | valor `1:12` |
| `math.attach([x], t: 1)` | agregado `1:11` | valor `1:20` |
| `math.mono(1)` | `Span::detached`, sem range | valor `1:10` |
| `math.script` negativos | mesma causa compartilhada | argumento ofensivo |

No owner aprovado `call_dispatch.rs`, SHA-256
`ceb021c1f9ec0edc5480005e61dc0cb24dcfd8f2ed0efc6e4c8c1499afeb54d5`,
`FloatIsNanCallSpans` já captura AST privada em `:60-113`; a forma ligada
captura antes de `eval_args` em `:953-960`; a forma nativa genérica preserva a
identidade resolvida antes de `eval_args` em `:1274-1303`. `NamedArg` expõe
span completo e `expr().span()`, suficientes para distinguir named
desconhecido de valor named inválido.

No consumer baseline `math_style.rs`, SHA-256
`439729b339dadd12cce1c7d7223c7b7aa6306ff2273cac4c645f21c378d651a4`,
os cinco erros do helper usam detached em `:32-101`; `native_mono` e
`native_script` já entregam seus nomes fechados em `:181-217`.

## Decisão L0 primeiro

Somente dois L0s foram alterados:

| Prompt L0 | SHA-256 pré-resselo | Consumer 1:1 |
|---|---|---|
| `00_nucleo/prompts/compiler/eval/call_dispatch.md` | `bc923d906d7c11c88e1fa420962a772b299d9d4ef92bfa7ce559cf8c65aa6340` | `01_core/src/compiler/eval/call_dispatch.rs` |
| `00_nucleo/prompts/compiler/stdlib/math_style.md` | `9cdfeaf9160fde11ce470c68344b4fad31dc404cbe2c6e468633ab7259223180` | `01_core/src/compiler/stdlib/math_style.rs` |

`call_dispatch` estende o precedente privado somente às identidades nativas
resolvidas `attach`, `binom`, `mono`, `script`. Captura call, positional,
named completo, valor do named e spread; depois de uma única avaliação,
seleciona em `args.span` chamada para missing, valor para cast, primeiro extra
positional para extra e named completo para unknown named. `script.cramped`
inválido usa o valor. Spread/estrutura ambígua preserva fallback agregado.

`math_style` usa o `args.span` recebido somente quando a identidade existente
é `mono` ou `script`; as outras doze funções conservam detached. Mensagens,
precedência, casts, constructors, reuso direto, ordem/quantidade de avaliações,
valores, morfologia, layout e defaults permanecem intactos.

Spans diagnósticos são observáveis da linguagem sob ADR-0107. O contrato já
os exige, portanto a correção é interna e contínua sob ADR-0127. Não há
campo, entidade, trait, API pública, `Args`, default, compatibilidade ou fase
nova; nenhum novo gate humano é necessário. Se outro owner ou contrato público
for necessário, a cadeia deve parar.

## Ownership e freeze dos demais L0s

V15 e V26 passam. Cada Prompt continua com um consumer e nenhum owner 1:N foi
criado. Os demais L0s P1293 permanecem byte-identical:

| Prompt L0 | SHA-256 congelado |
|---|---|
| `compiler/stdlib/structural/math.md` | `d12e6f40931aa60ccf0972f88d441aa439b115a649452839f5d78167147fce72` |
| `compiler/eval/math.md` | `90ae89477cc55ea25c4f90e2ceac24900a57ecc59e54c7672e637c5cfd7a1ad5` |
| `entities/elements/math_attach.md` | `53c54d6e23bf40497e066daf7a3e8ac9c2a5f86a60010c5df3ef5498ee3695b6` |
| `compiler/math/layout/attach.md` | `c9b6e3b3eb5724226ffe65597d0e66d7ab487f41907f2f9cf58cbad54381e9aa` |
| `compiler/eval/repr.md` | `5ef385638f3d828025f18735cb180b57288c170a44ac131752ae92d79ebc4737` |
| `compiler/stdlib/foundations/float.md` | `f28cb58e46ced68ddb29335a35e8ade4f03a7c30a573c7e79a86b1ef3bc99107` |
| `compiler/eval/bindings/field_access.md` | `3e5b96f5d62a693cb8291f63cd656a73ba19ebd69e29446858d36e475bff4a60` |
| `compiler/stdlib/html.md` | `66aeb6c2c84ca0ab4c6ffff99b97c22de02a2c438ca9a40c797ee4237d5229de` |
| `compiler/eval.md` | `98d8255070dd4f23626d174ef3eef021299d62bd0a039ff521a7b5d363848f79` |
| `wiring/tests/p1293_contract.md` | `6e3355f87a69366fab64bd0160c8e4f49f6c4b5fb95e82e82991fbef199b779f` |

## Contrato, invalidação e gates

O contrato canônico, oráculo, lots/comparison, 26 mutações e política
`Unknown` permanecem inalterados. O hash conceitual de
lots/comparison/mutações continua
`df0f4f6ab5e50495c95b2d0531a397913e58026583f63aaa1f54e74c33ade41a`.
O selo `6370e740...` fica inválido pelo owner gap; não foi editado.

Gates pré-resselo:

- `crystalline-lint --checks v15 .`: PASS;
- `crystalline-lint --checks v26 .`: PASS;
- dry-run: exatamente
  `call_dispatch.rs old=548a37bb hash-a=6cdb3f22 hash-b=eaca4b89` e
  `math_style.rs old=58661c6c hash-a=a7d71b3a hash-b=258e9e0e`;
- parse JSON do manifesto: PASS;
- `git diff --check`: PASS;
- nenhum `--fix-hashes` foi executado por esta autoria;
- nenhum produto, teste, oráculo, selo, ataque ou veredito foi editado.

Próxima ação: o coordenador ressella mecanicamente somente os dois consumers,
revalida V5/V15/V26/diff e solicita gate/selo serial substitutos. Só então o
implementador pode corrigir spans nos dois owners e revalidar os dez GREENs;
o lote C segue proibido.
