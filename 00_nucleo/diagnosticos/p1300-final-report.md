# P1300 — relatório final do verificador independente P8

## Veredito terminal

`P1300_BLOCKED_IMPLEMENTATION`

Não é emitido certificado. O fragmento focal contém `12`
`DIFFERENT_DIAGNOSTIC` determinísticos, não contém `Unknown`, e a correção
exata pertence a um owner fora do allowlist selado P1300. Há ainda drift
byte-level dos três L0 confirmados depois do selo, embora limitado às linhas
`Hash do Código`; a regra de invalidação do manifesto/selo não contém exceção
para esse resselo mecânico.

Regime: protocolo completo Tekt, executado sem atestação de isolamento
técnico. O filesystem é compartilhado; hashes, capacidades, comandos e
limitações são registrados, mas não se alega isolamento que o ambiente não
prova.

## Integridade das entradas

O rehash P8 confirmou os sete artefatos congelados:

| Artefato | SHA-256 | Estado |
|---|---|---|
| manifesto | `c3e946e1330ccab108dba2f9a438fd6a8d4c3a2af7fca3307842dbd7403feed8` | PASS |
| contrato | `c5eace73e5ff07ab49057ec1741283543408accf132ca2f4431a1ad3827a3e37` | PASS |
| oracle | `a9a519a8230e39c55fa07223e306a7156d6c2b07bd71ef2926cc9e43f1ce8278` | PASS |
| mutantes | `cefa2fb00d6ad5adb81783b86b94f2450c9b0366591e42637d926adc7e598501` | PASS |
| runner | `85fe95a2f6e7c48d613221d201e4aefe88ba8a0a270752073c45fe48122bad43` | PASS |
| receipt discriminatório | `9be1840e0837af8baafce7c2253ad882acc7c1e4d1a890ec2fdb89eb05c885a4` | PASS |
| selo | `c8cb9cbfad604e6f5c638c63dfb1dc8c91a36f3bf6f50601624ed27de2439956` | PASS |

A matriz pós-integração também coincide com o hash exigido:
`69d0dad6bb14f3f98db8847355c6e2836ac62c205d809f0a68cc78eb7f05a28a`.
Os JSON P1300 existentes foram aceitos pelo parser padrão.

Os L0 atuais, porém, já não têm os hashes byte-level selados:

| L0 | Selado | Atual | Delta exato |
|---|---|---|---|
| `compiler/eval.md` | `3bf911a0…` | `ce02b7d2…` | `Hash do Código: c0181201 -> 38aa5e89` |
| `compiler/eval/tests.md` | `d4d6382d…` | `a0e613fd…` | `Hash do Código: 5c55b251 -> c82a31ba` |
| `compiler/stdlib/color.md` | `81150210…` | `60de24ea…` | `Hash do Código: dfc7912c -> 356f1537` |

Substituir apenas cada linha pelo valor anterior reconstrói exatamente o hash
selado correspondente. Portanto não há deriva semântica escondida nessa
diferença, mas há `input drift` literal. O manifesto dá ao P7 capacidade para
resselar linhagem e, simultaneamente, declara que qualquer byte alterado nos
L0 confirmados invalida a cadeia, sem exceção. P8 não pode resolver esse
conflito por interpretação favorável nem emitir sucesso.

## Resultado focal: 6 negativos × 4 perfis

P8 validou todas as `44` linhas da matriz e reclassificou as `24` coordenadas
negativas contra o contrato congelado em ordem normal e invertida. O vetor
ordenado é invariável nas duas ordens, SHA-256
`ee1296eecd331abd0c6319d67215b35fcf747fbb4ff10423c4e357804c95fedb`.

| Perfil | Bare exatos | `std.*` diferentes | Unknown |
|---|---:|---:|---:|
| default | 3 | 3 | 0 |
| html | 3 | 3 | 0 |
| a11y | 3 | 3 | 0 |
| html+a11y | 3 | 3 | 0 |
| **Total** | **12** | **12** | **0** |

Exit code, stdout vazio e hints vazios coincidem em `24/24`. Mensagem, span
renderizado e stderr integral coincidem somente em `12/24`. Logo, em cada
perfil apenas três dos seis negativos satisfazem o contrato exato.

Testemunha mínima, `repr(type(std.hsl))` no perfil default:

- vanilla: ``module `global` does not contain `hsl` ``, span no field `hsl`,
  âncora renderizada `1:14`;
- cristalino: `module 'std' does not contain field "hsl"`, span em `std.hsl`,
  âncora renderizada `1:10`.

O mesmo ocorre com `std.hsv` e `std.linear_rgb` nos quatro perfis. Não é
diferença mecânica irrelevante: mensagem e localização de erro são
observáveis públicos exigidos pelo contrato e pelo ADR-0107.

O binário pós-integração registrado na matriz,
`/dev/shm/p1300-pre-gate.hSBewa/target/release/typst` de hash `799546de…`, já
não existia quando P8 iniciou. Por isso P8 não alega uma nova execução desse
binário: validou a matriz congelada e reproduziu sua classificação em duas
ordens. O vanilla `/usr/local/bin/typst` foi rehasheado em `7b4f40c…`.

## Auditoria dos testes P5

O lote focal real foi reexecutado:

```text
CARGO_TARGET_DIR=/dev/shm/typst-crystalline-p1300-p5 \
RUSTFLAGS=-Awarnings CARGO_TERM_COLOR=never \
cargo test -p typst-core p1300 -- --test-threads=1
```

Resultado: exit `0`; `10 passed`, `0 failed`, `0 ignored`, `5417 filtered
out`; execução dos testes `0.24s`, janela de parede
`2026-09-03T20:22:27.616838497-03:00`–`20:23:29.446288531-03:00` devido ao
rebuild do target dedicado.

Esse GREEN é uma testemunha contra a suficiência do teste, não contra a matriz:

- `p1300_assert_negative` chama somente a API interna
  `eval_expression_with_features` sobre `hsl` ou `std.hsl`;
- para `std.*`, espera expressamente
  `module 'std' does not contain field "<name>"`;
- exige o span interno `0..expression.len()`, isto é, a expressão inteira;
- nunca invoca o vanilla, não consome o oracle P2 e não verifica exit/stdout/
  stderr do CLI.

Assim, P5 prova ausência dos aliases nos quatro perfis e preserva controles
positivos selecionados, mas não prova paridade vanilla do diagnóstico. Nos
três casos `std.*`, congela precisamente a divergência que o L0 de testes
manda rejeitar: comparar classe, mensagem, hints e span público de cada erro
com o vanilla ratificado.

## Owner causal e escopo

A falha não cabe no allowlist P1300 atual.

- `01_core/src/compiler/eval/bindings/field_access.rs:106-111` escolhe
  `access.span()` para erros ordinários, cobrindo todo `std.field`;
- `01_core/src/compiler/eval/bindings/field_access.rs:373-379` emite
  `module '<name>' does not contain field "<field>"`;
- o owner 1:1 é
  `00_nucleo/prompts/compiler/eval/bindings/field_access.md`;
- o allowlist produtivo P1300 contém apenas `eval/mod.rs` e `stdlib/color.rs`;
  `eval/tests.rs` pertence ao testador A/B.

Mudar apenas `eval/mod.rs`, `color.rs` ou os testes não corrige mensagem e
span na origem. Qualquer tentativa de fazê-lo apenas para estes três nomes
exigiria blacklist, wrapper ou ramo especial, proibidos. A classificação é
`REQUIRES_NEW_OWNER_AND_SCOPE`.

## Rendimento decrescente e gates

Como o fragmento focal falhou de forma determinística, P8 não executou build,
check ou testes de workspace, replay integral do runner, rebuild release nem
`crystalline-lint`. Estado de todos: `skipped_due_to_focal_failure`.

Quando o focal for reparado, o lint deve usar o seletor real `--checks`. A
forma textual `--check` no passo é incompatível e não foi fingida como gate
válido.

`git diff --check` passou e o índice permaneceu vazio. P8 não fez staging,
commit ou push e não editou produto, testes, L0, contrato, oracle, mutantes,
selo ou outputs P1-P7.

## Próximo gate humano necessário

O dono deve escolher entre reabrir/expandir P1300 ou criar um passo dedicado
de paridade de field access. A autorização precisa:

1. incluir L0-first o owner `compiler/eval/bindings/field_access.md` e seu
   consumer 1:1;
2. autorizar um novo output A/B independente que substitua as expectativas
   `std.*` cristalinas pelo oracle vanilla;
3. resolver explicitamente a contradição entre resselo mecânico P7 e a regra
   de invalidação byte-level;
4. reconstruir o binário e exigir `24/24` diagnósticos exatos, zero `Unknown`,
   em ordem normal e invertida;
5. somente depois executar workspace e lint com `--checks` e considerar um
   certificado.

A alegação máxima preservável agora é limitada: os aliases foram removidos e
os controles positivos selecionados permaneceram na matriz registrada, mas o
contrato diagnóstico selado não foi satisfeito.
