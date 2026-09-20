# Prompt L0 — testes independentes de `math.norm` P1438
Hash do Código: 8769c4b9

**Camada**: L1 test-only  
**Alvo**: `01_core/src/compiler/stdlib/structural/p1438_tests.rs`

## Obrigações observáveis

- o módulo `math` contém a função `norm`;
- exatamente um positional convertível em conteúdo constrói
  `Content::MathLr`; ausência, positional extra, `body:`, named desconhecido
  e inteiro falham fechado;
- o body do carrier é exatamente a sequência morfológica `‖`, body, `‖`, sem
  colapsar nesting e sem converter para `MathDelimited`;
- `none`, string, símbolo, located content e spread canônico seguem o cast e a
  expansão gerais;
- `size` relativo conserva valor e presença, inclusive `100%` explícito;
- `repr` usa a projeção `lr(size?, body: sequence([‖], ..., [‖]))` e a função
  pública mantém o nome `norm`;
- a rota bare e a qualificada delegam ao mesmo builder, avaliam cada ocorrência
  uma vez e preservam os mesmos diagnósticos e carrier;
- cascata, short-fall, layout, igualdade e walkers permanecem os de `MathLr`
  definidos em P1436.
