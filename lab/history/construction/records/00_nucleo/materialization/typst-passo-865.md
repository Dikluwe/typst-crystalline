# Prompt — typst-passo-865: `#text(size:)` como chamada rejeitado (achado de P861, item 4)

**Origem**: nota solta de P786 §7, confirmada ainda ativa por P861 — `#text("hello", size: 20pt)` (chamada de função, não `#set`) — vanilla aceita; cristalino: `error: text() argumento nomeado desconhecido: 'size'`
**Estado**: aguardando execução

---

## Achado (medição de P861)

`#text("hello", size: 20pt)` — vanilla aceita, produz texto em 20pt; cristalino rejeita `size:` como argumento nomeado desconhecido.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Reproduzir o caso nos dois binários.
2. Confirmar quais outros argumentos de `#set text(...)` também faltam em `#text(...)` como chamada — testar pelo menos `font:`, `weight:`, `style:`, `fill:` (alguns provavelmente já funcionam, dado que P836 implementou `variations:` na mesma via) para mapear o alcance real da lacuna, não corrigir só `size:` isoladamente se o problema for mais amplo.
3. Localizar no cristalino onde `native_text` (chamada de função) e o `#set text` (regra) validam argumentos — confirmar se são duas listas de argumentos válidos mantidas separadamente (o que explicaria a lacuna: `size:` foi adicionado a uma mas não à outra) ou um mecanismo unificado com uma exceção pontual para `size:`.

## Passo 2 — Implementação

Se as duas listas (chamada vs `#set`) forem mantidas separadamente, unificá-las (ou pelo menos sincronizar) para que qualquer argumento válido em `#set text(...)` também seja válido em `#text(...)` como chamada, exceto onde o vanilla de fato distinguir os dois casos (confirmar se existe alguma distinção real antes de assumir que devem ser idênticos).

## Passo 3 — Validação

1. `size:` (e os outros argumentos identificados no Passo 1.2) funcionando em `#text(...)` como chamada, batendo com o vanilla.
2. Confirmar que `#set text(...)` continua funcionando sem regressão.
3. Suíte completa, comando + contagem antes/depois, discriminada por crate.

## Relatório

`00_nucleo/diagnosticos/typst-passo-865-relatorio.md` com medição antes (incluindo o alcance real da lacuna, não só `size:`), código identificado, diff, medição depois, contagem de testes.
