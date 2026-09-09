# P1334 — gate anterior ao candidato satisfeito

**Apto para C**, sobre manifesto
`2cac9aab9dc8e911e2a14e932515db3efd5be64b10b1089d06caac0b310be1da`
e freeze completo sucessor
`684143d5fdbb4005e5d641f772e603f48da1c32c928840c89873bbcff5fa50b5`.
Nenhuma objeção semântica restante anterior ao candidato. O parecer não
antecipa GREEN nem fechamento dos gates finais.

A revisão nativa R1 alterou exclusivamente a construção da fixture do
dispatcher para extrair Value::Module; diff inspecionado sem mudança de
asserção. Freeze nativo R1
`7ce3db16ae9d064eda15384bb40253130174300f26bee4c9d641edc868f409b9`
e integração R1
`bd00a92dac80e894e49be19d476b6e098bb50c91518feca474aa02401a9268eb`
conferem os hashes informados. Os artefatos originais permanecem como
registro da falha de compilação descrita em `p1334-review-tests.md`.

RED R1 **compilou e executou**. `p1334-unit-red-r1.json`, SHA-256
`8b1f6687fa1e0a934b12176f70afd08faaa6eb1f68ab6fa34b34f0d17a377799`,
registra `cargo test --release --locked -p typst-core p13 -- --nocapture`,
UTC `2026-09-09T16:05:10.306709+00:00` a
`2026-09-09T16:07:02.134564+00:00`, exit 101: 217 testes executados,
204 passaram e 13 falharam. As falhas inspecionadas são as diferenças
esperadas de mensagem missing/sobras/hint e agregado abs, inclusive controles
históricos migrados. Inventários produtivos antes/depois coincidem; recibo
identifica HEAD, árvore não commitada, diff/stat e manifesto corretos.
O primeiro `unit-red.json` permanece compile failure, não é somado ao RED.

O freeze completo original, SHA-256
`497755c551dc85205c80922160e4f06498040f8d08bea3f8369b9aecbbe83ad2`,
conserva 816 células CLI. O sucessor acrescenta apenas 12 células de import
real de `p1334-ab-fixtures/origins.typ`: primeiro named value duplicado por
With; primeira sobra posicional em arguments; primeira sobra named em With.
Conferi todos os hashes inputs/artifacts do freeze sucessor e li runner,
fixture e referências do suplemento. Cada expressão chama uma função/Args
importada; o diagnóstico vanilla aponta o arquivo de origem e conserva
trace abs na expressão chamadora. As expectativas são literalmente os
campos exit/stdout/stderr vanilla, sem normalização do candidato.

`p1334-ab-cross-baseline.json` tem SHA-256
`b2296deda368661b6fb68e099e93a2f011a622a4f188fb7477bcc6eef3094fcf`;
`p1334-ab-cross-expected.json` tem SHA-256
`4b3a1b9f8bea86c3e15e354ce4319d46110b36ff555ef93e28aa6b2a8756332a`.
O baseline suplementar preserva argv/UTC por célula, perfis e binários
pinados do baseline público. Os testes nativos entre FileIds distintos
ficam complementados por essa cobertura pública de Sources reais.

A sequência verificável agora é L0 → freeze nativo → falha de compilação
da fixture → revisão somente da construção → RED executado → freeze CLI
complementado antes de C. Implementação ainda precisa preservar normas e
testes congelados, passar GREEN, build/workspace/fmt/lint/linhagem e ambos
os corpora CLI normal/repetido/invertido. Unknown obrigatório bloqueia.

Executado sem atestação técnica de isolamento em filesystem compartilhado;
sem selo de refinamento ou mutation score. Escrita do revisor somente neste
parecer. Incidente anterior de nomes restritos permanece registrado em
`p1334-review-scope.md`; não li passos históricos.
