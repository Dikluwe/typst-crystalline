# Prompt L0 — `entities/document_format_options`
Hash do Código: 00000000

**Estado:** CONTRATO P1462 CONFIRMADO — sem consumer; hash sentinela não
selado até a materialização posterior ao novo selo discriminatório R2.

**Camada:** L1
**Alvo planejado:** `01_core/src/entities/document_format_options.rs`
**Consumer futuro exclusivo:** `01_core/src/entities/document_format_options.rs`
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

No baseline P1462, `PagedDocument` não transporta opções de formato e não há
`DocumentFormatOptions`/`SvgFormatOptions`. O vanilla ratificado possui um
carrier documental fechado antes dos exporters; `format.svg.pretty` tem
default `false`. O estado é domínio puro de L1, sem I/O ou dependência de
exportador.

## Contrato público P1462

Definir tipos públicos clonáveis, comparáveis e com `Default`:

```rust
pub struct SvgFormatOptions {
    pub pretty: bool,
}

pub struct DocumentFormatOptions {
    pub svg: SvgFormatOptions,
}
```

Os dois defaults produzem `pretty == false`. Não há estado tri-state no
documento: presença/ausência da regra já foi resolvida pela cascata L1. O
`Option<bool>` pertence ao override externo de L3/L4 e não entra nestas
entidades.

## Incompletude deliberada do constructor

O carrier P1462 é deliberadamente incompleto e registra os subconjuntos que o
completam:

- P1463 acrescenta opções de `format.png`;
- P1464 acrescenta opções de `format.pdf`;
- P1465 acrescenta opções de `format.html`.

Até cada L0 e gate próprio, não antecipar campos, enums, placeholders ou maps
abertos. O uso de structs fechadas não afirma que o conjunto futuro terminou.

## Aceitação e gate

Testes provam defaults, clone/igualdade e independência entre duas instâncias.
Não testar layout Rust, tamanho/alinhamento ou debug como paridade. Os campos
públicos ampliam contrato entre camadas. A confirmação humana posterior ao
commit causal P1462 foi recebida em 2026-09-19; consumer e header aguardam o
novo selo discriminatório R2.
