# L0 — Layout: Contadores e Numeração
Hash do Código: 15c691a5

## Módulo
`01_core/src/rules/layout/counters.rs`

## Propósito
Encapsula os braços do Layouter que alteram ou exibem o estado de
contadores: `SetHeadingNumbering`, `CounterUpdate`, `CounterDisplay`.
Funções chamadas por `layout.rs` (orquestrador).

## Regras de negócio
- `SetHeadingNumbering { active }` → muta `self.counter.numbering_active`.
- `CounterUpdate { key, action }` → delega em `step_flat`/`update_flat`/
  `step_hierarchical`.
- `CounterDisplay { kind }` → lê o estado actual e gera `Content::text`.
- Nenhuma destas funções gera geometria de página directamente.

## Critérios de verificação
- `CounterUpdate(Update(5))` → `counter.get_flat("equation") == 5`.
- `CounterDisplay("heading")` → texto contém o número formatado.
