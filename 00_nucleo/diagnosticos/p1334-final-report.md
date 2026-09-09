# P1334 — argumentos de calc.abs: resultado e pendências

## Resultado

Implementados os diagnósticos que faltavam na assinatura da nativa `calc.abs`:
ausência de `value`, uso indevido de `value:` com hint e primeira sobra na ordem
conjunta de posicionais/nomeados. O erro aponta para a ocorrência inteira;
quando não há argumento, aponta para a chamada. As origens importadas e os
pré-argumentos de `with` são preservados. Um primeiro valor inválido continua
vencendo as sobras, sem alterar conversões, fórmulas ou espécies numéricas.

O dispatcher reconhece somente a identidade da nativa, inclusive através de
`With`; não decide mensagens nem inspeciona argumentos. A fachada recebeu
apenas reexport interno. Os três L0s foram atualizados antes do runtime e
resselados após a implementação; ownership 1:1 e núcleo existente preservados.

## Medição e alcance

No corpus congelado deste passo, a igualdade literal com o vanilla passou de
**628/828 para 772/828**:
144 observações mudaram e 0 regrediram de paridade.
São 816 células principais e 12 de import real entre arquivos. As expectativas
congeladas foram satisfeitas nas três execuções (normal, repetida e invertida).
Essas células incluem controles e dívidas preservadas: passar no corpus não
significa que todas as células sejam iguais ao vanilla, nem paridade geral.

O corpo de `calc.abs` foi corrigido apenas nos guards de ausência/sobra.
`Some(occurrences)` conserva ordem, duplicatas e origem; `None` sintético usa
posicionais antes dos nomeados, com spans individuais detached. Fixtures antigas
incoerentes não são prova de paridade; sua migração está no ledger independente.
Não foi criado fallback para um `Some` incoerente.

## Validação

- RED compilado: 204 passaram, 13 falharam nos casos previstos.
- GREEN focal: 217 passaram, 0 falharam.
- Workspace: 6734 passaram, 0 falharam, 3 ignorados; o focal está incluído, não se soma ao total.
- `cargo build --release --locked`, formatação e `git diff --check`: exit 0.
- Linhagem dos três owners e gate estrito V5/V15/V26: sem violações.
- Linter geral: exit 0, mas mantém avisos/info preexistentes. A comparação
  integral das mensagens, descontando somente coordenadas de linha, não encontrou
  novos apontamentos (`p1334-lint-comparison.json`). Não se declara zero avisos global.

A primeira tentativa RED não executou testes: o teste novo usava `scope()` em
um `Value` retornado por `make_calc_module`. O autor B corrigiu somente a
extração `Value::Module` em sucessor, antes de C; original e falha de compilação
ficaram preservados. A evidência RED válida é `p1334-unit-red-r1.json`.

## O que ainda falta

Este passo fecha os diagnósticos da assinatura nativa no domínio coerente testado.
A resolução de `abs` importado em math permanece fora do recorte: não foi
contornada por spelling nem por mudança global no dispatcher. Persistem também
as outras dívidas carregadas como controles. Casos ainda diferentes neste corpus:

- `fallback-path-construction-debt`: `calc.abs(path())`
- `name-gradient-conic`: `gradient.conic(red,blue,space:calc.abs)`
- `name-gradient-linear`: `gradient.linear(red,blue,space:calc.abs)`
- `name-gradient-radial`: `gradient.radial(red,blue,space:calc.abs)`
- `name-math-import`: `{import calc: abs; $abs(-1)$}`
- `name-other-sqrt`: `{let a=calc.sqrt.with("x"); a()}`
- `name-show`: `{show calc.abs: it => it; [x]}`
- `name-where`: `calc.abs.where()`
- `parser-direct-minimum-debt`: `calc.abs(-9223372036854775808)`
- `precedence-math-import-known-debt`: `{import calc: abs; $abs(-1, 2)$}`
- `prior-float-fraction`: `calc.abs(-calc.inf * 1fr)`
- `signature-foreign-sqrt`: `calc.sqrt()`
- `signature-imported-math-debt`: `{import calc: abs; $abs()$}`
- `sqrt`: `calc.sqrt([x])`

Esses casos são observações específicas, não inventário exaustivo da linguagem.
O próximo recorte deve medir a rota de resolução math e o respectivo L0 antes de
decidir sua correção; nenhum passo futuro foi implementado implicitamente.

## Proveniência e reprodução

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, **working tree não commitado**. Todas as medições trazem
inventário SHA-256, diff/stat, argv, início/fim UTC e estado antes/depois nos
recibos privados `p1334-*.json`; os recibos CLI públicos identificam o executável.
Build: 2026-09-09T16:13:22.677126+00:00 → 2026-09-09T16:15:36.537368+00:00.
Workspace: 2026-09-09T16:13:27.613903+00:00 → 2026-09-09T16:18:43.100892+00:00.
Binário candidato `/tmp/p1334-target.Ujq0xU/release/typst`:
`11e3164fa509030cc78dc048d5bb4f2426348e32a24edc7cd2f320c704e6ef61`.
Vanilla ratificado upstream/main `a51e02804`, `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
BASE P1333 SHA-256 `79470612fc121fa42a6898846f85d0b0325edf517c90a830c01cde947f748ffe`.
As referências CLI foram executadas antes de C e reutilizadas por cache imutável,
com hashes e UTC originais verificados; cada rodada executou novamente o candidato.

Estado rastreado exato no momento da validação (inclui alterações herdadas):

```text
 .../prompts/compiler/eval/bindings/field_access.md |  187 +-
 00_nucleo/prompts/compiler/eval/call_dispatch.md   |   88 +-
 00_nucleo/prompts/compiler/eval/modules.md         |   47 +-
 00_nucleo/prompts/compiler/eval/tests.md           |   78 +-
 00_nucleo/prompts/compiler/stdlib/_comum.md        |   25 +-
 00_nucleo/prompts/compiler/stdlib/calc.md          |  542 +++-
 00_nucleo/prompts/compiler/stdlib/loading.md       |  198 +-
 00_nucleo/prompts/wiring.md                        |  104 +-
 01_core/src/compiler/eval/bindings/field_access.rs |  731 ++++-
 01_core/src/compiler/eval/call_dispatch.rs         |  205 +-
 01_core/src/compiler/eval/modules.rs               |    5 +-
 01_core/src/compiler/eval/tests.rs                 |  236 +-
 01_core/src/compiler/stdlib/calc.rs                | 3067 +++++++++++++++++++-
 01_core/src/compiler/stdlib/loading.rs             |  466 ++-
 01_core/src/compiler/stdlib/mod.rs                 |    3 +-
 04_wiring/src/main.rs                              |   50 +-
 16 files changed, 5914 insertions(+), 118 deletions(-)
```

Reprodução: usar `CARGO_TARGET_DIR=/tmp/p1334-target.Ujq0xU` nos comandos cargo;
executar `p1334-ab-cli.py` e `p1334-ab-cross-cli.py` com `--candidate` apontando
para o binário acima, `--output` novo e `--order normal` ou `reverse`.
Não sobrescrever recibos congelados. O manifesto, freezes, métricas e closure
registram os hashes necessários para verificar a mesma árvore e os artefatos.

## Independência e limites

A skill `tekt-materializacao-segregada` orientou a separação entre implementação,
testes/expectativas pré-C e revisão. Regime A/B **sem atestação técnica de isolamento**,
sem selo de refinamento e sem alegação de mutation score. Testes B não receberam
código candidato nem resultados nativos privados; o veredito CLI usa saídas públicas.
Pareceres: `p1334-review-candidate.md`, `p1334-review-final.md` e
`p1334-ab-receipt.md`. O fechamento verificável está em `p1334-closure.json`.
Histórico e alterações herdadas preservados. Sem stage, commit ou push.
