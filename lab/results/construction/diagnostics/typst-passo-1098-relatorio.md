# Passo 1098 — Relatório: Investigação da Margem de Página `auto` com Texto Fluido e Matemática Inline

## 1. Contexto e Diagnóstico

A investigação do Passo 1098 analisou a discrepância de margens em páginas `width/height: auto` quando combinam texto corrido (parágrafos) e matemática inline (`$...$`), conforme evidenciado na Seção 31 (`.typ/sec_31.typ`).

---

## 2. Testes de Isolamento e Resultados Numéricos (§2)

### Teste 1: Parágrafo Fluido Puro sem Matemática (`width: auto`)
- **Largura da Página**: Crystalline `515.19 pt` vs Vanilla `513.96 pt` (Delta $= +1.23\text{ pt}$).
- **Posição de Glifos**: $\Delta X = 0.000003\text{ pt}$, $\Delta Y = 0.000005\text{ pt}$ em todos os 77 glifos do texto.
- **Conclusão**: O layout de texto corrido isolado possui paridade exata de glifo a glifo.

### Teste 2: Parágrafo com Matemática Inline (`$sum_(k=1)^n k^2$`)
- **Comportamento no Vanilla Typst**: Em modo inline (`MathSize::Text`), operadores com limites (`sum`, `prod`, `integral`) decaem por padrão para **scripts laterais** (subscrito e sobrescrito à direita do símbolo $\sum$), ocupando maior largura horizontal e altura contida na linha.
- **Comportamento no Crystalline**: Em modo inline, o layouter empilhou os limites verticalmente sob/sobre o $\sum$ (`limits: true` de Display), alterando a geometria da linha e o avanço subsequente do texto.

### Teste 3: Equações com Espaços Internos em Meio a Frases (`$ a/b $`)
- Na Seção 31, as expressões `$ a/b $` e `$ sqrt(x+1) $` contêm espaços adjacentes aos delimitadores `$` no meio de uma linha de texto.
- O parser AST avalia `eq.block()` como `true` devido à presença de espaços (`$ expr $`).
- No Crystalline, a ocorrência de `Content::Equation(block: true)` no meio de uma linha dispara `flush_line()`, forçando uma quebra de linha de bloco e centragem horizontal diferida em `pending_equation_centering`.

---

## 3. Análise dos Dois Mecanismos de Discrepância

1. **Eixo Horizontal (Largura de Página e Margem Direita)**:
   - A quebra espúria em bloco de `$ a/b $` e `$ sqrt(x+1) $` no meio da frase cria múltiplos sub-frames de linha que alargam a página e deixam a margem direita inflada.
   - O modo inline de $\sum$ com limites empilhados altera o ponto de término da linha de texto.

2. **Eixo Vertical (Altura de Linha e Margem Inferior)**:
   - A presença de limites empilhados em $\sum$ inline eleva a altura necessária da linha de texto, gerando o offset de $\approx 1.27\text{ pt}$ por linha na baseline vertical quando comparado com limites laterais.

---

## 4. Conclusão e Próximos Passos

A causa raiz foi isolada em dois pontos de produção no compilador:
1. **Regra de Limites Padrão de Operadores em Inline**: `math/layout/mod.rs` deve configurar `limits: false` (scripts laterais) quando `math_size == MathSize::Text` (inline), reservando limites empilhados apenas para `MathSize::Display` (bloco).
2. **Contexto de Bloco de Equação**: Uma equação com espaços `$ expr $` não deve quebrar a linha como bloco isolado se não for precedida por quebra de parágrafo / início de fluxo no documento.
