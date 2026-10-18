# L0 — stdlib/structural/heading
Hash do Código: 1184a705

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/heading.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/heading.rs:34` identifies the ownership boundary. Inspection is not new bilateral parity evidence or upstream intent. Historical evidence retains its original provenance in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own heading construction and presence transport, not numbering or rendering. Preserve four forms: positional level+body, body-only (level 1), named level+positional body, and named level/body. Initial Int is level with required second body; initial Content|Str is body; other initial types fail `heading(): primeiro argumento deve ser int (level) ou content/string (body), recebeu {type}`. Missing body fails `heading() exige body`. Named level overrides the body-only default, requires Int and range 1..6; preserve `heading(level:): espera int, recebeu {type}` and `heading(): level deve estar entre 1 e 6, recebeu {n}`.

Named numbering:Str|none defaults None; outlined:Bool|none defaults true; bookmarked:Bool|none defaults None (follows outlined). Keep outlined versus bookmarked separate (document outline versus PDF bookmarks). set_fields marks HEADING_SET_LEVEL for explicit positional Int or named level, and HEADING_SET_OUTLINED whenever outlined is named, including true. bookmarked Option retains its presence distinction. Emit heading_numbered_native when numbering exists, otherwise heading_native, with all fields/bits intact. Preserve type diagnostics and show-selector integration.

has/at/fields distinguish explicit defaults from omission; these bits are an introspection contract, not expendable mechanics. Context/World/file parameters remain pass-through; numbering here would violate phase ownership. No new helper or render logic.

## Acceptance

Preserve syntax, semantics, morphology, diagnostics/hints/spans, bindings, defaults and identities. No productive code or compatibility change is authorized by this sanitation. Preserve pinned shared claims and exclusive ownership under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

## A1 — callbacks de suplemento de Heading, 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c e working tree/UTC/hash
registrados em 00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz.
before-pdf-probe.json.gz: heading-supplement-panic termina em sucesso no Tekt
e em A1_HEADING no vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-supplement-valid aceita ambos, mas só o vanilla usa Custom na referência.
Fonte upstream model/heading.rs:249-289 sintetiza supplement por ocorrência e
model/reference.rs:393-415 aplica a função e display. Set.rs:286-320 ignora
supplement e limpa numbering; snapshot_fields heading.rs:55-68 fixa tradução.
Isto é callback/valor observável, não exigência de mecânica upstream.

### Decisão e responsabilidade exclusiva

Transportar named supplement válido por Styled custom heading.supplement,
sem novo campo de HeadingElem nem execução de Func. Validar auto/none/Content/
Str/Func; rejeitar outros tipos. Usar wrapper existente sem tornar Styled um
novo elemento. Presença/reflexão crua desse suplemento por Styled não é
ratificada por esta fatia; o snapshot realizado é definido por introspect/heading.

Correção de paridade em fluxo ADR-0127: sem nova API Rust, campo público ou
transferência de lógica existente entre eval/layout. O pós-walk com Engine já
é a fronteira de aplicação de callbacks. L0 primeiro, RED e controles bilaterais,
sem callbacks em encoder/walk/layout e sem alegar paridade de todos os campos.


## A1-H2 — suplemento antes das consultas, aprovado em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
final-freeze.json.gz e target-controls.json.gz no diagnóstico
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17 registram
UTC/diffstat/fontes/argv. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-raw-constructor-func passa no baseline/vanilla e falha no candidato:
Styled de transporte altera func. Query também vê wrapper causal temporário;
HTML ignora callback panic. Fonte walk.rs:930–945; pipeline/paged.rs:419–427 e
pipeline/html.rs:101–106 preparam a consulta pelo walk puro. São valores e erros
de linguagem, não um requisito de copiar algoritmo/estrutura do vanilla.
Gate A1-H2 confirmado pelo dono nesta sessão: revisão
00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md,
SHA-256 66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c.

### Responsabilidade exclusiva

Construir Heading base uma vez e transportar numbering/supplement explicitamente
fornecidos em um único Styles, com marcador interno heading.constructor=true.
Não confundir ausência com numbering:none ou supplement:none/auto: transportar
presença usando o Styles existente. Não executar callback. Numeration usa os
mesmos heading.numbering/heading.numbering.pattern existentes. O marcador é
interno e só é colocado pelo construtor; wrappers Styled externos não o herdam.
Validar supplement antes de retornar. Não mudar HeadingElem ou qualquer campo
público. A igualdade semântica do suplemento continua dívida separada A3.

### Aceitação e limites

H1–H8 da revisão ratificada, RED bilateral antes do código. Nenhum contrato
público além da fachada Heading está autorizado por H2; A1-S2 cobre separadamente
cinco helpers State. Sem novos campos públicos. Styled ordinário e demais
callbacks conservam suas fronteiras. Workspace/doctests/build/Clippy/Rustdoc,
fmt, V1/V5/V15/V26/reversa e V16 sem aumento; 12GiB/High8GiB/swap0/jobs1,
comandos pesados sequenciais. Ganho de RAM exige medição, não intenção.
