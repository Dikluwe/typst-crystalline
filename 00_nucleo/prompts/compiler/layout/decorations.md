# L0 — decorations
Hash do Código: dfef5f8b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/decorations.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/decorations.rs:21`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Dispatch the body under a temporary decoration collector and restore the outer collector. Preserve wrap segments and the final still-pending segment. Resolve offset and extent against the active font size; defaults are underline `0.10em`, strike `-0.25em`, overline `-0.80em`, thickness `max(0.05em, 0.4pt)`. Explicit stroke color takes precedence over active fill. Emit the existing butt-capped lines with matching segment geometry. Do not create a parallel line-commit or page algorithm.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

### Decorações: medição complementar de origem (2026-10-18)

Os12 pares SVG em bibliography-hanging-2026-10-18/deco-before.json.gz,
com a leitura corrigida de primitivas line/path em deco-inspection.json,
usam o mesmo binário/freeze da suíte inicial9026 e o oracle pinado bd5ebec.
Em overline h10, a segunda linha tem texto em x18 mas traço em x8;
vanilla começa o traço em x18. h0 já tem diferenças de y/espessura/cauda,
que não serão encobertas nem fechadas por este reparo horizontal.
Fonte local: decorations.rs:43–45,76–78 usa cursor/line_start_x globais;
line_commit.rs:615–619 e sub_frame.rs:350–354 capturam início global.
Isso é geometria da linguagem, não obrigação de copiar buffers vanilla.
Refutação: origem duplicada, consumo precoce da compensação, mudança de
baseline, fim, espessura, texto ou região fora do delta horizontal prescrito.
Correção privada de paridade ADR0127 em fluxo contínuo, sem fase/API nova.

Responsabilidade deste owner: Capturar start_x_initial pelo preview cursor_start privado. O último segmento após wraps usa a borda esquerda efetiva de paragraph_indent, não line_start_x global. Não antecipar baseline/admissão, alterar offsets/espessura/extent ou reexecutar body; coletores aninhados e final sem flush continuam preservados. A aceitação deste refinamento horizontal cobre LTR; a geometria completa de decorações e RTL permanece a medir, sem alegação universal.

### Preview sem material

O preview negativo pode ser menor que o cursor real antes de qualquer
admissão; portanto end > preview sozinho não prova um segmento. Acrescentar
teste RED de decorações vazias após quebra com h negativo, incluindo body
vazio, metadata e Par vazio. Expor apenas uma consulta privada read-only de
applied no owner paragraph_indent. No owner decorations, quando o preview
difere do cursor real inicial e não houve segmento flushado, o segmento
final exige origem realmente aplicada. Não iniciar escopo/admissão para
satisfazer a decoração e não criar tinta/altura de um body vazio.
