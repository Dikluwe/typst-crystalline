# Passo 904 — três achados pequenos registados em P897/898, agrupados

**Precede este passo**: `typst-passo-897-relatorio.md` e `typst-passo-898-relatorio.md`, secções
"Fora de âmbito". Cada achado é pequeno e potencialmente independente dos outros dois — tratados
juntos por conveniência de programação, não porque partilhem causa. Se a Fase A de qualquer um
revelar mais complexidade do que o esperado, destacar para passo próprio em vez de forçar caber
aqui.

**Pré-condição de árvore**: `git status`.

---

## Item 1 — unidades `fr` em linhas de grid sob `height: auto` (achado do Agente A, P898)

`grid.rs:460` — distribuição de `fr` entre linhas de grid sob `height: auto` diverge do vanilla,
onde `fr` numa página `auto` degenera a 0 (não há espaço "restante" para distribuir quando a altura
é indefinida).

### Fase A
1. Confirmar o comportamento actual do cristalino (o que `fr` produz hoje sob `height: auto` —
   infinito? erro? valor arbitrário?).
2. Confirmar a regra exacta do vanilla (`lab/typst-original/`) — "degenera a 0" é a descrição do
   achado original, confirmar se é exactamente isso ou uma aproximação.

### Fase B (TDD directo, mapeamento de caso especial, não geometria nova)
1. Teste que falhe primeiro: grid com linha `fr` sob `height: auto` produz altura de linha 0 (ou o
   valor correcto confirmado na Fase A).
2. Implementar.
3. Suíte verde, discriminada por crate.

---

## Item 2 — `Content::Place` aninhado em sub-frame usa `origin_y = 0.0` local (achado do Agente B, P898)

A correção diferida de P897/898 resolve contra a altura/largura final da página **raiz** — quando
`Content::Place` está aninhado dentro de um sub-frame (por exemplo, dentro de um `#box`/`#block`
com o próprio sistema de coordenadas), `origin_y` local não corresponde à posição na página raiz, e
a correção pode ficar errada. Limitação já existente no eixo X desde P897, não expandida em P898.

### Fase A
1. Confirmar um caso mínimo que reproduza isto (`Content::Place` dentro de `#box` com `height:
   auto` na página) e medir o sintoma exacto — a correção não é aplicada, ou é aplicada com valor
   errado?
2. Confirmar se corrigir isto exige propagar a posição do sub-frame na hierarquia até à raiz (mudança
   de assinatura/dados adicional) ou se há um atalho mais barato (por exemplo, aplicar a correção só
   quando não está aninhado, e documentar a limitação explicitamente para o caso aninhado, em vez de
   resolver os dois).

### Fase B
1. Se a Fase A encontrar um atalho barato: implementar com TDD, teste do caso mínimo.
2. Se exigir mudança estrutural maior: **não implementar aqui** — registar como achado maior,
   candidato a passo próprio dedicado, e documentar a limitação explicitamente no código (comentário
   + nota no L0) em vez de deixar implícita.

---

## Item 3 — `measure_content` devolve `content_w = 0.0` para `Content::Place`/texto simples (P772j, confirmado ainda presente em P898)

`helpers.rs::measure_content` só tem braços para `Content::Shape`/`Content::Sequence` — para
`Content::Place` com corpo de texto simples, devolve largura 0. P772j corrigiu isto para
`Content::Align`, deixou `Content::Place`/`Content::Transform` fora de âmbito.

### Fase A
1. Reler `typst-passo-772j.md` (ou o L0 correspondente) para entender por que `Place`/`Transform`
   ficaram de fora nesse passo — pode haver uma razão (complexidade, ambiguidade de medição) que
   ainda se aplica, não presumir que foi só falta de tempo.
2. Confirmar se a correção de `Content::Align` em P772j é directamente extensível para
   `Content::Place`/`Content::Transform`, ou se cada um precisa de tratamento próprio (`Transform`
   em particular pode ter medição não-trivial se envolver rotação/escala).

### Fase B (TDD directo, extensão de função já existente)
1. Teste que falhe primeiro: `measure_content` devolve largura correcta para `Content::Place` com
   texto simples.
2. Implementar para `Content::Place`. Se `Content::Transform` também for simples de estender,
   incluir; se não, registar como novo achado separado, não forçar aqui.
3. Suíte verde, discriminada por crate.
4. Confirmar visualmente com o caso de `#place(bottom + right, dy: 1cm)[texto]` já usado como
   confirmação em P898 (que mostrou o texto fora da página) — confirmar que agora fica dentro.

---

## Fase C — Regressão (uma vez, para os três itens implementados)

Benchmark completo, 7 cenários, `--warmup 5 -m 20`.

## Resultado esperado

- Relatório por item: veredicto da Fase A, implementado ou destacado para passo próprio, testes,
  confirmação visual onde aplicável.
- Benchmark completo no fim, cobrindo qualquer combinação dos itens que tenha sido implementada.
