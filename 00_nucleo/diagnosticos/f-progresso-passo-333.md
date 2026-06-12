# Progresso P333 (retomável)

Decisão da fronteira E1 + spike-2 `#show` + L0 do F + plano de lotes + baseline estrutural.

| Parte | Estado | Entregável |
|-------|--------|-----------|
| 1 — decisão da fronteira (registro) | 🟡 em curso | ADR-0106 + DEBT 99.E + nota de fecho P332 + adendo dossiê |
| 2 — spike-2 `#show` (agente bg) | 🟡 lançado | `lab/spikes/f-extensao/e1/` + `f-spike2-show-passo-333.md` (S*) |
| 3 — L0 do F (3a/3b/3c) | ⬜ pendente (consome S* da Parte 2) | L0 element + style + contrato |
| 4 — plano de lotes | ⬜ pendente | sequência + custo preditor |
| 5 — baseline estrutural lente (agente bg) | 🟡 lançado | `baseline-estrutural-lente-passo-333.md` (R*) |

**Ordem**: 1 → (2 ‖ 5 em paralelo) → 3 (após S*) → 4. **Termina na Trava** (checkpoint; nenhum código de produto até o dono aprovar o L0).

## Notas
- Fronteira E1 decidida pelo dono (P332). `Content::Dynamic(Arc<dyn Element>)`, 65 nativos monomórficos.
- Conserto folded: a Parte 4 do `f-experimento-extensao-passo-332.md` tinha placeholder duplicado (defeito do commit P332) — removido na nota de fecho da Parte 1.
- Lente em `/home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm`.
