# Prompt L0 — `entities/document_format_options`
Hash do Código: 51e84ec4

**Estado:** P1462 E P1463 MATERIALIZADOS; P1465 CAUSAL — AGUARDA CONFIRMAÇÃO.

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

- P1463 propõe acrescentar opções de `format.png`;
- P1464 acrescenta opções de `format.pdf`;
- P1465 acrescenta opções de `format.html`.

Até cada gate próprio, não antecipar campos, enums, placeholders ou maps
abertos. O uso de structs fechadas não afirma que o conjunto futuro terminou.

## Aceitação e gate

Testes provam defaults, clone/igualdade e independência entre duas instâncias.
Não testar layout Rust, tamanho/alinhamento ou debug como paridade. Os campos
públicos ampliam contrato entre camadas. A confirmação humana posterior ao
commit causal P1462 foi recebida em 2026-09-19; os carriers SVG foram
materializados e validados.

## Contrato público P1463 (MATERIALIZADO)

Acrescentar o carrier público:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct PngFormatOptions {
    pub ppi: f64,
}

impl Default for PngFormatOptions {
    fn default() -> Self { Self { ppi: 144.0 } }
}

pub struct DocumentFormatOptions {
    pub svg: SvgFormatOptions,
    pub png: PngFormatOptions,
}
```

O `Default` de `PngFormatOptions` é manual e produz `ppi == 144.0`;
`DocumentFormatOptions::default()` combina SVG `pretty == false` e PNG
`ppi == 144.0`. O campo guarda o valor documental público tal como escrito;
não divide por 72 e não armazena pixels por ponto. O override externo continua
fora de L1.

Como `f64` não implementa `Eq`, `DocumentFormatOptions` passa a derivar
`PartialEq`, não `Eq`; `SvgFormatOptions` conserva `Eq`. É proibido inventar
igualdade total, bit wrapper ou sentinela para preservar uma mecânica Rust.

Testes provam defaults, clone, `PartialEq`, independência das instâncias e
ortogonalidade SVG/PNG. Não aceitar `Option<f64>` no documento nem acrescentar
PDF/HTML nesse recorte. A confirmação humana posterior ao commit causal P1463
foi recebida em 2026-09-19; os carriers PNG foram materializados e validados.

## Contrato público P1465 (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `8e74134a721310cc4fb2a596a195398e16f1d5db`, o carrier possui
SVG e PNG. O vanilla resolve `format.html.pretty` como booleano documental com
default `false`; o `Option<bool>` existe somente na fronteira do override CLI.

### Contrato

Acrescentar o carrier público:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HtmlFormatOptions {
    pub pretty: bool,
}

pub struct DocumentFormatOptions {
    pub svg: SvgFormatOptions,
    pub png: PngFormatOptions,
    pub html: HtmlFormatOptions,
}
```

`HtmlFormatOptions::default().pretty` e
`DocumentFormatOptions::default().html.pretty` são `false`. O campo é o valor
resolvido pela cascata do documento, não `Option<bool>` e não um alias do campo
SVG. `DocumentFormatOptions` conserva `PartialEq` por causa do `f64` PNG;
nenhuma mudança artificial de trait ou representação é autorizada.

Testes provam default, clone, igualdade, independência de instâncias e
ortogonalidade SVG/PNG/HTML. Não acrescentar PDF, exporter, feature ou lógica
de precedência neste owner. O novo tipo e campo públicos aguardam confirmação
humana posterior ao commit causal P1465 antes de consumer/header/teste RED.
