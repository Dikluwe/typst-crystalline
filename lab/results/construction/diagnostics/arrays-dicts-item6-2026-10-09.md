# Item 6 — alocações, capacidade e clones de arrays/dicionários

## Medição antes da decisão

A bateria [measurements.json](arrays-dicts-item6-2026-10-09/measurements.json) rodou de `2026-10-09T03:17:17.932871+00:00` a `03:18:30.985256+00:00`, no HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, com working tree não commitado. O [status completo](arrays-dicts-item6-2026-10-09/git-status-short.txt) e o [diff stat](arrays-dicts-item6-2026-10-09/git-diff-head-stat.txt) foram capturados no início. Os binários foram o Tekt SHA-256 `4c2b720e7f242ec9eb6b725ae5d91854c601680f5deebd9014c4e47219fc61b0` e o vanilla pinado SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; fonte vanilla `9f2b6e8715237cb086899a42873660fe744622e8` conforme P1489. O script [measure.py](arrays-dicts-item6-2026-10-09/measure.py) conserva comandos, hashes das entradas/fontes, ambiente, CPU, 216 amostras e ordem alternada de 3 blocos × 4 pares por caso, com um aquecimento por motor/caso/bloco. `%M` do `/usr/bin/time` mede pico RSS de processos CLI novos, com filesystem aquecido; não mede bytes vivos do heap.

Os nove documentos imprimem somente o comprimento da coleção (controle `0`, arrays de 1.000/5.000/20.000/40.000 e dicionários de 200/1.000/2.000/4.000). Todos imprimiram o valor esperado em ambos os motores. A [qualificação](arrays-dicts-item6-2026-10-09/qualification.json), com status/diff stat próprios, confirmou mesmas palavras, páginas e raster a 36 dpi; maior delta de caixa de palavra `0,00002 pt`. Isso qualifica este corpus numérico, sem exigir identidade de PDF nem generalizar paridade da linguagem.

| Caso | RSS Tekt / vanilla, MiB | Crescimento sobre `plain`, KiB Tekt / vanilla |
|---|---:|---:|
| plain | 28,08 / 29,40 | 0 / 0 |
| array 20.000 | 32,90 / 30,16 | 4.938 / 778 |
| array 40.000 | 38,33 / 30,79 | 10.494 / 1.424 |
| dicionário 2.000 | 28,64 / 29,66 | 570 / 272 |
| dicionário 4.000 | 28,72 / 29,88 | 662 / 488 |

As colunas são medianas de 12 execuções por motor/caso. O crescimento subtrai medianas do controle, não é uma alocação individual. O array de 40 mil aumentou o RSS em cerca de 10,25 MiB no Tekt contra 1,39 MiB no vanilla. O sinal de dicionário é menor: em 4 mil entradas, o Tekt ainda tem RSS absoluto inferior ao vanilla neste corpus, embora o crescimento sobre seu controle seja maior. Os intervalos mínimo/máximo estão no JSON; diferenças pequenas devem ser lidas à luz da variação do RSS.

## Alocações e capacidade do valor real

A sonda [probe.py](arrays-dicts-item6-2026-10-09/probe.py) compilou [tekt-probe.rs](arrays-dicts-item6-2026-10-09/tekt-probe.rs) contra o `typst_core` release e [vanilla-probe.rs](arrays-dicts-item6-2026-10-09/vanilla-probe.rs) contra o `typst_library` release local. Seu allocator `System` instrumentado mediu pedidos, realocações e bytes vivos por operação, repetidos três vezes com saída idêntica. [probe-results.json](arrays-dicts-item6-2026-10-09/probe-results.json) registra UTC, rustc, comandos, hashes de fontes, rlibs e executáveis. O rlib vanilla é um controle local de representação associado à árvore pinada; seu hash não prova, sozinho, que seus bytes foram vinculados ao binário CLI pinado. As medições de RSS acima usam os binários CLI comprovados.

| Operação isolada | Tekt | Vanilla |
|---|---:|---:|
| `size_of::<Value>()` | 144 B | 32 B |
| construir array de 20.000 inteiros | 1 alocação + 13 realocações; 4.718.592 B vivos | 1 + 13; 1.048.592 B vivos |
| clonar esse array uma vez | 1 alocação; +2.880.000 B | 0 alocações; +0 B |
| construir dicionário de 2.000 inteiros | 2.012 alocações, 10 realocações, 2.010 liberações; 667.664 B vivos | mesmas contagens; 237.584 B vivos |
| clonar esse dicionário uma vez | 2 alocações; +667.664 B | 0 alocações; +0 B |

No Tekt, o `Vec<Value>` de 20 mil termina com capacidade **32.768**, ou **12.768 slots / 1.838.592 B excedentes**. O `IndexMap` de 2 mil termina com capacidade **3.584**, ou **1.584 pares / 253.440 B excedentes** só na região de pares; buckets e cabeçalhos não entram nesse último cálculo. `requested` soma tamanhos pedidos inclusive em realocações e não representa bytes simultaneamente vivos. O clone do mapa preserva a capacidade medida e aloca seu armazenamento novamente. No vanilla, `Array(EcoVec<Value>)` e `Dict(Arc<IndexMap<...>>)` tornam esses clones isolados sem nova alocação; os campos e implementações estão em `lab/typst-original/crates/typst-library/src/foundations/{array,dict}.rs:75,81`.

## Clones da compilação real

O binário de debug foi executado sob o [script GDB](arrays-dicts-item6-2026-10-09/clone-count.gdb), que conta a entrada em `<Value as Clone>::clone`. O [resultado](arrays-dicts-item6-2026-10-09/clone-results.json) registra SHA-256 do binário de debug, fixtures e scripts, [status exato](arrays-dicts-item6-2026-10-09/clone-git-state-at-start.txt) estável durante a série e [logs](arrays-dicts-item6-2026-10-09/clone-logs). GDB mede **chamadas** no build de debug, não bytes alocados nem duração de produção.

| Documento | Chamadas de `Value::clone` |
|---|---:|
| `plain` | 1.336 |
| array 20.000 com `.len()` | 41.350 |
| mesmo array, saída constante sem `.len()` | 1.348 |
| dicionário 2.000 com `.len()` | 19.360 |
| mesmo dicionário, saída constante sem `.len()` | 15.358 |

Remover apenas a leitura de `.len()` retira **40.002** chamadas no array e **4.002** no dicionário: dois clones profundos por elemento, mais duas chamadas de controle. A medição é compatível com o código: `expression_dispatch.rs:79-81` clona o binding; `call_dispatch/instance_methods.rs:17-18` avalia todo alvo de acesso a campo mesmo sem tratar coleções; `call_dispatch/collection_methods.rs:17-18` avalia novamente o alvo antes de despachar `.len()`. `foundations/cast.rs:204-219` constrói `range` com `Vec::new` e `push`. No loop de inserção, `bindings/method_dispatch.rs:230-244,358-374` muta o dicionário por acesso local; a diferença de 14.022 chamadas entre `plain` e dicionário sem `.len()` não é evidência de 14.022 cópias integrais do mapa. O número de cópias integrais é isolado pela diferença com/sem `.len()` e pelo custo de um clone da sonda.

## Decisão

**Não alterar a representação neste item.** Para arrays, o tamanho de `Value`, a reserva geométrica e duas cópias profundas na leitura de `.len()` explicam a direção do crescimento observado; os bytes de sonda não devem ser somados diretamente ao pico RSS do processo. O primeiro experimento corretivo é eliminar a avaliação redundante do alvo nos métodos de coleção, preservando a semântica de avaliação e os erros, e repetir a mesma bateria. Depois, medir pré-reserva de `range` e só então comparar uma representação compartilhável para arrays. Para dicionários, o crescimento atual é modesto diante do custo fixo e não justifica, sozinho, uma troca de payload público. Uma mudança de `Value::Array`/`Value::Dict` no enum público acionaria a ratificação ADR-0127 e exige L0 atualizado antes de código.

Nenhum L0 ou código produtivo foi alterado nesta medição. O `git diff HEAD --stat` permaneceu com SHA-256 `e7790bd020ad6784ab857e2e5532a9590a75490a5388baa6023d1dd10956d5a9` entre bateria, sondas e qualificação; os novos arquivos ficam neste diagnóstico. As medidas antigas de memória5 usavam outro binário Tekt e não servem como baseline numérico deste estado.
