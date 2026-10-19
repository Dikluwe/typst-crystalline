# L0 — compiler/eval/rules/realize — Realização de parágrafos
Hash do Código: 2685624e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:f225f3039d4663e9bfcb408d26cf1cd6e557f089013bc2d95090ce16c52080c6
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:7e41c96527dd5936bebe6916f2a9ea93477c057495845c7e8b52ce3c5917855a
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0


Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/realize.rs`

## Medição antes da decisão

A working tree não commitada capturada em `lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz` contém as rotinas desta família em `01_core/src/compiler/eval/rules.rs`. A suíte L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem prova bilateral nova contra o vanilla. As medidas históricas e ressalvas do pin continuam em `00_nucleo/prompts/core/compiler/eval/rules.md`.

## Contrato de linguagem

Realizar fluxo em nós Par antes de show quando aplicável; preservar scoped flow, estilos, containers, matemática e folhas sem duplicação ou quebra de morfologia. A geometria pertence ao layout.

As assinaturas, identidades, casts, defaults, precedência de erros e fase de avaliação observados permanecem. O parent mantém diagnóstico compartilhado e bindings estáticos; outros owners de selector matching, terminação e textual realization continuam exclusivos. Não duplicar esses algoritmos.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. Paridade em semântica, sintaxe e morfologia (ADR-0107/0108), ownership exclusivo e pins válidos (ADR-0129/0130).

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

Transformações de parágrafos/containers conservam novos campos/plano/etapa e respeitam fronteira de corpo ocultado; não construir entidade perdendo causalidade. Não substituir aplicação root-only Table/Cell do owner apply.

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

workspace-seventh/composition_show_par refuta Source como fronteira de scope/flow: classificar Source.body em Par/carriers e scoped prefix, transformar flow/inline/nó conservando origem. Não criar segundo parágrafo ou permitir estilo interno vencer show-set externo por wrapper novo; matemática segue seu owner.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## R3-T1 — fronteira de estilo através de transporte privado

### Medição anterior à decisão

workspace-rest.log, candidato7-sources.json.gz e workspace-validation-resources.json
em lab/results/construction/diagnostics/r3-t1-materializacao-2026-10-18 conservam HEAD b1b486f3, working
tree não commitado, UTC, fontes/diffstat e comandos sob 12 GiB. O controle
composition_show_par_overrides_outer_set_with_page_configuration produz quatro
palavras hifenizadas em vez de zero quando text(hyphenate:true) cobre o fluxo
inteiro sob show par:set text(hyphenate:false); a forma inline deve continuar
hifenizando. O packet de inserção esconde Styled da classificação whole_style
em realize_flow:47–51. A regra de precedência vigente não mudou; perdeu-se a
fronteira já reconhecida pelo owner antes do novo transporte R3.

### Decisão e responsabilidade exclusiva

O núcleo R3 pinado preserva raiz/produto e estilos sem executar packets nesta
unidade. Um helper puro flow_boundary_body atravessa Source e ShowRecipe somente
para reconhecer fronteiras Styled de fluxo. realize_flow usa a vista nos
predicados scoped_prefix/whole_style; realize_scoped_flow reconstrói ShowRecipe
com o mesmo snapshot/guards/arquivo e corpo preparado recursivamente, sem executar
regra ou remover o packet. is_paragraph_node reconhece Par através desses carriers.
Não preparar indiscriminadamente o corpo de ShowRecipe em realize_node; Sequence
escrita pelo autor ainda precisa da decisão de sua raiz pelo owner apropriado.
O helper tem complemento Content nominal: todo tipo que não seja Source/ShowRecipe
encerra a busca; grupos V19 ficam visíveis, sem prova fictícia de cobertura.

### Aceitação

O RED existente deve passar sem mudar expectativas de zero no fluxo inteiro e
hifenização no inline. Revalidar controles Par/Sequence, todo núcleo e integração,
PDF/HTML e geometria congelada, Clippy/Rustdoc estritos e linhagens. Sem contrato
público novo, alteração de pipeline adicional ou supressão de lint. Este reparo
materializa o transporte e a ordem já ratificados pelo manifesto R3-T1
47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302.
