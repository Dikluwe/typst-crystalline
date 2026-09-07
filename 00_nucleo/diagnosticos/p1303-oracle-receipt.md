# P1303 — recibo independente do ORACLE

**Veredito:** `P1303_ORACLE_MEASUREMENT_CONFIRMED`

A medição fresca confirma, antes de qualquer decisão ou patch candidato, o vetor exigido pelo P1303. O regime foi **executado sem atestação de isolamento técnico**: o filesystem e o contexto de coordenação eram compartilhados, de modo que hashes, ordem e allowlists documentam a execução, mas não provam isolamento técnico.

## Identidades congeladas

- HEAD: `5b4a0d0438a535c54fdb5e74b28903c1313f5bc2`
- P0: `00_nucleo/diagnosticos/p1303-baseline-status.txt`, SHA-256 `8e06d7deba171604986f5ae3eca4d9c2c964a2aaf6d2a0f1971f7aa27574d9a8`
- passo autorizado: `00_nucleo/materialization/typst-passo-1303.md`, SHA-256 `f1db5c02b7e6af5ef461900c213e928a8d16bbed156d8eba7ff5c6b32c584fe9`
- vanilla `/usr/local/bin/typst`: SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`
- cristalino fresco `/dev/shm/p1303-oracle.gem2yY/release/typst`: SHA-256 `28667d00fd9959344981b23060594d112ada0ccab2ae1bd2e39955b851b4de8a`, modo `0555`, 47597656 bytes, `typst 0.15.1 (5b4a0d04)`
- runner do oracle: SHA-256 `bbe74a4475bea5dd91e5a00333ef72131fe2e3c7e0262d99d72ba4b12e25fde6`
- medição integral: `00_nucleo/diagnosticos/p1303-pre-measurement.json`, SHA-256 `ef3a4eb0b6fcb3fb0e9b1d8ec54bfbfa8f1a4f7b58dd7c675cbf677bada573d4`

Build fresco: `CARGO_TARGET_DIR=/dev/shm/p1303-oracle.gem2yY cargo build --release --bin typst`, exit `0`, duração reportada pelo Cargo `2m 59s`. Após o build, `chmod -R a-w` congelou o target; o hash do binário foi idêntico antes e depois dos probes.

## Matriz medida

| Perfil | Features | Vetor em ordem normal | Repetição invertida |
|---|---|---:|---:|
| `default` | nenhuma | 3 `DIFFERENT_DIAGNOSTIC` | idêntica |
| `html` | `html` | 3 `DIFFERENT_DIAGNOSTIC` | idêntica |
| `a11y` | `a11y-extras` | 3 `MATCH_VALUE` | idêntica |
| `html+a11y` | `html,a11y-extras` | 3 `MATCH_VALUE` | idêntica |

A execução aceita contém 48 runs e 24 comparações. Somando as duas ordens: 12 `DIFFERENT_DIAGNOSTIC`, 12 `MATCH_VALUE`, 0 `Unknown`/`EXECUTION_UNKNOWN` e 0 divergências de repetição. A ordem normal usou fields `data-cell`, `header-cell`, `table-summary` e precedência vanilla→cristalino; a invertida usou fields reversos e precedência cristalino→vanilla.

## Observações negativas

Nos perfis `default` e `html`, cada lado produziu exit `1`, stdout vazio, exatamente um erro primário, zero diagnósticos laterais, a mesma mensagem e os mesmos dois hints, na mesma ordem. Somente a âncora divergiu:

| Field | Range vanilla | Range cristalino | Slice vanilla | Slice cristalino |
|---|---:|---:|---|---|
| `data-cell` | `14..23` | `10..23` | `data-cell` | `pdf.data-cell` |
| `header-cell` | `14..25` | `10..25` | `header-cell` | `pdf.header-cell` |
| `table-summary` | `14..27` | `10..27` | `table-summary` | `pdf.table-summary` |

Mensagem exata por field (com o nome concreto no lugar de `<field>`):

```text
cannot access field `<field>` because the `a11y-extras` feature is not enabled
```

Hints exatos, na ordem:

```text
try enabling the `a11y-extras` feature
see https://typst.app/help/compiler-features for more details
```

Os bytes integrais de cada mensagem estão preservados no JSON, juntamente com argv, exit, stdout/stderr e hashes SHA-256.

## Observações positivas

Nos perfis `a11y` e `html+a11y`, ambos os binários produziram exit `0`, stderr vazio e stdout bilateralmente idêntico:

- `data-cell`: `"(function, \"data-cell\")"`
- `header-cell`: `"(function, \"header-cell\")"`
- `table-summary`: `"(function, \"table-summary\")"`

## Reprodutibilidade e calibração

Cada run no JSON preserva argv integral, identidade do binário, timestamps, duração monotônica em nanos, perfil/features, exit, stdout/stderr integrais, hashes de ambos os streams, range derivado e estado `complete`/`Unknown`. O range normativo é derivado sobre os canários ASCII por `coluna reportada - 1` e contagem dos carets. Como `typst eval` exibe a expressão sem o byte sintético usado pelo span interno, o JSON registra separadamente `displayed_caret_range` e `source_slice`, derivados da indentação visual dos carets.

Uma primeira passagem foi excluída por `ORACLE_CARET_SEPARATOR_PARSE_ERROR`: o coletor esperava `|`, enquanto o diagnóstico preservado usa `│`. Seu JSON bruto tinha SHA-256 `dcfa12fa04f364840b8594b308c337bd1a56f4a9bb52d5c8d6c70c122d16b678`, 48 runs e 24 `Unknown`. A única revisão passou a selecionar a linha que contém carets; binários, argv, corpus e classificador permaneceram iguais. A repetição corrigida zerou `Unknown`, portanto a falha anterior não é evidência contra o produto.

## Capacidades e limite da alegação

O ORACLE leu somente as instruções autorizadas, o P0, manifests via `cargo metadata --no-deps` e a interface/saída pública dos binários. O Cargo consumiu mecanicamente a fonte do workspace para o build, sem inspeção do conteúdo pelo ORACLE. O ORACLE não leu implementação candidata, L0 candidato nem testes candidatos e não editou produto, L0, testes, staging ou commits.

Durante a matriz, `p1303-contract.md` apareceu no filesystem compartilhado como artefato não rastreado de outra autoridade; não foi lido. Isso explicita a ausência de isolamento técnico, sem alterar o HEAD ou os hashes congelados. A alegação deste recibo limita-se aos três canários `pdf.data-cell`, `pdf.header-cell` e `pdf.table-summary`, nos quatro perfis e nas duas ordens registradas; não afirma equivalência funcional geral.

## Decisão posterior à medição

O vetor observado coincide integralmente com o esperado e todas as 12 verificações field×perfil passaram. A hipótese P1303 não foi refutada: o trabalho pode prosseguir para as fases posteriores do protocolo; não se aplica `P1303_BLOCKED_MEASUREMENT_REFUTED`.
