# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/shaper/cache.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `f03ae96cc5d2f6454016778084e230d85784abddea9392bdeddce7d3fc34f9a4`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: d1228c74
prompt: infra/shaper/cache
layer: L3
created: 2026-09-26
adr: ADR-0120, ADR-0127, ADR-0109, ADR-0129, ADR-0130
---

# Prompt L0 — infra/shaper/cache

## Medição anterior à decisão

Extração estrutural de `03_infra/src/shaper.rs:70-117, 131-183` no HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. Proveniência da auditoria registada em `00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md` (hash da fonte anterior, diff HEAD --stat, untracked e instante). A suíte baseline shaper passou sem alterações. Estas referências indicam a posição pré-extração, não linhas dos arquivos novos.

## Decisão e contrato da unidade

Possuir ShapeCache, CachedRun, FaceCache e CachedFace. Cache de runs local por documento, chave textual exata com texto, slot, direção, variações formatadas em quatro casas, tracking absoluto/em em quatro casas e nível ssty; valores clonáveis com glifos e largura i32 em unidades de fonte. Cache de faces por slot armazena sucessos e ausências; bytes owned de Font mantêm o empréstimo da face enquanto CachedFace existir. Preservar o parsing índice zero e o encapsulamento do unsafe existente. Sem cache global, nova política de eviction ou alteração de chave.

Separação mecânica em L3, sem alteração de contrato público, comportamento por defeito ou fase do pipeline (ADR-0127). A estrutura Rust é mecânica, não uma nova decisão de paridade linguística (ADR-0107/0108). Cada consumer tem este owner exclusivo; referências L0 são canônicas.
