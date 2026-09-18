# Prompt L0 — testes independentes de `math.abs` P1447

**Camada**: L1 test-only  
**Alvo**: `01_core/src/compiler/stdlib/structural/p1447_tests.rs`

## Entradas congeladas

Passo P1447 aprovado no commit `91a6fee083e743ed60f966b39c1250d8365ad89b`,
baseline `9e1c891fe31d732b3f0b926cb1343f63a97e07f8`, fonte vanilla pinada
`586e1bd43` e os dois Prompts L0 produtivos atualizados pela proposta.

## Obrigações observáveis

- o módulo `math` contém a função `abs`;
- exatamente um positional convertível em conteúdo constrói
  `Content::MathLr`; ausência, positional extra, `body:`, named desconhecido
  e inteiro falham fechado;
- o body do carrier é exatamente a sequência morfológica `|`, body, `|`, sem
  colapsar nesting e sem converter para `MathDelimited`;
- `none`, string, símbolo, located content e spread canônico seguem o cast e a
  expansão gerais;
- `size` relativo conserva valor e presença, inclusive `100%` explícito;
- o carrier projeta `body` sempre e `size` somente quando explicitamente
  fornecido;
- `repr` usa a projeção `lr(size?, body: sequence([|], ..., [|]))`; a função
  pública chama-se `abs`, enquanto `.func()` do carrier permanece `lr`;
- a rota bare e a qualificada delegam ao mesmo builder, avaliam cada ocorrência
  uma vez e preservam os mesmos diagnósticos e carrier;
- cascata, short-fall, layout, igualdade e walkers permanecem os de `MathLr`
  certificados em P1436.

## Política e gate discriminatório

Falha de build, timeout, identidade ambígua ou observação opaca é `Unknown` e
bloqueia o selo. Mutantes válidos: ausência do binding; uso de
`MathDelimited`; barra esquerda/direita ausente ou trocada; presença de `size`
apagada; zero/extra/`body:`/named aceito; inteiro aceito; rota bare deixada no
braço histórico. Todos devem ser rejeitados; `mutation_score = 1.0`.

## Regime

`executed_without_isolation_attestation`: contrato, implementação e veredito
ocorrem no mesmo workspace e não recebem alegação de isolamento. A ordem
RED→GREEN e os hashes dos artefatos continuam obrigatórios como recibo.
