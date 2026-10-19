# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/tests/p1468-format-pdf-pages.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `fb3f2fc16c1213cb5ca145850cc4ff1650f514118a0976a52169fb1bbef6720f`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — integração de `format.pdf.pages` / `compile --pages`

Hash do Código: bbbbbbbb

**Estado:** MATERIALIZADO. Contrato de teste escrito pelo Agente B (RED);
implementação do Agente A integrada pelo papel C; 12/12 testes GREEN sem
regressão na suíte de PDF. Ver `00_nucleo/materialization/typst-passo-1468.md`
para a proveniência completa.

**Camada:** L4 (testes de integração)
**Alvo planejado:** `04_wiring/tests/p1468_format_pdf_pages.rs`
**Consumer futuro exclusivo:** `04_wiring/tests/p1468_format_pdf_pages.rs`
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Responsabilidade

Executar o binário cristalino como processo externo, compilando PDFs de
verdade (sem mocks), e provar a fatia comportamental de seleção de páginas do
export PDF: o campo documental `pdf.pages` (`#set format.pdf(pages: ...)`), a
flag `--pages`, a precedência CLI > documento, a interação `pages`+`tagged`
(erro documental, warning `auto`, silêncio `false`, erro CLI-only distinto) e
a árvore de bookmarks nativa — na versão RETIFICADA decidida pelo dono
(headings de páginas incluídas sobrevivem; descendentes de headings excluídos
são reparentados ao ancestral incluído mais próximo), não a variante "outline
sempre vazia" que é um bug identificado e não reproduzido do vanilla (ver
`infra/export/builder.md` §P1468, subseção "4. Bookmark tree nativa"). Não
duplica a cobertura `tagged`/`pretty` isolada já fechada em
`wiring/tests/p1467-format-pdf-tagged-pretty.md`; aqui o eixo é
exclusivamente `pages` e sua interação com esses dois campos. Não alega
`pdf.standard` ou conformidade (P1469).

## Contrato coberto

- `#set format.pdf(pages: 2)` num documento de 4 páginas produz PDF com
  exatamente 1 página (`/Type /Page ` e `/Count` refletindo a contagem
  filtrada).
- `#set format.pdf(pages: (from: 2, to: 3))` produz 2 páginas.
- `--pages=2,4` sem decisão documental produz 2 páginas.
- `--pages` vence uma decisão documental de `pages` divergente (precedência
  CLI > documento).
- Uma seleção além do fim do documento (`--pages=10` num documento de 3
  páginas) não faz o processo morrer por sinal; este L0 não fixa a contagem
  de páginas resultante como contrato (o L0 causal de `infra/pipeline.md` e
  `infra/export/builder.md` não especifica esse caso extremo).
- `#set format.pdf(pages: 1, tagged: true)` explícito falha a compilação com
  a mensagem documental exata `"cannot enable tagged PDF and export a page
  range"`.
- `--pages=1 --pdf-tagged=true` falha ANTES de compilar com a mensagem
  CLI-only exata `"cannot enable PDF tags when exporting a page range"`
  (texto distinto da mensagem documental — dois canais de diagnóstico
  diferentes, `wiring.md`/`infra/pipeline.md` P1468), exit 1.
- `#set format.pdf(pages: 1)` sem decisão de `tagged` (default `auto`) produz
  warning `"exporting a page range disables PDF tagging"` com os dois hints
  (`"the resulting PDF will be inaccessible"`,
  `"set \`pdf(tagged: false)\` to silence this warning"`) e PDF sem
  `/StructTreeRoot`.
- `#set format.pdf(pages: 1, tagged: false)` não emite esse warning e produz
  PDF sem struct tree.
- Um documento com três headings de nível 1 em páginas distintas, exportando
  só a página do heading do meio, produz um outline nativo (`/Outlines`
  presente no catálogo) contendo o título do heading incluído e omitindo os
  dois excluídos — divergência deliberada do vanilla (que zera a árvore
  inteira mesmo cobrindo todas as páginas).
- Um heading pai (nível 1, página excluída) com um heading filho (nível 2,
  página incluída): o filho sobrevive na árvore, reparentado; o pai é
  omitido.
- Um link para uma página removida tem sua annotation omitida (ausência em
  `/Annots`), não redirecionada — confirmado por controle (mesmo documento
  sem seleção de páginas produz pelo menos uma annotation).

Inspeção por estrutura parseada (contagem de `/Type /Page `, `/Count`,
presença/ausência de `/StructTreeRoot`/`/Outlines`/`/Title <hex>`/`/Annots`),
nunca igualdade integral de bytes (ADR-0107, ADR-0108). O helper de título
(`utf16be_hex_string`) é um mirror mecânico do owner de produção
(`03_infra/src/export/builder.rs`), não uma alegação de paridade de mecânica.

## Proveniência da medição RED

Medido contra o baseline causal `0b322a926dd2f86b49aabfeb0d2f287ce06aaace`
(estado descrito em `00_nucleo/materialization/typst-passo-1468.md` §8): hoje
`format.pdf` só reconhece `tagged`/`pretty` (P1467); `pages` não existe em
`CompileArgs`/`CompileIntent`, `DocumentFormatOptions`, nem no exportador.
Todos os casos deste ficheiro são RED esperado contra esse baseline — a
suite inteira falha por erro de compilação de `02_shell`/`03_infra`/
`04_wiring` (campo/tipo/função ausentes) até a implementação (Agente A) e
`crystalline-lint --fix-hashes .` materializarem o contrato.

## Controles de fronteira

- `format.svg`, `format.png`, `format.html`, `pdf.tagged`/`pdf.pretty`
  isolados (sem `pages`) e seus defaults permanecem fora de escopo — nenhum
  caso deste ficheiro os exercita além do necessário para isolar `pages`.
- `pdf.standard`, conformidade PDF/A e `/PageLabel` ficam fora (scope-out
  explícito de `infra/export/builder.md` P1468).
- `--pages` aplicado a PNG/SVG fica fora (scope-out explícito de
  `infra/pipeline.md` P1468: a capacidade estrutural de multi-ficheiro não
  existe ainda no cristalino).
- Fixtures descartáveis usam exclusivamente uma raiz em `/dev/shm` via
  `TMPDIR`, sem fallback ao disco (mesma disciplina de
  `wiring/tests/p1467-format-pdf-tagged-pretty.md`); ausência de `TMPDIR`
  explícito é falha de configuração do harness, não do contrato.
- Execução serial (`--test-threads=1`) e timeout finito por processo.

## Gate

Escrito pelo papel B (Agente B) sem acesso ao código de implementação do
papel A, a partir apenas dos oito Prompts L0 do gate ADR-0127 de P1468. O
hash acima é placeholder até `crystalline-lint --fix-hashes .` e a
confirmação humana explícita de materialização (que substitui o texto deste
cabeçalho por "MATERIALIZADO" e regista o par de commits RED/GREEN, seguindo
o padrão de `wiring/tests/p1467-format-pdf-tagged-pretty.md`).
