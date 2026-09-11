# P1344 — autoria independente do oráculo R1

## Veredito limitado

`FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED`.

Regime: **executado sem atestacao de isolamento**. O workspace é
compartilhado; a separação aplicada foi de papel, entradas congeladas, ordem e
allowlist de escrita. Esta autoridade não leu nem executou candidato produtivo,
não emitiu selo e não executou o corpus completo.

## Raízes autorais

- corpus canônico: `p1344-oracle-corpus-r1.json`, SHA-256
  `7ce12848ce375ab21f5ca9e112e5b653d750fdd9e4a7daf93c9d8d1e7d0aa0d6`;
- source verifier: `p1344-source-verifier-r1.py`, SHA-256
  `83a0021d504c22ddf1591210015c7d451162789b1dd0256e0299d78e9e7fc9ea`;
- checker: `p1344-oracle-checker-r1.py`, SHA-256
  `375602081721b368aa0bd032738392ee7fae2ff429183d1f04fd4f7f7144c564`;
- authority root segundo a fórmula contratual:
  `e4b4d3d6ba6da88824d2e594032a5949c88d78e011c5f8cfb0644949b494f75b`.

O checker aceita apenas o path canônico e o hash literal do corpus; o source
verifier também compila esse hash. JSON rejeita chaves decodificadas duplicadas
em qualquer profundidade e números não finitos. Um corpus/checker/root
autoconsistente escolhido pelo atacante não constitui autoridade.

## Cobertura composta, sem execução full nesta autoria

O corpus fecha 139 identidades:

- 85 casos do corpus P1343 R1;
- 11 casos `ADV01`–`ADV11` do P1343 R2, sem duplicar controles herdados;
- 20 classes `R2A01`–`R2A20` do relatório adversarial P1343 R2, incluindo os
  dezesseis sobreviventes que motivaram a gramática fechada;
- 23 casos P1344 owner-local/autoridade: 2 positivos, 1 opaco e 20 negativos.

A rota `--full` do checker reexecuta o predecessor protegido e os recortes
P1344 em normal, repeat e reverse. Ela não foi chamada pelo autor; pertence ao
pré-verificador independente e não se sela a si própria.

## Resultado focal

Comando autoral final:

```text
python3 -B 00_nucleo/diagnosticos/p1344-oracle-checker-r1.py --focus --corpus 00_nucleo/diagnosticos/p1344-oracle-corpus-r1.json
```

Resultado: 23/23 classificações concordantes; 20/20 negativos `Violated`;
2/2 controles positivos `Preserved`; 1/1 opaco `Unknown`; zero sobreviventes;
mutation score focal `1.0`; `full_corpus_runs = 0`. O probe de validade
sintática parseou com rustfmt os três owners (`Func`, `Content`,
`CounterUpdate`) em `/dev/shm`.

Durante a calibração foram medidos dois deltas discriminatórios legítimos:

1. uma detecção excessiva de `cfg!` tratava qualquer `!` distante como o
   operador de `cfg`; corrigida para a sequência lexical adjacente `cfg !`;
2. uma função aninhada dentro de helper owner-local sobreviveu; a produção foi
   fechada contra item `fn` no corpo do método.

Nenhum negativo foi enfraquecido e `Unknown` permaneceu exclusivo ao payload
opaco. Sete invocações focais foram usadas no total, incluindo duas falhas de
infraestrutura de captura de exceção; isso permanece dentro do budget autoral
de oito. Não houve execução completa.

## O que a fonte prova e não prova

O source verifier compõe o baseline P1343 de 36 cápsulas com o overlay P1344,
exige 38 cápsulas em 12 consumers, markers lexicais reais, anchors adjacentes,
normalização inversa dos arquivos completos e placement físico dentro de
`impl Func`, `impl Content` e `impl CounterUpdate`. Os três conjuntos de
métodos são exatos e disjuntos; nested/nonlocal impl, close/reopen,
wrapper/const/macro, trait, import/reexport, método no owner errado e 13º
consumer falham fechados.

Isso não prova reachability, freshness, identidades, cardinalidades ou
semântica runtime. A aceitação futura continua exigindo teste Rust P1344
independente e recibo real, compilado e desafiado. Runtime sintético é aceito
somente dentro do corpus discriminatório.

## Próximo gate

Um adversário independente deve atacar estes hashes exatos sem corrigir os
artefatos julgados. Zero sobreviventes é pré-condição para o pré-verificador
executar a única passagem full normal/repeat/reverse e exigir score `1.0` antes
de qualquer selo.
