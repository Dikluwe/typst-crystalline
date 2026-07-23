# Prompt — typst-passo-867: `#set page(height: auto)` rejeitado (achado de P861, item 4)

**Origem**: nota registrada desde P848, confirmada ainda ativa por P861 — `#set page(height: auto)` — vanilla aceita (página com altura infinita/auto-ajustável); cristalino: `error: expected length, float, or int, found auto`
**Estado**: aguardando execução

---

## Achado (medição de P861)

`#set page(height: auto)` seguido de conteúdo — vanilla aceita: a página cresce para acomodar o conteúdo (altura infinita/dinâmica, útil para exportar documentos de página única sem quebra). Cristalino rejeita `auto` como valor de `height:`, só aceitando comprimento numérico.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Reproduzir o caso nos dois binários — testar com conteúdo curto e com conteúdo longo (várias "páginas" de texto) para confirmar que o vanilla de fato produz uma única página crescendo, não múltiplas páginas do tamanho auto-calculado individualmente.
2. Testar `width: auto` também, já que o vanilla provavelmente aceita `auto` nas duas dimensões — confirmar se `width` já funciona no cristalino ou tem a mesma lacuna.
3. Localizar no vanilla como páginas de altura `auto` são tratadas no layout — provavelmente desabilita a paginação automática por completo para esse documento (uma única "página" infinita).
4. Localizar no cristalino onde `#set page(height:)` valida o tipo do valor e confirmar a ausência do caso `auto`.

## Passo 2 — Implementação

Adicionar suporte a `Value::Auto` em `height:` (e `width:`, se o Passo 1.2 confirmar a mesma lacuna), propagando para o mecanismo de layout de página — desabilitando a paginação automática/quebra de página quando a dimensão correspondente é `auto`, permitindo que a página cresça conforme o conteúdo.

## Passo 3 — Validação

1. `#set page(height: auto)` com conteúdo curto e longo, confirmando página única crescendo, batendo com o vanilla (comparar dimensões finais da página gerada).
2. Confirmar que `#set page(height: <valor numérico>)` (o caso comum) continua funcionando sem regressão.
3. Suíte completa, comando + contagem antes/depois, discriminada por crate.

## Relatório

`00_nucleo/diagnosticos/typst-passo-867-relatorio.md` com medição antes, código identificado, diff, medição depois, contagem de testes.
