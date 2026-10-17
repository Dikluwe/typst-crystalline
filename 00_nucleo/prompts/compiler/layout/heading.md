# L0 — heading
Hash do Código: e463dfde

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:f10f7afaabd1b97f3048736842df89d0ba48d4daa0ae3cd453c9be317f32ef91

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/heading.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/heading.rs:28`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Scale the active body size through the helper's level mapping and create a bold Heading TextStyle that retains the other outer fields, including inherited italic. Resolve above spacing against the body size (`1.8em` at level one, `1.44em` otherwise) and below (`0.75em`); collapse entry against the preceding block's physical bottom and pending margin, without empirical offsets. A top-of-region heading starts at margin plus its own top edge. Number only when the chain enables heading numbering: format the causal hierarchical counter with the supplied pattern and append one space, or retain the formatted-counter compatibility prefix with its period. Dispatch and close the body, record its causal baseline and pending below margin, clear stale paragraph/equation collapse state, and restore the outer style. A forced `italic: false` clause in older prose is superseded by the actual inherited-style consumer, not a new behavior change.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.


## Caminhos residuais internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/layout/heading.rs` nas linhas 114 contém os itens sem consumo/visibilidade sinalizados pelo compilador. A pesquisa de usos e os dispatchers ativos estão registrados antes da edição em `00_nucleo/diagnosticos/caminhos-residuais-ci-2026-10-13/before.json.gz`, com fontes, L0, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status e diffstat. Isto não é nova medição bilateral de paridade.

### Decisão e aceitação

Eliminar a aquisição de text_edges e o valor heading_descent descartados depois do body. O cálculo não alimenta baseline, espaçamento nem geometria; preservar as consultas necessárias de métricas e o flush. Não inferir número de chamadas como observável de linguagem nem ganho de desempenho.

Correção interna em fluxo contínuo ADR-0127, sem nova assinatura pública, default de produto ou fase. Preservar os resultados de linguagem e as expectativas dos testes. Revalidar todos os targets, suíte core, build, Clippy e linhagem; não suprimir warnings para substituir a correção.

## TS1 — bridge do tamanho resolvido de Heading

### Medição e refutação anterior à decisão

TS1 after-extended.json.gz/analysis.json refuta o fechamento inicial de S4:
Heading afim37.8 contra36.6. Inspeção deste consumer: heading_size Pt é
instalado apenas em layouter.style, enquanto chain conserva base não escalada.
Uma declaração relativa descendente lê chain em style_scope e resolve contra
essa base antiga. A hipótese é verificável por A16.8/B33.6 sobre12pt, e por
A36.6/B73.2 com show-set2em+3pt; testes devem falhar antes do bridge. Não se
usa aparência ou nome da fonte para inferir tamanho. Baseline completa e hash
deste owner/consumer preservados no before.json.gz TS1.

### Correção interna

Depois de resolver heading_size no layout existente, vincular esse Pt como
Length absoluto em um delta de chain local, antes de despachar o corpo; restaurar
a chain original juntamente com o estilo. Isso faz estilos relativos filhos
lerem o tamanho efetivamente ativo sem multiplicar o em declarado novamente.
Não mudar a escala, as demais propriedades, fontes, numeração ou fórmulas
de espaçamento deste owner. Nenhum campo público/assinatura/fase novo; é
o mesmo adapter TS1 para tamanho já resolvido, aplicado ao consumer adicional
exigido pela validação S3/S4. Preservar geometria dos controles absolutos.
