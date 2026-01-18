# ADR-001: Shield Migration Strategy

**Status**: Accepted  
**Date**: 2026-01-18  
**Context**: Crystalline Architecture Migration

## Decision

Adotar o padrão **Shield** para separar responsabilidades de diagnóstico entre camadas:

```
┌─────────────────────────────────────────────────────────┐
│ 01_core: VoidSignal (dados puros, sem mensagens)        │
├─────────────────────────────────────────────────────────┤
│ 02_shell: DiagnosticShield (narrativa para usuário)     │
├─────────────────────────────────────────────────────────┤
│ 04_wiring: DiagnosticBridge (composição via injeção)    │
└─────────────────────────────────────────────────────────┘
```

## Rationale

1. **Pureza do Core**: Sinais não contêm strings de mensagem, apenas dados estruturados
2. **Localização**: Shields diferentes podem gerar mensagens em idiomas diferentes
3. **Verificação Formal**: Sinais puros são verificáveis com Creusot
4. **Acessibilidade**: Shields especializados para diferentes contextos (CLI, voz, IDE)

## Consequences

### Positivas
- Core 100% puro e testável
- Mensagens de erro centralizadas e consistentes
- Preparação para i18n e verificação formal

### Negativas
- Overhead de 3 arquivos por novo tipo de erro
- Necessidade de atualizar NarrativeVisitor para cada sinal

## Implementation

1. Sinais em `01_core/shared/src/diagnostics.rs`
2. Shield em `02_shell/interaction/src/diagnostics_shield.rs`
3. Bridge em `04_wiring/fusion/src/diagnostics_bridge.rs`

## Migration Order

1. `primitives/diag.rs` (P0)
2. `semantics/eval.rs` (P0)
3. `foundations/*` (P1)
4. Demais módulos (P2-P3)
