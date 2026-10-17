# Próxima auditoria — construção de inteiros grandes

Evidência desta etapa: before/supplement-before/after guardam os dois casos manuais e três controles int.min. Não é comportamento incorreto de calc.abs: o literal chega como Float no Tekt e é rejeitado antes da chamada no pin.

Fonte localizada: `01_core/src/compiler/lexer/code.rs:167–178` promove inteiros decimais grandes a Float quando i64 transborda e f64 aceita. `entities/ast/expr.rs:324–339` expõe Int::get com retorno i64 e fallback default. `compiler/eval/expression_dispatch.rs:71` lê esse getter. Pin9f2b6e871: typst-eval/src/ops.rs:116–133 valida overflow da literal negada e gera a mensagem especial para max_plus_one. Isso corrige a atribuição inicial genérica ao parser: há classificação no lexer cristalino e diagnóstico no eval upstream.

Próximo passo: medir limites i64, valores maiores, decimal/binário/octal/hex, sinal unário/binário, parênteses/espaços, floats explícitos/exponentes e sufixos dimensionais, com mensagens/hints/spans e eval/compilação. Ler os owners vigentes antes de escolher a fase da validação. Não remover a promoção cegamente e deixar Int::get retornar zero.

Se a solução alterar assinatura pública de Int::get ou mover validação entre fases, preparar revisão e pedir confirmação conforme ADR-0127 §2.1/§2.3/§2.4 antes de materializar. Não adotar lookbehind do caractere '-' como substituto de sintaxe/semântica. Preservar os defaults e fases não ratificados. Nenhuma correção desse eixo foi implementada neste lote de V16.
