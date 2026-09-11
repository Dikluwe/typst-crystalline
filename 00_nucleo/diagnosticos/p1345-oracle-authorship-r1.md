# P1345 — autoria do oráculo canônico R1 sobre contrato FINAL R2

Regime: **executado sem atestacao de isolamento**.

Autoridade: `/root/p1345_oracle`. O contrato FINAL R2 reparou o bloqueio
`OPAQUE_PROBE_PREIMAGE_NOT_OBSERVABLE`; o bloqueio e os artefatos R1 permanecem
imutáveis na cadeia causal.

## Veredito

`FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED`

Este veredito autoral não é selo. O adversário independente ainda precisa
atacar estes artefatos e o pré-verificador é a única autoridade autorizada a
executar o corpus completo antes do selo.

## Artefatos autorados

| Artefato | SHA-256 |
|---|---|
| `p1345-positive-fixture-r1.json` | `8cf4078146a3625931027d65a56f2610b132b2e9f6cf775ab4761b849a4d9e12` |
| `p1345-canonical-capsule-table-r1.json` | `ffe9322a609c3f36ea152ce8a872407ebd7daddfef033660b495e235f0d2301f` |
| `p1345-oracle-corpus-r1.json` | `daf610f19634bea6245f08e530fcb75a53532500c63c2f8a98ef583096573a1f` |
| `p1345-source-verifier-r1.py` | `a787f52fbcc2590255d50d982c8f0ccbca84f0c750062ede05aa6b171744d8b7` |
| `p1345-opaque-probe-r1.rs` | `863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc` |
| `p1345-oracle-checker-r1.py` | `8edf345e27e4b12bf4e2eb39cefa2a854f6ea73c2749e6a76f3d93c1119abc5e` |

## Produção canônica

- `38/38` cápsulas, na ordem fechada do contrato;
- `12/12` arquivos owners;
- `38` IDs, sequências e digests únicos;
- `2.506` tokens canônicos consumidos integralmente;
- cada registro contém owner, kind, markers, anchors, replacement inverso,
  sequência completa, cardinalidade e digest;
- a normalização inversa consumiu `38/38` regiões e recuperou `12/12` hashes
  completos candidate-free;
- whitespace e comentários são os únicos lexemas ignorados.

O source verifier compara cardinalidade e todos os pares
`[lexical_kind, exact_raw_lexeme]`. Em divergência ele registra o primeiro
índice, token real e token esperado. Nenhuma blacklist, contagem de nomes ou
presença parcial decide aceitação.

## Corpus answer-free

O corpus contém `107` casos únicos e nenhum campo de classificação ou witness:

- `40` ataques históricos P1344 R1+R2, recompostos como mutações efetivas;
- `38` mutações token-level, uma por cápsula;
- as quatro classes extra/ausente/reordenado/substituído;
- controles positivos de bytes exatos, whitespace e comentários;
- decoy de string, markers, owner, anchor, normalização, paths e autoridade;
- corpus/fixture/table alternativos e JSON com chave decodificada duplicada;
- ataques de replay, recibo ausente, emissor/executável errados, nonce/digest,
  response e stdout extra;
- um único caso opaco executado.

## Probe opaco R2

O probe lê somente o request JSON fechado por stdin, gera `process_nonce_hex`
de 32 bytes em `/dev/urandom` e emite somente os nove campos primitivos R2.
O checker:

1. compila a fonte pinada com o binário `rustc` congelado;
2. rehasha o executável antes e depois do processo filho;
3. cria challenge e invocation nonce novos;
4. valida schema/ordem e decodifica os 32 bytes do process nonce;
5. recomputa os três SHA-256 exigidos;
6. rejeita repetição no registry;
7. deriva `Unknown` somente depois dos predicados de opacidade.

A implementação SHA-256 do probe foi validada contra o SHA-256 externo no
vetor de invocation nonce do controle e o probe foi compilado/executado em
`/dev/shm`.

## Execução focal

Comando:

```text
python3 -B 00_nucleo/diagnosticos/p1345-oracle-checker-r1.py \
  --focus \
  --corpus 00_nucleo/diagnosticos/p1345-oracle-corpus-r1.json
```

Resultado final:

- casos focais: `13`;
- positivos: `3/3 Preserved`;
- negativos: `9/9 Violated`;
- score focal: `1.0`;
- opaco executado: `1/1 Unknown`;
- sobreviventes: `0`;
- execuções do corpus completo: `0`.

Também passaram `py_compile` para os dois scripts, `rustfmt --check` para o
probe e parsing `rustfmt` de `12/12` arquivos da fixture sintética. A árvore
positiva foi materializada somente em `/dev/shm`.

## Limites

- nenhum candidato produtivo foi lido, escrito ou executado;
- nenhum L0, passo, teste, contrato ou baseline foi alterado;
- nenhum Cargo, A/B real, full, adversário, pré-selo ou certificado foi
  executado por esta autoridade;
- source-token equality prova somente a forma privada fechada; não prova
  reachability, tipos, borrow, identidade, cardinalidade nem equivalência geral;
- o filesystem e o contexto herdado impedem atestação técnica de isolamento.

Próximo estado: `P1345_INDEPENDENT_ADVERSARY_FOCAL_ALLOWED`; pré-selo continua
bloqueado até zero sobreviventes independentes.
