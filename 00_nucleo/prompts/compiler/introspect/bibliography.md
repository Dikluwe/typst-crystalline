# L0 — introspect/bibliography
Hash do Código: fdbf2d5a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/bibliography.rs`.
No estado `df72a6c4a701917ab56a7c6169764a92ea775693` com árvore limpa, `01_core/src/compiler/introspect.rs:67-286` contém o comportamento que esta unidade recebe. A fonte e o estado estão registrados em `lab/results/construction/diagnostics/atomizacao-introspect-2026-09-30-before.json.gz`. A decisão é separá-lo por comportamento, sem alterar a API ou alegar ganho de desempenho.

## Contrato

Preservar a varredura recursiva de chaves bibliográficas e converter apenas referências cuja chave aparece nas entradas. A transformação mantém a ordem e a estrutura dos demais conteúdos; o walk recebe as citações resultantes.

As invariantes de linguagem, a ordem causal e os limites de paridade continuam definidos em `00_nucleo/prompts/compiler/introspect.md`; este owner não cria uma segunda interpretação. O código usa chamadas estáticas na camada L1, sem vtable, I/O ou estado global mutável.

## Aceitação

Preservar os testes de introspecção e a suíte L1, compilar e passar `crystalline-lint .` com hashes coerentes. Mudanças semânticas futuras exigem medição de linguagem e atualização do L0 antes do código.

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

Varredura/conversão de citações segue a vista visual preparada, preservando fonte/plan/stage e ignorando corpo ocultado; não executar recipe nem perder papel do walker próprio.

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


## Introspecção I1 — carriers e classificações nominais (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/tekt-v16-introspect-carriers-2026-10-17/before.json.gz
preserva UTC/status/diff HEAD --stat, fontes/L0 e 2130 hashes conferidos do
fechamento anterior. contexts-before.json identifica os três V16 vigentes.
before-citation-probe.json conserva argv, rlib/programa com SHA-256, source
freeze e saída: nos 12 casos (quatro carriers × três posições), a ordem k2,k1
da árvore sem wrapper vira vazia; citações conhecidas são perdidas. A coleta
bibliography.rs:21–76 e a conversão em :79–220 não atravessam Source/ShowRecipe/
Located/Realized. Estes são carriers internos sem syntax vanilla; a prova é
metamórfica de transparência já exigida, não comparação de representação Rust.

labelled.rs:45–80 devolve ausência para Source, embora o walk encontre o alvo
interno. Hipótese: sua projeção de compatibilidade perde o texto do mesmo
Heading/Equation/Figure no mesmo Location; o teste RED deve confirmar antes de
alterar esse helper. Seu L0 proíbe desmontar Styled arbitrário; Source não é
Styled, e a decisão anterior é preservada. populate.rs:24–44 satura futuros
ElementPayload em None. O domínio atual tem 19 variantes; as condições bool
ativas devem continuar exigidas. before-probe-result.json registra 290 casos
simbólicos e zero E0004 ao acrescentar variantes fictícias aos dois enums:
proteção estrutural ausente, sem alegar falha numérica atual nem ganho de
leitura por IA. Payloads opacos e guard booleano não provam semântica de dados.

### Responsabilidade exclusiva

I1-carriers: em collect_bib_keys, visitar Source/ShowRecipe/Located pelo body e Realized pelo visual. Em convert_refs, reconstruir esses quatro carriers alterando somente body/visual, preservando span, Location, original e captura de recipe (rules/styles/guards/file/context/textual); não executar receitas. Conservar o fast path vazio e todos os outros braços, inclusive o comportamento Dynamic existente. Acrescentar dois testes locais de ordem/posições e negativos/original/identidade. A travessia está explicitamente INCOMPLETA: o passo I2-bibliography-static-coverage completará a auditoria e os casos faltantes de containers estáticos (por exemplo células raw/headers/footers, Repeat/Columns e outros corpos). Até I2, o wildcard de coleta e o passthrough legado da conversão continuam débito, não recebem silenciamento nem prescrição de terminalidade; substituir apenas o comentário que indevidamente chama todos de terminais.

### Gate e aceitação

Fluxo interno/paridade ADR-0127: sem contrato público, default, fase, I/O,
registro dinâmico ou assinatura nova. L0 precede testes e código. RED funcional
antes da implementação; não usar falhas de montagem de sonda como RED. GREEN
exige as 12 ordens de citação iguais à base, visual de Realized sem contribuições
do original, desconhecidos preservados e fast path sem chaves sem reconstrução.
Label Source deve conservar a projeção direta e o Location/lang, enquanto
Styled e outros alvos continuam com os limites do contrato vigente.

O mutante descartável deve passar de zero para dois E0004 selecionados. Os
290 controles simbólicos mantêm a classificação, salvo os dois do helper de
label que agora reconhece Source. As 17 alternativas negativas de payload
incluem flags recusadas (Equation separa block false de block true/numbering
false); Source não é neutralizado como alvo desconhecido. Notas V19 permanecem
visíveis e justificadas por owner; sem V20 novo ou supressão.

Esperado no SARIF: remover só dois V16 (labelled/populate), acrescentar dois
V19 nominais e preservar o V16 de bibliography::collect_bib_keys até I2, com
os demais diagnósticos intactos por fonte/regra/severidade/mensagem. Não tratar
variantes estáticas ainda não percorridas como terminais legítimos. Replay
permite apenas os reparos prescritos e testes. Grafo V15/V26 antes do resselo
restrito aos três pares; build, testes focais/workspace, Clippy all-targets
-D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito, fmt, linhagem
estrita/reversa, source freeze e diff --check. Pesados sequenciais sob
MemoryMax12GiB/High10GiB/swap0/jobs1. Sem commit/push/merge/release. O objetivo
global e as frentes anteriores continuam ativos; esta etapa não certifica
bibliografia ou referências universais.


### Refinamento I1 — identidades válidas na fixture (2026-10-17)

Medição: located-fixture-attempt conserva o source freeze e o primeiro GREEN:
11 posições coincidem e Located/refs produz só k2. A fixture reiniciava Locator
para cada wrapper, atribuindo a mesma identidade a referências distintas.
Isso refuta a fixture, não autoriza gerar identidades novas na conversão.
Corrigir somente os helpers de sonda/testes: um Locator por árvore, com prefixo
reservado para não coincidir com ocorrências frescas do walk. Source/recipe/
visual e assertivas ficam iguais. Repetir baseline com os algoritmos originais
extraídos de before.json.gz sobre tipos reais da rlib; adaptar só imports para
o executável de sonda. Observar Cite por walker estrito da fixture e, no GREEN,
confirmar também citation_order público. Distinguir essa reconstrução controlada
da medição pública inicial arquivada. Mesmo programa/hashes antes/depois;
enum/modelo, teste de label e código produtivo não mudam por este refinamento.


## I2 — cobertura estática bibliográfica (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/tekt-v16-bibliography-static-2026-10-17/before.json.gz
registra UTC, status, diff HEAD --stat, os textos anteriores e 2130 hashes
conferidos. bibliography.rs:21–79 enumera só parte dos containers e omite
Heading, Repeat, Columns e TableCell crua; :87–266 repete o catálogo parcial.
traversal.rs:337–1105 já possui enumeração estática pré-ordem completa, com
substituição terminal; :89–107 distingue Source, Located e Realized. O mapper
TableCell seleciona standalone.visual, resolved.visual ou body; visual() possui
outra prioridade, esclarecida no refinamento abaixo.
introspect/walk.rs:1206–1268 confirma os corpos de grid/header/footer/cell,
attribution de Quote, title de Bibliography e supplement de Cite; :1396
estabelece Document/Asset como recursos separados. São perdas internas de
conteúdo, não razão para copiar igualdade, bytes ou mecânica upstream.

A hipótese é que os conteúdos estáticos atualmente ignorados perdem chaves e
conversão Ref→Cite. Testes RED devem confirmar antes do código. A hipótese é
refutada se o caminho já conservar a chave/citação; não alterar esperados para
forçar essa classificação. Carriers internos usam a prova metamórfica da I1;
sem inferir paridade universal ou corrigir os limites do walker de math/HTML.
### Responsabilidade exclusiva

Completar I2 por reutilização das travessias do domínio: coleta por visita
emprestada de conteúdo estático, conversão por map_content_pre já existente.
Não manter segunda tabela de reconstrução dos elementos no compiler.
A coleta insere entries de toda Bibliography alcançada e também visita title.
Document/Asset são fronteiras de outro recurso; não vazar chaves nem converter
suas referências usando bibliografia do documento hospedeiro. Dynamic e
ContextBlock são opacos na coleta; não executar callbacks, recipes ou closures.
Não descer em Value, estilos, snapshots de regras ou Realized.original.

Conversão substitui somente Ref de chave conhecida pelo mesmo Cite existente,
com supplement preservado; desconhecidos ficam intactos. Dynamic mantém sua
delegação legada dyn_map_content com convert_refs. Outline conserva título e
corpo já realizado, sem refazer realização nem perder capturas. O resto usa
map_content_pre, incluindo células raw/standalone/resolved, headers/footers,
math e demais containers. Source segue sua normalização causal já ratificada;
Location, Realized.original, planos/props/stage e snapshots são preservados.
Fast path sem chaves devolve a entrada. A classificação aberta de callback
(None significa sem substituição) é contrato de transformação, não default
semântico para elementos desconhecidos; a enumeração exaustiva vive no domínio.

### Aceitação

RED→GREEN de coleta/conversão em containers omitidos, campos opcionais e
seqüências aninhadas; isolamento de Document, origem Realized e raw oculto de
células preparadas. I1 continua íntegra. Conversão desconhecida não cria Cite;
coleta não chama Dynamic; conversão mantém callbacks legados e limites de Ref.
Não certificar que o walker de introspecção já publica eventos de todos os
containers atravessados; medir a transformação e a ordem publicada nos casos
que o walker suporta. Não alterar o walker, API, fases ou pipeline.

### Gate e validação

Correção interna ADR-0127: L0 antes de testes/código e resselo restrito aos
dois owners após grafo V15/V26 válido. Sem campo/assinatura pública, flag,
default ou mudança de fase; sem dependência ou publicação. Não silenciar V16,
V19 ou V20. Justificar no L0 o grupo nominal de terminais da visita (folhas
ou dados/callbacks opacos, sem Content estático ativo). Build, RED→GREEN,
I1, workspace completo, fmt, Clippy all-targets -D warnings e
-D unfulfilled_lint_expectations, Rustdoc estrito/quatro índices, linhagem
V1/V5/V15/V26/reversa, diff --check e freeze das fontes. Pesados seqüenciais,
MemoryMax12GiB/High10GiB/swap0/jobs1. Relatório com estado exato, argv,
hashes e UTC. I2 substitui a dívida de cobertura dos containers estáticos da
bibliografia da I1; demais frentes e objetivo global permanecem abertos.

### Confirmação RED e refinamento de empréstimo

red-result.json / red-state.json.gz registram a execução de 2026-10-17T20:02:52.979326+00:00 a
2026-10-17T20:06:42.623098+00:00: 0 passou, 2 falharam, sem erro de compilação, fontes
inalteradas durante a medição. As 18 posições perdem chave e conversão.
A navegação de OutlineEntry.prefix usa o novo accessor interno prefix_ref
do seu owner, pois prefix() faz clone; não mudar a interface pública.
O grupo nominal de 38 terminais da visita é: Empty, Text, Space, SpaceFromNewline, Parbreak, Raw, MathIdent, MathText, MathPrimes, MathAlignPoint, Linebreak, Ref, CounterDisplay, CounterUpdate, Image, Shape, Curve, SetPage, Flush, Divider, Asset, SmartQuote, PdfAttach, HSpace, VSpace, Pagebreak, Colbreak, GridHLine, GridVLine, TableHLine, TableVLine, Metadata, State, StateUpdate, StateDisplay, CounterDisplayCallback, ContextBlock, Dynamic.
Esses casos não possuem Content estático navegável nesta vista; Ref mantém
supplement opaco como no map_content vigente, valores/estados são dados e
Dynamic/ContextBlock não são avaliados. V19 deve continuar visível e
justificado; novas variantes exigem decisão no match exaustivo.

### Refinamento I2 — vista visual selecionada de Cell

selected-visual-attempt registra o primeiro GREEN: 5 passaram e o controle
de coexistência falhou na coleta (resolved contra a expectativa standalone).
A expectativa confundia a prioridade do mapper genérico com a vista ativa.
Fonte reavaliada: TableCellElem::visual, table_cell.rs:261–268, usa resolved,
senão standalone, senão body; finish_cell_product em realize/table_cells.rs:
1065–1080 publica o corpo realizado em resolved quando presente. O L0 da
entidade §correção de spans descreve coexistência: standalone conserva
propriedades observadas enquanto resolved mantém o plano ativo. A cláusula
de prioridade standalone do mapper é outra operação, não redefine visual().
A medição anterior que atribuía essa prioridade ao visual estava incorreta.

Testar separadamente resolved-only, standalone-only e coexistência. O ativo
é resolved quando existe, senão standalone; manter a origem raw e o carrier
inativo intactos. Registrar novo RED por Ref ainda presente no visual ativo,
sem mudar nenhum teste antigo de mapper nem corrigir seu contrato neste lote.
A conversão recebe arm específico de TableCell: copiar só o carrier ativo,
converter exclusivamente seu visual (ou raw se nenhum preparo), preservando
campos, presença, stage/guards/props e o outro carrier. Coleta usa visual()
existente. Nenhuma mudança em visual(), fases ou API. O desacordo dos
mappers genéricos com a vista ativa fica registrado para auditoria própria;
não usar a fixture anterior para mudar o produto.
