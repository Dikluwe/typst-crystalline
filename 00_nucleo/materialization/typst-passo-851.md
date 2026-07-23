# Prompt — typst-passo-851: `layout::axes` — `stack(dir:)` aceita string em vez de `direction` (achado #61 de P848)

**Origem**: achado #61 da tabela de P848 (lote 6)
**Estado**: aguardando execução

---

## Achado (medição de P848)

`#stack(dir: ltr, ...)` — cristalino: `error: stack(dir:) deve ser string, recebeu direction`; vanilla: compila. Testado o inverso: `stack(dir: "ltr", ...)` — cristalino compila, vanilla `error: expected direction, found string`. Conclusão de P848: o cristalino trata `dir:` como `string`, o vanilla como o tipo `direction` (`ltr`/`rtl`/`ttb`/`btt`, o mesmo tipo já usado em outros pontos do projeto, ex. `text(dir:)`, achado #39 de P810/P842 já mediu erros de operador sobre esse tipo).

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Reproduzir os dois casos do achado (`dir: ltr` e `dir: "ltr"`) nos dois binários, confirmar as mensagens exatas.
2. Testar os quatro valores do tipo `direction` (`ltr`, `rtl`, `ttb`, `btt`) com `stack(dir:)` no vanilla, confirmar que todos são aceitos como identificador (não string).
3. Localizar no cristalino (`01_core/src/engine/stdlib/layout.rs` ou onde `native_stack` estiver) o ponto que valida `dir:` como string, e confirmar se o tipo `direction` já existe no projeto (é usado em outro lugar — `text(dir:)`, possivelmente `grid`) e pode ser reaproveitado, ou se precisa ser criado.
4. Localizar no vanilla a assinatura de `stack` (`lab/typst-original/`) e o tipo `Dir` correspondente.

## Passo 2 — Implementação

Trocar a validação de `dir:` em `native_stack` para aceitar o tipo `direction` (reaproveitando o tipo já existente no projeto, se houver — não duplicar). Se `direction` como tipo de primeira classe não existir ainda de forma reutilizável, confirmar isso antes de decidir criar um novo, para não duplicar algo que já existe sob outro nome.

## Passo 3 — Validação

1. Recompilar. Os quatro valores de `direction` funcionando em `stack(dir:)`, batendo com o vanilla.
2. Confirmar que `stack(dir: "ltr")` (string) agora rejeita, como o vanilla.
3. Teste de regressão: qualquer uso existente de `stack` sem `dir:` explícito continua funcionando (valor default).
4. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-851-relatorio.md` com medição antes, código identificado, diff, medição depois, contagem de testes.
