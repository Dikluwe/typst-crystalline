# Passo 1468 — materializar `pdf.pages` e `compile --pages`

**Estado:** L0S ESCRITOS (9 ficheiros — ver secção 8); V15/V26 verdes; AGUARDA
COMMIT CAUSAL SOMENTE-L0 E CONFIRMAÇÃO HUMANA ADR-0127 antes de RED/código. A
confirmação de P1467 não cobre P1468 (declarado explicitamente no L0 original).

**Data da auditoria original:** 2026-09-20T17:40:24-03:00
**Data da execução (rebase + medição extensa + L0s):** 2026-09-20T21:00 a
22:15-03:00

**Baseline de redação original:** `306ca50022104619447fae2625090b8d9c7b0a35`.
**Baseline real de execução (rebase feito, conforme instruído):**
`0b322a926dd2f86b49aabfeb0d2f287ce06aaace` (HEAD após materialização de
P1467). `git status --short` no momento da execução: só o WIP não relacionado
de P1470 (`compiler/layout.md`, `entities/introspector.md`,
`compiler/layout/mod.rs`, `entities/introspector.rs`), preservado intocado.

**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`, SHA-256 do
binário `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`
(confirmado idêntico ao pin). Fonte consultada diretamente:
`lab/typst-original/crates/typst-{library,pdf,cli}/`.

## 1. Objetivo

Fechar a seleção pública de páginas do PDF por duas entradas equivalentes:

- linguagem: `#set pdf(pages: ...)`;
- CLI: `compile --pages ...`.

A seleção usa números físicos de página, iniciados em um, e ocorre no export
PDF. Não altera o contador de página da linguagem, o layout do documento nem
PNG/SVG/HTML.

## 2. Medição anterior à decisão

O inventário confirma `format.pdf.pages == none` em ambos os binários; portanto
a rota nominal já existe e o trabalho é comportamental.

A fonte pinada mede em `typst-pdf/src/format.rs:108-133` o domínio documental:

- `none` para todas as páginas;
- inteiro positivo para uma página;
- dict com `from` e/ou `to`, limites inclusivos e pelo menos uma chave;
- array que mistura páginas e ranges.

`typst-cli/src/args.rs:340-353,777-806` mede a gramática CLI: tokens separados
por vírgula, cada um `N`, `A-B`, `A-` ou `-B`; zero, vazio, range invertido e
múltiplos hífens falham. Os números são físicos, não seguem numeração impressa.

`typst-pdf/src/lib.rs:99-146` mede a interação normativa: seleção de páginas
desabilita tagging automático; tagging explicitamente `true` junto de pages é
erro; `auto` produz warning e resolve para false; standards acessíveis podem
proibir a combinação.

## 3. Contrato a escrever no L0

### 3.1 Tipo de domínio único

Criar em L1 um carrier puro e fechado para ranges inclusivos, reutilizado pelo
set rule, pelo `PagedDocument` e pelo export. Ele deve distinguir limite aberto
de página um e validar inteiros positivos sem sentinel `0`.

Não transportar strings CLI para L1. L2 parseia a gramática textual para o
carrier; L1 parseia valores da língua para o mesmo domínio. O algoritmo de
normalização, deduplicação e ordem só pode ser decidido depois de sondar casos
com ranges sobrepostos, duplicados, fora do documento e ordem não monotônica no
vanilla. Registrar a saída e a fonte que sustentam a decisão; não inferir.

### 3.2 Precedência e tagging

```text
--pages presente > pdf.pages documental > none
```

Depois de resolver pages:

- ausência mantém a decisão de tagging do P1467;
- pages + `tagged: true` explícito é erro;
- pages + `tagged: auto` desabilita tags e emite warning;
- pages + `tagged: false` é válido e não emite esse warning.

O warning/erro da CLI e o diagnóstico da regra documental devem conservar o
canal e as mensagens medidos no pin. L2 não importa o encoder; L4 só compõe;
L3 aplica a seleção no export.

### 3.3 Integridade do documento exportado

Ao selecionar páginas, ajustar todas as estruturas dependentes da coleção
exportada: page tree, destinos internos, annotations, outlines, labels e
referências. Uma página removida não pode deixar referência inválida. Não
relayoutar o documento nem renumerar o observável físico antes da seleção.

## 4. Scope-out

- nenhum standard novo ou validação de conformidade: P1469;
- nenhuma mudança de `pdf` module→function: P1471;
- **retificação medida (ver §8.1):** "nenhum slicing de PNG/SVG/HTML" aqui
  não é uma escolha arbitrária — o vanilla ratificado de facto usa a mesma
  seleção `--pages` para filtrar páginas no export multi-ficheiro de
  PNG/SVG (`typst-cli/src/compile.rs:547-560`). O scope-out é válido porque
  o cristalino ainda não suporta multi-ficheiro/templates `{p}`/`{0p}`/`{t}`
  para PNG/SVG (pré-requisito estrutural ausente, não decisão de produto);
- nenhum `/PageLabel` (numeração física preservada no leitor de PDF): o
  vanilla emite isso (`typst-pdf/src/convert.rs:130-141`), o cristalino não
  emite `/PageLabel` para nenhum caso hoje; scope-out explícito, ver §8.1;
- nenhum novo seletor de páginas de conteúdo;
- nenhuma comparação de PDF byte a byte.

## 5. L0s causais e gate

Antes de código, atualizar os owners canônicos de:

- `compiler/stdlib/format` e `compiler/eval/rules` para o cast da língua;
- `entities/document_format_options` para o campo público;
- `compiler/layout/format_options` para a coleta root;
- `shell/cli` para `--pages` e seus diagnósticos;
- `infra/pipeline` e o owner do export PDF para seleção/validação;
- `wiring` para a precedência fina.

A adição do carrier/campo público, a flag e a mudança do conjunto de páginas
exportadas acionam ADR-0127. Criar commit causal de L0, validar V5/V15/V26 e
**parar para confirmação humana**. A aprovação do P1467 não cobre P1468.

## 6. RED obrigatório

Congelar bilateralmente:

- cada forma válida documental e CLI;
- zero, negativo, float, string, dict vazio, chave extra, limite invertido,
  array misto inválido, range CLI vazio e múltiplos hífens;
- página única, range fechado e limites abertos;
- sobreposição, duplicação, ordem não monotônica e página além do final;
- precedência CLI/documento;
- as três combinações `tagged: auto|true|false`;
- contagem e textos públicos das páginas preservadas;
- links/destinos para páginas preservadas e removidas;
- neutralidade em PDF sem pages e nos outros targets.

## 7. Gates finais

Executar focais L1/L2/L3/L4, depois suíte completa, Clippy, build, fmt,
`crystalline-lint .`, V5/V15/V26 com warning fatal e `git diff --check`.
Registrar no próprio passo o estado exato e as contagens produzidas.

## 8. Medição detalhada e proveniência da execução (2026-09-20)

### 8.1 Achados adicionais não previstos na redação original

1. **`--pages` também filtra PNG/SVG multi-ficheiro no vanilla**
   (`typst-cli/src/compile.rs:547-560`, `export_image_pages`). O campo
   *documental* `pdf.pages` (`#set format.pdf(pages: ...)`) é exclusivo do
   elemento `pdf` e nunca afeta PNG/SVG — isso está correto e é o núcleo
   deste passo. Mas a *flag CLI* `--pages` é genérica no vanilla. O
   cristalino hoje só exporta a primeira página para PNG/SVG (sem templates
   `{p}`/`{0p}`/`{t}` nem multi-ficheiro), logo a extensão ao PNG/SVG não
   tem onde aterrar — scope-out corrigido em §4, não silencioso.
2. **A árvore de outline nativa do PDF (`/Outlines`) fica sempre vazia
   quando `pdf.pages` está definido**, mesmo cobrindo *todas* as páginas do
   documento (`--pages 1-4` num documento de 4 páginas → `/Outlines`
   ausente do catálogo, confirmado por dump de objetos via `qpdf --qdf
   --object-streams=disable`). A causa raiz, lida diretamente em
   `typst-pdf/src/outline.rs:25-31`, é uma expressão que parece
   contra-intuitiva (`visible = pages.is_none_or(|r| !r.includes_page(p))`
   — headings em páginas *incluídas* contam como `visible=false`), cujo
   efeito combinado com a poda de `OutlineNode::build_tree`
   (`typst-library/src/model/outline.rs:338-395`) é: nenhum heading
   sobrevive à árvore, seja qual for a seleção. **P1468 reproduz este
   comportamento observável fielmente** (ADR-0107 — paridade com o medido,
   não com a leitura "pretendida" do código); não é uma correção de bug.
3. **Links para páginas removidas são removidos, não redirecionados.** Um
   teste inicial com `#outline()` + heading-alvo removido pareceu mostrar um
   link "sobrevivente" redirecionado para a última página do range — depois
   de isolar (removendo `#outline()`, testando um alvo não-heading, e
   relendo `typst-pdf/src/link.rs:191-209`/`typst-pdf/src/convert.rs:835-875`)
   confirmou-se que a regra real e única é "link para página excluída →
   annotation omitida"; o resultado anterior era um efeito colateral
   específico de `#outline()` (mecanismo de outline de conteúdo, não
   bookmark nativo) não investigado a fundo — não é citado como contrato
   deste passo, e não deve ser tratado como redirecionamento intencional.
4. **Mensagens de erro por dois canais distintos e com texto diferente**
   para o mesmo conflito semântico "pages + tagged:true": CLI-only
   (`"cannot enable PDF tags when exporting a page range"`, verificado antes
   de compilar) vs. documental (`"cannot enable tagged PDF and export a page
   range"`, verificado depois de resolver com o documento). Não são a mesma
   string reaproveitada — registado em `infra/pipeline.md` e `wiring.md`.
5. **Domínio `PageRanges`/`PageRange` do vanilla não normaliza nem
   deduplica** (`typst-library/src/layout/page.rs:760-799`): é uma lista de
   ranges consultada via `.any()`; a aparência de "conjunto único ordenado"
   vem de quem itera páginas físicas em ordem crescente e testa
   `includes_page`, nunca de pós-processar a lista.

### 8.2 Arquitetura cristalina consultada (via fork de auditoria)

O exportador cristalino (`03_infra/src/export/mod.rs` +
`03_infra/src/export/builder.rs`, ~3300 linhas) é estruturalmente mais
simples que o vanilla (sem `krilla`/`Document`): três variantes de build
(`build_helvetica`/`build_cidfont`/`build_multifont`) escrevem bytes PDF
diretamente, cada uma assumindo hoje `n = doc.pages.len()` sem filtro.
Links usam destino **nomeado** (`/A /GoTo /D {name}`, resolvido por
`emit_named_destinations` via `/Names /Dests`), não `/Dest` posicional
direto — divergência mecânica permitida por ADR-0107. Já existe um
fallback "página inválida → última página" em `emit_named_destinations`
(`:2219-2227`) e `emit_outlines` (`:2380-2384`) para um caso **diferente**
(label cujo número de página nunca existiu no array `doc.pages`) — não deve
ser reaproveitado para o caso de página validamente excluída pela seleção
(que exige omissão, não fallback). Outline nativa já existe, construída a
partir de `doc.extracted_headings` (lista simples, não árvore aninhada como
o vanilla) — a decisão "outline sempre vazia com pages ativo" é uma
condição adicional trivial de implementar nessa estrutura mais simples.

### 8.3 L0s escritos (9 ficheiros)

`entities/document_format_options.md`, `compiler/eval/rules.md`,
`compiler/layout/format_options.md`, `shell/cli.md`, `infra/pipeline.md`,
`infra/export/mod.md`, `infra/export/builder.md`, `wiring.md` (todos com
contrato substantivo novo), e `compiler/stdlib/format.md` (nota de scope,
sem contrato novo — mesmo padrão de P1467, já que `format.pdf.pages ==
none` já é default nominal materializado em P1464).

### 8.4 Gate ADR-0127

Após `crystalline-lint --fix-hashes .` e validação V15/V26, commit causal
somente-L0 (sem código produtivo, sem RED) e parada para confirmação humana
explícita — a confirmação de P1467 não cobre P1468 (repetido do L0 original,
seção 5).

