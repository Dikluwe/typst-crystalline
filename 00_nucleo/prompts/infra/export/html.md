# Prompt L0 — exportação HTML semântica
Hash do Código: 129adddd

**Camada:** L3  
**Ficheiro alvo:** `03_infra/src/export/html.rs`  
**ADR:** ADR-0128

## Medição anterior à decisão

O vanilla ratificado gera 175 bytes para `Hello, parity.`. Não existe exporter
HTML cristalino; SVG/PDF recebem `PagedDocument`, forma inadequada para HTML.

## Contrato inicial incompleto

```rust
pub fn export_html(content: &Content) -> Result<String, SourceDiagnostic>;
```

Emite doctype/head/body compactos e converte semanticamente o subconjunto da
ADR-0128. Texto de topo é agrupado em `<p>`. Escapa `& < > "`. Heading vira
`h1..h6`; strong/emph preservam morfologia. Variante fora do subconjunto retorna
erro explícito.

Testes: fixture plain byte-idêntica; escape; heading; strong/emph; variante não
suportada falha. A spec permanece incompleta até os grupos futuros da ADR-0128.
