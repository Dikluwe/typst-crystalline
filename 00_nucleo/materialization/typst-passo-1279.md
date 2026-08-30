# Passo 1279 — corrigir a identidade de offsets no harness polar

## Objetivo

Readjudicar os 10 `Unknown-generalization` polares de P1278 depois de testar a
hipótese de que a divergência residual pertence ao adaptador diagnóstico, não
ao algoritmo produtivo: offsets `f64` eram transportados ao probe com apenas
15 dígitos e, em construtores, vinham do gerador Python anterior à avaliação da
fonte.

## Classe ADR-0127

Diagnóstico e correção de harness. Nenhum Prompt L0, consumer produtivo,
contrato público, comportamento por defeito ou fase de pipeline muda.
`paint_is_svg_native` permanece inalterado e nenhuma promoção é autorizada.

## Contrato congelado

1. Reusar sem alteração matriz, budgets, política de `Unknown`, ataques e
   baseline ratificado de P1278.
2. Transportar ao probe os offsets efetivamente avaliados pelo candidato, que
   são parte do observável público já comparado ao vanilla.
3. Serializar cada offset com 17 dígitos significativos, suficientes para
   round-trip de `f64`; componentes `f32` mantêm o transporte anterior.
4. Exigir identidade de cardinalidade, offsets SVG e cores nos cinco stopsets
   únicos que representam os 10 casos residuais.
5. Reexecutar a população geral completa, inválidos, determinismo, custos,
   raster e fronteira produtiva. `Unknown` nunca vira sucesso por arredondar o
   budget.

## Ataques focais

- reduzir o transporte novamente a 15 dígitos;
- usar offsets do gerador antes da avaliação da fonte;
- converter offsets para `f32` antes do probe;
- aceitar apenas cardinalidade, ignorando offset ou cor;
- promover produto a partir do diagnóstico.

## Atestação

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`. Há separação lógica e por artefatos,
mas executor, implementação histórica e verificador compartilham checkout e
capacidade de leitura.
