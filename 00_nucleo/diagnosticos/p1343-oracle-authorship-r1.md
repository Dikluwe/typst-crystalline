# P1343 — autoria independente do verificador de source e dos oráculos R1

## Veredito desta autoridade

`FOCAL_PASS_NOT_VERIFIED_NOT_SEALED`.

Regime: **executado sem atestacao de isolamento**. O workspace é compartilhado;
a separação observada foi de papel, entradas, ordem e allowlist de escrita.

Esta autoridade escreveu somente o verificador de source, o checker do corpus,
o corpus e este par de artefatos de autoria. Não leu candidato futuro, não
escreveu código produtivo ou teste Rust, não executou o corpus completo e não
emitiu selo nem veredito de candidato.

## Entradas congeladas

- passo P1343: `6db7b3bb119f4038f88b8916209a4fcd7d1485b820143308ae788192a921a87b`;
- manifesto de autoridade: `d41aff08708b9fa72ac1d130ee71543f5048ebf2dc6944c25aba9a42b866dc3b`;
- baseline de 36 cápsulas: `db143d9fec795ec81ce7a69be5e08b09e451a9d6eb07a1a39cd1dcd41ee82977`;
- contrato P1343: `67107164ddb631e2c1539ca88c90658041811dc389bbae47cb2738312a5f3467`;
- binding P1343: `fbd5ff5f9f32594f95b6b76ea11b562fd1772172dde3a80d4083c9e401ccea85`;
- recibo do contrato: `dfccfd81e7254c5c22d3de9530a3fd55b267e906e503de62c27da91539026e15`;
- freeze L0: `2fb962c3edd8cdd83848d2cd7c9158c39a5d0218f510e81bb5e8011a540e8c0e`;
- fixture de 178 bytes: `98159f5ac529520590a197521dfb23383cec0ba6373b431b8b33f426cfc3a714`;
- contrato/binding P1342 R3: `07689c204f8741cdcfcc23bef9fa4af969dba6d991caaa93afaf117eb7ac73e7` /
  `8630a376350d374f854345c37282ac8fbb657f2f9a8506b7de47bda7b8276bf5`;
- checker/corpus P1342 R5 composto: `78ac4f374b52f3eeaaed6f65de004730693b4abea2150544607894194d00531a` /
  `3666955e792f61ff7cdeafe33b2b1b7c485102114089d6a6b34204ea8017e01f`.

## Hipótese e desenho

A hipótese foi que a regressão Y01 de R5 vinha exclusivamente da inferência de
alinhamento global. O R1 substitui essa inferência por uma operação fechada:

1. scanner lexical byte a byte reconhece somente comentários Rust físicos de
   linha completa; strings normais/raw/byte, chars, comentários de bloco
   aninhados e reconstruções textuais de macro não produzem marcador;
2. BEGIN/END são pareados por ID exato, sem desconhecido, duplicata,
   aninhamento, sobreposição, ausência ou arquivo alternativo;
3. âncoras são exigidas imediatamente antes/depois da região derivada;
4. o corpo é validado contra o cfg exato; inserts contêm uma única unidade
   dominada e replacements preservam o payload normal byte-exato;
5. cápsulas são substituídas em offsets live decrescentes e o hash do arquivo
   normalizado completo deve ser o hash candidate-free pinado;
6. hashes de arquivo/cápsula/corpo/âncora/símbolo-witness, offsets, rows,
   writer/callsites, projeções e freeze são derivados novamente dos bytes;
7. o checker recompõe o runtime P1342 protegido e compara o evidence fornecido
   com uma nova projeção derivada. Booleanos e hashes do envelope não provam
   nada sozinhos.

O writer é identificado como a única função de cápsula que contém a única
mutação `.push(...)`; toda row precisa alcançar pelo menos um callsite desse
writer em suas cápsulas ligadas. A fachada exata também é derivada do código.
O scanner continua focal e conservador, não é um parser Rust geral.

## Calibração focal

Somente `--focus` foi executado. Temporários ficaram em `/dev/shm`; nenhum
corpus completo, repetição global, ordem reversa global ou race foi executado
por esta autoridade.

Tentativas e delta discriminatório:

1. Primeira execução, 30 casos: 27/27 negativos `Violated`, mas P01, P02 e Y01
   regrediram pela mesma causa `CFG`: o verificador removia a indentação que
   fazia parte do replacement byte-exato. Nenhum selo era possível.
2. Revisão focal única dessa classe: passou a consumir somente o LF após o
   atributo normal, preservando todos os bytes do replacement. Resultado:
   2 `Preserved`, 1 `Unknown`, 27/27 negativos `Violated`.
3. Fronteira ampliada com C22 (node não guardado) e prova de unidade dominada:
   2 `Preserved`, 1 `Unknown`, 28/28 negativos `Violated`.
4. Hardening sem mudança de vetor: writer derivado por corpo e `.push`, uma
   única escrita direta, callsite por row e fachada nominal real. O mesmo vetor
   esperado permaneceu verde. A autoria para aqui; não há nova tentativa local.

Última execução focal: 31 casos, 28 negativos válidos rejeitados, mutation
score focal `1.0`, dois controles `Preserved`, um opaco `Unknown`, zero
survivors. Custo observado das quatro execuções focais: aproximadamente 32,3 s
de wall time, um processo Python por execução e diretórios efêmeros por caso em
`/dev/shm`.

Cobertura focal nova: marcador em string normal, raw string, char/byte-char,
comentário de bloco e macro-decoy; nested, duplicate, unknown, missing e
mismatched ID; replacement falsa; byte externo; cfg removido; branch normal
alterado; âncora/path/symbol/hash forjados; cápsula vazia; node insert não
guardado; Rust focalmente inválido com envelope `parse_ok`/`PASS`.

## Limite decisivo de runtime

`p1343_runtime()` existe **somente** dentro do checker como fixture sintética do
corpus discriminatório candidate-free. Não é evidência de produto e nunca pode
aceitar um candidato. O checker rejeita fail-closed qualquer invocação com
`--candidate-root` para impedir que esse runtime sintético seja combinado com
código produtivo.

Um candidato/final deve usar o CLI pinado do source verifier com runtime real
externo, produzido pela fachada Rust e pelo teste A/B, e ainda passar o checker
final integrador, `cargo test --no-run` com cfg de observação, `cargo build` sem
cfg e `rustfmt --check`. A saída do source verifier é necessária, mas isolada é
insuficiente para aceitar implementação.

## Saídas candidatas

- `p1343-source-verifier-r1.py` — SHA-256
  `07c5827a578ad298ba5f777d56395d9bc66eab156a9500a6730a5cdc335bf146`;
- `p1343-oracle-checker-r1.py` — SHA-256
  `ee8f8b5fa212753d1e90294ee6f6d1559ec9d717a667169f596e33c8b2b576e8`;
- `p1343-oracle-corpus-r1.json` — SHA-256
  `294e83c280086f64466651e9f5971e55968710dae639fdbaa6eb984cd2442ea1`.

Esses hashes ainda não são selo. A próxima autoridade é o adversário
independente; depois, e somente se não houver survivor, o pré-verificador pode
executar uma única vez o corpus completo e decidir `SEALED_FOR_IMPLEMENTATION`.

