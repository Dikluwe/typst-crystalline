# L0 — heading
Hash do Código: 9d5ffb10

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/block-margin-priority.toml sha256:bdf93363a4120d16985ebf48bed6261fd5981d3539910478ee5b6b6b41e726f7
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

## Prioridade causal das margens — 2026-10-18

### Medição anterior à decisão

00_nucleo/diagnosticos/bibliography-margins-2026-10-18/before.json.gz e
before-sources.json.gz preservam HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c,
working tree não commitado, diffstat/status/UTC/fontes/argv/binários. Os 78 pares
foram medidos contra vanilla pinado bd5ebec285505889e8ff1d4f68804ab147a699f3
(binário 9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9).
Com par.spacing 2em, Block below0 seguido de Block above auto posiciona BODY
em y37.064 no Tekt contra13.064; below3 resulta37.064 contra16.063996.
Heading→Block auto produz BODY40.004 contra25.004. Block auto→Heading produz
TITLE35.7128 contra33.312804. Na bibliografia2em, título35.7128 contra33.31281
e primeiro corpo71.9 contra54.5. Os controles explícito/explícito e auto/auto
coincidem, assim como auto→explícito. O desvio em duas direções refuta um offset
especial de bibliografia e confirma prioridade antes da magnitude.

Fontes locais: block.rs:170–176 só trata a entrada forte depois de margem
fraca; heading.rs:62 usa max incondicional. No vanilla pinado,
typst-layout/src/flow/collect.rs:253–260 atribui fraquezas diferentes a Auto
versus margem relativa explícita; distribute.rs:244–259 compara prioridade
antes da magnitude. typst-library/src/model/heading.rs:305–312 configura
above/below explicitamente. Posições são linguagem; copiar enums/algoritmo
Rust não é requisito e não se infere intenção da fonte. Refutação: regressão
em zero explícito, ordens inversas, margens iguais, wrappers, par.spacing,
continuação, fracionários ou terminal; medir e não compensar com constantes.

### Classificação

Correção interna de paridade em fluxo contínuo ADR-0127: sem campo, trait,
assinatura pública, default/flag novo ou transferência de fase. A regra de
colapso anterior por max fica supersedida somente na presença de prioridades
diferentes do protocolo existente. Não amplia o significado privado de
prev_margin_is_parbreak nem encerra hanging, vspace/sticky ou métricas mistas.

### Responsabilidade exclusiva

Ao entrar após fronteira de bloco ativa, colapsar above próprio como explícito
com a margem anterior através do helper privado do owner
00_nucleo/prompts/compiler/layout/block.md. Manter cálculo de prev_bottom,
escala/bold, top-edge, numeração, scope lexical, flush e below explícito
existentes. Margem anterior automática cede, mas margem explícita maior
continua a prevalecer. Nenhuma alteração para título no topo da região.

### Aceitação

Testes RED→GREEN no próprio consumer: título após Block automático não muda
quando par.spacing fraco supera seu above; below explícito maior ainda move
título pelo delta legítimo. Cobrir níveis1/2/3, título inicial e invariância
da restauração de estilo. Revalidar título→Block no corpus bilateral.

Repetir os 78 pares, controles nested/regionais, bibliografia sem/com título,
cores e corpus anterior; build, suíte workspace/doctests, Clippy estrito,
Rustdoc/fmt, V1/V5/V15/V26/reversa e diferencial Tekt. Limite12GiB/High8GiB,
swap0/jobs1/debug0/incremental0. Não apagar notas nem certificar todo o objetivo.
