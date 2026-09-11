# P1343 — blocker físico da gramática FINAL R2 para o oráculo R3

## Veredito

`CONTRACT_GRAMMAR_PHYSICALLY_UNSATISFIABLE_STOP`.

Regime: **executado sem atestacao de isolamento**. A autoridade de oráculos
parou antes de emitir source verifier, corpus, checker ou autoria R3. Nenhum
candidato foi lido, nenhum L0/baseline/código produtivo foi alterado e nenhum
corpus focal ou completo foi executado.

## Entradas decisivas

- contrato FINAL R2:
  `d4e444f6966a2be34018854c2b3857452447af1741e799e4cc2cfd6f31c48967`;
- binding FINAL R2:
  `216100a3970bdc79b3d77cd34f4528d46cd04e06dbdc9d63d552f723a557af58`;
- recibo FINAL R2:
  `013c7bb1b36a5a182a69dcb5c8451af3915239ed292c83682dc1655b51243637`;
- baseline das 36 cápsulas:
  `db143d9fec795ec81ce7a69be5e08b09e451a9d6eb07a1a39cd1dcd41ee82977`.

O contrato declara `contract_revisions_remaining = 0` e proíbe nova revisão
sem passo humano novo. Portanto a autoridade de oráculos não pode corrigir ou
reinterpretar a produção normativa.

## Contradição medida

A cápsula `P1343-FUNC-CARRIER-HELPERS`, em
`01_core/src/entities/func.rs`, tem owner protegido `impl Func`. A sua âncora
anterior termina dentro desse `impl`, imediatamente depois do método
`namespace`; a âncora posterior começa com `}`, que fecha o mesmo `impl Func`.
Logo todo o corpo da cápsula ocupa uma posição de **associated items dentro de
`impl Func`**.

O binding FINAL R2, porém, fixa para essa mesma cápsula:

- `syntax_form`: “finite inherent impl Func plus inherent impl Content plus
  inherent impl CounterUpdate”;
- `grammar_category`: `helper_impl`;
- a categoria exige exatamente os inherent impl blocks e métodos enumerados,
  sem wrapper, item extra ou forma alternativa.

Rust não permite declarar um `impl` dentro de outro `impl`. A prova focal com
`rustfmt --edition 2021 --emit stdout /dev/stdin` rejeitou `impl Func`,
`impl Content` e `impl CounterUpdate` nessa posição com
“implementation is not supported in traits or impls”, recomendando movê-los
para module scope.

## Alternativas examinadas e não autorizadas

- Emitir apenas métodos associados de `Func` na cápsula: é Rust válido, mas
  omite os `impl Content` e `impl CounterUpdate` exigidos literalmente.
- Fechar o `impl Func`, emitir os três impls e reabrir o owner: adiciona tokens
  estruturais/wrapper e altera a topologia fixa da cápsula, sem produção no
  binding.
- Usar `const`/bloco associado como container para items: introduz wrapper e
  associated const não enumerados pela gramática fechada.
- Mover os helpers para outra cápsula ou module scope: viola os 36 boundaries,
  owner/adjacência e a regra de composição preservada.
- Afrouxar a gramática do verificador ou tratar a frase apenas como intenção:
  seria uma revisão contratual clandestina, proibida pelo budget final.

Nenhuma alternativa é simultaneamente Rust-válida e autorizada pelos bytes do
contrato/binding FINAL R2. Assim, um positivo R3 não pode satisfazer ao mesmo
tempo a gramática e o gate sintático; escrever um checker para aceitá-lo seria
fabricar conformidade.

## Consequência operacional

- `full_corpus_runs = 0`;
- `focal_corpus_runs = 0`;
- source verifier/checker/corpus R3: **não emitidos**;
- nenhum selo, preseal, RED, implementação ou evidência de candidato existe;
- o rascunho incompleto `p1343-source-verifier-r3.py` foi removido via
  `apply_patch` e não deve ser tratado como artefato.

O próximo passo exige nova autorização humana capaz de rever o contrato ou a
topologia da cápsula. Sem essa autoridade, a parada é final para esta cadeia.

