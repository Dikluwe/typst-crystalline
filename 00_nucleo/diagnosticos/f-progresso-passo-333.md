# Progresso P333 (retomável)

Decisão da fronteira E1 + spike-2 `#show` + L0 do F + plano de lotes + baseline estrutural.

| Parte | Estado | Entregável |
|-------|--------|-----------|
| 1 — decisão da fronteira (registro) | ✅ commit `15e384833` | ADR-0106 + DEBT 99.E + nota de fecho P332 + adendo dossiê |
| 2 — spike-2 `#show` (agente bg) | ✅ S1–S7 + Trava-Q1/Q2; 5 casos PASS | `lab/spikes/f-extensao/e1/` + `f-spike2-show-passo-333.md` |
| 3 — L0 do F (3a/3b/3c) | ✅ completo (S* preenchidos; `Value::Custom` fora) | `prompts/entities/f_fronteira_e1.md` |
| 4 — plano de lotes | ✅ F-1 (fronteira) → F-2 (canal Set*) → fila incremental | `f-plano-lotes-passo-333.md` |

**TODAS as 5 partes fechadas. Termina na Trava** (checkpoint; nenhum código de produto até o dono aprovar o L0).
| 5 — baseline estrutural lente (agente bg) | ✅ commit `2405e48aa` | `baseline-estrutural-lente-passo-333.md` (R0-R5) |

**Ordem**: 1 → (2 ‖ 5 em paralelo) → 3 (após S*) → 4. **Termina na Trava** (checkpoint; nenhum código de produto até o dono aprovar o L0).

## Notas
- Fronteira E1 decidida pelo dono (P332). `Content::Dynamic(Arc<dyn Element>)`, 65 nativos monomórficos.
- Conserto folded: a Parte 4 do `f-experimento-extensao-passo-332.md` tinha placeholder duplicado (defeito do commit P332) — removido na nota de fecho da Parte 1.
- Lente em `/home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm`.
