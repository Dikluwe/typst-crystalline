# L0 — builder/fonts
Hash do Código: b714220e

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/fonts.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/bitmap-embedding.toml sha256:016908bda7174f00ff63a909070553e9728d3d44c7ebf17b9616b4f676ad4def
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

## Consumer evidence

`03_infra/src/export/builder/fonts.rs:51` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Font preparation and construction

Own CIDFont/multifont build paths, font usage collection, subsetting and variable
instancing, descriptors, widths, ToUnicode, embedded font streams and CIDSet.
Font identity includes FontList, FontVariant and FontVariations. Merge derived and
explicit axes using the existing precedence; do not extend the single-font variable
instancing scope. Accumulate only the existing subsetting timing into subset_ms.

Collect shaped cluster text once before the per-font loop. Include codepoints and
glyphs from fallback Text, including spaces, in each applicable subset. ToUnicode
must retain shaped clusters and ligatures even when subsetting falls back to full
embedding; remap GIDs only when a subset mapping exists. Prepare the per-font reverse
map for direct mathematical glyphs; do not hardcode their resource to F1.

Compute glyph_to_nominal from the same effective face and axes as /W. Use variable
advances only when instancing succeeded; a default-face fallback uses default-face
advances consistently on both sides of the TJ boundary.

Emit Type0 Identity-H fonts with the correct descendant and stream representation:
TrueType uses CIDFontType2 and FontFile2/SFNT; CFF1 uses CIDFontType0 and
FontFile3/CIDFontType0C with the extracted CFF table; CFF2 uses CIDFontType0 and
FontFile3/OpenType with the complete container. Preserve descriptor metrics and
mapping semantics. Resolve BaseFont/FontName from PostScript name, then full name,
then family, then fallback; remove spaces and use the deterministic AAAAAA subset
prefix without renaming internal Fn resources.

Retain the 256 KiB font-stream compression cutoff and existing raw-data fallback
on compression failure. Every embedded CID font has a CIDSet: set MSB-first bits
for the effective CIDs after subset remapping, or used original CIDs for full
embedding, and allocate its stream outside reserved IDs. Bitmap-only fonts do not
emit an outline font or CIDSet; mixed fonts retain outline embedding for those glyphs
that need it. Prepare bitmap references under the pinned bitmap-embedding nucleus;
resource dictionary merging belongs to the resources child, and painting to the
text child. Unsupported bitmap formats retain the normal font path.

## Verification

Exercise TrueType, CFF1/CFF2, variable fallback, multifont identity, sparse/remapped
CIDSet, ligatures, spaces and bitmap-only/mixed fonts. Verify extracted text and
glyph morphology, not equivalence to an external object-allocation algorithm.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/builder/fonts.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/core/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Incluir o glyph de U+0020 na face correspondente a cada carrier no subset multifont, usando a resolução existente de estilo.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Geometria auto de separadores literais — 2026-09-29

### Medição anterior à decisão

`lab/results/construction/diagnostics/correcao-auto-espacos-2026-09-29/before/results.json`, `provenance.json` e `initial.json` medem 30 fontes/60 PDFs, font-path, comandos, UTC, HEAD, working tree e binários contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Só espaços literais (`#text("   ")`) com tracking 0/±0.2 produzem no Tekt `/MediaBox [0 0 0 0]` (Poppler apresenta 612×792), enquanto o vanilla mede respectivamente 9.36/9.76/8.96×8.568pt. Separadores literais finais após texto perdem a sua largura em páginas auto; os iniciais seguidos de texto já coincidem. Escopo com tamanho 18pt termina em 14.44×12.852pt vanilla para +0.2pt. Texto vazio e markup sem tinta revelam outra falta de baseline/mínimo; newline final tem divergência vertical própria e não é fechado por esta fatia.

Causa fonte: `space.rs:22-35` não transporta Literal inicial sem tracking; `page_geometry.rs:104-135` usa `helpers::line_content_right` que filtra todos os TextSeparator, por necessidade correta do alinhamento RTL, e a altura de linha só com carrier não recebe topo tipográfico; `builder.rs:291` e `builder/fonts.rs:633,1167` escrevem MediaBox zero. Vanilla `crates/typst-pdf/src/convert.rs:108-118` impõe mínimo físico de 3×3 na saída PDF. Isso justifica projeção auto separada do alinhamento RTL e mínimo compartilhado entre os três caminhos exportadores. Não inferir intenção histórica do escopo antigo.

### Contrato interno da correção

Nos caminhos CIDFont único e multifont, aplicar ao MediaBox o mesmo helper privado do builder para mínimo físico de 3pt por eixo. Preservar fonte/subsetting, operadores, texto, coordenadas e demais dimensões. Não duplicar a constante nem atribuir a correção de largura lógica ao exportador.

Correção interna de paridade em fluxo contínuo ADR-0127, sem campo público, assinatura/flag/fase nova. Aceitação RED→GREEN para espaços 0/± e PDF mínimo, casos causais bilaterais, corpus anterior, build e lint sem erros, resselo estritamente no manifesto. Texto vazio explícito e newline final permanecem para rodada própria.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.


## Memoização local da associação — medição 2026-10-05

`first-benchmark.json` desta rodada mede +25.36ms na fixture grid320 e +14ms em rtl160/mixed160. `font_binding.rs:index` compara bytes completos por consulta; `runs.rs:verbose_run_compatible` compara Font por bytes em cada adjacência. Inferência causal, a confirmar por A/B.

Usar cache privado por tabela de recursos/exportação para positivos validados: conservar clone Arc da fonte para manter sua memória viva e index após igualdade de bytes/pedido na primeira ocorrência. Acerto exige identidade de slice (endereço e comprimento) e mesmo pedido completo. Não serializar endereço ou criar identidade pública por ponteiro. Fontes de alocações distintas consultam igualdade completa na primeira vez. Não cachear miss no binder que anexa recursos; índices positivos permanecem estáveis por append. Nenhum global ou reúso entre sessões. Comparação de merge usa identidade de slice como atalho para igualdade, mantendo comparação de bytes para fontes de alocações diferentes. Este cache não memoiza nem invalida conteúdo entre Worlds.

## Instância variável antes do subset — correção interna 2026-10-05

### Medição anterior à decisão

`03_infra/src/export/builder/fonts.rs:933-974` submete os bytes originais ao subsetter antes de chamar o instancer. `03_infra/src/export/subset.rs:56` usa a API subset sem coordenadas. `03_infra/src/font_variant.rs:209-213` devolve bytes sem alteração quando fvar já não existe; esse Some era tratado como sucesso de instanciação. A sonda de fonte isolada e catálogo completo em `lab/results/construction/diagnostics/eixos-variaveis-export-2026-10-05/probe*.txt` preserva wght=250/900 e mede Hello em 2310/2557 unidades de 1000. O PDF anterior embute hmtx default, sem fvar, com H=697 em ambas as fontes; o controle independente fontTools mede H=689/734. Proveniência anterior em before.json. O vanilla pinado entrega bytes originais e coordenadas conjuntamente à biblioteca PDF (`lab/typst-original/crates/typst-pdf/src/text.rs:79-94`), sem exigir cópia da sua mecânica.

### Decisão e contrato interno

Correção de paridade de geometria e forma visual, não identidade de bytes/algoritmo (ADR-0107). Dentro da preparação multifont existente, fornecer os bytes originais e as coordenadas derivadas+explícitas ao subsetter::subset_with_variations por adapter privado. A sonda native.rs/native.txt demonstra H=689/734 e ausência de fvar na saída já instanciada. Subset e fallback full-font devem usar a mesma instância solicitada. Calcular glyph_to_nominal com essa instância antes do remapeamento e /W com seus bytes subsetados; não anunciar sucesso quando uma fonte foi congelada no default. Falha do instancer mantém aviso e fallback default consistente entre nominal e /W. Preservar GIDs, cluster mappings, fontes estáticas, eixos desconhecidos, APIs, flags e fase export; não introduzir unsafe. A ordem de operações privadas na mesma fase não é mudança de fase/ordem arquitetural do pipeline. Não estender a rota CIDFont única.

Aceitação: teste RED→GREEN do PDF com Ubuntu Sans, pedidos explícitos conflitantes com weight=700/400, H/e/l/o=689/547/247/580 e 734/584/316/607; ambos os modos de stream. Controles completos fonte→PDF, variações aninhadas, pesos derivados, fonte estática, TrueType/CFF2; caixas e render conferidos contra o vanilla pinado. Build e lint sem novos erros, ownership/pins válidos e resselo apenas deste owner. Custo CLI medido sem promessa de ganho. Não chamar fontTools após subset sucesso; no fallback, chamar sobre a fonte original, preservando a instância solicitada. A dependência/guard vigente de fontTools permanece neste pacote.

O pedido privado de subset multifont inclui coordenadas mesmo quando a lista está vazia: significa instância default. Isso permite congelar CFF2 default dos espaços da mesma exportação; APIs públicas legadas e a rota CIDFont única preservam seus escopos. O controle PDF, além de coordenadas, exige ausência de aviso Poppler para os recursos preparados nessa rota.


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

Coletar glyphs das células preparadas e integrar registro/emissão de PatternType 1 nos caminhos CID/multifont, com mesmos recursos e fontes, sem semântica ou links duplicados.

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


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

03_infra/src/export/builder/fonts.rs:63 (C040); 03_infra/src/export/builder/fonts.rs:67 (C041); 03_infra/src/export/builder/fonts.rs:846 (C043).
O snapshot before.json.gz em lab/results/construction/diagnostics/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

Nos três tipos repetidos da coleção de fontes, usar o FontKey público já existente em export/render.rs, reexportado por export. Seu RHS permanece ((FontList, FontVariant, FontVariations), Vec<u8>). É abreviação transparente do mesmo tipo Rust: preservar campos, ordem, borrows, movimentos, seleção, instanciamento, mapas e timings. Remover o import FontVariant sem uso após a abreviação; FontList/FontVariations requeridos pelos testes descendentes ficam em cfg(test). Não criar newtype, converter buffers ou clonar fontes.

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


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

03_infra/src/export/builder/fonts.rs:289 (clippy::single_match). O snapshot `lab/results/construction/diagnostics/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

Substituir o match de compress_zlib com único braço útil por if let Ok(compressed), mantendo cutoff 256 KiB, construção da stream, retorno no sucesso e fallback cru no erro. Não mudar política de compressão ou dados.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.


## Aridade interna — materialização de seis casos, 2026-10-16

### Medição anterior à decisão

As fontes, chamadores, cláusulas vigentes, spans e inventário de 28 avisos foram conferidos antes da decisão. `lab/results/construction/diagnostics/clippy-aridade-interna-2026-10-16/before.json.gz` guarda fontes/L0 completos, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A auditoria anterior classifica estes casos como correção; não são exceções diretas. O RED deve observar as aridades atuais antes de alterações; controles funcionais existentes serão corridos antes/depois. É refatoração de representação privada, sem nova paridade ou ganho de performance medido.

### Responsabilidade exclusiva e aceitação

Migrar as duas composições de PageContext para with_font_scenario e FontScenario::{Cidfont,Multifont} nomeados com exatamente os mesmos empréstimos e índices. CIDFont conserva bitmap Some/None por bitmap_only; Multifont conserva bindings Some(LookupCache::default()) criado uma vez por contexto. Manter setters with_pdf_tags e with_oracle, tiling_names, ordem de emissão, subsetting e demais algoritmos intactos. Importar FontScenario somente do módulo interno stream, sem reexport público novo.

Fluxo contínuo ADR-0127: nenhuma API externamente pública, default do produto, fase, dependência ou semântica é alterada. Tipos/constructors novos ficam internos; contratos públicos existentes permanecem. Este adendo concretiza somente os seis casos nominais; substitui proibições genéricas de supressão apenas para as duas expectativas residuais condicionadas C047/C066 após remover _text. As outras cinco aridades a corrigir continuam visíveis. Não elevar limiar ou silenciar no módulo/impl/crate. L0 precede código e resselo restrito. Confirmar preservação das assinaturas externamente públicas, campos/valores transportados, controles PNG/SVG/PDF, precedência/diagnósticos/recursos/MCIDs e ausência de mutações globais. Usar os testes existentes sem relaxar fixtures/esperados; não escrever testes que espelhem os carriers privados. Revalidar suíte workspace completa, build, Clippy all-targets e unfulfilled_lint_expectations, Rustdoc, fmt e linhagem estrita/reversa, mantendo 12 GiB, swap zero e um job.
