# Auditoria da dupla avaliação

Auditoria da Frente A do [inventário de heranças](herancas-reescrita-frentes-2026-10-05.md). A dupla avaliação surgiu como solução tática para preservar elementos consumidos por regras `show`, tornou-se contrato L0 e ganhou consumidores e compensações posteriores. Seu custo está demonstrado em estudos anteriores; esta auditoria também reproduziu falhas de introspecção no pin atual. Remover apenas a primeira chamada não é uma correção suficiente.

O usuário esclareceu que evita `unsafe` porque o código é escrito pela IA: deseja conservar as garantias verificadas pelo compilador. A auditoria mantém essa preferência e o código explícito. Nenhuma alteração produtiva ou de L0 foi feita.

## Estado e método

HEAD Tekt: `06848cdcb5450f2c555626bf116bc7acd82bd82b`, árvore rastreada sem alterações no início das verificações. Os arquivos não rastreados da sessão estão enumerados no recibo. Os controles executados em 2026-10-05 têm timestamps UTC de início e fim, comandos, fontes completos, saídas, códigos de retorno e hashes dos executáveis em [results.json.gz](dupla-avaliacao-auditoria-2026-10-05/results.json.gz). A [sonda reproduzível](dupla-avaliacao-auditoria-2026-10-05/probe.py) gera os casos localmente; os fontes `.typ` são ignorados pelo repositório, mas seu conteúdo está preservado no script e no recibo.

- Tekt release SHA-256: `eec9edd5ded17b0cd58c3297d313ff5796694899b39d5b87f6c5426f5dbc8e49`.
- Vanilla pin: `9f2b6e8715237cb086899a42873660fe744622e8`.
- Vanilla release SHA-256: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, árvore upstream e receita ratificadas no P1489.

São controles linguísticos, não novos benchmarks. Tempos anteriores são usados somente com a proveniência dos relatórios [macro e micro](macro-micro-2026-10-05.md) e [paginação](grid-pagination-cost-2026-10-05.md). Não extrapolar medições de CPU de uma fixture para toda a CLI.

## Origem e permanência

O [P498](../materialization/typst-passo-498.md), de 2026-06-29, tratou um heading que desaparecia da introspecção após `#show heading.where(...): it => upper(it.body)`. A intercepção durante eval substituía o elemento original pelo resultado visual.

O plano recomendava preservar conteúdo pré-show. O [relatório da implementação](paridade-funcional-p498.md:33) registra “H1 adaptada — passagem dupla”: executar o programa sem aplicação de show para obter o original e novamente com show para obter o renderizado. A justificativa documentada foi evitar refatoração profunda e alterações nos elementos individualmente. A implementação está nos commits históricos `a63e739d288125d477fa6a8fe81a87a1b384a560` e `b3e11fe2d2b817b39f6ff83a8f7cd77e1bf24a21`; o segundo está no histórico com autoria humana. Autoria de commit e a palavra “aceite” no relatório não comprovam deliberação humana específica sobre o custo ou a solução arquitetural.

O [P507](../materialization/typst-passo-507.md) já nomeava a passagem dupla como custo conhecido e propunha medi-la. Sua enumeração das fases não deve ser tomada como descrição fiel da ordem atual: hoje as duas avaliações acontecem dentro da entrada eval, antes de ela retornar ao pipeline.

**Classificação:** o requisito de linguagem é manter elementos observáveis pela introspecção após sua transformação visual. A repetição integral do programa é mecânica de execução escolhida para satisfazê-lo. Preservar o requisito não implica preservar a repetição.

## O que acontece hoje

Em `01_core/src/compiler/eval/mod.rs:777–865`, cada `run_pass` cria contexto, rota, estilos, regras, guards, biblioteca base, escopos e Engine. A primeira chamada, em `:872`, desliga aplicação de show; a segunda, em `:880`, liga. O arquivo é parseado/verificado antes desse par; não afirmar que o parser principal também roda duas vezes. Os inputs principais são adquiridos uma vez em `:776`.

A primeira passagem conserva apenas seu conteúdo. Scope, estilos bibliográficos e metadados retornados por ela são descartados. A segunda fornece conteúdo, scope e demais resultados oficiais. As duas escrevem no mesmo Sink; deduplicação por span e mensagem não evita execução repetida. Uma falha da primeira passagem retorna imediatamente e impede a segunda.

No caminho de sucesso, o programa é avaliado duas vezes mesmo sem show. Isso inclui cálculos, definições, chamadas e travessias que produzam os mesmos valores. Contextos frescos explicam por que mutações locais podem produzir o mesmo resultado final em ambas: resultado final correto não prova execução única. Serviços World podem ter cache próprio, portanto repetição de uma chamada não implica automaticamente repetir seu I/O físico.

## Dependências e propagação encontradas

| Unidade | Dependência observada | Consequência para a correção |
| --- | --- | --- |
| `entities/module.rs:39,97–104` | Carrier público `introspection_content`, além de `content` | Não remover o getter/setter ou mudar significado sem auditar consumidores e contrato |
| `compiler/eval/rules/intercept.rs:24,143,181` e `rules/textual.rs:56,87` | A flag desliga transformações na passagem original | O original precisa ser preservado por outro mecanismo antes de retirar esse caminho |
| `pipeline/paged.rs:232–242` | Constrói introspector do original e expande o conteúdo renderizado | A correção deve transportar identidade, localização e contexto compatíveis |
| `query_helpers.rs:469,511–520` | Constrói índices das duas árvores e escolhe um deles | Escolha por quantidade não representa todos os elementos; falha reproduzida abaixo |
| `compiler/eval/modules.rs:44–114,173` | Imports têm avaliação isolada e recomposição própria de base, sem memoização neste helper | O principal visita imports novamente na outra passagem; imports repetidos acrescentam trabalho por ocorrência |
| `compiler/eval/modules.rs:303–316` | Includes chamam `eval_markup` no contexto do chamador | O conteúdo incluído participa de ambas as passagens, sem iniciar necessariamente outro par por include |
| `compiler/eval/call_dispatch/invoke.rs:64–115` | Matrizes usam a flag para isolar escopos e truncar arrays em situações específicas | Há lógica de compensação dependente da passagem; preservação exige teste de efeitos e ordem |
| `compiler/eval/math/native_dispatch.rs:250–265` | O caminho bare de `mat` salva/restaura escopos quando show está desligado | Também precisa ser revisto junto da coordenação de avaliação |
| `pipeline/html.rs:91`, `bundle.rs:102`, `query_helpers.rs:497` | Chamam o mesmo adaptador de avaliação de documento | A duplicação não está limitada ao PDF |

O helper de imports define contexto local sem aplicar show porque só devolve bindings. Isso não é, por si, outra dupla avaliação interna. Tampouco se deve concluir crescimento exponencial apenas porque há imports: é necessário contar ocorrências e profundidade do grafo.

As compensações de `mat` entraram no commit `cf92c3d07`, de materialização do P1431. A dependência da flag é comprovada pelo código. A auditoria não demonstrou que todas as compensações são causadas exclusivamente pelo P498, nem que podem ser removidas sem outro reparo no avaliador de argumentos.

A heurística de query entrou no commit `f4d0e5c7ae33d53e657d5b3ff89d30a607bd2a64`, de 2026-08-31. Seu comentário cita expressamente o P498 e a necessidade de incluir elementos criados por receitas show. É evidência direta de adaptação posterior às duas árvores.

A repetição de layout paginado é um mecanismo diferente. Nesta auditoria não foi encontrada prova de que sua origem seja a existência da dupla avaliação. A chamada a eval em `context_stabilization.rs:1160` está sob `#[cfg(test)]`; não é reavaliação produtiva do documento durante a estabilização.

## Falhas de introspecção reproduzidas

Os controles usam `query heading --field body`. As comparações abaixo resumem os corpos textuais, preservados integralmente no recibo. Ambos os motores retornaram sucesso em todos estes comandos, apesar das divergências.

| Caso | Tekt | Vanilla pinado | Resultado |
| --- | --- | --- | --- |
| Show substitui o heading por texto em maiúsculas | Original | Original | A correção que motivou P498 continua funcionando |
| Show substitui o heading por um novo heading | Original | Original, Generated A | Tekt perde o elemento gerado |
| Show substitui o heading por dois novos headings | Generated A, Generated B | Original, Generated A, Generated B | Tekt perde o original |

No primeiro caso com geração, os índices têm a mesma quantidade e o Tekt escolhe o original. No segundo, o renderizado tem mais elementos e o Tekt escolhe esse índice, descartando o original. A cardinalidade não estabelece equivalência de identidades nem união de elementos.

**Conclusão medida:** duas árvores avaliadas separadamente e escolha do maior índice não bastam para cumprir o observável da linguagem nestes casos. Não concluir que uma união simples resolveria: identidade, ordem, labels, localizações e elementos de receitas recursivas também precisam de critérios.

## Testes existentes e limites dos controles

`compiler/eval/tests.rs:12376–12404` contém o teste P498 que exige um heading original apesar da transformação. É uma sentinela útil do requisito, mas exige o carrier atual e não cobre elementos novos produzidos pela regra. A aceitação de uma nova solução deve começar por observáveis de linguagem e manter esse caso.

`compiler/eval/p1431_tests.rs:148–168` testa ordem e efeitos de argumentos de matriz por meio do array retornado no scope final. Isso verifica o resultado oficial, não quantas vezes o documento inteiro executou o código.

A auditoria executou também dois documentos com `math.mat`: um duplica intencionalmente um array e afirma seu tamanho, outro acrescenta um marcador e afirma o resultado. Ambos passaram nos dois motores. Não houve falha de matrizes demonstrada. Esses controles não autorizam retirar as compensações nem atribuir-lhes um bug específico.

Ainda faltam controles causais de contagem por serviço World, closures capturadas, múltiplos imports, falhas em cada passagem e efeitos dentro de receitas show. Esta auditoria não executou toda a suíte de paridade, não mediu alocações nem prototipou a remoção da passagem.

## Comparação com o vanilla

No pin atual, `lab/typst-original/crates/typst/src/lib.rs:124–132` avalia o documento principal antes do loop de estabilização. `typst-eval/src/lib.rs:38–96` memoiza essa avaliação. A avaliação de markup guarda receitas em conteúdo estilizado (`typst-eval/src/markup.rs:43–50`); a realização aplica as receitas, prepara localização e conserva tags do elemento enquanto visita o conteúdo produzido (`typst-realize/src/lib.rs:353–430`).

Portanto, o vanilla mantém a relação entre elemento original e resultado visual durante a realização. Não recupera essa relação avaliando todo o programa outra vez e escolhendo a maior de duas listas. Esse comportamento pode informar uma solução Tekt segura e explícita, sem copiar vtables ou `unsafe`.

## Auditoria do L0 e da linhagem

O [L0 vigente de eval](../prompts/compiler/eval.md:90) exige literalmente passagens independentes com contextos e escopos frescos. O [L0 de Module](../prompts/entities/module.md:51) exige os dois carriers, mas a existência de dois carriers não impõe duas execuções. O [L0 de query](../prompts/infra/query-helpers.md:46) também conserva a heurística que falhou nos controles atuais. Assim, uma solução antiga foi incorporada à especificação e sua correção precisa revisar a substância do L0, não apenas o código.

`crystalline-lint --checks v5,v15,v26 .` foi executado sem reparos automáticos. O recibo e os SHA-256 completos dos L0 lidos estão em [lineage.json.gz](dupla-avaliacao-auditoria-2026-10-05/lineage.json.gz). Há avisos V5 no código produtivo, inclusive `compiler/eval/mod.rs`: hash calculado pelo linter `5f0b02e6`, header `7a024574`. Não declarar linhagem limpa com base no código de saída. A auditoria não ressela hashes para ocultar essas divergências; antes de materialização, é necessário verificar sua causa e alinhar L0 e consumer.

## Frentes concretas de correção

1. **Introspecção completa.** Transformar os casos de heading gerado em sentinelas de linguagem e especificar identidade, ordem e localização dos elementos originais e produzidos. Isso expõe o problema que a arquitetura precisa resolver, além do custo.
2. **Avaliação única com preservação de origem.** Mapear onde cada transformação consome o original e como sua relação com o resultado pode sobreviver. Uma possível direção é registrar essa relação durante a transformação; ainda não é uma solução escolhida. Não basta clonar o conteúdo final, pois os originais já podem ter sido substituídos.
3. **Efeitos e compensações.** Contar avaliações efetivas com um World de teste e revisar os caminhos de matrizes, erros, warnings, imports e includes. Distinguir efeitos por avaliação de efeitos por realização/contexto.
4. **Base compartilhada.** Investigar separadamente a composição repetida de stdlib e registros. Pode reduzir trabalho dentro do desenho atual, mas não fecha a dupla avaliação nem os gaps de introspecção.

Para a frente principal, o critério de conclusão é preservar a linguagem enquanto o documento comum deixa de executar integralmente duas vezes. Contagem de avaliações e benchmark com proveniência verificam a economia; testes de query, efeitos, contexto, HTML e bundle verificam o comportamento.

Atualizar os L0 proprietários antes de implementar. Uma solução que mova aplicação de show de eval para realização/layout exige o gate explícito de fase da ADR-0127; alterações públicas do carrier também exigem esse gate. O resultado desta auditoria é diagnóstico e conjunto de requisitos para essa decisão, sem escolher ou materializar a mudança arquitetural.
