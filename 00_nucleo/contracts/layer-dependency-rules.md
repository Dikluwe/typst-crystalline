# Layer Dependency Rules

**Version**: 1.0  
**Context**: Crystalline Architecture

## Layer Hierarchy

```
┌─────────────────────────────────────────┐
│ 00_nucleo   Documentation (no code)     │
├─────────────────────────────────────────┤
│ 01_core     Pure logic, no I/O          │
├─────────────────────────────────────────┤
│ 02_shell    User-facing adapters        │
├─────────────────────────────────────────┤
│ 03_infra    Exporters, tools            │
├─────────────────────────────────────────┤
│ 04_wiring   Composition, orchestration  │
└─────────────────────────────────────────┘
```

## Dependency Matrix

| From ↓ / To → | 01_core | 02_shell | 03_infra | 04_wiring |
|---------------|---------|----------|----------|-----------|
| **01_core**   | ✅      | ❌       | ❌       | ❌        |
| **02_shell**  | ✅      | ✅       | ❌       | ❌        |
| **03_infra**  | ✅      | ❌       | ✅       | ❌        |
| **04_wiring** | ✅      | ✅       | ✅       | ✅        |

## Rules

### Rule 1: Core Isolation
`01_core` MUST NOT depend on any other layer.

### Rule 2: Shell Independence
`02_shell` MUST NOT depend on `03_infra` or `04_wiring`.

### Rule 3: Wiring as Facade
`04_wiring` is the ONLY layer that can import from all others.

### Rule 4: No Cycles
Cyclic dependencies are FORBIDDEN.

## Verification

Run before every PR:
```bash
cargo check --workspace
```

If cyclic dependency detected:
1. Identify the cycle
2. Move shared code to `01_core`
3. Use dependency injection if needed
