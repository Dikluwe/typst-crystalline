# Prompt — typst-passo-861: verificação de estado — `Duration`, achado #41, e itens registrados de P859

**Origem**: levantamento do que ainda está em aberto no projeto, sem relatório de confirmação recente para alguns itens
**Estado**: aguardando execução — **isto é verificação de estado, não implementação de nenhum achado novo**. O objetivo é saber com certeza o que está feito, o que está em andamento, e o que nunca foi tocado — antes de decidir o que fazer com cada um.

---

## Item 1 — Status real da migração de `Duration`

A decisão de migrar `Duration` para representação com sinal (Opção A de P850) foi tomada, e o dono relatou que a execução já tinha começado em paralelo a outros passos desta sequência. Nenhum relatório de conclusão chegou depois disso.

1. Verificar o estado real da árvore: `git status`/`git log` para ver se há commits ou working tree com alterações relacionadas a `Duration` (`01_core/src/entities/duration.rs`, e os outros pontos listados no relatório de P850 — constructor, operadores, repr, field access, cast).
2. Se houver trabalho em andamento (branch separada, working tree com mudanças não commitadas, ou parcialmente aplicado): descrever o estado exato — quanto da lista de 7 pontos do relatório de P850 (§3.1 a §3.7) já foi tocado, e quanto falta.
3. Se não houver nenhum vestígio de trabalho: relatar isso claramente — a migração pode não ter de fato começado, apesar do que foi comunicado nesta conversa. Não assumir que "já está em execução" é verdade só porque foi dito; confirmar com o estado real do repositório.
4. Testar o caso que motivou tudo (`#repr(-duration(seconds: 3))`) no binário atual, independentemente do que o `git status` mostrar — o teste concreto é a prova mais confiável.

## Item 2 — Achado #41 (F2, de P843) — fusão de texto no parser

1. Confirmar que o achado continua sem correção: `#repr([hello world])` no cristalino continua produzindo um único nó `Text`, enquanto o vanilla produz `Text+Space+Text` (três nós).
2. Confirmar a ligação que P859 estabeleceu: esse achado é sintoma da ausência de uma fase de composição de conteúdo (o vanilla mantém a granularidade até a fase `realize`, que depois reagrupa).
3. Não corrigir neste passo — só confirmar o estado atual e reunir, num só lugar, as referências already espalhadas entre P843 e P859 sobre esse achado específico.

## Item 3 — Itens do Grupo 2 de P859 (sem sintoma observado)

Para cada um dos quatro itens que P859 listou como "não coberto, sem sintoma observado ainda" — ausência de `ParElem`, ausência de agrupamento de listas/citações, símbolos matemáticos fora de `$...$` não envolvidos automaticamente, regex de show rule não atravessando `Space`/múltiplos nós — fazer uma checagem rápida e direta (não uma investigação nova completa, isso já foi feito por P859): testar um caso simples de cada um nos dois binários, só para confirmar se **continua** sem sintoma observável, ou se algum documento de teste mais elaborado (não o caso mínimo que P859 usou) revela uma diferença que passou despercebida.

## Item 4 — Notas soltas nunca abertas como achado

Testar rapidamente, nos dois binários, os quatro itens mencionados de passagem em P831/P848 e nunca formalizados: `#text(size:)` como chamada (não `#set`), `box(width:)` em unidade absoluta, a extensão do arquivo de saída do CLI (`-o arquivo.png` gerando PDF mesmo assim, ou o que for o caso exato), e `#set page(height: auto)`. Para cada um: confirmar se a divergência realmente existe hoje (algumas podem ter sido resolvidas incidentalmente por outros passos, como já aconteceu antes no projeto — ex. P805a resolveu um bug que afetava mais do que só `lorem`).

## Relatório

`00_nucleo/diagnosticos/typst-passo-861-relatorio.md` com quatro seções, uma por item acima, cada uma com resposta direta e objetiva:
- Item 1: estado real da migração de `Duration` (feita / em andamento com X de 7 pontos / não iniciada), com evidência (comando + saída, não suposição).
- Item 2: confirmação do estado do achado #41.
- Item 3: tabela dos quatro itens do Grupo 2 de P859, confirmando se continuam sem sintoma ou se algum apareceu.
- Item 4: tabela das quatro notas soltas, confirmando se a divergência existe, já foi resolvida, ou nunca foi de fato um problema.

Este relatório não deve conter nenhuma correção de código — só o levantamento de estado, para decidir os próximos passos depois com informação confiável.
