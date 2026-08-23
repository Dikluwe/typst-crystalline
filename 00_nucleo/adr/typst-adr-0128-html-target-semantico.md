# ADR-0128 — HTML é target semântico, não conversão do layout paginado

**Estado:** PROPOSTO  
**Data:** 2026-08-23

## Contexto medido

O vanilla ratificado `a51e02804` produz, para `Hello, parity.`, um documento
HTML5 compacto de 175 bytes. O cristalino não possui exporter HTML,
`OutputFormat::Html` nem dispatch L4. `eval/mod.rs:1813-1816` fixa `target()` em
`"paged"` e os exporters existentes atravessam `PagedDocument`. A ADR-0076
reservou HTML/bundle para ADR explícita.

## Decisão

HTML é target semântico separado:

```text
Source → eval(target=html) → Content → expansão HTML → HTML
```

É proibido rasterizar páginas, embrulhar SVG/PDF ou converter
`PagedDocument`. O target é dado explícito do eval; APIs existentes preservam
`target="paged"`.

### Entrega inicial P1137-X-002

Recorte explicitamente incompleto: documento, parágrafo, texto, sequência,
heading, strong, emph e linebreak; escape HTML; envelope HTML5 vanilla;
warning experimental; `target() == "html"`. Variante não suportada produz
diagnóstico, nunca desaparece.

Completudes futuras: tabelas/listas/links/imagens/math/CSS, fixpoint HTML e
bundle. Cada grupo exige L0 antes do código.

## Consequências

A sentinela simples pode tornar-se `MATCH`, sem afirmar paridade HTML global.
A separação impede que geometria paginada contamine a morfologia HTML.
