# Prompt L0 — testes independentes de `math.mid` P1437

**Camada**: L1 test-only  
**Alvo**: `01_core/src/compiler/stdlib/structural/p1437_tests.rs`

## Entradas congeladas

Passo P1437 aprovado no commit `b0f48017d29f476357279a678aea3ff2e4c9df70`,
baseline `39826a825a11d966be7adb832606fe6c14373e46`, fonte vanilla pinada
`586e1bd43` e os treze Prompts L0 proprietários atualizados pela proposta.

## Obrigações observáveis

- o módulo `math` contém a função `mid`;
- exatamente um positional convertível em conteúdo constrói
  `Content::MathMid`; ausência, positional extra, `body:`, named desconhecido
  e inteiro falham fechado;
- `none`, string, símbolo, located content e spread canônico seguem o cast e a
  expansão gerais;
- `repr` usa sempre `mid(body: ...)`, incluindo vazio e nesting;
- igualdade, `ObservationEq`, `plain_text` e `map_content` preservam identidade
  e body; `map_text` é terminal;
- a classe de fronteira e a classe efetiva de cada item são `Relation`;
- fora de `lr`, o body preserva tamanho natural;
- no `lr` mais próximo, cada delimitador esticável do body usa a mesma meta
  vertical do grupo, com short-fall `0.1em`; um `lr` interior bloqueia a meta
  exterior;
- `MathMid` é não-locatable e terminal nos walkers aproximados declarados.

## Política e gate discriminatório

Falha de build, timeout, identidade ambígua ou geometria não observável é
`Unknown` e bloqueia o selo. Mutantes válidos: ausência do binding, aridade
zero/extra aceita, `body:` aceito, cast inteiro aceito, identidade/repr apagada,
classe `Normal`, stretch neutralizado e travessia de `lr` interior. Todos devem
ser rejeitados; `mutation_score = 1.0`.

## Regime

`executed_without_isolation_attestation`: contrato, implementação e veredito
ocorrem no mesmo workspace e não recebem alegação de isolamento. A ordem
RED→GREEN e os hashes dos artefatos continuam obrigatórios como recibo.
