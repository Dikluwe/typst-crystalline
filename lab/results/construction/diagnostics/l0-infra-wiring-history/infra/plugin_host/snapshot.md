# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/plugin_host/snapshot.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `9d987b3363e10fa2084e471dd185fa8a4989d060ad016be4dd6bb6fb5360c4ac`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: 771c2423
prompt: infra/plugin_host/snapshot
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0111, ADR-0127, ADR-0129, ADR-0130
---

# Prompt L0 — infra/plugin_host/snapshot

## Medição anterior à decisão

Consumer anterior `03_infra/src/plugin_host.rs:47-52, 305-344`; HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, observado em `2026-09-26T09:11:38-03:00`. Proveniência exata em `00_nucleo/diagnosticos/plugin-host-atomization.md`; contrato anterior arquivado em `00_nucleo/diagnosticos/plugin-host-l0-before-atomization.md`. Referências plugin.rs do texto histórico abaixo correspondem às medições antigas de P696/P819, não a nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo: `03_infra/src/plugin_host/snapshot.rs`. Possuir Snapshot (mem_pages u64 e mem_data Vec<u8>), instance_memory, snapshot_memory e restore_memory. Obter memory já validada por load. Capturar páginas e cópia dos bytes; restaurar crescendo apenas se necessário e copiar bytes no prefixo, sem reduzir memória. Preservar expects de export e grow. Globals WASM não são capturados. Não serializar em disco nem alterar limites, custos ou lifetime; execução coordena quando capturar/restaurar.

Atomização reversível dentro de L3 (ADR-0127, fluxo contínuo): nenhuma alteração de API pública, comportamento por defeito, fase de pipeline ou compatibilidade. Fronteiras Rust são mecânica (ADR-0107); bytes, efeitos de transition e mensagens permanecem os observáveis. Não atualizar runtime/features ou o contrato L1 nesta mudança.
