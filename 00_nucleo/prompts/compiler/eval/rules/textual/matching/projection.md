# L0 — compiler/eval/rules/textual/matching/projection
Hash do Código: e29edf67

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:f225f3039d4663e9bfcb408d26cf1cd6e557f089013bc2d95090ce16c52080c6

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual/matching/projection.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the working tree captured in `lab/results/construction/diagnostics/atomizacao-textual-projecao-2026-09-30-before.json.gz`, `01_core/src/compiler/eval/rules/textual/matching.rs:186-223` contains unmatched-range projection and flattening of replacement sequences. The existing `p1494` filter passed 17 tests after the space/matching movement, before this projection movement. These helpers use content and ranges only; engine, selector and style state are absent.

## Decision and behavior

Own reconstruction of an unmatched textual range from its original content nodes. Preserve complete nodes, including explicit and inferred spaces and style, when wholly included. Slice only partially included Text into a new Text node. Flatten replacement sequences recursively in their original order. Keep UTF-8 byte ranges supplied by the matching owner and do not change selector matching or error behavior. The matching owner calls these helpers; it retains rule precedence, guard state, engine restoration and transform invocation. No public signature, phase, default or compatibility changes.

The existing unmatched-range projection test belongs to this module. No dynamic dispatch or additional I/O is introduced.

## Acceptance

The projection test, existing `p1494` filter and full core suite remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. No speedup is claimed without measurement. Header ownership is 1:1.

## R3-T1 — realização causal ratificada (2026-10-18)

### Medição anterior à decisão

lab/results/construction/diagnostics/textual-realization-2026-10-18/{before,scope,refuters}.json.gz
preserva HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não
commitado, UTC, status/diffstat, fontes/argv/hashes dos binários e pin vanilla
bd5ebec285505889e8ff1d4f68804ab147a699f3. before-sources.json.gz contém
fontes/L0 vigentes completos. Não há código candidato implementado nesta medição.
matching.rs:118–150 aplica regex reversa e depois literais em declaração;
:194–207 volta a expor produtos a passes subsequentes. textual.rs:102–123
captura snapshot completo. realize/mod.rs:177–217 visita receitas bottom-up.
apply.rs:620–648 e matching.rs:218–225 chamam funções sem capturar os filhos.
expression_dispatch.rs:125–129 já distingue insertion_rules do estado local.

Exemplos medidos: literais idênticos C/B; empate literal/regex R/L; overlap
X/YB; primeiro efeito LATER/FIRST; função encadeada inversa BBBB/BB;
produto separado AB/X; produto+vizinho X/AB; Strong ocultando filho dá
panic HIDDEN/Q; global capturado seguido de global nova dá L/R, mas local
capturado dá L/L. Entrada CJK local 水\n果 dá 水果/X. A fonte pinada
foundations/selector.rs:110–125 unifica literal escapado com regex;
typst-realize/src/lib.rs:1328–1405 escolhe o início e carrega revogação
no produto. Esses dados medem linguagem/efeitos; não ratificam mecânica Rust,
igualdade de buffers ou bytes PDF. É inferência causal que a separação de
escopos e a visita anterior aos filhos expliquem o conjunto; os controles de
acceptance.json, igualdade/fields/estilo/query e erros podem refutá-la.

### Decisão ratificada

Aplicar o núcleo textual-recipe-frontier pinado neste L0. A confirmação humana
do manifesto 47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302
ratificou esta ordem. Ele supersede neste escopo as
cláusulas históricas de execução provisória de show em ContentBlock e da
visita bottom-up de receitas pendentes, sem alterar a ordem de avaliação
de expressões comuns. Não reavaliar o programa, antecipar State/Counter,
assentar render em morfologia nem criar contrato Rust público.

### Responsabilidade exclusiva desta unidade

Projetar intervalos a partir dos nós originais e das fronteiras causais, preservando nó inteiro quando totalmente coberto. Partes de Text mantêm Source/origem e posições UTF-8 corretas; não atravessar produto concluído, estilo efetivo ou snapshot incompatível ao recompor run. Aplanar só scaffolding privado do produto quando sua fronteira continuar representada; não apagar authored Sequence, Empty explícito ou proveniência. Este owner continua puro: nenhuma chamada de Engine/callback/normalização.

### Aceitação desta revisão

RED→GREEN das famílias de acceptance.json, corpus R2-S/R3 e controles de uma avaliação. Revalidar PDF e HTML separadamente; build/workspace/doctests, Clippy/Rustdoc estritos, fmt, V1/V5/V15/V26 e linhagem reversa; diferencial SARIF sem novo erro/warning. Teto 12 GiB/High8 GiB/swap0/jobs1 e comandos pesados sequenciais. Não alegar ganho de RAM antes de ensaio pareado nem zerar o gate com supressão. Sem commit/push/release.
