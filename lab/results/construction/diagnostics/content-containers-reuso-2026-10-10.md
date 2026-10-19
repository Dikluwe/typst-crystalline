# Reuso de contêineres — candidato 2

## Resultado

Implementado em **34 variantes**: Block, Boxed, Strong, Emph, Equation e toda a família Math presente no dispatcher. Sem substituição, o mapeamento conserva o Arc existente; quando um filho muda, materializa o pai e conserva campos, presença e irmãos não substituídos. O enum fechado e o match estático/exaustivo permanecem.

Os quatro alvos reduziram pedidos glibc entre **5,60% e 11,64%**. Blocos e ênfase em tabela melhoraram em tempo nas três rodadas. Não houve redução clara de pico de RSS nesses dois casos. Matemática repetida conserva divergências grandes contra vanilla; suas razões entre motores não qualificam trabalho equivalente.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`; **working tree não commitado**. `before.json.gz`, `validation.json.gz` e os recibos contêm UTC, `git status --short --untracked-files=all`, `git diff HEAD --stat`, fontes, fixtures, argv, ambiente e hashes. `owner-diff.patch` isola os owners desta etapa; inclui também o núcleo novo. As alterações anteriores do projeto foram conservadas.

Tempo final: **2026-10-10T01:02:11.696679+00:00 → 2026-10-10T01:03:41.384474+00:00**. Baseline de alocações: **2026-10-10T00:53:00.341623+00:00 → 2026-10-10T00:53:06.821650+00:00**; candidato: **2026-10-10T00:57:37.599182+00:00 → 2026-10-10T00:57:48.433410+00:00**.

| Motor | SHA-256 |
|---|---|
| before | `55dc425ccca18ca895008f4af871a8189958e689ff1c7f33b8689c1877ca0643` |
| after | `2d7f5dc93f6a7f15c81ff93b74270c80a14077d6753c8f79c73c9575d113d5f1` |
| vanilla | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |

Before é `/tmp/tekt-before-containers-2026-10-10`, o release aceito ao fim do candidato 1. After é `target/release/typst`, construído com `cargo build --release`. Vanilla é `lab/typst-original/target/release/typst`, pin `9f2b6e8715237cb086899a42873660fe744622e8`, com proveniência P1489. A formalização final dos pins/headers não alterou algoritmos; o rebuild preservou exatamente os bytes do release medido e qualificado.

## Medição antes da decisão

O snapshot captura os mapas dos owners e os braços de `entities/content/traversal.rs:111–192`: resultados eram embutidos em Some após reconstrução, mesmo com callbacks None. O RED enumerou reconstrução nas 34 variantes e perdeu o armazenamento dos irmãos de uma matriz alterada; ordem, erro e metadados já passaram como controles. A hipótese de custo foi refutável por preservação, pedidos de alocação e tempo, sem inferir intenção do vanilla nem copiar vtable.

Método: contador glibc por LD_PRELOAD, três repetições por alvo. Pedidos e bytes solicitados são **cumulativos**, não heap vivo. Tempo: três rodadas de quatro pares por fixture, warmup, ordem embaralhada entre os motores, CPU fixo, uma thread, fontes e ambiente fixados. Limite de stack de 64 MiB igual nos três motores. Tempo abaixo é a mediana das razões pareadas; RSS é a mediana de picos por processo.

## Alocações

| Caso | Before | After | Redução de pedidos | MiB solicitados evitados |
|---|---:|---:|---:|---:|
| blocks-400 | 681,317 | 602,007 | 11.64% | 27.33 |
| semantic-1600 | 1,453,959 | 1,313,003 | 9.69% | 42.28 |
| math-400 | 987,666 | 921,556 | 6.69% | 28.54 |
| matrix-400 | 1,324,299 | 1,250,189 | 5.60% | 26.89 |

## Tempo e pico de processo

| Caso | Before (ms) | After (ms) | Tempo pareado | RSS before (KiB) | RSS after (KiB) |
|---|---:|---:|---:|---:|---:|
| blocks-400 | 246.87 | 239.05 | -2.70% | 46742 | 46820 |
| local-set-1600 | 207.86 | 208.05 | +0.47% | 42174 | 41408 |
| math-400 | 504.95 | 498.50 | -1.04% | 54540 | 53406 |
| matrix-400 | 724.31 | 715.13 | -0.81% | 55064 | 54424 |
| no-fill-1600 | 188.60 | 186.64 | -0.32% | 38448 | 37522 |
| semantic-1600 | 495.93 | 476.99 | -3.27% | 73356 | 73370 |

Blocos e ênfase em tabela melhoraram em todas as rodadas; reduções pareadas por rodada:

- blocks-400: -2.35%, -3.64%, -2.48%.
- semantic-1600: -1.94%, -3.00%, -4.35%.

Os controles local-set/no-fill ficaram +0,5%/−0,3%, com resultados mistos entre rodadas; não há ganho geral comprovado. O alvo matrix-400 também é misto entre rodadas. A diminuição de tráfego de alocação não é certificação de redução de memória residente ou retida entre recompilações.

## Implementação e governança

- Cada owner de elemento implementa helper crate-private `map_content_if_changed`; o dispatcher delega em um braço estático por variante.
- Elementos com body único, campos opcionais e slots transformam filhos na ordem anterior e clonam o payload somente depois de Some. Omitted, ExplicitNone e Present(Empty) continuam distintos.
- Cases/Vec usam o mapeamento lazy de slices; Matrix cria rows somente na primeira linha alterada, conservando ordem e conteúdos compartilhados dos irmãos.
- Primes e AlignPoint são terminais: helper None; o callback do nó continua sendo chamado pelo dispatcher. Não mudam identidade ou contagem.
- A API pública `Content::map_content` e o trait Element não mudaram. O método legado do trait delega ao helper e pode reconstruir a raiz a partir de `&Elem`; o reuso do Arc original é feito por Content, que possui esse armazenamento.
- Some explícito, inclusive igual, continua substituição. Não há comparação/hash/endereço para inferir mudança, preflight que suprima callbacks, vtable, campo novo ou mudança de fase. `map_content_pre`/`map_text` não foram migrados.

A obrigação compartilhada foi declarada em `00_nucleo/prompts/_nuclei/content/map-content-change.toml`, effective SHA-256 `ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a`. O núcleo é TOML estrito, sem consumer ou Hash do Código. **25 prompts owners** consomem o pin: 23 deste dispatcher/elementos e os owners Table/Cell já migrados, que receberam apenas formalização de protocolo/headers. Cada consumer continua com exatamente um owner. `seal.json` registra os hashes; o lint final não contém V5/V15/V26 nos owners/núcleo desta etapa.

## Validação

- RED: **3 passed, 2 failed** nos cinco testes novos; as falhas demonstraram reconstrução das 34 variantes e perda de reuso dos irmãos em Matrix.
- GREEN: **6.680 passed, 0 failed, 5 ignored** na suíte core, incluindo os cinco testes novos. Cobertura: no-op, substituição igual explícita, mudança com preservação de tipo/metadados, slots e sua ordem, Matrix/augment/flags/irmãos, primeiro erro sem visitas posteriores.
- **40 documentos atuais**: 25 tabelas, 6 estendidos e 9 desta etapa. Texto, páginas, tamanho de página, bbox e raster a 36 dpi preservados entre before/after; delta bbox máximo zero. A sonda de efeitos usa counter.step: três hits e uma tabela nos três motores.
- **6 consultas em três motores**: `query(table).len()` igual a 1. `qualification-verdict.json` deriva o gate dos recibos.
- `cargo build`, `cargo build --release`, `crystalline-lint .`: exit 0. Existem avisos no repositório; não se declara zero avisos global. Sem V5/V15/V26 na linhagem alterada.

Os recibos `qualification-containers.json.gz` conservam também a primeira sonda de state.update; os oito outros documentos seguem atuais. A sonda substituta de efeitos está em `qualification-effects.json.gz`, que fecha o controle counter.step atual. A comparação mecânica qualifica preservação desta otimização; não substitui paridade de linguagem global.

## Divergências prévias medidas, ainda abertas

As sondas matemáticas não são equivalentes ao vanilla em fluxo/paginação:

| Caso | Tekt before | Tekt after | Vanilla |
|---|---:|---:|---:|
| blocks-400 | 16 | 16 | 19 |
| math-400 | 386 | 386 | 27 |
| matrix-400 | 388 | 388 | 31 |

O before já possui essas diferenças; a otimização preserva sua saída. Por isso o tempo de math-400/matrix-400 e blocks-400 só prova comparação Tekt before/after, não fechamento da diferença de desempenho com vanilla para trabalho equivalente. `semantic-1600`, local-set e no-fill mantiveram a cardinalidade de páginas/palavras entre motores; render/seleção gerais continuam fora da certificação.

A primeira sonda de efeito mostrou `state.update(n => n + 1)` devolvendo uma função em Tekt before/after, enquanto o vanilla imprime 3. O input foi preservado em `state-update-preexisting.typ`; a extração de palavras está no recibo original. É divergência prévia independente desta mudança; counter.step foi usado para qualificar execução única sem encobri-la.

Também apareceu no baseline um panic de block com borda/radius repetido atravessando páginas (`compiler/layout/block.rs:398`, insertion index maior que current_items.len). O alvo de reuso foi medido sem borda/radius; esta etapa não corrige nem certifica esse caminho de layout. Não transformar essas sondas em alegação de paridade fechada.

## Reprodução

Drivers/recibos: `00_nucleo/diagnosticos/content-container-reuse-2026-10-10/`.

```sh
python3 00_nucleo/diagnosticos/content-container-reuse-2026-10-10/allocation_gate.py red
python3 00_nucleo/diagnosticos/content-container-reuse-2026-10-10/allocation_gate.py green
python3 00_nucleo/diagnosticos/content-container-reuse-2026-10-10/measure.py
```

Requer o baseline copiado em /tmp e o release final com os hashes acima. `before.json.gz` contém fontes/L0 prévios; `validation.json.gz` contém fontes/L0 finais, núcleo, logs/toolchain/linhagem. Os recibos guardam fixtures/fontes/comandos completos, páginas/palavras e hashes raster. O candidato 3 (pré-exames) não foi alterado nesta etapa.

Continuação: [retirada dos pré-exames](content-preflight-removal-2026-10-10.md), com pequeno ganho de tempo e contagem de alocações preservada.
