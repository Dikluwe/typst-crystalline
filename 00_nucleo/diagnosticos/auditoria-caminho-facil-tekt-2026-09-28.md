# Auditoria ampla: onde o Tekt tomou o caminho comum e fácil

## Critério e alcance

Pedido do dono: procurar, em todo o Tekt, desenvolvimento que tomou o caminho comum e fácil, adiando implementação mais exigente. O critério inclui construção sem consumo, parâmetros aceitos sem efeito, algoritmos locais/simplificados, aproximações e validação que não enfrenta os casos difíceis. A simplicidade pode ser a escolha correta; cada caso precisa de evidência de limitação, alternativa e critério que possa refutar a necessidade de mudança.

O resultado é uma triagem global com leitura aprofundada dos principais sinais, não uma certificação semântica de cada unidade. Foram inventariados todos os arquivos .rs/.md/.toml de L1–L4, L0 e ADRs, excluindo target. Testes estão incluídos na triagem e foram distinguidos na revisão manual.

| Área | Arquivos inventariados | Arquivos Rust | Resultado desta leitura |
| --- | ---: | ---: | --- |
| L1 | 551 | 549 | Limitações confirmadas em layout, stdlib, closures e matemática; parsing/entidades também triados. |
| L2 | 10 | 9 | Comentários de capacidade desatualizados; nenhum novo atalho comportamental confirmado nesta leitura. |
| L3 | 129 | 126 | Aquisição/reuso de recursos, projeções aproximadas e testes dependentes de fontes do sistema. |
| L4 | 26 | 25 | Novo World por ciclo de watch; composição e publicação atuais conferidas. |
| L0 | 745 | 0 | Owners e núcleos inventariados; contratos dos achados lidos; limites parciais explicitamente normativos. |
| ADRs | 119 | 0 | Inventário completo; decisões de paridade, disciplina, atomização, gate e perfil parcial consideradas. |

Proveniência: working tree não commitada sobre HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`. UTC, status completo, git diff HEAD --stat e hashes integrais estão em [proveniencia-inicial.json](auditoria-atalhos-tekt-2026-09-28/proveniencia-inicial.json). [proveniencia-final.json](auditoria-atalhos-tekt-2026-09-28/proveniencia-final.json) registra a contagem acima e confirma que nenhum arquivo do inventário foi alterado durante a auditoria. Não foram executados novos builds, testes, benchmarks ou sondas de paridade. As constatações de comportamento abaixo são de inspeção estática, salvo evidências históricas explicitamente identificadas.

O pin de paridade permanece `9f2b6e8715237cb086899a42873660fe744622e8`. Fontes em lab são referências locais com hashes registrados; a identidade dessa árvore com a ratificação upstream não foi revalidada aqui. Não há novo fecho de paridade ou promessa de performance.

## Evidência antes da classificação

### A. Matemática: exceção de produção para satisfazer cenário artificial de teste

- `01_core/src/compiler/math/layout/accent.rs:166–171`: comentário diz que uma variante artificialmente esticada de teste unitário exige expansão; a produção escolhe largura do acento quando ela excede duas vezes a largura da base.
- `01_core/src/compiler/math/layout/tests.rs:2965–2991`: o teste injeta variante com advance 50_000 e exige que a largura resultante domine mais de duas vezes a base.
- `00_nucleo/prompts/compiler/math/layout/accent.md:39`: reconhece a heurística de largura de acento grande e atribuição incompleta de attachment como limitações de auditoria, não novas exceções normativas.
- Git blame liga a regra/comentário ao commit `770e191419`; a saída exata está em [origem-heuristica-acento.txt](auditoria-atalhos-tekt-2026-09-28/origem-heuristica-acento.txt).

**Classificação:** evidência direta de acomodação da regra produtiva ao teste. É o achado mais forte desta auditoria; não depende de supor intenção. Uma razão numérica não representa por si só o estado semântico `exact_frame_width` citado pelo próprio comentário.

**Trabalho difícil adiado:** atribuição correta de frame/attachment do glifo escolhido, inclusive variante esticada, em vez de uma exceção por dimensão artificial. `compiler/layout/metrics.rs:186` registra que a interface por char usa o attach do glifo base para variantes; isso é uma limitação separada.

**Como decidir:** testar largura/attachment com fontes reais, variantes e acentos compostos, incluindo controles dos dois lados do limiar e sua composição em scripts/aninhamento. A necessidade de reparo é refutada se a regra puder ser derivada do contrato tipográfico para toda essa família; o teste sintético isolado não fornece essa derivação. Ler/atualizar os owners antes de qualquer código; mudança de trait público passa por ADR-0127.

### B. Dimensões relativas: resolver o problema descartando informação

- `01_core/src/compiler/stdlib/layout.rs:860–873`: extract_length converte Relative somente pela parte absoluta e Ratio em Length::ZERO.
- `:911–916`: extract_sides_from_value registra truncamento da parte relativa por falta de contexto de layout.
- `:1629`, `:1635`, `:1924`, `:1930`: block/box usam esse helper para inset/outset.
- `00_nucleo/prompts/compiler/stdlib/layout.md:387–391`: autoriza truncamento e explicita que inset de 50% vira zero.
- `layout.rs:878–886`: outro caminho já conserva percentagem em block(width:); a limitação não é uniforme em toda a linguagem.

**Classificação:** atalho ativo com perda de informação sem erro. O motivo técnico está explícito: o construtor não tem contexto para resolver. O trabalho exigente é conservar a expressão até o consumer que tenha região e estilo causal.

**Como decidir:** controles com região finita, percentagem pura e dimensão mista, nested block/box e interação com em; preservar semântica e geometria. Não aplicar o truncamento existente como critério de aceitação. Mudanças nos carriers públicos exigem L0 e gate apropriado. Não propõe mover resolução de regiões para eval.

### C. Flags weak: implementar a superfície e adiar o comportamento

- `compiler/stdlib/layout.rs:1312`, `:1413`, `:1437`: valida e preserva weak nos espaçamentos.
- `compiler/layout/pagebreak.rs:16–55`: lê to, mas não weak; o L0 `compiler/layout/pagebreak.md:21` declara isso expressamente.
- Os owners de h_space/v_space/colbreak e seus consumers conservam a limitação weak diferido.
- `00_nucleo/prompts/compiler/stdlib/layout.md:728`: collapse de pagebreak weak adiado.

**Classificação:** implementação de parsing/estrutura concluída com efeito semântico adiado. Manter a presença para repr é útil, mas não completa o comportamento.

**Como decidir:** matriz de quebras/espaços fracos nas bordas, adjacências, início/fim de página e colunas, com controles sem weak. Definir o colapso causal e sua interação com conteúdo e region progress. Não inferir comportamento esperado só por comparação com o flag false.

### D. figure.caption e footnote.entry: constructors públicos sem integração completa

- `compiler/stdlib/figure_image.rs:87`: receber FigureCaption extrai somente fc.body para a figura.
- `00_nucleo/prompts/compiler/stdlib/figure_image.md:157–174`: position/separator preservados no valor original, mas sem efeito na figura; síntese e realização explicitamente excluídas.
- `compiler/layout/mod.rs:1625`: FigureCaption isolada não emite conteúdo.
- `00_nucleo/prompts/compiler/stdlib/structural/footnote.md:98–116`: entry guarda separator/clearance/gap/indent sem efeito visual, e o renderer continua a gerar notas sem consultar esse constructor.
- `compiler/layout/mod.rs:1683`: FootnoteEntry isolada também é no-op.

**Classificação:** escopo reduzido explícito. O caminho fácil foi completar construção e inspeção do valor; o trabalho difícil de integração ficou para depois. Ser transparente no L0 não converte a fatia em funcionalidade completa.

**Como decidir:** medir construção, incorporação em figure/footnote, síntese, show rules e resultado final como obrigações distintas. Ex.: position top precisa de efeito na legenda incorporada, não só sobreviver em fields do valor inicial. Verificar lifecycle antes de modificar os renderers.

**Questão adicional de L0:** figure_image.md:171–174 chama a diferença do erro not known at this point de divergência mecânica aceitável, embora ressalve que não alega paridade de linguagem. O diagnóstico observável é a exceção explícita de ADR-0107/0108: a classificação precisa ser revista. Isto não autoriza completar a feature sem o owner.

### E. Versaletes: simulação por maiúsculas reduzidas

- `compiler/layout/text.rs:236–253`: separa minúsculas, aplica to_uppercase e escala 0.8.
- `00_nucleo/prompts/compiler/layout/text.md:23`: preserva essa regra como contrato vigente.
- `compiler/layout/measurement.rs:150–152`: no dispatch de medição restrita, SmallCaps mede o body sem essa transformação; a medição intrínseca por layout real é outro caminho.
- `compiler/stdlib/text/smallcaps.rs:24–27`: comentário histórico registra shaping OpenType como XL scope-out; a descrição de consumer transparente ficou antiga porque render por scaling já existe.

**Classificação:** aproximação visual ativa. O trabalho exigente é integrar versaletes da fonte e fallback causal, com medição/emissão coerentes. A diferença estática entre caminhos de medição é candidata a defeito, ainda sem sonda discriminante nova.

**Como decidir:** fontes com/sem versaletes, scripts/idiomas e expansão de caracteres ao converter caixa; medir largura, quebra e texto/morfologia. Simular com scaling pode ser um fallback legítimo, mas não substitui a avaliação da capacidade da fonte.

### F. Parágrafos: escolher a quebra que cabe agora

- `compiler/layout/cursor.rs:195–242`: mede a palavra corrente; quando não cabe, usa o maior prefixo hifenizado que cabe ou fecha a linha.
- `00_nucleo/prompts/compiler/layout/cursor.md:27`: conserva measured wrapping e fallback de hifenização.
- A referência local `lab/typst-original/.../inline/linebreak.rs:152–159` distingue Simple/Optimized; `:210–240` contém planejamento ao estilo Knuth–Plass. `inline/mod.rs:194–195` escolhe Optimized para texto justificado por default.

**Classificação:** mecanismo local simples confirmado. Não há nova demonstração de divergência linguística ou de qualidade tipográfica nesta auditoria. A referência mostra uma alternativa concreta a avaliar, não obrigação de copiar seu algoritmo.

**Trabalho difícil adiado:** considerar o parágrafo inteiro e restrições de espaçamento/hifenização, em vez de decisões locais irreversíveis.

**Como decidir:** corpus justificado, palavras longas, tamanhos mistos, links, RTL, inline boxes e idiomas. Avaliar geometria, uniformidade de espaços e custo/memória, sem contar bytes iguais como sucesso. A hipótese é refutada se o contrato/corpus real já for satisfeito pelo caminho local ou o custo da alternativa não compensar. Atualizar L0 antes de mudar o mecanismo; nova política/default requer gate.

### G. Closures: capturar o scope inteiro, reexecutar a chamada

- `compiler/eval/closures.rs:192–196`: snapshot eager de scopes; integração com comemo fica para trabalho futuro.
- `:40`: aplicação prepara scope/argumentos e avalia sem fronteira de memoização nessa função.
- `00_nucleo/prompts/compiler/eval/closures.md:24` e `entities/func.md:138`: snapshot lexical imutável é vigente.

**Classificação:** alternativa simples de captura e aplicação. Não foi provado que esteja errada; o risco é custo/retensão de valores irrelevantes e ausência de reutilização onde seria válida. Captura eager não é sinônimo de erro semântico.

**Como decidir:** scopes com valores grandes não referenciados, muitas closures e chamadas repetidas; provar captura/invalidação antes de otimizar. Medir memória retida, definição, chamada e recompilação. Não usar igualdade estrutural de Func como substituto de dependências causais.

### H. Notas de rodapé: encaixar corpos inteiros e forçar o que nunca cabe

- `compiler/layout/footnote_flush.rs:98–162`: greedy measure-then-fit; resto deferido; corpo maior que a área completa pode ser forçado a emitir para assegurar progresso.
- Owner real: `00_nucleo/prompts/compiler/footnote_overflow_columns.md`; o header confirma a correspondência. O L0 preserva o fallback forçado com warning.

**Classificação:** overflow entre páginas já existe, contrariando comentários antigos no hub. A limitação ativa é tratar cada corpo como uma unidade; não dividir uma nota longa conforme a região.

**Como decidir:** notas maiores que uma região/página, múltiplas notas, chamadas perto de fim de página e colunas, reserva/continuação sem sobreposição. A proteção de progresso é necessária; a alternativa difícil é fragmentação e coordenação com o fluxo principal. Não remover proteção antes de provar progresso do novo mecanismo.

### I. Campos de lista: marker_align fica sem consumer

- `00_nucleo/prompts/entities/elements/list_item.md:20`: campo aceito/preservado sem efeito visual.
- `compiler/layout/list_item.rs:49–79`: posiciona marcador por indent, largura e body_indent, sem consulta a marker_align.

**Classificação:** limitação delimitada. indent/body_indent/tight já têm consumer; acusar todos os campos seria falso.

**Como decidir:** marcadores de larguras variadas, alinhamentos e quebras em itens; verificar o efeito causal e distinguir propriedades do item e do container antes de completar o consumer.

### J. Resumos de query: aproximação auxiliar sem cardinalidade real de parágrafos

- `03_infra/src/query_helpers.rs:229–235`: parágrafo é presença booleana de texto, retornando zero/um.
- O L0 `infra/query-helpers.md` mantém approximate summary separado de query_elements/LocatedContent; o caminho localizado não deve receber contagens inventadas.

**Classificação:** helper aproximado ativo. Não é prova de que toda query CLI use essa cardinalidade. O perigo é reutilizar esse resumo como evidência completa de paridade.

**Como decidir:** localizar os consumidores de QuerySummary em relatórios/harnesses, exigir identificação de projeção aproximada e usar resultados reais quando a pergunta for cardinalidade linguística.

### K. Fontes e watch: lazy dentro de uma compilação, reconstrução entre ciclos

- `03_infra/src/world/fonts.rs:65–105`: primeira demanda constrói catálogo completo; OnceLock retém dentro daquele World.
- `04_wiring/src/watch.rs:63` chama compilação em cada ciclo; `04_wiring/src/compile.rs:246` cria novo World.
- Owners atuais preservam essa composição. Limites e alternativas detalhados em [oportunidades anteriores](oportunidades-estado-da-arte-2026-09-28.md).

**Classificação:** ciclo de vida simples com reutilização restrita. Não é rebuild em toda chamada book; é nova instância por ciclo. O trabalho exigente é sessão com invalidação de fontes/configuração/fontes de texto/erros/assets. Conservar World cegamente introduziria stale sources e erros.

**Como decidir:** repetir sem editar e alterar cada classe de dependência separadamente, comparando com execução fresca e memória limitada. Tempos históricos exploratórios não provam o ganho atual.

### L. Shaping: resultados em cache, preparação reconstruída no miss

- `03_infra/src/shaper/shaping.rs:243–289`: miss cria Face/UnicodeBuffer e chama shape; o cache de faces em cache.rs é ttf-parser e não substitui diretamente Face de rustybuzz.
- Width/emission têm contratos distintos; linguagem e eixos já receberam correções recentes.

**Classificação:** integração simples de um motor sofisticado. A investigação anterior registra planos/buffers como candidata; a eficácia depende da distribuição de runs distintos/repetidos. A atual suficiência da identidade de script/math ainda é questão separada não provada pelo reparo de idioma.

**Como decidir:** medir preparação, alocações e reaproveitamento com comportamento correto; não unificar medição/emissão por conveniência nem criar cache global fora do contrato.

## Atalhos na validação e no processo

### Testes que podem passar sem executar o cenário pretendido

`03_infra/src/shaper/tests.rs:500–511` retorna cedo quando faltam fontes/catálogo. `font_metrics/fallback.rs:1217–1220` também retorna cedo nesse caso. O runner normalmente registra sucesso se a função simplesmente retorna; nesta auditoria não se mediu em qual ambiente esses caminhos foram tomados.

Isso enfraquece prova baseada só na contagem de testes verdes. Os novos controles obrigatórios de shaping já exigem fixtures versionadas em `00_nucleo/prompts/infra/shaper/tests.md:31`; os controles antigos precisam ser distinguidos de testes obrigatórios e receber cobertura determinística quando usados como evidência. Não generalizar que toda a suíte de fontes tenha esse problema.

### Contratos que conservam o limite em vez de conduzir à completude

Exemplos atuais: L0 de pagebreak declara que weak não é inspecionado; text fixa SmallCaps a 0.8; caption/entry declaram construção sem realização. Isso evita código órfão, mas também pode estabilizar indefinidamente o comportamento parcial se a unidade for dada como concluída sem registrar a obrigação seguinte.

A ADR-0054 original explicitamente escolheu graded pelo alcance do lote e deixou shaping XL fora do fecho; também registra TextStyle como caminho de menor resistência. É evidência histórica de escolha por viabilidade. Shaping real foi posteriormente integrado: não usar a decisão antiga como justificativa automática de uma limitação atual. ADR-0107/0108 exigem nova decisão sustentada pela linguagem e fonte atual.

### Linter sem garantia de completude comportamental

`crystalline.toml:63–65` configura V5/V6/V7 como warning. V15/V26 são error. A configuração de severidade precisa ser considerada ao relatar zero erros: isso não equivale a zero violations nem a implementação completa. Nenhum lint foi executado nesta auditoria, e não se afirma que exista V5 hoje.

Ownership/hash/topologia são obrigações úteis. Um consumer pode obedecê-las e ainda devolver zero, ignorar uma flag conforme L0 ou usar aproximação. O gate de produto precisa exercitar o comportamento que está em causa.

## Sinais rejeitados ou reclassificados

- Regex query retorna vazio no Introspector, mas texto/regex são rejeitados como não locatable no caminho público pertinente. Não classificar esse arm sozinho como bug de query.
- Where legado retorna vazio; o selector Element atual possui matching por fields. A conversão para show usa outro tipo Selector. Não concluir que heading.where está inteiramente sem consumer só por encontrar o arm legado.
- Table/grid já têm repetição de header/footer visual no owner atual; comentários antigos dizem não implementado. Não reabrir essa dívida inteira sem controlar os casos realmente limitados.
- Transform mede sub-layout real; measure_content que devolve zero para vários casos só entra como fallback quando faltam items/dimensão. Não rotular todas as transformações como aproximação zero.
- Bibliografia já tem caminho CSL; comentários de placeholder no hub são históricos.
- CLI enum diz que PNG/SVG não estão implementados, mas os backends atuais existem. É documentação antiga, não ausência da feature.
- Aproximações numéricas com limite explícito, curvas Bézier para arcos, mocks de testes, placeholder de sintaxe e defaults neutros não constituem atalho defeituoso por mera ocorrência textual.
- Enum fechado/match exaustivo e atomização na camada de render são decisões deliberadas sob ADR-0109; esta auditoria não fornece razão para substituí-las por despacho dinâmico.
- SVG registra fallbacks de paints por classificação e data attributes; isso é suporte parcial declarado, não alegação comprovada de falha silenciosa.

## Ordem de trabalho sugerida pela auditoria

| Frente | Por que vem antes | Próximo resultado verificável |
| --- | --- | --- |
| Heurística de acentos/testes | Produção explicitamente acomodada ao cenário artificial; L0 reconhece limite. | Contrato de largura/attachment derivado da tipografia e regressão discriminante. |
| Percentagens e weak | Entradas públicas aceitas com informação/efeito perdidos. | Matriz de casos na linguagem e consumers que preservem o significado. |
| Caption/entry e marker_align | API disponível com obrigação final adiada. | Integração comprovada, em vez de somente construção/fields. |
| Smallcaps e parágrafos | Soluções simples afetam tipografia cotidiana. | Comparação de comportamento/qualidade/custo com corpus representativo. |
| Closures, fontes e watch | Ganho depende do trabalho reutilizável e da invalidação. | Perfil reproduzível e prova de equivalência entre fresco/reutilizado. |
| Notas longas | Exige coordenação de regiões e progresso. | Fragmentação correta, sem sobreposição e sem loop. |
| Validação/governança | Sustenta todas as frentes anteriores. | Testes determinísticos e fecho separado de construção, efeito, paridade e desempenho. |

As prioridades não são medições de severidade, custo ou speedup; são julgamento técnico a partir das evidências precedentes. Cada implementação ainda exige L0 vigente, RED→GREEN e as validações do repositório. Os escopos com contrato público/default/fase/compatibilidade passam pelo gate ADR-0127. Nenhuma alteração de código/L0 foi feita aqui.

O padrão transversal encontrado é: **construir e registrar é mais barato que integrar, e preservar o comportamento atual é mais barato que provar o comportamento completo**. Há casos em que essa divisão foi útil e depois superada; há casos em que a obrigação difícil continua pendente. A auditoria permite distinguir os dois.

Arquivos de apoio: [triagem original](auditoria-atalhos-tekt-2026-09-28/triagem.json), [sinais ampliados](auditoria-atalhos-tekt-2026-09-28/sinais-ampliados.json), [mapa de owners extraídos dos headers](auditoria-atalhos-tekt-2026-09-28/owners.json). As listas regex são pistas, incluem comentários/testes e não são contagens de defeitos.
