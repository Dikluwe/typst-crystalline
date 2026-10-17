# Reuso em normalize_insertions — candidato 1

## Resultado

Implementado: sequências canônicas sinalizam ausência de substituição; `map_content` conserva o armazenamento compartilhado. Achatamento, sequências degeneradas, origem, estilos e política de espaços continuam vigentes. O ganho é localizado em memória e na tabela com espaços estilizados. Os tempos dos outros casos são mistos; não há certificação de aceleração geral nem de paridade completa com vanilla.

## Medição antes da decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree **não commitado**. Cada recibo inclui `git status --short --untracked-files=all`, `git diff HEAD --stat`, UTC, fixtures integrais/hashes, argv, ambiente, binários e scripts SHA-256. As fontes alteradas estão integrais em `snapshot-normalization-before.json.gz` e `validation-normalization.json.gz`; `normalization-owner-diff.patch` isola esta mudança das alterações anteriores.

Janela final de tempo: **2026-10-10T00:39:45.584987+00:00 → 2026-10-10T00:41:13.088370+00:00**. Alocações: baseline **2026-10-10T00:34:12.789724+00:00 → 2026-10-10T00:34:19.070619+00:00**, final **2026-10-10T00:39:16.196228+00:00 → 2026-10-10T00:39:22.970520+00:00**.

| Binário | SHA-256 |
|---|---|
| before | `1765fa094270c83be515a8ca738b06a92e8ba4a7f5594b3d76cb084cfe82c9cc` |
| after | `55dc425ccca18ca895008f4af871a8189958e689ff1c7f33b8689c1877ca0643` |
| vanilla | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |

Vanilla: pin `9f2b6e8715237cb086899a42873660fe744622e8`; binário comprovado em `lab/typst-original/target/release/typst`, conforme receita P1489. Tekt before é `/tmp/tekt-before-normalization-2026-10-10`; after é `target/release/typst`, construído com `cargo build --release`. O primeiro candidato rejeitado foi preservado em `/tmp/tekt-normalization-first-2026-10-10` e `snapshot-normalization-first.json.gz`.

Método: um CPU fixo, uma thread, fontes fixadas, ordem embaralhada entre before/after/vanilla; três rodadas de quatro pares medidos por fixture, com warmup. Tempo é a mediana das razões pareadas; RSS é a mediana dos picos de processo. As sondas longas exigiram ampliar o limite de stack do baseline; todos os motores receberam igualmente **64 MiB**. Isso fica registrado nos drivers/recibos e não constitui correção do stack.

## Alocações

Contador glibc via `LD_PRELOAD`: pedidos cumulativos de malloc/calloc/realloc/memalign, não heap vivo nem memória retida. Três repetições por fixture.

| Caso | Pedidos before | Pedidos after | Pedidos evitados | Bytes solicitados evitados (MiB) |
|---|---:|---:|---:|---:|
| dense-1600 | 1,765,559 | 1,765,541 | 18 | 5.583 |
| styled-space-1600 | 1,650,659 | 1,650,641 | 18 | 5.583 |
| table-text-1600 | 910,194 | 910,187 | 7 | 0.004 |
| table-styled-space-1600 | 1,079,204 | 1,064,789 | 14,415 | 7.073 |

Texto puro e equações já prontas não acionam necessariamente a realização: `compiler/eval/mod.rs:910–929` condiciona-a a recipes ou math bare; `compiler/realize/bare_math.rs:133–145` trata Equation pronta como fronteira. Os primeiros controles registraram nenhuma redução de pedidos. Foram preservados em `normalization-bypass-{red,green}-gate.json.gz` e `normalization-equation-control-{red,green}-gate.json.gz`. Os alvos finais introduzem uma fração math bare; os arquivos e seu conteúdo exato estão nos recibos. `replacement-1600` permanece controle de preservação, sem alegação de ganho.

## Tempo e pico de processo

| Caso | Before (ms) | After (ms) | Tempo pareado | RSS before (KiB) | RSS after (KiB) | After vs vanilla |
|---|---:|---:|---:|---:|---:|---:|
| dense-1600 | 537.64 | 529.31 | -0.47% | 96260 | 96434 | +193.57% |
| local-set-1600 | 210.76 | 211.65 | +0.92% | 42372 | 42494 | +19.84% |
| no-fill-1600 | 196.10 | 194.09 | -0.36% | 38246 | 38314 | +15.54% |
| styled-space-1600 | 506.91 | 503.85 | -1.00% | 82400 | 82488 | +182.66% |
| table-styled-space-1600 | 461.99 | 441.62 | -3.08% | 57488 | 52610 | +144.23% |
| table-text-1600 | 383.53 | 393.08 | +1.48% | 53892 | 54102 | +125.38% |

A tabela com espaços estilizados evita 14.415 pedidos (−1,34%) e reduz RSS de 57.488 para 52.610 KiB (−8,49%). Texto denso evita apenas 18 pedidos, porém são buffers grandes: cerca de 5,58 MiB solicitados cumulativamente. Seu RSS não mostrou redução nesta janela.

Limite observado: local-set ficou +0,9% e table-text +1,5% em tempo pareado. São custos a acompanhar; a mudança não é um ganho universal. Em table-text, as medianas por rodada foram +1.48%, -0.30%, +2.39%. Em local-set foram +0.84%, +0.94%, +1.40%. Permanecem diferenças grandes contra vanilla nos documentos com mais palavras por célula.

## Refinamento e decisão

A primeira versão também certificava a forma canônica no normalizador in-place da avaliação. Não foi aceita: no-fill teve piora nas duas janelas, e table-text aumentou aproximadamente 2%. Recibos: `measurements-normalization-first.json.gz` e `measurements-normalization-refuter-first.json.gz`.

O refinamento conserva no consumidor in-place a aplicação de todos os grupos como antes. Scanner e aplicação compartilham uma única política; somente o consumidor emprestado verifica se há alteração e materializa Vec a partir da primeira mudança. No-fill deixou de apresentar piora uniforme entre rodadas; table-text permanece misto. A implementação é mantida pelo reuso e pela redução de memória medida, com as pequenas diferenças de tempo explicitamente abertas. Nenhuma alegação de tempo é extrapolada para documentos que não passam por este caminho.

## Código e linhagem

- `01_core/src/compiler/realize/mod.rs:107`: retorna None para sequências canônicas de pelo menos dois elementos; preserva achatamento de Sequence/Realized e canonicalização de zero/um elemento.
- `01_core/src/compiler/eval/rules/textual/space.rs`: scanner/aplicação compartilhados e helper emprestado com saída opcional. Conserva slots Empty, OR de newline, Unicode, Linebreak, Styled e Source; Located/Realized e Source não canônico conservam suas transformações anteriores.
- L0 editado antes do código. Somente os dois owners foram ressellados; relação 1:1 e pins de núcleos conservados. `seal-normalization.json` registra código/prompt/fontes.
- Sem igualdade/hash/endereço para decidir mudança. `Arc::ptr_eq` aparece somente nos testes que verificam o reuso.

## Validação

- RED: o teste de reuso falhou com o normalizador anterior; controle de achatamento passou.
- GREEN final: **6.675 passed, 0 failed, 5 ignored** em `cargo test -p typst-core --lib`. Os controles incluem canonicalização, proveniência de Realized, origem Source, espaços estilizados, Unicode e grupos destrutivos.
- **37 fixtures** (25 tabelas + 6 estendidos + 6 deste experimento): texto/páginas/tamanhos/bbox/raster idênticos entre release anterior e final; raster a 36 dpi, bbox com delta máximo zero. Comparação mecânica é gate de preservação desta otimização, não substituto de paridade de linguagem global.
- **6 consultas em três motores**: `query(table).len()` preservado. `qualification-normalization-verdict.json` contém o veredicto derivado dos recibos.
- `cargo build`, `cargo build --release`, `crystalline-lint .`: exit 0. O lint continua com avisos no repositório, incluindo fallback conservador de espaço; sem deriva V5 nos owners alterados. Não se afirma zero avisos global.

## Reprodução

Drivers e recibos ficam em `00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/`:

```sh
python3 00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/normalization_gate.py red
python3 00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/normalization_gate.py green
python3 00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/measure_normalization.py
```

Os comandos requerem o baseline copiado em /tmp e o release final com os hashes acima. `validation-normalization.json.gz` conserva fontes/L0/logs/toolchain; os recibos de qualificação conservam comandos, páginas, palavras e hashes raster. Esta etapa não remove pré-exames de outras passagens nem amplia o despacho para novos elementos.

## Continuação — candidato 2

[content-containers-reuso-2026-10-10.md](content-containers-reuso-2026-10-10.md) registra o reuso nas 34 variantes, o núcleo compartilhado e os ganhos localizados; as divergências prévias de matemática/estado permanecem abertas.
