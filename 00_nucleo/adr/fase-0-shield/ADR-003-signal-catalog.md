# ADR-003: Signal Catalog

**Status**: Draft  
**Date**: 2026-01-18  
**Context**: Crystalline Architecture - Diagnostics

## Decision

Manter um catálogo centralizado de todos os VoidSignals do sistema.

## Signal Categories

### Access & Binding
| Signal | Campos | Uso |
|--------|--------|-----|
| `AccessSignal` | target, property | Acesso negado a propriedade |
| `BindingNotFoundSignal` | name, scope | Variável não encontrada |
| `MutabilityViolationSignal` | name | Tentativa de mutar imutável |

### Types & Values
| Signal | Campos | Uso |
|--------|--------|-----|
| `TypeMismatchSignal` | expected, found | Tipos incompatíveis |
| `InvalidCastSignal` | from, to | Cast inválido |
| `MissingFieldSignal` | parent_type, field | Campo obrigatório ausente |

### Arithmetic
| Signal | Campos | Uso |
|--------|--------|-----|
| `DivisionByZeroSignal` | - | Divisão por zero |
| `OverflowSignal` | operation, value | Overflow aritmético |

### Function Calls
| Signal | Campos | Uso |
|--------|--------|-----|
| `ArgumentMismatchSignal` | expected, found | Nº errado de args |
| `MissingArgumentSignal` | name, func | Argumento obrigatório |
| `UnexpectedArgumentSignal` | name, func | Argumento desconhecido |

### Modules & Imports
| Signal | Campos | Uso |
|--------|--------|-----|
| `ModuleNotFoundSignal` | path | Módulo não encontrado |
| `CyclicImportSignal` | chain | Dependência circular |
| `PackageNotFoundSignal` | spec | Pacote não encontrado |

### Layout
| Signal | Campos | Uso |
|--------|--------|-----|
| `InfiniteLayoutSignal` | element | Layout infinito |
| `ConstraintViolationSignal` | constraint | Restrição violada |

## Implementation Status

- [x] AccessSignal
- [x] TypeMismatchSignal  
- [x] MissingFieldSignal
- [ ] Demais (a implementar conforme migração)
