# Prompt — typst-passo-854: a abordagem 4.2 (fase de realização) é de fato a mais inteligente para `measure()` (DEBT-69)?

**Origem**: revisão pedida sobre a recomendação de P849 — antes de prototipar a abordagem 4.2, confirmar que ela é realmente a melhor escolha, não só a mais alinhada com o vanilla na superfície
**Estado**: aguardando execução — **isto é pesquisa e análise, sem código, sem protótipo**. Não implementar nada neste passo.

---

## O que este passo não é

Não é o protótipo (isso fica para depois, se este passo confirmar que vale a pena). Não é reabrir a decisão do dono de investigar a Opção 2 em vez da Opção 1 — essa parte já está decidida. O que está em aberto é: **dentro** da Opção 2, a abordagem 4.2 era a recomendação de P849 entre quatro (4.1 a 4.4) que o próprio P849 inventou e comparou. Vale checar se essa comparação foi honesta e completa, ou se ficou curta.

---

## Passo 1 — Questionar a própria comparação de P849

1. A tabela comparativa de P849 (§5) pontuou 4.1 a 4.4 mais a Opção 1 em cinco critérios, sem metodologia explícita de como cada nota foi atribuída — reler essa tabela com espírito crítico: os números ali (ex.: "esforço 1-2 passos" para 4.2, "3+ passos" para 4.3) são estimativas de quem também estava recomendando a 4.2. Existe viés de confirmação possível. Não aceitar a tabela como neutra sem reexaminar pelo menos os dois ou três pontos mais decisivos dela.
2. Perguntar diretamente: as quatro abordagens de P849 esgotam o espaço de soluções, ou existem variantes que ele não considerou? Em particular:
   - Uma versão **limitada** da abordagem 4.1 (mover só a resolução de `measure()` para dentro do layout, sem mover a expansão inteira de `ContextBlock`) foi descartada junto com a 4.1 inteira — mas os dois problemas (measure com métricas erradas, e expansão de contexto) podem não precisar da mesma solução. Vale reconsiderar separadamente.
   - Um híbrido entre 4.2 e a Opção 1 (fase de realização só para os casos que usam `measure()`, deixando o resto do fluxo de `#context` como está) reduziria o escopo da mudança sem perder a paridade no caso que importa.

## Passo 2 — Estudar como o vanilla resolve isto de verdade, com mais profundidade

P849 já leu `measure.rs:47-105` do vanilla, mas descreveu o mecanismo de "realização" (realize) só em linhas gerais. Aprofundar:
1. Ler o código da fase de "realization" do vanilla por completo (não só o trecho de `measure()`), para entender o que mais essa fase faz além de resolver contexto — se ela também lida com outras coisas que o cristalino trata de forma diferente, a mudança pode ter efeitos colaterais bons (resolver outros achados de uma vez) ou o escopo pode ser maior do que P849 estimou.
2. Confirmar se "realization" no vanilla é uma fase de fato separada e nomeada assim no código-fonte, ou se é uma descrição informal de P849 para algo mais distribuído — isso muda o quão direto é "portar o padrão do vanilla".

## Passo 3 — Buscar precedente fora do próprio código do vanilla

1. Verificar se algum ADR ou ficheiro de arquitetura já existente no projeto (`00_nucleo/adr/`) discute a fronteira L1/L3 e por que ela é rígida do jeito que é hoje — entender se a rigidez é uma decisão deliberada com razão específica (ex.: pureza para testes, paralelismo, cache/comemo) ou se é só como o código foi crescendo. Isso muda o peso de "quebrar a fronteira" como argumento contra a Opção 1.
2. Se existir alguma outra reimplementação de Typst (ou de um compilador com arquitetura de fases parecida) publicamente documentada, uma busca rápida sobre como outros projetos resolvem o problema de "funções que precisam de dados de uma fase posterior" pode trazer um terceiro caminho que nem P849 nem este passo tinham considerado. Não é obrigatório encontrar algo, mas vale tentar antes de fechar a pesquisa.

## Passo 4 — Veredito

Responder diretamente: a recomendação de P849 (abordagem 4.2) se sustenta depois desta revisão, ou existe uma abordagem melhor (mais barata, menos arriscada, ou que resolve mais achados de uma vez) que não tinha sido considerada? Se a 4.2 se sustentar, isso valida o plano de prototipagem que já estava desenhado (P854 anterior, agora renumerado como próximo passo se este confirmar). Se não, escrever a nova recomendação com a mesma honestidade do Passo 1 — não forçar a conclusão para bater com o que já estava planejado.

## Relatório

`00_nucleo/diagnosticos/typst-passo-854-relatorio.md` com: a reavaliação crítica da tabela de P849, o que foi encontrado na leitura mais profunda da fase de realização do vanilla, o resultado da busca por precedente externo (mesmo que não tenha achado nada), e o veredito final — mantendo ou revisando a recomendação. Atualizar DEBT-69 com o resultado desta análise antes de qualquer prototipagem.
