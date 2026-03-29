# Dívida de instrumentação — ADR-0006

Os seguintes pontos de timing foram removidos para manter L1 puro.
Religação prevista no Passo 10 (isolamento de comemo/infra).

| Função       | Nome do scope original |
|--------------|------------------------|
| parse()      | "parse"                |
| parse_code() | "parse-code"           |
| parse_math() | "parse-math"           |

## Como religar no Passo 10

Localizar todos os `// ADR-0006: timing removed — ver 01_core/DEBT.md`
em `01_core/src/rules/parse.rs` e substituir `timing_scope!("...")` por
o mecanismo de telemetria escolhido (trait injectável ou outro).

Ver: `00_nucleo/adr/typst-adr-0006-typst-timing.md`

## StyleChain — dívida estrutural do sistema de estilos

`TextStyle { bold, italic, size }` é um struct plano.
O Typst real tem centenas de propriedades de estilo (kerning, tracking,
cores, stroke, fallback fonts, propriedades de tabela, etc.).
Manter um struct plano significa que cada nó da árvore copia N bytes
onde N cresce linearmente com propriedades — inaceitável a longo prazo.

O Typst original usa StyleChain: lista ligada construída de trás para
a frente. Cada nó carrega apenas o "delta" (o que mudou); o Layouter
sobe a cadeia para encontrar o primeiro valor definido. Custo: O(1)
de alocação por nó, não O(N).

A sintaxe `#set text(font: "Arial", size: 10pt)` requer StyleChain —
é incompatível com struct plano.

Estimativa de refactorização: Passo 30+, após o pipeline básico estar
estável. Não tentar antes.

Ficheiros a refactorizar quando chegar a hora:
- 01_core/src/entities/layout_types.rs (TextStyle → StyleChain)
- 01_core/src/rules/layout.rs (Layouter, contexto de estilo)
- 01_core/src/entities/content.rs (Content::Styled com StyleChain)
- 03_infra/src/export.rs (resolução de estilos para PDF)
