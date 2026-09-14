# Passo 1361 — formatter puro de duração e adaptador S11

## Objetivo

Materializar em L2 o formatter público e determinístico de
`std::time::Duration` do vanila ratificado, fechando a limitação instrumental
`S11_EXACT_BOUNDARY_VECTOR` sem executar nem alterar o ciclo de `watch`.

## Entradas e owners

- preflight: `00_nucleo/diagnosticos/p1361-duration-preflight.md`;
- novo Prompt L0 proprietário:
  `00_nucleo/prompts/shell/duration.md` → `02_shell/src/duration.rs`;
- owner do índice do crate:
  `00_nucleo/prompts/shell.md` → `02_shell/src/lib.rs`;
- baseline: vanila ratificado `586e1bd43`.

## Contrato observável

`format_duration(Duration) -> impl Display` é puro e produz o vetor:

```text
1000000 s       -> 11 d 13 h 46 min
86400 s         -> 1 d
3600 s          -> 1 h
3840 s          -> 1 h 4 min
364.77 s        -> 6 min 4 s
264.776 s       -> 4 min 24.78 s
3 s             -> 3 s
2.8492 s        -> 2.85 s
1 s             -> 1 s
999 ms          -> 999 ms
1 ms            -> 1 ms
294816 µs       -> 294.82 ms
999 µs          -> 999 µs
734 µs          -> 734 µs
1 µs            -> 1 µs
999 ns          -> 999 ns
1 ns            -> 1 ns
0 ns            -> 0 s
```

Separadores são um espaço entre número/unidade e entre peças. Com dia ou hora,
segundos e frações são omitidos; a partir de cinco minutos, segundos são
inteiros. Abaixo disso, a unidade é escolhida pelas fronteiras inclusivas de
1 s, 1 ms e 1 µs e a parte decimal é arredondada a duas casas, sem zeros finais
forçados. O controle `300.006 s -> 5 min 0 s` distingue deslocamentos reais do
limiar de cinco minutos; trocar apenas `>= 5 min` por `> 5 min` é mutação
equivalente para `Duration` e não recebe crédito.

## Scope-out

- nenhuma chamada ao relógio;
- nenhuma alteração ou execução de `watch`/`w`;
- nenhuma mensagem nova no processo;
- nenhuma dependência externa ou mudança em L1/L3/L4;
- não declarar paridade geral: este passo fecha apenas o formatter/adaptador.

## Ordem segregada

1. congelar L0, passo, preflight e baseline;
2. autor independente escreve contrato;
3. oráculos/adversário cobrem fronteiras, arredondamento e opacidade;
4. gate 1.0 sela antes de testes/código;
5. testador e implementador trabalham a partir do selo;
6. executar somente testes unitários focais e gates estáticos;
7. verificador independente emite certificado restrito.

O ambiente partilhado não permite atestar isolamento forte; registrar
“executado sem atestação de isolamento”.

## Contenção de memória

- `watch`/`w` como processo: proibidos;
- `CARGO_BUILD_JOBS=1`;
- um teste exato por comando, `timeout <= 120s`;
- stdout/stderr sempre em ficheiros;
- nenhum teste workspace, filtro amplo ou corpus integral;
- timeout, OOM, `Unknown` inesperado ou recibo com zero testes impedem PASS.

## Gate ADR-0127

A nova função pública Rust é mudança de contrato. A instrução explícita do
dono “Escreva e implemente o próximo passo” autoriza esta fronteira limitada;
qualquer ligação futura ao comportamento padrão de `watch` continua exigindo
passo próprio.
