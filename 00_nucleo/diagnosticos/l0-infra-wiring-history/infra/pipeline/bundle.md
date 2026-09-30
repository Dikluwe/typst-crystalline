# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/pipeline/bundle.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `46b1d95a2de001fee1e4820a382101c6c4dd90d355834837f003b266b75c4243`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Pipeline — bundle
Hash do Código: 942cdeea

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c

**Camada:** L3
**Consumer único:** `03_infra/src/pipeline/bundle.rs`

## Medição anterior à individualização

Baseline: HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree
não commitado observado em 2026-09-26. Os corpos vigentes do pipeline
e a medição por função estão no diagnóstico
`00_nucleo/diagnosticos/pipeline-atomization.md`. A classificação é
reorganização interna: assinatura, defaults, fases, diagnósticos e
observáveis de linguagem são conservados. Não é medição nova de paridade.

## Contrato proprietário

Orquestra o target bundle: entradas ordenadas, paths e colisões, snapshots globais, fronteiras Page por body e exportação por formato. Devolve a coleção completa ou erro; o write continua responsabilidade de L4. Reutiliza helpers de paged, html, pdf, svg e resources sem reconstruir snapshots isolados.

A fachada `00_nucleo/prompts/infra/pipeline.md` conserva os caminhos
públicos por reexport explícito. Helpers compartilhados têm visibilidade
`pub(super)`, limitada à família pipeline; nenhuma API externa é adicionada.
Os corpos são transferidos sem alterar algoritmo, fases ou políticas.
As medições e os rótulos históricos de gate nas seções transferidas
conservam a proveniência original; esta individualização não reabre gates
nem atribui nova conclusão de paridade. Referências ao antigo pipeline.rs
com números de linha são medições históricas anteriores à extração.

## P1485 — compilação multi-arquivo do target bundle (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, os
entrypoints L3 de `03_infra/src/pipeline.rs` devolvem um único
PagedDocument, HTML string ou bytes de formato único; não há
coleção de arquivos. No vanilla pinado,
`typst-bundle/src/lib.rs:112-305` realiza top-level bundle,
recolhe `asset`/`document`, rejeita conteúdo solto, detecta
paths duplicados após normalização e compila cada documento no
target PDF/SVG/PNG/HTML; `typst-bundle/src/export.rs:23-113`
devolve map path→bytes. As sondas bilaterais com payload, docs
mistos, duplicatas e HTML estão em
`00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md`.
Uma função `asset` que apenas retorna Content não pode produzir
essa árvore; supor o contrário é refutado pelo output medido.

### Decisão proprietária

Adicionar entrypoint L3 explícita, com assinatura e carriers:

```rust
pub struct BundleFile {
    pub path: String,         // relativo, normalizado e validado
    pub data: Vec<u8>,        // bytes finais do arquivo
    pub is_html_document: bool, // semântica do documento, não extensão
}
pub struct BundleCompileOptions {
    pub full_error: bool,
    pub features: Features,
    pub document_id: Option<[u8; 16]>,
    pub pdf_stream_mode: StreamMode,
    pub pdf_serialization: PdfSerializationMode,
    pub pdf_tagged_override: Option<bool>,
    pub pdf_pages_override: Option<PageRanges>,
    pub pdf_standard_override: Option<PdfStandards>,
    pub png_ppi_override: Option<f64>,
    pub html_serialization: HtmlSerializationMode,
    pub pretty_override: Option<bool>,
}
pub fn compile_to_bundle_files(
    world: &dyn World, source: &Source, options: BundleCompileOptions,
) -> (Result<Vec<BundleFile>, Vec<SourceDiagnostic>>,
      Vec<SourceDiagnostic>, Timings);
```

L4 transmite os overrides já parseados; ausência permanece `None`
para a precedência documental de cada formato. `pdf_stream_mode`
usa verbose por omissão e `pdf_serialization` usa compacto por
omissão; `html_serialization` conserva a escolha vanilla/cristalino
existente. `is_html_document` deriva do formato semântico do
documento, não da extensão de um asset: o servidor só injeta reload
em documento HTML. Nenhum caller recebe coleção parcial em erro.
O eval root usa target Bundle somente quando selecionado e
habilitado; top-level permite
`document`, `asset` e tags, não texto solto. Asset passa
path/payload sem conversão de formato; `Str` já foi convertido
a UTF-8 no constructor L1. Document seleciona formato por
extensão ou propriedade explícita e usa o pipeline paginado ou
HTML sem converter HTML a páginas (ADR-0128). HTML requer
`Feature::Html` adicional. Formato desconhecido, colisão,
path inválido, documento de imagem com mais de uma página e
conteúdo top-level não permitido são diagnósticos antes de
qualquer write. O mapa final não atribui sucesso parcial.

Reutilizar exporters e opções por formato quando aplicáveis,
sem copiar mecânica vanilla de paralelismo, Arc, introspector
ou bytes de PDF/SVG como critério de paridade. Introspecção,
labels e contexto entre documentos são semântica de linguagem;
se não estiverem íntegros, P1485 continua aberto e a lacuna
é medida, não convertida em MATCH. L3 não grava no diretório
final; `00_nucleo/prompts/wiring.md` possui a composição do write. Qualquer
novo endpoint público ou mudança de fase eval→layout exige
confirmação ADR-0127.

## P1485.2 — continuidade global de Locations no bundle (ratificado; ADR-0127)

### Medição anterior à decisão

Com P1485 materializado na working tree não commitada sobre
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, o bundle
valida todos os paths e emite assets, PDF/SVG/PNG/HTML, HTTP e
live reload. Os testes independentes B têm 8/9 GREEN; o único
RED extrai `1. Beta` de `b.pdf` onde o vanilla pinado
`586e1bd43` extrai `2. Beta`. O primeiro PDF extrai
`1. Alpha` em ambos. A causa medida é a divergência entre
o `Locator` do introspector global
(`01_core/src/compiler/introspect.rs:743-765`) e o
`Locator::new()` de cada layout
(`01_core/src/compiler/layout/mod.rs:1108,3423-3424`).
`context_stabilization.rs:408-423` também reintrospecta só o
body selecionado. O diagnóstico, fixtures, horas e estado Git
estão no Passo 1485. Sem corrigir os dois eixos, P1485 permanece
aberto por esta própria especificação (§P1485 acima).
Classificação ADR-0107: numeração, consultas, referências e
diagnósticos são observáveis da linguagem; seeds, iterações,
estrutura dos arquivos e algoritmo de coordenação são mecânica.
O resultado vanilla não demonstra intenção de usar o mesmo
algoritmo. Inferência: o seed global corrige a numeração, mas
não basta para contexto selecionado porque a reintrospecção
atual é local. Refutação: com seeds e candidato global corretos,
um caso cruzado ainda divergir; ou um contexto local demonstrar
dependência que o candidato global não consegue validar.

### Decisão ratificada para o caller L3

Depois da validação de paths, conservar a ordem dos bodies e seus
estilos herdados. Obter introspector global e um
`LocatorSeed` por body da entrypoint dona em
`00_nucleo/prompts/compiler/introspect.md`. Coordenar a
preparação contextual do conjunto pelo descendente
`00_nucleo/prompts/infra/pipeline/context_stabilization.md`:
toda seleção recompõe/reintrospecta **todos** os bodies antes
de considerar estável o snapshot; não promover o
`prepared.settled` de um documento produzido sob snapshot
anterior. Paged usa a nova entrypoint de layout com seed;
Html usa target semântico e seu exporter, sem layout paginado
exceto as ilhas `html.frame`. Revalidar referências, counters,
labels e contextos cruzados antes do primeiro write. Uma falha
aborta o conjunto, preservando publicação atômica de L4.

As APIs atuais de compilação single-file permanecem iguais e
não recebem novo default. O bundle não fabrica padding
`Metadata`, não deriva offset de `elements.len()`, não
tenta corrigir apenas a string renderizada de numbering.
Aceitação exige `1. Alpha`/`2. Beta` nos dois PDFs,
query de três headings em ambos os HTMLs, `target()`
correto por documento, mistura de formatos e provas de
contexto/refs cross-document sem sucesso parcial.
Os contratos públicos L1 dos quais a implementação depende tiveram
o gate ADR-0127 confirmado pelo dono em 2026-09-24.

## P1485.3 — isolamento lógico Page no target bundle (ratificado; ADR-0127)

### Medição anterior à decisão

L3 obtém starts por body (`03_infra/src/pipeline.rs:759-761`), mas
prepara contexto como Sequence (`03_infra/src/pipeline/context_stabilization.rs:742-745`).
Na sonda RED `p1485_bundle_html_nao_herda_page_counter_do_pdf`, HTML herda
5 do PDF anterior; vanilla pinado observa 1. Update explícito no HTML deve
continuar observável (sonda vanilla: 7). Medidas/proveniência no diagnóstico
P1485 de integração. Número exposto é semântica; Sequence/seed/histórico
são mecânica (ADR-0107). Inferência: nova introspecção runtime com fronteiras
corrige a divergência sem separar o estado global. Refutação: leitura Page
cruzada errada ou perda de query, heading, label, state ou Location.

### Decisão condicionada

O bundle conserva bodies ordenados e usa snapshots puro/runtime com as
mesmas fronteiras Page de `compiler/introspect.md` P1485.3. Não resetar
heading, state nem outra chave; não hardcodar 1 no serializer HTML. Nenhuma
nova fachada pública L3, default ou write parcial. Aceitação inclui PDF
update(5) → HTML Page 1, HTML update(7) → Page 7, PDFs com início Page local
e numbering de headings global, refs/contextos entre documentos.
`.final()` multi-documento segue fora deste contrato e não pode ser
declarado verde por essas sondas. A API pública L1 foi confirmada
pelo dono em 2026-09-24.

## P1486.1 — vista marginal local do PageStore no bundle

### Medição anterior à decisão

`03_infra/src/pipeline.rs:1026-1036` entrega a store global do bundle ao
layout de cada body; `render_bundle_document` já conhece `page_offset`.
`bundle_page_views` concatena os números lógicos e numerações visíveis
(`pipeline/context_stabilization.rs:829-915`). A fonte L1 de margem
consulta páginas por índice local, mas refs entre documentos consultam
`numbering_for_page` e `reference_numbering_for_page` com índice global
(`compiler/layout/references.rs:92-119`). O diagnóstico P1486 mostra
o desvio de margem no documento único; inferência: alinhar apenas as
vistas marginais na cópia usada para layout resolve o segundo PDF sem
deslocar refs. Refutação: bundle PDF/HTML/PDF ou PDF/SVG/PNG/PDF com
update/refs cruzadas perder número, contexto ou texto.

### Decisão privada

Antes do layout de cada body paginado com offset não zero, projetar
`logical_numbers` e `visible_numberings` do store global para índices
locais (`local_i → page_offset + local_i`) na cópia do introspector
usada somente nessa renderização. Preservar sem deslocamento o total,
`numberings`, `numbering_spans`, `reference_numberings` e `supplements`:
esses eixos continuam globais para refs e consultas. O `bundle_intr`
original, usado no bind final e demais bodies, não muda. Validar offset
contra o total; desalinhamento continua erro. Não há API pública nova
nem mudança de fase; a aceitação mede margem desenhada e rótulo PDF.
