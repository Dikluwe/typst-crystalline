# Prompt L0 — `pdf` — módulo de funcionalidade específica de PDF
Hash do Código: d4cf3233

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

A medição vanilla P1286 cobriu `pdf.attach` com path ou bytes,
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

## P1471 — retificação: `pdf` é `Value::Func`, não `Value::Module` (MATERIALIZADO)

### Medição anterior à decisão

Sonda bilateral repetida em 2026-09-21 contra o vanilla ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d` (`/usr/local/bin/typst`) e o
cristalino corrente (`target/debug/typst`, build de working tree):

```text
typst eval --features html '(type(pdf), repr(pdf))'
vanilla:    ["function","pdf"]
cristalino: ["module","<module pdf>"]

typst eval --features html 'pdf()'
vanilla: error: cannot be constructed manually
```

Este passo repete e confirma a medição original de `typst-passo-1471.md`
§2 (sondada em 2026-09-20T17:40:24-03:00; sem deriva). O root §1
("Constrói o módulo `pdf` como `Value::Module`...") e a linha 34
(comentário `/// Constrói o módulo pdf como Value::Module...`) descrevem a
forma **anterior**, refutada por esta medição — não são apagados, mas
deixam de ser a conclusão vigente para o valor raiz `pdf` (os membros do
namespace, `attach`/`artifact`/trio a11y, continuam corretamente descritos
como `function` em todo este L0 e não mudam).

### Viabilidade medida (não é decisão, é grounding)

`01_core/src/entities/func.rs:122,165-180` já expõe
`NativeFunc.namespace: Option<Arc<Scope>>` e o construtor público
`Func::native_with_namespace(name, call, namespace: Arc<Scope>)` —
mecanismo de "P493 — namespace anexado para sub-funções via field access
(`table.header`)", já em uso produtivo. `compiler/stdlib/format.rs`
já constrói `format.pdf`/`format.html`/`format.svg`/`format.png` exatamente
assim (`Value::Func(Func::native_with_namespace(...))`, ver
`compiler/stdlib/format.md` P1464/P1465). Isto não é decisão desta secção —
é a evidência que torna a decisão abaixo implementável sem novo tipo,
trait ou campo em `entities/func.rs`.

### Decisão

`make_pdf_module(features)` passa a devolver
`Value::Func(Func::native_with_namespace("pdf", native_pdf_root, Arc::new(scope)))`
em vez de `Value::Module(Module::new("pdf", scope))`. `native_pdf_root`
produz exatamente o diagnóstico medido `cannot be constructed manually`
(mesma função de erro já usada por `format.rs::native_format` — reutilizar,
não duplicar). O `scope` interno (attach/artifact/trio a11y gated) não
muda: mesmos nomes, mesmas funções, mesmo gate `A11yExtras`. `type(pdf)`
passa de `module` para `function`; `repr(pdf)` passa de `<module pdf>`
para `pdf`; `pdf.attach`/`pdf.artifact`/`pdf.table-summary`/
`pdf.header-cell`/`pdf.data-cell` continuam idênticos e continuam
acessíveis por field access (mecanismo já existente, não módulo).

`format.pdf.attach == pdf.attach` (e equivalentes) devem continuar `true`
— `format.rs::make_format_module` deixa de exigir `Value::Module` na
entrada e passa a ler `Value::Func` com `.namespace()` em vez de
`.scope()` (ver `compiler/stdlib/format.md` P1471 para o contrato exato
desse lado).

### Gate ADR-0127

Quebra deliberada de contrato público (`type(pdf)` muda de `module` para
`function`), exigida para paridade com o vanilla ratificado. Requer L0
causal (este), hashes ressealados, V5/V15/V26, commit documental e
**confirmação humana explícita posterior**, antes de qualquer código. A
confirmação de P1467-P1470 não cobre esta mudança.

A confirmação humana explícita foi recebida em 2026-09-21, posterior ao
commit causal somente-L0 `2ae054ab5`. Agente A (`e118c2053`) implementou
`make_pdf_module` sem ver os testes; Agente B (`b546f709b`) escreveu 4
testes RED (`p1471_pdf_type_and_repr_are_function_across_profiles`,
`p1471_pdf_call_fails_cannot_be_constructed_manually_across_profiles`,
`p1471_pdf_root_is_func_with_stable_namespace_across_profiles`,
`p1471_pdf_field_access_members_present_across_profiles`) sem ver a
implementação. Integração (papel C) corrigiu 4 oráculos históricos que
assumiam `type(pdf) == module` (`eval/tests.rs`, ver `compiler/eval.md`
P1471) e um consumer de produção fora do escopo dos 5 L0s auditados —
`P1293HtmlNative::from_func` em `call_dispatch.rs`, que também lia
`.scope()` sobre `make_html_module()` — achado pelo Agente A e corrigido
por ele mesmo. `01_core/src/compiler/stdlib/pdf.rs`: 10/10 testes
(4 novos + 6 irmãos). Zero regressão em typst-core (6059/6059),
typst-shell (83/83), typst-infra (995/995, com um flake pré-existente e
não relacionado — `p821_target_fora_de_contexto_erro_no_pipeline` —
reconfirmado GREEN em reexecução isolada e em suíte completa). Clippy
comparado por contagem `(ficheiro, mensagem)` contra o baseline `2ae054ab5`
em worktree isolada: zero avisos novos.
