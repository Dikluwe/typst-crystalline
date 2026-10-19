# Plano vivo de diferenças e decisões — Tekt × vanilla

Este inventário consolida o plano autorizado pelo usuário em 2026-10-06.
É diagnóstico/processo, não L0 nem autorização de mudança pública. O alvo é o
pin upstream `9f2b6e8715237cb086899a42873660fe744622e8`, com proveniência P1489.
As coletas recentes usam HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b` e working
tree não commitada; listas exatas de alterações, UTC, hashes e comandos estão
em `plano-debitos-2026-10-06/{before,math-before}.json.gz`. Resultados de rodadas
anteriores mantêm seus binários/escopos próprios; não são somados.

## Classes usadas

- **Decisão deliberada:** mecanismo escolhido e mantido; diferença de estrutura
  não constitui falha linguística nem custo excessivo demonstrado.
- **Débito comprovado:** diferença linguística medida ou trabalho redundante
  comprovado; desempenho só é débito quantificado quando houver ensaio válido.
- **Limite certificado:** implementação deliberadamente restrita ao subconjunto
  provado; ampliar exige novos testemunhos, não remover guardas por conveniência.
- **Experimento:** proposta/protótipo sem decisão produtiva nem ganho comprovado.
- **Lacuna de cobertura:** ainda não há evidência suficiente para afirmar match
  ou débito. Não chamar ausência de medição de incompatibilidade comprovada.

Relatórios históricos que dizem “aceite” não comprovam ratificação humana por
si só. Aprovação da ordem geral do plano também não ratifica APIs concretas
que ainda não tenham sido apresentadas.

## Decisões estabelecidas e mantidas

| ID | Decisão | Evidência/estado | Consequência para o plano |
|---|---|---|---|
| A01 | Rust seguro, enum fechado, despacho estático e código explícito | Preferências humanas registradas em herancas-reescrita-frentes-2026-10-05.md | Não propor unsafe, vtables ou macros para economizar escrita |
| A02 | L1 puro; apresentação em L2; recursos em L3; composição em L4 | CLAUDE.md e ADRs vigentes | Medir sem dissolver as camadas |
| A03 | Paridade de semântica/sintaxe/morfologia, com pin fixo | ADR-0107/0108 e P1489 | Estruturas e bytes internos diferentes não são débito por si |
| A04 | Avaliação única com realização pura de receitas | dupla-avaliacao-implementacao-2026-10-05.md, gate registrado | Não reintroduzir segunda execução global para resolver identidade |
| A05 | Identidade original/visual/localização explícita | Mesma implementação e reparo de show por ocorrência | Aliases conservam ocorrência; conteúdo novo recebe identidade própria |
| A06 | Índice realizado conservado no Module e invalidado por setters | memoizacao-indice-2026-10-05.md, gate registrado | O snapshot não é cache global entre edições |
| A07 | Primeira paginação reutilizável sob prova positiva | layout-inicial-2026-10-05.md | Contexto/páginas desconhecidos conservam convergência |
| A08 | Produto Auto de célula reutilizado em recorte restrito | frames-celulas-2026-10-05.md | Compatibilidade de região/estilos/contexto precisa ser demonstrada |
| A09 | Caches contextuais compartilham métricas/reconciliação/bidi na compilação | contexto-cache-fontes/reconciliacao/bidi, 2026-10-05 | Não afirmar retenção entre Worlds/watch |
| A10 | Origem bidi completa, fonte efetiva e Source causal explícitos | Relatórios e manifestos ratificados de bidi/fonte/Source | Não inferir identidade por família, nome ou conteúdo textual |
| A11 | Errors precedem warnings; ordem interna dos avisos preservada; query tem hint específico | ADR-0045 adendo 2026-10-06, gate confirmado e medição atual | Experimental HTML precede avisos do documento; depreciação query fica por último |
| A12 | PDF verboso como default; compacto explícito; HTML target separado | ADR-0126/0128 | Não mudar defaults para acelerar a medição |

## Débitos e próximos passos

| ID | Classe e evidência | Estado | Owner(s) L0 ou ponto de investigação | Critério de saída |
|---|---|---|---|---|
| D01 | CLI aceita stdin sem OUTPUT e deriva -.pdf | Guarda corrigida e validada; usage bruto ainda diverge | 00_nucleo/prompts/shell/cli.md; 00_nucleo/prompts/wiring/tests/cli.md | Rejeitar antes de I/O, status 2, output flag/positional e defaults físicos preservados |
| D02 | Compile/query ordenam warnings antes de errors; query usa hint genérico | Corrigido no pacote confirmado; controles diretos coincidem | 00_nucleo/prompts/wiring/compile.md; 00_nucleo/prompts/wiring/query.md; 00_nucleo/prompts/shell/cli.md | Decisão nova ADR-0045 e API L2 ratificada; stderr bilateral por cenário |
| D03 | Query stdin perde Source e bloco causal de erro | Callback direto/aninhado, Source e import corrigidos por query compilada | 00_nucleo/prompts/wiring/query.md | for_stdin e pipeline paginado existente; erro/trace/import e contribuição contextual resolvíveis |
| D04 | Função math.equation não é reconhecida no seletor de eval | Corrigido e validado no recorte direto/alias | 00_nucleo/prompts/compiler/stdlib/foundations/selector.md; 00_nucleo/prompts/compiler/stdlib/structural.md | Query/selector/alias funcionam por identidade, sem novo enum ou execução de constructor |
| D05 | math.equation.where rejeitado como não-elemento | Corrigido; 29/29 controles e regressão 36/36; ver equation-where-execucao-2026-10-06.md | 00_nucleo/prompts/compiler/eval/bindings/value_methods/element_where.md; 00_nucleo/prompts/compiler/eval/selector_matching.md | Campos/locatability/matching corretos; aliases não dependem de nomes |
| D06 | Math bare é emitido como texto no fallback; falta Equation automática consultável | Preparação/identidade/show/escopo corrigidos após gate; resíduos de render/math label separados em math-bare-execucao-2026-10-06.md | 00_nucleo/prompts/compiler/layout/equation.md e realização | Medir estilo/escopo/show/identidade; especificar realização e verificar gate de fase antes do código |
| D07 | Corpo consultado de matriz perde nome/rows/cells de linguagem | Corrigido; resíduos de sintaxe e spans também revalidados | Getters, identidade, serialização, namespace e âncoras | 29/29 + 14/14 na coleta final-residuos-locais; relatório math-residuos-execucao-2026-10-06.md |
| D08 | table.cell: query/selector/show seguem caminhos distintos | D08A/B1/B2/B3-A/B3-B/C/C2 materializados nos recortes medidos | C2-R1 tamanho/overlap100 corrigido; C3 misto6.04996pt e resíduos de métricas Styled permanecem | table-local-c2r1-execucao-2026-10-07.md; render C3/métricas separados da prioridade2/fontes |
| D09 | Origem de Content constante em certos erros de realização | Evidência histórica; reproduzir caso mínimo no binário novo | Show/realização/spans | Fonte causal real em mensagem/range/trace sem trocar o Source por nome |
| D10 | Watch recria World em cada run_compile_observed | Aquisição por ciclo confirmada no binário; baseline e recuperação medidos em watch-retencao-2026-10-07.md; ganho de retenção ainda não medido | 00_nucleo/prompts/wiring/watch.md; 00_nucleo/prompts/wiring/compile.md; system-world/cache/fonts | Retenção + invalidação correta, recuperação, publicação atômica, tempos e memória no mesmo processo |
| D11 | Memoização incremental abrangente entre edições ausente/incompleta | Inventário e prova de invalidação pendentes | Eval/layout/pipeline e World | Chaves/dependências/lifetime/hits conhecidos; conteúdo/estilo/contexto/recurso invalidam corretamente |
| D12 | Casos de layout/células ficam fora dos reúso existentes | Limite certificado; ganho de ampliação não provado | initial_layout_independent e measurement_cache | Contar misses/guardas/trabalho, validar efeitos, comparar tempo/memória |
| D13 | Aquisição seletiva de fontes e cobertura por intervalos | Ambas materializadas/validadas no escopo medido | coverage-ranges-execucao-2026-10-07.md e catalogo-seletivo-execucao-2026-10-07.md | 1.140 faces equivalentes; ganho incremental repetível na aquisição seletiva. Não fecha render ou garante aceleração universal |

D06: os primeiros controles #math.mat(a) e #math.frac(1,2) da coleta before
não servem de testemunho de renderização válida: a é variável desconhecida e
inteiros não são Content nesses construtores. Foram conservados como controles
de rejeição; math-before substitui-os para a análise positiva por argumentos
Content válidos. A diferença de span do argumento inválido frac permanece
registrada como achado adicional, sem ser misturada à realização automática.

D08: a hipótese antiga de “aceitar table.cell na query” foi refutada. O pin
rejeita query(table.cell) como cell is not locatable, aceita selector(table.cell)
e aceita show table.cell. Não implementar aceitação indiscriminada nem ampliar
o enum público a partir de uma descrição incompleta do débito.

O inventário inicial de chaves/proprietários/lifetime/invalidação de D10–D12
está em `plano-debitos-2026-10-06/cache-inventory.md`. Ele registra também o
unsafe encapsulado herdado dos caches de face, sem introduzir novo unsafe ou
atribuir defeito de memória por inferência.

## Reparos anteriores que não devem reaparecer como falha original aberta

Contador Page atual/final, recursão show por ocorrência, paths físicos dos
diagnósticos, identidade de documento/expressão/stdin em eval, carets de **/__;
fonte efetiva e eixos variáveis nos recortes; bidi/contexto/ocorrência e espaço
terminal; Block/Align/Grid/Table e cantos medidos; stroke:none e headings HTML;
compile stdin com saída explícita e output stdout. Seus relatórios registram
escopo e controles. Isso não é certificação universal dos respectivos domínios.

## Lacunas de cobertura, sem decisão de defeito ainda

Parser/sintaxe ampla; biblioteca padrão inteira; layout complexo com floats,
footnotes/colunas/spans; OpenType/shaping amplo; matemática inteira; SVG/PNG
como targets completos; HTML/bundle amplos; packages/bibliografia/imagens/plugins
e variantes de watch. Preparar testemunhos representativos antes de classificar.

## Ordem autorizada do trabalho

1. Manter este inventário reconciliado com a última evidência de cada item.
2. Fechar D01 e avançar D02–D09 pelos caminhos independentes; confirmar os
   contratos concretos sujeitos à ADR-0127.
3. Medir D10 em ciclos de edição no mesmo processo, antes de desenhar retenção.
4. Inventariar D11 por unidade cacheada, proprietário, chave, dependências,
   lifetime, invalidação e memória; evitar confundir tracking com cache hit.
5. Medir D12/D13 por guardas/custos, preservando o fallback certificado.
6. Fazer rebaseline macro/micro no mesmo estado/binários/ambiente, separando
   correção, trabalho eliminado, tempo e memória. Não somar ganhos históricos.

A cada materialização: L0 vigente primeiro, testes RED→GREEN, linhagem dos
owners alterados, build/lint e revalidação bilateral pertinente. Gate concreto
confirmado de D02 em `00_nucleo/materialization/warnings-cli-2026-10-06.md`.
Nenhuma API de watch ou fase matemática foi escolhida ou implementada neste
inventário. A ratificação humana das preferências não prova desempenho.

## Entrega desta etapa

D01: a guarda de output obrigatório foi corrigida e passou em compile/c, formatos
medidos e na extensão implícita do Tekt; output flag/positional e o default de
arquivo físico foram preservados. **O stderr integral ainda diverge no usage**
por árvore de opções/superfície diferente. A forma implícita também não existe
no oracle. Estes limites não foram classificados como igualdade bilateral.

D04: query(math.equation), alias e selector(math.equation) agora coincidem
integralmente nos controles. A tabela de função→tipo foi ampliada por identidade
estática e bridge interno; sem API pública nova, enum novo ou fase alterada.
Naquela coleta, D05–D08 continuavam negativos explícitos; D05 foi encerrado pela atualização final abaixo. D02 foi materializado após confirmação
da função pública L2 e da nova ordem ADR-0045. D03 corrigiu Source de stdin direto/import e callback contextual por query
compilada após gate confirmado; a coleta atual dos originais é 23/23.
A coleta warnings-cli-2026-10-06/after.json.gz coincide integralmente em 20
dos 23 controles dessa frente; os três resíduos são separados no relatório
warnings-cli-execucao-2026-10-06.md, sem percentual global de paridade.

O relatório de execução está em
`00_nucleo/diagnosticos/plano-debitos-execucao-2026-10-06.md`.
A próxima medição de D10–D13 e do rebaseline está especificada em
`plano-debitos-2026-10-06/proxima-medicao.md`; ainda não foi executada.

## Atualização — pedido de corrigir os três resíduos

Entrada query inexistente foi corrigida e coincide nas formas relativa, ./ e
absoluta em query-residuos-2026-10-06/after-missing.json.gz. Callback contextual e seletor composto foram materializados após confirmação
do gate query-compilada-2026-10-06.md: compilação paginada antes da consulta
e novas fachadas públicas tipadas.
Medição adicional comprova metadata contextual ausente, contexto aninhado e
precedência documento inválido/selector inválido. O caminho existente de eval
--in já reproduz callback e .where(level) corretamente; não criar outro ciclo.

## Atualização final — query compilada ratificada

query-compilada-execucao-2026-10-06.md registra aprovação, materialização,
reparos de descoberta de context aninhado e transporte de label, e checks.
Os 23 controles originais coincidem. Os controles ampliados revelaram débito
adicional independente: aspas de markup já são realizadas como Text no Tekt,
mas a linguagem vanilla conserva SmartQuote no corpo consultado. Query e
**eval --in existente** mostram a mesma diferença; os dois negativos continuam
explicitamente em query-compilada-2026-10-06/delivered.json.gz. A filtragem
.where(body) funciona e preserva o corpo disponível. Não se certifica paridade
morfológica de aspas ou biblioteca inteira; medir L0/realização/serialização
dessa unidade antes de decidir correção de representação/fase.

## Atualização — aspas medidas e gate preparado

smartquote-execucao-2026-10-06.md registra as sondas de morfologia, campos,
reuso com idioma/disabled, contexto numérico, show e HTML. Nome público
smartquote e seletor show da função nativa/alias foram corrigidos em fluxo
contínuo. A identidade de markup e a escolha contextual do glyph continuam
abertas. O pacote concreto smartquote-semantica-2026-10-06.md contém os L0s
candidatos e núcleo compartilhado; aguarda confirmação ADR-0127 por alterar
campos públicos e mover a decisão eval → render. Os 23 controles originais de
query compilada permanecem fechados; esta atualização não fecha a linguagem
inteira nem débitos independentes de HTML, watch ou math.

## Atualização final — aspas materializadas após confirmação

smartquote-execucao-2026-10-06.md substitui o estado pendente da atualização
anterior: L0s/núcleo ratificados, código e testes entregues. Eval conserva
SmartQuote, campos omitidos diferem de false explícito, render Paged/HTML
resolve idioma/disabled/contexto de uso e checkpoint conserva nesting.
Medição final de query: 36/36 controles coincidem, inclusive os dois corpos de
aspas antes abertos; os 23 originais continuam coincidentes. Medição de aspas:
41/44 controles integrais coincidem; os negativos remanescentes são wrapper
Styled geral, rejeição do selector show text e root lang HTML. Não se fecha
paridade universal. Proveniência, UTC/HEAD/diff/binários, RED→GREEN, builds,
suítes e selos de 27 owners estão no relatório e receipts associados.

## Atualização final — D05 materializado

math.equation.where aceita campos públicos e aliases por identidade estática.
Query preserva snapshots e resolve label legado pelo índice; show do literal
é capturado no fluxo existente e o resultado textual de bloco conserva sua
separação de parágrafo. Sem API pública ou fase nova, em fluxo ADR-0127.

equation-where-execucao-2026-10-06.md registra RED→GREEN, proveniência e limites:
29/29 controles desta frente e 36/36 da regressão query coincidem. Builds e
linter passaram sem erros; dez owners foram ressellados sem alteração externa
a essa frente nas fontes/L0. D06–D08 permanecem abertos. Próxima frente: medir
D06 com mat([a])/frac([1],[2]), estilo, escopo, show e identidade antes de
decidir realização e eventual gate de fase.

## Atualização — D06 medido e gate concreto preparado

math-bare-medicao-2026-10-06.md registra before/additional/refuters com estado e
binários completos. Confirma inserção automática fora de math, escopo block/alt,
show/aliases, identidade e produtos contextuais; refuta envolver texto simples,
alterar valor cru ou promover label do Mathy à Equation.

Nove L0s completos e núcleo proposto em materialization/math-bare-2026-10-06.md,
com manifesto/hash. A nova fachada pública de contribuição e a preparação antes
de show/índice exigem gate ADR-0127. Sem implementação ou alteração de L0 vigente
até ratificação. Mapping show equation direto foi achado separado; D07 e query
do Mathy original rotulado permanecem fora do certificado desta frente.

## Atualização final — D06 ratificado e materializado

math-bare-execucao-2026-10-06.md supersede o estado pendente: normalização externa
antes de show/índice, valores crus conservados, escopo local e contribuição
contextual corrigidos, sem wrappers em descendentes math. Fachada L1 e fase
ratificadas; 13 owners ressellados sem alterações externas nas fontes/L0.

Coletas finais: 38/41, 18/20 e 4/4 controles integrais coincidem, em conjuntos
distintos. D05 29/29 e query 36/36 preservados; builds/suítes/linter passaram
sem erros. Render de raiz/negrito e bbox já divergem em Equation explícita;
aviso de arquivo vazio, morfologia de mat (D07) e query do Mathy original com
label continuam separados. Não há certificação universal de math. Próximo D07.

## Atualização final — D07 materializado

matrix-morfologia-execucao-2026-10-06.md registra a medição e execução atuais,
com HEAD, árvore não commitada, binários e UTC em final-results.json. Nome mat,
rows/células, campos e presença, identidade/call de func() e canonização do
stroke estão corrigidos: 28/29 e 12/14 controles integrais coincidem, sendo
28/28 e 12/12 os controles válidos de morfologia. Os três negativos preservados
são sintaxe de none/left sem # em math e dois spans de augment:() inválido.

D05 29/29, query 36/36 e D06 38/41 revalidados no binário final; os três
negativos D06 são render de raiz e aviso de arquivo vazio em eval cru. Nove
owners ressellados, zero alteração externa nas fontes/L0, builds e testes
passaram, linter sem erros. Próxima frente D08: query de cell não locatable,
selector e show têm critérios distintos; não promover a hipótese refutada
de aceitar table.cell na query.

## Atualização — resíduos D07 fechados e D08 preparado

math-residuos-execucao-2026-10-06.md supersede os três negativos D07: namespace
math/Code e spans dos casts corrigidos; 29/29 e 14/14 nos conjuntos originais
revalidados, 20/20 nos novos controles de resíduos (não somar coleções com
repetição). D05 29/29, query 36/36, D06 38/41 conservados no binário final.
Dois owners ressellados; builds e testes passaram; linter sem erros.

D08 continua aberto: table-cell-medicao-2026-10-06.md confirma ausência de
grupo vs where vazio, identidade nativa, query/counter não locatable e
posição/padding antes do show. D08A propõe NativeElement(Func) no enum público
Selector, com dez L0s completos e núcleo, aguardando ratificação ADR-0127.
D08B é a continuação explícita para show/campos/realização e precisa de seu
próprio L0 e eventual gate de fase. Não promover Cell ao índice nem inferir
que o novo carrier sozinho corrige transformações de células.

Refutadores de tipos locais: None/Align/Auto/Bool antes eram convertidos a
display Content em argumentos math. L0 atualizado, outro RED→GREEN válido e
4/4 controles local-refuters-final confirmam a correção. O certificado mais
recente usa final/final-residuos-locais e seu novo binário, sem somar coletas
intermediárias. Core final 6.577, infra 1.194 e CLI query 10 passaram.

## Atualização — D08A ratificado e materializado

Manifesto f80f85c4ac3ab747d20d98ef3f5758c59727bcf5f5d762082660099aef4dcc9c
confirmado pelo humano; dez L0s/núcleo promovidos. Seletor bare NativeElement
conserva ausência de where, nome canônico e identidade real; query/counter
rejeitam cell não locatable com span do valor. 21/21 controles D08A coincidem.
O comparador exaustivo de observação precisou de L0/ramo suplementar interno;
onze owners ressellados, zero alteração externa nas fontes/L0. Builds,
6.582 testes core, 1.194 infra e 10 CLI query passaram; linter sem erros.

D05 29/29, D07 29/29 + 14/14, query 36/36 e D06 38/41 revalidados no
binário final atual, com evidências e UTC em table-cell-selector-2026-10-06.
Show direto/where de células continuam negativos explícitos (D08B), junto
de posições, padding, campos e escopo. Próxima frente: medir/especificar a
realização D08B sem promover cell ao índice e sem mudar fase por conveniência.

## Atualização — D08B1 materializado e D08B2 pronto para gate (2026-10-07)

Body, campos já conservados, nome cell, função nativa e mapping show foram
corrigidos em seis owners, L0 primeiro/RED→GREEN, sem mudança pública ou fase.
A coleção de 35 controles passou de 1 a 11 coincidências integrais; 24 permanecem
negativos, inclusive auto/none lexical, propriedades resolvidas e render/reuso.
Fonte/HEAD/working tree/diffstat/UTC/binários e resultados por controle estão em
table-cell-realizacao-2026-10-07/{before,after}.json.gz e final-results.json.
Core 6583, infra 1194, integração query 10; builds/lint zero erros; D08A 21/21,
D05 29/29, D07 29/29+14/14, query 36/36 e D06 38/41 revalidados no binário final.
Ver table-cell-realizacao-execucao-2026-10-07.md para limites e proveniência.

D08B2 possui contrato público concreto de presença lexical, 15 candidatos L0
e manifesto 35f73c60301e1e4b27ae8b3ed2928b6d421dbcd0f32502131c3e2458260301e6
em 00_nucleo/materialization/table-cell-presenca-2026-10-07.md. Não instalado
nem materializado: aguarda gate ADR-0127 por campo/tipos novos. D08B3 conserva
resolução por posição/estilo/padding/show e precisa de contrato de fase próprio.

## Atualização — D08B2 ratificado e materializado (2026-10-07)

Confirmação do manifesto 35f73c60 registrada; 15 L0s/owners e núcleo de presença
instalados antes de código. Presence lexical distingue omissão/auto/valor e
none explícito em fill/stroke, com transformação/observação/JSON preservados.
Body vazio projeta sequência vazia após RED→GREEN próprio. Não há fase nova.

72/72 controles lexicais coincidem; a coleção completa dá 75/92, com 17
negativos de realização nomeados no relatório. Core 6588, infra 1194, shell
90 e integração query 10; builds/linter zero erros. D08A 21/21, D05 29/29,
D07 29/29+14/14, query 36/36 e D06 38/41 preservados no binário final.
HEAD/working tree/diffstat/UTC/binários/fontes/resultados por controle constam
de table-cell-presenca-2026-10-07/final-results.json e snapshots/coletas.
Relatório: table-cell-presenca-execucao-2026-10-07.md.

D08B3 permanece aberto: placement lógico, estilos/defaults, padding e
realização show/contexto. Exige contrato/fase próprio; confirmação D08B2 não
o ratifica. Refutadores de casts conservam três divergências adicionais
fora do recorte: stroke:0pt, align:string e inset:integer; shapes ricos de
inset/stroke/paint seguem declarados fora do escopo. Não fechar D08B global.


## Atualização — D08B3 medido, contrato A pronto para gate (2026-10-07)

Medições de ordem, coordenadas/estilos/vazios, aliases, conteúdo ocultado e HTML
registradas em table-cell-realizacao-d08b3-2026-10-07. Mesmos binários e fontes
B2; nenhuma mudança produtiva/L0 canônico neste turno. O vanilla aplica regra
da Table antes de resolver as Cells, e regra da Cell antes do body emitido.
Tekt também precisa aceitar a função table como selector show por identidade.

Proposta concreta: cinco campos públicos, dois carriers públicos, fachada de
realização e fase lógica antes de Cell show. 43 owners completos/núcleo pinado.
Manifesto `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b` em
00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07/manifest.json.
Pendente ratificação ADR-0127; B3 não está corrigido nem fechado. B3-A tabela,
B3-B defaults de Cell isolada/recursão fresca; demais limites continuam abertos.
Relatório e proveniência: table-cell-realizacao-d08b3-medicao-2026-10-07.md.

## Atualização — D08B3-A ratificado e materializado (2026-10-07)

A confirmação `comfirmo` ratificou o manifesto 5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b.
Posições/propriedades/vazios, show Table→Cell→body, escopos/aliases, efeitos
ocultados e consumidores do plano foram materializados. Presença 92/92 e
selector 23/23; B3 60/62, refutadores 16/18. Os restantes nomeados pertencem
a Cell isolada/reconstrução fresca ou igualdade genérica de Table.
6600 core + 1194 infra + 90 shell + 10 integração query, builds e linter
zero erros. D05 29/29, D07 29/29+14/14, query 36/36, D06 38/41 preservados.

Geometria aninhada/selector/aliases corrigida; Boolean Raw continua com fonte
e tamanho divergentes no renderer geral, fora dos owners ratificados. Próximo:
medir e especificar defaults/show-set de Raw com overrides e controles de
largura/altura; depois concluir B3-B com gate próprio. Não fechar D08 global
nem a certificação geométrica completa. Proveniência, UTC, estado exato,
hashes, coletas e preservação em table-cell-realizacao-d08b3-2026-10-07/
final-results.json; relatório table-cell-realizacao-d08b3-execucao-2026-10-07.md.

## Atualização — defaults de Raw corrigidos (2026-10-07)

Mono80%, fill herdado, overrides absoluto/em/fonte, substituição e
restauração de escopo foram corrigidos em quatro owners e núcleo pinado,
com RED→GREEN em fluxo contínuo ADR-0127. Família/tamanho/cor coincidem
nas16 sondas. Os cinco controles geométricos B3-A conservam todas as palavras;
Boolean em tabela tem delta máximo bbox0.001793pt, demais até0.00004pt.
Não fechar render Raw universal: bloco mantém diferença vertical13.52998pt;
inline sob bold/italic conserva resíduo até0.14303pt. Whitespace/custom syntax
e reconstrução fresca continuam fora desta certificação.

6602 core + 1194 infra + 90 shell + 10 integração query passaram, builds
e linter zero erros (544 warnings existentes). Presença92/92, selector23/23,
D05 29/29, D07 29/29+14/14, query36/36 e D06 38/41 preservados.
B3 60/62 e refutadores16/18 conservam os negativos já nomeados.
Próximo contrato: B3-B Cell isolada/recursão fresca; espaçamento de Raw bloco
e resíduos de avanço ficam como frentes de render mensuradas próprias.
Relatório raw-defaults-execucao-2026-10-07.md; proveniência/UTC/hashes/estado
exato/coletas em raw-defaults-2026-10-07/final-results.json.

## Atualização — Raw bloco corrigido no recorte medido (2026-10-07)

Margens fracas/fortes, origem regional, medidas terminais, show substituído,
linhas sem gramática, margens0/negativas/auto e aliases set block foram
corrigidos em fluxo contínuo ADR-0127, com7 owners e núcleo pinado.
25 controles Raw têm delta vertical máximo0.00005pt;3 refutadores adicionais
coincidem até0.00002pt. Sizes-down já era negativo e conserva10.80002pt:
a regra de tamanho do parágrafo anterior continua aberta, fora do Raw.
6606 core +1194 infra +90 shell +10 integração query passaram; builds
e linter zero erros,544 warnings. Presença92/92, selector23/23, D05 29/29,
D07 29/29+14/14, query36/36, tipografia16/16 e D06 38/41 preservados.

raw-block-spacing-execucao-2026-10-07.md supersede a pendência vertical de
Raw bloco descrita na etapa anterior. Coleções/estado exato/UTC/hashes em
raw-block-spacing-2026-10-07/final-results.json. Whitespace/RawLine/rich styles
não são certificados. Próximo contrato: B3-B Cell isolada/recursão fresca;
resíduos de tamanho do parágrafo e avanços inline são frentes separadas.

## D08B3-B — medição e contrato proposto em 2026-10-07

Após concluir Raw block spacing, avançou-se na Cell isolada e geração fresh em show. Coleta única de 53 sondas contra pin 9f2b6e871, com estado/binários/fontes/UTC completos, em `table-cell-isolada-d08b3b-2026-10-07/complete.json.gz`. Defaults avulsos/getters/selector, efeito duplicado, geração morfologicamente igual e consumo de regras anteriores têm negativos. Contrato público novo proposto em `00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md`, manifesto 3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df; aguardando gate ADR-0127 antes de materializar. Código/L0 vigentes preservados. Relatório: `table-cell-isolada-d08b3b-medicao-2026-10-07.md`. `set table.cell` identificado como débito D08B3-C; não declarar B universalmente fechado.

## D08B3-B — execução ratificada e verificada em 2026-10-07

O pacote acima foi confirmado (`confirmo`) e materializado. Defaults da Cell avulsa, efeito único, regras anteriores, identidade de alias versus reconstrução nova, limite causal e caret foram corrigidos. Auto agora é Raw inline typc, com apresentação paginada e HTML atômico corrigidos em fluxo contínuo. Release final 721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3, snapshot tested-fifth e coletas finais no diretório table-cell-isolada-d08b3b-2026-10-07. 50/53 saídas completas, 8/8 adicionais e 10/10 controles debug; os três controles não idênticos são warnings antigos de arquivo vazio e set table.cell fora do recorte. Geometria: dez controles qualificados, delta vertical máximo 0.00002pt. 7.908 testes passaram; builds debug/release e linter sem erros (544 warnings, 1.630 notes). Preservação restrita aos 21 owners e dois núcleos novos, sem commit. Relatório e limites: table-cell-isolada-d08b3b-execucao-2026-10-07.md. Próximo: D08B3-C, set table.cell/defaults de estilo; set/text posicional também medido como negativo preexistente separado.


## D08B3-C — defaults e tinta verificados em 2026-10-07

Fluxo contínuo ADR-0127, sem contrato público/fase/modo novo. Defaults declarativos de Cell, condição/spreads/aliases simples, precedência e padding corrigidos; fold de stroke em sets/show-sets e marcas do literal Length+Color corrigidos. Campos observados de spans ficam separados da ocupação. Tinta do plano Table permanece mesmo quando a receita Cell substitui o corpo: o enquadramento anterior de sobrevivência foi refutado por PDF/SVG real e retificado no L0. Release 23eb4227d17b332af08414506b81c685c63cfa440d18b18e0c2ec69fd23c675c, debug 36de4629ebe213a25e584e9419a2f2756b74b19308cf540afcb51d137193101e; snapshot tested-final e coletas verified em table-cell-styles-d08b3c-2026-10-07. Main 58/58, extras 7/7, show-fold 2/2, adicionais 8/8, debug 10/10; 10/12 refutadores e 51/53 controles B completos pelos negativos documentados. 7.921 testes passaram, builds e lint sem erros (546 warnings,1.635 notes), sem nova divergência nos corpus anteriores e sem mudança produtiva/L0 fora dos 13 owners/núcleo. Sem commit. Relatório: table-cell-styles-d08b3c-execucao-2026-10-07.md.

Próximo D08B3-C2: medir e especificar origem/forma dos argumentos Table (caret de argumento cru e body local), com classificação ADR-0127 antes de materializar. Métricas horizontais 5pt/0.005pt foram reproduzidas no release B por constructors explícitos e não são fechadas por defaults. Casts/estilos ricos, spans/grupos multipágina, Table geral, Raw/math/set-text e PDF/UA continuam nos limites registrados. Não declarar D08 fechado universalmente.


## D08B3-C2 — origem e morfologia medidas, contrato proposto (2026-10-07)

Oito sondas bilaterais em table-origens-d08b3c2-2026-10-07/baseline.json.gz, com fontes/binários/HEAD/working tree/UTC: apenas plain-func coincide na saída completa. Alias aponta para definição, spread para item no vanilla; Args.occurrences sozinho não resolve origem. Corpo local e c.func() com set/show expõem perda de Styled lexical. Proposta pública SourceContent/Content::Source e correção da fronteira causal do join, sujeitas ADR-0127 §2.1/2.3/2.4. Contrato/candidatos/manifesto em materialization/table-origens-d08b3c2-2026-10-07; relatório table-origens-d08b3c2-medicao-2026-10-07.md. Nenhum código/L0 vigente alterado; correção e validação ainda pendentes.


## D08B3-C2 — origem e Styled lexical ratificados e verificados (2026-10-07)

O pacote C2 acima foi confirmado (`confirmo`), materializado com L0 primeiro e resselo restrito. Primeira origem via SourceContent/Content::Source conserva caret de bare/alias/spread; cauda local conserva Styled e snapshot causal, realizado na inserção. Todos os 57 owners e núcleo novo estão individualizados/pinados. A captura geral de Content foi refutada e restringida; Empty/equação/trace e concatenação plana com Source foram corrigidos pelos corpus. A regressão debug fresh-growing foi resolvida sem alterar limite de show ou pilha.

Principal8/8, adicionais10/10, estilos58/58, controles adicionais8/8, Cell isolada51/53 (dois warnings antigos), profundidade10/10 debug e10/10 release. Presença92/92, selector23/23, D05 29/29, D07 29/29+14/14, query36/36, Table62/62; demais corpus sem nova divergência contra C final. Builds e7925 testes aprovados, cinco core ignorados; lint zero erros,535 warnings/1636 notes históricos, owners sem V5/V6/V15/V26. Preservação115 arquivos produtivos/L0, restritos a57 pares e um núcleo; sem commit.

Relatório: table-origens-d08b3c2-execucao-2026-10-07.md. Estado tested-terminal, SHA release ac889581d6423d2fcfdd77d2f7e5cf57562dbf9adb40e73aebc25f959864c04f, pin vanilla9f2b6e871; fontes/UTC/argv/diffstat/hashes e verificação em table-origens-d08b3c2-2026-10-07/final-results.json.

Próximo D08B3-C3: composição de default show da Cell em produto misto. nested-local-text agora conserva palavras/fontes/cores, mas a segunda linha fica6.04996pt acima do vanilla; o C congelado já era negativo e tinha todas as palavras numa linha. Três controles locais, incluindo zero inset, têm posição igual até0.00002pt. Investigar limites de parágrafo/margens/padding antes de especificar; não fechar geometria por texto nem compensar com constante arbitrária. Resíduos horizontais5pt/0.005pt, set-text posicional, rich casts/Table geral/multipágina/Raw/math/whitespace/PDFUA permanecem separados.


## Medição de desempenho atual C→C2 — novo refutador C2-R1 (2026-10-07)

User pediu teste do binário atual. Sete fixtures, três rodadas,756 compilações CLI medidas +126 warmups, comparação com C congelado e vanilla9f2b6e871. Fonte/binários estáveis e blocos sem concorrência observada. Seis recortes equivalentes conservam saída; diferença pareada −1.21% a+0.96%, pequena frente à faixa de medianas entre rodadas acima de3%; nenhum ganho geral demonstrado. RSS separado por GNU time:378 medidas +126 warmups; seis controles equivalentes têm delta mediano −0.371 a+0.354MiB. wait4 RSS do launcher foi descartado como medição de memória do compilador.

Nova prioridade C2-R1 antes de C3/otimização: Table com100 aliases de Cell Styled local, sob set text(size10pt), retornou100 palavras mas alterou tamanho e posições no atual. Primeiro glyph-word width21.880→24.068pt; xMin da segunda palavra60.880→38.999, sobreposição real. Diferença de tempo+7.52%, RSS+1.049MiB, mas saída não equivalente. Não fechar paridade de estilos locais universalmente; resultados C2 originais permanecem certificados apenas nos seus fixtures. Diagnóstico/witness/proveniência completos em performance-c2-execucao-2026-10-07.md e performance-c2-2026-10-07/{summary,table-local-regression}.json. Medição apenas, sem alteração produtiva/L0, build novo, resselo ou commit.


## C2-R1 — prioridade1 corrigida e revalidada (2026-10-07)

Pedido humano “Faça a prioridade1”. Quatro owners, L0 primeiro, RED tamanho/overlap antes de código. Styled captura prefixo declarado desde a entrada (não defaults herdados), preservando valores explicitamente iguais ao default. Medida Auto projeta visual de Cell standalone já realizada. O testemunho100 Cells volta a coincidir com vanilla em palavras/fontes/cor/coordenadas até0.00002pt, sem sobreposição. Quatro testes novos;7929 aprovados +cinco ignorados; builds/lint sem erros,535 warnings/1636 notes, owners sem V5/V6/V15/V26. Oito arquivos produtivos/L0 próprios, resto herdado preservado, sem commit. Corpus C/C2/macro sem nova divergência nos gates anteriores.

Recoleta CPU4 com GNU time/taskset:324 execuções +54 warmups, três rodadas/três fixtures, estabilidade e repetibilidade<3%. Table local anterior170.94ms→atual170.10ms, delta pareado−0.50%; RSS46.11→45.36MiB. Hello/aliases praticamente iguais. Não declarar aceleração geral; a saída anterior estava errada e os wrappers diferem da população anterior sem wrapper. Fonte/binário0a2410ec5b4a075417943d25a4d24ac860da6ff0adf4845707d4f18dc9cc7add/UTC/receipts em table-local-c2r1-2026-10-07/{final-results.json,performance/summary.json}. Relatório table-local-c2r1-execucao-2026-10-07.md.

C3 mixed default continua6.04996pt vertical. Variantes ampliadas conservam resíduos:14pt0.004pt horizontal (não existia no C, muito menor que negativo C2), bold/italic0.68pt horizontal, tamanho local18pt5.264pt vertical; demais no relatório. Não fechar Styled/Table universalmente. Para velocidade, próxima prioridade2/cobertura de fontes por intervalos com gate/medição próprios; nenhuma implementação dessa frente foi iniciada neste R1.


## Prioridade2 — cobertura por intervalos preparada (2026-10-07)

Pedido humano “Continue com a2”. Auditoria confirma Coverage/extrator byte-idênticos ao experimento de05/10; L0 público vigente preparado ainda contém from_ranges não materializado, com gate explícito. Não tratar53.19→10.85ms como ganho da CLI atual. Pacote completo com contrato/três candidatos/owners1:1 e manifesto em materialization/coverage-ranges-2026-10-07; aguardando ratificação concreta do novo construtor público Coverage::from_ranges(impl IntoIterator<Item=Range<u32>>) -> Self, ADR-0127 §2. Guardar política/lookup/fallback/seleção e ordem de catálogo; parser4/12/13 privado L3, normalização L1, callback/rollback/legacy certificados. Sem código produtivo/L0/hash/build alterado, sem commit. Após confirmação, materializar e medir CLI/catálogo/fontes/PDF/RSS com proveniência e equivalência.


## Prioridade 2 — cobertura materializada e medida (2026-10-07)

Dono ratificou o pacote com “confirmo”; L0 primeiro, três owners 1:1, RED→GREEN e resselo restrito. Coverage::from_ranges em L1 e parser privado 4/12/13 em L3, mantendo callbacks/rollback/legacy; nenhuma política de catálogo/metadados/fallback mudou. 7.941 testes aprovados, cinco ignorados; builds/linter sem erros, 535 warnings históricos. Demais arquivos produtivos/L0 recebidos preservados; sem commit.

HEAD/working tree/snapshots/UTC/argv/assets e hashes em coverage-ranges-2026-10-07. Release medido/final SHA e44043d9471a6d2eec25da7562dd5b0a071d4e14ab0f307011ca978e46e49109. 1.140 faces (1.112 sistema +17 embutidas +11 fixtures) com cobertura idêntica; book/slots também iguais. Três rodadas/864 execuções CLI +144 warmups, 24 blocos estáveis, todas saídas equivalentes ao anterior; três controles raster idênticos ao anterior.

Controle Hello repetível e qualificado perante vanilla: 152,83→112,44ms, delta pareado −26,53%. Multiescrita −14,13%, repetível perante anterior; mantém negativas vanilla. Outros casos com fontes indicam −22,92–25,48% com dispersão >3%; imagem/shapes sem texto praticamente iguais, shapes +1,90% sem atribuição causal certificada. Memória da CLI praticamente igual. Extração+parsing 64,36→15,90ms (~−75%); catálogo integral 135,49→93,69ms (~−31%), todos os pares melhores, tempos absolutos com dispersão >3%. Sem promessa de ganho universal ou nova paridade de render.

Relatório: coverage-ranges-execucao-2026-10-07.md; final-verification.json/controls.json/performance/summary.json são os gates finais. D13/cobertura concluída neste escopo; prioridade 3/aquisição seletiva de metadados continua pendente.


## Prioridade 3 — aquisição seletiva preparada para ratificação (2026-10-07)

Pedido humano “Continue com a3”. Pacote completo em materialization/catalogo-seletivo-2026-10-07/contract.md, dois candidatos L0/owners1:1 e manifesto SHA045ef18d7a16143e640669cf5923db1a213eb677b6c278d4ee4a0bf542973c2e. Source::File regular/Unix poderá tentar projeção limitada (4.096 registros,8MiB buffer,16MiB pedidos) com identidade estável e fallback with_face_data; SharedFile conserva bytes retidos. FontSlot/shaping/export e coverage por intervalos permanecem com owners atuais. Sem nova API pública ou flag.

Auditoria source/assets/HEAD/dirty/UTC em catalogo-seletivo-preparacao-2026-10-07/{audit,preservation}.json e before.json.gz:1.140 faces dentro dos limites,20 tabelas e259.927 bytes máximos projetados;1.086 assets históricos idênticos e body metadata idêntico ao histórico comprovado. Tempos de05/10 não provam benefício da combinação atual. Gate classificado ADR0127§2.2/§5 pela substituição da aquisição padrão explicitamente exigida no L0; aguardando ratificação concreta deste pacote. Nenhum consumer/L0 canônico/hash/build/binário alterado, nenhuma medição de velocidade nova, sem commit. Próximo: ratificação, RED→GREEN, corpus/per-face e comparação catálogo/CLI/RSS contra release e44043d9 atual congelado e vanilla pin9f2b6e871.


## Prioridade 3 — aquisição seletiva materializada e validada (2026-10-07)

Ratificado o manifesto045ef18d7a16143e640669cf5923db1a213eb677b6c278d4ee4a0bf542973c2e. Dois owners, L0 primeiro/RED→GREEN, File regular Unix com limites/identidade/fallback, SharedFile com bytes retidos. 7.954 testes aprovados, cinco core ignorados; builds/lint sem erros,535 avisos históricos.1.140 faces comparadas,1.139 selecionáveis aceitas no fast path; book/slots/ordem/coverage iguais. Todos os oito fixtures com outputs/fontes/geom/stdout/stderr equivalentes ao anterior, três rasters também iguais.

Release final 96f22a1d63d51bfcc258ca2849563b037e66a67b7e34febaf4779da2dd146a4b; HEAD/working tree exata/UTC/assets/argv/receitas em catalogo-seletivo-2026-10-07. Coleta inicial conservada e repetida após fortalecimento do teste SharedFile alterar o hash do binário; usar performance-final/probe-results-final/controls-final. Três rodadas/864 execuções +144 warmups/24 blocos válidos. Gates CLI repetíveis: nenhum no release final; tempos indicativos. Catálogo integral final tem redução pareada 3.75%, repetível em ambas implementações; CLI final não passa o gate estrito, valores indicativos e negativas preservadas. RSS praticamente igual; outras dispersões/negativas no relatório catalogo-seletivo-execucao-2026-10-07.md. Ganho incremental perante prioridade2, não somar percentuais históricos. Nenhum commit; demais arquivos produtivos/L0 herdados preservados.

## Prioridade 4 — investigação de watch, 2026-10-07

Ver [watch-retencao-2026-10-07.md](watch-retencao-2026-10-07.md): baseline de
edições reais e contagem de catálogo no binário atual, além de recuperação de
imports/read/imagem e substituição/adição de fontes. D10 continua aberto:
retenção ainda não materializada. Próxima fronteira é medir a validação de uma
geração de fontes antes de definir API/L0; conservar o comportamento Tekt de
reconhecer fonte alterada na próxima compilação, sem copiar a retenção obsoleta
do vanilla. Nenhum ganho geral ou mudança em produção foi declarado.

## Prioridade 4 — validação e contrato concreto, 2026-10-07

A continuação em [watch-validacao-2026-10-07.md](watch-validacao-2026-10-07.md)
mede descoberta/identidade contra aquisição completa e treze controles de
invalidação. Resultados isolados indicativos, sem ganho de watch implementado.
O [pacote de materialização](../materialization/watch-catalogo-2026-10-07.md)
prepara sete owners candidatos, um Núcleo e duas assinaturas públicas novas.
D10 aguarda ratificação ADR-0127 do contrato/manifesto, depois RED→GREEN,
implementação e medição real. Consumers/L0 ativos e binário CLI preservados.

## Prioridade 4 — sessão de fontes materializada, 2026-10-07

A confirmação ratificou o pacote watch-catalogo-2026-10-07. Implementação e
validação em [watch-catalogo-execucao-2026-10-07.md](watch-catalogo-execucao-2026-10-07.md):
World novo por ciclo, sessão com geração validada, atualização de arquivos/fontes
e recuperação preservadas. Lint 0 erros; 19 testes novos passam. Suíte ampla
mantém sete resíduos HTML/bundle reproduzidos no binário anterior; duas falhas
de bind do sandbox passaram na repetição autorizada.

Nos pares atômicos de watch, redução mediana de CPU por par de 47,54% no Hello e
30,72% na tabela; tempos absolutos variáveis, sem certificado geral de CLI.
RSS retido sobe aproximadamente 4 MiB; startup indicativo adiciona 13 ms de CPU.
D10 passa de retenção não implementada para **retenção materializada/benefício por
edição observado**, conservando acompanhamento de startup/memória e custo de
redescoberta ainda presente. Nenhum fechamento global de D10–D12 nem soma de
ganhos das prioridades anteriores.
