# Prompt L0 — testes P1446 de `math.stretch`
Hash do Código: afe08bc3

**Camada:** L1
**Alvo:** `01_core/src/compiler/stdlib/structural/p1446_tests.rs`

## Contrato congelado

Os testes independentes cobrem: binding qualificado; casts content/string/
symbol/none; assinatura e diagnósticos; default versus `100%` explícito;
relative length percentual/absoluto; repr e nesting; fields; igualdade;
`map_content`; classe herdada e não-locatabilidade. Qualquer identidade
ausente, execução opaca ou construção impossível é `Unknown` e bloqueia.

Os testes não inspecionam bytes de frame como identidade da linguagem e não
adaptam expectativas ao patch candidato.
