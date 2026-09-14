# P1359 — preflight de `--pdf-tagged`

## Proveniência

- medição: 2026-09-12T22:05:04-03:00;
- HEAD: `8ceea20f92dff06b6ca5a34828135c0fd3e35248`;
- estado: working tree não commitado; `git diff HEAD --stat` registrava os seis
  arquivos já alterados por P1356–P1358, total 404 inserções e 116 remoções;
- vanilla ratificado: commit
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`, binário
  `lab/typst-original/target/release/typst`, SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino medido: `target/release/typst`, SHA-256
  `0139533ac8dc5b7e79d4476e13be39b1c493c3d2c2dc684f876096f476377de9`.

## Medição anterior à decisão

O backlog P1357 atribui 32 células do grupo `public_cli_pretty_pdf_tagged` ao
owner `00_nucleo/prompts/shell/cli.md`. Os dois nomes não representam o mesmo
eixo e foram novamente separados antes de decidir implementação.

No vanilla ratificado, `compile probe.typ out.pdf --pdf-tagged`,
`--pdf-tagged=true` e `--pdf-tagged=false` terminam com exit 0. No cristalino,
as três formas terminam com exit 2 e `unexpected argument '--pdf-tagged'`.
O efeito é observável na morfologia PDF: ausência, bare e `true` habilitam a
estrutura lógica; `false` a desabilita.

A fonte ratificada em `lab/typst-original/crates/typst-cli/src/args.rs:360-380`
define `Option<bool>` com `default_missing_value = "true"`, `num_args = 0..=1`
e `require_equals = true`. Em `compile.rs:153-184`, o valor explícito vence o
legado `--no-pdf-tags`; ausência mantém tags habilitadas, salvo o legado. As
interações com `--pages` e standards acessíveis dependem de capacidades ainda
ausentes no cristalino.

No cristalino, `02_shell/src/cli.rs:204-210,388-394,595-601` já transporta
`--no-pdf-tags` até `CompileIntent`. `04_wiring/src/main.rs:323-337` já traduz
essa polaridade para `PdfTags::{Enabled,Disabled}`, e todas as entry points PDF
já recebem o enum. Portanto `--pdf-tagged` não exige mudança de exportador,
fase ou entidade.

`--pretty` é estruturalmente distinto: a fonte ratificada o encaminha a PDF,
HTML e SVG, mas não PNG (`args.rs:329-340`, `compile.rs:643-684`). O cristalino
tem mecanismos diferentes por target e `--compact` é uma decisão própria do
stream PDF, não sinónimo comprovado de `--pretty`. Misturar os dois neste passo
violaria a disciplina de medir antes de decidir.

## Classificação

- `--pdf-tagged`: gap de produto com owner e fio produtivo comprovados;
- `--pretty`: gap separado, reservado para P1360 após preflight por target;
- `--pages`, standards PDF acessíveis e warning de depreciação do legado:
  fora do recorte P1359, sem alegação de paridade.

## Decisão de recorte

P1359 materializa somente o protocolo tri-state de `--pdf-tagged` em compile,
watch e alias `w`, preservando `--no-pdf-tags` como compatibilidade. A ordem do
dono “Escreva o passo e implemente”, em 2026-09-12, confirma o gate ADR-0127
para o novo campo público de `CompileIntent` e a nova flag.

