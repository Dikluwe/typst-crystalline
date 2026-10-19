# L0 — infra/shaper/traversal
Hash do Código: 83be52a8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/traversal.rs`

## Current evidence

- `03_infra/src/shaper/traversal.rs:17` — `shape_document`.
- `03_infra/src/shaper/traversal.rs:34` — `shape_page`.
- `03_infra/src/shaper/traversal.rs:40` — `shape_layer`.
- `03_infra/src/shaper/traversal.rs:59` — `shape_item`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/traversal.rs`.

## Owned contract

- Own shape_document/page/item traversal. Create run and face caches per document and visit background, items and foreground in that order.
- Descend into Group, Link and Semantic without changing wrapper metadata, positions or child order. Preserve the closed static dispatch and nontext items.
- Delegate Text with a requested font to shaping::try_shape_context; preserve original text on absent font, failed resolution or whitespace. Do not reshape existing TextShaped or implement selection in this walker.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/shaper/traversal.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Preservar TextSeparator como carrier; não o converter em tinta nem agregar ao texto vizinho.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## SHY autoral não selecionado — correção de identidade

### Medição anterior à decisão

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/initial.json` e
`before/resultados.json` registram estado, binários, comandos e trinta sondas
bilaterais. Em largura 300pt, repre<U+00AD>sentation extrai dois SHY no Tekt
e um no vanilla pinado. O controle spaces extrai seis versus um.
`03_infra/src/shaper/shaping.rs:313-325` transporta o cluster SHY produzido
pelo rustybuzz; `03_infra/src/export/fonts.rs:318-328` associa sua substring
ao GID de espaço. `export/builder/fonts.rs:523-535` conserva uma identidade
por GID; a stream do controle anterior associa CID 0001 a 00AD e o reutiliza
no espaço posterior. São observáveis textuais, não igualdade de bytes.
A hipótese seria refutada por ausência dessa associação ou por duplicação
já existente na string que entra no shaper. Não inferir intenção histórica.

### Decisão e limite

Correção de paridade interna em fluxo contínuo ADR-0127: reutiliza contrato
TextReplacement aprovado, sem campo, assinatura pública, flag ou mudança
de fase. Não modifica seleção de quebras ou desenho. A causa explícita é
o U+00AD existente no texto autoral, não aparência, GID ou posição.
Casos estreitos já têm divergências de seleção e não fecham a política SHY.

Depois de try_shape bem-sucedido, delegar a text_identity a proteção dos
sub-runs cujo texto contém U+00AD. Preservar glifos, texto visual e posições.
Dentro de TextReplacement já existente, processar filhos com proteção
suprimida, preservando o envelope externo e evitando substituição aninhada.
Usar helper privado de percurso para esse estado, preservando shape_document
na API pública; shape_item tem visibilidade pub(super) e pertence à suíte
conforme a delimitação medida abaixo. Falha de fonte conserva Text.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

## Contexto integral bidi ratificado — 2026-10-05

Medição anterior à decisão: o lote bilateral e a refutação Unicode estão em `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/{initial,probe}.json.gz`, com fontes/estado/binários. `layout_bidi.rs:73–82,227–240` redetecta base e inventa separadores; `shaper/bidi.rs:34` redetecta o fragmento; isto perde a entrada integral ratificada pelo humano. Correção interna de paridade, sem nova fase/assinatura pública externa.

Para spans certificados, usar registry proprietário local de strings auxiliares e mapas por input Arc, depois views BidiInfo emprestadas normais; não usar unsafe/self-reference. Embeddings virtuais LRE/RLE/PDF só quando dir do segmento difere da base; espaços reais e Linebreak não justificado LF (justify=true fora do certificado). Não retirar controles autorais. Resolver uma vez por input em cada registry; o pipeline paginado compartilha registry/views entre reorder e shape. Wrappers públicos existentes podem construir registry local.

Reorder consulta intervalo real da linha, intersects exatas por segment/range e níveis com regras de fim de linha; não midpoint nem espaços inventados. Text misto divide por fronteiras originais, preserva estilo e subspan. Itens não textuais e fontes desconhecidas não ganham certificado. Reorder conserva âncora física aceita por L1, largura/avanço reais; não aplicar fallback de suffix/coalescing ao contexto certificado.

Shape consulta os runs do input completo para span original e não auto-detecta direção de fragmento; manter offsets de subruns por travessia sequencial, emitir subspan exato para cada TextShaped. Chave de cache existente já discrimina direção; não pôr contexto em TextStyle. Reconciliation usa base do input quando existe em vez de qualquer run RTL. Fallback None anterior permanece. Testes RED→GREEN da refutação de 456 789, overrides/neutral, extração sem controles auxiliares e medição bilateral.

## Identidade bidi por ocorrência — clusters certificados (2026-10-06)

### Medição anterior à decisão

`lab/results/construction/diagnostics/bidi-identidade-ocorrencia-2026-10-06/before.json`
registra HEAD, working tree, código/L0, binários e sonda rustybuzz. Com a face
DejaVu versionada por SHA e REMOVE_DEFAULT_IGNORABLES, LRM+ABC e PDI+ABC
produzem A no cluster byte 0, B no byte 4 e C no byte 5. A substring 0..4
contém controle+A. O ToUnicode visual em fonts.rs:320-347 remove controles;
text_identity.rs:15 só certifica SHY. O lote anterior verification.json.gz
em controles-bidi-pdf-2026-10-06 mede falta de U+200E/U+2069 nessas três
ocorrências, com glifos preservados. O vanilla pinado 9f2b6e871, inline/
shaping.rs:1052-1099, fornece ranges completos dos clusters à exportação.
Semântica textual/Unicode e morfologia são o alvo; GIDs/bytes/algoritmo não.
Hipótese refutável por identidade ainda ausente, duplicação em outra ocorrência
da mesma letra ou alteração da tinta após envelope certificado. Não inferir
intenção histórica.

### Classificação

Correção interna de paridade ADR-0127: usa TextReplacement aprovado, sem
campo/método/assinatura pública, fase, modo, flag, dependência ou unsafe novo.
Certificado provém dos ranges UTF-8 do shaping bem-sucedido, não de GID,
avanço, posição, aparência, fonte ou mera presença de controle no run.
Os invariantes do envelope pertencem ao núcleo text-replacement já pinado.

### Responsabilidade exclusiva

Delegar ao helper privado protect_authored_identity depois de shaping válido,
somente fora de TextReplacement existente. Não examinar clusters aqui, não
implementar certificação no walker, preservar cache/face e descendência
estática. Dentro de envelope existente, preservar sua identidade autoral e
não adicionar envelope novo. Testes de integração asseguram escopo e cache hit.


## Auxiliares de teste e bindings produtivos — 2026-10-14

### Medição anterior à decisão

`03_infra/src/shaper/traversal.rs` nas linhas [77] apresenta os avisos selecionados.
`lab/results/construction/diagnostics/ci-helpers-testes-2026-10-14/before.json.gz`
conserva fonte/owner, HEAD, working tree não commitado, UTC e diffstat.
references-before.json.gz registra chamadas, imports, módulos e referências;
o estado da suíte/inventário anterior foi verificado por hashes. A lista de
testes foi capturada antes da edição. Não se infere inutilidade funcional
apenas de dead_code: as chamadas privadas e sua compilação foram auditadas.

### Responsabilidade e aceitação

shape_item é adaptador pub(super) exclusivo da suíte, não API pública L3: o módulo traversal é privado e a fachada shaper exporta shape_document. Compilar somente shape_item em cfg(test); manter shape_item_inner e shape_document produtivos com seus contextos e invariantes. Conservar todos os testes e a delegação, sem passar novamente pelo wrapper sem contexto na produção.

Manutenção interna em fluxo contínuo ADR-0127: nenhuma API pública, campo,
default de produto, fase, dependência ou supressão de lint muda. O gate de
diagnósticos selecionados deve falhar antes e passar depois; ele não é um
novo teste funcional nem prova de paridade. Preservar a lista e as expectativas
dos testes existentes, executar a suíte completa, compilar lib/all-targets,
verificar build/rustdoc/fmt/linhagem e registrar o inventário total. Não criar
testes que apenas espelhem cfg ou remoção de bindings. Resselo restrito ao owner.


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

Executar shaping visual das células preparadas uma vez por ocorrência via Arc::make_mut e caches locais existentes, preservando size/spacing/placement. A célula usa seu próprio contexto bidi, sem inserir suas identidades na página.

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
