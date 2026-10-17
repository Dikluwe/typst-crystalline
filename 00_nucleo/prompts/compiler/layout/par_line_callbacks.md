# L0 — par line callbacks
Hash do Código: 0074fefc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/par_line_callbacks.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/par_line_callbacks.rs:18`, `01_core/src/compiler/layout/par_line_callbacks.rs:27`, `01_core/src/compiler/layout/par_line_callbacks.rs:34`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the public request/resolution types, sealed par.line callback store, attempt transcript and observational matching. Match by function, line number and collapsed style-chain observation; consume each matching resolution at most once. Missing resolution records the request and remains Pending rather than fabricating Content. Preserve existing reexports and signatures through the hub, without introducing I/O, Engine or Func execution.

Own the final region-local maximum-number-width fixup over the existing hub buffer, keeping item indices and left/right horizontal alignment consistent. Physical baseline/page/counter selection belongs to `00_nucleo/prompts/compiler/layout/line_commit.md`; segmented candidate consolidation belongs to the column owner. Store/transcript lifetime is coordinated by `00_nucleo/prompts/compiler/layout.md`; this module does not decide rollback, regional progress or the document fixpoint.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.


## Tamanho dos enums — decisão medida 2026-10-16

### Medição anterior à decisão

00_nucleo/diagnosticos/clippy-enums-2026-10-16/before.json.gz conserva fontes/L0, hashes globais, HEAD, working tree não commitado, UTC e diffstat; callers.txt registra usos no workspace. probe-details.json e measurements.json guardam rustc 1.92/x86_64, comandos/binário/rlib e tamanhos/alocações de candidatos isolados antes de código. O enum privado é espelhado fielmente da fonte; os tipos públicos são medidos diretamente do rlib. O observador unsafe existe somente no executável diagnóstico, nunca na camada produtiva. É medição de representação Rust, não igualdade de linguagem, intenção upstream ou prova de ganho global.

### Responsabilidade exclusiva e aceitação

Preservar LayoutPassOutcome::Pending(LayoutCallbackRequests) e Complete(PagedDocument), públicos, diretamente por valor. A busca dos usos produtivos mostra retorno e consumo por match no coordenador, não coleção de outcomes. A sonda Complete-only com documento vazio e black_box mede inline sem alocação contra uma por Complete na candidata Box: 20.000 extras por amostra. O enum é 440 bytes e a candidata 48; o documento de 440 continua existindo separadamente na caixa. Portanto reduzir o discriminated container não é reduzir a memória total do resultado completo. Esta fronteira de resultado único deve continuar direta, sem impor alocação/indireção nova ao caminho Complete. Usar somente #[expect(clippy::large_enum_variant, reason="E002: resultado único de layout por valor; Box adiciona alocação ao Complete, sem reduzir o documento")] no enum. Não supor que esta sonda é benchmark de layout real ou prova sobre todos os workloads.

Fluxo contínuo ADR-0127: esta etapa altera somente representação/constructor privados ao crate e metadados nominais; os contratos externos dos outcomes e entidades permanecem. L0 antes do código, resselo restrito. Gate de tamanho privado RED→GREEN e Clippy large_enum_variant deve confirmar três spans selecionados antes e resolver um com código e dois com expectativas satisfeitas. Não elevar limiar ou silenciar módulo/impl/crate; não estender expectativas a Tag/Transformation ou ao request privado. Preservar fixtures/esperados e testar os controles existentes de outline (incluindo oito casos P1405), contexto/replay, callbacks e pipeline. Validar workspace completo/build/Clippy/expectativas/Rustdoc/fmt/linhagem, sem avisos Tekt novos, sob 12 GiB, swap zero e um job. Tag/Transformation continuam medidos, sem alteração pública autorizada neste pacote.
