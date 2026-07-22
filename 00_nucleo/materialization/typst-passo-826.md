# Prompt — typst-passo-826: `pdf::accessibility` — `pdf.artifact(kind:)` rejeitado (achado #14 de P810)

**Origem**: achado #14 da tabela de P810
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> `pdf.artifact(kind:)` rejeitado (gate A11yExtras confirmado nas outras 3 — paridade)

Nota: o achado confirma que as outras 3 variantes de `pdf.artifact` (ou o mecanismo relacionado, atrás do gate `A11yExtras`) já estão em paridade — o problema é específico ao argumento `kind:`.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

1. Compilar `#pdf.artifact(kind: "...")` (conferir no relatório de materialização de P810 os valores exactos de `kind` testados) com os dois binários — confirmar que o vanilla aceita e o cristalino rejeita o argumento `kind:`.
2. Localizar no vanilla (`lab/typst-original/`) a assinatura de `pdf.artifact` e a lista de valores válidos para `kind:` (provavelmente uma enumeração ligada à especificação de acessibilidade PDF/UA — tipos de artefacto como "Pagination", "Layout", "Background", etc.).
3. Localizar `native_pdf_artifact` (ou equivalente) no cristalino, atrás do gate `A11yExtras` já confirmado como correcto para as outras variantes, e identificar por que `kind:` não está implementado ali.
4. Registar os pontos antes de tocar em código.

## Passo 2 — Implementação

Adicionar o argumento `kind:` a `pdf.artifact`, com a enumeração de valores válidos replicada do vanilla, e propagar o valor para a estrutura PDF de saída (marcação de artefacto na árvore de acessibilidade) da mesma forma que as outras 3 variantes já confirmadas em paridade.

## Passo 3 — Validação

1. Recompilar. Repetir o comando do Passo 1, saída literal batendo com o vanilla.
2. Confirmar que as outras 3 variantes continuam em paridade (controlo de regressão).
3. Verificar a estrutura PDF de saída (não só ausência de erro) para confirmar que a marcação de artefacto reflecte o `kind:` correcto.
4. Teste novo cobrindo `kind:` com pelo menos dois valores válidos.
5. Suíte `typst-core` completa (e infra, se a validação tocar export), comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-826-relatorio.md` com: medição antes, código vanilla/cristalino identificado, diff, medição depois (incluindo estrutura PDF), contagem de testes.
