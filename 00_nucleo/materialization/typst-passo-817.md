# Prompt — typst-passo-817: `foundations::calc` — trigonometria inversa, `quo`, `pow`, `decimal` (achado #4 de P810, prioridade alta)

**Origem**: achado #4 da tabela de P810, marcado prioridade alta no handoff (`handoff-novo-chat-p810.md`)
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> asin/acos/atan/atan2 float vs `angle`; quo trunc vs floored; pow int-neg; decimal ausente; log10/deg/rad extra; precisão erf/log/exp

São vários sub-pontos dentro do mesmo módulo. Tratar cada um com sonda própria.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Sub-achado A — `asin`/`acos`/`atan`/`atan2` devolvem `float` em vez de `angle`

### Sonda
`#calc.asin(0.5)`, `#calc.atan2(1, 1)` nos dois binários — comparar o tipo do valor devolvido (`#repr(calc.asin(0.5))` deve mostrar unidade de ângulo no vanilla, ex. `28.9598°`, e um número puro no cristalino).

### Implementação
Alterar o tipo de retorno destas quatro funções para `Angle`, replicando a conversão de radianos para o tipo `angle` do vanilla.

---

## Sub-achado B — `quo` trunca em vez de arredondar para baixo (floor)

### Sonda
`#calc.quo(-7, 2)` — vanilla usa divisão euclidiana/floored (`-4`), confirmar o que o cristalino devolve (`-3` se for truncamento em direcção a zero).

### Implementação
Corrigir `quo` para floored division, igual ao vanilla.

---

## Sub-achado C — `pow` com expoente inteiro negativo

### Sonda
`#calc.pow(2, -1)` — confirmar comportamento do vanilla (deve devolver `float`, `0.5`) vs cristalino (pode estar a rejeitar ou a devolver `0`/erro).

### Implementação
Corrigir `pow` para aceitar expoente negativo e devolver `float` quando a base é inteira e o expoente negativo.

---

## Sub-achado D — `decimal` ausente

### Sonda
Confirmar se `calc` do vanilla expõe funções para o tipo `decimal` (ex.: `calc.round`/`calc.abs` sobre `decimal`, ou construção via `decimal("1.5")`) que o cristalino não suporta. Testar os casos que existirem.

### Implementação
Depende do que a sonda encontrar — se for suporte a decimal em funções já existentes de `calc`, estender essas funções; se for ausência total do tipo `decimal`, isso é maior do que este achado sugere — registar como achado à parte antes de decidir implementar aqui (o tipo `decimal` pode já ter sido tratado ou scope-out em passo anterior — verificar handoffs antigos antes de assumir que está totalmente ausente).

---

## Sub-achado E — `log10`/`deg`/`rad` a mais; precisão de `erf`/`log`/`exp`

### Sonda
Confirmar se `log10`, `deg`, `rad` existem no cristalino mas não no vanilla (ou vice-versa — o achado diz "extra", sugerindo que o cristalino tem funções que o vanilla não tem, o que não é necessariamente um problema, mas vale confirmar se não é o inverso por erro de leitura do achado original). Comparar `erf`, `log`, `exp` com valores de referência (várias casas decimais) nos dois binários para medir a divergência de precisão.

### Implementação
Se `log10`/`deg`/`rad` forem mesmo extras (funções que o vanilla não expõe), não é necessariamente uma correcção obrigatória — registar a decisão (manter como extensão, ou remover por fidelidade estrita) e não implementar sem essa decisão. Para precisão de `erf`/`log`/`exp`: ajustar a implementação (algoritmo ou biblioteca usada) para bater com a precisão do vanilla dentro de uma tolerância a definir com base na medição.

---

## Validação (comum aos cinco sub-achados)

1. Recompilar. Repetir todos os comandos de sonda, valores batendo com o vanilla (ou decisão registada, para os casos que não forem correcção directa).
2. Testes novos por sub-achado, nomeados `p817a_...` a `p817e_...`.
3. Suíte `typst-core` completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-817-relatorio.md`, um bloco por sub-achado (A-E), cada um com medição antes, código identificado, diff (ou decisão registada), medição depois. Contagem de testes final consolidada.
