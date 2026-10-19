# Heranças da reescrita e frentes de correção

Este inventário organiza diferenças que surgiram durante a construção do compilador e merecem revisão. Serve para abrir frentes independentes, com evidência e critérios de conclusão. Não é Prompt L0 e não autoriza implementação. A identificação de uma origem histórica não comprova ratificação humana nem que a mesma origem explique outros mecanismos.

## Escolhas mantidas pelo usuário

Nesta conversa de 2026-10-05, o usuário reafirmou a preferência por código explícito: a facilidade de leitura e o baixo custo de escrita com IA são motivos suficientes para mantê-lo. Trocar registros e despacho por macros não é objetivo destas frentes.

O usuário também reafirmou a importância da segurança oferecida pelo Rust e esclareceu o motivo: o código é escrito pela IA, então deseja manter as garantias verificadas pelo compilador. Este trabalho não propõe introduzir `unsafe`. Tecnicamente, `unsafe` transfere ao programador a responsabilidade por determinados invariantes; sua presença não é sinônimo de I/O ou impureza. Essa distinção não invalida a preferência por implementações seguras.

A separação entre domínio puro, infraestrutura e composição permanece. Reavaliar ciclos de execução e vida dos caches não exige abandonar essa separação nem o enum de conteúdo.

## Evidências e proveniência

Estado inspecionado: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, em 2026-10-05. `git diff HEAD --stat` vazio; os diagnósticos e sondas desta sessão permanecem não rastreados. Este inventário não introduz novas medições de desempenho.

As medições referenciadas têm estado do código, horários, binários e limites nos relatórios [macro e micro](macro-micro-2026-10-05.md) e [custo da paginação](grid-pagination-cost-2026-10-05.md). O oráculo atual é o vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`. Documentos antigos servem aqui como evidência histórica, sem transferir suas afirmações de paridade para o pin atual.

## Frente A Preservar introspecção sem avaliar todo o arquivo novamente

A [auditoria desta frente](dupla-avaliacao-auditoria-2026-10-05.md) encontrou a origem no P498, compensações posteriores e divergências atuais quando regras show produzem novos headings. Inclui controles reproduzíveis e limites da investigação.

**Origem encontrada.** O [P498](../materialization/typst-passo-498.md) tratou o desaparecimento de elementos originais após regras `show`, que quebrava consultas. O plano recomendava preservar conteúdo pré-transformação. O [relatório](paridade-funcional-p498.md:33) registra uma adaptação para duas avaliações, evitando uma refatoração mais profunda. A palavra “aceite” no relatório não comprova aprovação específica do usuário.

**Estado atual.** `01_core/src/compiler/eval/mod.rs:872–880` executa uma passagem sem aplicar `show` e outra aplicando. A composição da biblioteca base também ocorre dentro de cada passagem. A necessidade linguística é preservar identidade, conteúdo e propriedades observáveis pela introspecção; avaliar o programa inteiro duas vezes é a solução materializada, não a necessidade em si.

**Investigação.** Rastrear consumidores de `introspection_content`, efeitos de avaliação, labels, seletores, contadores e regras `show`. Verificar onde outros caminhos passaram a pressupor duas avaliações. Não presumir que os demais ciclos descendem do P498.

**Conclusão exigida.** Preservar semântica, sintaxe e morfologia nos casos que motivaram a separação; demonstrar a redução de trabalho com contagens e tempos reproduzíveis. A solução concreta deve ser especificada em L0 antes do código.

## Frente B Repetir a paginação somente quando as dependências exigirem

**Estado comprovado; origem pendente.** O caminho comum em `03_infra/src/pipeline/paged.rs:323–413` faz layout, obtém posições e páginas e volta a diagramar. A tabela simples também passa pelo ciclo. Esta repetição é distinta da dupla avaliação da Frente A.

**Investigação.** Localizar o passo que tornou o ciclo obrigatório e separar dependências reais de páginas e posições de repetição por conveniência. Mapear a reexpansão contextual e seus critérios de estabilização.

**Conclusão exigida.** Demonstrar quais documentos podem encerrar após o primeiro layout e quais precisam de convergência, preservando comportamento contextual. Medir a economia sem atribuir toda a diferença de CLI a esta etapa.

## Frente C Completar a estratégia de memoização

**Origem parcialmente encontrada.** A [ADR-0001](../adr/typst-adr-0001-estrategia-migracao.md) previa preservar `comemo` e compilação incremental. O [P458](../materialization/typst-passo-458.md:85) adiou `TrackedWorld` e memoização incremental para uma fase de desempenho condicionada a benchmark. A [ADR-0073](../adr/typst-adr-0073-comemo-introspector.md) registra tracking do `Introspector`. Não tratar as hipóteses daquele passo sobre avaliação lazy como fatos atuais sem nova verificação.

**Estado atual.** Há caches locais e funções memoizadas, mas tracking isolado não implica reutilização dos resultados de avaliação e layout. Há distância entre a intenção inicial e o mecanismo abrangente materializado. Ainda não foi demonstrado que o usuário ratificou esse adiamento.

**Investigação.** Inventariar unidade cacheada, chave, dependências, proprietário, duração e invalidação. Separar reutilização dentro de uma compilação, entre passagens e entre edições. Avaliar exigências das regras de pureza para o mecanismo concreto, sem assumir que copiar a implementação do vanilla seja necessário.

**Conclusão exigida.** Demonstrar hits e trabalho evitado, além de invalidação correta quando mudam conteúdo, estilos, contexto e recursos. A memória retida deve ser medida junto com o ganho.

## Frente D Preservar recursos entre ciclos de watch

**Origem encontrada na implementação.** No commit `41b3a89fe`, associado aos P1137–P1138, o loop de `watch` já chamava a rotina de compilação que construía o mundo. O [plano](paridade-p1137-cli-restante-plano.md:25) destaca dependências e publicação atômica, sem explicitar uma decisão sobre retenção de fontes e caches.

**Estado atual.** `04_wiring/src/watch.rs:63` chama `run_compile_observed`, que cria `SystemWorld` em `04_wiring/src/compile.rs:246`. O vanilla conserva o mundo e reinicializa o estado pertinente entre edições. A hipótese histórica é que o reaproveitamento da rotina de compilação isolada trouxe seu ciclo de vida para o `watch`; isso continua sendo inferência.

**Investigação.** Separar recursos duráveis de estado por compilação, definindo invalidação de arquivos, imports, fontes, assets, inputs e relógio. Esta frente pode conservar recursos mesmo antes de completar a memoização da Frente C.

**Conclusão exigida.** Medir recompilações no mesmo processo e garantir atualização correta após mudanças e recuperação após erros, preservando publicação atômica e a última saída válida.

## Frente E Eliminar construções cujo resultado é descartado

**Estado comprovado; origem pendente.** Chamadas de leitura a `Content::map_content` descartam a árvore reconstruída. Exemplos em `03_infra/src/pipeline.rs:275` e `03_infra/src/pipeline/paged.rs:122,129,853`. Separadamente, células constroem conteúdo para medir altura e voltam a construir para emitir.

**Investigação.** Contar nós, alocações e layouts de corpo. Tratar travessia de leitura e reaproveitamento de células como subtarefas distintas: têm contratos e riscos diferentes. Para células, verificar igualdade de largura, região, alinhamento, estilos e contexto antes de reutilizar resultados.

**Conclusão exigida.** Preservar ordem de visita e falhas das travessias e a geometria linguística das tabelas. Demonstrar redução de construção e impacto medido; uma duplicação encontrada não comprova grande custo.

## Frente F Rever preparação de texto e aquisição de fontes

**Estado comprovado; intenção histórica pendente.** Medição de larguras e produção final de glifos têm produtos e caches distintos. No shaping final há preparação anterior à consulta do cache. A aquisição de fontes também contém leituras estruturais repetidas. Os refinamentos recentes compartilharam caches sem eliminar essas diferenças.

**Investigação.** Separar aquisição, escolha de fonte, bidi, segmentação, shaping e reconciliação. Contar misses e trabalho anterior aos hits. Registrar a origem histórica da divisão entre métricas e glifos antes de classificá-la como resíduo.

**Conclusão exigida.** Demonstrar reúso válido com fonte, idioma, script, direção, eixos, estilos e contexto compatíveis. Manter correção de texto e geometria. Não reabrir automaticamente protótipos recentes sem ganho convincente.

## Frente G Corrigir contexto antes de encurtar seus ciclos

**Divergência observada; causa pendente.** A qualificação do estudo macro encontrou uma fixture com `counter(page).display()` produzindo `0` no Tekt e `1` no vanilla. O caso foi excluído da comparação de desempenho equivalente.

**Investigação.** Reproduzir no pin atual e localizar a passagem que fornece o snapshot incorreto. Não atribuir a falha à dupla avaliação ou à paginação sem isolamento causal.

**Conclusão exigida.** Teste RED reproduzível, correção e GREEN no nível da linguagem. Este caso é pré-requisito para encurtar caminhos de contexto que o atravessem.

## Como abrir as frentes

Cada frente deve começar por um passo de investigação com problema reproduzível, fontes e L0 vigentes, origem conhecida ou pendente, contagens e critérios de conclusão. A solução arquitetural vem depois da medição. Uma correção concluída precisa registrar estado exato do código e resultado, incluindo regressões e limites.

A Frente A é a prioridade de revisão histórica indicada pelo usuário. A Frente D permite estudar retenção de recursos separadamente. A Frente G precede otimizações contextuais afetadas. As frentes B, C, E e F podem ser investigadas em paralelo conceitualmente, mas suas implementações precisam coordenar ownership, caches e ciclos compartilhados.

Antes de materializar uma correção, auditar e atualizar seu Prompt L0 proprietário e aplicar os gates vigentes, inclusive ADR-0127. Este inventário não escolhe novas assinaturas, novos padrões de produto nem uma mudança de fase do pipeline.


## Atualização da Frente G — 2026-10-05

A divergência conhecida da página atual em counter(page).display foi corrigida com RED→GREEN, reutilizando a leitura lógica já usada em get/at. Relatório: `contexto-page-display-2026-10-05.md`. A frente não está totalmente encerrada: o controle both:true continua com total 0, contra 1 no vanilla pinado. Total/final por documento e bundles deve ser tratado antes de encurtar os ciclos dependentes. Não foi removida nenhuma passagem nesta correção.

## Atualização da Frente G — snapshot Page, 2026-10-05

O conjunto L0 de total/final foi ratificado e materializado. Current/final/replay passaram a compartilhar vistas Page por ocorrência e documento, combinando ordem causal e avanços físicos. As 17 fixtures CLI coincidiram com o vanilla pinado, incluindo bundles mistos e estados vazio/multinível. A retenção das duas vistas foi caracterizada nas fixtures, sem medir ganho temporal. Ver [relatório e limites](contexto-page-snapshot-2026-10-05.md). Essa evidência fecha os controles medidos da divergência contextual; não fecha universalmente a linguagem nem as frentes de memoização/watch/ciclos.

## Atualização das frentes C/F — métricas contextuais, 2026-10-05

A sessão contextual selecionada agora compartilha suas métricas entre tentativas de layout. Na fixture de 320 textos distintos, 965 cálculos após miss passaram a 643, preservando duas entradas no wrapper; CLI 215,59→205,43 ms na rodada final (4,71%). Sete controles de texto/geometria/callbacks e dois de erro passaram; 1155 testes L3 passaram. O escopo anterior restrito ao caminho comum era documentado, não foi provado como decisão por inércia. Ver [relatório e limites](contexto-cache-fontes-2026-10-05.md). A reconciliação selecionada ainda mantém métricas frescas; watch e memoização de frames permanecem próprios.

## Atualização C/F — reconciliação contextual, 2026-10-05

O cache do candidato selecionado agora chega à reconciliação final, conservando lifetime por compilação/World. Na fixture distinta, a reconciliação passou de 321 falhas para zero e a CLI de 209,10 para 200,08 ms (4,31%). Layouts permanecem dois; bidi selecionado continua fresco. 1156 testes L3 e oito controles de texto/geometria/callback/RTL passaram, além de dois controles de erro. Ver [relatório](contexto-cache-reconciliacao-2026-10-05.md). Não extrapolar para watch ou memoização de frames.

## Atualização C/F — bidi contextual, 2026-10-05

O handle selecionado agora chega também ao bidi, na mesma compilação/World. RTL distinto: cálculos bidi 299→137, reconciliação 161→24, totais 622→323, dois layouts preservados; CLI melhorou 5,4–5,8% em duas rodadas, 18 pares favoráveis. 1157 testes L3 passaram; seis controles preservaram texto/bbox/raster/diagnósticos baseline/candidato, além de dois erros. A limitação anterior era explícita para isolar contribuição, não inércia demonstrada. Ver [relatório e proveniência](contexto-cache-bidi-2026-10-05.md).

Nova pendência de correção: no RTL longo com palavras numeradas, a extração PDF do Tekt diverge do vanilla tanto no caminho selecionado quanto no ordinário, já no baseline. Os controles RTL simples coincidem. Antes de otimizar mais esse algoritmo, reduzir o caso e separar ordem de leitura PDF de geometria; localizar layout/bidi/reconciliação/exportação sem inferir causalidade pela proximidade. As saídas baseline/candidato foram idênticas, portanto o novo empréstimo não introduziu essa divergência nos casos medidos.

## Correção RTL — 2026-10-05

A divergência longa da etapa anterior foi reduzida e corrigida em duas causas: índices de linhas invalidados por inserções de sufixos e sufixo colocado fora do slot já reservado. O registro P569 documenta a origem da operação, sem provar a presença histórica dos defeitos no estado exato daquela medição. 1160 testes L3 passaram; 18 controles coincidiram em texto com vanilla, e 17 tiveram caixas de grupos dentro de 0,05pt (maior delta≈0,01305pt). Não houve medição de ganho de tempo. Ver [relatório, REDs e proveniência](rtl-numeros-2026-10-05.md).

Nova pendência delimitada: linha explicitamente LTR dentro de contexto RTL preserva texto, mas diverge no alinhamento horizontal; baseline e candidato Tekt coincidem nessa linha, portanto é anterior ao reparo. Auditar direção versus alinhamento/âncora do parágrafo antes de modificar seu L0. Os grupos RTL desse controle foram corrigidos; isso não fecha paridade integral de direção mista.

## Direção versus âncora de parágrafo — 2026-10-05

Separadas a direção de cada trecho e a direção compartilhada do parágrafo no layout textual certificado. A âncora deixa de mudar quando uma linha começa com run oposto. Slots privados acompanham escopo/cauda e rollback; subframes isolam/restauram o pai. 6519 testes L1 passaram (5 ignored) e 1160 L3; nove controles coincidiram em texto/caixas contra vanilla. Maior delta observado≈0,01560pt, sem medição de tempo. O histórico P576 documenta a origem do alinhamento por item, mas seus controles não certificavam este caso. Ver [relatório, refutações e proveniência](direcao-alinhamento-2026-10-05.md).

Permanecem três responsabilidades distintas: contexto de direção-base ausente no bidi por linha/espaços em scripts misturados; corpo de Block com default por direção; precedência de Align explícito. Seis controles da matriz registram essas divergências. A âncora das linhas de wrap foi corrigida, sem fechar ordem de leitura ou span da borda oposta. A próxima correção deve medir esse transporte antes de alterar contratos ou misturar fases.

## Auditoria de contexto completo do bidi — 2026-10-05

Treze sondas CLI e uma sonda de unicode-bidi refutaram o transporte apenas de direção-base: com base RTL, o intervalo 456 789 preserva a ordem no parágrafo completo, mas é resolvido como 789/espaço/456 isoladamente. L1 precisa conservar entrada lógica e ranges originais para L3 resolver sem perder contexto, mantendo direção dos trechos separada. Foi preparado o pacote L0 público para entidade imutável e campos opcionais de origem em Text/TextSeparator/TextShaped, sem alteração produtiva. Aguarda confirmação ADR-0127 do manifesto. Inventário textual identifica 72 owners candidatos, incluindo sites de teste/comentário; não representa 72 mudanças já feitas. Ver [auditoria e proposta](bidi-contexto-2026-10-05.md). Block/Align e as fronteiras de espaços continuam limites explícitos.

## Materialização do contexto integral — 2026-10-05

O pacote foi ratificado e implementado. L1 conserva input/spans UTF-8; L3 reutiliza resolução completa entre reorder/shaping, sem nova fase/unsafe. 11 controles antes divergentes passaram e nenhum antes positivo regrediu, na matriz de 54 fontes; 47 passam texto e caixas agrupadas. Soft wrap e Linebreak padrão ficaram distintos; duas ocorrências visuais de espaço fraco através de Styled foram corrigidas. 6528 testes L1 (+5 ignorados) e 1165 L3 passaram. CLI pareada não demonstra ganho de tempo. Estado/fontes/hashes/UTC estão em [relatório da materialização](bidi-contexto-implementacao-2026-10-05.md), não neste inventário.

A nova medição separa três heranças restantes: Block/Align (geometria fora do certificado), extração/identidade ToUnicode de controles autorais (tinta correta, cópia divergente), e associação da face real de fallback entre shaping/export (Emph árabe renderiza glifos incompatíveis; Tekt incorpora Oblique, vanilla regular). A última é uma inferência ancorada em fontes/renders/seleção por família e precisa de auditoria da face/GIDs antes de escolher contrato. Coverage::from_ranges permanece proposta não materializada.


### Auditoria da identidade da face — 2026-10-05

A sonda controlada confirmou GIDs da face regular transportados sob estilo italic, que seleciona Oblique na exportação. P534 documentava transportar somente a família real. [Auditoria e L0 preparado](identidade-fonte-shaped-2026-10-05.md): campo público proposto somente em TextShaped; aguarda gate ADR-0127. Não materializado, sem ganho de tempo alegado. Coverage::from_ranges continua independente e pendente.


### Fonte efetiva materializada — 2026-10-05

O contrato foi confirmado e implementado: TextShaped transporta a fonte carregada, CachedRun conserva essa fonte em hits, e recursos/subset/CMap usam binding local por face/eixos. PNG/SVG preservam origem versus instância; sem unsafe novo, API de exportação ou FontKey alterados. 51/58 controles passam texto/caixas agrupadas, dois antes divergentes corrigidos e nenhum antes positivo regrediu. Quatro controles de tinta passam a 600dpi. 6528 testes L1 (+5 ignorados), 1170 L3, integrações e builds passaram. A regressão inicial foi removida; não há ganho consistente de CLI demonstrado. Fonte VF fonte→PDF ainda tem diferença preexistente de caixas, sem causa isolada. Ver [relatório e proveniência](identidade-fonte-shaped-implementacao-2026-10-05.md).


### Eixos variáveis na preparação PDF — 2026-10-06

Corrigida a herança subset-first de P530 após a troca de biblioteca P874: a preparação multifont agora fornece coordenadas à API nativa, conserva nominal/TJ e /W na mesma instância e resolve também o default CFF2 dos espaços. A inspeção visual revelou e eliminou stroke sintético sobre wght já realizado, mantendo o peso semântico. 1.172 testes L3 passaram; 58/64 controles passam texto/caixas agrupadas, seis melhoraram e nenhum positivo regrediu. TrueType e CFF2 coincidem nos controles de tinta a 600dpi dentro de 1px; sem novo unsafe, API ou fase. CLI CFF2: aproximadamente 508→180ms; tabela 320 células sem ganho demonstrado. Lint sem novos diagnósticos. Valores e proveniência completos no [relatório](eixos-variaveis-export-2026-10-05.md).

Restam Block/Align, controles bidi autorais, espaço literal terminal e revisão geral de faux-bold estático. Não se certificam todos os eixos/formatos ou se remove a dependência fontTools nesta rodada. Coverage::from_ranges continua separada.


### Block/Align e linhas causais — 2026-10-06

Separadas posição do container, alinhamento de cada linha e direção dos trechos. Override horizontal privado/lexical com checkpoint; Align textual finito usa origem geométrica e conserva margens causais de entrada/saída. Block automático textual simples reposiciona os frames emitidos sem renderizar novamente o body. Linebreak autoral vazio agora reserva sua linha, conservando o flush técnico vazio como no-op. Alinhamento vertical automático mantém a rota anterior após refutação pelos testes P898. Sem unsafe novo, campo/API pública ou fase nova.

6.536 testes L1 passaram (+5 ignored), 1.172 L3 com SOURCE_DATE_EPOCH=0; builds e lint sem erros ou novos warnings. Duas V20 informacionais novas. 85/88 controles coincidem em texto/caixas agrupadas, 23 melhoraram e nenhum positivo regrediu; quatro renders passam dentro de 1px a 600dpi. CLI pareada sem ganho consistente. Falha temporal de dois testes CMYK foi isolada às datas, sem alterar o produto; permanece dívida independente dos testes. Ver [relatório e proveniência](block-align-2026-10-06.md).

Próxima frente: identidade lógica/extração de controles bidi autorais no PDF. Espaço literal terminal, carriers geométricos/paginados, largura automática e revisão vertical geral permanecem limites explícitos. Coverage::from_ranges continua separada.


### Controles bidi: mapa visual e fallback — 2026-10-06

Corrigidas colisão controle/espaço no ToUnicode, volta ao fallback após shaping
vazio válido e classificação incompleta dos controles na seleção de fontes.
Os dois controles herdados passaram: lote anterior 87/88; 21/24 controles novos,
total 108/112, nenhum positivo regrediu. 1.175 testes L3, builds e diff-check
passaram. Lint sem novos diagnósticos, V15/V26 íntegros; sem novo unsafe/API/fase.
Quatro controles de tinta passam dentro de 1px a 600dpi. Não houve medição de tempo.
Ver [proveniência e limites](controles-bidi-pdf-2026-10-06.md).

Próxima unidade: identidade por ocorrência em clusters que unem LRM/PDI e letra.
Três novos controles continuam divergentes na cópia; espaço literal terminal
permanece aberto no lote anterior. Coverage::from_ranges continua separada.


### Identidade por ocorrência em clusters bidi — 2026-10-06

LRM/PDI unidos a letra agora recebem o envelope TextReplacement existente,
certificado pelo range UTF-8 real do cluster emitido. O mapa global permanece
visual e não contamina outras letras. Filhos conservam fonte/bidi/glifos/avanços/
posição e o walker não aninha em envelope existente. Sem novo unsafe/API/fase.
Os três casos pendentes passaram: 111/112 no lote herdado; 17/17 novos,
total 128/129, nenhum positivo regrediu. Compacto direcionado 20/20.
1.177 testes L3 e builds passaram; lint sem novos diagnósticos, V15/V26 íntegros.
Quatro controles de tinta passam dentro de 1px a 600dpi; tempo não medido.
Ver [relatório e proveniência](bidi-identidade-ocorrencia-2026-10-06.md).

Próxima frente: espaço literal terminal, único controle ainda divergente no lote.
Coverage::from_ranges e refinamentos gerais de contexto/geometry permanecem separados.


### Espaço literal terminal no PDF — 2026-10-06

Corrigido o caso pendente por fronteira de direção exclusiva do PDF ActualText,
certificada pelo span do espaço literal terminal LTR em input RTL. Documento,
input bidi, glifo, posição, avanço e ToUnicode conservados; sem API/fase/unsafe.
Lote herdado agora 129/129, sem regressões. Matriz adicional 11/19, total 140/148;
compacto dirigido 12/20. Oito casos adicionais já falhavam antes (Block RTL
terminal e geometria/quebras de tabela), com fontes e coordenadas registradas.
1.178 testes L3 e builds passaram; lint sem novos diagnósticos, V15/V26 íntegros.
Quatro controles de tinta passam dentro de 1px a 600dpi; tempo não medido.
Ver [proveniência e limites](espaco-terminal-2026-10-06.md).

O caso solicitado está fechado. Próxima auditoria possível: efeito do espaço
literal terminal na largura/alinhamento de Block, com testemunho geométrico
separado da extração, e depois os seis controles de tabela. Não atribuir todos
à mesma causa; Coverage::from_ranges continua independente.


### Fluxo de Block e tabelas — 2026-10-06

Corrigidos os oito casos herdados: medição inline de Sequence, extensão literal
intrínseca de Block RTL, ancoragem de uma coluna e origem bidi/baseline de espaços
no corpo certificado da célula. Lote ampliado 154/168 → 168/168; compacto 26/40 →
40/40, sem regressões. Quatro testes RED→GREEN; 6.540 L1 passaram (5 ignorados),
1.178 L3 passaram; builds/diff-check e lint sem novos diagnósticos, V15/V26 íntegros.
Sem novo unsafe/API/fase. Não houve ganho consistente de tempo: grid320 ~196ms
antes/depois na CLI inteira. Raster conserva pequena diferença nos cantos das
bordas das tabelas; paridade visual exata de junções continua aberta.
Ver [proveniência, causas e limites](block-tabela-fluxo-2026-10-06.md).

Reutilização do produto da medição de fluxo continua como frente separada;
Coverage::from_ranges permanece não materializada e independente.


### Cantos das bordas — 2026-10-06

Corrigida a extensão final de segmentos de borda por meia espessura em cada
extremo; tracks/células/texto conservados. Cantos das seis tabelas herdadas
agora sem lacuna bilateral dentro de 1px a 600dpi. 18/18 controles de tinta
(0,5/1/3pt, uma/duas colunas, LTR/RTL) passam; texto/caixas 18/18 em ambos modos.
Corpus herdado 168/168, sem regressões. 6.542 L1 passam (5 ignorados), integração/
doc-tests/builds/diff-check passam; lint sem novos diagnósticos, V15/V26 íntegros.
Sem novo unsafe/API/fase; tempo não medido. A falha de canto indicada na frente
anterior está fechada nesse recorte. Controles exploratórios em Align expuseram
origem geométrica divergente já anterior; evidência preservada, frente separada.
Ver [proveniência e validação](bordas-cantos-2026-10-06.md).


### Align com Grid/Table e baseline por domínio — 2026-10-06

Corrigida origem geométrica e override físico no corpo certificado Grid/Table
em Align; âncora de colunas usa origem da região e override explícito em uma/
várias colunas, sem inverter ordem lógica. 182/228 → 228/228, sem regressões;
foco 2/48 → 48/48 em verboso/compacto; 12/12 tinta bilateral ±1px a 600dpi.
6.543 L1 passam (5 ignorados), integração/doc-tests/builds/diff-check passam;
lint sem novos diagnósticos, V15/V26 íntegros. Sem novo unsafe/API/fase, tempo
não medido. Ver [correção e limites](align-tabela-2026-10-06.md).

Baseline macro nova no mesmo binário/pin: 36 sondas declaradas, 27 MATCH e 9
DIFF. Aberto: paths de diagnóstico (4), stroke:none em rect/circle (2), HTML
headings (2), recursão de show (1). Query seletiva de heading gerado passa;
fixture recursiva preservada em unidade própria. Não extrapolar percentual
nem somar recortes sobrepostos. Ver [matriz por domínio](paridade-dominios-2026-10-06.md).


### Formas sem stroke e headings HTML — 2026-10-06

Corrigidos None explícito nos quatro construtores de formas fechadas, sem
confundir omissão/fallback: 16/16 controles bilaterais. Corrigidas classificação
de blocos e raiz sob Styled/Sequence e offset do heading HTML: 13/13 DOM.
6.545 L1 passam (5 ignorados), 1.183 L3 passam; integração/doc-tests/builds/
diff-check passam; nenhum novo erro/aviso pelos reparos, V15/V26 íntegros.
Sem novo unsafe/API/fase; tempo não medido. Corpus PDF 228/228 sem regressões.
Macro 27/36 → 31/36; ainda 4 paths de diagnóstico e show-recursion.
Ver [formas](stroke-none-2026-10-06.md), [HTML](html-headings-2026-10-06.md),
[matriz atual](paridade-dominios-2026-10-06.md).

Auditoria de show encontra política α em P348/código/testes/L0, sem presumir
ratificação humana. Oito testemunhos separam alias/finitos de fresh recursivo;
quatro diferenças de aceitação. Proposta L0 por ocorrência no owner apply
aguarda gate de compatibilidade; nenhum código de show alterado e V5 desse
owner é esperado até materialização. Ver [auditoria](show-recursao-auditoria-2026-10-06.md).

## Atualização — show por ocorrência materializado em 2026-10-06

O gate foi confirmado e implementado. Ver `00_nucleo/diagnosticos/show-ocorrencia-reparo-2026-10-06.md` e seu receipt repair/execution.json.gz: 6.551 testes L1, 1.183 L3 e 105 CLI passaram; 228/228 fixtures de layout não regrediram. As quatro gerações ilimitadas medidas agora rejeitam, com aliases e gerações finitas preservados. A matriz macro mantém 31 MATCH/5 DIFF crus; todos os cinco DIFF atuais se limitam ao caminho da fonte, com mensagem/range/hints iguais após projeção explícita. A proposta de apply não está mais aguardando confirmação. Persistem origem de Content constante, caminhos crus, duas fixtures de matriz e seletor table.cell; não se afirma ganho de tempo.

## Atualização — caminhos dos diagnósticos, 2026-10-06

O cálculo relativo de arquivos foi corrigido com pathdiff pinado, sem mudança de API ou unsafe. A matriz macro passou a 36 MATCH/36 nos critérios originais, incluindo stderr completo nos cinco erros antes divergentes, sem projeção. A suíte CLI passou 108 testes; três testes físicos foram repetidos após delimitar a expectativa transiente incorreta. A coleta ampliada de 15 casos mantém seis DIFF explícitos de origem/avisos/stdin. Ver `00_nucleo/diagnosticos/diagnostic-paths-2026-10-06.md`.

A auditoria seguinte encontrou identidade compartilhada de Source entre documento e expressão, com perda de origem em eval contextual; o L0 vigente também determina essa identidade. A frente requer especificação antes do código, conforme o passo `00_nucleo/materialization/origem-fontes-expressao-2026-10-06.md`. Matemática fora de Equation, morfologia de query body, selector math.equation e table.cell permanecem lacunas próprias. Os resultados desta matriz não são percentual global de paridade.

## Atualização — identidade de Source corrigida, 2026-10-06

Após o gate confirmado, eval contextual passou a manter documento, expressão
e stdin com identidades distintas, preservando origem de imports/callbacks e
base de read. A compilação usa o introspector existente sem outra execução do
programa. O alocador não recicla ids nem sobrescreve fontes ao esgotar.

A coleta dedicada subiu de 5/16 para 14/16 coincidências integrais; os dois
negativos restantes são o caret curto de **, já com origem correta. Matriz macro
36 MATCH/36 nos critérios anteriores e 111 testes CLI passaram. Não é percentual
global de paridade nem ganho de tempo. Ver
`00_nucleo/diagnosticos/origem-fontes-expressao-2026-10-06.md` e receipt associado.
As fachadas string legadas conservam compatibilidade; a CLI usa Source-based.
Matemática automática/morfologia/seletores, table.cell, origem de Content
constante, warnings de query e compile stdin continuam em frentes próprias.

## Atualização — carets vazios corrigidos, 2026-10-06

O alcance de **/__ foi corrigido pela relação real entre o marcador de warning
e seu construct na Source numerada, sem alterar a CST ou fases. A coleta de
identidade passou de 14/16 para 16/16 integrais; seis controles adicionais e a
matriz macro de 36 mantiveram seus critérios. Ver
`00_nucleo/diagnosticos/empty-markup-warning-2026-10-06.md` e receipt. Estes dois
negativos estão fechados; demais frentes permanecem conforme seus próprios
diagnósticos. Não é prova de paridade global nem de ganho de tempo.

## Atualização — compile stdin com saída explícita, 2026-10-06

Corrigida a composição de compile - OUTPUT: stdin é adquirido uma vez e
for_stdin conserva Source/base de recursos. Output - agora publica em stdout,
também para input físico; o arquivo físico - não é aberto/sobrescrito.
15/16 controles bilaterais passam nos critérios declarados; a omissão de output
continua DIFF no parser L2 e fica aberta. PDF mede texto/páginas/caixas;
HTML mede DOM; SVG/PNG têm alcance restrito, sem nova certificação de tinta.
115/115 CLI no estado final, três regressões RED→GREEN, builds e diff-check
passam; lint zero erros, 530 warnings, 1578 notes, owners sem V5/V6/V15/V26.
Somente dois owners selados, sem API/fase/unsafe; tempo não medido. Não somar
esta coleta à matriz macro anterior. Ver
`00_nucleo/diagnosticos/compile-stdin-2026-10-06.md` e execution.json.gz.

## Atualização — plano vivo e primeiros débitos, 2026-10-06

O inventário consolidado está em
`00_nucleo/diagnosticos/plano-debitos-2026-10-06.md` e a execução em
`00_nucleo/diagnosticos/plano-debitos-execucao-2026-10-06.md`.
Corrigida a exigência de output para stdin (usage bruto ainda diferente) e
reconhecimento estático de math.equation em query/selector/alias. Filtros,
morfologia matemática, realização automática e table.cell ficam separados.
Warnings aguardam revisão da ordem vigente ADR-0045 e gate da função pública
L2, com L0s candidatos e manifesto completos. 118 CLI no estado entregue,
L1/L2/builds/diff-check aprovados; owners selados sem drift. Coletas não são
somadas à matriz macro histórica; o binário novo não reexecutou essa matriz.
Watch/incremental têm inventário de dependências e plano de medição, sem API
ou ganho de tempo escolhido. A aprovação geral do plano não fecha as frentes.
