# Prompt L0 — testes independentes de `math.mid` P1437
Hash do Código: 02687e52

**Camada**: L1 test-only  
**Alvo**: `01_core/src/compiler/stdlib/structural/p1437_tests.rs`

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
