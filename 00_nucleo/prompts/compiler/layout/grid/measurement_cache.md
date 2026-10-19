# L0 — grid/measurement_cache
Hash do Código: fd7b4dc0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/grid/measurement_cache.rs`
**Governança:** ADR-0107/0108/0109/0127/0129/0130.

## Evidência antes da decisão

`grid/tracks.rs:175` constrói o subframe de altura Auto e descarta todos os itens; `grid/cell_body.rs:155` os reconstrói na emissão. `grid.rs:218–239` mede antes de placement. O diagnóstico `lab/results/construction/diagnostics/reuso-sem-transformacao-2026-10-05.md` mediu diferenças de contexto e documentou a remoção histórica do cache P234 por reordenação. A etapa layout-inicial removeu um relayout completo, mas não esse par medição/emissão. O baseline e as fontes atuais estão preservados em `lab/results/construction/diagnostics/frames-celulas-2026-10-05/before-state.json.gz`. Fonte `paragraph_plan.rs:48–56` recusa planejamento em subframes, e `line_commit.rs:493` não consome contador raiz dentro de subframe. `word_layout.rs:21–55` emite uma palavra mantida sem callback/identidade; `sub_frame.rs:314–346` devolve produtos locais e restaura estado pai, preservando last_sub_frame_width/bottom como resultados laterais. Isso motiva um experimento limitado; não prova antecipadamente ganho.

## Unidade e fronteira

Possuir um cache consumível de produtos medidos, privado a uma invocação de grid. Despacho estático descendente, sem campos novos em Layouter/entidades, sem I/O, unsafe, globals ou retenção entre layouts/compilações. Não usar Hash/PartialEq visual de Content, endereço cru nem gerar IDs. Grid coordena o produto, tracks o captura após a medição e cell_body o consome antes de instalar região da célula; os owners delegam as regras a esta unidade.

Aceitar somente token ASCII gráfico não vazio (0x21–0x7E; sem espaços ou controles) em Text (inclusive sob Source), Sequence com um único material Text e Empty neutro, e wrappers GridCell/TableCell de placement sequencial sem alinhamento/coords/spans especiais. Não aceitar Styled/Realized/Located/contexto/qualquer locatable nem outros carriers. Todos os cells do grid devem ter placement simples (x/y ausentes, spans ausentes ou 1); sem isso, não construir cache. Linhas Fixed/Fraction não são capturadas.

Condições ambientais de captura/consumo: fora de subframe/célula/column mode/page columns, sem smallcaps, floats pendentes, transação de sufixo ou checkpoint pendente, sem par.line.numbering ativo, e alinhamento de célula ausente. Guardar o estilo pai, margens laterais e presença do coletor de decoração na criação do cache; rejeitar mudança na captura/consumo. Comparar o TextStyle efetivo atual com o TextStyle emitido medido (igualdade de dados mecânicos de render, não identidade de linguagem).

O registro precisa identificar row/col de entrada, origem x e largura de corpo finitas/positivas, correspondência do token, altura e resultado completo do subframe. Capturar somente resultado de uma única folha FrameItem::Text cujo texto é o token e largura lateral last_sub_frame_width cabe na região (tolerância exclusivamente classificatória de 1e-9 pt para cancelamento em x absoluto, sem alterar geometria), sem alinhamentos órfãos. Conservar decoração e last_sub_frame_width/bottom. No consumo exigir row/col e spans 1 do PlacedCell, mesma origem e largura, mesmo token/estilos/ambiente. Consumir no máximo uma vez, retirando a entrada; divergência ou ausência executa o layout anterior. A pool usa índice derivado de row/col apenas sob essa prova de placement, nunca como chave geral.

Ao reutilizar, repor last_sub_frame_width/bottom e devolver o produto para o pipeline de emissão vigente: insets, rebase y, decoração, fill/stroke, clipping, overflow e caudas continuam nos owners existentes. Não emitir diretamente fora deles. O cache não recupera regiões diferentes por tradução ad hoc de x.

## Aceitação

RED→GREEN: duas células Auto de um token requerem dois subframes totais, não quatro; conteúdo e métricas laterais preservados. Fixed rows, alinhamento, posicionamento explícito e conteúdos com identidades conservam fallback. Cobrir multipágina, insets, RTL, estilos, overflow, fontes e diagnósticos com controles existentes e sondas adicionais. Captura/consumo provados pelos testes e contagens externas; ganho decidido por CLI pareada com fontes/binários/estado/timestamps. Falha de saída ou ausência de benefício refuta o experimento e deve ser documentada. Ampliações exigem nova prova, não variantes adicionadas por conveniência.

O cache é interno na mesma fase, sem API externa/default/compatibilidade: fluxo contínuo ADR-0127.

Refinamento medido na sonda: para Beta/FixedMetrics, last_sub_frame_width foi 26.400000000000006 e body_w 26.4; o critério estrito descartava o produto por cancelamento de f64 em cursor_x − origin_x. A tolerância classificatória acima permite esse carrier, mas origem/largura de entrada e o produto continuam exatamente iguais. A revalidação em `lab/results/construction/diagnostics/frames-celulas-2026-10-05/width-probe.json.gz` conserva esses números, a sonda cfg(test) e seu estado exato; o log discovery original permanece exploratório. Recusar ambiente matemático também conserva o fallback.

Provas locais do helper: origem ou largura diferentes invalidam e consomem a tentativa do registro, sem hit posterior; mudança de tracking no canal custom da chain altera o TextStyle efetivo e exige fallback mesmo quando o estilo pai achatado não mudou. Esses testes complementam a integração de grid e legitimam a guardas mecânicas, não paridade por igualdade de Content.

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

Plano preparado não prova elegibilidade sozinho. Conservar cache para o subcaso comprovado de coords sequenciais unit-span e visual ASCII token, mesmos styles/insets/origins/métricas e sem metadados/locatables/contexto/efeito; medir/consumir uma vez. Callbacks ou mudanças de corpo/ambiente forçam fallback. Acrescentar prova para a nova representação sem recertificar por igualdade de Content nem relaxar controle antigo de dois subframes; sem cache fora da invocação.

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

## Prioridade 5 — pontuação ASCII, fluxo contínuo ADR-0127

### Medição anterior à decisão

lab/results/construction/diagnostics/celulas-prioridade5-2026-10-07/before.json.gz conserva HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC/diffstat/fontes de
código e binário fc5b2571. audit.log e audit-test.rs.txt conservam a sonda: Alpha,
12.34, item-2, abc_def e a/b produzem uma folha Text de mesmo texto; pontuação é
rejeitada apenas pelo predicado alfanumérico atual. CJK produz mais folhas e texto
com espaço produz três itens. word_layout.rs:21-66 faz Keep/Wrap/Hyphenate e devolve
Text; word_segment.rs:28-45 reconhece scripts segmentados. sub_frame.rs:314-346
restaura estado e devolve produto; paragraph_plan.rs:48-64 recusa subframes.
É custo/mecânica de execução; texto, efeitos e geometria são observáveis de língua.
Não há inferência de intenção upstream nem nova paridade global.

### Decisão interna e aceitação

Ampliar apenas o predicado do mesmo carrier Text para ASCII gráfico não vazio.
Todos os demais guards, wrappers, placement, estilo efetivo, itens/altura/laterais,
origem/largura, subframe/célula/colunas, efeitos/locatables/contexto, decoração e
consumo único permanecem. Exigir ainda uma única folha Text cujo texto é exatamente
o corpo integral. Wrap/hifenização/segmentação que altere ou multiplique produtos
continua recusada. Não aceitar Unicode, Styled/Realized/Located ou espaços/controles
neste lote, mesmo quando uma sonda individual parece simples. Não modificar a
política global de texto, quebra de linha, rendering ou fonte.

Hipótese de ganho refutada por geometria/tinta/efeitos diferentes, guardas relaxadas
ou nenhum benefício repetido. RED→GREEN no owner: decimais/IDs com pontuação podem
ser consumidos uma vez preservando métricas; controles/Unicode/carriers compostos
continuam recusados; produto que não cabe ou se transforma continua fallback.
Contar subframes no release separadamente de tempos; medir CLI pareada contra o
fc5b2571 congelado com corpus/table/grid/RTL/insets/páginas/show/fontes/negativos.
Resselo somente deste owner, build/lint/suite pertinente e proveniência completa.
Nenhuma assinatura pública, modo padrão ou fase muda; não há gate público novo.

### Revalidação do carrier causal Source

A primeira versão ASCII gráfico ca7b2257 / release
f2eea959438e8b4a626abd4d04578ca667089ab537d8f390bf2eae63204662b9
conservou 640 subframes tanto em células literais quanto interpoladas, inclusive
no controle alfanumérico: count-before-source.json.gz. carrier.log inspeciona a
primeira célula real: Source(Text("item-0.25")); inspect.log registra pool habilitada
com 320 entradas e produto de uma folha Text. Esses recibos pertencem à árvore
final-state.json.gz anterior à extensão Source, não são prova do candidato final.

01_core/src/compiler/layout/mod.rs:1158 delega Source diretamente ao body sem
estabelecer estilo, identidade, bidi ou contexto.
01_core/src/entities/content/source.rs:11-12 remove apenas wrappers Source para
classificação; source-origin.toml declara transparência e ocorrência única.
É transporte causal interno, não elemento novo da língua. A hipótese de que o
wrapper bloqueia o cache será refutada se a contagem não diminuir após a extensão.

Classificar placement, coordenadas de Cell preparada e token através de
source_body(), sem alterar/remover o carrier original, span ou produto. Em todos
esses pontos remover apenas Source, nunca Styled/Realized/Located/ShowRecipe.
Source sobre conteúdo inelegível continua inelegível; Source sobre posição/span
especial deve conservar as recusas do conjunto inteiro. A origem não é chave de
cache e não é projetada para outro corpo: a pool continua pertencendo à mesma
ocorrência medida/emitida e ao mesmo grid, sem retenção entre compilações.
Exigir RED→GREEN para Source simples, recusa de Source(Styled) e de placement
especial, além de recontagem/tempos e preservação de texto/efeitos/geometria.
Esta correção de classificação interna segue ADR-0127 contínua, sem mudar os
contratos de origem ratificados no C2.

## Escopo local de Cell sem efeito textual — 2026-10-09

### Medição antes da decisão

`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/stacks-current.json.gz`
registra HEAD, árvore não commitada, diffstat/status, UTC, fontes, binário e
interpositor. Em 1.600 Cell, a amostragem de alocações sob
`style_scope::layout_styled` é 5.435 no `local-set` e 1.471 no
`block-no-set`; `FontBook::select_pattern` dentro desses escopos é 1.597
contra 477. São contagens inclusivas, não tempo exclusivo. A fonte atual
`measurement_cache.rs:60-90` recusa todo `Styled`, mesmo quando a Cell já
foi preparada e o wrapper transporta apenas `cell.*` e
`eval.table.local.*`. `style_scope.rs:24-61` empilha esse delta; na folha
Text, esses nomes não alteram o estilo textual. A inferência de que o cache
perde o produto medido neste caso será refutada por ausência de hits ou
ganho após a mudança. O padrão visual e os efeitos continuam a ser medidos
nas 23 fixtures do diagnóstico, não presumidos pela forma do wrapper.

### Decisão interna

Na prova privada de `token`, atravessar `Styled` somente se todos os campos
tipados do delta estiverem ausentes, houver ao menos uma chave custom e cada
chave pertencer a
`cell.*`, `eval.table.local` ou `eval.table.local.recipe`. O corpo ainda
precisa ser um único token ASCII gráfico e o produto medido uma única folha
Text idêntica. No consumo, comparar o TextStyle efetivo com o da folha;
estilo textual, propriedade desconhecida, múltiplos itens, locatable,
alinhamento e todas as outras guardas existentes mantêm fallback. Não
normalizar ou apagar o Styled no documento; reutilizar somente o produto
medido dentro da mesma invocação de grid. A prova não se estende a wrappers
com `text.*`, `par.*`, `raw.*` ou campos tipados, nem a outras fontes de
efeitos. Não mudar API, default ou fase do pipeline; ADR-0127 contínuo.

### Aceitação

RED→GREEN em teste do owner para wrapper permitido e recusas; gate externo
de alocações predefinido e recompilação pareada Tekt/vanilla para
`local-set`, `block-no-set` e `no-fill`, incluindo outputs das 23 fixtures.
Se a elegibilidade não reduzir trabalho ou alterar conteúdo/geométrica/raster,
retirar o experimento e registrar a refutação. Build, suites e lint após
qualificação, com proveniência do estado exato.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T134: 01_core/src/compiler/layout/grid/measurement_cache.rs:69; T135: 01_core/src/compiler/layout/grid/measurement_cache.rs:80; T136: 01_core/src/compiler/layout/grid/measurement_cache.rs:83. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Nos braços Text, GridCell e TableCell de token, mover somente os critérios de elegibilidade ao corpo e devolver None quando falham. Não usar then_some com recursão eager. Preservar ASCII gráfico não vazio, simple_placement antes de align, e visual resolvido somente quando seu align também está ausente. Não ampliar o cache.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.
