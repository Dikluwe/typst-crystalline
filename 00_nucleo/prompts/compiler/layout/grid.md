# L0 — grid
Hash do Código: bcf419c4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/grid.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/grid.rs:125`, `01_core/src/compiler/layout/grid.rs:154`, `01_core/src/compiler/layout/grid.rs:536`, `01_core/src/compiler/layout/grid.rs:30`, `01_core/src/compiler/layout/grid.rs:31`, `01_core/src/compiler/layout/grid.rs:32`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own Grid dispatch and the shared visual grid/table engine. Delegate cell-row emission and pagination to `00_nucleo/prompts/compiler/layout/grid/cell_emission.md`. Delegate column/row track sizing and its pagination point to `00_nucleo/prompts/compiler/layout/grid/tracks.md`. Delegate header/footer normalization and collision detection to `00_nucleo/prompts/compiler/layout/grid/row_groups.md`, and visual h/v rules to `00_nucleo/prompts/compiler/layout/grid/rules.md`. Delegate row-border coalescing and h/v segment emission to `00_nucleo/prompts/compiler/layout/grid/borders.md`, retaining their invocation order and geometry. Preserve track resolution (fixed/auto/fractional), gutters, per-cell overrides and span measurement, row-group extraction/repetition, h/v rules, fill/stroke and row-border coalescing. Obtain occupied placement from `00_nucleo/prompts/compiler/layout/grid_placement.md`; no second placement algorithm is defined in the hub or cell wrappers.

Measure cell content with injected metrics/styles in the existing finite column widths, apply per-axis inherited alignment and inset, and lay out each cell once in its absolute cell origin/region. Recompose by incremental deltas, keeping parent bbox, cell alignment/context, decorations and orphan fixup paths consistent and restored. Preserve breakable-row splitting, repeated header/footer visual output and deferred cell tails within the source's bounded support; `00_nucleo/prompts/compiler/layout/slicing.md` performs partition/rebase and `00_nucleo/prompts/compiler/layout/cell_tail_commit.md` later emits tails. Table semantic metadata belongs to `00_nucleo/prompts/compiler/layout/table.md`; do not derive it from visual strokes/fills. Unsupported multipage/span/container combinations remain limited rather than silently promoted to parity.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.


## Produto medido consumível — 2026-10-05

Evidência em tracks.rs:175 e cell_body.rs:155 e diagnóstico `00_nucleo/diagnosticos/reuso-sem-transformacao-2026-10-05.md` precede a decisão. Coordenar o produto local devolvido por tracks e encaminhá-lo à emissão; registrar novo child measurement_cache. Não armazenar cache no Layouter nem mover placement antes de medição. Delegar invariantes de cache ao owner canônico `00_nucleo/prompts/compiler/layout/grid/measurement_cache.md`. Mudança privada de mecânica, na mesma fase e fluxo contínuo ADR-0127.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

Manter engine física única; adicionar entrada privada para plano de Table com posições já conhecidas. Este caminho usa plano antes de measurement; Grid legacy conserva placement depois de measurement. Isso supersede apenas para Table preparada a restrição histórica de ordem. Delegar toda ocupação ao owner, sem novo algoritmo no hub. Props conhecidas e roles seguem planos, tracks/fontes/paginação permanecem físicos; não executar Engine/Func nem uma segunda medição para obter show.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.


## Gutter horizontal simples — 2026-10-14

Medição anterior: block-auto-width-2026-10-14/green-first.log conserva o
RED integrado: Block com inset7 e tracks20+30/gutter4 resulta em largura64,
contra68 no SVG pinado em before/grid-vanilla.svg. grid.rs:171 recebe
_gutter mas :217 resolve tracks sem reservar o espaço nem deslocar origens.
A largura160 do backing anterior ocultava essa falta. before-grid-owner
conserva fonte/L0/HEAD/diffstat/UTC antes desta correção.

Resolver gutter Length no estilo atual, reservar (n-1)*gap antes da resolução
das tracks e acrescentar i*gap à origem de cada coluna. A largura de cada
track continua excluindo o gap; span usa as origens efetivas existentes.
Nenhum gap antes da primeira ou depois da última coluna. Frações recebem
o restante após a reserva. Sem novo campo público, default ou fase.
Este adendo corrige somente o eixo horizontal do gutter escalar existente;
o eixo vertical, gutters ricos, regras entre gutters e spans fragmentados
continuam incompletos e serão concluídos na integração de Grid, não são
certificados pelo caso de uma linha. Exigir RED→GREEN sem mudar o esperado,
mais controles de tracks fixas/fracionárias e estado exato.


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

Encaminhar o Layouter ao emitter privado de regras para preparar seus paints antes de criar Shape. Não alterar os traits públicos nem a comparação declarativa de Stroke, tracks ou células.

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

- `C014`: `01_core/src/compiler/layout/grid.rs:164`, `layout_grid`, 13 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C014` — `layout_grid`: Correção interna prescrita

Grid e Table repetem a mesma projeção de doze campos posicionais para um único motor visual. Essa operação tem um conjunto coeso de tracks, conteúdo, regras e defaults; os campos devem ser nomeados no pedido privado, preservando o motor compartilhado.

GridLayoutInput emprestado, privado ao motor grid, com os doze campos nomeados e genéricos H/V preservados; o método recebe self e input. Migrar as projeções Grid/Table sem novo owner de placement.

Aceitação: Grid e Table mantêm tracks/gutters, header/footer, spans, estilo herdado e semântica de tabela; verificar ordem de campos e nenhuma cópia profunda nova.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Aridade final — materialização interna, 2026-10-16

### Medição anterior à decisão

Fontes e L0 vigentes conferidos antes do código; snapshot 00_nucleo/diagnosticos/clippy-aridade-final-2026-10-16/before.json.gz conserva HEAD, working tree não commitado, UTC, status, diffstat e hashes globais. callers.txt registra busca no workspace, incluindo fixtures; as chamadas homônimas do math::MathLayouter são outro motor e permanecem intactas. Inventário anterior tem 22 avisos, cinco destas aridades. Trata-se de estrutura privada Rust, sem nova afirmação de paridade ou intenção upstream.

### Responsabilidade exclusiva e aceitação

Concretizar C014 com GridLayoutInput emprestado, pub(super) somente para Grid/Table, e doze campos nomeados columns, rows, cells, hlines, vlines, gutter, align, inset, header, footer, stroke e fill, com tipos atuais e H/V preservados. layout_grid recebe self/input, desempacota uma vez e mantém o corpo do motor. Grid projeta exatamente seus campos. Sem nova cópia profunda, algoritmo de placement ou nova exportação pública.

Este adendo materializa somente o escopo nominal e supersede seu estado pendente da auditoria. Fluxo contínuo ADR-0127: sem contrato externo, default, fase ou compatibilidade novos. L0 primeiro e resselo restrito. RED de too_many_arguments antes do código, GREEN sem novas supressões ou alteração de limiar; expectativas anteriores devem continuar satisfeitas. Conferir corpos e transporte de valores, preservar fixtures/esperados, sem testes que espelhem carriers. Executar controles existentes de Grid/Table/outline e SVG/tiling antes/depois, suíte workspace completa, build, Clippy all-targets, Rustdoc, formato, linhagem estrita/reversa e Tekt sem novos avisos, sob 12 GiB, swap zero e um job. Resultados não certificam paridade global ou conformidade PDF/UA.
