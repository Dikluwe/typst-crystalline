# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/pipeline/paged.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `eb13522c4ed60953178ceda97c74fcda970fe394438350f3683c166aefd41ac9`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Pipeline — paged
Hash do Código: d6b0f8b1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43

**Camada:** L3
**Consumer único:** `03_infra/src/pipeline/paged.rs`

## Medição anterior à individualização

Baseline: HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree
não commitado observado em 2026-09-26. Os corpos vigentes do pipeline
e a medição por função estão no diagnóstico
`00_nucleo/diagnosticos/pipeline-atomization.md`. A classificação é
reorganização interna: assinatura, defaults, fases, diagnósticos e
observáveis de linguagem são conservados. Não é medição nova de paridade.

## Contrato proprietário

Orquestra a compilação paginada, introspecção runtime, realização de outlines, callbacks math/par.line, PageStore e binding dos números lógicos. Devolve somente candidatos completos, conserva os ciclos e os orçamentos vigentes e passa ao shaping a geração final validada.

A fachada `00_nucleo/prompts/infra/pipeline.md` conserva os caminhos
públicos por reexport explícito. Helpers compartilhados têm visibilidade
`pub(super)`, limitada à família pipeline; nenhuma API externa é adicionada.
Os corpos são transferidos sem alterar algoritmo, fases ou políticas.
As medições e os rótulos históricos de gate nas seções transferidas
conservam a proveniência original; esta individualização não reabre gates
nem atribui nova conclusão de paridade. Referências ao antigo pipeline.rs
com números de linha são medições históricas anteriores à extração.

## P1140.4-A2 — introspecção runtime no caminho de produção

**Medição:** produção chamava só `introspect_with_introspector`, enquanto
`state.update(Func)`, `state.display`, `counter.display(callback)` e numbering
funcional de equação eram materializados somente no caminho runtime L1.

Depois de expandir `ContextBlock` e estabilizar Locations, L3 constrói
`Engine + EvalContext` e chama `introspect_with_runtime`. L3 não executa
callbacks nem duplica regras. Diagnósticos são propagados e layout recebe o
`TagIntrospector` runtime final. A primeira introspecção direta permanece só
para localizar/expandir context blocks. HTML fica fora sem medição própria.

## Conversão de `layout_warnings` em diagnósticos

Após `layout_with_introspector_and_metrics`, a pipeline converte cada
entrada de `doc.layout_warnings` (strings puras produzidas em L1 — ex.:
aviso de body de footnote que excede a página/coluna, ver
`compiler/footnote_overflow_columns.md`) em
`SourceDiagnostic::warning(Span::detached(), msg)` e adiciona-a ao `Vec`
de `warnings` que a pipeline já retorna. L1 permanece data-only (ADR-0043):
nenhum `SourceDiagnostic` é construído em L1.

## P1159 — realização de page numbering

Após o primeiro layout, a pipeline realiza cada `Numbering` com Engine em duas
vistas: visível recebe `[current, total]` e referência recebe `[current]`.
Constrói um `PageStore` completo, reinjeta posições e páginas no introspector e
executa novo layout. O ciclo é limitado a cinco passagens e converge pelo
número de páginas, snapshots lógicos, objetos crus e conteúdo realizado.
Erros do callback são devolvidos como `SourceDiagnostic`; L3 não duplica o
dispatch Pattern/Func, que pertence a `stdlib/numbering`.

A vista unária é realizada somente para as páginas que são alvo de um
`ref(form: "page")` resolvido pelo primeiro layout. Isso preserva a diferença
observável de aridade: uma função binária é válida como numbering visível num
documento sem referência, mas falha com `missing argument: total` quando a
referência exige a chamada unária. Páginas não referenciadas não executam o
callback unário nem produzem warnings laterais desse consumer.

A substituição contextual preserva o próprio `ContextBlock` como marcador
zero-size antes do conteúdo realizado. O marcador mantém a Location estável e
permite selar a sua Position; em cada iteração a expansão parte do conteúdo
original, portanto o marcador não se acumula nem duplica conteúdo visível.

Seleção e recolha de fontes percorrem `background`, `items` e `foreground` em
ordem. Running matter realizado não pode ficar fora do conjunto de fontes
embutidas só por residir na camada marginal.

## P1486 — fechar o snapshot lógico antes do PDF (PROPOSTO; GATE ADR-0127)

### Medição anterior à decisão

`realize_page_numberings` em `03_infra/src/pipeline.rs:1780-1856` já constrói
`PageStore` com `logical_numbers` a partir de `logical_start`; o resultado
final da compilação sai como `(PagedDocument, TagIntrospector)` em
`pipeline.rs:1558-1575`. O probe com `counter(page).update(10)` do diagnóstico
`typst-p1486-page-labels.md` mostra que índice físico não substitui esse
snapshot. A inferência de que a store devolvida acompanha o documento final
seria refutada por desalinhamento de comprimento ou número depois da última
passagem de layout. A igualdade de store/estrutura é mecânica; o rótulo PDF
e o número lógico de linguagem são semânticos (ADR-0107).

No bundle, `pipeline.rs:812-842` exporta documentos em ordem; em
`pipeline/context_stabilization.rs:829-915`, o `PageStore` concatena as
páginas de cada body paginado e não avança em HTML. O renderer do bundle
(`pipeline.rs:950-1070`) recebe o store global mas hoje devolve só bytes,
sem transportar o offset físico do documento. Inferência: o offset deve
acompanhar cada documento paginado; refutação seria uma sequência PDF/HTML/PDF
em que a label da segunda saída consultasse a posição errada no store.

### Decisão condicionada

Depois da última estabilização de layout, antes do shaping/export, copiar
para cada `doc.pages[i].logical_number` o valor de
`intr_for_positions.page_store.logical_number_for_page(i + 1)` do snapshot
final. Exigir alinhamento 1:1; não inventar número lógico a partir do índice
quando o store deveria estar completo. Documentos sem páginas ficam vazios.
Isto não move a realização entre fases nem cria novo callback. PNG/SVG/HTML
ignoram o novo campo e preservam seus comportamentos. A alteração depende do
campo público `Page.logical_number` em `entities/layout_types.md` P1486 e
aguarda a mesma confirmação ADR-0127.

No bundle, percorrer os documentos na ordem usada pela store global, com
offset físico acumulado apenas para bodies paginados (PDF/SVG/PNG); HTML e
assets não consomem entrada. Antes de exportar cada body paginado, copiar
`bundle_intr.page_store.logical_number_for_page(offset + i + 1)` para
`document.pages[i].logical_number`; avançar o offset por
`document.pages.len()`, inclusive para SVG/PNG. O renderer pode devolver
internamente a contagem de páginas para esse fim, sem mudar API pública.
Desalinhamento da store é erro, não fallback para índice exportado.
Testes incluem PDF/HTML/PDF com update do contador e seleção na segunda
saída, além de SVG/PNG entre dois PDFs.

## P1288 — features também no pipeline paginado (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`03_infra/src/pipeline.rs:77-82` aceita features somente no adaptador de eval;
`:111-150` possui helper target+features e default vazio, e `:156-197` aplica
o gate somente ao HTML. Os entrypoints paginados/PDF não recebem a coleção,
portanto `a11y-extras` não pode chegar ao eval de produção PDF.

### Decisão proposta

Os entrypoints paginados usados por L4 recebem ou delegam a variantes que
recebem `entities::compiler_features::Features` e o encaminham sem transformação
ao mesmo eval. As variantes de compatibilidade sem parâmetro continuam com
conjunto vazio; não existe ativação por PDF, HTML, `PdfTags`, `StreamMode` ou
exporter.

Depois do layout, a pipeline preserva integralmente uma única árvore semântica
de tabela até o builder, inclusive THead/TBody quando aplicáveis e identidade
lógica de headers repetidos em várias páginas. Transporta separadamente os
vínculos físicos por página necessários a StructParents, MCID e ParentTree.
Não reclassifica células, não deriva summary de caption, não transforma
metadata em texto e não muda a ordem eval→introspect→layout→export. Uma árvore
ausente/ambígua continua falha ou Unknown conforme o contrato congelado; nunca
ganha fallback de sucesso.

## P1291 — realização entre passagens de `cancel.angle` (PROPOSTO; GATE ADR-0127)

### Medição anterior à decisão

A pipeline já é o ponto onde callbacks que exigem `Engine` são executadas:
page numbering é realizada entre passagens de layout em
`03_infra/src/pipeline.rs:671-735`. Em contraste, o `MathLayouter` L1 não
possui `World`, `Scopes` ou `Engine`. O vanilla chama `CancelAngle::Func` com o
ângulo default somente depois de medir o body e faz cast do retorno para Angle
(`lab/typst-original/crates/typst-layout/src/math/cancel.rs:92-103`).

### Decisão proposta

No caminho paginado, a pipeline chama a entry point de passagem com store
vazia e recebe `Pending(transcript)`; o documento provisório já foi descartado
em L1 e não fica acessível. Fora do layouter, onde `Engine` existe, realiza cada
request em ordem por `apply_func(func, [Value::Angle(default)], ...)`, usando o
`StyleChain` e span capturados. Somente retorno convertível a `Angle` gera uma
`MathCancelResolution`; erro da closure ou cast inválido aborta sem exportar o
documento provisório.

As resoluções completas formam `SealedMathCallbacks`. A pipeline repete layout
com essa store. `Complete(document)` encerra a realização; novo
`Pending(transcript)` invalida a store, realiza o transcript novo e repete sob
teto finito. Teto excedido ou store parcialmente consumida é diagnóstico de
não convergência, nunca sucesso, `Unknown` rebaixado ou `Auto` final.

A pipeline encapsula isso num ciclo `stabilize_math_layout`: antes do layout
inicial e antes de **cada** relayout externo do ciclo P1159, repete
`Pending → realizar → nova passagem` até obter `Complete`. A rotina L1 pode
internamente tentar mais de um `Layouter::new` por TOC, mas somente o transcript
da tentativa candidata retorna ao L3; transcripts de tentativas internas
rejeitadas são descartados. `realize_page_numberings` recebe exclusivamente um
documento `Complete`. Se a store de page numbering provocar relayout, esse
novo candidato volta a estabilizar callbacks math antes de alimentar a próxima
realização de numbering. Esse ciclo não é mesclado com
`compiler::introspect::run_fixpoint` e não compartilha estado com ele.

Uma request corresponde a uma chamada da função. Assim `cross=true` produz e
executa duas requests consecutivas, conforme as duas chamadas reais de
`draw_cancel_line` no vanilla. A store é local ao ciclo de compilação, não é
global nem cacheada entre compilações. PNG/SVG reutilizam o documento paginado
já estabilizado. HTML não atravessa layout math paginado e fica fora até
medição própria.

Esta forma replica P240/P241/P1159: L3 realiza callbacks entre passagens e L1
layout recebe somente dados validados. Não existe implementação L3 de trait L1,
reentrada layout→eval ou execução em `native_math_cancel`. A ligação de fase
continua gate ADR-0127.

## P1340 — delegação da estabilização contextual seletiva

### Medição anterior à decisão

A medição histórica congela o antecedente: a expansão
`:244-281` aborta por Err e o ciclo `:721-760` termina por páginas iguais.
As cláusulas P1339 antes contidas nesta seção continuam necessárias, mas
sua coordenação passa ao owner individualizado, evitando acrescentar essa
responsabilidade ao monólito.

### Contrato proprietário de integração

Declarar módulo privado descendente `context_stabilization`. O owner
`infra/pipeline/context_stabilization.md` legitima somente seu consumer.
As fachadas públicas de expansão e compilação preservam suas assinaturas;
a pipeline entrega World/Source, conteúdo original e introspector de origem
à sessão seletiva. Nenhum caller implementa segunda regra de estabilização.

No caminho paginado com seleção efetiva, receber conteúdo, introspector,
documento e warnings finais da coordenação seletiva; seguir com os mesmos
passos de metadados, shaping e exportação. Não executar depois outro ciclo
contextual/P1159 sobre esse resultado. Sem seleção, preservar o percurso
legado e seus diagnósticos. Os helpers privados de layout math, page numbering
e introspecção runtime continuam disponíveis ao descendente, sem alargar
visibilidade pública ou transferir suas próprias responsabilidades.

As fachadas de expansão delegam sua parte à mesma sessão, sem alegar
estabilidade de páginas que não produziram. HTML mantém caminho semântico
separado. A individualização não corrige show rules/P1037 incidentalmente,
não altera features/defaults nem fecha o P1339. Os L0s de leituras e de eval
permanecem proprietários da semântica e de sua revalidação.

## P1403 — preservar PageStore no introspector devolvido

### Medição anterior à decisão

Com documento sem blocos `context`, `#set page(numbering: "I")` seguido de
avaliação por `--in` devolve página `1`, mas `location.page-numbering()` devolve
`none`. O loop legado realiza e injeta corretamente o `PageStore` em
`layout_intr`, porém devolve o clone pré-layout `intr_for_positions`, no qual
só injeta posições ao final. Assim o consumer contextual de P1403 não consegue
observar o numbering que o próprio pipeline já realizou.

### Obrigação

Antes de devolver o caminho legado estabilizado, copiar o `PageStore` já
realizado de `layout_intr` para `intr_for_positions`. Não executar novo layout,
não recalcular numbering e não alterar o caminho seletivo, que já devolve a
candidate paginada. Posições finais continuam sendo injetadas depois do
shaping. Esta é correção interna de perda de snapshot em paridade contínua.

## P1406 — realização pós-introspecção de outline (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `fe3209458`, eval produz conteúdo e show rules antes de a
introspecção sintetizar `OutlineEntry`; o layout depois fabrica texto próprio.
No vanila ratificado, a sequência de entries é realizada sob o escopo lexical
do outline entre introspecção e layout. A fixture com dois outlines prova que a
pilha final/global não representa esse observável.

### Decisão proposta

No caminho paginado, L3 orquestra
`eval → introspecção → realização L1 de outlines → layout`. Cada ocorrência usa
seu snapshot lexical causal; L3 não interpreta selector, parent, entry ou show
default. Se páginas ou `prefix-info` mudarem, o candidato volta ao ciclo
limitado já contratado, sempre reconstruído da entrada não transformada. Só um
candidato convergido alcança shaping/export. HTML e LoF/LoT não atravessam esta
fase neste passo.

Confirmação humana explícita recebida em 2026-09-15; esta alteração da fronteira
eval/introspecção/layout está autorizada pelo gate ADR-0127.

## P1411 — contexto produzido pela realização de outline

### Medição anterior à decisão

No baseline `82e210bd1826ccbf0cd2378cddc605fb2546cf34`, os callbacks
`show outline.entry` que devolvem `context [#it.page()]` ou
`context [#it.prefix()]` são executados somente pela realização P1406, depois
da expansão contextual original. O layout ignora o `ContextBlock` ainda não
expandido e o texto desaparece. O vanila ratificado produz respectivamente
`1` e `1.`. A tentativa falsificadora P1411 de fazer
`Content::map_content` descer em `Outline.realized_body` não altera o resultado:
o corpo ainda não existe quando a expansão original percorre a árvore.

### Decisão proposta

No ciclo paginado, depois de cada realização de outlines e antes do layout do
candidato, L3 deve descobrir os `ContextBlock` recém-produzidos na árvore
realizada, atribuir-lhes Locations causais por introspecção dessa árvore e
expandir seus callbacks com o snapshot corrente de páginas/posições. O conteúdo
expandido, nunca o candidato com blocos pendentes, segue para layout.

Produtores que já existiam na entrada da realização já foram expandidos pela
fase contextual original. A passagem pós-realização identifica esses mesmos
objetos causais e remove somente seus marcadores da vista temporária antes de
descobrir produtores novos; seus resultados irmãos permanecem intactos. Não
reexecutar um produtor antigo, não comparar apenas id numérico e não duplicar
sua contribuição ou seus warnings.

A repetição usa o orçamento de convergência já limitado da compilação e sempre
reconstrói o candidato a partir da entrada não transformada; não acumula bodies,
callbacks, warnings ou eventos de introspecção. O primeiro layout pode fornecer
o snapshot provisório necessário a `page()`, mas somente um candidato estável
alcança shaping/export. HTML continua fora da realização P1406. Matching e
execução de `outline.entry` permanecem em L1; L3 somente ordena as fases e usa
as fachadas existentes de introspecção e expansão contextual.

Esta inclusão executa callbacks contextuais após a realização P1406 e altera a
fronteira introspecção/realização/contexto/layout. Confirmação humana explícita
recebida em 2026-09-15; a mudança de fase está autorizada conforme ADR-0127.

## P1415 — realização de `par.line(numbering: function)` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `0a7de3dd9`, `layout_with_realized_math_callbacks` já executa o
protocolo puro `Pending → realizar em L3 → repetir`, com teto oito, mas só para
`cancel.angle`. O vanila chama o callback de `par.line` depois de conhecer cada
linha física e passa um único inteiro; a fixture binária falha com
`missing argument: b`. Eval antecipado e layout com Engine estão refutados.

### Decisão proposta

Substituir a coordenação exclusivamente math por coordenação combinada dos
stores independentes de math e `par.line`. Cada candidato chama a entry point
L1 combinada. Ao receber requests de linha, L3 constrói `Engine` com o
`StyleChain` e span capturados, chama `apply_func` com exatamente
`[Value::Int(number)]` e usa a conversão canônica `value_to_content`. Erro da
closure, aridade ou conversão aborta sem documento exportável; warnings da
chamada são preservados uma vez e em ordem.

Resoluções completas formam store validada e uma nova passagem. Transcript novo,
store parcialmente consumido ou requests de qualquer família ainda pendentes
continuam o ciclo sob teto oito. Somente `Complete(PagedDocument)` alcança
realização P1159, shaping ou export. Antes de cada relayout externo de page
numbering, os stores são reconstruídos para o novo candidato; não são cacheados
entre compilações.

Math e linha partilham somente a orquestração e o orçamento: uma família não
executa a outra nem altera sua chave. HTML permanece fora do layout paginado.
Não mover `Engine` para L1, não reentrar eval a partir do Cursor e não devolver
documento provisório. A nova fase e sua superfície pública aguardam confirmação
humana explícita.
