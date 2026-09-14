# P1395 — relatório final de verificação

**Veredito:** `PASS_LIMITED_FRAGMENT`. O candidato corrige a precedência
lexical de callees `MathIdent` bare no fragmento selado: closures locais
`abs`/`sqrt` sombreiam as natives, o caso não-função preserva a morfologia e o
fragmento diagnóstico capturado, e a ausência de binding conserva os dois
controles nativos. Isto **não** é alegação de paridade math geral nem de
equivalência funcional geral.

Regime: protocolo completo da skill `tekt-materializacao-segregada`,
**executado sem atestação de isolamento**. Contrato, oráculos, adversário,
implementador e verificador tiveram autoridades de escrita distintas, mas o
workspace foi compartilhado. O verificador final escreveu somente este
relatório e `p1395-verification.json`; nenhum artefato julgado foi editado.

## Cadeia e integridade

Baseline causal: HEAD
`dd33334ca36e2d7abf79d16b200f07691d950c59`, com P1395 ainda não commitado.
O pré-selo R6 tem SHA-256
`e72f73ba38aad5e663b0948dc5ae2379f50215847870495dfa3e51d0417742a4`.
Os três inputs semânticos protegidos continuam exatamente nos hashes selados:

- oráculo CLI: `883ea48d230e95c56d6eaec44a0dc4cd73976793fefa8bb4089d1c09fe041b29`;
- recibo de oráculos: `e7b0f32a1240fa6b10e20f9003eec4fc02f814fd62892c70c4b4161ce4311d8b`;
- catálogo adversarial: `18621537f3a8340fb965cfffc9ba325a60ae67ce20f73aefde1cd1a46e6000f7`.

O `eval/tests.rs` guloso não foi executado e está restaurado byte a byte ao
baseline `95020fe449edece4e3c469dcd2efe306e3e6f1a6d3d4a03e91f28df22632e730`;
seu L0 também permanece no baseline
`5347098452ec77a72e8e0e969ad656b453c1df8ee04c3d2f28fb87ecfa84be33`.

R7 altera somente o timeout externo da retomada incremental, de 240 para 360
segundos. O contrato R7 referencia os hashes exatos do contrato e do passo R6
que aparecem no pré-selo; memória, paralelismo e entradas semânticas não
mudaram. Como as cópias autônomas R6 do contrato e do passo não permanecem na
árvore, essa transição é rastreada pelos predecessores registrados e pela
inspeção do conteúdo R7, não por um novo diff byte a byte.

O L0 final tem SHA-256
`2c80f156f83b6e0cc9d876dfba189770b494570cd52bc328117587b46713dc32`.
Substituir apenas `Hash do Código: 60a4f5cc` pelo valor pré-resselo
`fcd396d9` reproduz exatamente o L0 R6 selado
`6e25eefa26e0cf55410866bb4633bb325c5f728458c7039ab1dcce613e5e4d65`.
Logo a mudança pós-selo no L0 é somente a metadata de linhagem autorizada. O
candidato final `math.rs` tem SHA-256
`9658a77c3bfdc5b66e7cfab89999a9524d4776b2da92b6437a54f6c84226e94f`.

## Gates observáveis

O recibo GREEN `/tmp/p1395-green-r6.json`, SHA-256
`0bca007505173dab4a84016a4f596f8b8f0f23ae6aef5d11116008f261218304`,
identifica o binário final como
`5d0ff9dedaa3198217ac8b3a87fb91352d9beca1052e3f3c3b1a23a79030207a`.
O vetor foi `7 Preserved / 0 Difference / 0 Unknown`, idêntico na execução
direta, repetida e em ordem inversa:

- `LOCAL-ABS` e `LOCAL-SQRT` para as closures locais;
- `7(1, 𝑥)` para binding local não-função com posicionais;
- primary + hint de reparo corretos para named e spread;
- `|1|` e `√ 1` nos controles sem binding.

O gate consumiu no máximo 39.324 KiB de RSS e zero swap. O script executou
serialmente, com limite virtual externo de 512 MiB e timeout de 60 segundos.

A inspeção do candidato confirma que `get_local` ocorre antes do `match`
hardcoded, sem allowlist nominal; o miss cai no despacho nativo preexistente;
o branch não-função retorna antes de qualquer native; e o delta não altera
field access nem callees qualificados.

O gate mutacional final rejeita `14/14` mutações aplicáveis, score `1.0`, sem
sobrevivente ou `Unknown` inesperado. A base é o gate R6 estático/em memória,
repetido duas vezes em ordem normal e duas em ordem inversa, combinado aos
witnesses dinâmicos finais e aos guards estáticos do candidato. Nenhum mutante
de fonte foi persistido, compilado ou executado; o score deve ser entendido
nesse regime, não como campanha de binários mutados.

## Memória e arquitetura

Todos os gates pesados foram seriais, sob teto de dados de 2 GiB e zero swap:

| Gate | Exit | Pico RSS |
|---|---:|---:|
| primeiro build release incremental | 0 | 1.145.216 KiB |
| retomada incremental R7 | 0 | 694.080 KiB |
| `cargo check -p typst-core` | 0 | 822.828 KiB |
| resselo de hashes | 0 | 113.528 KiB |
| `crystalline-lint .` | 0, zero violações | 70.260 KiB |

Os números acima vêm dos recibos de execução entregues ao verificador; o
binário e a linhagem finais foram conferidos diretamente. `git diff --check`
também terminou com exit 0. Não houve teste workspace, rebuild completo nem
nova compilação do lib-test monolítico.

## Limitações sem crédito

- **Avaliação exatamente uma vez:** `NotObserved / INSUFFICIENT_PUBLIC_EVIDENCE`.
  O candidato contém uma única invocação visível do builder canônico no branch
  da função local, mas o oráculo público selado não possui witness independente
  de cardinalidade. Fica fora do denominador e recebe crédito zero.
- **Anotação secundária de span:** `NotObserved /
  INSUFFICIENT_PUBLIC_EVIDENCE`. O vanila ratificado exibe também ``abs is not
  a function`` como anotação do callee; o carrier diagnóstico simplificado e o
  extrator não a representam. Fica fora da equivalência diagnóstica integral e
  recebe crédito zero.

Assim, o P1395 fecha apenas o fragmento explicitamente observado e selado,
com gates funcionais, mutacionais, arquiteturais e de memória verdes.
