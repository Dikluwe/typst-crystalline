# L0 — infra/export/font_subset
Hash do Código: f5807ac3

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/subset.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/subset.rs:19` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Subset adapter

Own only FontSubset and subset_font_with_mapping and remap_glyph_id
in the exclusive consumer. Preserve public signatures and FontSubset fields:
data contains subset font bytes; mapping is old GID to new GID.
L1 continues carrying original glyph IDs; it does not know this adapter.

Use subsetter::GlyphRemapper to include .notdef explicitly (0 → 0), every GID
from char_to_old_gid, and additional_gids such as ligatures. Call subsetter::subset
at face index zero on the already extracted individual-face bytes. Build mapping
from that remapper, not from a subset cmap; removal of cmap is intentional.
Return None on subsetter failure, retaining the existing test-only failure message
"subsetter::subset falhou: {:?}". Return valid TrueType or CID-keyed CFF subsets
when supported by the library. The mapping adapter retains default-coordinate behavior. A private sibling adapter accepts variation coordinates from the font builder and delegates instancing plus subsetting to the installed subsetter 0.2.6 API, without changing public signatures.

The unused internal legacy subset_font wrapper is retired by the 2026-10-16 maintenance below. remap_glyph_id returns the mapped ID
or zero for an absent key. Whether an empty mapping means full embedding is a
caller decision, not a different meaning of that helper.

## Ownership boundaries

- `00_nucleo/prompts/infra/export/builder/fonts.md` owns fallback to full
  embedding, variable instancing, font names, descriptors, widths/ToUnicode
  orchestration, CIDSet and the font compression threshold.
- `00_nucleo/prompts/infra/export/fonts.md` owns Unicode/cluster helpers.
- `00_nucleo/prompts/infra/export/stream/text.md` owns mapped text operators.
- `00_nucleo/prompts/infra/export/bitmap_glyphs.md` owns raster collection.

Do not duplicate those contracts or claim that bitmap-only fonts must be embedded.
Unsupported CBDT/CBLC subsetting retains None/fallback rather than invented support.
Shaping positions already supplied by L1 do not depend on preserving shaping tables
in the embedded subset.

## Verification

Check .notdef for empty input, char and additional-GID mappings, absent remap,
parseable TrueType/CFF fixtures and text extraction through the caller integration.
Exact subset size, object IDs or equality to an upstream subset algorithm are not
language-parity criteria.

## Coordenadas no subset — correção interna 2026-10-05

Medição anterior à decisão: `03_infra/src/export/subset.rs:56` chama subset sem coordenadas. Fonte local subsetter-0.2.6/src/lib.rs:108-126 distingue subset default de subset_with_variations; este último instancia e subseta conjuntamente (CFF2 converte para TrueType). `00_nucleo/diagnosticos/eixos-variaveis-export-2026-10-05/native.txt` mede H=689/734 para wght=250/900; a cadeia anterior congela H=697. Sonda e proveniência em before.json/native.rs.

Adapter privado pub(super) subset_font_with_variations aceita os mesmos inputs de mapping mais coordenadas ttf_parser Tag/f32; reutiliza remapper/mapping/error path do adapter existente e converte tags sem arredondar eixos. As APIs públicas mantêm subset default e o comportamento CFF2 anterior; o adapter privado usa subset_with_variations mesmo com coordenadas vazias para resolver a instância default, permitindo CFF2→TrueType. A decisão de eixos, fallback e descriptor pertence exclusivamente ao owner builder/fonts. Sem nova API pública, dependência, unsafe ou fase. Aceitação pelo teste PDF do owner e parseabilidade/font widths; RED→GREEN e revalidação.

Medição adicional antes da decisão: o controle variable-cff2-after.pdf ainda continha um recurso de espaço sem eixos explícitos com CFF2/fvar/HVAR intactos, enquanto os dois recursos com eixos eram TrueType estáticos. Poppler emitiu um aviso restante; por isso distinguir API pública legada sem pedido de coordenadas (subset) de preparação multifont privada com pedido explícito, inclusive default vazio (subset_with_variations). Testar a fonte Cantarell default pelo adapter privado e confirmar recurso PDF legível no controle completo.


## Simplificações locais de Clippy — 2026-10-15

### Medição anterior à decisão

`03_infra/src/export/subset.rs:85–87` associa o erro a um binding utilizado somente no diagnóstico sob cfg(test).
`00_nucleo/diagnosticos/avisos-clippy-tekt-2026-10-15/before.json.gz`
conserva fontes, owners, HEAD, working tree não commitado, UTC e diffstat;
selected-red.json identifica os diagnósticos do compilador antes da edição.
São expressões Rust equivalentes, sem nova medição de paridade upstream.

### Responsabilidade e aceitação

Nomear esse binding `_error` e conservar seu uso no mesmo diagnóstico de teste. A falha do subset continua a retornar None; conservar fallback e mensagem. A preservação histórica do wrapper de compatibilidade é supersedida pela decisão interna de 2026-10-16 abaixo.

Manutenção interna em fluxo contínuo ADR-0127: conservar contratos públicos,
defaults e fases. Não adicionar supressões nem testes que espelhem sintaxe.
O gate dos avisos selecionados deve passar a GREEN; validar a suíte existente,
build, Clippy all-targets, formato, documentação e linhagem com resselo restrito.


## Wrapper sem chamadores produtivos — 2026-10-16

### Medição anterior à decisão

03_infra/src/export/mod.rs:42 declara mod subset privado, sem pub use de subset_font. Busca de todas as camadas/benches encontra somente a definição em subset.rs:113 e duas chamadas no seu módulo de testes. Os adapters produtivos são subset_font_with_mapping e subset_font_with_variations. O snapshot 00_nucleo/diagnosticos/clippy-retencao-2026-10-16/before.json.gz conserva fontes/L0, hashes globais, HEAD, working tree não commitado, UTC e diffstat. A palavra pub da função não cria API externamente alcançável por um módulo privado sem reexportação. O contrato anterior chamava de compatibilidade um wrapper sem consumer produtivo; essa prescrição foi refutada pela busca dos usos. É correção de representação/manutenção Rust, sem nova paridade medida.

### Responsabilidade exclusiva e aceitação

Remover somente subset_font e sua documentação de wrapper legado. Migrar as duas fixtures para subset_font_with_mapping(&data, &BTreeMap::new(), &used), consumindo .data do FontSubset. Para esses inputs (GIDs 65/66 ou conjunto vazio), a ordem dos GIDs no remapper é preservada; .notdef continua explícito. Manter nomes e assertions dos testes e todos os demais adapters, assinaturas, retornos None, mapping/variações, mensagens e fallbacks. Não duplicar a conversão de GIDs em caracteres no teste, criar helper substituto sem caller ou expor/reexportar símbolo só para silenciar o aviso.

Fluxo contínuo ADR-0127 §2, item 4: alteração interna cuja reversão não quebra consumidores fora do próprio módulo. A decisão supersede exclusivamente as cláusulas antigas que exigiam manter esse wrapper; não autoriza alterar APIs externas ou os adapters vigentes. L0 primeiro, controles subset e gate dead_code RED→GREEN, resselo restrito e revalidação all-targets/workspace/build/doc/fmt/linhagem sob 12 GiB, swap zero e um job. Não adicionar allow/expect ou alterar esperados/tolerâncias.
