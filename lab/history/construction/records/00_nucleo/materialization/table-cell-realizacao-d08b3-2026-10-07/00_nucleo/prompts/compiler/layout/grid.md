# L0 — grid
Hash do Código: 6e4460f9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/grid.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/grid.rs:125`, `01_core/src/compiler/layout/grid.rs:154`, `01_core/src/compiler/layout/grid.rs:536`, `01_core/src/compiler/layout/grid.rs:30`, `01_core/src/compiler/layout/grid.rs:31`, `01_core/src/compiler/layout/grid.rs:32`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own Grid dispatch and the shared visual grid/table engine. Delegate cell-row emission and pagination to `00_nucleo/prompts/compiler/layout/grid/cell_emission.md`. Delegate column/row track sizing and its pagination point to `00_nucleo/prompts/compiler/layout/grid/tracks.md`. Delegate header/footer normalization and collision detection to `00_nucleo/prompts/compiler/layout/grid/row_groups.md`, and visual h/v rules to `00_nucleo/prompts/compiler/layout/grid/rules.md`. Delegate row-border coalescing and h/v segment emission to `00_nucleo/prompts/compiler/layout/grid/borders.md`, retaining their invocation order and geometry. Preserve track resolution (fixed/auto/fractional), gutters, per-cell overrides and span measurement, row-group extraction/repetition, h/v rules, fill/stroke and row-border coalescing. Obtain occupied placement from `00_nucleo/prompts/compiler/layout/grid_placement.md`; no second placement algorithm is defined in the hub or cell wrappers.

Measure cell content with injected metrics/styles in the existing finite column widths, apply per-axis inherited alignment and inset, and lay out each cell once in its absolute cell origin/region. Recompose by incremental deltas, keeping parent bbox, cell alignment/context, decorations and orphan fixup paths consistent and restored. Preserve breakable-row splitting, repeated header/footer visual output and deferred cell tails within the source's bounded support; `00_nucleo/prompts/compiler/layout/slicing.md` performs partition/rebase and `00_nucleo/prompts/compiler/layout/cell_tail_commit.md` later emits tails. Table semantic metadata belongs to `00_nucleo/prompts/compiler/layout/table.md`; do not derive it from visual strokes/fills. Unsupported multipage/span/container combinations remain limited rather than silently promoted to parity.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.


## Produto medido consumível — 2026-10-05

Evidência em tracks.rs:175 e cell_body.rs:155 e diagnóstico `00_nucleo/diagnosticos/reuso-sem-transformacao-2026-10-05.md` precede a decisão. Coordenar o produto local devolvido por tracks e encaminhá-lo à emissão; registrar novo child measurement_cache. Não armazenar cache no Layouter nem mover placement antes de medição. Delegar invariantes de cache ao owner canônico `00_nucleo/prompts/compiler/layout/grid/measurement_cache.md`. Mudança privada de mecânica, na mesma fase e fluxo contínuo ADR-0127.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

Manter engine física única; adicionar entrada privada para plano de Table com posições já conhecidas. Este caminho usa plano antes de measurement; Grid legacy conserva placement depois de measurement. Isso supersede apenas para Table preparada a restrição histórica de ordem. Delegar toda ocupação ao owner, sem novo algoritmo no hub. Props conhecidas e roles seguem planos, tracks/fontes/paginação permanecem físicos; não executar Engine/Func nem uma segunda medição para obter show.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.
