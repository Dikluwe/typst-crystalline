# Passo 1099 — Relatório: Investigação de Matemática Aninhada Dentro de Funções de Layout

## 1. Contexto e Diagnóstico

A investigação do Passo 1099 analisou se expressões matemáticas delimitadas por `$...$` perdem o processamento tipográfico quando aninhadas como argumentos de funções de layout (como `text()`, `box()`, `align()`, `pad()`).

---

## 2. Testes de Isolamento e Resultados (§3, §4, §5)

### 2.1. Teste de Parsing e Avaliação Tipográfica (`text()`)
- **Expressão Testada**: `$ a + #text(size: 8pt)[$b^2$] + c $`
- **Glifos Emitidos no Crystalline**:
  - `𝑎` (11.0 pt, $X = 28.3465$, $Y = 29.2595$)
  - `+` (11.0 pt, $X = 36.6075$, $Y = 29.2595$)
  - `𝑏` (8.0 pt, itálico matemático `U+1D44F`, $X = 47.6123$, $Y = 29.2595$)
  - `2` (5.6 pt, sobrescrito $0.7 \times 8\text{ pt}$, $X = 51.1564$, $Y = 32.1555$)
  - `+`, `𝑐` (11.0 pt)
- **Veredicto**: O processamento matemático (variável em itálico, redução de tamanho para $0.7\times$, posicionamento vertical de sobrescrito) é **preservado com paridade absoluta**. Não há perda de contexto no parser.

### 2.2. Teste de Caixa com Efeito Visual (`box()`)
- **Expressão Testada**: `$ a + #box(stroke: 1pt, inset: 2pt)[$x + y$] + b $`
- **Glifos e Traços**:
  - Glifos internos `𝑥` (`U+1D465`) e `𝑦` (`U+1D466`) reconhecidos como matemática.
  - O traço de borda retangular (`stroke: 1pt`) é emitido no PDF em ambos os compiladores.

### 2.3. Teste com `pad()` e `align()`
- **`#pad(x: 5pt)[$x^2$]`**: O conteúdo interno preserva o sobrescrito $x^2$.
- **`#align(center)[$x$]`**: O conteúdo interno preserva a variável matemática $x$.

---

## 3. Identificação das Causas e Conexão com `attach()` (§1)

1. **Parser vs Layout**:
   - O parser e o avaliador avaliam `[...]` como um bloco de conteúdo que, ao encontrar `$...$`, entra corretamente em modo matemático via `Expr::Equation`.
2. **Divergência Geométrica Identificada**:
   - A discrepância no posicionamento dos elementos subsequentes (ex.: $+ b$) dentro da linha matemática decorre do cálculo do avanço horizontal (`extent_x`) do sub-frame de container externo em `layout_external` (onde o padding lateral ou trailing de script precisa ser propagado para o cursor da linha matemática hospedeira).
3. **Relação com `attach()`**:
   - A causa de `attach()` era a falta de suporte à função nativa no despachante de funções matemáticas.
   - O caso de `#box`/`#text` é o cálculo da largura de avanço de nós de container externo embutidos na linha matemática.

---

## 4. Conclusão

- A hipótese de perda de processamento matemático por erro sintático de parse foi **refutada**: o parser e a AST processam as expressões aninhadas corretamente.
- O efeito visual de `box()` (stroke e inset) é emitido com sucesso.
- O único ponto de ajuste é a propagação do avanço geométrico total do container externo para o cursor da linha matemática em `layout_external`.
