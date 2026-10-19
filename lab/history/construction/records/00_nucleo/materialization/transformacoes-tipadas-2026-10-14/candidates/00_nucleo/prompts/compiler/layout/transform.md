# L0 — transform
Hash do Código: 15e7e2b0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:cbd8b56af347a35e867b2c4695581721d5983ec24ee7bb022e9f9987500d40f3

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/transform.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/transform.rs:26`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Apply the existing rotate/scale/skew/move matrix to a measured subframe around the resolved pivot and preserve Group/local-child representation, inline extent, reflow behavior and nested fixup paths. Bounds and pivot calculation ignore only direct private HTML markers; keep those markers among the transformed children. Centered reconstruction retains matrix.tx/ty in addition to pivot terms so move translations reach causal positions. Do not derive transformed bounds from marker coordinates, discard the point before Group composition or apply the parent origin twice. `00_nucleo/prompts/compiler/layout/html_frame.md` owns extraction/removal, not this unit.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/transform.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar posição, avanço já consumido, estilo e origem do TextSeparator sem criar tinta, baseline ou novo avanço; manter ordem dos filhos.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

A segunda medição da AABB inline, após rebase dos sub_items, também exclui TextSeparator antes de acumular max_x. Transportar carrier não pode ampliar orig_w_exact nem deslocar o pivô.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Referencial de pintura — transporte aprovado T1b — 2026-10-14

### Medição anterior à decisão

O manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
foi aprovado pelo humano para T1–T3; recibo em
`00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A evidência bilateral de Parent e controles Solid está no contrato aprovado;
ela não é substituída por inspeção estrutural. A rodada
`00_nucleo/diagnosticos/paint-root-2026-10-14/before/results.json.gz` conserva
fontes, estado sujo, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, UTC,
diffstat e hashes dos binários; cobre contêineres, Place e Rotate com Solid
e Gradient Parent. O inventário before-source/syntax-nodes registra neste
consumer as construções/desestruturações antes da edição: [149, 303].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

Migrar construções e padrões de Group para o campo paint_root aprovado. Grupos técnicos novos e fixtures legadas são false; reconstruções transportam o valor recebido, sem inferir raiz a partir de dimensão, clipping ou transformação. Preservar os demais campos e o comportamento de desenho/recursos. Exportadores ainda não usam este metadado para patterns; essa integração é T2 e não está certificada aqui.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.

## Transformações tipadas — contrato substitutivo

## Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/{before,results}.json.gz` e
`controls/results.json`, `binding-controls/results.json`, `context-controls/results.json`
preservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC,
diffstat, fontes, comandos e hashes de binários. Pin 9f2b6e8715237cb086899a42873660fe744622e8.
TransformElem em entities/elements/transform.rs:19 armazena apenas matrix/body;
stdlib/transforms.rs:19/51/91/191 descarta argumentos e reduz valores à matriz;
layout/transform.rs calcula pivô central e usa has_orphaned como desvio de layout.
No pin, typst-library/src/layout/transform.rs conserva os quatro elementos e
typst-layout/src/transforms.rs resolve estilos, região, corpo, origin e reflow.

Sondas: fields vazio versus body/argumentos; func falso versus função nativa;
rotate(0deg)[X] == scale(100%)[X] true versus false; defaults explícitos colapsam.
Origin/reflow são ignorados ou rejeitados; scale(x:150%) escala também y no
Tekt, enquanto y permanece100% no pin; move perde percentual/em. Campos ausentes
devem continuar desconhecidos lexicalmente. set rotate(23deg) é válido, mas
angle nomeado não é; getters contextuais refletem estilo no pin. Os primeiros
controles contextuais rotularam o wrapper e são inválidos para medir value;
context-controls corrige a posição da label. HTML pinado avisa e elimina o body;
Tekt rejeita a operação, de acordo com seu contrato HTML estrito.

Semântica, sintaxe, morfologia, posições/avanços e diagnósticos são linguagem;
enum versus structs upstream e número de passos de medição são mecânica.
A hipótese é refutável por campos/presença que ainda colapsem, estilo assado,
origem incorreta, erro perdido, reflow falso alterando avanço ou body descartado.

Esta seção substitui cláusulas anteriores conflitantes somente para a família
Move/Rotate/Scale/Skew; demais responsabilidades permanecem.

## Resolução e geometria

Consumir TransformOperation explicitamente. Matrix mantém o comportamento
genérico anterior por um braço próprio; não usar essa compatibilidade como
fallback das quatro variantes nativas. Conservar assinatura privada, topologia
estática e representação Group com paint_root false para envelope técnico.

As variantes nativas medem frame lógico do body com tamanho/baseline reais,
incluindo espaços, boxes e conteúdo vazio. Não usar apenas bbox da tinta ou
arestas da fonte como substituto universal da medida. Ignorar marcadores privados
de HTML no cálculo sem removê-los do transporte. Preservar caminhos dos fixups,
separadores, Source, TextReplacement, recursos e collectors.

Move resolve Rel<Length> com a região do uso e em com o estilo de fonte atual;
aplica translação visual e mantém tamanho/baseline/avanço. Rotate/Skew resolvem
angles/defaults, e Scale resolve Ratio diretamente ou Length contra a dimensão
lógica medida do body; um eixo Auto acompanha a escala do outro. Dois Auto
produzem `x and y cannot both be auto` na resolução, inclusive via set. Preservar
escala zero e sinais negativos sem substituí-los por identidade. Medir casos
degenerados antes de escolher diagnósticos; não inventar limites por conveniência.

Origin resolve alinhamento por eixo sobre o frame lógico (topo do frame, não
baseline usada como origem arbitrária): T(pivot)*M*T(-pivot). Start/end usam
direção efetiva. Um alinhamento parcial faz fold com herdado/default. A posição
do parent é aplicada uma vez. Não inferir reflow pela presença de alinhamentos
órfãos, tipo do body ou coeficientes da matriz.

Reflow false mantém a caixa lógica e baseline do body; somente a tinta muda,
inclusive em contexto inline, block, place e nested. Reflow true transforma os
quatro cantos do frame, desloca pela mínima coordenada e ajusta caixa/avanço.
Scale sem flip vertical preserva baseline transformada; skew conserva a linha
transformada da baseline no meio do frame; rotate e flip vertical seguem os
observáveis medidos do pin sem fingir uma baseline de fonte para qualquer body.
Transformações não perdem labels, texto, clipping, recursos ou ordem de leitura.

Resolução de units passa da extração prematura em eval para layout; avaliação
do body, efeitos e diagnósticos não são duplicados por medições. Sub-layout e
tentativas restauram cursores, coletores e contexto; nenhuma mutação global.
O mecanismo de medição pode diferir do pin desde que preserve os observáveis.
Exporters não avaliam argumentos nem precisam conhecer TransformOperation.

## Aceitação

Geometria causal com controles default/origin parcial/extremos, reflow false/true,
Scale x somente, fator+override, absolute/auto, move percent/em, skew e nested.
Incluir texto/matemática inline e baseline, following content, quebra de página,
RTL, vazio/zero/flip, clips, gradientes Parent e recursos. Validar fonte→PDF/SVG/PNG
e campos antes/depois da realização; não alinhar imagens para esconder posição.
HTML fora de ilhas permanece no diagnóstico estrito do owner HTML, sem drops.
