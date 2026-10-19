# L0 — infra/export/p1431-line-cap-tests
Hash do Código: 25e2b5ff

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/p1431_line_cap_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/p1431_line_cap_tests.rs:29` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Independent raster/PDF line-cap observations

Own only the existing cfg(test) consumer and its fixtures/assertions. Construct
Butt/Round/Square through math.mat, isolate the transported line and observe actual
exporters. Keep the frozen effective automatic thickness 0.05*11pt=0.55pt.
Raster endpoints must distinguish Butt < Round < Square; automatic augment matches
the explicit Square witness. Preserve the existing deterministic local byte guards
for that witness and the legacy Butt line, not a universal byte-parity claim.

The PDF parser tokenizes operators, verifies balanced q/Q, selects the internal
scope containing w/m/l/S, and attributes its sole J before the line path in that
same scope. Round requires 1 J, Square 2 J, and Butt no redundant 0 J at top/local
drawing. An outer scope alone cannot attribute cap.
Missing/truncated/unattributable stream, pixel or line identity is exactly
UNKNOWN_EXPORT_OBSERVATION and never receives acceptance credit.

## Ownership boundaries

`00_nucleo/prompts/infra/export/render.md` owns raster Line cap mapping;
`00_nucleo/prompts/infra/export/stream/shape.md` owns PDF cap primitives;
`00_nucleo/prompts/infra/export/stream/draw.md` owns their drawing scope.
This suite does not claim equivalent SVG or Shape-stroke cap support.

## Correction — 2026-09-28

### Measurement before decision

The five `place_regression_*` refutators in `03_infra/src/integration_tests.rs:5690` reproduce loss of configured empty-page dimensions, image clipping, text top edge, and unwanted catalog acquisition. `01_core/src/compiler/layout/helpers.rs:185` discards clip_rect; `sub_frame.rs:143` and `placement.rs:327` acquire unconditional ascenders. `mod.rs:1978` skips the initial empty page. The raster test at `03_infra/src/export/p1431_line_cap_tests.rs:204` uses an A4 viewport at 16 pixels per point for a single isolated line. Provenance and RED logs are recorded in `00_nucleo/diagnosticos/correcao-falhas-2026-09-28/`. These observations justify internal parity and fixture corrections, not a new public contract or a general speedup claim.

### Owner responsibility

The raster witness crops only its isolated line viewport to endpoint bounds padded for the largest cap and antialiasing. Translate both endpoints by a common integral-point offset, preserve 16 pixels per point, thickness, color and caps. Retain Butt < Round < Square and automatic == Square assertions. PDF witness pages and byte guards remain unchanged.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: 00_nucleo/diagnosticos/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: Atualizar somente construtor Value de stroke para From<Stroke>, mantendo prova de cap/offset/augment no export e expectativas vigentes.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.
