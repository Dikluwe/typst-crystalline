# L0 — TableHeaderElem
Hash do Código: 89eac155

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/row-groups.toml sha256:472e9e168c439ce5e1d17557fd8dbb55047e2f3c2f259eada2ff8b763ae27ade

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/table_header.rs`

## Contract


`TableHeaderElem` stores public body: Content and repeat: bool, with
Debug/Clone/PartialEq and the specified hash obligation. Plain text and
emptiness delegate to body. Both maps recurse into body, preserving repeat
and Content::TableHeader identity. Fields/kind/payload default to None:
non-locatable. Structural equality includes body and repeat. Construction
and repetition during layout belong to their own owners.


The trait API/defaults belong to
`00_nucleo/prompts/entities/elements/_comum.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`00_nucleo/diagnosticos/sanitization-entities-contracts-history.md`.


## RG1 — grupos Table ordenados e síntese estrutural — candidato 2026-10-17

### Medição anterior à decisão

Medição anterior: 00_nucleo/diagnosticos/table-row-groups-2026-10-17/before.json.gz, measurement.json.gz, supplement.json.gz, lines.json.gz, styles-phase.json.gz e svg.json.gz conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC/status/diffstat/fontes/binários/argv/cwd. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. table_grid.rs:126–150 funde grupos; row_groups.rs:22–35/70/355–366 conta entradas, table_cells.rs:1204 aplica receita antes do planner e :1259–1282 deriva prefixos. model/table.rs:307–316 sintetiza; typst-realize/src/lib.rs:367–385/558–568 sintetiza antes de receita; layout/grid/resolve.rs:1380–1457/1597–1650 preserva intervalos e cursores; typst-html/src/rules.rs:591–665 projeta papéis. TableHeader level/presença somem no Tekt, coordenadas de linha não são lidas em table_lines.rs:173–260, e supressão None em layout/grid/rules.rs:40–46 deixa traços. svg-analysis.json confere todos os strokes retos e mudanças pareadas (mesmo binário): top vermelho vanilla 82→41, Tekt 82→82; azul após None vanilla 41→0, Tekt 40→40. Estes são efeitos/células/roles/erros/geometria da língua; não obrigação de copiar vtables, estruturas Rust, bytes SVG/PDF ou inferir intenção upstream. O primeiro extrator SVG não reconhecia line/h/v; foi descartado como prova, conservado e substituído por extração completa registrada.

### Responsabilidade exclusiva

Adicionar pub level: std::num::NonZeroU32 e pub set_fields: u8. Constantes públicas de módulo TABLE_HEADER_SET_REPEAT=1 e TABLE_HEADER_SET_LEVEL=2. body e repeat mantêm tipos. O nível efetivo intrínseco é 1, repeat true; máscara distingue valor omitido de explícito. get_field só revela repeat/level assentes; children é sempre conhecido, array dos membros do body, inclusive vazio. Uma Sequence rica dentro de Cell continua uma única Cell. Maps preservam level, máscara, repeat e origem; não executam receitas nem assam defaults. Continua não-locatável. Campos/has/at/fields/func/repr/encoders respeitam o schema repeat,level,children. Este owner não normaliza ocupação.

### Gate, aceitação e incompletude

Contrato candidato RG1: não vigente antes da confirmação humana do manifesto em 00_nucleo/materialization/table-row-groups-2026-10-17/review.md. A aprovação precisa abranger contratos públicos, compatibilidade e fase (ADR-0127 §2.1/§2.3/§2.4). Após aprovação, esta seção supersede apenas cláusulas históricas conflitantes de grupos Table/linhas e da fronteira estrutural/receitas. Outras propostas não são ratificadas. Não implementar campo/API novo antes do gate. Instalar L0/kernel/pins primeiro, testes funcionais RED antes do código, sem tratar erro de compilação/setup como RED; GREEN mais R1–R20 e corpus bilateral, build, workspace completo, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, formato, V1/V5/V15/V26 e reversa, diferencial SARIF/replay/diff-check. V16 externos continuam abertos; não alterar severity, criar allow/expect ou esconder um complemento em if-let só para zerar lint. V19 tem justificativa local e corpo compartilhado quando a política é a mesma. Núcleos são compartilhamento normativo; cada L0 conserva exatamente um consumer. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais; sondas/mutações usam target Cargo isolado e nenhuma variante mutante vai para target principal. Sem commit/push/merge/release ou pin novo.
