# Prompt L0 — `compiler/layout/format_options`
Hash do Código: f056d980

**Estado:** P1462, P1463 E P1465 MATERIALIZADOS; RETIFICAÇÃO P1466 E EXTRAÇÃO
P1467 PROPOSTAS; AGUARDAM GATE ADR-0127.

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

A operação pública e a etapa de extração foram materializadas e validadas
em P1462 depois da confirmação humana de 2026-09-19.

## P1463 — extração root de `format.png.ppi` (MATERIALIZADO)

### Medição anterior à decisão

O consumer P1462 reconhece somente `format.svg.pretty`. O vanilla aceita
`format.png.ppi` no root, aplica a última decisão lexical, trata set rule vazia
como neutra e rejeita a ocorrência dentro de container com a mesma mensagem
já contratada para SVG.

### Contrato

Ampliar as duas travessias existentes para reconhecer também a chave canônica
`format.png.ppi`. A travessia root aceita `Value::Float`, atualiza
`options.png.ppi` em ordem lexical e começa do default 144.0. A travessia
exaustiva conta ocorrências PNG e SVG; qualquer ocorrência de qualquer formato
fora dos envelopes root causa exatamente:

```text
format set rules are not allowed inside of containers
```

Uma regra vazia não produz chave e, portanto, não apaga valor anterior. Uma
chave PNG interna com variante diferente de `Value::Float` falha fechada com
diagnóstico atribuível; nunca volta a 144.0. Chaves futuras PDF/HTML continuam
neutras até seus L0s.

Não misturar os campos: regra SVG não altera PNG e regra PNG não altera SVG.
O owner continua puro, não conhece CLI, unidade PPI, `RenderOptions` ou target
selecionado. Coleção num único passe ou helpers por chave são mecânica livre;
ordem, default, isolamento por campo e fronteira root são contrato.

### Aceitação e gate

Testar ausência/default, integer já convertido em eval, float, repetição nas
duas ordens, regra vazia, composição SVG+PNG, containers distintos, tipo
interno impossível e chave futura neutra. A confirmação humana posterior ao
commit causal P1463 foi recebida em 2026-09-19; a chave PNG foi materializada
e validada sem alterar a semântica SVG.

## P1465 — extração root de `format.html.pretty` (MATERIALIZADO)

### Medição anterior à decisão

No baseline `8e74134a721310cc4fb2a596a195398e16f1d5db`, as travessias
vigentes reconhecem somente `format.svg.pretty` e `format.png.ppi`. No vanilla,
`format.html.pretty` root controla o whitespace estrutural do export HTML, a
última decisão não vazia vence, regra vazia é neutra e ocorrência em container
falha com a mesma mensagem já contratada para SVG/PNG.

### Contrato

Ampliar as duas travessias existentes para a chave canônica
`format.html.pretty`. A travessia root aceita somente `Value::Bool`, atualiza
`options.html.pretty` em ordem lexical e começa do default `false`. A
travessia exaustiva conta ocorrências HTML junto das ocorrências SVG/PNG;
qualquer chave de formato fora dos envelopes root causa exatamente:

```text
format set rules are not allowed inside of containers
```

Regra vazia não produz chave. Uma chave HTML interna de variante diferente de
`Value::Bool` falha fechada com diagnóstico atribuível, nunca volta ao default.
SVG, PNG e HTML permanecem ortogonais: nenhuma chave altera o campo de outro
formato. Chaves PDF futuras continuam neutras até L0 próprio.

Este owner continua puro e não conhece feature, CLI, target, encoder ou
serialização. A pipeline decide quando chamar a coleta no caminho semântico
HTML. Implementação em um passe ou helpers separados é mecânica; default,
ordem, isolamento, fronteira root e mensagem são contrato.

Aceitação cobre ausência/default, true/false, repetição nas duas ordens, regra
vazia, composição SVG+PNG+HTML, dois containers, tipo interno impossível e
neutralidade PDF. A confirmação humana posterior ao commit causal
`e300314e2f87e90e543c4923a8cae97f9ad978a8` foi recebida em 2026-09-20; a
extração e os testes unitários foram materializados.

## P1466 — preservar PPI, não escala de raster (PROPOSTA; AGUARDA GATE ADR-0127)

`collect_document_format_options` continua a guardar em
`options.png.ppi` o valor público em pixels por polegada. Este owner não divide
por 72, não constrói `RenderOptions` e não conhece pontos tipográficos. A
retificação P1466 corrige exclusivamente o consumer L3 que havia confundido a
unidade; coleta, precedência, default `144.0`, erro de container e isolamento
dos formatos permanecem idênticos.

## P1467 — extração root de `format.pdf.tagged` e `format.pdf.pretty` (MATERIALIZADO; gate ADR-0127 satisfeito)

### Medição anterior à decisão

No baseline `306ca50022104619447fae2625090b8d9c7b0a35`, as travessias vigentes
reconhecem `format.svg.pretty`, `format.png.ppi` e `format.html.pretty`; não há
braço para chaves `format.pdf.*`. `compiler/eval/rules.rs` ainda não produz
essas chaves (ver `compiler/eval/rules.md` P1467), então hoje elas nunca
aparecem em `Styles.delta().custom`.

Probes contra o vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`
(medidos em 2026-09-20T19:18 via `typst compile`) confirmam, no root do
documento:

```text
#set format.pdf(tagged: false)                    -> PDF sem struct tree
#set format.pdf(tagged: false); #set format.pdf() -> regra vazia neutra;
                                                       PDF permanece sem struct tree
#set format.pdf(tagged: false); #set format.pdf(pretty: true)
                                                   -> tagged:false E pretty:true
                                                      (campos independentes)
#block[ #set format.pdf(tagged: false) ... ]      -> error: format set rules
                                                      are not allowed inside
                                                      of containers
```

Essa é a mesma mensagem já contratada para SVG/PNG/HTML, medida no mesmo
ponto (`typst-realize/src/lib.rs:603-670` no vanilla).

### Contrato

Ampliar as duas travessias existentes (exaustiva e root) para reconhecer
também as chaves canônicas `format.pdf.tagged` e `format.pdf.pretty`,
seguindo exatamente o padrão já aplicado a SVG/PNG/HTML:

- `format.pdf.tagged` aceita `Value::Auto` ou `Value::Bool`. A travessia root
  atualiza `options.pdf.tagged` para `PdfTagged::Auto` ou
  `PdfTagged::Explicit(bool)`, em ordem lexical, começando do default
  `PdfTagged::Auto`. Uma variante interna diferente dessas duas falha fechada
  com diagnóstico atribuível (estado interno impossível), nunca retorna ao
  default.
- `format.pdf.pretty` aceita somente `Value::Bool`, atualiza
  `options.pdf.pretty` em ordem lexical a partir do default `false`, e falha
  fechada do mesmo modo para variante interna inválida.

A travessia exaustiva conta ocorrências de qualquer chave `format.*`
(SVG/PNG/HTML/PDF, os dois campos PDF inclusive); qualquer ocorrência fora dos
envelopes root causa exatamente `format set rules are not allowed inside of
containers`, sem produzir `PagedDocument` parcial. Regra vazia não produz
chave para nenhum dos dois campos e por isso não apaga decisão anterior de
nenhum dos dois. Os dois campos de PDF são ortogonais entre si e com
SVG/PNG/HTML: uma chave nunca altera o campo de outro formato ou do outro
campo do mesmo formato.

Este owner continua puro: não conhece CLI, `PdfTags`, `PdfSerializationMode`,
override externo ou exporter. `pdf.pages`/`pdf.standard` continuam neutros até
P1468/P1469. Implementação em um passe único ou helpers por chave é mecânica
livre; ordem, default, isolamento por campo e fronteira root são contrato.

### Aceitação e gate

Testar ausência/default (`Auto`/`false`), `auto`/`true`/`false` para
`tagged`, `true`/`false` para `pretty`, repetição/inversão em ambas as ordens,
regra vazia depois de decisão anterior, composição com SVG/PNG/HTML, os dois
campos PDF definidos independentemente na mesma regra e em regras distintas,
pelo menos dois containers estruturalmente distintos, e tipo interno
impossível para cada campo. Este é um dos quatro L0s do gate ADR-0127 de
P1467.

A confirmação humana explícita posterior ao commit causal somente-L0
`e675e8b7acc29e272bd59939e44681a7b521acd3` foi recebida em 2026-09-20.
Agente A implementou a extração sem ver os testes; Agente B escreveu 9
testes a partir do mesmo baseline sem ver a implementação, todos falhando
por erro de compilação (`options.pdf`/`PdfTagged` inexistentes). O teste
independente passou de RED (9/9 falhas de compilação) a GREEN (9/9) após a
integração, sem regredir SVG/PNG/HTML (16/16 no ficheiro, incluindo os
irmãos).
