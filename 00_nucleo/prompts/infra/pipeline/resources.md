# L0 — infra/pipeline/resources
Hash do Código: 939088c3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/resources.rs`

## Current evidence

- `03_infra/src/pipeline/resources.rs:33` — `resolve_and_instantiate_fonts`.
- `03_infra/src/pipeline/resources.rs:93` — `collect_fonts_from_doc`.
- `03_infra/src/pipeline/resources.rs:123` — `collect_fonts_in_items`.
- `03_infra/src/pipeline/resources.rs:172` — `resolve_fonts`.
- `03_infra/src/pipeline/resources.rs:195` — `first_font_from_doc`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/resources.rs`.

## Owned contract

- Own ordered document font collection, fallback resolution, complete FontKey identity and variable-font instantiation. Keep first-occurrence deduplication and declared-family order.
- Traverse background, items, foreground and Group/Link/Semantic transparently. Preserve semantic kind/placement/alt; never flatten the wrapper or turn alt into visible text.
- Include Text, TextShaped and math Glyph resources. Resolve a Glyph's actual font via FallbackFontMetrics::resolve_font_combo, not just body style.font, so a companion MATH glyph cannot be omitted from export selection.
- Keys include FontList, FontVariant and explicit FontVariations. Derived axes plus explicit per-tag overrides determine variable-instancing need and separate resources, even if only an explicit axis differs.
- Require the Python/fontTools availability probe only when a merged request contains an axis actually present in the resolved variable face. Requests containing only unknown axes follow the instancing owner's unchanged-bytes contract; preserve the missing-tool error for matching axes. A private local predicate owns this resource guard without exposing a new API.
- An empty collected font set resolves to an empty resource set without requesting World::book or World::font. All PDF, bundle PDF and raster/SVG callers use this same private resolution boundary.
- A family wins only after selection and source loading both succeed; stale/missing sources continue to later families. Preserve unresolved-entry filtering and variable-instancing diagnostics. Keep test-only first-font helpers internal; production selection uses the complete resolved font set.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/pipeline/resources.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Recolher a fonte ativa de TextSeparator como recurso textual; conservar a mesma deduplicação de fonte/variante/variações.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Diagnóstico comum de instanciação PDF/bundle — 2026-09-30

### Medição anterior à decisão

Com a mesma fonte variável Ubuntu Sans e `TYPST_CRYSTALLINE_PYTHON` apontando
para executável inexistente, `03_infra/src/pipeline/pdf.rs:444-467` produz
`fonte variável 'ubuntu sans' requer instanciação, mas Python/fontTools não está disponível`;
`03_infra/src/pipeline/bundle.rs:501-518` produz a frase inglesa correspondente.
`03_infra/src/pipeline/resources.rs:39-63` já possui a primeira mensagem para
SVG/PNG. Sem fonte ou com fontes estáticas, ambos exportam; com eixo existente,
ambos rejeitam. A mensagem de erro é observável da linguagem (ADR-0107/0108),
enquanto o número de ramos de exportação e os bytes PDF são mecânica. A
inferência de deriva por duplicação seria refutada se as três rotas já
delegassem o mesmo construtor de diagnóstico; as fontes citadas mostram que não.

### Contrato do owner

Concentrar num helper privado a decisão de dependência e o diagnóstico para
fontes resolvidas. Só consultar a disponibilidade do instanciador se algum
pedido fundido corresponder a eixo real da fonte. Preservar a primeira fonte
correspondente, mensagem portuguesa, span detached e ausência de hint vigentes.
PDF isolado, PDF de bundle e SVG/PNG chamam esse helper; não criar API pública,
mudar seleção de fontes ou alterar o despacho de exportação. O teste pode
injetar apenas o resultado da sondagem para cobrir indisponibilidade de modo
determinístico.

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
