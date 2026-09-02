# P1293 — recibo de reabertura contratual do lote B / owner de morfologia

## Estado

```text
status: ready-for-coordinator-fix-hashes-and-replacement-b-gate-seal
active-seal: none
invalidated-seal: 3f436a01616b580b7412e35b33071e73305446d7e48f10dc2a00cd2d80501c5c
canonical-contract: unchanged
protected-oracle: unchanged-and-unread
productive-code-written-by-this-role: none
```

Este recibo reabre somente o lote B de P1293. Não aprova implementação,
não autoriza o lote C e não é selo nem veredito.

## Autoridade, tempo e entradas

- papel: `autor_contrato_p1293`, autoria L0/contrato segregada;
- medição própria: `2026-09-01T15:55:55-03:00`;
- fechamento deste recibo: `2026-09-01T15:59:09-03:00`;
- HEAD: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`, branch `Tekt`, working tree
  não commitada;
- manifesto predecessor: SHA-256
  `68386dd0481a58584ea0e7047e39f7e259398135a1e8a818028219afb5c14bd3`;
- recibo de implementação B: SHA-256
  `ae74fa392fcb1b3b93d209347ba211bc207da72fb0ed6a0f1d14c1584937d8f1`;
- selo serial B recebido: SHA-256
  `3f436a01616b580b7412e35b33071e73305446d7e48f10dc2a00cd2d80501c5c`;
- baseline independente P1293: SHA-256
  `682cad4bbef22ed6364fd0a5fcb9a8994230e7d22eb716a631f2f2fba6a294b2`;
- medição vanilla: SHA-256
  `39f11f324677885ba093178fd5bc9cc40187a6dcceb67fa28fd55b247531c9a7`;
- contrato canônico congelado: SHA-256
  `2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072`;
- oráculo protegido, não aberto por esta autoridade: SHA-256
  `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5`;
- recibo contratual atualizado: SHA-256
  `e97141f1e7d1ef7b1b75a6881e591d89d780f23d6f489706e223731a3f8400dd`.

O estado inicial medido às `15:55:55` tinha exatamente os 18 paths
modificados registrados pelo `git diff HEAD --stat`: nove L0s P1293
(`compiler/eval.md`, `eval/bindings/field_access.md`, `eval/call_dispatch.md`,
`eval/math.md`, `math/layout/attach.md`, `stdlib/foundations/float.md`,
`stdlib/html.md`, `stdlib/structural/math.md` e
`entities/elements/math_attach.md`) e seus nove consumers correspondentes.
Esta autoria não abriu os diffs produtivos.

## Medir antes de decidir

O recibo B registra cinco REDs próprios antes de código produtivo:
`0 passed / 5 failed / 5375 filtered`, exit `101`. As cinco causas eram apenas
bindings ausentes. O consumer com os REDs tinha SHA-256
`07b26199e1fe03f4fcab51591c1ded7da45f462156fcbb8c08bc983dbb446a88`;
nenhuma implementação produtiva B foi escrita.

No consumer baseline deste owner, SHA-256
`6582446811b7469bb5e632d1c4f130417d27ad30388138a22027ee3add083eb4`,
`repr_content` começa em `01_core/src/compiler/eval/repr.rs:503`:

- `MathSequence` apenas concatena filhos em `repr.rs:583`;
- `MathText` usa a forma string em `repr.rs:584`;
- `MathFrac` ignora `line` em `repr.rs:585-587`;
- `MathAttach` vira scripts em `repr.rs:588-614`;
- `MathDelimited` usa forma genérica em `repr.rs:617-619`.

No baseline imutável de `HEAD:01_core/src/compiler/eval/math.rs:963-1002`,
`binom` conserva lowers nas posições pares de `MathSequence`, insere
`MathText(", ")` nas ímpares, usa `MathFrac(line:false)` e envolve o body em
`MathDelimited('(', ..., ')')`. O owner de `MathAttachElem` já transporta
sete campos e distingue `None` de `Some(Content::Empty)`. Assim, a forma
causal é fechada por variantes/campos/posições existentes; não exige payload
ou provenance novos.

## Decisão L0 primeiro

Somente `00_nucleo/prompts/compiler/eval/repr.md` foi alterado, raw SHA-256
`60a57b2c6a76b7a326576387e076551e37032b6aeecd904f23ea93a1f2c22cb5`.
Seu consumer exclusivo continua
`01_core/src/compiler/eval/repr.rs`; o header vigente em `repr.rs:2-3` aponta
para esse owner e ainda carrega `@prompt-hash 643e33d3`.

A decisão legitima exclusivamente:

1. `MathAttach` como `attach(base: ...)`, emitindo apenas slots presentes na
   ordem `t,b,tl,bl,tr,br`; `Some(Content::Empty)` é `none`, `None` é omissão.
2. `binom(upper: ..., lower: (...,))` apenas para o envelope estrutural exato
   de parênteses, `MathFrac.line == false` e `MathSequence` ímpar não vazia
   alternando lower/separador. Lowers usam índices pares; separadores ímpares
   devem ser exatamente `MathText(", ")`; singleton conserva vírgula.
3. Sintaxe e chamada qualificada convergem pelo mesmo payload canônico. Não
   há reconhecimento de nome da função, conteúdo-testemunha ou origem.

Fração com linha, outro delimitador, outro body ou sequência que não
satisfaça toda a alternância conserva a representação genérica. Isso evita
classificação nominal/permissiva.

Classificação: projeção de morfologia da linguagem sob ADR-0107 e correção
interna de paridade em fluxo contínuo sob ADR-0127. Não há API pública,
campo, entidade, trait, payload, `Args`, cast, default, compatibilidade ou fase
eval/layout nova; portanto não se abre novo gate humano.

## Ownership 1:1 e freeze dos demais L0s

V15 e V26 passam. O novo owner permanece 1:1 e não cria owner compartilhado.
Os demais L0s P1293 foram auditados por SHA-256 e permanecem byte-identical:

| Prompt L0 | SHA-256 congelado |
|---|---|
| `compiler/stdlib/structural/math.md` | `d12e6f40931aa60ccf0972f88d441aa439b115a649452839f5d78167147fce72` |
| `compiler/eval/math.md` | `90ae89477cc55ea25c4f90e2ceac24900a57ecc59e54c7672e637c5cfd7a1ad5` |
| `entities/elements/math_attach.md` | `53c54d6e23bf40497e066daf7a3e8ac9c2a5f86a60010c5df3ef5498ee3695b6` |
| `compiler/math/layout/attach.md` | `c9b6e3b3eb5724226ffe65597d0e66d7ab487f41907f2f9cf58cbad54381e9aa` |
| `compiler/stdlib/foundations/float.md` | `f28cb58e46ced68ddb29335a35e8ade4f03a7c30a573c7e79a86b1ef3bc99107` |
| `compiler/stdlib/html.md` | `66aeb6c2c84ca0ab4c6ffff99b97c22de02a2c438ca9a40c797ee4237d5229de` |
| `compiler/eval.md` | `98d8255070dd4f23626d174ef3eef021299d62bd0a039ff521a7b5d363848f79` |
| `compiler/eval/call_dispatch.md` | `bff7a205e96855e1df8106201d8fb55dd3102f81f8dfe47159d934255d3bda60` |
| `compiler/eval/bindings/field_access.md` | `3e5b96f5d62a693cb8291f63cd656a73ba19ebd69e29446858d36e475bff4a60` |
| `wiring/tests/p1293_contract.md` | `6e3355f87a69366fab64bd0160c8e4f49f6c4b5fb95e82e82991fbef199b779f` |

## Contrato, Unknown e invalidação

O núcleo canônico A–D, lots/comparison, 26 mutações e `Unknown=0`
permanecem byte-conceitualmente idênticos. O hash conceitual interno de
lots/comparison/mutações continua
`df0f4f6ab5e50495c95b2d0531a397913e58026583f63aaa1f54e74c33ade41a`.
O contrato já exige `attach(base: ..., t:none)`, binom com lower tuple,
singleton/vírgulas e convergência sintaxe/qualificada. O oráculo não precisa
ser alterado; se um gate novo provar o contrário, a cadeia para antes de
qualquer adaptação.

O selo B `3f436a01...` está invalidado porque omitiu o owner produtivo
`repr.rs` da allowlist. O selo não foi reescrito. O checkpoint A aprovado
permanece evidência histórica, mas não autoriza B no lineage reaberto.

## Gates executados e próxima ação

- `crystalline-lint --checks v15 .`: PASS, zero violações;
- `crystalline-lint --checks v26 .`: PASS, zero violações;
- `crystalline-lint --fix-hashes --dry-run .`: exatamente um drift esperado:
  `01_core/src/compiler/eval/repr.rs`, `old=643e33d3`,
  `hash-a=7c714164`, `hash-b=c024746c`;
- parse JSON do manifesto por `ruby -rjson`: PASS;
- `git diff --check`: PASS;
- nenhum `--fix-hashes` foi executado por esta autoria;
- nenhum produto, teste, oráculo, selo, ataque ou veredito foi lido/editado
  além das fontes baseline expressamente medidas e dos hashes autorizados.

O coordenador deve aplicar somente o resselo mecânico de `repr.rs`, confirmar
V5/V15/V26 e diff, e obter gate/selo substituto antes de reautorizar B.
Somente depois o implementador pode levar os cinco REDs próprios a GREEN.
