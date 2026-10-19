# L0 — entities/content/page-structure
Hash do Código: d7529c8f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0


**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/page_structure.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own structural page and column segmentation and `wrap_page_columns`.
Preserve source order, nested content, page markers, column markers, page
configuration, and degenerate-case normalization. The behavior is pure and
must not perform layout, measurement, rendering, or I/O.

## Acceptance

Existing page/column tests pass unchanged and the module includes local
tests.


## Realização com avaliação única

Segmentação de páginas conserva os carriers de ocorrência como unidades causais; não descarta identidade nem reemite origem consumida.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/
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

Preservar planos/etapas/fontes ao segmentar página/coluna; descer apenas na contribuição visual efetiva preparada onde houver descida, sem reemitir body original.

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


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Source é transparente nesta projeção, delegando ao body; não expõe span como identidade de língua nem executa receitas. Segmentação conserva origem por segmento quando aplicável.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Histórico: caminho sem colunas — 2026-10-09 (substituído abaixo)

Medição anterior à decisão: `lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/table-map-passes-before.json.gz` atribui uma reconstrução da Table a `wrap_page_columns` para cada fixture de 100/1.600 células, sem `page(columns:)`. `page-column-fast-path-red-gate.json.gz` registra HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado/status/diffstat, UTC, release `702060185fdc0533de86e7bf00c107e26fd0d281170ccd9c76d12b91e4e46883`, script/interpositor/fixtures/comandos: 512.679 pedidos glibc em `local-set-1600`, acima do gate 500.000. Fonte `page_structure.rs:185-245` chama `map_content` em toda a árvore antes de saber se alguma Sequence contém `SetPage` com `columns: Some`; o callback retorna `None` nos casos medidos. Inferência refutável: um pré-exame emprestado que prove ausência do gatilho elimina esta reconstrução; qualquer contêiner desconhecido deve ser tratado como potencial gatilho, nunca pulado.

Decisão interna de fluxo contínuo (ADR-0127): antes de `wrap_page_columns`, detectar a ausência de `SetPage { columns: Some(_) }` com descida emprestada no enum para os contêineres conhecidos, incluindo Table/Cell preparada; em variante não coberta, executar o caminho antigo. Quando há gatilho, manter a transformação antiga integralmente. Nenhum contrato público, representação, vtable, configuração padrão ou fase muda. Aceitação: teste RED→GREEN para a prova conservadora e casos com/sem colunas, gate de alocações, tempo pareado contra release anterior/vanilla pinado e qualificação de render/query. Retirar a otimização se houver divergência ou regressão de tempo.

## Travessia única após propagação de ausência de mudança — 2026-10-10

### Medição anterior à decisão

`lab/results/construction/diagnostics/content-preflight-removal-2026-10-10/{before,measurements,confirmation,allocation-red,allocation-green,validation}.json.gz` registra HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, status/diffstat, fontes, UTC, comandos, fixtures e binários. Release anterior `2d7f5dc93f6a7f15c81ff93b74270c80a14077d6753c8f79c73c9575d113d5f1`; candidato `9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2`; oracle vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`/binário `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. O snapshot before preserva as guardas emprestadas; entities/content/traversal.rs:24–82 fornece a prova explícita no mapeamento. Remover as duas guardas reduz tempo pareado de blocks-400 em 1,21% nas duas coletas, seis rodadas com medianas negativas; seis casos mantêm a contagem de alocações. 34 controles de texto/geometria/raster e consultas foram preservados; a suite core permanece verde. Matemática/blocos repetidos continuam fora de comparação de trabalho equivalente com vanilla.

### Decisão interna vigente — substitui os pré-exames acima

Usar diretamente `map_content` na operação desta unidade, sem pré-exame emprestado duplicado. O protocolo de ausência explícita de mudança pertence ao Núcleo content/map-content-change pinado; manter o callback existente e suas fronteiras, ordem, erro, origem, localização e transformação. Não inferir mudança por igualdade, hash ou ponteiro; não ampliar o universo de variantes nem trocar enum por despacho dinâmico. Source continua transparente e conserva a primeira origem conforme o núcleo vigente. Nenhum contrato público, comportamento padrão, fase ou compatibilidade é alterado.

### Aceitação

Conservar controles com/sem coluna, ocorrência já localizada e pendente, origem, efeitos e query; testes core, build e lint. Gate quantitativo externo de tempo pareado passa do baseline sem ganho ao candidato com benefício repetido e sem regressão robusta, sem aumento de alocações. Reuso de Arc é prova auxiliar de engenharia; aceitação de linguagem se dá por texto, geometria, morfologia e efeitos, sem exigir bytes de PDF iguais. A melhoria é pequena e não certifica paridade global nem redução de memória residente.
