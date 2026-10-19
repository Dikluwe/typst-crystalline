# L0 — table
Hash do Código: a851d61c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/table.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/table.rs:27`, `01_core/src/compiler/layout/table.rs:127`, `01_core/src/compiler/layout/table.rs:155`, `01_core/src/compiler/layout/table.rs:174`, `01_core/src/compiler/layout/table.rs:229`, `01_core/src/compiler/layout/table.rs:243`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Render an optional caption above the table, adding a prefix only when table.numbering is a String. Read the causal Table counter from the injected introspector with current fallback one, register its page for LoT, format or use decimal fallback, and retain the existing English `Table number: ` extension prefix. Caption remains outside the table carrier. This direct table-numbering extension is not a Figure and does not acquire figure selectors/show behavior by documentation.

Project visual children/header/footer and delegate all tracks, lines, align/inset, stroke/fill and visual row-group/pagination behavior to `00_nucleo/prompts/compiler/layout/grid.md`. Build the current parallel, visually empty Table semantic carriers after that layout, preserving summary and explicit cell kind/level/scope/spans in their current representation. Header Auto cells receive the current level-one Column classification; explicit Data remains Data. Uniform automatic headers produce grouped head/body carriers, mixed explicit Data retains direct rows. Place header logical rows only on the first affected page; retain the source's current sequential logical-row projection and even partition by affected pages.

The source does not construct logical footer rows, derive arbitrary header levels from TableHeader or align logical partition to measured visual breaks. Full Headers applicability relations and PDF/UA/AT/reflow proof remain unresolved, not promises of this sanitation. The older enclosing-frame proposal must not hide the actual parallel-empty-carrier behavior. Exporters own MCIDs and relation serialization; this owner creates no new layout pass.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

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

Table com plano completo consome cells/coords/roles do plano e delega a engine grid comum; não despejar wrappers, recalcular ocupação, executar receitas ou indexar corpos crus ocultados. Sem plano, conservar fallback histórico para fixtures/callers não preparados. Semantic carriers usam coords/spans/kind do plano, preservando modelo paralelo vazio e limites PDF/UA existentes; não duplicar header em body. Caption continua no owner atual.

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
