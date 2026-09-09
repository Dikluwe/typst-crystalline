# P1330 — candidato e GREEN focal

**PASS_CANDIDATE** no recorte L0 R2. Revisor `/root/p1330_review`, A/B
sem atestação de isolamento, sem selo. Gates finais ainda pendentes.

Conferência de fonte em `2026-09-09T13:20:54.996Z`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
Owner candidato SHA-256
`84dc0e8d42225e1f5955bf20d8ba3bd8bb7ea3628e03d5df6d03df806f787c36`.
Reconstrução em memória demonstrou: o owner inteiro corresponde exatamente
ao baseline com sucessor P1328 e snippet P1330 congelados, substituindo
apenas o braço Int de calc_abs e excluindo somente metadata @prompt-hash.
Não há mudança nos outros ramos, helpers, registro, assinatura ou P1329.

O braço Int aplica módulo verificado e retorna Int exato quando possível.
Na falha produz um único SourceDiagnostic::error com texto normativo,
busca primeira ocorrência posicional e usa diretamente value_span,
inclusive detached; ausência mantém detached. Não adiciona hint ou trace.
O guard named e o match de aridade permanecem anteriores ao diagnóstico.

GREEN recebido: `p1330-unit-green.json`, SHA-256
`73bed500fb3507fee378861532fddf285fb98fc7bac777d7134ace7fadcb81a9`.
Execução `cargo test --release --locked -p typst-core
compiler::stdlib::calc::p13 -- --nocapture`, de
`2026-09-09T13:20:14.215749+00:00` a
`2026-09-09T13:22:00.331156+00:00`, exit 0:
23 passaram, nenhuma falha. São os mesmos testes do RED, incluindo os
sucessores e as rotas posteriores antes interrompidas pela primeira falha.
Inventário produtivo before/after idêntico e manifesto R2 correto.
Todos os seis artefatos congelados no recibo de integração conservam hashes.

O recibo contém HEAD/snapshot/diff/stat exatos. Perante o RED, apenas o
braço Int muda: calc.rs passa a 1263 no stat, total
`14 files changed, 3651 insertions(+), 108 deletions(-)`; demais arquivos
são os do inventário anterior preservado. Este GREEN precede o resselo
recíproco final; portanto não autoriza reutilizar seu inventário como se
fosse o estado final. Nova validação focal após resselo, build/workspace,
CLI nas três ordens, fmt, lint, linhagem e histórico seguem necessários.

Nenhum artefato julgado foi alterado pelo revisor. A confirmação é local
ao overflow e à preservação testada, não paridade geral de abs/calc.
