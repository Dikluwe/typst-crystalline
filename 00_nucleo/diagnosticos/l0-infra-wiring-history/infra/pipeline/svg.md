# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/pipeline/svg.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `d4c654361ad3fec6c061f5bda03045fb619fb67de0d839c49791705c3a79e5ba`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Pipeline — svg
Hash do Código: f6e85901

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c

**Camada:** L3
**Consumer único:** `03_infra/src/pipeline/svg.rs`

## Medição anterior à individualização

Baseline: HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree
não commitado observado em 2026-09-26. Os corpos vigentes do pipeline
e a medição por função estão no diagnóstico
`00_nucleo/diagnosticos/pipeline-atomization.md`. A classificação é
reorganização interna: assinatura, defaults, fases, diagnósticos e
observáveis de linguagem são conservados. Não é medição nova de paridade.

## Contrato proprietário

Possui as fachadas SVG, o contexto de destinos por página e a associação de fontes de glifos. Compila pelo caminho paginado comum, conserva pretty, primeira página, identidade de fontes e Unknown nos casos sem associação comprovada.

A fachada `00_nucleo/prompts/infra/pipeline.md` conserva os caminhos
públicos por reexport explícito. Helpers compartilhados têm visibilidade
`pub(super)`, limitada à família pipeline; nenhuma API externa é adicionada.
Os corpos são transferidos sem alterar algoritmo, fases ou políticas.
As medições e os rótulos históricos de gate nas seções transferidas
conservam a proveniência original; esta individualização não reabre gates
nem atribui nova conclusão de paridade. Referências ao antigo pipeline.rs
com números de linha são medições históricas anteriores à extração.

## P1247 — composição do contexto de destinos SVG

Esta composição é obrigação vigente do pipeline.

Depois de obter o `PagedDocument`, o caminho de compilação SVG seleciona a
página exportada e deriva um `SvgDestinationContext` somente das entradas de
`extracted_label_pages` cuja página corresponda ao índice selecionado, usando a
posição homóloga de `extracted_label_positions`. Ausência de posição não é
inventada e exclui o destino do contexto.

O pipeline passa esse contexto à variante explícita do exporter com ou sem
fontes. Não move os mapas para `Page`, não cria novo `FrameItem`, não repete
layout e não decide ortografia de IDs. O caminho atual exporta apenas a primeira
página; links para outras páginas permanecem `Unknown`, sem fragmento pendente.

Política futura de bundle, nomes de ficheiro ou rotas cross-page pertence ao
caller de composição e exige L0 próprio. A expansão pública de `link()` para
label/location/page-position é decisão separada.

## P1249 — composição da identidade de fonte de glifo proposta

Esta composição é obrigação vigente do pipeline.

No caminho SVG com fontes, a pipeline reutiliza a decisão já existente de
`FallbackFontMetrics::resolve_font_combo(base_char, style)` para cada
`FrameItem::Glyph`, associa um `GlyphFontRequest` normalizado ao `FontKey`
completo efetivamente resolvido e entrega essa associação no contexto imutável
do exporter. Não relê fonte, não repete layout e não altera
`FrameItem::Glyph`.

A chave distingue `base_char`, família solicitada, variante, variações e estado
matemático relevante; é independente de índice, endereço, ordem de travessia e
ocorrência. Entradas iguais têm resolução igual dentro da mesma compilação;
conflito entre resultados para a mesma chave invalida a entrada e conserva
`Unknown`. Ausência não escolhe índice zero por default. A coleta
recursa em background, body, foreground, Group, Link e Semantic como já ocorre
para fontes. O caminho sem fontes não constrói associação fictícia.

## P1462 — extração e resolução de `format.svg.pretty` (MATERIALIZADO)

### Medição anterior à decisão

No baseline `7442d33b6bf48f136826d28cf71bd62f8710a432`, as fachadas SVG
sem argumento explícito delegam `false`; a fachada ativa recebe `pretty: bool`
e constrói `SvgOptions` diretamente. A pipeline não consulta conteúdo nem
`PagedDocument`. O cristalino não possui opção documental.

No vanilla ratificado, probes válidos numa página vazia de 10 pt mediram:

```text
documento ausente + CLI ausente       -> compacto
documento true    + CLI ausente       -> pretty
documento true    + CLI false         -> compacto
documento false   + CLI true          -> pretty
```

O primeiro SVG tem 243 bytes e o segundo 248 bytes/3 LF no corpus medido; esses
números são evidência auxiliar daquela fixture, não contrato de bytes. A
morfologia normalizada do SVG permanece igual.

### Extração documental

No caminho paginado, chamar `collect_document_format_options` sobre o conteúdo
realizado final que alimenta o candidato de layout. A extração ocorre depois
das realizações/contextos necessários e antes de devolver um
`PagedDocument` completo. Em `Complete(mut document)`, associar exatamente as
opções extraídas ao campo `document.format_options`. Erro de regra em container
aborta pelo canal `SourceResult`; não exportar documento provisório.

Relayouts do ciclo calculam as opções do respectivo candidato sem acumular
estado entre iterações. A operação é pura L1; L3 não percorre `Content`, não
interpreta chaves de style e não duplica a mensagem de erro.

### Precedência e APIs SVG

Adicionar uma entry point pública explícita que receba
`pretty_override: Option<bool>`. O valor efetivo passado a `SvgOptions` é:

```text
pretty_override.unwrap_or(document.format_options.svg.pretty)
```

As fachadas SVG sem argumento `pretty` delegam `None`, permitindo a opção do
documento. A fachada de compatibilidade que já recebe `pretty: bool` delega
`Some(pretty)`, preservando a semântica de override explícito e sua assinatura.
O exporter `infra/export/svg.rs` continua a receber somente o booleano final e
não aprende precedência.

Este fio aplica o Núcleo Tekt `format-option-precedence` pinado acima. O SVG
interno de `html.frame` não é um export do documento e permanece controlado
pelo modo HTML; não deve observar `format.svg`. PDF, PNG e HTML permanecem
inalterados até P1463–P1465.

### Aceitação e gate

Testes de pipeline cobrem as quatro linhas da matriz, default, regras root,
rejeição em containers, erro de tipo/named vindo de eval e neutralidade de
PDF/PNG/HTML/frame HTML. Comparar estrutura/whitespace pertinente, sem fixar
bytes auxiliares como contrato.

A nova API pública e a extração antes do documento completo foram
materializadas depois da confirmação humana de 2026-09-19.
