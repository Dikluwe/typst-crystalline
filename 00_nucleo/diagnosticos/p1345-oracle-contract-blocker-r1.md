# P1345 — bloqueio contratual do probe opaco R1

Regime: **executado sem atestacao de isolamento**.

Autoridade: `/root/p1345_oracle`, no papel segregado de autor do oráculo.
Este diagnóstico foi produzido antes de qualquer fixture positiva, tabela
canônica, source verifier, probe, checker ou corpus P1345.

## Veredito

`CONTRACT_PROBE_OBSERVATION_PHYSICALLY_UNSATISFIABLE_STOP`

O contrato P1345 R1 não contém informação pública suficiente para o checker
verificar a transformação de challenge que ele próprio exige. Prosseguir faria
o checker confiar num campo emitido pelo próprio probe e recriaria exatamente
a autoatestação de `Unknown` que P1345 pretende eliminar.

## Medição antes da decisão

Estado medido em `2026-09-11T07:25:45-03:00`:

- `HEAD = 2f42d64253547734564513a1159ee6b584c1c4b4`;
- `sha256(git status --porcelain=v1 -z) =
  38a372b22176323c713d72a81323709bc44f895cc6f26ebb3e0cfd96633b1d02`;
- `sha256(git diff --binary HEAD) =
  099901758e0dda60cc4c16de0091adfdcedd3fcc63029e1270c34147b60aadef`;
- working tree não commitado; nenhuma alteração produtiva foi feita por esta
  autoridade.

O binding fechado declara, em
`p1345-contract-binding-r1.json:123`, somente estes oito campos de saída do
probe:

```text
schema
challenge_response_sha256
invocation_nonce_sha256
process_nonce_sha256
opaque_handle_count
public_projection_sha256
payload_octets_exposed
completed_phase
```

Em `p1345-contract-binding-r1.json:145`, o probe é obrigado a devolver
**somente** esses oito campos. Porém, em
`p1345-contract-binding-r1.json:146`, o checker é obrigado a verificar:

```text
challenge_response_sha256 =
SHA-256(domain_separator || challenge || process_nonce)
```

e também que `process_nonce` foi gerado recentemente pelo probe.

O único valor observável relacionado ao nonce é
`process_nonce_sha256`. SHA-256 não fornece a preimagem `process_nonce`; nenhum
outro canal para essa preimagem é permitido. Logo o checker não consegue
recomputar a igualdade exigida. Comparar somente o digest do nonce não resolve
a fórmula escrita; aceitar `challenge_response_sha256` tal como emitido pelo
probe seria autoatestação.

## Reprodução mínima

Dados públicos disponíveis ao checker:

```text
domain_separator
challenge
process_nonce_sha256
challenge_response_sha256
```

Dado necessário para recomputar a fórmula contratada:

```text
process_nonce
```

Esse dado não pertence ao schema fechado. Não existe algoritmo reprodutível
que obtenha uma preimagem SHA-256 arbitrária a partir de seu digest. A falha é
de observabilidade do contrato, não de implementação do probe.

## Correção contratual necessária

O orçamento em `p1345-contract-spec-r1.json:136` ainda permite uma revisão.
Uma revisão R2 deve escolher e fechar **uma** destas formas, sem misturá-las:

1. acrescentar `process_nonce_hex` canônico de 32 bytes ao output primitivo,
   exigir `process_nonce_sha256 = SHA-256(process_nonce)` e manter a fórmula
   original sobre os bytes do nonce; ou
2. mudar explicitamente a fórmula para
   `SHA-256(domain_separator || challenge || process_nonce_sha256)` e declarar
   que a prova pública vincula o digest, não a preimagem.

A primeira forma é preferível porque permite verificar separadamente o digest,
a transformação do challenge e a não repetição dos bytes efetivos do nonce.

## Escopo e parada

- artefatos autorais de oráculo P1345 emitidos: `0`;
- execuções focais: `0`;
- execuções do corpus completo: `0`;
- probes compilados/executados: `0`;
- candidato produtivo lido, escrito ou executado: `não`;
- selo, RED, implementação ou certificado: `não`.

O próximo estado autorizado é revisão contratual P1345 R2. Fixture, tabela,
verifier, probe, checker, corpus e autoria permanecem causalmente bloqueados.

