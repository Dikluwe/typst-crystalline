# P1292 — amendment-12: resselo de linhagem após `--fix-hashes`

**Papel:** autor contratual segregado, sem escrita em produto/testes/oracle,
ataques ou veredito

**Predecessor:** contrato canônico v12
`493f5c58ce956b5a142cbf5016db4bbd7e4dc776aaff86749be7ebd156c2e5ef`,
seal v12
`d1a04bcc9351d8f63e77c35ee8bfddf581691bf44484c35b9c4975173edaae8f`

## Proveniência

- `HEAD`: `0eb39f8ecb48930515f2cadb6a378450855b5a72`;
- working tree não commitada;
- auditoria iniciada em `2026-09-01T12:37:11-03:00`;
- manifest corrente SHA-256
  `1d9841747760d1dcb48522fe3108a4670c8096cb64aad436fe877e71e2f7c044`;
- vanilla ratificado permanece `a51e02804`, binário SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

## Mudança pós-selo medida, não presumida

O seal v12 protegia 27 L0s. A validação byte a byte encontrou exatamente duas
divergências:

| L0 | SHA v12 | SHA atual |
|---|---|---|
| `compiler/math/layout/cancel.md` | `84b2c85216b568155318284d6a4579a8c13f232ad32dd9472e2207a0ef394ffd` | `9bf37e4d2f8a5e6517564ace60c99cb76a9899e12399812d1c2e955b148b82cd` |
| `entities/elements/math_cancel.md` | `cc164630f81c98c08951bae556cf9d12dbe094649f0e5ecb645bb4cbb825e3cc` | `320008c4e4915de15c8e85338943ea6383d833bb8d09f5cb8083ac94dd6a0d38` |

Os outros 25 L0s coincidiram. Para auditar substância sem confiar na narrativa
do reparador, os bytes v12 foram reconstruídos a partir dos arquivos atuais,
substituindo somente a linha `Hash do Código`:

```text
cancel.md:      81399128 → b804d54b
resultado:      84b2c85216b568155318284d6a4579a8c13f232ad32dd9472e2207a0ef394ffd

math_cancel.md: 2bb6fdd1 → e3993092
resultado:      cc164630f81c98c08951bae556cf9d12dbe094649f0e5ecb645bb4cbb825e3cc
```

Os hashes reconstruídos são exatamente os hashes protegidos pelo v12. Logo
todos os bytes fora dessa única linha são idênticos aos L0s v12; não houve
alteração de obrigação, aceitação, default, API, fase ou compatibilidade.

O gate Tekt atual `V5,V15,V26 --fail-on warning` retorna zero violações. Isso
confirma que `81399128` e `2bb6fdd1` são os hashes de código canônicos atuais
segundo a normalização do linter; o SHA bruto dos arquivos produtivos não é o
algoritmo normativo de `Hash do Código`.

## Correção test-only P1030

Antes do reparo de linhagem, a verificação final reabriu um teste legado:
`p1030_set_vec_delim_aplica`. O produto já construía `Content::MathVec` e os
controles públicos de render/`repr` estavam corretos; o helper test-only
observava somente `Content::MathMatrix` e, por isso, devolvia `None` para vec.

A correção foi confinada a `01_core/src/compiler/eval/tests.rs`: o observador
passou a distinguir `MathMatrix` e `MathVec`, sem alterar expectativas. Evidência
registrada:

- consumer test-only SHA-256 atual:
  `7e5c362b1854214aaedb90e1e01949a78f812164da8fcf9f6a2cdcaa4b052017`;
- receipt independente:
  `edabc574c2b8748e7b114230348aac3312b4acb66e70e5c587b04b6ca2ca5a61`;
- P1030: 7/7 GREEN;
- P1292 core: 17/17 GREEN;
- workspace: GREEN, incluindo oracle protegido 11/11;
- oracle protegido permaneceu byte-idêntico:
  `fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`;
- `Unknown=0`.

Essa correção test-only não muda o contrato v12. Ela é citada no v13 apenas
para fechar a causalidade temporal entre verificação final e reparo de
linhagem; não adiciona vetor, expectativa ou owner aos lots.

## Decisão de resselo

Invalidar formalmente o seal v12 pela mudança pós-selo, preservar exatamente
os 27 paths, substituir apenas os dois hashes L0 atuais e ressellar como v13.
Os objetos `lots` e `comparison_policy` permanecem byte-canonically idênticos:

```text
lots SHA-256
edeb8a195e15263dbfbc71de9d242657e79f9dd65e94c97e990b066e03f119bf

comparison SHA-256
71eb4eafff8d88233aaa3df6b2666a651db75ebcdfd0fe2d470dc6620d190a46
```

## ADR-0127

Não há novo gate humano: o delta comprovado é metadata de linhagem em dois
L0s e uma correção test-only de observador. Nenhum campo/método/assinatura
público, default de produto, fase de pipeline ou compatibilidade mudou. Se a
reconstrução não tivesse produzido exatamente os hashes v12, o resselo teria
parado por possível alteração substancial.
