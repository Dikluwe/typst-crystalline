# P1342 — ataque adversarial independente R2

Regime: **executado sem atestacao de isolamento**. Veredito:
**BLOCKER_NOT_SEALED**.

O ataque cobre exclusivamente o poder discriminatório do contrato/oráculo
composite R2. Nenhum candidato futuro foi lido. R1 e R2 julgados permaneceram
intactos.

## Resultado

O corpus R2 intacto foi reproduzido primeiro: 21/21 negativos `Violated`, um
positivo `Preserved`, um opaco `Unknown`, score local 1.0 e agreement verdadeiro.

Depois, o harness independente repetiu A01–A21 e acrescentou 22 controles
mínimos nas fronteiras pedidas. Nas três ordens, o vetor foi idêntico:

- A01–A21: 21/21 `Violated` — os blockers publicados em R1 foram fechados;
- total independente: 43 negativos válidos;
- 34 `Violated`, 8 `Preserved`, 1 `Unknown`;
- mutation score: **34/43 = 0.7906976744186046**;
- normal/repeat/reverse: estáveis.

Nove sobreviventes impedem o selo.

## Blockers R2

### R2-B01 — `receipt_tail` runtime não está ligado à cadeia raw

X05 substitui o tail H12A por 64 zeros; X06 substitui o tail H00D por 64
`f`. Ambos continuam `Preserved`. O checker verifica apenas forma
null/hexadecimal, não igualdade com o último `append_receipt.receipt` do mesmo
hook/célula. Além disso, H00/H04/H06/H08/H09/H10 têm contagens/tails runtime,
mas não possuem recibo de evento correspondente na cadeia raw.

### R2-B02 — recibo do emissor do challenge não é resolvido

X09 troca os três `challenge_issuer_receipt_sha256` por 64 zeros e continua
`Preserved`. O hash out-of-band protege a integridade do envelope, mas o
checker não recebe nem resolve o recibo canônico referenciado. Qualquer texto
hexadecimal de 64 caracteres satisfaz esse campo.

O controle X08 confirma que igualdade direta dos challenges é rejeitada; a
lacuna é a proveniência/frescura alegada pelo recibo, não a comparação dos
vetores presentes.

### R2-B03 — ordem causal base não é completa

X11 troca `context-dispatch` e `dict-prebound-produced`, reconstitui
canonicamente toda a cadeia de recibos, raw digest, projeção e digests externos,
e permanece `Preserved`. `cell_semantics` só ordena a subcadeia walk → replay;
não impõe a sequência causal anterior exigida pelo contrato.

### R2-B04 — endpoints ainda desconectáveis

X15 e X16 mostram que `callback_with` e `func_callback` registrados na criação
da ocorrência não são comparados aos endpoints usados depois pelo dispatch,
With e corpo. X17 e X18 mostram que `run` e `ledger` são validados por domínio,
mas não por continuidade na célula. Todos classificam `Preserved` com recibos,
raw e projeção internamente válidos.

Os ataques publicados A08–A15 agora são corretamente rejeitados; os novos
vetores atingem elos não cobertos por aqueles ataques.

### R2-B05 — precedência de `Unknown` ainda pode ser burlada

X22 desconecta o `func_callback` na criação e torna apenas o witness opaco. O
checker devolve `Unknown` nas três ordens, embora uma falha não-payload de
identidade exista antes da opacidade. Deveria ser `Violated`.

## Controles corretos

Foram rejeitados: hash externo de evidência divergente, membro extra no
envelope, alcance/status falso de âncora, H00D+H00S simultâneos, challenge
duplicado, digest/chain corrompido, raw não congelado, digest before/after
divergente, projeção diferente do raw, R externo incompatível, evento extra
conhecido e bool usado como inteiro. A06/A07 e A01–A21 também foram rejeitados.

## Reprodução

```text
python3 -B 00_nucleo/diagnosticos/p1342-adversary-runner-r2.py \
  --checker 00_nucleo/diagnosticos/p1342-oracle-checker-r2.py \
  --contract 00_nucleo/diagnosticos/p1342-contract-spec-r2.json \
  --manifest 00_nucleo/diagnosticos/p1342-contract-binding-r2.json \
  --corpus 00_nucleo/diagnosticos/p1342-oracle-corpus-r2.json
```

Pins:

- contrato: `62d656241ecd96cd3d984025e59990d607a0efd75c3c326503c7ec07a3486c4d`;
- binding: `3b522cf1b0f288fe1e472f21f775089541835ede754250a14a3c2d17c17e9d48`;
- checker: `06889314af3b889c75e3b3b73f22d425ecc3ae813910c3ccd1e61f6deb3b169e`;
- corpus: `1dd1a7905942571ccc69297530e520014b1c9d920ac1f5ceeeca8962b4e00dcc`;
- harness: `0949026d345aa5ff3695b0f13a48b555b463dc96e317f54fef8d82a97c69407a`.

Medição em `2026-09-10T21:47:15Z`, HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitada.
SHA-256 de `git status --porcelain=v1 -z`:
`6a2041b45bb4afe2720c4931d6e335bcad7018353d120005bb4f1eaedbb0c1a4`.
SHA-256 de `git diff --binary HEAD`:
`ff4dfff0c1401e48855b4116c5e11ce895e4c21fafc9977d8af5c218f6338a86`.

## Decisão

O score obrigatório é 1.0. Com oito negativos `Preserved` e um negativo
`Unknown`, o único veredito admissível é **BLOCKER_NOT_SEALED**. Este papel não
corrige os artefactos julgados e não emite qualquer conclusão fora do fragmento
P1342.
