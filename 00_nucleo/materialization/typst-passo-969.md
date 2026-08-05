# Passo 969 — módulo oráculo: porta literal das fórmulas de posição do vanilla, para testes de geometria

**Precede este passo**: pergunta do dono depois de ver achados novos (9.1/9.2/9.3) que continuam a
aparecer, cada um exigindo leitura manual do código-fonte do vanilla e medição manual (`mutool
trace`) — o mesmo processo repetido em praticamente todos os 84 passos da frente P885-968. Proposta:
um módulo de referência com as fórmulas de posição do vanilla portadas literalmente, usado como
oráculo em teste, não como parte do motor de layout de produção.

**Distinção importante, já esclarecida com o dono**: isto não é sobre o exportador (`Tm`/`Td`/`Tj`,
já resolvido por P956) — é sobre o **cálculo de posição**, que acontece antes do exportador, no
motor de layout matemático.

**Pré-condição de árvore**: `git status`. Confirmar P968 presente.

---

## Fase A — desenhar o oráculo (gate obrigatório — infraestrutura nova, escopo a decidir com cuidado)

1. Confirmar o escopo: um oráculo **completo** (reimplementar toda a geometria matemática do
   vanilla em paralelo) seria enorme e duplicaria o próprio motor de layout — provavelmente não é
   a intenção. Desenhar um escopo mais contido: funções puras, isoladas, que recebem os mesmos
   parâmetros de entrada (métricas de fonte, dimensões de conteúdo) e devolvem a posição esperada
   segundo a fórmula literal do vanilla — uma função por fórmula já identificada nesta frente
   (índice de raiz, shift de limite de operador, ancoragem de grelha, etc.), não um motor de layout
   paralelo completo.
2. Confirmar onde este módulo vive — candidato: `01_core/src/testing/` ou um crate de
   desenvolvimento separado, não misturado com o código de produção (`01_core/src/engine/`) — para
   deixar claro que é ferramenta de verificação, não caminho de execução real.
3. Confirmar o formato de uso: cada teste de geometria futuro chama a função do oráculo com os
   mesmos parâmetros que o código de produção usa, compara o resultado — reduzindo a necessidade de
   read-and-derive manual da fórmula do vanilla a cada novo achado (a leitura já feita fica
   registada em código, reutilizável).
4. Confirmar como popular o oráculo inicialmente — migrar as fórmulas já lidas e confirmadas nesta
   frente (P901, 905, 906, 912-921, 944-968) para dentro do módulo, uma a uma, com o `file:line`
   do vanilla já citado em cada relatório como referência — isto é trabalho de consolidação, não
   nova leitura de código.
5. Editar L0s, sincronizar hashes, **parar para confirmação do dono antes da Fase B** — infraestrutura
   nova de escopo ainda incerto, per o critério de `ADR-0127`.

## Fase B — Implementação (protocolo de dois agentes se o escopo confirmado for grande; TDD directo
se for pequeno o suficiente)

1. Migrar um primeiro conjunto de fórmulas já confirmadas (candidatos: shift de limite de operador
   grande de P959/963, ancoragem de grelha de P945/952) para o oráculo, com testes que comparam o
   resultado do motor de produção contra o oráculo.
2. Confirmar que os testes existentes dessas fórmulas continuam a passar, agora referenciando o
   oráculo em vez de valores hardcoded soltos no teste.
3. Suíte completa verde, discriminada por crate.

## Fase C — validar a utilidade prática

Usar o oráculo para investigar pelo menos um dos três achados novos (9.1/9.2/9.3, ver passos
próprios) — confirmar se de facto acelera a investigação comparado ao processo manual anterior, ou
se o ganho é menor do que esperado (por exemplo, se cada fórmula nova ainda precisar de leitura
extensa do vanilla antes de poder ser portada para o oráculo, o ganho é só de organização, não de
velocidade).

## Resultado esperado

- Módulo oráculo com escopo definido, não um motor de layout paralelo.
- Conjunto inicial de fórmulas já confirmadas nesta frente migradas para dentro dele.
- Avaliação honesta de quanto isto acelera investigações futuras, não assumida sem teste.
