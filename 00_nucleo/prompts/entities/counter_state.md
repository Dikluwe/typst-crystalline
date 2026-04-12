# Prompt — `entities/counter_state.rs`

## Camada: L1

## Propósito

Estado de contadores que viaja com o Layouter durante uma única passagem de layout.

Rastreia valores numéricos associados a tipos de nó (headings, figuras, equações).
O caso primário é a numeração automática de secções: `= Introdução` → `1. Introdução`.

Cristalino diverge do Typst original aqui: o original resolve contadores em duas passagens
com `comemo` (para suportar referências para a frente). Esta implementação usa uma única
passagem — suficiente para numeração sequencial sem referências para a frente.

DEBT-10: Resolver contadores em duas passagens com estado global quando o motor de
introspecção completo for implementado (Passos 60+).

## Estrutura

```rust
#[derive(Debug, Clone, Default)]
pub struct CounterState {
    heading: Vec<usize>,
    pub heading_numbering: bool,
}
```

## Comportamento de `step_heading(level: usize)`

Avança o contador para o nível indicado:

- `[]` + level 1 → `[1]`
- `[1]` + level 2 → `[1, 1]`
- `[1, 1]` + level 1 → `[2]`
- `[1, 2]` + level 2 → `[1, 3]`
- `[1, 1, 1]` + level 2 → `[1, 2]` (trunca e incrementa)

## Comportamento de `format_heading()`

Retorna `None` se o vector estiver vazio.
Retorna `Some("1")`, `Some("1.2")`, `Some("1.2.3")` para estados não-vazios.

## Restrições

- Sem I/O de sistema.
- `Default` derivado — estado inicial é vazio com `heading_numbering: false`.

## Critérios de Verificação

- `CounterState::new()` → `format_heading()` retorna `None`
- `step_heading(1)` → `format_heading()` retorna `Some("1")`
- `step_heading(1), step_heading(2)` → `Some("1.1")`
- `step_heading(1), step_heading(2), step_heading(2)` → `Some("1.2")`
- `step_heading(1), step_heading(2), step_heading(1)` → `Some("2")`
- Sequência `1, 2, 3, 2, 1` → `Some("2")`
