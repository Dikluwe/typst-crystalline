# Grid: identidade de células e caches de shaping no Tekt real

## Medição antes da decisão

Em 2026-10-03 02:33:53 UTC, HEAD
`d87e2b002ad6a61e10703f3a8e2e1d837795cd2f`, `git diff HEAD --stat`
vazio. A árvore continha apenas o runner novo
`?? 00_nucleo/diagnosticos/grid-cache-real-2026-10-03/run.py` no `git status
--porcelain=v1 --untracked-files=all`; as duas fixtures `.typ` e os logs da
sonda estavam ignorados por `00_nucleo/diagnosticos/.gitignore` e foram
versionados explicitamente depois. O runner registra hashes SHA-256 das
fixtures e binários, hora, CPU, amostras e estado da árvore em
[results.json](grid-cache-real-2026-10-03/results.json).

As fixtures têm 80 linhas, quatro colunas `Auto`, 320 células e texto de quatro
caracteres: [distintas](grid-cache-real-2026-10-03/grid80-distinct.typ) usam
`C000`–`C793`; [iguais](grid-cache-real-2026-10-03/grid80-same.typ) usam
`C000` em todas. O runner fez duas warmups e 16 pares de subprocessos novos
em ordem alternada por motor, CPU 0, `SOURCE_DATE_EPOCH=0`,
`RAYON_NUM_THREADS=1`, incluindo spawn, compilação, exportação PDF e escrita.
Binário Tekt release SHA-256
`a12d75ded40944932b20dbd751d7969d3f846ac04de10c817cfffee707175cc4`;
vanilla pinado em `9f2b6e8715237cb086899a42873660fe744622e8`, binário
SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

| Motor | Células distintas | Células iguais | Diferença pareada, iguais − distintas |
| --- | ---: | ---: | ---: |
| Tekt | 247,58 ms | 215,81 ms | **−31,61 ms** |
| Vanilla | 149,08 ms | 149,19 ms | −0,38 ms |

Os PDFs têm uma página e 320 palavras. Para cada motor, todas as coordenadas
`xMin/yMin/xMax/yMax` extraídas por `pdftotext -bbox` foram idênticas entre
as duas fixtures; apenas o texto das palavras mudou. Esta equivalência de
geometria não prova que o custo de shaping seja a única causa do delta de
tempo: o conjunto de palavras distintas também muda outras consultas do
pipeline.

## Sondas sem editar L1–L4

O binário debug Tekt tem SHA-256
`107983d12b47381cf6f750d7b9a2c5d15eed816f2604165812e49a45a3ac451b`.
Os scripts GDB versionados são [conteúdo](grid-cache-real-2026-10-03/grid-content-probe.gdb),
[hit/miss](grid-cache-real-2026-10-03/grid-font-cache-count.gdb) e
[instâncias](grid-cache-real-2026-10-03/grid-font-epochs.gdb). A sonda de
conteúdo observou **640 chamadas** a `measure_content_constrained` em cada
fixture, todas `Content::Text`, com **640 endereços de `Content` distintos**
nessas execuções: [distintas](grid-cache-real-2026-10-03/distinct-content-probe.txt),
[iguais](grid-cache-real-2026-10-03/same-content-probe.txt). Os textos curtos
estavam inline no `EcoString` mostrado pelo GDB. A igualdade do texto não
fornece uma alocação `Arc` compartilhada para a chave do laboratório. Endereço
de `Content` tampouco é identidade semântica entre passagens; o alocador pode
reutilizá-lo.

O breakpoint na consulta de `shaped_width_cache` (`caches.rs:170`) contou
**2.880 consultas** em cada fixture. O breakpoint no cálculo após falha
(`caches.rs:175`) contou **962 falhas** com textos distintos e **5** com texto
igual: [distintas](grid-cache-real-2026-10-03/distinct-font-cache-count.txt),
[iguais](grid-cache-real-2026-10-03/same-font-cache-count.txt). São contagens
da **compilação inteira**, não apenas da resolução de colunas. Como GDB
interrompe o processo em cada evento, os tempos acima vêm do runner sem GDB.

Na fixture igual, `C000` falhou três vezes com mesmo `size_bits` e `font_hash`,
mas em **três instâncias de `shaped_width_cache` distintas**, observadas no
[log de instâncias](grid-cache-real-2026-10-03/same-font-epochs.txt). Os
[backtraces](grid-cache-real-2026-10-03/same-c000-stacks.txt)
associaram as duas primeiras a chamadas de `layout_grid` e a terceira
ao ajuste final em `shaper/reconciliation.rs:234`. A fonte confirma que
`paged.rs:318` e `paged.rs:390` chamam novamente
`layout_with_realized_math_callbacks`, que constrói
`FallbackFontMetrics::new(world)` em `paged.rs:521`; `fix_line_positions`
constrói sua própria instância em `shaper/reconciliation.rs:29`.

## Classificação e próximo experimento

**Fato:** o cache de fonte já elimina quase todo recálculo de shaping quando
o texto se repete, mas não compartilha resultados entre as duas passagens de
layout desta fixture. **Inferência a testar:** compartilhar o mesmo
`FallbackFontMetrics` entre essas passagens, mantendo World, chave completa e
stores de callbacks locais a cada passagem, pode evitar uma parte das falhas
com textos distintos. O que refutaria a hipótese: contagens de falha menores
sem redução estável de tempo total, ou mudança em texto/geometria/diagnósticos.

Esta é uma hipótese sobre mecânica e cache, não uma obrigação de paridade com
o vanilla. O cache por identidade de `Arc<[u16]>` do laboratório não representa
as células `Content::Text` aqui observadas. Antes de qualquer código
produtivo, atualizar o owner L0 de `infra/pipeline/paged`; a variante deve ser
medida contra o binário original no mesmo host e revertida se não trouxer
ganho sem alterar a saída.
