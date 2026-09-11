# P1345 — relatório adversarial focal R1

## Veredito

**ADVERSARIAL_R1_SURVIVORS_BLOCK_PRESEAL**

Regime: **executado sem atestacao de isolamento**.

O gate focal encontrou **13 sobreviventes em 80 negativos válidos**. O score
foi `67/80 = 0.8375`; portanto, o pré-selo, o RED, o candidato e a
verificação final permanecem bloqueados. Nenhum oráculo, contrato, L0, teste ou
código produtivo foi corrigido por esta autoridade.

## Proveniência da medição

- instante da execução focal final: `2026-09-11T08:01:54.899135903-03:00`;
- `HEAD = 2f42d64253547734564513a1159ee6b584c1c4b4`;
- working tree não commitado;
- `sha256(git status --porcelain=v1 -z) =
  6ff273dfd0fb59e8c13a32deeac4b00538ea9707cae0882a110f88b5efc3a0df`;
- 1.137 entradas no status medido com os quatro outputs adversariais presentes;
- `sha256(git diff HEAD --stat) =
  9783f07bc86c21e8874ebacec9d92ce5a12208e127748377c571b091a1c92e15`;
- stat rastreado herdado: 76 arquivos, 10.972 inserções e 833 remoções;
- comando focal:
  `python3 -B 00_nucleo/diagnosticos/p1345-adversary-runner-r1.py --output 00_nucleo/diagnosticos/p1345-adversary-report-r1.json`;
- temporários: `/dev/shm/p1345-adversary-*`;
- `full_corpus_runs = 0`;
- candidato produtivo: não lido, não executado e não modificado.

O runner tem SHA-256
`d758095443b701459e20a3193423771059348c4d76076bc251174564fa4a8a61`;
o relatório JSON resultante tem SHA-256
`44c54be9de183c6a6bef5d9a435b31916fbb0d9c6508b6e2066c5b7f6b99377f`.

## Cobertura focal

- 40 intenções de ataque P1344 reexecutadas: 38 rejeitadas e 2
  sobreviventes;
- 40 negativos P1345 novos: 29 rejeitados e 11 sobreviventes;
- 4 controles positivos de whitespace/comentários: 4 `Preserved`, zero
  regressões;
- mutações de fonte parser-valid foram verificadas com
  `rustfmt --edition 2021 --config skip_children=true --emit stdout`;
- foram exercitados token extra, ausente, reordenado e substituído, literais
  normais/raw/byte, comentários, markers, owner, anchors, normalização,
  duplicate JSON, corpus/tabela/checker/root, path/symlink, challenge, replay,
  nonce, emissor, executável e injeção de classificação/witness.

## Sobreviventes e causas públicas

### `INVOCATION_SCHEMA_AND_RUNTIME_BOUNDARY_NOT_ENFORCED`

Sobreviveram:

- `P1344-A23-synthetic-candidate-runtime` → `Preserved`;
- `P1344-R2A15-unknown-canonical-self-attestation` → `Unknown`;
- `P1345-N37-wrong-source-bundle` → `Preserved`;
- `P1345-N38-unknown-input-kind` → `Preserved`;
- `P1345-N39-extra-case-key` → `Preserved`.

O `judge` consome a operação diretamente e não valida, nessa fronteira, o
schema fechado do caso, `input_kind`, `source_bundle`, `runtime_bundle` nem
campos de resposta injetados. A validação de corpus pinado não substitui a
fronteira candidate/final exigida pelo contrato.

### `OPAQUE_PRIMITIVE_SCHEMA_AND_PROJECTION_NOT_CLOSED`

Sobreviveram como `Unknown`:

- `P1345-N20-probe-schema-forged`;
- `P1345-N21-probe-public-projection-forged`;
- `P1345-N22-probe-public-projection-null`;
- `P1345-N23-probe-bool-handle-count`;
- `P1345-N24-probe-bool-payload-count`.
- `P1345-N40-probe-float-handle-count`.

O caminho `run_probe` valida a lista de nomes das chaves, mas não valida o
valor enum de `schema`, o digest/tipo de `public_projection_sha256` nem os
tipos inteiros exatos dos dois contadores. Em Python, `true == 1`,
`false == 0` e `1.0 == 1`, de modo que booleanos e ponto flutuante atravessam
as comparações atuais. Os seis
negativos chegaram indevidamente a `Unknown` depois de o stdout real do probe
pinado ser mutado focalmente no limite de captura.

### `CHECKER_AND_ROOT_TRUST_DELIVERY_NOT_ENFORCED_BY_EXECUTABLE_INTERFACE`

Sobreviveram:

- `P1345-N19-symlink-root-alias` → `Preserved`;
- `P1345-N36-alternate-checker-self-root` → `Preserved`.

O source verifier resolve um root-symlink e o aceita como a própria raiz. Mais
grave para o protocolo, `validate_authorship_receipt` não recebe o hash
esperado do checker: bytes alternativos do checker podem ser incorporados a
um novo authoring root auto-consistente. Uma autoridade externa ainda pode
comparar o pin manualmente, mas essa obrigação não está fechada pela interface
executável julgada.

## Defesas que funcionaram

A igualdade canônica integral rejeitou as 33 mutações históricas de fonte e
as novas mutações de token/literal. Marker ausente/duplicado/desconhecido,
anchor, delta fora da cápsula, normalização, symlink de arquivo, digest de
nonce/resposta, replay, emissor, executável, stdout extra, phase forjada,
campo de resposta no JSON do probe e corpus alternativo também foram
rejeitados. O problema medido não é a produção canônica de tokens; está nas
fronteiras de invocação, probe e autoridade.

## Condição de parada

Qualquer sobrevivente bloqueia o pré-selo no Passo 1345. Como esta autoridade
é apenas adversarial, ela não corrige o oráculo e não decide um desenho de
reparo. O próximo ato causal permitido é uma revisão focal por autoridade de
oráculo, se ainda houver budget protocolar, ou um novo passo/decisão de
contrato quando a correção exigir alterar o contrato FINAL R2, cujo budget já
está esgotado.

Este relatório cobre somente o fragmento observável P1345. Não atesta
isolamento técnico, não executa corpus completo e não alega equivalência
funcional geral ou paridade Typst.
