# L0 — Visual bidirectional line ordering
Hash do Código: c402e136

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


**Layer:** L3
**Exclusive consumer:** `03_infra/src/layout_bidi.rs`

## Current evidence

- `03_infra/src/layout_bidi.rs:23`, `:39`, `:49`, `:128` — metrics, document/page/line passes.
- `03_infra/src/layout_bidi.rs:183`, `:305`, `:334`, `:409`, `:485` — visual ordering, space coalescing and suffix separation.
- `03_infra/src/layout_bidi.rs:646`, `:782` — controls preserving L1 wrapping.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Preserve `reorder_bidi_document(doc: PagedDocument, metrics: &dyn FontMetrics) -> PagedDocument`. Own horizontal visual ordering of top-level page items before shaping. The paged pipeline owner places this pass; paragraph direction and line breaking remain owned by L1 layout.

Cluster page items by baseline with `Y_TOLERANCE_PT=0.01`. Semantic envelopes retain their existing first-child coordinate convention. Detect an RTL base paragraph from concatenated plain Text with Unicode bidi auto direction; an LTR-base line remains unchanged. Reordering operates on plain Text only. Nontext items retain their geometry even when vector ordering changes.

For an RTL line, use `FontMetrics::text_width` for word widths and `line_content_right` for the occupied right edge. Compute nonnegative inter-item gap from occupied width minus total word widths, divided by the number of intervals. Resolve visual runs over item text ranges; explicit RTL text style supplies the RTL default level inside this ordering helper. Preserve text/style pairing, unmapped-item handling, empty-paragraph fallback and left-to-right positioning from the existing `x_min`.

Sort the line's items by increasing x. A standalone whitespace item can be coalesced as one trailing space into the preceding nonspace Text in visual order; retain the existing empty-item placeholder behavior rather than remove entries while indices are in use. Separate a terminal LTR suffix from an RTL base using L/EN/AN classes and existing ASCII punctuation handling. Preserve original trailing whitespace on the base, keep its style/y, and place the separate suffix at `x_base - suffix_width`. Pure-suffix text is not split.

L1 baselines are authoritative: never merge lines, recompute wrapping, infer paragraph boundaries from distance, estimate width from byte length or infer the region width from emitted ink. `ExplicitLinebreakBoundary` and `ParbreakBoundary` remain transparent carriers; no cross-baseline fusion consults them. Preserve page dimensions, pagination, leading and nontext positions. This pass neither supplies vertical writing nor guarantees an extractor-specific logical text order.

`00_nucleo/prompts/infra/shaper/bidi.md` owns character-level shaping runs. This owner does not duplicate its run data or shaping policies.

## Verification boundary

Existing tests cover Latin identity, Arabic/mixed visual order, real widths, whitespace and suffixes, empty text, nontext geometry and preserved wrapping/explicit breaks. Sequential extraction must not justify changing glyph order inside the shaper.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/layout_bidi.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar coordenadas do carrier sem coalescer sua origem em Text. Após reordenação de texto RTL, posicionar separadores no intervalo visual do slot precedente; não inventar carriers para gaps.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Preservar o envelope opaco nas operações que coalescem/splitam folhas Text; x/y vêm dos filhos. Não incluir texto substituto no shaper/reflow ou inferir novas quebras. Não certificar RTL completo nesta rodada.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Sufixos RTL em múltiplas linhas — 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/rtl-numeros-2026-10-05/initial.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree integral, UTC, binários e doze controles CLI. Oito grupos numerados colados coincidem; doze extraem 10 antes de 8/9 no Tekt, contra sequência no vanilla pinado; o controle com números separados coincide. Em `layout_bidi.rs:411–422,467–486`, inserir sufixo na primeira linha desloca índices das outras linhas sem atualizá-los. Isso pode separar o texto errado na linha seguinte (inferência refutada por teste causal que preserve a associação texto/baseline). A intenção histórica é separar sufixos, não inferida como autorização para perder linhas. Paridade é texto e posições; vetor e estratégia de índices são mecânica interna.

Decisão interna ADR-0127, L0 primeiro e RED→GREEN: congelar os índices originais elegíveis de todas as linhas RTL e processar inserções por índice original decrescente. Não reutilizar índices de outras linhas após mutação. Esse ordenamento protege as referências ainda não processadas independentemente de x, y ou ordem dos clusters. Os índices locais são privados e descartáveis após a separação. Preservar associação de cada base/sufixo com estilo e baseline originais, trailing whitespace e equação x_base−suffix_width existentes. Não alterar wrapping, ordem das fases, assinaturas públicas ou shaping. Corrigir somente validade das referências durante inserções; não prometer geometria universal RTL.

Aceitação: teste causal de pelo menos três baselines com sufixos e linha LTR intermediária, mais controle com ordem dos clusters distinta da ordem de índices e x não monotônico. Conferir texto por baseline e estilos, não exigir algoritmo de vetor como observável da língua. Os controles fonte→PDF devem preservar texto e baselines e medir posições contra o pin antes de fechar paridade; caso residual deve ser explicitamente registrado.

### Preservar o slot ocupado na separação — medição posterior ao reparo de índices

`rtl-numeros-2026-10-05/index-fix-verification.json.gz` mostra que reparar índices não fecha o caso longo: palavra palavra+10 começa em x=143.524530 no Tekt, contra sufixo em x=157.521600 no vanilla. O sufixo invade a palavra anterior, cuja caixa termina em x=154.025016 no vanilla. No controle de quebras explícitas, a palavra+2 inicia x=405.343870 no Tekt reparado, contra x=412.342400 no vanilla. A medição confirma a equação `x−suffix_width` aplicada sobre um slot que já incluía o sufixo, não sobre a posição de uma base nua.

Esta cláusula supersede somente a equação anterior de separação: conservar a borda esquerda original do slot colocando o sufixo em x_original e a base em x_original+suffix_width. Preservar baseline, estilo e trailing whitespace, sem reservar largura duas vezes, alterar bidi de caracteres ou reflow. Aceitação RED→GREEN por span reservado, mais posições e texto fonte→PDF medidos, sem exigir igualdade de bytes. A soma das larguras deve permanecer dentro do slot originalmente medido onde base/sufixo não interagem por shaping; casos complexos de tracking/kerning exigem controles próprios, não hipótese universal.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

## Contexto integral bidi ratificado — 2026-10-05

Medição anterior à decisão: o lote bilateral e a refutação Unicode estão em `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/{initial,probe}.json.gz`, com fontes/estado/binários. `layout_bidi.rs:73–82,227–240` redetecta base e inventa separadores; `shaper/bidi.rs:34` redetecta o fragmento; isto perde a entrada integral ratificada pelo humano. Correção interna de paridade, sem nova fase/assinatura pública externa.

Para spans certificados, usar registry proprietário local de strings auxiliares e mapas por input Arc, depois views BidiInfo emprestadas normais; não usar unsafe/self-reference. Embeddings virtuais LRE/RLE/PDF só quando dir do segmento difere da base; espaços reais e Linebreak não justificado LF (justify=true fora do certificado). Não retirar controles autorais. Resolver uma vez por input em cada registry; o pipeline paginado compartilha registry/views entre reorder e shape. Wrappers públicos existentes podem construir registry local.

Reorder consulta intervalo real da linha, intersects exatas por segment/range e níveis com regras de fim de linha; não midpoint nem espaços inventados. Text misto divide por fronteiras originais, preserva estilo e subspan. Itens não textuais e fontes desconhecidas não ganham certificado. Reorder conserva âncora física aceita por L1, largura/avanço reais; não aplicar fallback de suffix/coalescing ao contexto certificado.

Shape consulta os runs do input completo para span original e não auto-detecta direção de fragmento; manter offsets de subruns por travessia sequencial, emitir subspan exato para cada TextShaped. Chave de cache existente já discrimina direção; não pôr contexto em TextStyle. Reconciliation usa base do input quando existe em vez de qualquer run RTL. Fallback None anterior permanece. Testes RED→GREEN da refutação de 456 789, overrides/neutral, extração sem controles auxiliares e medição bilateral.

Folhas Text de range vazio preservam posição/estilo/span e efeitos causais; não participam da permutação visual de tinta, nem são removidas quando o intervalo Unicode não tem bytes. Aceitação RED→GREEN com Empty Text junto de texto misto, conservando a identidade do input.


## Auxiliares de teste e bindings produtivos — 2026-10-14

### Medição anterior à decisão

`03_infra/src/layout_bidi.rs` nas linhas [572] apresenta os avisos selecionados.
`00_nucleo/diagnosticos/ci-helpers-testes-2026-10-14/before.json.gz`
conserva fonte/owner, HEAD, working tree não commitado, UTC e diffstat.
references-before.json.gz registra chamadas, imports, módulos e referências;
o estado da suíte/inventário anterior foi verificado por hashes. A lista de
testes foi capturada antes da edição. Não se infere inutilidade funcional
apenas de dead_code: as chamadas privadas e sua compilação foram auditadas.

### Responsabilidade e aceitação

split_ltr_suffixes_page é adaptador privado chamado somente pelos controles da suíte; compilá-lo apenas com cfg(test). Manter seu corpo e assinatura de teste, sem restringir split_ltr_suffixes_line nem o algoritmo produtivo que o chama com índices preservados. Reorder público, fallback, origem, coordenadas e texto continuam iguais.

Manutenção interna em fluxo contínuo ADR-0127: nenhuma API pública, campo,
default de produto, fase, dependência ou supressão de lint muda. O gate de
diagnósticos selecionados deve falhar antes e passar depois; ele não é um
novo teste funcional nem prova de paridade. Preservar a lista e as expectativas
dos testes existentes, executar a suíte completa, compilar lib/all-targets,
verificar build/rustdoc/fmt/linhagem e registrar o inventário total. Não criar
testes que apenas espelhem cfg ou remoção de bindings. Resselo restrito ao owner.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Migrar fixtures de FrameItem::Shape para FramePaint/FrameStroke, preservando valores e asserts semânticos existentes. Tiling deve construir célula preparada explícita; fixtures de avaliação continuam declarativas. Não adicionar conversão produtiva total nem enfraquecer asserções.

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
