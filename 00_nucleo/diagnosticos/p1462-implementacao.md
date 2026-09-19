# P1462 — relatório do implementador de `format.svg.pretty`

## Papel, autoridade e regime

- Papel executado: implementador segregado.
- Regime: completo, conforme `tekt-materializacao-segregada`.
- Commit causal: `c5bef1306c8c0eaaf3e5b49e1d98ca7d5fd86d97`.
- Autoridade de execução: `00_nucleo/diagnosticos/p1462-recovery3-preseal.json`, estado `SEALED`.
- Contrato público: `00_nucleo/diagnosticos/p1462-r2-contrato-congelado.md`.
- Foram lidos integralmente os Prompts L0 proprietários afetados, os ADRs
  0107, 0108, 0109, 0127, 0129 e 0131, e `01_core/CLAUDE.md` antes da
  implementação.
- Não foram lidos nem executados o teste A/B mantido, o recibo RED protegido,
  oráculos independentes, catálogos/resultados de mutantes, bundles de mutação,
  recibos privados ou P00.

Atestação: **executado sem atestação de isolamento**.

## Estado inicial observado

Antes da primeira edição, `git rev-parse HEAD` devolveu o commit causal acima.
`git status --short` mostrava somente estes artefatos preexistentes e não
rastreados, ambos protegidos e não abertos:

```text
?? 00_nucleo/diagnosticos/p1462-red-test.json
?? 04_wiring/tests/p1462_format_svg.rs
```

## Materialização

Foram alterados exclusivamente os owners produtivos autorizados:

- `01_core/src/compiler/stdlib/format.rs`: filho público nominal `format.svg`,
  namespace `pretty: false` e chamada direta recusada.
- `01_core/src/compiler/eval/rules.rs`: reconhecimento estrutural exato da set
  rule, named opcional, neutralidade da regra vazia e diagnósticos explícitos.
- `01_core/src/compiler/layout/format_options.rs`: novo coletor puro L1,
  travessia root por `Sequence`/`Styled`, ordem last-write-wins, validação
  fail-closed e rejeição de decisão aninhada.
- `01_core/src/compiler/layout/mod.rs`: reexport do coletor.
- `01_core/src/entities/document_format_options.rs`: novos carriers fechados.
- `01_core/src/entities/mod.rs`: registro do módulo de carriers.
- `01_core/src/entities/layout_types.rs`: transporte em `PagedDocument` com
  default `false`.
- `03_infra/src/pipeline.rs`: coleta sobre o conteúdo realizado final, ligação
  ao documento, API `Option<bool>`, precedência e testes internos.
- `04_wiring/src/main.rs`: preservação de `CompileIntent.pretty` somente no
  braço SVG; PDF/HTML continuam com o booleano de P1360 e PNG permanece neutro.

Os dois novos consumers usam `@prompt-hash 00000000`. Nenhum Prompt L0,
contrato, teste mantido ou diagnóstico alheio foi editado. Não foi executado
`--fix-hashes`; o resselo pertence ao integrador.

## RED → GREEN interno

O teste unitário da factory foi atualizado antes da lógica produtiva. Uma
primeira tentativa com filtro `--exact` selecionou zero testes e foi descartada
como evidência. A execução válida foi:

```text
cargo test -p typst-core p1462_format_module_exposes_only_svg_with_pretty_default_false -- --nocapture
```

Resultado RED: exit `101`, falha em `scope.len()`, `left: 0`, `right: 1`.

Após a implementação, os comandos focais finais foram:

```text
cargo test -p typst-core p1462_ --lib
# 7 passed; 0 failed

cargo test -p typst-infra p1462_ --lib
# 6 passed; 0 failed

cargo build -p typst-wiring
# sucesso

git diff --check
# sucesso, sem saída
```

Os testes internos cobrem factory/default, carriers, travessia root, ordem e
repetição, regra vazia neutra, dois containers puros distintos, nested real no
pipeline, valor interno inválido, diagnósticos da set rule, construção manual,
as quatro classes da precedência e a diferença estrutural compacto/pretty do
SVG externo.

A formatação seletiva foi feita com `rustfmt --edition 2021` somente nos nove
arquivos produtivos autorizados. A formatação mecânica alheia à mudança em
`pipeline.rs` foi restaurada para preservar o diff do owner.

## Proveniência das medições finais

Medição em `2026-09-19T17:42:55-03:00`, no HEAD
`c5bef1306c8c0eaaf3e5b49e1d98ca7d5fd86d97`, com working tree não commitado.
Imediatamente antes de redigir este próprio relatório, `git diff HEAD --stat`
registrou:

```text
01_core/src/compiler/eval/rules.rs    |  44 ++++++++
01_core/src/compiler/layout/mod.rs    |   3 +
01_core/src/compiler/stdlib/format.rs |  48 +++++++-
01_core/src/entities/layout_types.rs  |   4 +
01_core/src/entities/mod.rs           |   1 +
03_infra/src/pipeline.rs              | 199 ++++++++++++++++++++++++++++++++--
04_wiring/src/main.rs                 |  21 +++-
7 files changed, 302 insertions(+), 18 deletions(-)
```

Como `git diff HEAD --stat` não enumera novos arquivos não rastreados, o mesmo
snapshot tinha adicionalmente os dois novos owners
`01_core/src/compiler/layout/format_options.rs` e
`01_core/src/entities/document_format_options.rs`, além dos dois artefatos
protegidos preexistentes listados no estado inicial. Este relatório é o único
arquivo adicional criado depois dessa medição.

## Limites do resultado

- Não houve execução nem inspeção do teste A/B mantido ou de qualquer artefato
  proibido; portanto este relatório não alega certificação A/B independente.
- Não houve mutation testing, leitura de oráculos privados nem veredito de
  auditoria; esses papéis permanecem segregados.
- `crystalline-lint .` não foi executado nesta etapa porque os novos headers
  ainda estão intencionalmente com sentinel e a árvore contém o teste protegido
  fora da autoridade de leitura do implementador. O integrador deve ressellar
  hashes e executar os gates arquiteturais/finais.
- Os builds emitiram warnings preexistentes do workspace, sem erro novo.

