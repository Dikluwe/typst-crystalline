# Prompt L0 — `pdf` — módulo de funcionalidade específica de PDF
Hash do Código: 83fa3676

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/pdf.rs`
**Origem**: Passo 735 — namespace `pdf` ausente no cristalino ("unknown variable"); vanilla expõe como `module` (medido em P731/P735).
**ADRs**: ADR-0107 (paridade linguagem vs mecânica), ADR-0108 (medir antes de decidir), ADR-0029 (pureza L1).

---

## 1. Contexto e medições

O vanilla expõe `pdf` como módulo (`type(pdf)` → `module`, medido), definido em `lab/typst-original/crates/typst-library/src/lib.rs:347` + `pdf/mod.rs`. Conteúdo no binário medido:

- `pdf.attach` → `function` (medido) — `AttachElem`, embute ficheiros no PDF.
- `pdf.artifact` → `function` (medido) — `ArtifactElem`, marca conteúdo como artefacto decorativo (só afecta tagging/a11y; o render é inalterado).
- `table_summary`, `header_cell`, `data_cell` — gated em `Feature::A11yExtras`, ausentes do binário medido.

O estado histórico P735/P826 implementou `attach` como erro e `artifact` como
passthrough porque então se assumia ausência global de embedding/tagging. Essa
premissa foi refutada por P1140.6 e pela medição P1286: a árvore atual já emite
Formula, MCID e StructTreeRoot. O comportamento histórico não é normativo; o
contrato vigente é o P1286 abaixo e está bloqueado pelo gate ADR-0127.

## 2. Funções propostas após confirmação do gate

```rust
// 01_core/src/compiler/stdlib/pdf.rs
pub fn make_pdf_module() -> Value;   // Value::Module("pdf", scope) com as 2 funções
fn native_pdf_attach(...) -> SourceResult<Value>;   // Content::PdfAttach
fn native_pdf_artifact(...) -> SourceResult<Value>; // Content::PdfArtifact
```

## 3. Registo no scope

```rust
// 01_core/src/compiler/eval/mod.rs
scope.define("pdf", make_pdf_module());
```

## 4. Critérios de verificação

- `type(pdf)` → `module`; `type(pdf.attach)`/`type(pdf.artifact)` → `function`.
- `pdf.attach(path)` exige sempre o primeiro posicional `path` e lê seus bytes.
- `pdf.attach(path, bytes, relationship: ..., mime-type: ..., description: ...)`
  aceita `data` somente como segundo posicional opcional; `data:` named é erro.
- Attachment é visualmente vazio, mas aparece como EmbeddedFile/Filespec; path
  virtual duplicado é erro antes do export.
- `pdf.artifact[conteúdo]` preserva visual/texto e mantém o wrapper até o
  stream; com tags enabled emite `/Artifact ... EMC` sem MCID/StructElem.
- `pdf.artifact(kind: "header")[...]` é aceite; `kind: "banana"` conserva o
  erro medido. AT real permanece `Unknown`.

## P1286 — carriers semânticos para attach/artifact (GATE ADR-0127)

### Medição anterior à decisão

O receipt vanilla P1286 (`e16ea033…`) mediu `pdf.attach` com path ou bytes,
metadata opcional, attachment invisível e erros de tipo/MIME/path/duplicado.
Mediu também que `pdf.artifact` conserva visual/texto mas envolve a pintura em
`/Artifact`; AT real e fallbacks por versão permanecem `Unknown`. A premissa
histórica de que o cristalino não tem tagging está refutada: P1140.6 já emite
Formula, MCID e StructTreeRoot.

### Contrato proposto

`pdf.attach(path, data?, relationship:none, mime-type:none,
description:none)` tem `path` sempre obrigatório como primeiro posicional e
`data` somente como segundo posicional opcional. Resolve o path relativamente
a `current_file`; quando data é omitida usa `World::read_bytes`. Data explícita
deve ser `Bytes`. Produz
`Content::PdfAttach` invisível, preservando nome virtual, bytes e metadata.
MIME inválido, path ausente/inexistente, tipo incorreto e relationship fora de
`source|data|alternative|supplement|none` são erro; named `data:` é inesperado.

`pdf.artifact(kind:"other", body)` produz `Content::PdfArtifact`; os doze
kinds medidos permanecem o domínio fechado. O wrapper é visualmente
transparente, mas nunca colapsa ao body antes do stream PDF.

As funções requerem variantes/tipos públicos descritos nos L0s de entidade,
ativando paragem ADR-0127 §2.1. Nenhum código pode ser escrito antes da
confirmação. Novos ficheiros de elemento/layout terão L0 proprietário 1:1
criado junto do consumer somente pós-confirmação; este prompt não os legitima
e nenhum Núcleo novo é criado.

## P1288 — trio acessível de tabelas (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

- `01_core/src/compiler/stdlib/pdf.rs:32-42` registra hoje somente `attach` e
  `artifact`.
- A fonte vanilla pinada registra, sob `Feature::A11yExtras`, exatamente
  `table_summary`, `header_cell` e `data_cell` em
  `typst-library/src/pdf/mod.rs:12-23`.
- As assinaturas medidas estão em `pdf/accessibility.rs:137-143`, `:203-220`
  e `:264-272`: summary nomeado opcional + table; header cell com level 1,
  scope column e content-or-table.cell; data cell content-or-table.cell.

### Decisão proposta

O namespace público ganha o trio indivisível:

- `pdf.table-summary(summary: str, table)` devolve a mesma tabela com summary
  semântico substituído; `summary` é named-only e opcional por omissão. Só a
  omissão produz ausência (`None`) interna; `summary: none` explícito é erro
  (`expected string, found none`). Todos os demais campos permanecem;
- `pdf.header-cell(level: positive-int = 1, scope: "column"|"row"|"both" =
  "column", cell)` devolve uma célula explicitamente Header;
- `pdf.data-cell(cell)` devolve uma célula explicitamente Data.

`cell` aceita conteúdo cru ou um `table.cell(...)` já construído. No segundo
caso, todos os campos visuais/geométricos são preservados; somente a
classificação semântica explícita é substituída. No primeiro, cria-se a célula
default morfologicamente equivalente e aplica-se a classificação.

Diagnósticos medidos e normativos:

- `pdf.table-summary`: tabela ausente → `missing argument: table`; conteúdo no
  lugar de tabela → `expected table`; resumo inteiro → `expected string, found
  integer`; resumo posicional → `expected table`; `summary: none` explícito →
  `expected string, found none`;
- `pdf.header-cell`: célula ausente → `missing argument: cell`; célula inteira
  → `expected content, found integer`; nível zero/negativo → `number must be
  positive`; nível float → `expected integer, found float`; escopo inválido →
  `expected \"both\", \"column\", or \"row\"`; `scope: none` conserva esse conjunto
  esperado e acrescenta `, found none`;
- `pdf.data-cell`: célula ausente → `missing argument: cell`; célula inteira →
  `expected content, found integer`; `cell:` nomeado → ``the argument `cell`
  is positional``.

As três funções só são instaladas quando `A11yExtras` está ativa, conforme o
Núcleo pinado. O trio não produz stub, marcador descartável nem alteração
visual; os carriers sobrevivem até o PDF tagueado.
