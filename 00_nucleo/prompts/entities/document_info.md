# Prompt L0 — `entities/document_info`
Hash do Código: bd3e50a2

## Camada
L1

## Ficheiro alvo
`01_core/src/entities/document_info.rs`

## Propósito
Transporte puro de metadados do documento definidos por `#set document(...)`
desde a avaliação (`EvalContext`) até ao módulo (`Module`) e, via pipeline, até
ao documento paginado (`PagedDocument`) e ao exportador PDF (`/Info`).

## Tipo

```rust
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DocumentInfo {
    pub title: Option<EcoString>,
    pub author: Option<EcoString>,
    pub keywords: Option<EcoString>,
}
```

- Campos opcionais — só os argumentos efectivamente fornecidos em
  `#set document(...)` são preenchidos.
- `author` aceita tanto `str` como `array` de strings (convertido para uma
  única string separada por vírgula, paridade vanilla simplificada).
- `Default` e `empty()` produzem metadados vazios.
- `is_empty()` é true quando todos os campos são `None`.

## Semântica

Não contém I/O nem lógica de render. É um contentor de valores já avaliados.
A decisão de formato (ex.: data de criação, creator) fica no exportador PDF
(L3), não aqui.
