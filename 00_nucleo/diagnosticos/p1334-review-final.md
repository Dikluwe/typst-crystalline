# P1334 — veredito final de revisão

**PASS_SCOPED** — candidato e evidências satisfazem o recorte congelado de
diagnósticos da assinatura nativa de abs. Sem objeção pendente para executar
o fechamento. Não é atestação de isolamento nem declaração de paridade geral.

A inspeção produtiva anterior permanece válida: somente guards de abs,
whitelist por ponteiro e reexport interno mudaram. Nenhum erro do primeiro
valor, fórmula, entidade, resolução math, eager ou trace_call foi alterado.
A auditoria final em `2026-09-09T16:19:41.080Z` confirmou os inventários
antes/depois de todos os gates iguais à árvore produtiva atual, e todos os
hashes históricos preservados. Normas e testes congelados não mudaram.

Gates conferidos:

- RED válido R1: 204 passaram e 13 falharam na obrigação nova; a tentativa
  original permaneceu falha de compilação da fixture. O sucessor corrigiu
  somente extração Value::Module, sem trocar expectativas.
- GREEN dos mesmos testes: 217 passaram, nenhuma falha.
- Workspace: soma dos resultados publicados confirma 6734 passaram,
  nenhuma falha e 3 ignorados. O focal não é somado novamente.
- Build release/locked, fmt, diff check, linhagem efetiva/inversa dos três
  owners e V5/V15/V26 estrito: exit 0. Linter geral: exit 0 com avisos/info
  preexistentes. Recalculei a igualdade do multiconjunto integral das mensagens
  descontando somente coordenadas; não há novos apontamentos.
- CLI principal e suplemento entre arquivos: 816 + 12 células distintas
  em cada ordem normal/repetida/invertida. Recomparei exit/stdout/stderr
  candidato diretamente ao índice congelado, sem normalização, duplicata,
  omissão ou divergência. Os seis recibos públicos identificam o mesmo
  executável e estão ligados a wrappers de gate sobre a mesma árvore.

Recontagem independente do mesmo corpus: 628/828 observações BASE iguais
ao vanilla, 772/828 no candidato; 144 observações alteradas e nenhuma
regressão de paridade. As diferenças remanescentes são controles/dívidas
explicitados no relatório, incluindo resolução math importada. Não usar
esses números como percentual de toda a linguagem.

Li `p1334-final-report.md` e `p1334-ab-receipt.md`: concordam com os recibos
e delimitam evidência, falha inicial, domínio causal de Args e dívidas. O
recibo B PASS observa somente CLI público; esta revisão acrescenta os gates
produtivos/nativos. A restrição do papel B permanece explicitada, sem
confundir hashes de entradas com prova técnica de isolamento.

Revisei `p1334-close.py`: exige os gates finais sobre a mesma árvore,
freeze R1, norma/linhagem, histórico, candidato identificado, índices CLI
completos das duas suítes e pareceres antes de gravar closure. O campo
candidate_binary corresponde ao schema publicado; ausência de l0_norm no
suplemento é coberta pelo freeze e gate de linhagem, não convertida em
Unknown/sucesso implícito. O revisor não executou closure nem editou inputs.
O coordenador deve executar esse fechamento e verificar o estado posterior.

## Proveniência

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
Diff/stat e inventário exatos da medição estão em todos os gates root e no
relatório final. Binário candidato `/tmp/p1334-target.Ujq0xU/release/typst`,
SHA-256 `11e3164fa509030cc78dc048d5bb4f2426348e32a24edc7cd2f320c704e6ef61`.
Vanilla ratificado upstream/main `a51e02804`, binário pinado
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

```text
manifest      2cac9aab9dc8e911e2a14e932515db3efd5be64b10b1089d06caac0b310be1da
freeze-r1     684143d5fdbb4005e5d641f772e603f48da1c32c928840c89873bbcff5fa50b5
unit-green    5429f21f7a83b10a7a47c8606ebc4ea0cf075ac6fc97ea746e42c6c86a29a733
workspace     e6cdf31cf9bd0a23f11836896a8f673763d63a6c237fa0053d2517a7a0f01d5f
final-report  ca2d8080d08093a776416a66d844b1cf31ed887426b30060cf3c4afead3b4092
B-receipt     dbca4235dd7fd803d7d81547e5fbfedcd01c180d9386cd02208bd16f6b37c337
B-audit       a54393293fca5675e64375055ecd7d7d1b569e8ec5ffcecc9d12cbf386ff4f36
close-script  465d07da8d31bc5f466cd6d2d7d8981b9056cba7dedbb131f08dff6e5004d9c2
```

Regime A/B executado sem atestação técnica de isolamento, sem selo de
refinamento e sem mutation score. Fixtures Some incoerentes históricas
são somente robustez fora do domínio causal. Revisor escreveu apenas
`p1334-review-*`. Incidente inicial de exibição incidental de nomes
restritos permanece em `p1334-review-scope.md`; nenhum conteúdo histórico
das pastas restritas foi lido. Sem stage, commit ou push pelo revisor.
