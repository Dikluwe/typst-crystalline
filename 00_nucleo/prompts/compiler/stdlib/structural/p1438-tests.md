# Prompt L0 — testes independentes de `math.norm` P1438

**Camada**: L1 test-only  
**Alvo**: `01_core/src/compiler/stdlib/structural/p1438_tests.rs`

## Entradas congeladas

Passo P1438 aprovado no commit `0a1d813f1243f04d36435632dc364919aec4ce15`,
baseline `c8071e1004c4ec5ee80cc1b87aeb234d6a5c7e14`, fonte vanilla pinada
`586e1bd43` e os dois Prompts L0 proprietários atualizados pela proposta.

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
  certificados em P1436.

## Política e gate discriminatório

Falha de build, timeout, identidade ambígua ou observação opaca é `Unknown` e
bloqueia o selo. Mutantes válidos: ausência do binding; uso de
`MathDelimited`; cerca esquerda/direita ausente ou trocada; presença de `size`
apagada; zero/extra/`body:`/named aceito; inteiro aceito; rota bare deixada no
braço histórico. Todos devem ser rejeitados; `mutation_score = 1.0`.

## Regime

`executed_without_isolation_attestation`: contrato, implementação e veredito
ocorrem no mesmo workspace e não recebem alegação de isolamento. A ordem
RED→GREEN e os hashes dos artefatos continuam obrigatórios como recibo.
