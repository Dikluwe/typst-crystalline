# Prompt L0 — integração de `format.pdf(tagged, pretty)`

Hash do Código: aaaaaaaa

**Estado:** CONTRATO DE TESTE P1467 ESCRITO (Agente B, RED); AGUARDA
IMPLEMENTAÇÃO (Agente A) E `crystalline-lint --fix-hashes .` para selar o
hash bidirecional. Este L0 nasce já depois do commit causal somente-L0 de
P1467 (`compiler/eval/rules.md`, `compiler/layout/format_options.md`,
`entities/document_format_options.md`, `infra/pipeline.md`, `wiring.md`)
descrito em `00_nucleo/materialization/typst-passo-1467.md`.

**Camada:** L4 (testes de integração)
**Alvo planejado:** `04_wiring/tests/p1467_format_pdf_tagged_pretty.rs`
**Consumer futuro exclusivo:** `04_wiring/tests/p1467_format_pdf_tagged_pretty.rs`
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Responsabilidade

Executar o binário cristalino como processo externo, compilando PDFs de
verdade (sem mocks), e provar a primeira fatia comportamental de
`format.pdf`: os campos `tagged` (`auto`/`bool`) e `pretty` (`bool`), sua
resolução via `#set format.pdf(...)` no documento, e a precedência das flags
`--pdf-tagged`, `--no-pdf-tags` (legado) e `--pretty` sobre essa decisão
documental. Não duplica a superfície/gate de `a11y-extras` nem a matriz de
perfis de feature já fechada em `wiring/tests/p1464-format-pdf.md`; aqui o
eixo é exclusivamente `tagged`/`pretty` e a precedência CLI > documento >
default. Não alega `pdf.pages`, `pdf.standard` ou conformidade (P1468/P1469).

## Contrato coberto

- Ausência de `#set format.pdf(...)` e de ambas as flags CLI: `auto` sem
  seleção de páginas resolve tagueado; `pretty` ausente resolve compacto.
- `#set format.pdf(tagged: false)` sem override CLI: PDF sem
  `/StructTreeRoot`/`/MarkInfo`.
- `#set format.pdf(tagged: true)` sem override CLI: PDF com
  `/StructTreeRoot`/`/MarkInfo`.
- `--pdf-tagged=false` vence `#set format.pdf(tagged: true)`; `--pdf-tagged=true`
  vence `#set format.pdf(tagged: false)`; `--no-pdf-tags` (legado) vence
  `#set format.pdf(tagged: true)`.
- `#set format.pdf(pretty: true)` sem override CLI produz mais newlines
  estruturais que o mesmo documento compacto (proxy relativo — nunca
  contagem exata de bytes como contrato).
- `--pretty` vence `#set format.pdf(pretty: false)`.
- `tagged` e `pretty` combinam-se de forma independente na mesma árvore
  (`tagged: false` + `pretty: true` no mesmo documento).
- `#set format.pdf(...)` dentro de `#block[...]` falha a compilação com a
  mensagem canônica `format set rules are not allowed inside of containers`
  (mesma mensagem já contratada para SVG/PNG/HTML).

Inspeção por estrutura parseada (presença de `/StructTreeRoot`/`/MarkInfo`,
contagem relativa de newlines), nunca igualdade integral de bytes (ADR-0107,
ADR-0108).

## Proveniência da medição RED

Medido contra o baseline causal `306ca50022104619447fae2625090b8d9c7b0a35`
(estado descrito em `00_nucleo/materialization/typst-passo-1467.md` §2.1):
hoje `#set format.pdf(...)` não tem braço em `compiler/eval/rules.rs`, cai no
aviso genérico de target não suportado, e `04_wiring/src/main.rs` resolve
`pdf_tags`/`pdf_serialization` antes de consultar a pipeline — logo a decisão
documental é sempre ignorada e as flags CLI (`--pdf-tagged`, `--no-pdf-tags`,
`--pretty`) já funcionam isoladamente. Os testes de resolução documental pura
(sem override CLI) e o teste de container aninhado são RED esperado contra
esse baseline; os testes de precedência CLI pura coexistem como cobertura de
regressão (já verdes nesse baseline, devem continuar verdes depois).

## Controles de fronteira

- `format.svg`, `format.png`, `format.html` e seus defaults permanecem fora
  de escopo — nenhum caso deste ficheiro os exercita além do necessário para
  isolar `format.pdf`.
- `--compact`/`StreamMode`, seleção de páginas, `pdf.standard` e PDF/A-UA
  ficam fora.
- Fixtures descartáveis usam exclusivamente uma raiz em `/dev/shm` via
  `TMPDIR`, sem fallback ao disco (mesma disciplina de
  `wiring/tests/p1465-format-html.md`); ausência de `TMPDIR` explícito é
  falha de configuração do harness, não do contrato.
- Execução serial (`--test-threads=1`) e timeout finito por processo.

## Gate

Escrito pelo papel B (Agente B) sem acesso ao código de implementação do
papel A, a partir apenas dos sete Prompts L0 do gate ADR-0127 de P1467. O
hash acima é placeholder até `crystalline-lint --fix-hashes .` e a
confirmação humana explícita de materialização (que substitui o texto deste
cabeçalho por "MATERIALIZADO" e regista o par de commits RED/GREEN, seguindo
o padrão de `wiring/tests/p1464-format-pdf.md` e
`wiring/tests/p1465-format-html.md`).
