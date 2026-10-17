# L0 — infra/export/gradients/mod
Hash do Código: 6b373abf

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/pdf-conic-mesh.toml sha256:90087538b1ce25b44a6d22122c3981b22ec40d33c5166ebfb8db3b8e67900954
- 00_nucleo/prompts/_nuclei/pdf-conic-placement.toml sha256:9010358e51cfa3267960432244b972b179d447f572c902101eba650f6d6a67cb
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/mod.rs:54` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Gradient discovery façade

Own PatternRef, GradientObjectKind/GradientObject, RectKey/DedupKey, bbox/key helpers,
scan_all_gradients, pattern_resources_for_page and restricted child reexports.
Keep private static child modules; neither formatting functions nor sampling/
mesh algorithms are duplicated here.

Traverse page background, body and foreground recursively through Group/Link/
Semantic. Register Shape.fill and Shape.stroke Gradient paints in encounter order
with the same effective key; reuse matching fill/stroke identity once.
DedupKey combines gradient Arc pointer with effective bbox quantized by the existing
RectKey representation. Group passes its local bbox context; the shape's own
parent_bbox_at_emit wins over the inherited override.

Spot gradients additionally retain the painted shape rect for placement and lookup,
separate from a container's relative-parent bbox. The same spot gradient reused
at different shape locations must retain those placements; linear/radial process keys preserve
their existing policy; conic placement follows the refinement below. Keep sequential allocated pattern/shading/function IDs and
recursive page resource lookup synchronized with discovery, without collisions
or unresolved references. Do not include nonvisual alt as paint.

## Delegated owners

- `00_nucleo/prompts/infra/export/gradients/linear.md`: axial coordinates/RGB sampling.
- `00_nucleo/prompts/infra/export/gradients/radial.md`: focal circles/RGB sampling.
- `00_nucleo/prompts/infra/export/gradients/cmyk.md`: CMYK sampling/conversion.
- `00_nucleo/prompts/infra/export/gradients/relative.md`: relative tuple transforms.
- `00_nucleo/prompts/infra/export/gradients/adaptive.md`: PDF counts/SVG adaptive stops.
- `00_nucleo/prompts/infra/export/gradients/conic.md`: Type 6 mesh payloads.
- `00_nucleo/prompts/infra/export/gradients/function_dict.md`: interpolation functions.

`00_nucleo/prompts/infra/export/builder/resources.md` owns emitted resource objects
and shading placement; `00_nucleo/prompts/infra/export/stream/shape.md` owns paint
operators. This façade performs no I/O or entity interpolation.

## Verification

Check recursive three-layer discovery, fill/stroke reuse, distinct effective boxes,
spot placements and page-local reference closure. Pointer/key mechanics preserve
the current implementation; they do not define language equality.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/gradients/mod.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

TextSeparator não possui paint nem recolhe gradientes.

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

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/gradients/mod.rs` nas linhas 18, 20, 184, 351 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `00_nucleo/diagnosticos/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Remover somente os imports privados comprovadamente sem uso neste consumer; manter os reexports públicos e os imports exigidos por chamadas ou contratos. A lista exata de spans anterior à edição está no diagnóstico citado.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.

### Refinamento de escopo após compilação — 2026-10-13

A remoção dos imports do facade revelou reexports privados usados apenas pela suíte: perceptual_distance_in_space, rgb_to_cmyk, bezier_control_points_for_arc, compute_coons_patches_n_stops, compute_coons_patches_n_stops_extended e multispace_sample_stops_conic. Exportá-los ao pai somente sob cfg(test). Definições e chamadas internas produtivas continuam; conic consulta rgb_to_cmyk pelo caminho cmyk, não por este alias. Evidência e estado: `00_nucleo/diagnosticos/infra-imports-docs-ci-2026-10-13/`. Revalidar todos os targets e preservar nomes/assertions da suíte.

## Posicionamento cônico — 2026-10-14

### Medição anterior à decisão

`03_infra/src/export/gradients/mod.rs:224-264` perde a posição própria de
Conic RGB/CMYK e deduplica por Arc/bbox; `builder/resources.rs:564-574`
emite matriz identidade nesses ramos. `stream/draw.rs:522-555,939-965`
compõe grupos, mas Pattern é relativo ao espaço inicial do stream.
`stream/page.rs:45-49` acrescenta bleed antes dos itens. O corpus constante
red versus conic(red,red), em before/probe.json.gz, reproduz a perda de
cobertura na árvore atual. Fonte, L0, HEAD, UTC, status e diffstat estão em
`00_nucleo/diagnosticos/conic-posicionamento-2026-10-14/placement-before.json.gz`.
Os dois lados do teste são Tekt: o defeito independe de imitar o vanilla.

### Responsabilidade e aceitação

Capturar a matriz cônica na descoberta, usando TransformMatrix existente para composição. A chave cônica inclui identidade da ocorrência FrameItem e Arc, sem quantizar posição nem colidir entre páginas. FrameItems possuem filhos Vec e os três builders percorrem o mesmo PagedDocument emprestado sem cloná-lo entre descoberta e pintura; a chave é local à exportação, nunca persistida ou igualdade de linguagem. Demais gradientes conservam a política existente. Recurso da página usa a mesma chave. O contexto privado de percurso carrega matriz até PDF e caixa parent local, separado do override legado dos outros gradientes.

Correção interna ADR-0127, sem entidade/assinatura pública externa, flag,
default ou fase nova. L0 precede testes RED e implementação; resselo restrito.
Testar posição e dimensão, mesmo Arc em ocorrências/páginas distintas,
fill/stroke, grupos afins aninhados, bleed, parent explícito/local/página,
recursos e pintura nos modos Compact/Verbose. Repetir o corpus raster sem
recortar ou realinhar o resultado.

Esta etapa permanece incompleta quanto à funcionalidade cônica integral:
a etapa seguinte, conic-geometria-amostragem, deve completar centro autoral,
cobertura dos cantos e amostragem CMYK/offsets. Não atribuir cobertura total
à correção de posicionamento nem legitimar o círculo inscrito como resultado
final. O owner conic conserva a malha nesta etapa para isolar a causa.

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

Além da matriz já validada, transportar o tamanho físico da referência conic e
os limites do canvas convertidos pela inversa da matriz até o espaço unitário do
padrão. Incluir a caixa unitária; isso cobre stroke/caminhos fora da bbox nominal
sem adivinhar miter ou expansão de path. Conservar canvas através de grupos e
bleed. Inversa singular/não finita conserva domínio unitário defensivo, sem
alegar cobertura da situação degenerada. Façade reexporta somente tipos/helper
privados exigidos; wrappers antigos tornam-se aliases de teste. Atualizar os
comentários de deduplicação para distinguir identidade de ocorrência conic das
chaves históricas linear/radial.

Correção interna de fórmula/paridade ADR-0127, sem contrato público externo,
modo/flag padrão ou mudança de fase. L0 precede testes RED e código. Não mudar
expectativas funcionais para absorver erro; assertions mecânicas obsoletas são
substituídas pela verificação independente descrita. Comparar corpus completo,
validar qpdf, raster, todas as regressões e linhagem/reverso. A referência
binária cônica só pode ser regenerada após demonstrar a mudança intencional
pelo harness e pelo protocolo vigente do corpus.

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
consumer as construções/desestruturações antes da edição: [668].
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

### Integração PDF ainda refutada nesta etapa

first-render-delta.json e after-first/results.json.gz em paint-root-2026-10-14
registram uma regressão no controle block-auto-gradient-parent/PDF: 3840
pixels mudam mais de 16 níveis contra o baseline e a distância ao pin cresce.
Os demais 35 renders Tekt do corpus mantêm pixels. O collector legado de
gradientes troca parent_bbox_override a cada Group; o novo envelope de raiz
expõe que esse override não é um referencial afim. Os campos de Shape ainda
são declarativos. Essa diferença é falha aberta, não nova referência aceita,
e não autoriza mudar tolerância ou o esperado. T1c/T2 devem fechar o contexto
resolvido de pintura e os recursos para corrigir a causa; T1b isoladamente
não certifica render nem produção. Preservar a amostra refutadora e também
os controles que coincidem, inclusive a divergência vanilla PDF de quarter.


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

Consumir FramePaint nas varreduras de Shape. Registrar gradientes também dentro das células com canvas/raiz locais e dimensões da célula; manter identidade de ocorrência dos frames emprestados para cônicos. Recursão de recursos inclui gradientes aninhados e mantém regras existentes fora de tiling.

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

Gradientes de células PDF usam espaço local Y-down (to_pdf identidade), pois a Matrix do PatternType 1 faz a inversão externa.


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

- `C045`: `03_infra/src/export/gradients/mod.rs:320`, `walk`, 9 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C045` — `walk`: Correção interna prescrita

O walk recursivo recebe repetidamente o mesmo mapa, duas saídas e dois contadores mutáveis que formam o estado de descoberta. O L0 exige sincronismo de índices/IDs; esse estado deve ter um dono local nomeado, separado do referencial geométrico por ramo.

Collector local de descoberta possui ptr_to_idx, refs, grad_objs, next_id e counter. A recursão recebe o collector mais items/referencial/altura/placement; não trocar a travessia nem dedup.

Aceitação: IDs/índices sincronizados, ordem de background/body/foreground, reutilização fill/stroke, conic em grupos/tiling e referências de página preservados.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Aridade interna — materialização de seis casos, 2026-10-16

### Medição anterior à decisão

As fontes, chamadores, cláusulas vigentes, spans e inventário de 28 avisos foram conferidos antes da decisão. `00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16/before.json.gz` guarda fontes/L0 completos, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A auditoria anterior classifica estes casos como correção; não são exceções diretas. O RED deve observar as aridades atuais antes de alterações; controles funcionais existentes serão corridos antes/depois. É refatoração de representação privada, sem nova paridade ou ganho de performance medido.

### Responsabilidade exclusiva e aceitação

C045: em scan_all_gradients, criar Collector local proprietário de ptr_to_idx, refs, grad_objs, next_id e counter. O walk recursivo recebe items, parent_bbox_override, page_height, placement e &mut Collector. Inicializar next_id=first_id e counter=1, manter incremento por Function/Shading/Pattern e a mesma ordem de background/body/foreground, Shape fill/stroke e células de tiling; mover as mesmas saídas ao retorno. Não alterar o segundo walk de pattern_resources_for_page, dedup keys, posição/conic domain, match exaustivo, IDs, geometria, ownership de Arc ou limites.

Fluxo contínuo ADR-0127: nenhuma API externamente pública, default do produto, fase, dependência ou semântica é alterada. Tipos/constructors novos ficam internos; contratos públicos existentes permanecem. Este adendo concretiza somente os seis casos nominais; substitui proibições genéricas de supressão apenas para as duas expectativas residuais condicionadas C047/C066 após remover _text. As outras cinco aridades a corrigir continuam visíveis. Não elevar limiar ou silenciar no módulo/impl/crate. L0 precede código e resselo restrito. Confirmar preservação das assinaturas externamente públicas, campos/valores transportados, controles PNG/SVG/PDF, precedência/diagnósticos/recursos/MCIDs e ausência de mutações globais. Usar os testes existentes sem relaxar fixtures/esperados; não escrever testes que espelhem os carriers privados. Revalidar suíte workspace completa, build, Clippy all-targets e unfulfilled_lint_expectations, Rustdoc, fmt e linhagem estrita/reversa, mantendo 12 GiB, swap zero e um job.
