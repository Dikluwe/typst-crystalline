# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/pipeline/pdf.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `0f804b32ff6c2be0d7629606887e1167df7cdff684783f2b30c3b5e385d1eaf8`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Pipeline — pdf
Hash do Código: a5e2cbda

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11

**Camada:** L3
**Consumer único:** `03_infra/src/pipeline/pdf.rs`

## Medição anterior à individualização

Baseline: HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree
não commitado observado em 2026-09-26. Os corpos vigentes do pipeline
e a medição por função estão no diagnóstico
`00_nucleo/diagnosticos/pipeline-atomization.md`. A classificação é
reorganização interna: assinatura, defaults, fases, diagnósticos e
observáveis de linguagem são conservados. Não é medição nova de paridade.

## Contrato proprietário

Possui todas as fachadas PDF, os resolvedores documentais e o dispatch font-aware. Conserva os wrappers de compatibilidade, DocumentID, StreamMode, tagged, pretty, pages, standard, diagnósticos UA-1, attachments e oráculo. Recebe o documento paginado pelo owner próprio.

A fachada `00_nucleo/prompts/infra/pipeline.md` conserva os caminhos
públicos por reexport explícito. Helpers compartilhados têm visibilidade
`pub(super)`, limitada à família pipeline; nenhuma API externa é adicionada.
Os corpos são transferidos sem alterar algoritmo, fases ou políticas.
As medições e os rótulos históricos de gate nas seções transferidas
conservam a proveniência original; esta individualização não reabre gates
nem atribui nova conclusão de paridade. Referências ao antigo pipeline.rs
com números de linha são medições históricas anteriores à extração.

## `compile_to_pdf_bytes`

```rust
pub fn compile_to_pdf_bytes(
    world: &dyn World,
    source: &Source,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

- Pipeline `eval → introspect → layout → (dispatch export)`.
- **P617** — variantes `_with_document_id` aceitam um `Option<[u8; 16]>`
  externo e propagam-no ao `PdfBuilder`. Quando `Some`, esse valor
  fixa o `xmpMM:DocumentID`; `xmpMM:InstanceID` continua aleatório.
  Quando `None`, mantém o comportamento de P615 (aleatório). As
  funções públicas sem sufixo mantêm `None` e preservam a API
  existente.
- Dispatch font-aware multi-font (Passos 140B + 141 + 146,
  ADR-0055 `IMPLEMENTADO`; decisão 5 anotada por 146):
  - **Colecciona** todas as `FontList` distintas no
    `PagedDocument` em ordem de primeira ocorrência (atravessa
    `FrameItem::Text` e `FrameItem::Group` recursivamente).
    Dedup estrutural via `Vec::contains` (O(N²); N tipicamente
    pequeno).
  - Para cada `FontList`, **itera todas as famílias** em ordem
    (Passo 141): consulta
    `world.book().select(name, &FontVariant::default())` →
    índice, depois `world.font(index)` → bytes. **Primeira
    família a completar ambos os passos vence**. Cenário
    patológico (índice stale) não curto-circuita.
  - Filtra entries que não resolvem (silent drop).
  - Dispatch:
    - `[]` (vec vazio) → `export_pdf(&doc)` (fallback
      Helvetica Type1).
    - `[(_, bytes)]` (uma única) →
      `export_pdf_with_font(&doc, &bytes)` (caminho preservado
      do 140B/141 — output `/CrystallineFont` único).
    - `many` (2+) → `export_pdf_multifont(&doc, many)` —
      resource dict `/F1..N` com `/CrystallineFont1..N`; cada
      `FrameItem::Text` selecciona `/F{i+1}` por match
      estrutural contra a sua `style.font` (default `/F1`
      quando `style.font` é `None` ou não casa).
- **Multi-font per document** (ADR-0055 decisão 5,
  materializada no Passo 146): N fonts distintas → N
  `/Subtype /Type0` no PDF. Single-font como caso particular
  (preservado por dispatch).
- **Instrumentação de benchmark** (Passo 518): a struct
  `Timings` expõe `shape_ms` e `subset_ms` para permitir a
  análise de gargalos do pipeline de produção real. O tempo
  de `render_ms` passa a ser o tempo de export PDF *após* o
  subsetting.
- **Array fallback chain** (ADR-0055 decisão 4, Passo 141):
  dentro de uma `FontList`, todas as famílias são tentadas
  até resolver.
- Selecção usa `FontVariant::default()` (regular, normal, normal
  stretch). `weight`/`style` no documento continuam a ser
  renderizados via faux-bold/faux-italic (Passo 139) — selecção
  variant-aware é candidato ADR-0055bis.
- Warnings sempre devolvidos (mesmo em erro) — caller decide se
  os imprime.
- Módulo sem `content` (AST puramente executivo) produz
  `Ok(Vec::new())` — não é erro.

## P956 — `stream_mode` nas entry points PDF

ADR-0126 (emendada P956): todas as variantes `compile_to_pdf_bytes*`
(`compile_to_pdf_bytes`, `_with_timings`, `_with_timings_full_error`,
`_with_timings_full_error_and_document_id`, `_full_error`,
`_full_error_and_document_id`) ganham `stream_mode: StreamMode` como **último
parâmetro**, propagado ao dispatch de export (`export_pdf*` —
`infra/export/mod.md` §P956). Quebra de assinatura deliberada (mesma razão de
`mod.md` §P956: caller nenhum fica ambíguo sobre o modo).

- O dispatch font-aware (`export_pdf` / `export_pdf_with_font` /
  `export_pdf_multifont`) passa o modo recebido a qualquer dos três ramos.
- `compile_to_png_bytes*` / `compile_to_svg_string*` **inalterados** — a flag
  `--compact` não tem significado fora do PDF (ver `shell/cli.md` §P956).

## P1140.6 — threading de `PdfTags`

### Medição antes da decisão

As entry points PDF propagam apenas `StreamMode`; a pipeline não representa o
default tagueado medido no vanilla nem um caminho explícito sem tags.

### Decisão

Todas as variantes públicas `compile_to_pdf_bytes*` recebem `pdf_tags:
PdfTags` como último parâmetro, depois de `stream_mode`, e o propagam sem
transformação aos três ramos `export_pdf*`. O chamador de produção L4 usa
`Enabled` na ausência de `--no-pdf-tags` e `Disabled` quando a flag está
presente. PNG, SVG e HTML permanecem inalterados. `PdfTags` não depende de
`StreamMode` e não ativa validação PDF/UA nem diagnóstico por falta de alt.

**Retificado por P1467** (ver seção própria abaixo): "o chamador de produção
L4 usa `Enabled` na ausência de `--no-pdf-tags`" deixa de ser exato quando o
documento contém `#set format.pdf(tagged: ...)`. A partir de P1467, a ausência
de decisão CLI (moderna e legada) não colapsa mais em `Enabled` — é preservada
como `None` até a pipeline poder consultar `document.format_options.pdf.tagged`.
O restante desta seção (threading de `PdfTags` até os três ramos `export_pdf*`,
independência de `StreamMode`, ausência de validação PDF/UA) permanece exato.

## P1286 — validação global de attachments sem nova fase

### Medição anterior à decisão

O erro vanilla de duplicado depende da coleção global final; uma nativa
isolada não consegue observá-lo. `compile_to_pdf_bytes*` já devolve
`Result<_, Vec<SourceDiagnostic>>`, enquanto entrypoints diretos do exporter
são infalíveis.

### Decisão condicionada ao gate

Depois do layout e antes do dispatch font-aware, a pipeline valida
`PagedDocument.attachments` em ordem. Nome virtual derivado repetido produz
`attempted to attach file {path} twice` e aborta antes de exportar. O builder
recebe somente a lista validada. Não alterar assinatura pública, `World`,
ordem eval→layout→export, PNG/SVG/HTML ou defaults. Entry points diretos do
exporter têm como precondição um `PagedDocument` validado; isso não substitui
o erro de linguagem no pipeline de produção.

## P1467 — resolução documental de `format.pdf.tagged`/`format.pdf.pretty` (MATERIALIZADO; gate ADR-0127 satisfeito)

### Medição anterior à decisão

No baseline `306ca50022104619447fae2625090b8d9c7b0a35`,
`compile_to_pdf_bytes_impl` (`03_infra/src/pipeline.rs:1390-1401`) recebe
`pdf_tags: PdfTags` e `serialization: PdfSerializationMode` já resolvidos
pelo caller; nunca consulta `doc.format_options` (populado em
`:1046` pela mesma função, antes do render em `:1477`). Ao contrário de
`resolve_svg_pretty` (`:1964`) e `resolve_html_pretty` (`:250`), não existe
`resolve_pdf_tags`/`resolve_pdf_pretty`: os 13 entry points públicos
`compile_to_pdf_bytes*` (incluindo os 3 oráculo) tomam o enum já decidido e o
repassam sem transformação aos três ramos `export_pdf*` font-aware.

Medido em 2026-09-20 com `/usr/local/bin/typst` (vanilla ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`, SHA-256 confirmado idêntico ao
pin): `#set format.pdf(tagged: false)` root produz PDF sem
`/StructTreeRoot`/`/MarkInfo`; ausência da regra produz PDF tagueado (auto
resolve habilitado sem seleção de páginas); `--pdf-tagged=false` e
`--no-pdf-tags` vencem a regra do documento; `--pretty` vence
`#set format.pdf(pretty: false)`. O Núcleo `format-option-precedence` pinado
acima (`absence-defers-to-document`, `no-early-collapse`) já proíbe
exatamente o colapso hoje presente nesta função para o eixo PDF; SVG/HTML já
cumprem o Núcleo desde P1462/P1465.

### API e resolução

Introduzir dois resolvedores privados, no mesmo padrão de
`resolve_svg_pretty`/`resolve_html_pretty`:

```rust
fn resolve_pdf_tags(
    tagged_override: Option<bool>,
    options: &DocumentFormatOptions,
) -> PdfTags {
    let enabled = match tagged_override {
        Some(explicit) => explicit,
        None => match options.pdf.tagged {
            PdfTagged::Explicit(value) => value,
            // Sem seleção de páginas (P1468 revisita): auto resolve habilitado.
            PdfTagged::Auto => true,
        },
    };
    if enabled { PdfTags::Enabled } else { PdfTags::Disabled }
}

fn resolve_pdf_pretty(
    pretty_override: Option<bool>,
    options: &DocumentFormatOptions,
) -> PdfSerializationMode {
    if pretty_override.unwrap_or(options.pdf.pretty) {
        PdfSerializationMode::Pretty
    } else {
        PdfSerializationMode::Compact
    }
}
```

Acrescentar três entry points públicas explícitas, análogas às `_override` já
existentes para PNG/SVG/HTML, que substituem `pdf_tags: PdfTags` por
`pdf_tagged_override: Option<bool>` e `serialization: PdfSerializationMode`
por `pretty_override: Option<bool>`:

```rust
pub fn compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: Features,
    pretty_override: Option<bool>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings)

pub fn compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: Features,
    pretty_override: Option<bool>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>)

pub fn compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: Features,
    pretty_override: Option<bool>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>)
```

`compile_to_pdf_bytes_impl` troca seus parâmetros `pdf_tags: PdfTags` e
`serialization: PdfSerializationMode` pelos dois overrides acima. Imediatamente
depois de obter `doc` (logo após `:1405-1415`, antes da resolução de fontes),
chamar os dois resolvedores uma única vez e vincular os nomes locais `pdf_tags`
e `serialization` já usados pelo resto do corpo da função — nenhuma outra
linha do corpo muda. Os três ramos `export_pdf*` continuam recebendo os enums
já resolvidos e continuam ignorantes de CLI/documento/override.

Os 10 entry points públicos pré-existentes que hoje recebem `pdf_tags`/
`serialization` já resolvidos **preservam assinatura e comportamento**: cada
um passa a delegar para a nova entry point equivalente envolvendo seu próprio
valor em `Some(...)` (`Some(matches!(pdf_tags, PdfTags::Enabled))` para o eixo
tagged; `Some(matches!(serialization, PdfSerializationMode::Pretty))` para o
eixo pretty), reproduzindo exatamente o comportamento anterior — um override
explícito sempre vence, então o documento nunca é consultado por esses
caminhos de compatibilidade. Nenhuma assinatura pública existente quebra;
somente as três entry points novas expõem a consulta documental, e são elas
que L4 passa a chamar (ver `wiring.md` P1467).

Este fio aplica o Núcleo `format-option-precedence` pinado acima:
`Some` explícito vence, `None` é preservado até o documento, e o default por
formato (`auto`→habilitado sem páginas; `false` para pretty) só se aplica
quando caller e documento silenciam. `pdf_tagged_override`/`pretty_override`
não alteram `StreamMode`, features, `document_id`, timings nem os três ramos
font-aware. PNG, SVG e HTML permanecem inalterados.

### Neutralidade e aceitação

`tagged`/`pretty` não alteram eval, layout, PNG, SVG, HTML, `compact`,
`document_id`, timings, resolução de fontes ou o caminho oráculo além do que
já é medido (oráculo aplica a mesma resolução, sem mudar a transformação de
paridade de operador). Testes de pipeline cobrem: `None`/`None` com documento
`Auto`/`false` (habilitado, compacto), `None`/`None` com documento explícito
nos dois campos, `Some` explícito vencendo documento nos dois sentidos para
cada campo, os dois campos definidos independentemente, orthogonalidade com
`compact`/PPI/serialização HTML, e que as 10 entry points de compatibilidade
preservam exatamente seu comportamento anterior (regressão zero). Não
congelar bytes PDF íntegros como contrato; inspecionar presença de
`/StructTreeRoot`/`/MarkInfo` e a forma de serialização (whitespace
estrutural), nunca igualdade total de bytes.

Este é um dos quatro L0s do gate ADR-0127 de P1467 (mudança de comportamento
por defeito do produto quando o documento contém `#set format.pdf(...)`).

A confirmação humana explícita posterior ao commit causal somente-L0
`e675e8b7acc29e272bd59939e44681a7b521acd3` foi recebida em 2026-09-20.
Agente A implementou `resolve_pdf_tags`/`resolve_pdf_pretty` e as três
entry points `_tagged_and_pretty_override` sem ver os testes, fazendo as 10
entry points de compatibilidade delegarem preservando comportamento; Agente
B escreveu 11 testes de pipeline a partir do mesmo baseline sem ver a
implementação (RED por erro de compilação — entry point inexistente). O
teste independente passou de RED a GREEN (79/79 no módulo `pipeline::tests`,
incluindo os 11 novos e os pré-existentes de SVG/PNG/HTML) após a
integração, sem regressão.

## P1468 — resolução de `format.pdf.pages` e interação com `tagged` (MATERIALIZADO; gate ADR-0127 satisfeito)

### Medição anterior à decisão

No vanilla ratificado, a resolução acontece em dois pontos distintos:

1. **CLI-only, antes de compilar** (`typst-cli/src/compile.rs:162-165`): se
   `--pdf-tagged` é explicitamente `true` **e** `--pages` está presente,
   `bail!("cannot enable PDF tags when exporting a page range")` — sem sequer
   compilar o documento. Medido: exit 1, sem tentar layout.
2. **Depois de compilar e resolver com o documento**
   (`typst-pdf/src/lib.rs:95-118`, `PdfOptions::resolve`): combina override
   (CLI) com `doc: &PdfFormatOptions` primeiro; só então, se `pages` (já
   resolvido) está presente:
   - `tagged` resolvido `== Custom(true)` → erro
     `"cannot enable tagged PDF and export a page range"` (só alcançável
     quando é o **documento** que pede `tagged: true`, porque o caso CLI já
     foi barrado no ponto 1);
   - `tagged` resolvido `.is_auto()` → warning
     `"exporting a page range disables PDF tagging"` com hints
     `"the resulting PDF will be inaccessible"` e
     `"set \`pdf(tagged: false)\` to silence this warning"`, e o tagged
     efetivo final é `false`;
   - `tagged` resolvido `== Custom(false)` → nenhum diagnóstico, tagged
     efetivo `false`.

Medido também: quando **nenhuma** flag CLI de tagging está presente mas
`--pages` está, o vanilla emite o warning CLI `"using \`--pages\` implies
\`--pdf-tagged=false\`"` com hints próprios (diferente do warning
documental — `compile.rs:168-177`); quando `--pdf-tagged=false` é explícito
junto de `--pages`, nenhum warning é emitido. Isto é uma terceira via de
diagnóstico (CLI, ausência de decisão), distinta das duas acima.

### Contrato

Ampliar `resolve_pdf_tags` (P1467) para receber também `pages_override:
Option<PageRanges>` e devolver, além de `PdfTags`, o `PageRanges` efetivo e
os diagnósticos aplicáveis:

```rust
fn resolve_pdf_pages_and_tags(
    pages_override: Option<PageRanges>,
    tagged_override: Option<bool>,
    options: &DocumentFormatOptions,
) -> SourceResult<(Option<PageRanges>, PdfTags, Vec<SourceDiagnostic>)>
```

Precedência de `pages`: `pages_override` (CLI) vence `options.pdf.pages`
(documento) vence `None` (todas as páginas) — mesmo Núcleo
`format-option-precedence` já pinado, sem colapso antecipado.

Com `pages` resolvido `Some(_)`:

- se `options.pdf.tagged == PdfTagged::Explicit(true)` **e** não há
  `tagged_override` que já tenha vencido com `Some(false)` — isto é, o
  tagged efetivo (pela mesma regra de `resolve_pdf_tags`) resolveria
  `Enabled` só por causa do documento — devolver `Err` com a mensagem
  documental exata acima. A checagem CLI-only (`tagged_override ==
  Some(true)` + `pages_override.is_some()`) **não** pertence a este owner —
  já foi barrada em `wiring.md` antes de a pipeline ser chamada;
- caso contrário, se o tagged efetivo resolveria `Enabled` só porque
  `options.pdf.tagged == PdfTagged::Auto` (nem CLI nem documento pediram
  explicitamente) → produzir o warning documental exato (2 hints) e forçar
  `PdfTags::Disabled`;
- se o tagged efetivo já resolveria `Disabled` (CLI ou documento
  explícitos) → `PdfTags::Disabled`, sem diagnóstico novo.

Com `pages` resolvido `None`: comportamento idêntico a P1467
(`resolve_pdf_tags` inalterado nesse ramo).

O warning/erro CLI de "ausência de decisão + `--pages` presente" (terceira
via medida acima) pertence a `wiring.md`, não a este owner — a pipeline só
produz o diagnóstico **documental**. As três entry points
`_tagged_and_pretty_override` (P1467) ganham `pages_override: Option<PageRanges>`
como novo último parâmetro, renomeadas para `_tagged_and_pretty_and_pages_override`;
as 3 entry points `_tagged_and_pretty_override` de P1467 continuam a existir
e delegam `None` para `pages_override`, preservando comportamento exato (10 +
3 entry points de compatibilidade, todas byte-idênticas ao que já produzem
hoje). O `PageRanges` efetivo é passado à camada de export
(`infra/export/mod.md`/`infra/export/builder.md`) para reindexar páginas,
links, destinos e outline.

### Aceitação e gate

Testes de pipeline cobrem: `pages` ausente (comportamento P1467 idêntico),
`pages` presente com `tagged` documental `Explicit(true)` (erro), `Auto`
(warning + Disabled), `Explicit(false)` (Disabled sem warning), precedência
CLI/documento de `pages`, e que as três novas entry points delegam
corretamente das antigas. Este é um dos L0s do gate ADR-0127 de P1468
(mudança de comportamento por defeito, novos diagnósticos).

A confirmação humana explícita posterior aos commits causais somente-L0
`9bb20b8e8`/`70d09eae2` foi recebida em 2026-09-20/21. Agente A implementou
`resolve_pdf_pages_and_tags` e as três entry points `_and_pages_override`
sem ver os testes (removendo os imports das três funções de export sem
`_and_pages` que ficaram órfãs — correção mecânica de integração, papel C);
Agente B escreveu 10 testes a partir do mesmo baseline sem ver a
implementação (RED por erro de compilação). O teste independente passou de
RED a GREEN (89/89 no módulo `pipeline::tests`, incluindo os 10 novos e os
79 pré-existentes) após a integração, sem regressão.

## P1469 — resolução de `format.pdf.standard` e interação com `tagged`/`pages` (ESCOPO REDUZIDO — contrato de linguagem) (MATERIALIZADO; gate ADR-0127 satisfeito)

Ver `entities/document_format_options.md` P1469 para a nota de escopo
reduzido (decisão do dono, 2026-09-21): esta secção resolve *nomes* e
*versão efetiva*, não conformidade estrutural.

### Medição anterior à decisão

No vanilla ratificado (`typst-pdf/src/lib.rs:120-143`, já citado em
`infra/pipeline.md` P1468), depois de resolver `pages`/`tagged` (checagem já
materializada em P1468), uma segunda checagem itera os quatro perfis
acessíveis (`A_1a`, `A_2a`, `A_3a`, `UA_1`): se algum está no conjunto de
standards **já resolvido** (CLI combinado com documento) e o `tagged`
**final** (já considerando a checagem de `pages` anterior) é `false`, falha:

```text
tagged_final == false
  E standards_final.contains(a-1a|a-2a|a-3a|ua-1)
    -> "cannot set `pdf(tagged: false)` when exporting a {name} document"
    -> hint "using `pdf(pages: ...)` implies `pdf(tagged: false)`" SÓ SE
       pages estava ativo E tagged não foi explicitamente `false` por CLI/documento
       (isto é, a causa raiz do `false` foi o colapso por `pages`, não uma escolha)
```

Esta é a checagem **documental completa** (depois de CLI+documento
combinados); a checagem CLI-only equivalente (só a lista crua da CLI, antes
de compilar) pertence a `wiring.md` P1469, mesmo padrão de duas camadas já
usado para `pages`+`tagged` em P1467/P1468.

### Contrato

Ampliar `resolve_pdf_pages_and_tags` (P1468) para
`resolve_pdf_pages_tags_and_standards`, recebendo também `standard_override:
Option<PdfStandards>` (já validado por `wiring.md` a partir da lista crua da
CLI — a pipeline não valida conflitos entre nomes CLI, só resolve
precedência) e devolvendo também o `PdfStandards` efetivo:

```rust
fn resolve_pdf_pages_tags_and_standards(
    pages_override: Option<PageRanges>,
    tagged_override: Option<bool>,
    standard_override: Option<PdfStandards>,
    options: &DocumentFormatOptions,
) -> SourceResult<(Option<PageRanges>, PdfTags, PdfStandards, Vec<SourceDiagnostic>)>
```

Ordem de resolução (preserva P1468 integralmente, acrescenta um passo
final):

1. `pages_final = pages_override.or_else(|| options.pdf.pages.clone())`.
2. `standards_final = standard_override.unwrap_or_else(|| options.pdf.standard
   .clone().unwrap_or_default())` — precedência `CLI > documento >
   PdfStandards::default()`, mesmo Núcleo `format-option-precedence` já
   pinado.
3. Resolver `tagged` exatamente como P1468 (considerando `pages_final`):
   produz `PdfTags` e os diagnósticos de `pages`+`tagged` já existentes.
4. Se `standards_final.archival` é `Some(A1a|A2a|A3a)` **ou**
   `standards_final.accessibility` é `Some(Ua1)`, e o `PdfTags` do passo 3 é
   `Disabled`: falha com a mensagem documental exata acima. O hint condicional
   só aparece quando a causa raiz de `Disabled` foi `pages_final.is_some()`
   sem um `tagged` explícito `false` de CLI/documento — reaproveitar a
   informação já calculada no passo 3 para decidir isto, sem recalcular.

As três entry points `_tagged_and_pretty_and_pages_override` (P1468) ganham
`standard_override: Option<PdfStandards>` como novo último parâmetro,
renomeadas para `_tagged_and_pretty_and_pages_and_standard_override`; as três
antigas de P1468 continuam a existir e delegam `None` para `standard`
(comportamento byte-idêntico preservado — default `PdfStandards::default()`
quando o documento também não decide).

O `PdfStandards` efetivo (versão + archival + accessibility) é passado à
camada de export (`infra/export/mod.md`/`infra/export/builder.md` P1469)
para ajustar o header `%PDF-{version}`. Este owner não sabe que existe
`krilla`, não valida conformidade estrutural e não altera a estrutura
interna do PDF além da versão declarada.

### Aceitação e gate

Testes de pipeline cobrem: `standard` ausente (comportamento P1468 idêntico,
default `1.7`); `standard` presente sem perfil acessível + `tagged: false`
(sem erro); perfil acessível + `tagged` documental `Explicit(false)` (erro,
sem hint de pages); perfil acessível + `pages` ativo sem `tagged` explícito
(erro, com hint de pages); perfil acessível + `tagged: true`/documento sem
decisão (sem erro, tagged habilitado); precedência CLI/documento de
`standard`; e que a nova entry point delega corretamente da antiga quando
`standard` é `None`. Este é um dos L0s do gate ADR-0127 de P1469 (mudança de
comportamento por defeito, novos diagnósticos).

A confirmação humana explícita posterior aos commits causais somente-L0
`65e68ad45`/`e14002d55` foi recebida em 2026-09-21. Agente A implementou
`resolve_pdf_pages_tags_and_standards` como um wrapper sobre
`resolve_pdf_pages_and_tags` (P1468, preservada sem alteração — evita
qualquer risco de regressão nos testes já existentes dessa função) e as três
entry points `_and_standard_override` sem ver os testes; Agente B escreveu 8
testes a partir do mesmo baseline sem ver a implementação. O teste
independente passou de RED (erro de compilação) a GREEN (97/97 no módulo
`pipeline::tests`, incluindo os 8 novos e os 89 pré-existentes) após a
integração, sem regressão.

## P1480 — PDF/UA-1 exige `document(title:)`: novo erro de compilação

Ver `typst-passo-1480.md` §0 para o achado de integração completo (owner
original era só `infra/export/builder.md`; este owner foi trazido para
dentro do escopo pela mesma medição). Gate ADR-0127 disparado — critério
(2) novo comportamento por defeito de uma flag já existente
(`--pdf-standard ua-1`) e (4) quebra de compatibilidade para quem hoje
depende do sucesso silencioso; humano confirmou inclusão no escopo em
chat, 2026-09-22, registrado em `typst-passo-1480.md` §0 antes desta
edição de L0.

### Medição anterior à decisão

`/usr/local/bin/typst compile --pdf-standard ua-1 <fixture sem
document(title:)>`: exit 1, `error: PDF/UA-1 error: missing document
title` + hint `set the title with \`set document(title: [...])\``.
Cristalino (mesmo commit `90aad760b`, `target/release/typst`): exit 0,
PDF emitido sem título, não-conforme, sem qualquer diagnóstico —
divergência confirmada por medição direta bilateral, não inferida.

Fonte vanilla: `krilla::configure::validate::ValidationError::
NoDocumentTitle`, convertida em `SourceDiagnostic` por
`typst-pdf/src/convert.rs:748-752` (mensagem) e `:535` (`prefix =
"{validators} error:"`, onde `{validators}` é a lista dos validadores
falhados — só `"PDF/UA-1"` no caso isolado desta fixture; combinações
com archival ficam fora do escopo desta medição, não testadas).

### Contrato

1. Checagem nova em `compile_to_pdf_bytes_impl`, logo após
   `resolve_pdf_pages_tags_and_standards` devolver `Ok` (mesmo ponto em
   que `pdf_standards`/`doc` já estão resolvidos), mesmo padrão do erro
   de anexo duplicado já existente na função (`attachment_paths`,
   alguns parágrafos abaixo): `if matches!(pdf_standards.accessibility,
   Some(PdfAccessibility::Ua1)) && doc.document_info.title.is_none() {
   return Err(...) }`.
2. Mensagem e hint byte-idênticos ao vanilla medido acima:
   `"PDF/UA-1 error: missing document title"` +
   `.with_hint("set the title with \`set document(title: [...])\`")`.
   Não generalizar para uma lista de validadores combinados (`prefix`
   dinâmico do vanilla) — fora do escopo desta medição; fixo em
   `"PDF/UA-1"` até uma medição futura confirmar o comportamento
   combinado.
3. Não muda a assinatura pública de `export_pdf*`
   (`03_infra/src/export/mod.rs`) — continuam infalíveis, `-> Vec<u8>`.
   A família `compile_to_pdf_bytes_*` já é `Result<Vec<u8>,
   Vec<SourceDiagnostic>>` desde P1468/P1469; este é mais um ramo de
   erro na mesma família, não uma mudança de contrato dela.
4. Título vazio (`document(title: [])`) não é medido aqui — só o eixo
   `None` vs `Some`. Não inventar comportamento para esse caso.

### Aceitação e gate

RED: `ua-1` sem título compila com exit 0 hoje (estado atual,
confirmado). GREEN: mesma combinação retorna `Err` com a mensagem
exata acima; `ua-1` com título continua a compilar normalmente (controlo
de não-regressão); nenhum outro perfil (archival isolado, ausência de
standard) ganha esta checagem — só `accessibility == Some(Ua1)`. Fluxo
contínuo após este L0 (a paragem ADR-0127 já foi satisfeita pela
confirmação humana registrada em `typst-passo-1480.md` §0).

## P1481 — validação de alt ausente sob PDF/UA-1 (IMPLEMENTADO; confirmado em 2026-09-23)

Medição bilateral do corpus `p1481-corpus.typ` (SHA-256
`90475d9f6bd2b6964495861203c480af064b7273c14dc0feb492f79444dbd7d4`):
com `image(alt: "A small test image")` o vanilla compila e passa
veraPDF UA-1 106/0. Sem `alt` (fixture SHA-256
`7d3b364ffe1e979d127cf303a2f13b55ad969d7d844aea831c549d6f058ae504`),
o vanilla recusa compilar: `PDF/UA-1 error: missing alt text`,
hint `make sure your images and equations have alt text`
(`typst-pdf/src/convert.rs:720-725`). O cristalino hoje rejeita
`alt` na avaliação e aceita uma imagem sem descrição até export.

Contrato confirmado: depois de resolver standards e documento, rejeitar
imagem real com `alt == None` sob UA-1 com essa mensagem e hint;
`Some("")` permanece valor fornecido, sujeito às regras externas
do perfil. Respeitar `pdf.artifact`: decoração sob seu envelope não
precisa de alt. Sem UA-1, não criar esse erro. O erro usa span de
origem se houver carrier; span detached segue a limitação existente
de `native_image` até um passo próprio. Isto muda o comportamento
de uma flag já existente e depende de `ImageElem.alt` público:
Gate ADR-0127 satisfeito em 2026-09-23.
