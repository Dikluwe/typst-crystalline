# Prompt L0 — integração de `format.pdf.standard` / `compile --pdf-standard`

Hash do Código: a1b2c3d4

**Estado:** MATERIALIZADO. Contrato de teste escrito pelo Agente B (RED);
implementação do Agente A integrada pelo papel C; 10/10 testes GREEN sem
regressão na suíte de PDF. Ver
`00_nucleo/materialization/typst-passo-1469.md` para a proveniência
completa.

**Camada:** L4 (testes de integração)
**Alvo planejado:** `04_wiring/tests/p1469_format_pdf_standard.rs`
**Consumer futuro exclusivo:** `04_wiring/tests/p1469_format_pdf_standard.rs`
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Escopo reduzido — decisão do dono, 2026-09-21

Ver `entities/document_format_options.md` P1469 para a nota completa: não há
validador de conformidade PDF/A/PDF/UA disponível neste ambiente (sem
veraPDF, sem `pikepdf`), e o vanilla delega essa mecânica inteira à crate
`krilla`, que o cristalino não usa. Este ficheiro materializa **somente o
contrato de linguagem**: parsing dos 17 nomes, validação de conflitos entre
standards, cálculo da versão efetiva do PDF, ajuste do header
`%PDF-{version}`, e a interação `standard`+`tagged`/`pages` já materializada
em P1467/P1468. **Não** verifica identificação XMP com nível de conformidade,
restrições de cor/fontes/transparência, contraste de acessibilidade, nem
qualquer outra estrutura interna do PDF específica de cada perfil — isso é
scope-out explícito (mesmo escopo reduzido de
`infra/export/builder.md` P1469).

## Responsabilidade

Executar o binário cristalino como processo externo, compilando PDFs de
verdade (sem mocks), e provar a fatia comportamental de linguagem de
`pdf.standard`: o campo documental `pdf.standard` (`#set format.pdf(standard:
...)`), a flag `--pdf-standard`, a precedência CLI > documento, o cálculo da
versão efetiva do header `%PDF-{version}` (explícita, ou máximo do range do
perfil quando ausente), a interação `standard`+`tagged` no canal documental
(mensagem `"cannot set..."`) e no canal CLI-only (mensagem `"cannot
disable..."`, com hint de `--pages` quando a causa raiz foi `pages` sem
decisão explícita de `tagged`), e a validação de conflito entre nomes na
própria lista `--pdf-standard` (mesma função reaproveitada de
`compiler/eval/rules.rs` P1469). Não duplica a cobertura de conteúdo/páginas/
bookmarks já fechada em `wiring/tests/p1468-format-pdf-pages.md`; aqui o eixo
é exclusivamente `standard` e sua interação com `tagged`/`pages`. Não alega
conformidade PDF/A ou PDF/UA estrutural real.

## Contrato coberto

- `#set format.pdf(standard: "1.4")` produz um PDF cujo header começa
  exatamente `%PDF-1.4`.
- `#set format.pdf(standard: "a-2b")` (sem versão explícita) produz header
  `%PDF-1.7` (máximo do range medido de `a-2b`, 1.4..=1.7).
- `#set format.pdf(standard: "a-1b")` (sem versão explícita) produz header
  `%PDF-1.4` (range de `a-1b` é 1.4..=1.4, min == max).
- `--pdf-standard=2.0` sem decisão documental produz header `%PDF-2.0`.
- `--pdf-standard=a-4` (CLI, efetivo 2.0) vence uma decisão documental
  `standard: "a-1b"` (efetivo 1.4) divergente — header final `%PDF-2.0`
  (precedência CLI > documento).
- Documento sem nenhuma decisão de `standard` (nem documental, nem CLI)
  produz header `%PDF-1.7` — regressão zero face aos testes pré-existentes
  de `export/tests.rs` que fixam esse default.
- `#set format.pdf(standard: "a-1a", tagged: false)` falha a compilação com
  a mensagem documental `"cannot set \`pdf(tagged: false)\` when exporting a
  {name} document"` (o nome exato do perfil na mensagem não é fixado por
  este L0 — só a forma estável do texto; a substituição exata é
  responsabilidade de `compiler/eval/rules.rs`/`infra/pipeline.md` P1469).
- `--pdf-standard=a-1a --pdf-tagged=false` falha ANTES de compilar com a
  mensagem CLI-only exata `"cannot disable PDF tags when exporting a
  PDF/A-1a document"` (texto distinto da mensagem documental, sem hint —
  medido via `typst compile` em 2026-09-21), exit 1.
- `--pdf-standard=ua-1 --pages=1` (sem `--pdf-tagged` explícito) falha ANTES
  de compilar com a mensagem CLI-only exata `"cannot disable PDF tags when
  exporting a PDF/UA-1 document"` COM o hint `"using --pages implies
  --pdf-tagged=false"` (causa raiz foi `pages`, sem decisão explícita de
  `tagged`), exit 1.
- `--pdf-standard=1.4,1.7` (dois nomes de versão conflitantes na própria
  lista CLI) falha ANTES de compilar com a mensagem de conflito reaproveitada
  de `compiler/eval/rules.rs` P1469, `"PDF cannot conform to PDF 1.4 and PDF
  1.7 at the same time"`, exit 1.

Inspeção pelos primeiros bytes do ficheiro (prefixo `%PDF-{version}` até o
próximo `\n`/`\r`), nunca igualdade integral de bytes do resto do PDF
(ADR-0107, ADR-0108) — o resto da estrutura (xref, tipos de objeto,
`/Catalog`) não muda com a versão declarada (ver `infra/export/builder.md`
P1469, scope-out explícito).

## Proveniência da medição RED

Medido contra o baseline causal `85b7fc5f1eda67e89494a478c2458de36d4ee20d`
(P1468 materializado) mais os 9 Prompts L0 de P1469 (estado descrito em
`00_nucleo/materialization/typst-passo-1469.md`): neste baseline,
`PdfFormatOptions` não tem campo `standard`; `02_shell/src/cli.rs` mantém
`--pdf-standard` como `_pdf_standard: Option<String>` oculto e descartado
(nunca lido, nunca transportado a `CompileIntent`); o exportador escreve o
literal fixo `b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n"` como único ponto de escrita
do header PDF. Todos os casos deste ficheiro são RED esperado contra esse
baseline — a suite inteira falha por erro de compilação de `02_shell`/
`03_infra`/`04_wiring` (campo/tipo/função ausentes: `PdfStandard`,
`PdfStandards`, `options.pdf.standard`, `CompileArgs.pdf_standard`,
`CompileIntent.pdf_standard`, a nova entry point de pipeline) até a
implementação (Agente A) e `crystalline-lint --fix-hashes .`
materializarem o contrato.

## Controles de fronteira

- `format.svg`, `format.png`, `format.html`, `pdf.tagged`/`pdf.pretty`/
  `pdf.pages` isolados (sem `standard`) e seus defaults permanecem fora de
  escopo — nenhum caso deste ficheiro os exercita além do necessário para
  isolar `standard`.
- Conformidade estrutural PDF/A ou PDF/UA real (identificação XMP com nível
  de conformidade, restrições de cor/transparência/fontes, validação por
  ferramenta externa) fica fora — scope-out explícito de
  `entities/document_format_options.md` P1469 e
  `infra/export/builder.md` P1469 (sem validador disponível neste
  ambiente).
- Qualquer mudança estrutural do PDF específica de cada versão além do
  header declarado (xref stream obrigatório em 2.0, etc.) fica fora —
  scope-out explícito de `infra/export/builder.md` P1469.
- Fixtures descartáveis usam exclusivamente uma raiz em `/dev/shm` via
  `TMPDIR`, sem fallback ao disco (mesma disciplina de
  `wiring/tests/p1468-format-pdf-pages.md`); ausência de `TMPDIR` explícito
  é falha de configuração do harness, não do contrato.
- Execução serial (`--test-threads=1`) e timeout finito por processo.

## Gate

Escrito pelo papel B (Agente B) sem acesso ao código de implementação do
papel A, a partir apenas dos oito Prompts L0 do gate ADR-0127 de P1469
(`entities/document_format_options.md`, `compiler/eval/rules.md`,
`compiler/layout/format_options.md`, `shell/cli.md`, `infra/pipeline.md`,
`infra/export/mod.md`, `infra/export/builder.md`, `wiring.md`) e do passo
`00_nucleo/materialization/typst-passo-1469.md`. O hash acima é placeholder
até `crystalline-lint --fix-hashes .` e a confirmação humana explícita de
materialização (que substitui o texto deste cabeçalho por "MATERIALIZADO" e
regista o par de commits RED/GREEN, seguindo o padrão de
`wiring/tests/p1468-format-pdf-pages.md`).
