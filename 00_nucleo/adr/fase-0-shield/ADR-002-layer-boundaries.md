# ADR-002: Layer Boundaries

**Status**: Accepted  
**Date**: 2026-01-18  
**Context**: Crystalline Architecture

## Decision

Definir fronteiras rígidas entre camadas:

```
00_nucleo/  → Documentação, specs, ADRs (não compila)
01_core/    → Lógica pura, sem I/O, verificável
02_shell/   → Adaptadores para mundo externo (CLI, IDE)
03_infra/   → Exportadores, ferramentas auxiliares
04_wiring/  → Composição, orquestração, facade
```

## Dependency Rules

```
01_core  ──────────────────┐
   ↓                       ↓
02_shell  ───────────→  04_wiring
   ↓                       ↑
03_infra  ─────────────────┘
```

### Proibições

| De | Para | Permitido? |
|----|------|------------|
| 01_core | 02_shell | ❌ Proibido |
| 01_core | 03_infra | ❌ Proibido |
| 02_shell | 01_core | ✅ Permitido |
| 04_wiring | * | ✅ Pode importar todos |

## Consequences

- Previne dependências cíclicas
- Core permanece puro e verificável
- Wiring é o único ponto de acoplamento
