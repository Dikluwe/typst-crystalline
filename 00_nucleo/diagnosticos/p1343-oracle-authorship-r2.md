# P1343 — revisão focal R2 do verificador e dos oráculos

## Veredito desta autoridade

`FOCAL_R2_PASS_NOT_PRESEALED_NOT_SEALED`.

Regime: **executado sem atestacao de isolamento**. A revisão foi segregada por
papel, entradas, ordem e allowlist de escrita, mas ocorreu no workspace
compartilhado. Esta autoridade não leu candidato futuro, não escreveu código
produtivo ou teste Rust, não executou o corpus completo e não emitiu selo.

## Causa e delta R1 → R2

Os artefatos adversariais protegidos (runner `6064ab8d…a899`, relatório
`2ceaf773…ff8d` e recibo `00dd7a22…1802`) demonstraram ADV01–ADV11. O R2 mata
exatamente essas classes:

- liga cada row a owner Rust resolvido, payload de hook literal e callsite no
  corpo autorizado; função aninhada/morta não prova reachability;
- exige `P1343RawCarrier::p1343_append_raw` no carrier autorizado, sobre o
  objeto exato `self.p1343_raw_events`, e fecha mutações por `push`, `extend`,
  `append`, `insert`, `splice`, assignment, `clear`, `truncate`, `drain` e
  alias mutável;
- proíbe construtor post-hoc, torna a projeção read-only e prova em H16 o clone
  exato do ledger antes do limite de projeção;
- exige fachada concreta com três caminhos explícitos e runtime com três
  challenges, run IDs, refs e snapshots únicos, cardinalidade de 19 rows e
  join literal modo/run/challenge/row/hook;
- torna `--corpus-sha256` externo obrigatório e rehasha os bytes antes do
  parse; `--candidate-root` é rejeitado fail-closed no checker sintético;
- gera corpos contextuais Rust válidos. Nove replacements expression-tail
  usam a allowlist fechada H06-BOUND-UPDATE, os sete FUNC-CTOR e H15, derivada
  da posição sintática do baseline e da autorização contratual de um único
  wrapper. Os demais 27 mantêm a forma direta.

O scanner lexical, pareamento/allowlist de IDs, normalização inversa por
offsets decrescentes, hash integral dos dez arquivos, cfg/branch normal,
âncoras e símbolos permanecem derivados dos bytes pelo R1 pinado e são
endurecidos, não substituídos por flags de evidence.

## Calibração focal

Somente os 18 casos autorizados foram executados: P01, P02, Y01, ADV01–ADV11
e C01/C10/C11/C22. A execução final em 2026-09-10T22:08–03:00, sobre HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado,
produziu:

- 18/18 classificações concordantes;
- 14/14 negativos `Violated`, score focal `1.0`, zero survivors;
- P01, Y01 e ADV11 `Preserved`; P02 permaneceu o único `Unknown`;
- `rustfmt --edition 2021 --config skip_children=true --emit stdout` aceitou
  10/10 arquivos sintéticos e a normalização recompôs 10/10 hashes baseline.

Vetores intermediários foram, em ordem: wrapper expression-tail incompatível;
freeze apenas nominal herdado; resolução de `eval_expr` sem genericidade;
resolução lógica `eval_expr_inner`; regex de projeção sem return type; e bytes
não canônicos antes do replacement. Cada correção mudou a causa; nenhuma causa
sem ganho discriminatório recebeu duas revisões sem parada. Houve oito
invocações focais de processo (duas repetidas apenas por limite de transporte
de output, sem revisão semântica), aproximadamente 3,5 minutos de wall time,
com temporários exclusivamente em `/dev/shm`.

Também foram exercidos diretamente dois fail-closed: corpus com hash externo
zero → `CORPUS_PIN`; presença de `--candidate-root` → `RUNTIME_COVERAGE`.

## Corpus completo reservado

O R2 compõe, por hash, a ordem fechada integral de 85 casos do corpus R1 e
acrescenta ADV01–ADV11 sem duplicar P01/P02/Y01/C01/C10/C11/C22. Assim, o modo
externo não-focal tem 96 casos e três ordens. Esta autoria não o executou:
`full_corpus_runs = 0`. A autoridade independente de preseal deve executar
essa composição uma única vez; o resultado focal não é selo.

## Limite decisivo de runtime

O runtime fabricado por `runtime()` no checker R2 serve **somente** ao corpus
discriminatório sintético preseal. Nem candidato nem final podem ser aceitos
por esse runtime. O checker rejeita qualquer `--candidate-root` antes da
classificação.

Candidato/final exige evidence real externo, com hashes controlados fora do
candidato, produzido pela fachada/teste Rust real, seguido do teste Rust A/B,
build normal sem cfg de observação e integração final independente. Mesmo uma
saída `PASS` do source verifier R2 é necessária, porém insuficiente.

## Proveniência e limites

Na medição final o repositório tinha 72 arquivos tracked alterados
(`10889 insertions`, `830 deletions`) e os cinco artefatos R2 desta autoridade
como saídas novas; digest do status porcelain naquele ponto:
`d96d17dbb1f64f2b200afcb81c40d265a0aad5d0bd45bb4b4c55571b45f802de`.
As alterações tracked pertencem ao trabalho compartilhado e não foram tocadas
por esta autoridade. O ambiente compartilhado impede atestar isolamento.

O resultado cobre somente o contrato focal P1343/P1342. Não prova equivalência
funcional geral, não fecha P1340, NT01–NT06, retenção ou política terminal e
não autoriza implementação.

## Saídas R2 antes do recibo

- source verifier: `fa7c15eea4b45e83c5f078a7a3820bd0499d55087aaa96f7863ea5229495512d`;
- checker: `ff6d83c9dbfe356b2068204e15d05025f10d83eacb902d9b9c1511a20b098ab9`;
- corpus: `72eb41172966de03a42dfc26eea39f7c426f38123f6ba512bb8bc663a30cfcce`.

