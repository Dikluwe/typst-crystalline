# Prompt — typst-passo-840: `text::font::exceptions` — tabela de exceções de família e peso ausente (#29, #30)

**Origem**: achados #29 e #30 de P831 (lote 5)
**Estado**: aguardando execução

---

## Achado #29 (E1) — exceções de família ausentes

Com a fonte **New Computer Modern embutida nos dois binários**: `#set text(font: "New Computer Modern")` — cristalino `warning: unknown font family: new computer modern` (usa o ID1 cru `NewComputerModern10`); vanilla compila (`NewCM10-Regular`), porque tem uma tabela de exceções que mapeia o nome documentado da linguagem para o ID real da fonte. O nome documentado na referência oficial do Typst falha no cristalino. Vanilla: `exceptions.rs:5-7,46-342` (tabela extensa).

### Sonda
Confirmar o caso exato com a fonte embutida nos dois binários. Olhar a tabela `exceptions.rs` do vanilla — é grande (linhas 46-342), cobre muitas fontes conhecidas do ecossistema Typst.

### Implementação
Portar a tabela de exceções do vanilla — não precisa ser 1:1 completa no primeiro momento, mas o caso medido (New Computer Modern) precisa funcionar, já que é a fonte que o próprio Typst embute e documenta. Registrar no L0 quais entradas da tabela do vanilla foram portadas e quais ficaram de fora (scope-out parcial, expandido sob demanda — mesmo padrão usado para outras tabelas grandes no projeto).

## Achado #30 (E2) — exceções de peso ausentes

Fonte com `usWeightClass` errado na tabela OS/2 (ex.: uma face Bold marcada como peso 400/regular) — vanilla tem uma tabela de exceções que corrige isso e permite `#set text(weight: "bold")` selecionar a face certa; cristalino não tem essa correção e fica preso na regular. Vanilla: `info.rs:60-61,105-112`.

### Sonda
Confirmar com uma fonte de teste (sintética, se necessário) que tenha esse desalinhamento de metadado.

### Implementação
Portar a tabela de exceções de peso do vanilla (provavelmente menor que a de família), com o mesmo critério de escopo parcial se for extensa.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino) para cada achado. Contagem de testes de `typst-core` a bater com os testes novos.

## Validação
1. Recompilar. Repetir os dois casos, batendo com o vanilla.
2. Confirmar que fontes sem exceções continuam resolvendo normalmente (sem regressão).
3. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-840-relatorio.md`, uma seção por achado (#29, #30), com o alcance exato da tabela portada documentado.
