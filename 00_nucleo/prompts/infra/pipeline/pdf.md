# L0 — infra/pipeline/pdf
Hash do Código: 69927649

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/pdf.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
```

## Current evidence

- `03_infra/src/pipeline/pdf.rs:180` — `resolve_pdf_pages_and_tags`.
- `03_infra/src/pipeline/pdf.rs:243` — `resolve_pdf_pages_tags_and_standards`.
- `03_infra/src/pipeline/pdf.rs:288` — `require_ua1_document_title`.
- `03_infra/src/pipeline/pdf.rs:308` — `require_ua1_image_alt`.
- `03_infra/src/pipeline/pdf.rs:351` — `compile_to_pdf_bytes_impl`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/pdf.rs`.

## Owned contract

- Own PDF compilation facades, resolved document options, PDF-only validations and font-aware export dispatch. Obtain a complete paged document from its owner; preserve raw warnings on both success and failure.
- Preserve every compatibility wrapper's explicit/default choices, DocumentID transport, oracle path, timings and public import paths. Some DocumentID fixes the document id, not the instance id; None leaves existing random behavior. Empty executable-only content preserves the existing empty-output path.
- Resolve pages from caller then document then all pages; resolve standards from caller then document then PdfStandards default. Resolve pretty from explicit caller then document. StreamMode (verbose production default versus explicit compact), serialization, tagging, features and DocumentID are independent axes.
- Without page selection, explicit tagged decisions win; documentary Auto enables tagging. With pages, effective Explicit(true) fails with `cannot enable tagged PDF and export a page range`; Auto disables tagging and emits `exporting a page range disables PDF tagging` with hints `the resulting PDF will be inaccessible` and ``set `pdf(tagged: false)` to silence this warning``; explicit false stays silent.
- CLI-only rejections and the separate CLI pages warning belong to `00_nucleo/prompts/wiring/compile.md`, not this module. Do not collapse caller absence early or duplicate those messages.
- After pages/tagging resolution, an accessible archival profile (A1a/A2a/A3a) or Ua1 with Disabled tags fails with ``cannot set `pdf(tagged: false)` when exporting a {name} document``. Add `using `pdf(pages: ...)` implies `pdf(tagged: false)` only when pages caused disabling and false was not explicitly selected.
- Pass resolved pages/tags/standards/serialization unchanged to all font-aware exporter branches. Standard-name/version resolution is not proof of full PDF/A or PDF/UA structural conformity; do not add a conformity engine or version-specific internal rewrites.
- Before export validate attachments in final order: duplicate derived virtual paths fail with `attempted to attach file {path} twice`. Direct infallible exporters require a validated document; they do not replace this language error channel.
- Only under Ua1, absent document title fails with `PDF/UA-1 error: missing document title` and hint ``set the title with `set document(title: [...])```. This checks None versus Some, not a new empty-title policy or combined-validator prefix.
- Only under Ua1, real images without alt outside `pdf.artifact` fail with `PDF/UA-1 error: missing alt text` and hint `make sure your images and equations have alt text`. Some empty text is still supplied; keep available origin spans and existing detached-span limitation.
- Resolve/instantiate ordered complete font keys through the resource owner, passing the World without requesting its catalog in advance. Dispatch no resolved fonts to fallback export, one to single-font export and many to multifont export without dropping explicit variations. Preserve ordinary and oracle behavior, not total PDF-byte identity as a language criterion.

The resource owner's private matching-axis predicate governs the early instancer dependency check. Unknown-axis-only requests must not produce a missing-tool diagnostic; retain dependency errors for actual font axes and preserve export dispatch/options.

Quando a dependência do instanciador faltar para um eixo real, delegar o gate
e diagnóstico ao helper privado de `infra/pipeline/resources.md`, preservando
mensagem, span e precedência. A medição causal está no L0 daquele owner; esta
correção interna não altera o despacho de zero/uma/várias fontes.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Validação PDF partilhada com bundle — 2026-09-30

### Medição anterior à decisão

`00_nucleo/diagnosticos/auditoria-composicao-construcao-2026-09-29/probe.json` mede o mesmo corpo com `format.pdf(standard: "ua-1")`: PDF isolado falha com `PDF/UA-1 error: missing document title`, mas PDF dentro de bundle compila e declara `pdfuaid:part=1` sem título. `03_infra/src/pipeline/pdf.rs:404-426` verifica título, alt e duplicação de anexos antes da exportação; `03_infra/src/pipeline/bundle.rs:463-513` não executa essas verificações. A inferência é que o target PDF perdeu invariantes ao ser composto no bundle; seria refutada se o mesmo body não passasse pelas duas rotas com o mesmo standard efetivo. A sonda documental confirmou o standard efetivo no XMP; não se presume conformidade PDF/UA completa nem paridade byte a byte.

### Contrato do owner

Concentrar num helper privado ao pipeline, pertencente a este owner PDF, as validações pré-exportação já existentes de documento paginado: primeiro título UA-1, depois alt UA-1, depois caminho de anexo duplicado na ordem original. Preservar exatamente mensagens, hints, spans, condições e precedência. PDF isolado e bundle chamam esse mesmo helper depois de resolver standards e antes de escolher fontes ou exportar. Não criar API pública, novo modo, fase ou regra de conformidade; a delegação de bundle não transfere ownership do contrato PDF.

### Aceitação

Sob UA-1, ausência de título falha em ambas as rotas com o mesmo diagnóstico; título fornecido continua a compilar. Alt ausente em imagem real e anexo duplicado usam os diagnósticos vigentes quando a rota os alcança. Fora de UA-1, o título continua opcional. A correção é de paridade interna do target PDF e de invariantes já especificadas, em fluxo contínuo pela ADR-0127 (sem assinatura pública ou mudança de fase), com teste RED→GREEN e revalidação.
