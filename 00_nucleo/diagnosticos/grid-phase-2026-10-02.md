# Custo das fases de grid: colunas, linhas e emissão

## Método e proveniência

Em 2026-10-02 22:20:29 UTC, HEAD `b388851b081bdf54579d9e731aed057dd9ea468f`, árvore limpa no início (`git status --porcelain=v1 --untracked-files=all` e `git diff HEAD --stat` vazios). O release Tekt tem SHA-256 `a12d75ded40944932b20dbd751d7969d3f846ac04de10c817cfffee707175cc4`, o mesmo da [sonda end-to-end anterior](grid-time-2026-10-02.md). Nenhum arquivo de L0 ou código produtivo foi editado. A análise usa apenas as fixtures de 4, 80 e 320 células já versionadas naquele diagnóstico.

O [script GDB](grid-phase-2026-10-02/grid_phase_full.gdb) coloca breakpoints de entrada e retorno nos símbolos release de `resolve_grid_columns`, `resolve_grid_rows` e `emit_grid_cells`. O [runner](grid-phase-2026-10-02/run_grid_phase_full.py) executou dez processos por fixture, `SOURCE_DATE_EPOCH=0`, `RAYON_NUM_THREADS=1`, mesma CLI de compilação e gerou os [spans individuais](grid-phase-2026-10-02/grid_phase_full_results.json). O GDB observou duas passagens completas do grid por processo; esta é uma característica desta fixture, não garantia geral do pipeline. Os valores abaixo usam somente a **segunda passagem**, para evitar o custo inicial do primeiro callback de breakpoint que inflou a primeira fase de cada processo. Mesmo assim, o GDB altera o tempo observado; os números são diagnóstico local de ordem de grandeza, não previsão de ganho em produção. Não houve rodada longa com controle de carga do host.

| Tabela | Colunas `Auto` | Linhas `Auto` | Emissão | Soma das três fases |
| --- | ---: | ---: | ---: | ---: |
| 4 células | 0,45 ms | 0,34 ms | 0,36 ms | 1,14 ms |
| 80 células | 3,00 ms | 1,50 ms | 2,10 ms | 6,57 ms |
| 320 células | 11,20 ms | 5,19 ms | 7,66 ms | 24,07 ms |

Valores são medianas de dez processos; a soma é mediana das somas por processo, não necessariamente soma das três medianas. Na tabela de 320 células, os intervalos observados foram 10,94–11,77 ms (colunas), 5,08–5,31 ms (linhas), 7,53–8,07 ms (emissão) e 23,58–25,14 ms (soma). A primeira passagem das linhas soma trabalho menor que a resolução das colunas e que a emissão neste caso.

## Controle de colunas fixas

Em [fixture derivada](grid-phase-2026-10-02/grid80-fixed.typ), apenas `columns: 4` virou `columns: (100pt, 100pt, 100pt, 100pt)`. O PDF Tekt conservou uma página de 500 × 3500 pt e o mesmo texto extraído em ordem (`pdftotext -raw` SHA-256 `12708d834a1603d47953b59234a06399296692c51b452b1804ae949406454bdb`), mas a geometria de colunas mudou. Dez processos sob o mesmo GDB e o [runner de controle](grid-phase-2026-10-02/run_grid_fixed.py) deram, na segunda passagem:

| 320 células | Colunas | Linhas | Emissão | Soma |
| --- | ---: | ---: | ---: | ---: |
| `Auto` | 11,20 ms | 5,19 ms | 7,66 ms | 24,07 ms |
| Fixas | 0,35 ms | 13,73 ms | 7,61 ms | 21,70 ms |

Os [spans do controle](grid-phase-2026-10-02/grid_fixed_results.json) confirmam que remover a medição de largura quase zera o tempo da fase de colunas, mas o custo da fase de linhas aumenta cerca de 8,5 ms. **Inferência**, ainda não comprovada por contadores de cache: a passagem por colunas aquece métricas/shaping que as linhas reutilizam. A fonte sustenta a possibilidade: `grid/tracks.rs:62–84` chama `measure_content_constrained` para cada célula `Auto`; `compiler/layout/measurement.rs:70–92` mede palavras via `FontMetrics::text_width`; a implementação L3 mantém caches de largura moldada em `font_metrics/caches.rs:155`. O controle muda largura e contexto das células, logo não isola cache nem estabelece speedup de 2,37 ms para uma otimização de mesma saída.

## Decisão experimental

`grid/tracks.rs:175` usa `layout_sub_frame` para altura de linha e descarta itens/decoracão; construir menos nessa chamada ainda é uma ideia válida. Nesta fixture simples, **5,19 ms por passagem** é o limite superior bruto da própria fase de linhas observada sob GDB; um medidor especializado preservaria o custo necessário de texto e geometria. Portanto não há prova de que essa mudança reduza materialmente o delta CLI de [94,33 ms](grid-time-2026-10-02.md) frente ao vanilla. O controle fixo mostra que subtrair tempos de uma fase isolada pode superestimar o ganho quando trabalho migra para outra.

Próxima medição causal antes de prototipar código: contar hit/miss e tempo de `FontMetrics::text_width`/shaping entre colunas e linhas, e repetir com células de texto longo, formas e alinhamento. Se o custo de itens descartados nas linhas permanecer relevante depois desses controles, redigir L0 owner antes do protótipo e exigir equivalência geométrica RED→GREEN. Este relatório não altera contrato, default ou implementação.
