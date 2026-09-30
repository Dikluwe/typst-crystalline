# L0 — footnote
Hash do Código: a8cd810a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/footnote.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/footnote.rs:31`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Read the current causal footnote counter from the injected introspector, with the existing fallback of one. Format the supplied pattern or default `1`, falling back to the decimal value, and dispatch a superscript marker without square brackets. Queue the numbered body for deferred emission. Footnote-body drain and region progress remain in `00_nucleo/prompts/compiler/layout/cursor.md`; terminal coordination remains in `00_nucleo/prompts/compiler/layout.md`.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Admissão da chamada com o primeiro corpo — proposta pendente de materialização

### Medição anterior à decisão

`00_nucleo/diagnosticos/footnote-composicao-2026-09-30/results.json` fixa o
vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, o binário cristalino,
HEAD e árvore não commitada da sonda. Em `defer.typ`, o vanilla coloca a linha
`Alpha¹ omega.` e a nota na página seguinte; o cristalino deixa a chamada na
página anterior e adia só o corpo. `footnote.rs:47-51` emite a chamada e enfileira
o corpo, enquanto `footnote_flush.rs:128-169` decide o cabimento tarde demais
para mover a linha de origem. A causalidade é inferida desses pontos e seria
refutada por um percurso que já movesse a linha completa antes do flush.

### Contrato proposto, ainda não vigente

O protótipo em `footnote.rs` foi refutado por `defer.typ` fonte→PDF: o
parágrafo preparado já confirmou a linha e a fila local não consegue movê-la.
A implementação deve acontecer em passo futuro coordenado com os owners de
linha/região; este texto não autoriza alterar o consumer atual como se a
correção estivesse concluída.

Antes de emitir a chamada, quando a nota inteira cabe numa região nova mas não
na região corrente junto à linha portadora, mover a linha em curso inteira para
a próxima região. Conservar itens, texto anterior/posterior, avanço horizontal,
estilos e número causal; não mover só o sobrescrito. A nota permanece pendente
para o flush normal na região da chamada. Não tentar essa migração se não houver
progresso regional ou se a nota exceder a altura útil completa; a continuação
de notas oversized pertence a outra materialização. Esta é correção privada de
paridade em fluxo contínuo ADR-0127, sem campo/assinatura pública, default,
fase ou compatibilidade novos.

Aceitação: `defer` coloca chamada e corpo na mesma página como o vanilla;
`fit` permanece numa página; a linha portadora mantém o texto em ordem; nota
que excede a região inteira não entra em ciclo de páginas. A prova é por texto,
páginas e morfologia, não por bytes de PDF.
