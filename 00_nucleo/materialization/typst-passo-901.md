# Passo 901 — barra do radical (`sqrt`/`root`) mal posicionada

**Precede este passo**: `typst-passo-894-relatorio.md`, achado sobre posicionamento do radical.
Reler o relatório para os casos exactos e qualquer medição já feita antes de começar.

**Pré-condição de árvore**: `git status`. Confirmar estado dos passos anteriores da fila.

---

## Sintoma (confirmar contra P894 antes de assumir os detalhes abaixo)

A barra horizontal do radical (`sqrt(x)`, `root(3, x)`) não está posicionada corretamente em
relação ao conteúdo sob ela — confirmar exactamente o que diverge (altura da barra, extensão
horizontal, posição vertical do símbolo `√` em relação à barra, ou posição do índice em `root`).
Exemplos visíveis nos PDFs de seções 1/13/14/17/18 (`√𝑎²+𝑏²=𝑐`, `√3𝑥=𝑦`, expressões aninhadas com
radical dentro de radical) já mostrados em mensagens anteriores desta conversa — usar como
referência dos casos a testar, não como diagnóstico já feito.

## Fase A — diagnóstico

1. Localizar o código de layout de radical (`01_core/src/engine/math/layout/` — procurar por
   `radical`/`sqrt`/`root`, nome exacto do módulo a confirmar).
2. Confirmar a fórmula usada para a extensão/posição da barra e comparar com a leitura do
   `radical_vertical_gap`/`radical_rule_thickness` da tabela MATH (já lidos em P893, mesmo que P893
   ainda esteja parado — os *valores* já foram medidos nesse passo, reaproveitar essa medição, não
   remedir do zero). Confirmar se este passo depende da conclusão de P893 (`math_constants` real em
   vez de fallback) para produzir resultado correto, ou se é um bug de fórmula independente disso.
3. **Se depender de P893**: registar essa dependência explicitamente e decidir se faz sentido
   avançar mesmo assim com os valores de fallback (sabendo que vai ficar sutilmente errado até P893
   fechar) ou esperar. Não implementar uma correção que sabidamente vai ficar errada sem essa nota.
4. Comparar com o cálculo do vanilla para a mesma fórmula (`lab/typst-original/`) — confirmar a
   fórmula exacta de extensão de barra e altura de radical antes de replicar.
5. Isolar um caso mínimo (`$ sqrt(x) $` sozinho) e medir exactamente (via `mutool trace` ou
   equivalente, mesmo método usado em passos anteriores) a posição/dimensão da barra nos dois
   binários, para ter um alvo numérico concreto para o teste.

## Fase B — Implementação (protocolo de dois agentes de P898, já que é cálculo geométrico)

1. Agente A escreve teste(s) com o alvo numérico confirmado na Fase A ponto 5, confirma vermelho.
2. Agente B implementa a correcção sem ver a implementação de referência além do necessário.
3. Revisão do orquestrador antes de fechar — mesmo padrão de P898 (não aceitar "está correto" só
   pelo relatório do Agente B; testar pelo menos um caso adicional não coberto pelos testes do
   Agente A, por exemplo `root(3, x)` se os testes de A só cobrirem `sqrt`).
4. Suíte completa verde, discriminada por crate.
5. Recompilar as secções relevantes (1, 13, 14, 17, 18) e confirmar visualmente/geometricamente.
6. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`.

## Resultado esperado

- Relatório com: dependência ou não de P893 registada explicitamente, fórmula corrigida, alvo
  numérico confirmado antes/depois, protocolo de dois agentes documentado, achado extra encontrado
  pela revisão do orquestrador (se houver, mesmo padrão de P898), benchmark completo.
