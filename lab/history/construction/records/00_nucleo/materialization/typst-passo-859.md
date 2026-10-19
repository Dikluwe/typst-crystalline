# Prompt — typst-passo-859: o que a fase `realize` do vanilla garante de verdade — investigação, não decisão de arquitetura

**Origem**: pergunta levantada depois de P857/P858 — a Opção 1 fechou o achado #34 (`measure()`) sem adotar a fase de realização do vanilla, por decisão deliberada de manter a arquitetura própria do cristalino. Mas a fase `realize` do vanilla (mencionada de passagem em P857 §3 — "aplica show rules e agrupa parágrafos/listas/etc.") pode existir por razões tipográficas que não têm nada a ver com `measure()`. Este passo separa as duas coisas: a decisão sobre `measure()` está fechada; a pergunta aqui é se falta alguma garantia de correção que o cristalino não tem hoje, independente daquele achado.
**Estado**: aguardando execução — **isto é pesquisa, sem código, sem proposta de arquitetura nova**. O objetivo é entender e relatar, não decidir se implementar seja o que for.

---

## Por que isto não é reabrir a decisão do measure()

A escolha da Opção 1 para `measure()` continua valendo (P858). Este passo não é sobre "devíamos ter feito a fase 4.2 afinal" — é sobre uma pergunta mais ampla que só apareceu por causa dessa investigação: será que o cristalino, ao não ter uma fase de realização, está sujeito a bugs de tipografia que nenhum achado até agora capturou, porque a triagem sistemática (P785-P848) testou módulo por módulo, não o comportamento de composição entre eles que uma fase de realização normalmente garante?

---

## Passo 1 — Ler a fase `realize` do vanilla por completo, com foco tipográfico

1. Ler `lab/typst-original/crates/typst-realize/` inteiro (não só os trechos que P857 já leu para `measure()`). Para cada coisa que a fase faz, perguntar: que problema tipográfico isso evita se não for feito nesse ponto específico do pipeline?
2. Casos conhecidos de sistemas de tipografia que precisam de uma fase de "resolução" antes do layout (para orientar a leitura, não afirmar que o vanilla faz exatamente isso sem confirmar):
   - Agrupamento de itens de lista consecutivos num único bloco de lista (evitar que cada item vire um parágrafo isolado com espaçamento errado entre eles).
   - Ordem de aplicação de show rules quando várias regras poderiam se aplicar ao mesmo elemento (cascata determinística).
   - Resolução de elementos que só fazem sentido depois de saber o que vem ao redor (ex.: `#pagebreak` implícito por certos elementos, quebra de parágrafo por elementos de bloco intercalados no meio de texto corrido).
   - Fusão/normalização de sequências de texto adjacentes antes do shaping (relevante: P843 já registrou como achado adjacente que o cristalino funde texto de forma diferente do vanilla — `[hello world]` vira um só `Text` no cristalino, três nós no vanilla — vale conferir se isso é sintoma do mesmo buraco).
3. Para cada mecanismo encontrado, confirmar (não supor) se o cristalino já resolve isso de outra forma — em outro ponto do pipeline, com outro nome, ou não resolve.

## Passo 2 — Cruzar com achados já registrados

Revisar a lista de achados já fechados ou pendentes (a fila inteira, desde #1 até #63, mais os achados adjacentes registrados mas não numerados ao longo dos relatórios) procurando por qualquer um que já seja, sem ter sido reconhecido como tal, sintoma de falta de uma fase de composição — não só o caso de fusão de texto do Passo 1.2, mas qualquer outro que, olhando de novo com essa lente, faça mais sentido.

## Passo 3 — Classificar o que foi encontrado

Para cada mecanismo do `realize` identificado no Passo 1, classificar em três grupos:
1. **Já coberto** — o cristalino resolve isso, só que em outro lugar/de outra forma (documentar onde).
2. **Não coberto, mas sem sintoma observado ainda** — o cristalino não tem esse mecanismo, e nenhum achado até agora expôs isso, mas é plausível que exponha em algum documento não testado.
3. **Não coberto e já é sintoma de um achado conhecido** — like a fusão de texto de P843, se a leitura confirmar a ligação.

## Passo 4 — Relatar sem prescrever solução

Este passo termina com um relatório do que existe e por quê — não com uma recomendação de implementar uma fase de realização no cristalino. Se o Passo 3 encontrar itens no grupo 3 (sintoma conhecido) ou um número preocupante no grupo 2 (buracos plausíveis), isso é informação para o dono decidir se abre um passo de arquitetura depois — não decidir isso dentro deste passo.

## Relatório

`00_nucleo/diagnosticos/typst-passo-859-relatorio.md` com: a lista de mecanismos da fase `realize` do vanilla e a razão tipográfica de cada um (Passo 1), a checagem cruzada com achados já registrados (Passo 2), a classificação nos três grupos (Passo 3), e nenhuma proposta de solução — só o levantamento, para decisão futura do dono.
