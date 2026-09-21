# Prompt L0 — `entities/document_format_options`
Hash do Código: 1b68276b

**Estado:** P1462, P1463, P1465, P1467 E P1468 MATERIALIZADOS; RETIFICAÇÃO
P1466 E CARRIER P1469 (ESCOPO REDUZIDO) PROPOSTOS; AGUARDAM GATE ADR-0127.

**Camada:** L1
**Alvo planejado:** `01_core/src/entities/document_format_options.rs`
**Consumer futuro exclusivo:** `01_core/src/entities/document_format_options.rs`
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

No baseline P1462, `PagedDocument` não transporta opções de formato e não há
`DocumentFormatOptions`/`SvgFormatOptions`. O vanilla ratificado possui um
carrier documental fechado antes dos exporters; `format.svg.pretty` tem
default `false`. O estado é domínio puro de L1, sem I/O ou dependência de
exportador.

## Contrato público P1462

Definir tipos públicos clonáveis, comparáveis e com `Default`:

```rust
pub struct SvgFormatOptions {
    pub pretty: bool,
}

pub struct DocumentFormatOptions {
    pub svg: SvgFormatOptions,
}
```

Os dois defaults produzem `pretty == false`. Não há estado tri-state no
documento: presença/ausência da regra já foi resolvida pela cascata L1. O
`Option<bool>` pertence ao override externo de L3/L4 e não entra nestas
entidades.

## Incompletude deliberada do constructor

O carrier P1462 é deliberadamente incompleto e registra os subconjuntos que o
completam:

- P1463 propõe acrescentar opções de `format.png`;
- P1465 acrescenta opções de `format.html`;
- P1467 acrescenta a primeira fatia de opções de `format.pdf` (`tagged`,
  `pretty`); `pages` e `standard` continuam reservados a P1468/P1469.

P1464 materializou somente descoberta/namespace de `format.pdf` em
`compiler/stdlib/format.md`; não tocou este carrier e não é citado acima como
dono de subconjunto.

Até cada gate próprio, não antecipar campos, enums, placeholders ou maps
abertos. O uso de structs fechadas não afirma que o conjunto futuro terminou.

## Aceitação e gate

Testes provam defaults, clone/igualdade e independência entre duas instâncias.
Não testar layout Rust, tamanho/alinhamento ou debug como paridade. Os campos
públicos ampliam contrato entre camadas. A confirmação humana posterior ao
commit causal P1462 foi recebida em 2026-09-19; os carriers SVG foram
materializados e validados.

## Contrato público P1463 (MATERIALIZADO)

Acrescentar o carrier público:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct PngFormatOptions {
    pub ppi: f64,
}

impl Default for PngFormatOptions {
    fn default() -> Self { Self { ppi: 144.0 } }
}

pub struct DocumentFormatOptions {
    pub svg: SvgFormatOptions,
    pub png: PngFormatOptions,
}
```

O `Default` de `PngFormatOptions` é manual e produz `ppi == 144.0`;
`DocumentFormatOptions::default()` combina SVG `pretty == false` e PNG
`ppi == 144.0`. O campo guarda o valor documental público tal como escrito;
não divide por 72 e não armazena pixels por ponto. O override externo continua
fora de L1.

Como `f64` não implementa `Eq`, `DocumentFormatOptions` passa a derivar
`PartialEq`, não `Eq`; `SvgFormatOptions` conserva `Eq`. É proibido inventar
igualdade total, bit wrapper ou sentinela para preservar uma mecânica Rust.

Testes provam defaults, clone, `PartialEq`, independência das instâncias e
ortogonalidade SVG/PNG. Não aceitar `Option<f64>` no documento nem acrescentar
PDF/HTML nesse recorte. A confirmação humana posterior ao commit causal P1463
foi recebida em 2026-09-19; os carriers PNG foram materializados e validados.

## Contrato público P1465 (MATERIALIZADO; gate ADR-0127 satisfeito)

### Medição anterior à decisão

No baseline `8e74134a721310cc4fb2a596a195398e16f1d5db`, o carrier possui
SVG e PNG. O vanilla resolve `format.html.pretty` como booleano documental com
default `false`; o `Option<bool>` existe somente na fronteira do override CLI.

### Contrato

Acrescentar o carrier público:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HtmlFormatOptions {
    pub pretty: bool,
}

pub struct DocumentFormatOptions {
    pub svg: SvgFormatOptions,
    pub png: PngFormatOptions,
    pub html: HtmlFormatOptions,
}
```

`HtmlFormatOptions::default().pretty` e
`DocumentFormatOptions::default().html.pretty` são `false`. O campo é o valor
resolvido pela cascata do documento, não `Option<bool>` e não um alias do campo
SVG. `DocumentFormatOptions` conserva `PartialEq` por causa do `f64` PNG;
nenhuma mudança artificial de trait ou representação é autorizada.

Testes provam default, clone, igualdade, independência de instâncias e
ortogonalidade SVG/PNG/HTML. Não acrescentar PDF, exporter, feature ou lógica
de precedência neste owner. A confirmação humana posterior ao commit causal
`e300314e2f87e90e543c4923a8cae97f9ad978a8` foi recebida em 2026-09-20; o
novo tipo/campo e seus testes foram materializados.

## P1466 — unidade pública do PNG (PROPOSTA; AGUARDA GATE ADR-0127)

O campo `PngFormatOptions::ppi` continua a representar **pixels por polegada**,
inclusive no default `144.0`. O carrier preserva o número documental resolvido
sem conversão, porque a conversão para pixels por ponto pertence à fronteira de
rasterização em L3. Esta retificação não altera tipo, campo, default, traits ou
cascata e proíbe reinterpretar o valor como `pixel_per_pt`.

Aceitação mantém os testes de entidade vigentes e acrescenta apenas a prova de
unidade nos owners de pipeline/export. Nenhum sentinel, wrapper, `Option` ou
segundo campo de escala é autorizado.

## P1467 — carrier `PdfFormatOptions` (`tagged`, `pretty`) (MATERIALIZADO; gate ADR-0127 satisfeito)

### Medição anterior à decisão

No baseline `306ca50022104619447fae2625090b8d9c7b0a35`, o carrier possui SVG,
PNG e HTML; não existe `PdfFormatOptions`. `compiler/stdlib/format.rs`
(P1464) já publica os defaults nominais `format.pdf.tagged == auto` e
`format.pdf.pretty == false` no namespace da função nativa, mas nenhuma
entidade documental os transporta até a pipeline.

Probes bilaterais contra o vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`
(medidos em 2026-09-20T19:18 via `typst eval`/`typst compile` com
`/usr/local/bin/typst`, SHA-256 `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`,
confirmado idêntico ao binário pinado):

```text
repr((type(format.pdf.tagged), format.pdf.tagged,
      type(format.pdf.pretty), format.pdf.pretty))
=> (type(auto), auto, bool, false)

#set format.pdf(tagged: false)   -> PDF exportado sem /StructTreeRoot,/MarkInfo
#set format.pdf(tagged: true)    -> PDF exportado com /StructTreeRoot,/MarkInfo
ausência de #set format.pdf(...) -> PDF exportado com /StructTreeRoot,/MarkInfo
                                     (auto resolve tagueado sem seleção de páginas)
#set format.pdf(pretty: true)    -> serialização com whitespace estrutural
                                     (337 linhas vs. 114 no mesmo documento compacto)
tagged:false + pretty:true       -> sem struct tree, com whitespace pretty
                                     (campos ortogonais)
```

O domínio de `tagged` tem três estados reais na linguagem (`auto`, `true`,
`false`); não é um `Option<bool>` de conveniência do override externo — essa
distinção pertence exclusivamente ao override de CLI em L3/L4
(`infra/pipeline.md` P1467), não a esta entidade L1.

### Contrato

Acrescentar ao owner os tipos públicos:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PdfTagged {
    #[default]
    Auto,
    Explicit(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PdfFormatOptions {
    pub tagged: PdfTagged,
    pub pretty: bool,
}

pub struct DocumentFormatOptions {
    pub svg: SvgFormatOptions,
    pub png: PngFormatOptions,
    pub html: HtmlFormatOptions,
    pub pdf: PdfFormatOptions,
}
```

`PdfFormatOptions::default()` produz `tagged == PdfTagged::Auto` e
`pretty == false`; `DocumentFormatOptions::default()` combina os quatro
formatos sem alterar os defaults SVG/PNG/HTML já materializados.
`DocumentFormatOptions` conserva `PartialEq` (não `Eq`, por causa do `f64`
PNG); `PdfFormatOptions` e `PdfTagged` conservam `Eq` porque não contêm `f64`.
Não usar sentinela, wrapper nem `Option<bool>` para representar o terceiro
estado de `tagged`.

### Incompletude deliberada e fronteira

Este carrier fecha somente `tagged` e `pretty`. `pdf.pages` (P1468) e
`pdf.standard`/conformidade (P1469) continuam reservados; não antecipar campo,
enum ou variante para eles. Este owner não decide precedência entre CLI e
documento — isso pertence a `infra/pipeline.md`; não decide o cast do valor
recebido do `Styles` — isso pertence a `compiler/eval/rules.md` e
`compiler/layout/format_options.md`; não sabe que existe CLI, exporter ou
`PdfTags`/`PdfSerializationMode`.

### Aceitação e gate

Testes provam default (`Auto`/`false`), clone, `PartialEq`/`Eq` de
`PdfTagged`/`PdfFormatOptions`, independência de instâncias e ortogonalidade
com SVG/PNG/HTML. Não testar layout Rust, tamanho/alinhamento nem exporter
como paridade. Este é um dos quatro L0s do gate ADR-0127 de P1467: campo
público novo em `DocumentFormatOptions` e mudança de comportamento por
defeito quando o documento contém `#set format.pdf(...)`.

A confirmação humana explícita posterior ao commit causal somente-L0
`e675e8b7acc29e272bd59939e44681a7b521acd3` foi recebida em 2026-09-20.
Execução segregada (ADR-0108/protocolo de nucleação): Agente A implementou o
carrier sem ver os testes; Agente B escreveu 6 testes de entidade a partir do
mesmo baseline sem ver a implementação. O teste independente passou de RED
(falha de compilação — `E0609`/`E0433` por `pdf`/`PdfTagged` inexistentes) a
GREEN (6/6) após a integração.

## P1468 — carrier `PageRanges`/`PageRange` e campo `PdfFormatOptions::pages` (MATERIALIZADO; gate ADR-0127 satisfeito)

### Medição anterior à decisão

No baseline `0b322a926dd2f86b49aabfeb0d2f287ce06aaace`, `PdfFormatOptions` tem
`tagged`/`pretty`; não existe tipo de range de páginas em `01_core/`. O
vanilla ratificado (`lab/typst-original/crates/typst-library/src/layout/page.rs:760-869`)
modela o domínio como:

```rust
pub struct PageRanges(EcoVec<PageRange>);
pub struct PageRange { pub start: Option<NonZeroUsize>, pub end: Option<NonZeroUsize> }
```

`PageRanges::includes_page` é um simples `.any()` sobre os ranges — a lista
**não é normalizada, deduplicada nem ordenada**; a "deduplicação" observável
vem de iterar as páginas físicas do documento em ordem crescente e testar
`includes_page` para cada uma (`typst-pdf/src/convert.rs:883-898`,
`PageIndexConverter::new`). Confirmado experimentalmente: `--pages 3,1,2-3`
num documento de 6 páginas produz exatamente as páginas físicas `{1,2,3}`, na
ordem física, não na ordem dos tokens.

O cast do valor documental (`typst-library/src/layout/page.rs:806-847`,
`cast! { PageRange, ... }`) aceita `i64` (via `parse_page_number`: `0` →
`"page numbers start at one"`; outro valor que falhe `NonZeroUsize` →
`"not a valid page number"`) ou `Dict` com chaves `from`/`to` (pelo menos uma
presente — `"page range must not be empty"` se ambas ausentes; `"start of
page range cannot be larger than end"` se `from > to`; `dict.finish(&["from",
"to"])` rejeita chave extra com `"unexpected key \"extra\", valid keys are
\"from\" and \"to\""`). `PageRanges` em si aceita `PageRange` único ou
`Array` de `PageRange`/inteiro misto. Medido via `typst eval`/`typst compile`
com `/usr/local/bin/typst` em 2026-09-20T21:1x (mensagens exatas na secção 2
do passo).

### Contrato

Acrescentar ao owner os tipos públicos:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRange {
    pub start: Option<NonZeroUsize>,
    pub end: Option<NonZeroUsize>,
}

impl PageRange {
    pub fn includes(&self, page: NonZeroUsize) -> bool {
        match (self.start, self.end) {
            (Some(s), Some(e)) => (s..=e).contains(&page),
            (Some(s), None) => page >= s,
            (None, Some(e)) => page <= e,
            (None, None) => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PageRanges(Vec<PageRange>);

impl PageRanges {
    pub fn new(ranges: Vec<PageRange>) -> Self { Self(ranges) }
    pub fn includes_page(&self, page: NonZeroUsize) -> bool {
        self.0.iter().any(|r| r.includes(page))
    }
}
```

E `PdfFormatOptions` ganha `pub pages: Option<PageRanges>`, default `None`
(todas as páginas). `PageRanges`/`PageRange` não normalizam, deduplicam nem
ordenam a lista interna — a semântica de "conjunto físico único e ordenado"
emerge exclusivamente de `includes_page` ser consultado por quem itera as
páginas físicas em ordem (owner de `infra/export/builder.md`). Este owner não
sabe que existe CLI (`--pages`), Dict/Array do eval (isso é
`compiler/eval/rules.md`) nem exporter.

`PageRange`/`PageRanges` derivam `Eq` (só contêm `NonZeroUsize`). Todos os
campos de `PdfFormatOptions` são `Eq`-capazes (`PdfTagged: Eq`, `bool: Eq`,
`Option<PageRanges>: Eq`), logo `PdfFormatOptions` passa a derivar `Eq`
também, não só `PartialEq`. `DocumentFormatOptions` continua apenas
`PartialEq` (nunca `Eq`), inalterado, por causa do `f64` de PNG.

### Scope-out e fronteira

Este owner não decide precedência CLI/documento, não decide a interação
`pages`+`tagged` (isso é `infra/pipeline.md`), não sabe que o exportador usa
o valor para reindexar páginas/links/outline (isso é
`infra/export/builder.md`). `pdf.standard` continua reservado a P1469.

### Aceitação e gate

Testes provam: `PageRange::includes` para os quatro casos (fechado, aberto no
início, aberto no fim, universal); `PageRanges::includes_page` com múltiplos
ranges sobrepostos/disjuntos; `PageRanges::default()`/`PdfFormatOptions::default().pages
== None`; `Eq`/clone/independência de instâncias; ortogonalidade com
`tagged`/`pretty`/SVG/PNG/HTML. Este é um dos L0s do gate ADR-0127 de P1468
(campo público novo).

A confirmação humana explícita posterior aos commits causais somente-L0
`9bb20b8e8`/`70d09eae2` (retificação da decisão de outline) foi recebida em
2026-09-20/21. Agente A implementou o carrier sem ver os testes; Agente B
escreveu 9 testes a partir do mesmo baseline sem ver a implementação. O
teste independente passou de RED (erro de compilação — `PageRange`/
`PageRanges` inexistentes) a GREEN (17/17 no ficheiro, incluindo os 8
pré-existentes de P1462/P1465/P1467) após a integração, sem regressão.

## P1469 — carrier `PdfStandards` (contrato de linguagem; ESCOPO REDUZIDO por decisão do dono) (PROPOSTA; AGUARDA GATE ADR-0127)

### Escopo reduzido — decisão do dono, 2026-09-21

O L0 original do passo pedia "conformidade observável real" para PDF/A e
PDF/UA (estrutura XMP, restrições de cor/transparência/fontes, validação por
ferramenta externa). Medição prévia à decisão: (1) o vanilla delega essa
mecânica inteira à crate `krilla` (`~/.cargo/git/checkouts/krilla-*/crates/krilla/src/configure/`,
só o módulo de *seleção* de validador já tem >1500 linhas; a imposição real
das regras de cada perfil está espalhada pelo resto da crate), que o
cristalino não usa; (2) não há validador de conformidade PDF/A/PDF/UA
disponível neste ambiente (sem veraPDF, sem `pikepdf`; só `qpdf`/`mutool`,
que validam estrutura genérica de PDF, não conformidade A/UA). O próprio L0
original antecipa esta situação: "se um perfil não puder ser implementado de
forma real nesta fatia, o L0 deve reduzir explicitamente o conjunto
anunciado; não aceitar e ignorar". Consultado, o dono confirmou: materializar
apenas o **contrato de linguagem** (parsing, validação de conflitos,
mensagens, versão efetiva do header) — sem implementar nem alegar
conformidade PDF/A ou PDF/UA real. Registado aqui e em
`00_nucleo/materialization/typst-passo-1469.md`.

### Medição anterior à decisão

No baseline `85b7fc5f1eda67e89494a478c2458de36d4ee20d`, `PdfFormatOptions` não
tem campo `standard`; `02_shell/src/cli.rs:377-378` mantém `--pdf-standard`
como flag oculta e descartada (`_pdf_standard: Option<String>`, nunca lida).

O vanilla ratificado (`typst-library/src/layout` não se aplica aqui — a
fonte é `typst-pdf/src/format.rs:552-719` e a crate `krilla::configure`)
modela o domínio em três níveis:

```rust
// nomes brutos aceites pelo cast (17 no total)
enum PdfStandard { V_1_4, V_1_5, V_1_6, V_1_7, V_2_0,
    A_1b, A_1a, A_2b, A_2u, A_2a, A_3b, A_3u, A_3a, A_4, A_4f, A_4e, UA_1 }

// conjunto validado e resolvido
struct PdfStandards { config: krilla::configure::Configuration }
// Configuration { validators: Validators (archival: Option<Archival>,
//                 accessibility: Option<Accessibility>), version: PdfVersion }
```

Tabela de compatibilidade de versão medida diretamente em
`krilla/src/configure/validate.rs:993-1013,1187-1201` (não em `typst-pdf`):

| Perfil | min | max |
|---|---|---|
| `a-1a`, `a-1b` | PDF 1.4 | PDF 1.4 |
| `a-2a`, `a-2b`, `a-2u`, `a-3a`, `a-3b`, `a-3u` | PDF 1.4 | PDF 1.7 |
| `a-4`, `a-4f`, `a-4e` | PDF 2.0 | PDF 2.0 |
| `ua-1` | PDF 1.4 | PDF 1.7 |

Resolução da versão efetiva (`krilla/src/configure/mod.rs:68-96`, medida via
`typst compile`): versão explícita dentro do range da interseção dos
validators escolhidos → usa-a; fora do range → erro; sem versão explícita e
com validators → usa o **máximo** do range da interseção (`a-2b` sozinho →
efetivo `1.7`; `a-1b` sozinho → efetivo `1.4`); sem validators e sem versão
→ default `1.7`. Interseção vazia entre validators → erro distinto do erro
de versão explícita fora do range.

Mensagens exatas medidas via `typst eval`/`typst compile` em 2026-09-21
(`/usr/local/bin/typst`, SHA-256 confirmado idêntico ao pin):

```text
("1.4","1.7")                    -> "PDF cannot conform to PDF 1.4 and PDF 1.7 at the same time"
("a-1b","a-2b")                  -> "choose at most one PDF/A standard"
("ua-1","ua-1")                  -> "choose at most one PDF/UA standard"
("ua-1","2.0")                   -> "PDF 2.0 is not compatible with PDF/UA-1"
                                      hint: "PDF/UA-1 requires a version between PDF 1.4 and PDF 1.7"
("a-1b","1.7")                   -> "PDF 1.7 is not compatible with PDF/A-1b"
                                      hint: "PDF/A-1b requires version PDF 1.4"  (min==max: sem "between")
("a-4","1.7")                    -> "PDF 1.7 is not compatible with PDF/A-4"
                                      hint: "PDF/A-4 requires version PDF 2.0"
"3.0" (nome inválido)             -> "expected \"1.4\", \"1.5\", \"1.6\", \"1.7\", \"2.0\", \"a-1b\",
                                      \"a-1a\", \"a-2b\", \"a-2u\", \"a-2a\", \"a-3b\", \"a-3u\", \"a-3a\",
                                      \"a-4\", \"a-4f\", \"a-4e\", \"ua-1\", or array"
17 (inteiro)                      -> mesma mensagem + ", found integer"
none                               -> mesma mensagem + ", found none"  (standard NÃO tem "none"
                                      explícito como pages — regra vazia é a única forma de "ausente")
(1, "x") array com elemento str   -> "expected integer or dictionary, found string"  (nota: este é o
                                      erro de PageRange, não de PdfStandard — PdfStandard dentro de
                                      array espera string, não dict/int)
```

### Contrato

Acrescentar ao owner os tipos públicos:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PdfVersion { V1_4, V1_5, V1_6, V1_7, V2_0 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PdfArchival { A1a, A1b, A2a, A2b, A2u, A3a, A3b, A3u, A4, A4f, A4e }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PdfAccessibility { Ua1 }

/// Um dos 17 nomes brutos aceites pelo cast documental/CLI, antes de
/// categorizados em versão/archival/accessibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PdfStandard {
    V1_4, V1_5, V1_6, V1_7, V2_0,
    A1a, A1b, A2a, A2b, A2u, A3a, A3b, A3u, A4, A4f, A4e,
    Ua1,
}

/// Conjunto validado e resolvido de standards PDF.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PdfStandards {
    pub version: PdfVersion,
    pub archival: Option<PdfArchival>,
    pub accessibility: Option<PdfAccessibility>,
}

impl Default for PdfStandards {
    fn default() -> Self {
        Self { version: PdfVersion::V1_7, archival: None, accessibility: None }
    }
}

/// Motivo estruturado de rejeição; o consumer (compiler/eval/rules.rs)
/// formata a mensagem final exata a partir destes dados — este owner não
/// formata texto de diagnóstico (fronteira igual à de outros carriers L1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdfStandardsConflict {
    ConflictingVersions(PdfVersion, PdfVersion),
    MultipleArchival,
    MultipleAccessibility,
    NoOverlappingRange,
    VersionOutOfRange { version: PdfVersion },
}

impl PdfStandards {
    /// Valida uma lista de nomes brutos e produz o conjunto resolvido, na
    /// mesma ordem/regras medidas em `krilla::configure::ConfigurationBuilder`.
    pub fn new(
        names: impl IntoIterator<Item = PdfStandard>,
    ) -> Result<Self, PdfStandardsConflict> { /* ver Contrato acima */ }
}
```

`PdfFormatOptions` ganha `pub standard: Option<PdfStandards>` (default
`None` — documento não decidiu; distinto de `Some(PdfStandards::default())`,
que também é alcançável explicitamente via `#set format.pdf(standard: ())`,
array vazio — ver `compiler/eval/rules.md` P1469). `PdfStandards` deriva
`Eq`/`Hash` (todos os campos são enums simples); `PdfFormatOptions` continua
a derivar `Eq` (já conquistado em P1468).

### Scope-out explícito (ver nota no topo desta secção)

Este owner não implementa nem afirma:

- conformidade estrutural real PDF/A (restrições de cor/transparência/fontes,
  identificação XMP com nível de conformidade correto, ausência de
  encriptação, etc.);
- conformidade estrutural real PDF/UA (validação de estrutura de tags além
  do que P1140.5-A/P1405 já materializam, contraste, alt-text obrigatório);
- qualquer mudança estrutural do PDF específica de cada versão além do
  header declarado (`infra/export/builder.md` P1469) — xref, tipos de
  objeto e o resto do corpo do PDF continuam idênticos independentemente da
  versão declarada;
- validação por ferramenta externa (nenhuma disponível neste ambiente).

Um passo futuro, quando houver uma biblioteca de geração PDF/A/UA conforme
ou um validador disponível, pode reabrir estes eixos sem quebrar este
contrato (que continua correto como validação/resolução de *nomes* e
*versão*, independentemente da conformidade estrutural).

### Aceitação e gate

Testes provam: `PdfStandards::new` para cada um dos cinco conflitos
estruturados (`ConflictingVersions`, `MultipleArchival`,
`MultipleAccessibility`, `NoOverlappingRange`, `VersionOutOfRange`);
resolução de versão efetiva (explícita válida, ausente com validators →
máximo do range, ausente sem validators → default `1.7`); a tabela de
min/max por perfil (14 casos); `PdfStandards::default()`; `Eq`/`Hash`/clone;
ortogonalidade com `pages`/`tagged`/`pretty`/SVG/PNG/HTML. Este é um dos L0s
do gate ADR-0127 de P1469 (campo público novo, flag antes inerte torna-se
funcional); aguarda confirmação humana explícita posterior ao commit causal
somente-L0 antes de qualquer teste RED ou código produtivo.
