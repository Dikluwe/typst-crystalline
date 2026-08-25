# Prompt L0 — exportação HTML semântica
Hash do Código: 29d9c88d

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

## P1165 — serialização de `HtmlElem` (RASCUNHO; ADR-0127)

**Medição:** com feature ligada, o vanilla representa
`html.elem("article", attrs: (lang: "pt"))[Olá]` como nó `elem` e o exporter
preserva tag/atributos/body. O exporter cristalino não possui variante HTML.

Após aprovação do contrato L1, `export_html` serializa `Content::HtmlElem`
diretamente: valida nome já tipado, escapa valores de atributo e texto, mantém
ordem de atributos, respeita elementos void/raw conforme a tabela aprovada e
recursa no body. O primeiro corte garante `html.elem`; tags tipadas, frame,
CSS, MathML e expansão completa permanecem fora e devem falhar explicitamente
quando ainda não representáveis.

## P1167 — primeiro lote tipado (APROVADO NO GATE ADR-0127 EM 2026-08-25)

Medição ratificada da fixture aninhada confirmou que `div`, `span`, `p`,
`h1..h6`, `strong`, `em` e `ul` usam serialização normal de abertura, body e
fecho; nenhum é void/raw. Como os constructors produzem o mesmo `HtmlElem` e
os casts ocorrem em L1 antes do exporter, P1168 não altera a assinatura nem o
algoritmo público deste owner: recursão/escape/ordem existentes devem emitir
os nós diretamente, sem wrapper `<p>` adicional.

`br` fica fora de P1168. P1171 deve introduzir uma tabela declarativa void e
então exigir `<br>` sem end tag. O estado P1166 `<br></br>` fica registado como
gap conhecido, não como comportamento legitimado.
