# L0 — tiling
Hash do Código: bec49086

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee
- 00_nucleo/prompts/_nuclei/tiling-placement.toml sha256:9c88c9c3bde8b005623fa70b5213400655a9a741b1a140b6b76c7225fc16edb4

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/tiling.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/tiling.rs:15`, `01_core/src/compiler/layout/tiling.rs:124`, `01_core/src/compiler/layout/tiling.rs:144`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Materialize only supported Content-backed tiling fills into existing Groups. Reject invalid/nonpositive/nonfinite target, explicit cell dimensions, pitches, offsets or angle without a loop; None means unsupported, not a successful transparent paint. Lay out the body once in an isolated region, derive auto cell dimensions from its actual width/height, and compute pitch as cell size plus resolved spacing. Use the pinned tiling-placement geometry contract. Shapes' Auto/Itself phase is local; explicit Parent cancels the supplied shape-origin translation in the target basis, separately from grid rotation. Preserve cell clipping and the enclosing shape clip, clone the already-laid-out cell in the bounded lattice (current maximum 10000 cells), and keep default/unsupported text-parent cases outside this owner. Do not execute body per copy, rasterize or change public Paint types. Shape owns invocation and stroke behavior.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.


## Operações numéricas puras — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/layout/tiling.rs` nas linhas 74 contém as chamadas sinalizadas como disallowed_methods pelo Clippy vigente. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. `lab/results/construction/diagnostics/geometria-numerica-ci-2026-10-13/before.json.gz` conserva fonte, owner, configuração, libm, compilador, UTC, status e diffstat. A sonda host-math compara os backends antes da escolha: 4968 casos, 69 diferenças de até 1 ULP em trigonometria/atan2, sem mudança de classe numérica ou sinal de zero; potências e hypot coincidem na amostra. É medição deste host, não prova universal nem nova paridade contra vanilla.

### Decisão e aceitação

Historicamente, o raio diagonal usava libm::hypot (0.2.11 pinada). A fórmula por raio foi substituída pelo inverso dos cantos em Célula geométrica e cobertura da repetição; não é obrigação vigente de algoritmo. Não substituir por sqrt(width*width+height*height), que perde robustez em extremos. Preservar validações de finitude/positividade, contagem máxima 10000, ordem das transformações e layout único do body.

Mudança interna em fluxo contínuo ADR-0127, sem contrato público novo, default ou fase. Preservar testes de geometria/linguagem existentes e comparar os exemplos SVG medidos antes/depois, declarando diferenças. A comparação geométrica não certifica todos os casos ou plataformas. Validar build, todos os targets, core, Clippy e linhagem. Não suprimir o lint nem substituir aceitação por identidade de bytes do compilador.


## Célula geométrica e cobertura da repetição — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-cobertura-2026-10-14/before/results.json.gz`
conserva fontes/owners, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado com diffstat, binários e UTC. No canvas inteiro
Poppler 144 dpi: empty pinta 3840 pixels Tekt/0 vanilla; empty-dense
40000/0; dense 40000/10000; self e offset-control 0/960. O pin vanilla
é 9f2b6e8715237cb086899a42873660fe744622e8. O controle large-offset
não pinta em nenhum binário; isto não legitima perder periodicidade.
A fonte deste owner:35 usa a origem de baseline do helper; :47 usa
last_sub_frame_width, que é apenas avanço final de cursor (sub_frame.rs:350),
não a largura de um shape em bloco; :57 rejeita conteúdo sem itens e :76
usa um quadrado centrado na origem, não o inverso do recorte transformado.
São hipóteses causais a discriminar por RED e recoleta, não paridade já provada.

### Decisão e responsabilidade exclusiva

Conservar a materialização privada na fase layout e os FrameItems existentes.
Usar o helper isolado com origem geométrica; texto adquire sua própria baseline.
Na dimensão automática, medir a largura dos itens retornados com o helper
line_content_right existente e conservar também o avanço final; não executar
body novamente. Altura usa a extensão devolvida pelo sub-layout. Dimensões
explícitas inválidas são rejeitadas antes de executar o body.

Com tamanho/pitch/offset/ângulo válidos, body sem itens é um preenchimento
transparente materializado com sucesso (Some grupo vazio), nunca None/fallback
preto. Não construir uma grelha vazia só para atingir o limite de células.

A grelha cobre o inverso dos quatro cantos do recorte alvo pela transformação
de placement, incluindo o tamanho da célula nas bordas. Reduzir deslocamentos
periódicos pelo pitch antes de combiná-los, evitando saturação de índices por
offsets grandes. Isto conserva a fase periódica; a ordem do offset é retificada abaixo pelo núcleo tiling-placement;
não ratifica todo o referencial parent/rotação como paridade bilateral.
Verificar finitude e spans antes de conversões/loops e manter o máximo de
10000 células efetivamente candidatas. O limite residual continua Unknown,
não sucesso de render nem prova de produção; não elevá-lo para esconder erro.

### Aceitação e limites

RED→GREEN para transparência, geometria da célula, size:auto de shape em
bloco, padrão denso abaixo do limite real, cobertura parent deslocado/rotacionado
e periodicidade de offset grande. Controles preservam recorte de cada célula,
ordem de pintura, layout único e rejeição de entradas inválidas. Recolher o
corpus bilateral e validar build, workspace, formato e linhagem. Stroke ainda
não transporta corpo materializado ao exporter; esta correção não o encerra.
Sem campo/assinatura pública nova, default ou mudança de fase (ADR-0127).


## Origem da rotação de tiling — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-transformacao-2026-10-14/before/results.json.gz`
conserva fontes/owners, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado, diffstat, UTC e binários. Referência pinada
9f2b6e8715237cb086899a42873660fe744622e8. Para angle90 e offset(3pt,5pt),
o SVG vanilla emite matrix(0 1 -1 0 3 5); o Tekt emite (0 1 -1 0 -5 3).
No pin, visualize/tiling.rs:173-174 descreve rotação em torno do offset;
:375-377 concatena translate(offset) com rotate(angle), e
layout/transform.rs:334-343 calcula T*R. O texto histórico "offset antes de
angle" foi interpretado incorretamente como rotação do próprio offset.
Semântica geométrica é obrigação de linguagem; ordem de chamadas Rust não é.

before/render-comparison.json.gz discrimina targets: em quarter sem offset,
Tekt PDF e SVG coincidem exatamente com vanilla SVG, enquanto vanilla PDF
difere em 414 pixels >16. Não copiar essa diferença do PDF como requisito.
Em quarter-offset, os SVGs diferem em 1764 pixels >16. Rasterizações usam
canvas inteiro 240×192, branco, librsvg e Poppler, sem recortes/tolerâncias
criadas para aprovar. A evidência não certifica parent composto nem todo PDF.

### Responsabilidade deste owner

Materializar o contrato do núcleo tiling-placement. placement_transform compõe
translate(offset - origem fornecida do shape para Parent) * rotate(angle).
Self/Auto não subtraem a origem. A origem do pai não é rotacionada junto com
a grade. Antes de gerar a grelha, transformar a translação para a base inversa
da rotação, reduzir pelos pitches, retornar à base de placement e manter
os coeficientes lineares. Validar finitude antes de normalizar ou iterar.
Conservar a cobertura pelo inverso dos cantos, recortes, tamanho auto, corpo
vazio transparente, layout único e limite 10000 do reparo anterior.
Não confundir a origem local disponível com o referencial composto do pai:
Place/aninhamentos continuam abertos até transporte e medição próprios.
O teste histórico que esperava offset girado passa a exigir pivô invariável;
novos RED discriminam Parent com rotação e periodicidade na base correta.
Recolher PDF e SVG e revalidar workspace, build e linhagem.

Correção interna/de paridade ADR-0127 em fluxo contínuo: L0 primeiro,
RED→GREEN e resselo restrito. Nenhuma mudança pública ou nova fase.

## Referencial de pintura — transporte aprovado T1b — 2026-10-14

### Medição anterior à decisão

O manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
foi aprovado pelo humano para T1–T3; recibo em
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A evidência bilateral de Parent e controles Solid está no contrato aprovado;
ela não é substituída por inspeção estrutural. A rodada
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` conserva
fontes, estado sujo, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, UTC,
diffstat e hashes dos binários; cobre contêineres, Place e Rotate com Solid
e Gradient Parent. O inventário before-source/syntax-nodes registra neste
consumer as construções/desestruturações antes da edição: [85, 130, 147, 153].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

Migrar construções e padrões de Group para o campo paint_root aprovado. Grupos técnicos novos e fixtures legadas são false; reconstruções transportam o valor recebido, sem inferir raiz a partir de dimensão, clipping ou transformação. Preservar os demais campos e o comportamento de desenho/recursos. Exportadores ainda não usam este metadado para patterns; essa integração é T2 e não está certificada aqui. Cópias legadas de células são raízes locais; máscaras externas e o grupo da matriz de repetição são técnicos. O limite/clonagem legados permanecem pendentes até o transporte nativo T1c/T2, sem crédito de correção por este metadado.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.


## Preparação única de célula — transição T1c aprovada — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-preparacao-2026-10-14/before.json.gz` conserva
HEAD, UTC, working tree, diffstat e bytes/hashes das fontes e owners.
Neste consumer:35–81 o layout do body, validação de célula e resolução de
placement estão misturados com a cobertura do alvo; :124–145 limita a grade
e clona os itens. `frame_paint.rs:40–48` já declara TilingFrame aprovado,
mas não recebe células preparadas. `sub_frame.rs:358` fornece a largura
causal, distinta de cursor terminal; `checkpoint.rs:192,311` salva/restaura
runtime. Na fonte vanilla pinada 9f2b6e871, visualize/tiling.rs:235–299
valida size, spacing, offset e angle antes/depois do layout e distingue
size:auto vazio. As mensagens abaixo provêm dessa fonte onde coincidem;
pitch e corpos Rust legados são decisões explícitas Tekt, não inferência
de intenção upstream. O manifesto aprovado é d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d.

### Contrato de preparação e transição

Extrair a preparação para função privada que retorna SourceResult<TilingFrame>.
Ela não recebe dimensão do alvo e não enumera repetições. Retém apenas uma
célula com tamanho físico, spacing assinado, offset resolvido contra pitch,
angle e RelativeTo (Auto/Itself → Self_; Parent preservado). Lay out Content
uma vez em origem geométrica. Auto usa largura causal e extensão dos itens,
conservando o avanço terminal legado; height usa extensão do subframe.
O próprio frame é raiz implícita, independentemente dos grupos do body.
Estabelecer parent_bbox local da célula durante layout, restaurando o pai.
Isolar estado de fluxo/runtime pelo checkpoint existente; devolver erros
causais produzidos pelo body, sem consumir os erros anteriores do documento.
Não duplicar nem promover labels/posição/counters da célula à página.
A célula continua contendo seus itens visuais; filtros de anotações e
coletores especializados dos exporters são responsabilidade de T2.

Antes de executar body, rejeitar size explícito não positivo/não finito com
`tile size must be non-zero and non-infinite`, spacing não finito com
`tile spacing must be finite`, offset font-relative com
`tile offset must not be font-relative`, componentes não finitos ou offset
resolvido não finito com `tile offset must be finite`, angle não finito com
`tile angle must be finite`. Rejeitar pitch não positivo/não finito com
`tile pitch must be positive and finite`; se size explícito, checar antes
do body. Auto vazio/zero usa `tile size must be non-zero` e hint
`try setting the size manually`; auto não finito usa a mensagem de size
não finito. Célula vazia com size explícito válido é transparente e válida.
Span detached é a fronteira atual do Tiling Rust sem span; não inventar
localização de fonte. O helper devolve SourceResult, nunca cor representativa.

Corpos legados Image tornam-se Content::Image e usam o mesmo layout. Color e
Gradient exigem size explícito: criar shape retangular com paint original,
sem consultar fallback; sem size, diagnóstico
`tile size must be explicit for a color or gradient body`. A ponte é L1,
não responsabilidade do exporter.

Transição explícita: materialize_fill usa esta preparação e mantém apenas
como adaptador temporário a grade/clip legados. Erros entram em layout_errors,
portanto não são sucesso silencioso; o limite 10000 ainda é incompletude.
A troca de Shape.fill/stroke, remoção deste adaptador, preparação de stroke e
consumidores nativos completam T1c/T2 já aprovados. Este passo NÃO os substitui
nem certifica tiling de produção. A cláusula antiga Content-only é supersedida
pela ponte explícita; nenhuma conversão From<Paint> é autorizada.

### Aceitação

RED→GREEN de célula única e origem, auto/explicit, empty, spacing assinado,
offset relativo, rotação/parent, validação antes de executar body, isolamento
de fluxo/runtime e propagação de erro, pontes Image/Color/Gradient sem redução.
Preservar os controles geométricos da grade temporária e revalidar core/build/
linhagem; suíte e targets completos continuam gates de integração. Estrutura
de célula é evidência de transporte limitado, não prova de paridade visual.


### Isolamento de transação externa

A inspeção de `checkpoint.rs:409–415` mostrou que restore_flow_suffix_checkpoint
limpa a transação/replay/depth do sufixo de fluxo. `flow_suffix.rs:41–43`
incrementa/decrementa esse depth em torno do layout; usar o checkpoint durante
uma célula sem preservar seu envelope apagaria a transação do pai. O RED
`tiling-preparacao-2026-10-14/isolation-red.log` refuta a primeira extração.
Este owner deve suspender os campos privados de transação do pai (pending,
active, rejected, checkpoint, replay, depth), preparar a célula com envelope
local neutro e restaurá-los integralmente depois do checkpoint, também quando
o body produz diagnóstico. Não alterar o contrato do checkpoint compartilhado.
Para size:auto, parent_bbox legado fica None durante a preparação; a raiz
implícita do TilingFrame será consumida por T2 após conhecer a dimensão final.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Preparar Paint/Stroke por helpers privados de layout: Solid/Gradient mantêm todos os dados; Tiling usa prepare_cell e Arc, sem grade de cópias. FrameStroke preserva todos os campos geométricos e presença. Devolver SourceResult e registrar erros causais ao emitir, sem trocar por cor. Remover materialize_fill e seu limite de repetição do caminho produtivo. Retirar testes mecânicos da grade e substituir por testes de célula/placement/transporte; conservar refutadores visuais no corpus nativo. A preparação e o isolamento já especificados permanecem.

### Aceitação e continuidade

Aplicar o núcleo pinado. A ratificação supersede cláusulas antigas que
aguardavam aprovação ou exigiam Paint/Stroke declarativo dentro de Shape,
somente no âmbito do contrato aprovado. Preservar os demais comportamentos
vigentes desta unidade. L0 primeiro e testes RED→GREEN; migração explícita de
fixtures e consumidores, sem conversão total From<Paint>, sem PartialEq nos
carriers e sem fallback sólido para obter build verde. Não elevar o limite
antigo nem transferir layout para L3. Validar transporte fill/stroke, recursos,
clipping, Parent afim, alvos nativos, controles Solid/Gradient, semântica de
página e toda a suíte/build/doc/fmt/linhagem. T1c, T2 e T3 só estão concluídos
quando integrados e verificados; estado intermediário quebrado ou parcial
não é certificação de produção. T4 e publicação continuam fora deste pacote.
