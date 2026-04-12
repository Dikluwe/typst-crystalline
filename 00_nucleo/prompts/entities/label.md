# Prompt — `entities/label.rs`

## Camada: L1

## Propósito

Estrutura de domínio para etiquetas semânticas de conteúdo.

Produzida pela sintaxe `<nome>` em Typst (ex: `= Introdução <intro>`).
Usada pelo motor de introspecção para resolver referências cruzadas.

## Estrutura

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);
```

## Restrições

- Sem I/O de sistema.
- String simples — não usar EcoString para evitar dependência desnecessária.
- `Hash` obrigatório para uso em `HashMap` no futuro motor de introspecção.

## Critérios de Verificação

- `Label("a") == Label("a")` → true
- `Label("a") != Label("b")` → true
- `Label` implementa `Hash` e `Eq`
