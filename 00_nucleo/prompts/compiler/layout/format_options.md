# Prompt L0 — `compiler/layout/format_options`
Hash do Código: 00000000

**Estado:** CONTRATO P1462 CONFIRMADO — sem consumer; hash sentinela não
selado até a materialização posterior ao novo selo discriminatório R2.

**Camada:** L1
**Alvo planejado:** `01_core/src/compiler/layout/format_options.rs`
**Consumer futuro exclusivo:** `01_core/src/compiler/layout/format_options.rs`
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## Medição anterior à decisão

No baseline P1462 não existe carrier documental nem extração de
`format.svg.pretty`. `StyleDelta.custom` já transporta chaves tipadas por valor
e `Content::Styled`; `Content::map_content` já oferece travessia exaustiva das
variantes. No vanilla ratificado,
`lab/typst-original/crates/typst-realize/src/lib.rs:603-670` coleta opções de
formato na realização top-level e rejeita regra equivalente dentro de
containers com `format set rules are not allowed inside of containers`.

Probes medidos confirmam que regra top-level altera o SVG, enquanto a mesma
regra dentro de `{ ... }` falha com essa mensagem. É inferência que comparar a
travessia exaustiva da árvore com uma travessia restrita ao envelope root
permite distinguir as duas classes sem ensinar containers individualmente ao
owner. Uma variante que escape de `Content::map_content`, uma regra de formato
sem `Content::Styled` ou a perda da ordem lexical refuta a inferência e exige
reabrir o desenho antes de código.

## Contrato

Expor a operação pura:

```rust
pub fn collect_document_format_options(
    content: &Content,
) -> SourceResult<DocumentFormatOptions>
```

A operação começa em `DocumentFormatOptions::default()` e reconhece apenas a
chave canônica `format.svg.pretty` com `Value::Bool`. Uma travessia exaustiva
conta todas as ocorrências dessa chave. Em paralelo, a travessia root desce
somente por envelopes transparentes de raiz (`Content::Sequence` e
`Content::Styled`), na ordem lexical/cascaded, e aplica as ocorrências
alcançadas. Se a contagem exaustiva exceder a contagem root, devolver erro com
a mensagem exata:

```text
format set rules are not allowed inside of containers
```

Nenhum `PagedDocument` parcial deve ser produzido nesse caso. Entre regras
root repetidas, a decisão lexical/cascaded mais interna ou posterior é a
efetiva. Ausência conserva `pretty == false`. O helper não remove nem reescreve
conteúdo; apenas extrai opções.

Um `Content::Styled` cujo delta não contém `format.svg.pretty` é envelope
transparente, não uma ocorrência. Em particular, `#set format.svg()` não cria
a chave: depois de uma regra `pretty: true`, a regra vazia preserva `true`,
conforme a medição vanilla pré-candidata R1.

O cast boolean já foi fechado em eval; encontrar outra variante de `Value` na
chave canônica é estado interno inválido e deve falhar fechado, nunca usar
fallback. Chaves futuras de PNG/PDF/HTML permanecem ignoradas até os seus L0s,
sem serem classificadas como ocorrência SVG.

## Fronteiras

Este owner não chama layout, não cria `Engine`, não lê `World`, não conhece
flags CLI e não exporta bytes. A fachada `compiler/layout/mod.rs` apenas o
reexporta. L3 chama o helper depois da realização final candidata e associa o
resultado ao documento completo. O uso de travessia, contadores ou tipos de
coleção é mecânica; default, ordem, top-level, rejeição de container e mensagem
de erro são observáveis da linguagem.

## Aceitação e gate

Testes puros cobrem ausência, `true`, `false`, regra vazia neutra depois de
`true`, repetição/inversão, sequence,
styled root, styled dentro de ao menos dois containers estruturalmente
distintos, tipo interno impossível e neutralidade de chaves não-SVG. Mutation
obrigatória inverte precedência, faz regra vazia escrever `false`, ignora
nested, trata nested como root, muda o default e aceita valor não booleano.
Unknown bloqueia.

A operação pública e a nova etapa de extração antes de selar o documento são
mudanças de API/fase. A confirmação humana posterior ao commit causal P1462
foi recebida em 2026-09-19; consumer, teste RED e header aguardam o novo selo
discriminatório R2.
