# Prompt — typst-passo-822: `layout::grid::resolve` — mensagens divergentes e footer fora do fim aceite (achado #9 de P810)

**Origem**: achado #9 da tabela de P810
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> header/footer não repetem (débito reconfirmado); mensagens divergentes; footer fora do fim aceite

Nota: "header/footer não repetem entre páginas" é o débito já conhecido e reafirmado várias vezes (P772i, P789, e de novo aqui) — **não é para corrigir neste passo**, continua registado como débito grande. Este passo cobre só os dois pontos novos: mensagens divergentes e footer fora do fim aceite.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

1. "Footer fora do fim aceite": construir um `grid`/`table` com `footer` declarado numa posição que não é a última linha — confirmar que o vanilla rejeita (erro) e o cristalino aceita silenciosamente. Reproduzir o caso exacto do relatório de materialização de P810 antes de assumir a sintaxe exacta.
2. "Mensagens divergentes": identificar quais mensagens de erro de `grid.resolve` (conflitos de célula, argumentos inválidos, etc.) diferem em texto entre os dois binários — comparar pelo menos dois ou três casos de erro já cobertos por testes existentes de P772g/P772i/P789, para ver se o texto mudou de forma não intencional ou se é uma divergência nunca corrigida.
3. Localizar no vanilla (`lab/typst-original/`) a validação de posição de `footer` e o texto das mensagens de erro relevantes.
4. Localizar o cristalino e identificar os pontos de divergência.
5. Registar os pontos antes de tocar em código.

## Passo 2 — Implementação

Adicionar a validação de posição de `footer` (erro se não for a última linha, replicando o vanilla). Corrigir o texto das mensagens de erro divergentes identificadas no Passo 1.2.

## Passo 3 — Validação

1. Recompilar. Repetir os comandos do Passo 1, saída literal batendo com o vanilla.
2. Confirmar que `header`/`footer` na posição correcta continuam a funcionar sem regressão (controlo).
3. Testes novos cobrindo footer fora de posição e as mensagens corrigidas.
4. Suíte `typst-core` completa, comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-822-relatorio.md` com: medição antes, código vanilla/cristalino identificado, diff, medição depois, contagem de testes. Confirmar explicitamente que o débito de repeat-across-páginas continua registado, sem tentativa de correcção neste passo.
