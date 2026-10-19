# L0 — boxed
Hash do Código: e1cbb84f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:42175932c75f37ed19205d1d82526969999e18fba8fed16f1b1dcff6e418bc0f
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:ff71dfc6648d6deb4b2b8054e3d58a93777d17837129f0f1b1e7e138ffb6943c
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/boxed.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/boxed.rs:17`, `01_core/src/compiler/layout/boxed.rs:387`, `01_core/src/compiler/layout/boxed.rs:415`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own inline Box geometry: explicit dimensions, insets/outsets, baseline shift, clipping, radius, fill/stroke and parent bbox. Measure through the existing constrained helper but obtain natural text baseline/inline extents from composition. A body with a text baseline uses inset plus actual ascent; a non-text body retains its box-bottom fallback. The baseline may lie outside the box; aggregate nonnegative ascent/descent without clipping its meaning.

Isolate the parent's pending line, horizontal fractions, natural widths and repeats while composing the body, then restore and append body-local boundaries rebased by the parent prefix once. A fractional box registers its boundary only after body emission and records its provisional natural width; `00_nucleo/prompts/core/compiler/layout/line_commit.md` replaces that occupancy by the allocation rather than adding both. Child repeats remain associated with their real boundaries.

Preserve the existing paragraph-to-box calibration branch and its size-scaled values at the source evidence anchor. It is an empirical compatibility behavior, not a vanilla-derived general typography formula; replacement requires separate evidence. Ignore only direct private HTML markers when deciding an empty line. Restore outer bbox/width; the composition correction below supersedes preservation of defective shape/group emission for non-fractional boxes.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/boxed.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/core/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar posição, avanço já consumido, estilo e origem do TextSeparator sem criar tinta, baseline ou novo avanço; manter ordem dos filhos.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/core/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Continuidade lexical do tracking — 2026-09-29

### Medição anterior à decisão

`lab/results/construction/diagnostics/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` e `summary.json` registram 30 fontes/90 PDFs, comandos, fontes e estado exato em initial.json e before-pdf/provenance.json, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Escopos iguais repetidos acumulam cerca de 0.4pt e aninhados 0.8pt; wrap positivo muda a linha. `text_tracking.rs:50` compara apenas TextStyle resolvido. Vanilla `inline/collect.rs:287-295` conserva segmentos de cadeias distintas; `inline/shaping.rs:1269-1295` soma tracking apenas dentro de cada run. Isto mede fronteiras textuais observáveis, não exige copiar PartialEq ou representação da chain. Hide/Box também são fronteiras: hide.rs:21-24 e boxed.rs:157-208 substituem/restauram buffers sem preservar essa distinção.

### Contrato interno da correção

Depois de reunir body e prefixo da caixa inline, interromper a continuidade via State::interrupt. A caixa é barreira de objeto mesmo quando seus itens são materializados como folhas textuais transparentes. Não somar intervalo adicional ao exterior nem alterar avanço nominal, dimensões, clipping, fr/repeats ou SHY.

Correção interna de paridade ADR-0127 em fluxo contínuo. Aceitação RED→GREEN, controles bilaterais de texto/posições/linhas e preservação dos controles de tracking uniforme; build e lint sem erros, resselo somente destes owners. Demais limites abertos mantidos.


## Caminhos residuais internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/layout/boxed.rs` nas linhas 263 contém os itens sem consumo/visibilidade sinalizados pelo compilador. A pesquisa de usos e os dispatchers ativos estão registrados antes da edição em `lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`, com fontes, L0, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status e diffstat. Isto não é nova medição bilateral de paridade.

### Decisão e aceitação

Eliminar somente a soma local box_h sem consumidor no ramo de altura explícita. Manter h_pt, inner_height do Group, fallback de conteúdo, inset, clip e contribuições à linha.

Correção interna em fluxo contínuo ADR-0127, sem nova assinatura pública, default de produto ou fase. Preservar os resultados de linguagem e as expectativas dos testes. Revalidar todos os targets, suíte core, build, Clippy e linhagem; não suprimir warnings para substituir a correção.

## Composição geométrica da caixa — 2026-10-13

### Medição anterior à decisão

box-dimensoes-2026-10-13/probe.json.gz e containers-clip-2026-10-13
preservam sondas fonte→PDF, pin vanilla, binários, fontes, HEAD/working tree,
status/diffstat e UTC. boxed.rs:99-107 omite o limite direito ao instalar
width; :164-202 coleta só a cauda da linha e translada novamente formas
comitadas; :219-225 soma inset/outset à largura explícita; :261-329 clipa
apenas por overflow vertical e ignora radius; :337-383 usa fundo separado
da linha e altura proxy. O texto Q após forma fica em outra baseline;
inset/outset deslocam Q mesmo com tamanho explícito fixado. O controle
da caixa vazia sem inset/outset coincide. Fonte normativa medida:
upstream container.rs:96-127, inline/box.rs:26-50, flow/block.rs:255-295.
A composição interna é mecânica; tamanho, baseline e recorte são geometria
observável. Snapshot inicial desta correção: box-composicao-2026-10-13/
before.json.gz.

### Contrato proprietário

Caixas não fracionárias compõem o corpo completo uma vez pelo subframe
isolado de origem geométrica. Capturar tanto itens comitados quanto a
cauda; o cursor/linha e pendentes do pai não pertencem ao corpo. Largura e
altura explícitas são dimensões totais antes de outset; inset reduz a região
interior e posiciona o corpo dentro dela. Em auto, a dimensão natural é
a união/avanço real do corpo mais inset. Outset só expande pintura, não
avanço nem ascent/descent da linha. Não impor medidas de probe como fórmula.

A primeira baseline textual efetiva do corpo, incluindo wrappers e
transformações, determina a baseline natural com inset superior; sem
baseline, usar a base da caixa. Aplicar o shift existente sem limitar a
baseline ao interior e informar extensões não negativas ao pai. Fundo,
corpo e recorte acompanham a mesma caixa inline quando a linha é deslocada.
O frame lógico tem as dimensões de layout; a região pintada pode excedê-lo.
Clip sempre restringe os filhos à região de pintura declarada, em ambos os
eixos e com height auto; a máscara usa os mesmos raios resolvidos. Fundo
e stroke não são recortados pela máscara do corpo. Insets e outsets são
translações locais explícitas, sem refletir ou somar a origem externa duas
vezes. Preservar wrappers semânticos, links, imagens e parent bbox local;
restaurar bbox externa após compor o corpo. A política existente de só
instalar bbox própria quando ambos os tamanhos são explícitos permanece
limitada, não é declarada paridade completa de gradientes relativos.

Reinserir segmentos de decoração com a origem do corpo. Resolver fixups
locais quando as dimensões da caixa são conhecidas, sem indexar a linha
pai com paths relativos ao filho. Conservar a calibração empírica de
parágrafo→box até medição própria. Sem contrato público ou fase novos.

### Etapas e limites explícitos

A composição permanece **incompleta** enquanto a etapa B de
`lab/history/construction/records/00_nucleo/materialization/box-frame-composition-2026-10-13.md` não migrar
caixas width_fraction. Essa largura só é conhecida no fechamento da linha;
seu caminho atual de índices de frações/repeats permanece separado e
identificado, sem tratar como resolvido o clip/inset/outset fracionário.
A etapa A trata a composição não fracionária integral, sem limitar corpos
a texto/shape de probes. Esta divisão não reduz o objetivo final.

Exigir testes RED→GREEN de dimensões, inset/outset, coleta de formas e texto,
baseline do prefixo/sufixo, clip horizontal/height auto/radius e controle
sem clip. Medir novamente PDFs e preservar semântica das suites. Testes
antigos que exigem representação flat ou ausência de máscara quando o corpo
cabe devem medir geometria/conteúdo, não conservar a mecânica defeituosa.

### Borda interna do recorte — refutação da primeira composição

Após os oito controles sem stroke coincidirem pixel a pixel, as quatro
sondas stroke/before/results.json mostram a tinta do corpo cobrindo metade
do stroke de 2pt. Na caixa 20×14pt sem radius/outset, a tinta ocupa 1120
pixels contra 864 no controle pinado; a borda preta conserva bounds externos
mas perde área. Upstream shapes.rs:617-662 resolve clip_rect pela borda
interna do stroke, enquanto o candidato usa o contorno central do fundo.
Fontes/L0 anteriores e gates estão em first-validation.json.gz.

Para o stroke uniforme já representado pela entidade, deslocar o clip
para dentro por metade da espessura não negativa, reduzindo ambos os
tamanhos em sua espessura (sem negativos). A semântica dos raios externos, centrais e internos é qualificada abaixo.
Outset continua expandindo somente a pintura. A posição do corpo não muda;
sua translação relativa ao clip compensa o novo início da máscara.
Exigir RED→GREEN e sondas com/sem radius/outset, conservar o traço completo
e o tamanho lógico. Nenhuma nova representação pública de stroke.

A regressão proprietária também compara o resultado de medição restrita
com as dimensões totais da caixa explícita, para impedir que pré-layout
e composição discordem sobre inset/overflow. A fórmula de medição pertence
exclusivamente a `00_nucleo/prompts/core/compiler/layout/measurement.md`.

### Raios com stroke — refutação da primeira redução

Medição anterior: stroke/after/results.json ainda distingue os casos rounded
e rounded-outset do oráculo; rect/outset já coincidem. Fonte pinada
lab/typst-original/crates/typst-layout/src/shapes.rs:1163-1178 define o raio
resolvido como externo; o central subtrai meia espessura e o interno uma
espessura inteira. A primeira correção confundiu raio externo com central.
Snapshot da implementação e da suíte: radius-legacy-before.json.gz.

Resolver cada raio externo, limitado a metade da menor dimensão de pintura
mais meia espessura; derivar raio central e interno pela distância entre
contornos concêntricos (todos não negativos). ShapeKind do fundo recebe o
raio central; o clip recebe o interno. Manter posições, dimensões e origem
do corpo. Teste exige raio interno 2pt para raio externo 4pt/stroke 2pt;
não conservar 3pt como esperado da implementação anterior. Correção interna.

## Baseline inicial de caixa sem strut fictício — 2026-10-14

### Medição anterior à decisão

box-baseline-inicial-2026-10-14/before/results.json.gz registra doze fontes
contra o vanilla pinado, binários, estado sujo, HEAD e UTC. Caixa 8×6pt
sozinha começa em y pixel 22 contra 20 a 144dpi; com Q adjacente ambos
começam em 22. Shift baseline 10pt sem texto revela a mesma contribuição
fictícia. boxed.rs:21-38 instala fonte antes de conhecer a caixa;
line_commit.rs:421-437 e sub_frame.rs:292-300 confundem zero com ausência
e só consideram texto em condições parciais. Fonte pinada inline/line.rs:
559-568 acumula top/bottom dos frames realmente presentes. A posição é
observável; Option e buffers são mecânica. Snapshot fonte/L0: before.json.gz.
Hipótese: diferir a baseline da primeira Box e combinar somente conteúdo
real elimina o deslocamento. Refutar se controles de texto, transformações,
Raw, HTML causal ou caixas aninhadas passarem a se deslocar.

### Contrato deste owner

Na primeira linha de origem geométrica ainda pendente, caixa não fracionária não instala ascent de fonte antecipado. Compor primeiro o corpo isolado, determinar sua baseline e instalar max(baseline,0) como ascent inicial (inclusive zero); o topo da caixa é origem + ascent instalado - baseline. Preservar o caminho fracionário, a calibração parágrafo→box e as linhas que já têm baseline. Informar ascent/descent próprios sem consumir a altura de fonte fictícia. A decisão é geométrica, não uma classificação por tipo do corpo.

Correção interna ADR-0127, sem API, default ou fase novos. L0 primeiro,
RED→GREEN, controle de ascent zero, sondas de geometria e suíte completa.
Não substituir esperados de regressão para conservar o defeito.

## Posições no referencial composto — 2026-10-14

### Medição anterior à decisão

box-contexto-2026-10-14/probe.json.gz, fontes/binários/HEAD/working tree/UTC
preservados: Box, inset2pt, prefixo e aninhamento publicam (0,0), mesmo
quando a tinta se desloca. A diferença da baseline do context direto é
uma questão separada: não copiar o vanilla apenas por diferir. mod.rs:
1131-1148/1180-1200 escreve pontos locais em runtime.positions; Box agrupa
o corpo em outro referencial; finish.rs:295-302 sela o mapa sem recompor
esses pontos. Realized escreve com or_insert; labels têm mapa paralelo.
HTML já transporta markers privados pelo frame em html_frame.rs:30-199.
O registro single-pass e sealing por sub-store são preservados, ADR-0073/
0074 §§posição/modelo híbrido; não se adota introspecção global vanilla.
A inspeção do núcleo coordinates e seus owners mostrou usos absolutos
em grid e explicitamente locais em Place/Box. A qualificação do núcleo
conserva ambos e exige transformação até a página, sem dupla origem.
Snapshot anterior: posicoes-compostas-2026-10-14/before.json.gz.
Geometria/identidade são observáveis; o carrier e seu índice são mecânica.

### Contrato deste owner

A aceitação deve incluir posições seladas de corpo localizado, inset, prefixo, aninhamento, shape e labels; não basta observar tinta. Testes consultam o documento final e verificam que carriers privados não escapam. A composição geométrica do consumer permanece a mesma nesta correção.

Correção interna ADR-0127: sem API/campo público, flag/default ou mudança
de fase. L0 primeiro; RED→GREEN de coordenadas, ausência de tinta/altura
fictícia, identidade, ordem, clones visuais e rollback; revalidar os
controles HTML, geometria, contexto/queries e suíte completa.

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
consumer as construções/desestruturações antes da edição: [299, 330, 525, 555, 576, 717].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

No caminho não fracionário, envolver somente o body (incluindo seu recorte técnico) numa raiz de pintura de tamanho total box_w/box_h e origem local zero, antes do inset. O Group externo de posição/baseline continua técnico. O backing permanece irmão anterior, fora da raiz criada para os filhos, pois usa o pai. Não mudar geometria, clipping, avanço, baseline, decoração ou política legada de gradientes. Box fracionário continua incompleto até sua composição no fechamento da linha; não marcar sua máscara provisória como raiz completa.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.


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

Nos dois caminhos de Box, preparar o stroke do backing com o Layouter antes do emit; fill Color torna-se FramePaint::Solid. Preservar radius, clip, inset/outset, pos e distinção entre raiz dos filhos e paint do próprio container. Não ampliar o tipo declarativo Box.fill.

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


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F002: 01_core/src/compiler/layout/boxed.rs:623 (first_baseline); F003: 01_core/src/compiler/layout/boxed.rs:668 (frame_items_have_text_baseline); F004: 01_core/src/compiler/layout/boxed.rs:763 (inspect). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F002 — `first_baseline`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Line`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F003 — `frame_items_have_text_baseline`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Line`, `Image`, `Shape`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F004 — `inspect`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Line`, `Glyph`, `Image`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.
A diferença entre first_baseline e frame_items_have_text_baseline é mantida: o segundo é do caminho fracionário legado preservado pelo contrato vigente e não recursa em Link. Esta manutenção não certifica nem corrige essa limitação de geometria; não tratá-la como nova decisão de paridade.


Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.

## Fronteira Par/Box e região finita — 2026-10-17

### Medição anterior à decisão

`lab/results/construction/diagnostics/par-box-fronteira-2026-10-17/before.json.gz`,
`before-measurement.json.gz`, `before-analysis.json`,
`supplement-before-measurement.json.gz` e `finite-before-measurement.json.gz`
guardam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não
commitado, status/diffstat, UTC, fontes, argv, hashes de binários e o pin
vanilla 9f2b6e8715237cb086899a42873660fe744622e8.
Em EmptyPar→Box8×7→AFTER, Tekt yMin31.108413 contra22.400001pt;
em BEFORE→EmptyPar→Box→AFTER,54.625613 contra45.917187pt.
Espaçamento30pt inverte o erro:70.225613 contra77.117188pt.
Block8 abaixo0 seguido só de Box tem altura33.1172 contra31pt.
Em alturas finitas60–63pt, AFTER desaparece no Tekt e fica visível na página2
no pin;64–70pt deixam tinta na página1 mas página terminal extra no Tekt.
Box alto45pt na primeira região30pt cria duas páginas contra uma no pin.
boxed.rs:22–42 usa constantes normalizadas e ensure_initial_baseline;
line_commit.rs:501–539 só admite antes do commit em altura reduzida;
region_progress.rs:118–133 pode avançar após tinta já comitada fora da página.
Fonte pinada inline/box.rs:35–68 publica a baseline do frame;
flow/distribute.rs:321–373 admite frames pela geometria real.
São posições, dimensões e páginas da linguagem, não exigência de copiar
buffers/algoritmos Rust ou bytes PDF. A intenção histórica não é inferida.
A hipótese de fronteira geométrica é refutada por controles inline,
margens fortes, fontes, baseline deslocada, assentamento tardio, posições,
primeiro frame alto, subframes ou reservas/rollback alterados.

### Decisão e limites

Correção interna/de paridade ADR-0127: L0 primeiro, RED→GREEN e resselo
restrito, sem API pública, novo modo/default, dependência ou fase.
A composição fracionária de Box permanece incompleta na etapa B anterior:
a testemunha width:1fr tem largura0 contra145.849609pt. Sua rota de
calibração é conservada neste reparo e não recebe crédito de paridade.
A admissão nova certifica somente página raiz finita ordinária, sem colunas,
reservas/floats, transação/checkpoint pendente ou collector externo de
decoração. Esses mecanismos conservam os protocolos próprios, não são
silenciados nem declarados corrigidos. Aceitação inclui matriz bilateral,
controles sem fronteira, finite60–70, fonte maior posterior, primeira caixa
alta e rebase de origem/contexto; suíte completa, build, Clippy, rustdoc,
linhagem bidirecional e inventário SARIF diferencial sob12GiB e swap zero.

### Responsabilidade exclusiva deste owner

Para Box não fracionário no começo de linha após cadeia de bloco, usar a
fronteira causal anterior mais a margem pendente. Se a margem anterior é
fraca de parágrafo, colapsar com par.spacing resolvido no tamanho ativo;
se é forte, usar apenas a margem anterior. Deixar a baseline pendente no
topo geométrico e limpar a cadeia/margens consumidas; layout_sized instala
o ascent real não negativo da caixa. Não consultar fontes para inventar
strut de caixa sem texto nem executar seu corpo duas vezes. Fora dessa
cadeia, preservar admissão ordinária. Box fracionário conserva a rota
anterior explicitamente incompleta. Topo e descida reais, inclusive baseline
fora da caixa, continuam agregados para o assentamento da linha inteira.

### Primeira caixa após transição regional — evidência adicional

`regions-before-measurement.json.gz` identifica explicitamente o binário CLI
anterior (SHA idêntico a before-measurement, build_source_hashes do checkpoint)
e separa os hashes da árvore observada durante a sonda. Sua conversão de
cada página PDF por pdftocairo está em regions-before-analysis.json, com
argv/SHA por página. Em EmptyPar→pagebreak→Box7, topo32.4375 contra8pt;
em texto→pagebreak→Box7,10.117188 contra8pt; Box45 após EmptyPar/pagebreak
fica em31.109375 contra8pt. Os controles pagebreak→Box7→texto e duas quebras
explícitas coincidem. Os SVGs nativos com template são evidência separada:
Tekt grava literalmente {p} e só a página inicial; não usar isso como
medição de geometria das demais páginas. Essa limitação CLI permanece aberta.
O RED regional após o primeiro reparo conserva duas páginas, mas ainda
mede topo31.182703 em vez10pt com FixedMetrics: o braço legado é alcançado
por prev_margin_is_parbreak herdado mesmo sem cadeia causal vigente.

A calibração remanescente deve ser condicionada explicitamente a Box
fracionário. Em raiz finita ordinária vazia após transição, sem linha/unidade
causal na região, sem flow witness, último flush, subframe/célula/colunas,
floats/reservas, collector externo ou transação/checkpoint, substituir a
baseline tipográfica preparada pelo topo útil geométrico e deixar a própria
caixa instalar seu ascent. Limpar a margem fraca herdada nesse ramo.
Separadores literais causais e unidades da região vigente impedem essa
recertificação; conservar suas posições. A hipótese é refutada por Box
pequeno sem texto, Box alto com texto, prefixo textual, duas quebras e
controles de linha inline. Não ampliar a certificação a regiões excluídas.


## TS1 — tamanho completo de texto (ratificado pelo humano, 2026-10-17)

### Medição anterior à decisão

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/before.json.gz`, `before-measurement.json.gz`,
`morph-before-measurement.json.gz` e `audit.json` conservam HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status/diffstat,
fontes/L0/binários/argv e o pin9f2b6e8715237cb086899a42873660fe744622e8.
Em named-em, A mede0pt contra24pt; em affine,3pt contra27pt; nested sobre18pt
mede0pt contra36pt. As seis permutações Size/Color/body são aceitas pelo pin
mas rejeitadas pelo Tekt. Set posicional18pt é ignorado (12 contra18pt).
Fonte pinada text/mod.rs:290,303,924–930,1136–1142 e args.rs:178–247
estabelece busca tipada, preferência nomeada, body restante e composição.
style_chain.rs:513–517 e constructor.rs:123 descartam em; collapse em
style_chain.rs:394–395,419–422 aplica top-wins onde tamanho precisa fold.
É tamanho/escopo observável da linguagem; a representação Rust é escolha
interna. Não inferir intenção histórica. Refutar por restauração, Raw,
heading, math, capture/show, dupes/spans, zero/negativos ou métricas divergentes.
A sonda de igualdade refutou a hipótese de comparar os valores declarados:
no pin, text(size:2em)[A] == text(size:24pt)[A] é true. StyledElem::eq no pin
content/mod.rs:779–782 compara child. Esse defeito de igualdade requer sua
própria correção; o tipo completo não é prova de fechamento desse eixo.

### Ratificação

O humano aprovou TS1 e os dois contratos públicos da proposta de manifesto
771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba.
Implementação e testes S1–S16 sob 12 GiB/swap zero; sem release/merge/push.
Resselo somente após implementação e grafo válido. RG1 continua independente.

### Responsabilidade exclusiva adicional

Somente fixtures/adapters de testes: substituir Style::Size(Pt) por Size(Length absoluto), e expectativas de StyleDelta.size por Option<Length>. Conservar os observáveis linguísticos e os contratos funcionais existentes; não trocar geometria correta por snapshots da representação nova.

## Recuo francês — aplicação física (2026-10-18)

### Medição anterior à decisão

lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz
preserva HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, árvore não commitada,
status/diffstat/UTC e2099 fontes/L0s. before.json.gz contém92 pares;
extra-before.json.gz contém26; boundary-before.json.gz contém10 e
repeat-before.json.gz contém4. Oracle bd5ebec285505889e8ff1d4f68804ab147a699f3,
binário9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9;
Tekt inicial2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095.
Em160pt/margem8/font12, hanging10 mantém a primeira linha x8 mas move
continuações x8→18; negativo-5 exige x3. RTL mantém primeira borda direita152
mas continuações vão para142. O Tekt ignora o comprimento. MLA tem default
18pt (1.5em), set ambiente3em não o troca, show-set3em exige36pt; IEEE/Plain
não adquirem recuo de hanging por esse show-set. bibliography.where(style:"mla")
não casa na sonda vanilla: conservar a decisão do seletor, não forçar override.

Block/Box com body apenas inline ignoram hanging10; Par explícito dentro
deles e Block com Parbreak aplicam. Heading/Raw/List/Quote simples não o
aplicam. Declaração parcial, zero, repetida e escopo vazio fecham fronteiras:
FIRST/MIDDLE/LAST em5.168/27.464/49.76, contra todos5.168 no Tekt. A primeira
sonda lexical usou colchetes literais: foi refutada, corrigida para #[...] e
preservada como extra-before-initial; não é evidência de escopo Styled.
Valores extremos geram tinta recortada pelo extrator: não equiparar ausência
em pdftotext a apagamento de Content. Fonte parcial24pt não muda o em do
comprimento compartilhado12pt, como partial-font registra.

Fontes pinadas: typst-realize/src/lib.rs:1061–1085,1190–1219 distinguem
fragmento/Par e interrupção; typst-layout/src/flow/collect.rs:72–134 escolhe
fluxo, inline/mod.rs:174,216–221 e collect.rs:141–147 resolvem forma;
inline/line.rs:496–503 posiciona; rules.rs:510–523 faz inset/compensação CSL.
typst-library/src/model/bibliography.rs:112–127,367 determina o default e
precedência. Isso mede língua/geometria; não exige a estrutura Rust vanilla
nem atribui intenção histórica. Refutação por fronteira perdida, em de run,
recuo duplo, reset em página, fallback greedy imposto, cópia perdida, callbacks
reexecutados ou regressões dos controles impede fechamento.

### Classificação e limites

Correção interna/de paridade ADR-0127, fluxo contínuo: sem campo ou assinatura
pública, flag/default externo novo, fase ou ordem de realização diferente.
A nova forma inicial/continuação supersede somente a limitação correspondente
de largura constante; RTL latino certificado segue as condições do núcleo,
sem BidiCursor ativo/tracking e sem inventar BidiSpan. Os demais limites
anteriores continuam. Os valores/cópia fora da página requerem inspeção dos
frames ou SVG, não igualdade de extração recortada. Não antecipar R3-T1.

### Responsabilidade exclusiva deste owner

Preservar admissão/layout de Box; verificações horizontais do slot exterior usam os limites efetivos da linha atual, sem propagar recuo genérico para body inline isolado. Subframe governa o contexto local; Box já presente na linha recebe somente a aplicação exterior pertinente.

### Aceitação

RED→GREEN antes da implementação; controles fonte→layout e bilaterais dos sinais,
fronteiras, variantes e restauração pertinentes. Não deduzir cobertura de todas
as alternativas por executar um arm agrupado: classificações nominais fechadas
do contexto inline/fluxo são intencionais e exigem testes por classe/variante.
Resselo apenas após V15/V26 válidos; testes, build, Clippy/Rustdoc/fmt, reversa
e diferencial Tekt do pin1279707, sob12GiB/High8GiB/swap0/jobs1. Sem publicação.
