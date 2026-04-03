# Baseline de Paridade — Passo 35

**Data**: 2026-04-03
**Testes existentes antes do passo**: 40
**Testes adicionados (matemática)**: 10
**Total**: 50

## Resultados

| Categoria | Total | Passam | Falham |
|-----------|-------|--------|--------|
| Markup geral | 13 | 13 | 0 |
| Trivia (whitespace) | 5 | 5 | 0 |
| Recuperação de erros | 4 | 4 | 0 |
| Math (pré-existentes) | 7 | 7 | 0 |
| Code/eval | 10 | 10 | 0 |
| Corpus de ficheiros | 1 | 1 | 0 |
| Matemática (Passo 34, novos) | 10 | 10 | 0 |
| **Total** | **50** | **50** | **0** |

## Divergências por categoria

Nenhuma divergência encontrada. O parser cristalino tem paridade total com o
oráculo (`typst-syntax`) para todos os 50 inputs testados.

## Detalhes dos 10 novos testes de matemática

| Teste | Input | Resultado |
|-------|-------|-----------|
| `parity_equation_inline_simples` | `$x$` | ✓ paridade |
| `parity_equation_block_simples` | `$ x^2 $` | ✓ paridade |
| `parity_equation_frac` | `$ frac(a, b) $` | ✓ paridade |
| `parity_equation_attach_sup` | `$ x^2 $` | ✓ paridade |
| `parity_equation_attach_sub` | `$ x_i $` | ✓ paridade |
| `parity_equation_attach_sub_sup` | `$ x_i^2 $` | ✓ paridade |
| `parity_equation_root` | `$ sqrt(x) $` | ✓ paridade |
| `parity_equation_complexa` | `$ sum_(i=0)^n x_i^2 $` | ✓ paridade |
| `parity_equacao_inline_em_texto` | `O valor de $x^2 + y^2$ é positivo.` | ✓ paridade |
| `parity_equacao_com_texto_literal` | `$ "resultado" = x $` | ✓ paridade |

## Estrutura verificada

`CompactNode` normaliza:
- Spans eliminados (comparação por estrutura apenas)
- `SyntaxKind` via `.name()` canónico (minúsculo)
- Nós de erro via `CompactNode::Error(msg, text)`
- Whitespace e trivia **incluídos** na comparação (preserva fidelidade)

## Conclusão para Passo 36

**Go** — o parser cristalino está pronto para o Passo 36 (motor de equações).

- `Expr::Equation`, `MathAttach`, `MathFrac`, `MathRoot`, `MathIdent`,
  `MathText` todos produzem árvores idênticas ao oráculo
- `attach.base()`, `attach.bottom()`, `attach.top()` extraem correctamente
  base, subscript e superscript — o motor pode confiar nesta API
- `sqrt(x)` parseia como chamada de função (`FuncCall`), não como `MathRoot`
  — comportamento idêntico ao oráculo, confirmado
- Primes (`a'''`) e `MathAlignPoint` (`&`) também têm paridade confirmada

Não há divergências de parsing que requeiram resolução antes do Passo 36.
