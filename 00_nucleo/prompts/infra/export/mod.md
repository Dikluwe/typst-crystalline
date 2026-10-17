# L0 — infra/export/mod
Hash do Código: e91c5bdb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/mod.rs:196` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Public exporter façade

Own the public PDF option enums, compatibility reexports, and the existing PDF,
PNG, SVG and HTML API surface. Preserve every current signature and parameter
order, including document IDs, timing variants and explicit tags/serialization/
pages/standard variants. Do not restore obsolete signatures without StreamMode.

StreamMode defaults to Verbose; Compact remains the explicit optional graphics
mode under ADR-0126. PdfTags defaults to Enabled. PdfSerializationMode defaults
to Compact; Pretty is an independent physical-serialization option. Conveniences
supply Enabled, Compact, no page selection and PdfStandards::default() (PDF 1.7,
no archival/accessibility profile) when their corresponding axes are omitted.
Pass resolved PageRanges and PdfStandards through without interpreting CLI flags,
feature gates, profile compatibility or upstream tagging-selection policy.

The font-bearing families use FontList/FontVariant/FontVariations identity.
Single-font entrypoints pass bytes to PdfBuilder; multifont entrypoints parse every
face and use the existing Helvetica fallback for an empty collection or any failed
face parse. Timing variants return the existing subset timing; non-timing variants
discard it. Preserve the external 16-byte document-ID option. Standard-specific
emission belongs to the builder descendants, including identification, output intent and tagged structure as well as the header.

The oracle entrypoints explicitly enable the diagnostic path. Keep their existing
single versus multifont routing, including the variable-font/merged-axis condition,
and ordinary defaults for axes not present in their signatures. Ordinary entrypoints
do not enable oracle transformations.

PNG wrappers delegate to raster. SVG and HTML use direct reexports rather than
parallel encoders; HTML remains the separate semantic target defined by ADR-0128.
Keep test-only helper reexports restricted to tests. No document layout, filesystem
routing, output validation or encoding algorithm belongs in this façade.

## Delegation

- `00_nucleo/prompts/infra/export/builder.md`: PDF orchestration.
- `00_nucleo/prompts/infra/export/stream.md`: borrowed PDF emission context.
- `00_nucleo/prompts/infra/export/fonts.md`: font/Unicode helpers.
- `00_nucleo/prompts/infra/export/font_subset.md`: subset adapter.
- `00_nucleo/prompts/infra/export/images.md`: PDF image façade.
- `00_nucleo/prompts/infra/export/gradients/mod.md`: gradient discovery.
- `00_nucleo/prompts/infra/export/render.md`: raster.
- `00_nucleo/prompts/infra/export/svg.md`: SVG.
- `00_nucleo/prompts/infra/export/html.md`: HTML.
- `00_nucleo/prompts/infra/export/oracle.md`: diagnostic transformation.
- `00_nucleo/prompts/infra/export/pdf_defaults.md`: shared constants.
- `00_nucleo/prompts/infra/export/tests/mod.md`: PDF regression aggregation.

## Verification

Check option forwarding and all convenience defaults through the existing public
families. Preserve text, geometry, profile fields, tagging relationships and exact
diagnostic messages through their owners. Complete PDF bytes, object IDs and
allocation algorithms are not language-parity requirements.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/mod.rs` nas linhas 45, 55, 56, 57, 61, 63, 64, 65, 70, 71, 74, 75, 76 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `00_nucleo/diagnosticos/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Remover somente os imports privados comprovadamente sem uso neste consumer; manter os reexports públicos e os imports exigidos por chamadas ou contratos. A lista exata de spans anterior à edição está no diagnóstico citado.

Manter em cfg(test) os seis helpers de gradientes e process_png_for_pdf usados pelos testes descendentes via super; retirar os imports de bitmap/stream e os tipos RectKey/PdfImagePayload não usados neste facade. Não remover definições, reexports dos filhos ou caminhos qualificados consumidos pelos builders.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.

## Import privado de chave — posicionamento cônico 2026-10-14

Medição anterior: first-validation/followup-clippy-inventory.json no diagnóstico
conic-posicionamento-2026-10-14 registra dedup_key_for sem uso em export/mod.rs:54.
O último consumidor era o alias privado do stream, agora substituído pela chave
por ocorrência do facade gradients. snapshot-before.json.gz conserva fonte/L0.
Remover somente esse import privado, mantendo definição, API e reexports usados.
Revalidar todos os targets e linhagem, sem suprimir avisos.

## Geometria e amostragem cônica — 2026-10-14

### Medição anterior à decisão

`conic.rs:142-143,241-242` fixa centro/raio; `:271-272` associa paradas CMYK
por índice circular, ignorando offsets e a amostra final. `resources.rs:508-552`
fixa coordenadas de 8 bits e Decode unitário. As sondas antes/depois de
posicionamento e refuters no diagnóstico conic-posicionamento-2026-10-14 provam
cantos brancos, centro ignorado e offsets CMYK sem efeito, apesar de sample()
responder corretamente. before.json.gz em conic-geometria-amostragem-2026-10-14
conserva fontes/L0, HEAD, working tree, diffstat, UTC e os binários. O corpus
bilateral mede centro e cores nos eixos/diagonal, quadrado/retângulo e rotação.
A fonte pinada typst-pdf/src/paint.rs:274-299 e os pixels concordam: o início
é à esquerda e o sentido é horário; o código Tekt anterior não corresponde.
O vanilla observado nesses controles não é um fallback uniforme.
PDF Reference 1.3, tabela 4.31 (página PDF 250), permite coordenadas 16 bits e
Decode independente por eixo; é contrato de formato, não igualdade upstream.

### Responsabilidade e aceitação

Ajustar apenas imports/reexports privados: emissor de malha e domínio para a
produção, wrappers e helpers históricos sob cfg(test). Preservar a superfície
pública PDF/PNG/SVG/HTML e todos os defaults. Sem algoritmo novo neste facade.

Correção interna de fórmula/paridade ADR-0127, sem contrato público externo,
modo/flag padrão ou mudança de fase. L0 precede testes RED e código. Não mudar
expectativas funcionais para absorver erro; assertions mecânicas obsoletas são
substituídas pela verificação independente descrita. Comparar corpus completo,
validar qpdf, raster, todas as regressões e linhagem/reverso. A referência
binária cônica só pode ser regenerada após demonstrar a mudança intencional
pelo harness e pelo protocolo vigente do corpus.


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

Declarar o módulo privado tiling para geometria de paints preparados compartilhada pelos exporters. Nenhuma assinatura pública de export, default, modo de serialização, tag ou seleção de páginas muda.

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

O módulo privado pdf_tiling possui a emissão/registro PatternType 1 segundo seu owner canônico; não altera API pública da fachada.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C074`: `03_infra/src/export/mod.rs:446`, `export_pdf_with_font_and_document_id_and_tags_and_serialization_and_pages_and_standard`, 8 argumentos contados pelo Clippy (limiar 7).
- `C075`: `03_infra/src/export/mod.rs:555`, `export_pdf_with_font_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard`, 8 argumentos contados pelo Clippy (limiar 7).
- `C082`: `03_infra/src/export/mod.rs:672`, `export_pdf_multifont_with_document_id_and_tags_and_serialization_and_pages_and_standard`, 8 argumentos contados pelo Clippy (limiar 7).
- `C089`: `03_infra/src/export/mod.rs:810`, `export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard`, 8 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C074` — `export_pdf_with_font_and_document_id_and_tags_and_serialization_and_pages_and_standard`: Exceção pontual justificada

Fachada pública single-font com oito eixos já publicados, incluindo tags, serialização, pages e standard independentes. O L0 proíbe trocar ordem/assinatura e exige encaminhamento literal ao builder.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C074: Fachada pública single-font com oito eixos já publicados, incluindo tags, serialização, pages e standard independentes")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C075` — `export_pdf_with_font_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard`: Exceção pontual justificada

Variante pública single-font com tempos conserva os mesmos oito inputs e devolve subset timing. É um adapter de compatibilidade, sem novo algoritmo ou resolução de opções.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C075: Variante pública single-font com tempos conserva os mesmos oito inputs e devolve subset timing")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C082` — `export_pdf_multifont_with_document_id_and_tags_and_serialization_and_pages_and_standard`: Exceção pontual justificada

Fachada pública multifont preserva a coleção completa com variantes/eixos e opções PDF explícitas. A assinatura está congelada pelo L0; não colapsar serialização com StreamMode ou tags com standard.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C082: Fachada pública multifont preserva a coleção completa com variantes/eixos e opções PDF explícitas")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C089` — `export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard`: Exceção pontual justificada

Variante pública multifont com tempos preserva os mesmos eixos e o retorno de timing. A aridade é da superfície de compatibilidade; eventual deduplicação dos corpos dos exporters é outra auditoria e não exige trocar esta assinatura.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C089: Variante pública multifont com tempos preserva os mesmos eixos e o retorno de timing")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### Limite da exceção

Autorizar somente `#[expect(clippy::too_many_arguments, reason = "...")]` no item
nominal acima, com o motivo arquitetural indicado. Não aplicar no módulo, impl,
crate ou configuração global; não elevar o limiar nem abranger outro lint.
Nos casos com remoção de argumento morto, a exceção residual depende dessa correção.
Se a função deixar de exceder o limiar, retirar/revisar a expectativa; não silenciar
`unfulfilled_lint_expectations`. Esta permissão específica supera proibições
genéricas de supressão em seções anteriores somente para este lint e os itens
nominais justificados aqui. As demais obrigações de correção permanecem.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

03_infra/src/export/mod.rs:589 (C076); 03_infra/src/export/mod.rs:598 (C077); 03_infra/src/export/mod.rs:613 (C078); 03_infra/src/export/mod.rs:630 (C079); 03_infra/src/export/mod.rs:651 (C080); 03_infra/src/export/mod.rs:674 (C081); 03_infra/src/export/mod.rs:727 (C083); 03_infra/src/export/mod.rs:736 (C084); 03_infra/src/export/mod.rs:751 (C085); 03_infra/src/export/mod.rs:768 (C086); 03_infra/src/export/mod.rs:789 (C087); 03_infra/src/export/mod.rs:812 (C088); 03_infra/src/export/mod.rs:868 (C090); 03_infra/src/export/mod.rs:885 (C091); 03_infra/src/export/mod.rs:991 (C092).
O snapshot before.json.gz em 00_nucleo/diagnosticos/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

Usar o FontKey público já reexportado nas quinze assinaturas multifont/PNG que repetem sua expansão. FontKey é alias transparente preexistente com RHS ((FontList, FontVariant, FontVariations), Vec<u8>), não entidade nova: após expandi-lo, todos os tipos das assinaturas públicas, ordem dos parâmetros, lifetimes, defaults e retornos permanecem exatamente iguais. Não mudar o RHS, visibilidade ou reexports do alias. A medição em 03_infra/src/export/tests/mod.rs:31-32 mostra imports próprios de FontVariant e FontVariations; o inventário validation-initial/validation-clippy.log registra os dois imports redundantes no pai. Removê-los do owner; manter somente FontList em cfg(test), usado pela suíte descendente. Registrar comparação das assinaturas antes/depois com expansão do alias, além da compilação dos chamadores existentes.

Correção interna em fluxo contínuo ADR-0127: nenhuma entidade, assinatura pública
resolvida, default ou fase é alterada. Uma abreviação de tipo só é permitida com
expansão comprovadamente idêntica; incompatibilidade ou tipo público novo exige
outro gate. As exceções são nominais apenas para type_complexity; não autorizar
supressão em módulo/impl/crate nem elevar limiar. Esta permissão pontual supera
proibições genéricas anteriores somente para os três itens justificados neste lote;
continuam proibidas supressões dos demais casos destinados à correção.
Usar os controles funcionais existentes sem mudar seus esperados. O gate selecionado
deve passar RED→GREEN, incluindo unfulfilled_lint_expectations como erro; revalidar
build, Clippy all-targets, suíte workspace, formato, Rustdoc e linhagem com resselo
restrito. Não criar testes que apenas espelhem aliases ou nomes de campos.


## Manutenção local e observação de testes — 2026-10-16

### Medição anterior à decisão

`03_infra/src/export/mod.rs` nas linhas 201, 224 contém os avisos registrados em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/before.json.gz`. O snapshot conserva fontes/L0, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A inspeção dos consumidores e usos precedeu esta decisão; o gate Clippy selecionado deve falhar antes do código. Manutenção de representação/observação Rust, sem nova medição de intenção upstream ou paridade.

### Responsabilidade e aceitação

Conservar os impls Default já públicos por meio de derive(Default) e #[default] explícito em StreamMode::Verbose e PdfTags::Enabled. Manter todos os outros derives, variantes e valores; não trocar o default de produção ADR-0126 nem PdfSerializationMode.

Fluxo contínuo ADR-0127: sem contrato público, default de produto ou fase novos. L0 precede código e resselo restrito. Nenhuma supressão de lint nova. Conferir redução exata do inventário e ausência de avisos novos, gate Clippy RED→GREEN e unfulfilled_lint_expectations como erro; executar suíte workspace, build, formato, Rustdoc e linhagem sob 12 GiB, swap zero e um job. Não criar testes que apenas espelhem derive, Entry ou posicionamento de módulos. O caso de bbox exige observação real conforme especificado; os demais controles existentes mantêm fixtures e expectativas.


## Exceções nominais materializadas — 2026-10-16

### Medição anterior à decisão

O inventário validado em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `00_nucleo/diagnosticos/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Materializar somente as expectativas diretas nominais já justificadas neste owner: C074, C075, C082, C089. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.
