# Prompt — typst-passo-829: achados adjacentes registrados em P814 e P815 (métodos de content, despacho math, campos extra, decisão de escopo do `#eval`)

**Origem**: achados medidos e registrados (não corrigidos) dentro de P814 e P815, fora do âmbito original desses passos
**Estado**: aguardando execução — **executar só depois de P828** (consolidação), para não medir sobre uma árvore parcial

---

## Contexto

P814 e P815 mediram e documentaram, de propósito, coisas que não corrigiram porque estavam fora do escopo dos achados #1 e #2 de P810. São quatro pontos distintos, cada um com sonda própria já parcialmente feita nos relatórios originais — este prompt não repete a sonda do zero, usa o que já foi medido como ponto de partida e completa o que faltar.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Item A — decisão pendente: `#eval` enxerga o escopo do chamador (P814)

**O que já foi medido (P814, `t12`):** `#let y = 10` + `#eval("y + 1")` — vanilla erra `unknown variable: y` (o `eval_string` do vanilla cria um `Scopes` fresco, só stdlib + `scope:`); cristalino devolve `11` (vê o escopo de quem chamou). O relatório de P814 registra que isso é uma decisão consciente já presente no L0 (`stdlib/eval.md` §4), com um teste (`eval_ve_escopo_actual`) que fixa esse comportamento — e que mudar isso é decisão do dono, não do executor.

### O que este item pede
Não é sonda nova — é levar a decisão ao dono antes de fechar de vez. Apresentar:
1. O comportamento vanilla medido (erro) vs o comportamento cristalino actual (funciona, vê o escopo externo).
2. O trade-off: manter como está é uma divergência de comportamento de linguagem (não só de mensagem); mudar para bater com o vanilla pode quebrar documentos `.typ` de teste ou de uso real que dependam do comportamento actual (buscar no repositório, se houver, qualquer uso de `#eval` que dependa de ver variáveis externas).
3. Registar a decisão formalmente (mesmo padrão de P807/P812-C/P825-C): se o dono decidir manter, formalizar como divergência consciente no L0 com a medição anexada (já parcialmente feito); se decidir corrigir, tratar como um passo de implementação normal (sonda já feita, só falta o `Scopes` fresco).

## Item B — métodos de `content` ausentes (P815, `m14`)

**O que já foi medido:** `#strong[x].func()` — vanilla exit 0, devolve `strong` (é um método real); cristalino, depois de P815, erra com a forma vanilla (`element strong has no method`) em vez de aceitar. O relatório de P815 lista `.func()`, `.has()`, `.at()`, `.fields()`, `.location()` como métodos de `content` que existem no vanilla e não no cristalino.

### Sonda (completar)
1. Para cada um dos cinco métodos, testar em pelo menos dois tipos de elemento de content diferentes (ex.: `strong`, `heading`) com os dois binários — confirmar assinatura e retorno exacto no vanilla (`crates/typst-library/src/foundations/content.rs` ou equivalente).
2. Confirmar se há outros métodos de `content` no vanilla além desses cinco que também estejam ausentes (a lista do relatório de P815 pode não ser exaustiva — ela só listou o que apareceu nos casos de teste do achado #2).

### Implementação
Implementar os métodos confirmados, replicando assinatura e retorno do vanilla.

### Validação
Testes cobrindo cada método em pelo menos dois tipos de elemento. Suíte completa, comando + contagem antes/depois.

## Item C — despacho de erro diferente dentro de modo math (P815, `m17`)

**O que já foi medido:** `$#d.x()$` (chamada de método dict-key dentro de modo math via `#`) — vanilla produz o mesmo erro de dict-key-call + hints que P815 implementou para o caminho normal; cristalino segue um caminho de despacho separado (`engine/eval/math.rs`) com mensagem própria (`chamada em modo math espera função, recebeu int`), sem passar pela lógica nova de P815.

### Sonda
1. Confirmar exactamente onde, no cristalino, o despacho de chamada dentro de `#...` em modo math diverge do despacho de chamada fora de modo math — são duas implementações separadas de "avaliar uma chamada de método", ou uma delega na outra em algum ponto?
2. Confirmar se o vanilla trata os dois caminhos (dentro/fora de math) com a mesma rotina de avaliação de chamada, ou se também tem uma separação (mesmo que produza o mesmo resultado observável).

### Implementação
Se o cristalino tiver duas implementações separadas onde o vanilla tem uma só (ou duas que convergem no mesmo resultado), fazer o despacho de math reutilizar `field_callee_error` (a função criada em P815) em vez de ter mensagem própria.

### Validação
`$#d.x()$` e variantes produzindo o mesmo erro + hints que o caminho não-math. Suíte completa, comando + contagem antes/depois.

## Item D — campos que existem no cristalino mas não no vanilla (P815, `m12`)

**O que já foi medido:** `arguments.positional`/`.named` (de P504) e `array.len`/`.first`/`.last` como campos (de P493a) existem como *campos* no cristalino; no vanilla são *métodos*, não campos — isso muda o comportamento em casos de borda como `m12` (`args.positional()` — no vanilla, erro de "no method"; no cristalino, o campo existe e é avaliado como valor, então a chamada seguinte falha de um jeito diferente).

### Sonda
1. Listar, para `arguments` e `array`, todos os pontos onde o cristalino expõe algo como campo que o vanilla expõe como método (ou vice-versa). Não assumir que são só os dois casos citados — conferir a definição completa dos dois tipos nos dois binários.
2. Para cada divergência encontrada, testar um caso que dependa da distinção campo-vs-método (como `m12`) e confirmar a mensagem/comportamento exacto dos dois binários.

### Implementação
Depende do que a sonda encontrar — pode ser mudança de "campo" para "método" nos pontos identificados, o que é uma alteração estrutural maior do que os outros itens deste prompt. Se o esforço for desproporcional ao ganho medido, registar como scope-out formal (mesmo padrão de decisão explícita), não implementar parcialmente sem decisão.

---

## Relatório final

Produzir `00_nucleo/diagnosticos/typst-passo-829-relatorio.md` cobrindo os quatro itens (A-D) separadamente:
- Item A: a decisão do dono, registrada explicitamente (mantida ou revertida), não uma correção de código por padrão.
- Itens B, C, D: sonda completa, código identificado, diff (ou scope-out formal para D se for o caso), validação, testes novos nomeados `p829b_...`, `p829c_...`, `p829d_...`.
- Contagem de testes final consolidada (`typst-core` e `typst-infra`).
